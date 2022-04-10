/// * Stream流处理
// Stream 特征类似于 Future 特征，但是前者在完成前可以生成多个值，这种行为跟标准库中的 Iterator 特征倒是颇为相似。
trait Stream {
    // Stream生成的值的类型
    type Item;

    // 尝试去解析Stream中的下一个值,
    // 若无数据，返回`Poll::Pending`, 若有数据，返回 `Poll::Ready(Some(x))`, `Stream`完成则返回 `Poll::Ready(None)`
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}
// 关于 Stream 的一个常见例子是消息通道（ futures 包中的）的消费者 Receiver。每次有消息从 Send 端发送后，它都可以接收到一个 Some(val) 值， 一旦 Send 端关闭(drop)，且消息通道中没有消息后，它会接收到一个 None 值。
async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // `StreamExt::next` 类似于 `Iterator::next`, 但是前者返回的不是值，而是一个 `Future<Output = Option<T>>`，
    // 因此还需要使用`.await`来获取具体的值
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}

///* 迭代和并发
// - 跟迭代器类似，我们也可以迭代一个 Stream。 例如使用map，filter，fold方法
// - 以及它们的遇到错误提前返回的版本： try_map，try_filter，try_fold。
// - 但是跟迭代器又有所不同，for 循环无法在这里使用
// - 但是命令式风格的循环while let是可以用的，同时还可以使用next 和 try_next 方法:
async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt; // 引入 next
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}

// 如果返回的类型是 Result<T, Error>，使用 await? 代码错误退出
async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt; // 引入 try_next
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    Ok(sum)
}
// * 上面代码是一次处理一个值的模式，但是需要注意的是：如果你选择一次处理一个值的模式，可能会造成无法并发，这就失去了异步编程的意义
// * 因此，如果可以的话我们还是要选择从一个 Stream 并发处理多个值的方式，通过 for_each_concurrent 或 try_for_each_concurrent 方法来实现:
async fn jump_around(
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt; // 引入 `try_for_each_concurrent`
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    stream
        .try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
            jump_n_times(num).await?;
            report_n_jumps(num).await?;
            Ok(())
        })
        .await?;

    Ok(())
}
