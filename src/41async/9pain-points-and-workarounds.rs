/// 一些疑难问题的规避解决办法
// async 在 Rust 依然比较新，疑难杂症少不了，而它们往往还处于活跃开发状态，短时间内无法被解决，因此才有了本文。下面一起来看看这些问题以及相应的临时解决方案。

/// * async 块中的 ? 运算符
// async 语句块和 async fn 最大的区别就是前者无法显式的声明返回值，在大多数时候这都不是问题，但是当配合 ? 一起使用时，问题就有所不同:
async fn foo1() -> Result<u8, String> {
    Ok(1)
}
async fn bar1() -> Result<u8, String> {
    Ok(1)
}
pub fn main1() {
    let fut = async {
        foo().await?;
        bar().await?;
        Ok(()) //! cannot infer type for type parameter `E` declared on the enum `Result`
    };
}
// * 编译报错
// 原因在于编译器无法推断出 Result<T, E>中的 E 的类型， 而且编译器的提示consider giving fut a type 你也别傻乎乎的相信，然后尝试半天，最后无奈放弃：目前还没有办法为 async 语句块指定返回类型。
//
// 既然编译器无法推断出类型，那咱就给它更多提示，可以使用 ::< ... > 的方式来增加类型注释：`Ok::<(), String>(())`
/*
let fut = async {
    foo().await?;
    bar().await?;
    Ok::<(), String>(()) // 在这一行进行显式的类型注释
};
 */

/// * async 函数和 Send 特征
// 多线程间数据传递 数据必须实现 Send 特征，对于 async fn 也是如此，它返回的 Future 能否在线程间传递的关键在于 .await 运行过程中，作用域中的变量类型是否是 Send。
use std::rc::Rc;

#[derive(Default)]
struct NotSend(Rc<()>);

async fn bar2() {}
async fn foo2() {
    // 编译依然 ok
    // 即使 foo2 返回的 Future 是 Send，但是在它内部短暂的使用 NotSend 依然是安全的，原因在于它的作用域并没有影响到 .await
    NotSend::default();
    bar2().await;
}

fn require_send(_: impl Send) {}

fn main2() {
    require_send(foo());
}

/*
报错：.await在运行时处于 x 的作用域内。在之前章节有提到过， .await 有可能被执行器调度到另一个线程上运行，而 Rc 并没有实现 Send，因此编译器无情拒绝了咱们。
async fn foo() {
    let x = NotSend::default();
    bar().await;
}

解决：
以将变量声明在语句块内，当语句块结束时，变量会自动被 drop，这个规则可以帮助我们解决很多借用冲突问题，特别是在 NLL 出来之前。
** 在 .await 之前就使用 std::mem::drop 释放掉 Rc 依然不行 （以后可能会支持）**
async fn foo() {
    {
        let x = NotSend::default();
    }
    bar().await;
}
 */

/// * 递归使用async fn
// 在内部实现中，async fn被编译成一个状态机，这会导致递归使用 async fn 变得较为复杂， 因为编译后的状态机还需要包含自身。

// foo函数:
async fn foo() {
    step_one().await;
    step_two().await;
}
//
// 会被编译成类似下面的类型：
enum Foo {
    First(StepOne),
    Second(StepTwo),
}

// 因此recursive函数: 无法编译 an `async fn` cannot invoke itself directly
// a recursive `async fn` must be rewritten to return a boxed future.
async fn recursive() {
    recursive().await;
    recursive().await;
}

// 会生成类似以下的类型
enum Recursive {
    First(Recursive),
    Second(Recursive),
}
// 这是典型的动态大小类型，它的大小会无限增长，因此编译器会直接报错(递归调用 无法计算大小):

// 要将其使用 Box 放到堆上而不是栈上，就可以解决
// 就算是使用 Box，这里也大有讲究。如果我们试图使用 Box::pin 这种方式去包裹是不行的，因为编译器自身的限制限制了我们。
// 为了解决这种问题，我们只能将 recursive 转变成一个正常的函数，该函数返回一个使用 Box 包裹的 async 语句块：
use futures::future::{BoxFuture, FutureExt};

fn recursive2() -> BoxFuture<'static, ()> {
    async move {
        recursive2().await;
        recursive2().await;
    }.boxed()
}

/// * 在特征trait中使用 async
// 无法在特征中定义 async fn 函数，不过也不用担心，目前已经有计划在未来移除这个限制了。
// 使用 async-trait 解决这个问题:
// https://github.com/dtolnay/async-trait
// * 不过使用该包并不是免费的，每一次特征中的async函数被调用时，都会产生一次堆内存分配。对于大多数场景，这个性能开销都可以接受，但是当函数一秒调用几十万、几百万次时，就得小心这块儿代码的性能了
use async_trait::async_trait;

#[async_trait]
trait Advertisement {
    async fn run(&self);
}

struct Modal;

#[async_trait]
impl Advertisement for Modal {
    async fn run(&self) {
        self.render_fullscreen().await;
        for _ in 0..4u16 {
            remind_user_to_join_mailing_list().await;
        }
        self.hide_for_now().await;
    }
}

struct AutoplayingVideo {
    media_url: String,
}

#[async_trait]
impl Advertisement for AutoplayingVideo {
    async fn run(&self) {
        let stream = connect(&self.media_url).await;
        stream.play().await;

        // 用视频说服用户加入我们的邮件列表
        Modal.run().await;
    }
}