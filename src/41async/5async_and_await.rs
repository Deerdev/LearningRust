/// * async/.await是 Rust 语法的一部分，它在遇到阻塞操作时( 例如IO )会让出当前线程的所有权而不是阻塞当前线程，这样就允许当前线程继续去执行其它代码，最终实现并发。
// 有两种方式可以使用async： async fn用于声明函数，async { ... }用于声明语句块
//
// `foo()`返回一个`Future<Output = u8>`,
// 当调用`foo().await`时，该`Future`将被运行，当调用结束后我们将获取到一个`u8`值
async fn foo() -> u8 {
    5
}

fn bar() -> impl Future<Output = u8> {
    // 下面的`async`语句块返回`Future<Output = u8>`
    async {
        let x: u8 = foo().await;
        x + 5
    }
}

/// * async 的生命周期
// async fn 函数如果拥有引用类型的参数，那它返回的 Future 的生命周期就会被这些参数的生命周期所限制:
// 当 x 依然有效时，该 Future 就必须继续等待( .await ), 也就是说x 必须比 Future活得更久。
async fn foo1(x: &u8) -> u8 {
    *x
}

// 上面的函数跟下面的函数是等价的:
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}

// 但是，若 Future 被先存起来或发送到另一个任务或者线程，就可能存在问题了:
use std::future::Future;
fn bad() -> impl Future<Output = u8> {
    let x = 5;
    borrow_x(&x) // ERROR: `x` does not live long enough
                 // x结束就会销毁
}

async fn borrow_x(x: &u8) -> u8 {
    *x
}
// 解决办法：将具有引用参数的 async fn 函数转变成一个具有 'static 生命周期的 Future
fn good() -> impl Future<Output = u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

/// * async move
// async 允许我们使用 move 关键字来将环境中变量的所有权转移到语句块内,允许它的生命周期超过当前作用域(outlive)，就像闭包那样
// - 好处是你不再发愁该如何解决借用生命周期的问题
// - 坏处就是无法跟其它代码实现对变量的共享:
//
// 多个不同的 `async` 语句块可以访问同一个本地变量，只要它们在该变量的作用域内执行
async fn blocks() {
    // 还在函数内，my_string 还有效，也没有发生 move，两个 async 内都可以访问
    let my_string = "foo".to_string();

    let future_one = async {
        // ...
        println!("{}", my_string);
    };

    let future_two = async {
        // ...
        println!("{}", my_string);
    };

    // 运行两个 Future 直到完成
    let ((), ()) = futures::join!(future_one, future_two);
}
// 由于`async move`会捕获环境中的变量，因此只有一个`async move`语句块可以访问该变量
//
// 但是它也有非常明显的好处： 变量可以转移到返回的 Future 中，不再受借用生命周期的限制
fn move_block() -> impl Future<Output = ()> {
    // my_string 被 move 进async，生命周期变长
    let my_string = "foo".to_string();
    async move {
        // ...
        println!("{}", my_string);
    }
}

/// * 当.await遇见多线程执行器
// 当使用多线程 Future 执行器( executor )时，Future 可能会在线程间被移动，因此 async 语句块中的变量必须要能在线程间传递。
// 至于 Future 会在线程间移动的原因是：它内部的任何.await都可能导致它被切换到一个新线程上去执行。
/// * 由于需要在多线程环境使用，意味着 Rc、 RefCell 、没有实现 Send 的所有权类型、没有实现 Sync 的引用类型，它们都是不安全的，因此无法被使用
//
// 类似的原因，在 .await 时使用普通的锁也不安全，例如 Mutex 。
// 原因是，它可能会导致线程池被锁：当一个任务获取锁 A 后，若它将线程的控制权还给执行器，然后执行器又调度运行另一个任务，该任务也去尝试获取了锁 A ，结果当前线程会直接卡死，最终陷入死锁中。

// 因此，为了避免这种情况的发生，我们需要使用 futures 包下的锁 futures::lock 来替代 Mutex 完成任务。
