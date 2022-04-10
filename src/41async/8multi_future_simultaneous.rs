///* 同时运行多个 Future
/// * join!
// futures 包中提供了很多实用的工具，其中一个就是 join!宏， 它允许我们同时等待多个不同 Future 的完成，且可以并发地运行这些 Future。
async fn enjoy_book_and_music() -> (Book, Music) {
    let book = enjoy_book().await;
    let music = enjoy_music().await;
    (book, music)
}
// WRONG -- 别这么做
async fn enjoy_book_and_music2() -> (Book, Music) {
    let book_future = enjoy_book();
    let music_future = enjoy_music();
    // 依然是串行
    (book_future.await, music_future.await)
}
//
// OK
use futures::join;
async fn enjoy_book_and_music3() -> (Book, Music) {
    let book_fut = enjoy_book();
    let music_fut = enjoy_music();
    join!(book_fut, music_fut)
}

/// * try_join!
// 由于join!必须等待它管理的所有 Future 完成后才能完成，如果你希望在某一个 Future 报错后就立即停止所有 Future 的执行，可以使用 try_join!，特别是当 Future 返回 Result 时:
use futures::try_join;

async fn get_book() -> Result<Book, String> {
    /* ... */
    Ok(Book)
}
async fn get_music() -> Result<Music, String> {
    /* ... */
    Ok(Music)
}

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book();
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}
// * 有一点需要注意，传给 try_join! 的所有 Future 都必须拥有相同的错误类型。
// 如果错误类型不同，可以考虑使用来自 futures::future::TryFutureExt 模块的 map_err和err_info方法将错误进行转换:
use futures::{future::TryFutureExt, try_join};

async fn get_book1() -> Result<Book, ()> {
    /* ... */
    Ok(Book)
}
async fn get_music1() -> Result<Music, String> {
    /* ... */
    Ok(Music)
}

async fn get_book_and_music2() -> Result<(Book, Music), String> {
    // 转换 错误类型
    let book_fut = get_book1().map_err(|()| "Unable to get book".to_string());
    let music_fut = get_music1();
    try_join!(book_fut, music_fut)
}

/// * select!
// 如果你想同时等待多个 Future ，且任何一个 Future 结束后，都可以立即被处理，可以考虑使用 futures::select!:
use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};

async fn task_one() { /* ... */
}
async fn task_two() { /* ... */
}

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    // 论两者哪个先完成，都会调用对应的 println! 打印相应的输出，
    // 然后函数结束且不会等待另一个任务的完成。
    select! {
        () = t1 => println!("任务1率先完成"),
        () = t2 => println!("任务2率先完成"),
    }
}

// * default 和 complete
// select!还支持 default 和 complete 分支:
// - complete 分支当所有的 Future 和 Stream 完成后才会被执行，它往往配合loop使用，loop用于循环完成所有的 Future
// - default分支，若没有任何 Future 或 Stream 处于 Ready 状态， 则该分支会被立即执行
use futures::future;
use futures::select;
pub fn main() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => panic!(), // 永远不会被执行(futures都准备好了,然后complete分支被执行)
        };
    }
    assert_eq!(total, 10);
}

/// * 跟 Unpin 和 FusedFuture 进行交互
// 首先，.fuse()方法可以让 Future 实现 FusedFuture 特征，
// 而 pin_mut! 宏会为 Future 实现 Unpin特征，这两个特征恰恰是使用 select 所必须的:
// - Unpin，由于 select 不会通过拿走所有权的方式使用Future，而是通过可变引用的方式去使用，这样当 select 结束后，该 Future 若没有被完成，它的所有权还可以继续被其它代码使用。
// - FusedFuture的原因跟上面类似，当 Future 一旦完成后，那 select 就不能再对其进行轮询使用。Fuse意味着熔断，相当于 Future 一旦完成，再次调用poll会直接返回Poll::Pending。
//
// 只有实现了FusedFuture，select 才能配合 loop 一起使用。假如没有实现，就算一个 Future 已经完成了，它依然会被 select 不停的轮询执行。
//
// * Stream 稍有不同，它们使用的特征是 FusedStream。 通过.fuse()(也可以手动实现)实现了该特征的 Stream，对其调用.next() 或 .try_next()方法可以获取实现了FusedFuture特征的Future:
use futures::{
    select,
    stream::{FusedStream, Stream, StreamExt},
};

async fn add_two_streams(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    total
}

/// * 在select循环中并发
// 一个很实用但又鲜为人知的函数是 Fuse::terminated() ，可以使用它构建一个空的 Future ，空自然没啥用，但是如果它能在后面再被填充呢？
//
// 考虑以下场景：当你要在select循环中运行一个任务，但是该任务却是在select循环内部创建时，上面的函数就非常好用了。
use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
    stream::{FusedStream, Stream, StreamExt},
};

async fn get_new_num() -> u8 {
    /* ... */
    5
}

async fn run_on_new_num(_: u8) { /* ... */
}

async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(run_on_new_num_fut, get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // 定时器已结束，若`get_new_num_fut`没有在运行，就创建一个新的
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                // 收到新的数字 -- 创建一个新的`run_on_new_num_fut`并丢弃掉旧的
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            // 运行 `run_on_new_num_fut`
            () = run_on_new_num_fut => {},
            // 若所有任务都完成，直接 `panic`， 原因是 `interval_timer` 应该连续不断的产生值，而不是结束
            //后，执行到 `complete` 分支
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}
// 当某个 Future 有多个拷贝都需要同时运行时，可以使用 FuturesUnordered 类型。
// 下面的例子跟上个例子大体相似，但是它会将 run_on_new_num_fut 的每一个拷贝都运行到完成，而不是像之前那样一旦创建新的就终止旧的。
use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    pin_mut, select,
    stream::{FusedStream, FuturesUnordered, Stream, StreamExt},
};

async fn get_new_num2() -> u8 {
    /* ... */
    5
}

async fn run_on_new_num2(_: u8) -> u8 {
    /* ... */
    5
}

// 使用从 `get_new_num` 获取的最新数字 来运行 `run_on_new_num`
//
// 每当计时器结束后，`get_new_num` 就会运行一次，它会立即取消当前正在运行的`run_on_new_num` ,
// 并且使用新返回的值来替换
async fn run_loop2(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    // 容器
    let mut run_on_new_num_futs = FuturesUnordered::new();
    run_on_new_num_futs.push(run_on_new_num2(starting_num));
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                 // 定时器已结束，若`get_new_num_fut`没有在运行，就创建一个新的
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num2().fuse());
                }
            },
            new_num = get_new_num_fut => {
                 // 收到新的数字 -- 创建一个新的`run_on_new_num_fut` (并没有像之前的例子那样丢弃掉旧值)
                run_on_new_num_futs.push(run_on_new_num(new_num));
            },
            // 运行 `run_on_new_num_futs`, 并检查是否有已经完成的
            res = run_on_new_num_futs.select_next_some() => {
                println!("run_on_new_num_fut returned {:?}", res);
            },
            // 若所有任务都完成，直接 `panic`， 原因是 `interval_timer` 应该连续不断的产生值，而不是结束
            //后，执行到 `complete` 分支
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}
