// Fearless Concurrency
// Handling concurrent programming safely and efficiently is another of Rust’s major goals.
// 安全高效地处理并发编程是 Rust 的另一个主要目标。
// Concurrent programming, where different parts of a program execute independently,
// 并发编程，程序的不同部分独立执行，
//  and parallel programming, where different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors.
//  和并行编程（程序的不同部分同时执行）随着越来越多的计算机利用其多处理器而变得越来越重要。
// Historically, programming in these contexts has been difficult and error prone: Rust hopes to change that.
// 从历史上看，在这些上下文中编程一直很困难且容易出错：Rust 希望改变这一点。

// Initially, the Rust team thought that ensuring memory safety and preventing concurrency problems were two separate challenges to be solved with different methods.
// 最初，Rust 团队认为确保内存安全和防止并发问题是两个不同的挑战，需要用不同的方法来解决。
// Over time, the team discovered that the ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems!
// 随着时间的推移，团队发现所有权和类型系统是一套强大的工具，可以帮助管理内存安全和并发问题！
// By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors.
// 通过利用所有权和类型检查，许多并发错误是 Rust 中的编译时错误而不是运行时错误。
// Therefore, rather than making you spend lots of time trying to reproduce the exact circumstances under which a runtime concurrency bug occurs,
// 因此，与其让您花费大量时间尝试重现发生运行时并发错误的确切情况，不如
//  incorrect code will refuse to compile and present an error explaining the problem.
//  不正确的代码将拒绝编译并显示一个解释问题的错误。
// As a result, you can fix your code while you’re working on it rather than potentially after it has been shipped to production.
// 因此，您可以在处理代码时修复代码，而不是在将代码交付到生产环境后进行修复。
// We’ve nicknamed this aspect of Rust fearless concurrency.
// 我们给 Rust 的这一方面起了绰号：无畏并发。
// Fearless concurrency allows you to write code that is free of subtle bugs and is easy to refactor without introducing new bugs.
// Fearless concurrency 允许您编写没有细微错误的代码，并且在不引入新错误的情况下易于重构。

// Note: For simplicity’s sake, we’ll refer to many of the problems as concurrent rather than being more precise by saying concurrent and/or parallel.
// 注意：为了简单起见，我们将许多问题称为并发，而不是通过并发和/或并行来更精确。
// If this book were about concurrency and/or parallelism, we’d be more specific.
// 如果这本书是关于并发和/或并行的，我们会更具体。
// For this chapter, please mentally substitute concurrent and/or parallel whenever we use concurrent.
// 对于本章，每当我们使用并发时，请在心里用并发和/或并行代替。

// Many languages are dogmatic about the solutions they offer for handling concurrent problems.
// 许多语言对于它们提供的处理并发问题的解决方案都是教条式的。
// For example, Erlang has elegant functionality for message-passing concurrency but has only obscure ways to share state between threads.
// 例如，Erlang 具有优雅的消息传递并发功能，但只有模糊的方式来在线程之间共享状态。
// Supporting only a subset of possible solutions is a reasonable strategy for higher-level languages,
// 仅支持可能解决方案的子集是高级语言的合理策略，
//  because a higher-level language promises benefits from giving up some control to gain abstractions.
// 因为高级语言承诺放弃一些控制以获得抽象。
// However, lower-level languages are expected to provide the solution with the best performance in any given situation and have fewer abstractions over the hardware.
// 但是，较低级别的语言有望在任何给定情况下提供具有最佳性能的解决方案，并且对硬件的抽象较少。
// Therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situation and requirements.
// 因此，Rust 提供了多种工具来以适合您的情况和要求的任何方式对问题进行建模。

// Here are the topics we’ll cover in this chapter:
// 以下是我们将在本章中介绍的主题：

// How to create threads to run multiple pieces of code at the same time
// 如何创建线程同时运行多段代码
// Message-passing concurrency, where channels send messages between threads
// 消息传递并发，通道在线程之间发送消息
// Shared-state concurrency, where multiple threads have access to some piece of data
// 共享状态并发，其中多个线程可以访问一些数据
// The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library
// Sync 和 Send 特性，将 Rust 的并发保证扩展到用户定义的类型以及标准库提供的类型


// Using Threads to Run Code Simultaneously
// In most current operating systems, an executed program’s code is run in a process, and the operating system will manage multiple processes at once.
// Within a program, you can also have independent parts that run simultaneously.
// The features that run these independent parts are called threads. For example, a web server could have multiple threads so that it could respond to more than one request at the same time.

// Splitting the computation in your program into multiple threads to run multiple tasks at the same time can improve performance, but it also adds complexity.
// Because threads can run simultaneously, there’s no inherent guarantee about the order in which parts of your code on different threads will run.
// This can lead to problems, such as:

// Race conditions, where threads are accessing data or resources in an inconsistent order
// Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
// Bugs that happen only in certain situations and are hard to reproduce and fix reliably

// Rust attempts to mitigate the negative effects of using threads,
//  but programming in a multithreaded context still takes careful thought and requires a code structure that is different from that in programs running in a single thread.

// Programming languages implement threads in a few different ways, and many operating systems provide an API the language can call for creating new threads.
// The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread.
// There are crates that implement other models of threading that make different tradeoffs to the 1:1 model.

// Creating a New Thread with spawn
// To create a new thread, we call the thread::spawn function and pass it a closure (we talked about closures in Chapter 13) containing the code we want to run in the new thread.
// The example in Listing 16-1 prints some text from a main thread and other text from a new thread:

// Filename: src/main.rs

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// Listing 16-1: Creating a new thread to print one thing while the main thread prints something else

// Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running.
// The output from this program might be a little different every time, but it will look similar to the following:

// hi number 1 from the main thread!
// hi number 1 from the spawned thread!
// hi number 2 from the main thread!
// hi number 2 from the spawned thread!
// hi number 3 from the main thread!
// hi number 3 from the spawned thread!
// hi number 4 from the main thread!
// hi number 4 from the spawned thread!
// hi number 5 from the spawned thread!

// The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run.
// The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads.
// In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code.
// And even though we told the spawned thread to print until i is 9, it only got to 5 before the main thread shut down.

// If you run this code and only see output from the main thread, or don’t see any overlap,
//  try increasing the numbers in the ranges to create more opportunities for the operating system to switch between the threads.

// Waiting for All Threads to Finish Using join Handles
// The code in Listing 16-1 not only stops the spawned thread prematurely most of the time due to the main thread ending,
//  but because there is no guarantee on the order in which threads run, we also can’t guarantee that the spawned thread will get to run at all!

// We can fix the problem of the spawned thread not running or ending prematurely by saving the return value of thread::spawn in a variable.
// The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.
// Listing 16-2 shows how to use the JoinHandle of the thread we created in Listing 16-1 and call join to make sure the spawned thread finishes before main exits:

// Filename: src/main.rs

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// Listing 16-2: Saving a JoinHandle from thread::spawn to guarantee the thread is run to completion

// Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.
// Blocking a thread means that thread is prevented from performing work or exiting.
// Because we’ve put the call to join after the main thread’s for loop, running Listing 16-2 should produce output similar to this:

// hi number 1 from the main thread!
// hi number 2 from the main thread!
// hi number 1 from the spawned thread!
// hi number 3 from the main thread!
// hi number 2 from the spawned thread!
// hi number 4 from the main thread!
// hi number 3 from the spawned thread!
// hi number 4 from the spawned thread!
// hi number 5 from the spawned thread!
// hi number 6 from the spawned thread!
// hi number 7 from the spawned thread!
// hi number 8 from the spawned thread!
// hi number 9 from the spawned thread!

// The two threads continue alternating, but the main thread waits because of the call to handle.join() and does not end until the spawned thread is finished.

// But let’s see what happens when we instead move handle.join() before the for loop in main, like this:

// Filename: src/main.rs

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// The main thread will wait for the spawned thread to finish and then run its for loop, so the output won’t be interleaved anymore, as shown here:

// hi number 1 from the spawned thread!
// hi number 2 from the spawned thread!
// hi number 3 from the spawned thread!
// hi number 4 from the spawned thread!
// hi number 5 from the spawned thread!
// hi number 6 from the spawned thread!
// hi number 7 from the spawned thread!
// hi number 8 from the spawned thread!
// hi number 9 from the spawned thread!
// hi number 1 from the main thread!
// hi number 2 from the main thread!
// hi number 3 from the main thread!
// hi number 4 from the main thread!

// Small details, such as where join is called, can affect whether or not your threads run at the same time.

// Using move Closures with Threads
// We'll often use the move keyword with closures passed to thread::spawn because the closure will then take ownership of the values it uses from the environment,
//  thus transferring ownership of those values from one thread to another.
// In the “Capturing References or Moving Ownership” section of Chapter 13, we discussed move in the context of closures.
// Now, we’ll concentrate more on the interaction between move and thread::spawn.

// Notice in Listing 16-1 that the closure we pass to thread::spawn takes no arguments: we’re not using any data from the main thread in the spawned thread’s code.
// To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it needs.
// Listing 16-3 shows an attempt to create a vector in the main thread and use it in the spawned thread.
// However, this won’t yet work, as you’ll see in a moment.

// Filename: src/main.rs

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// Listing 16-3: Attempting to use a vector created by the main thread in another thread

// The closure uses v, so it will capture v and make it part of the closure’s environment.
// Because thread::spawn runs this closure in a new thread, we should be able to access v inside that new thread.
// But when we compile this example, we get the following error:

// $ cargo run
//    Compiling threads v0.1.0 (file:///projects/threads)
// error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
//  --> src/main.rs:6:32
//   |
// 6 |     let handle = thread::spawn(|| {
//   |                                ^^ may outlive borrowed value `v`
// 7 |         println!("Here's a vector: {:?}", v);
//   |                                           - `v` is borrowed here
//   |
// note: function requires argument type to outlive `'static`
//  --> src/main.rs:6:18
//   |
// 6 |       let handle = thread::spawn(|| {
//   |  __________________^
// 7 | |         println!("Here's a vector: {:?}", v);
// 8 | |     });
//   | |______^
// help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
//   |
// 6 |     let handle = thread::spawn(move || {
//   |                                ++++
//
// For more information about this error, try `rustc --explain E0373`.
// error: could not compile `threads` due to previous error

// Rust infers how to capture v, and because println! only needs a reference to v, the closure tries to borrow v.
// However, there’s a problem: Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid.

// Listing 16-4 provides a scenario that’s more likely to have a reference to v that won’t be valid:

// Filename: src/main.rs

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}

// Listing 16-4: A thread with a closure that attempts to capture a reference to v from a main thread that drops v

// If Rust allowed us to run this code, there’s a possibility the spawned thread would be immediately put in the background without running at all.
// The spawned thread has a reference to v inside, but the main thread immediately drops v, using the drop function we discussed in Chapter 15.
// Then, when the spawned thread starts to execute, v is no longer valid, so a reference to it is also invalid. Oh no!

// To fix the compiler error in Listing 16-3, we can use the error message’s advice:

// help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
//   |
// 6 |     let handle = thread::spawn(move || {
//   |                                ++++

// By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.
// The modification to Listing 16-3 shown in Listing 16-5 will compile and run as we intend:

// Filename: src/main.rs

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// Listing 16-5: Using the move keyword to force a closure to take ownership of the values it uses

// We might be tempted to try the same thing to fix the code in Listing 16-4 where the main thread called drop by using a move closure.
// However, this fix will not work because what Listing 16-4 is trying to do is disallowed for a different reason.
// If we added move to the closure, we would move v into the closure’s environment, and we could no longer call drop on it in the main thread.
// We would get this compiler error instead:

// $ cargo run
//    Compiling threads v0.1.0 (file:///projects/threads)
// error[E0382]: use of moved value: `v`
//   --> src/main.rs:10:10
//    |
// 4  |     let v = vec![1, 2, 3];
//    |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
// 5  |
// 6  |     let handle = thread::spawn(move || {
//    |                                ------- value moved into closure here
// 7  |         println!("Here's a vector: {:?}", v);
//    |                                           - variable moved due to use in closure
// ...
// 10 |     drop(v); // oh no!
//    |          ^ value used here after move
//
// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `threads` due to previous error

// Rust’s ownership rules have saved us again!
// We got an error from the code in Listing 16-3 because Rust was being conservative and only borrowing v for the thread,
//  which meant the main thread could theoretically invalidate the spawned thread’s reference.
// By telling Rust to move ownership of v to the spawned thread, we’re guaranteeing Rust that the main thread won’t use v anymore.
// If we change Listing 16-4 in the same way, we’re then violating the ownership rules when we try to use v in the main thread.
// The move keyword overrides Rust’s conservative default of borrowing; it doesn’t let us violate the ownership rules.

// With a basic understanding of threads and the thread API, let’s look at what we can do with threads.


// Using Message Passing to Transfer Data Between Threads
// 使用消息传递在线程间传输数据
// One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data.
// 一种越来越流行的确保安全并发的方法是消息传递，线程或参与者通过相互发送包含数据的消息来进行通信。
// Here’s the idea in a slogan from the Go language documentation: “Do not communicate by sharing memory; instead, share memory by communicating.”
// 这是 Go 语言文档中的一句口号：“不要通过共享内存进行通信； 相反，通过交流来分享记忆。”

// To accomplish message-sending concurrency, Rust's standard library provides an implementation of channels.
// 为了实现消息发送并发，Rust 的标准库提供了通道的实现。
// A channel is a general programming concept by which data is sent from one thread to another.
// 通道是一种通用的编程概念，数据通过它从一个线程发送到另一个线程。

// You can imagine a channel in programming as being like a directional channel of water, such as a stream or a river.
// 你可以把编程中的通道想象成一条有方向的水道，比如小溪或河流。
// If you put something like a rubber duck into a river, it will travel downstream to the end of the waterway.
// 如果将橡皮鸭之类的东西放入河中，它会顺流而下到达水道的尽头。

// A channel has two halves: a transmitter and a receiver.
// 通道有两半：发送器和接收器。
// The transmitter half is the upstream location where you put rubber ducks into the river, and the receiver half is where the rubber duck ends up downstream.
// 发射器的一半是你将橡皮鸭放入河中的上游位置，而接收器的一半是橡皮鸭最终到达下游的地方。
// One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages.
// 代码的一部分使用要发送的数据调用发送器上的方法，另一部分检查接收端是否有到达的消息。
// A channel is said to be closed if either the transmitter or receiver half is dropped.
// 如果发射器或接收器的一半被丢弃，则称通道已关闭。

// Here, we’ll work up to a program that has one thread to generate values and send them down a channel, and another thread that will receive the values and print them out.
// 在这里，我们将编写一个程序，该程序有一个线程生成值并将它们发送到通道，另一个线程将接收值并将它们打印出来。
// We’ll be sending simple values between threads using a channel to illustrate the feature.
// 我们将使用通道在线程之间发送简单的值来说明该功能。
// Once you’re familiar with the technique, you could use channels for any threads that need to communicate between each other,
// 一旦你熟悉了这项技术，你就可以为任何需要相互通信的线程使用通道，
//  such as a chat system or a system where many threads perform parts of a calculation and send the parts to one thread that aggregates the results.
//  例如聊天系统或许多线程执行部分计算并将这些部分发送到聚合结果的线程的系统。

// First, in Listing 16-6, we’ll create a channel but not do anything with it.
// 首先，在示例 16-6 中，我们将创建一个通道但不对其进行任何操作。
// Note that this won’t compile yet because Rust can’t tell what type of values we want to send over the channel.
// 请注意，这还不能编译，因为 Rust 无法判断我们要通过通道发送什么类型的值。

// Filename: src/main.rs

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}

// Listing 16-6: Creating a channel and assigning the two halves to tx and rx
// 示例 16-6：创建一个通道并将两半分配给 tx 和 rx

// We create a new channel using the mpsc::channel function; mpsc stands for multiple producer, single consumer.
// 我们使用 mpsc::channel 函数创建一个新通道； mpsc 代表多生产者，单一消费者（multiple producer, single consumer）。
// In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.
// 简而言之，Rust 的标准库实现通道的方式意味着一个通道可以有多个发送端产生值，但只有一个接收端消费这些值。
// Imagine multiple streams flowing together into one big river: everything sent down any of the streams will end up in one river at the end.
// 想象多条溪流一起汇入一条大河：从任何一条溪流流下的所有东西最终都会汇入一条河流。
// We’ll start with a single producer for now, but we’ll add multiple producers when we get this example working.
// 我们现在从一个生产者开始，但是当我们让这个例子工作时，我们将添加多个生产者。

// The mpsc::channel function returns a tuple, the first element of which is the sending end--the transmitter--and the second element is the receiving end--the receiver.
// mpsc::channel 函数返回一个元组，元组的第一个元素是发送端--发送器--第二个元素是接收端--接收器。
// The abbreviations tx and rx are traditionally used in many fields for transmitter and receiver respectively, so we name our variables as such to indicate each end.
// 缩写 tx 和 rx 传统上分别用于发送器和接收器（in many fields），因此我们这样命名变量以指示每一端。
// We’re using a let statement with a pattern that destructures the tuples; we’ll discuss the use of patterns in let statements and destructuring in Chapter 18.
// 我们正在使用一个带有解构元组模式的 let 语句； 我们将在第 18 章讨论模式在 let 语句和解构中的使用。
// For now, know that using a let statement this way is a convenient approach to extract the pieces of the tuple returned by mpsc::channel.
// 现在，知道以这种方式使用 let 语句是提取 mpsc::channel 返回的元组片段的便捷方法。

// Let’s move the transmitting end into a spawned thread and have it send one string so the spawned thread is communicating with the main thread, as shown in Listing 16-7.
// 让我们将发送端移到衍生线程中并让它发送一个字符串，以便衍生线程与主线程通信，如清单 16-7 所示。
// This is like putting a rubber duck in the river upstream or sending a chat message from one thread to another.
// 这就像把一只橡皮鸭放到河流的上游或从一个线程向另一个线程发送聊天消息。

// Filename: src/main.rs

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}

// Listing 16-7: Moving tx to a spawned thread and sending “hi”
// 示例 16-7：将 tx 移动到生成的线程并发送“hi”

// Again, we’re using thread::spawn to create a new thread and then using move to move tx into the closure so the spawned thread owns tx.
// 同样，我们使用 thread::spawn 创建一个新线程，然后使用 move 将 tx 移动到闭包中，因此生成的线程拥有 tx。
// The spawned thread needs to own the transmitter to be able to send messages through the channel.
// 生成的线程需要拥有发送器才能通过通道发送消息。
// The transmitter has a send method that takes the value we want to send.
// 发送器有一个发送方法，它接受我们要发送的值。
// The send method returns a Result<T, E> type, so if the receiver has already been dropped and there’s nowhere to send a value, the send operation will return an error.
// send 方法返回一个 Result<T, E> 类型，所以如果接收器已经被丢弃并且没有地方可以发送值，发送操作将返回一个错误。
// In this example, we’re calling unwrap to panic in case of an error.
// 在这个例子中，我们调用 unwrap 以在出现错误时恐慌。
// But in a real application, we would handle it properly: return to Chapter 9 to review strategies for proper error handling.
// 但在实际应用中，我们会正确处理它：返回第 9 章回顾正确错误处理的策略。

// In Listing 16-8, we’ll get the value from the receiver in the main thread.
// 在示例 16-8 中，我们将从主线程中的接收器获取值。
// This is like retrieving the rubber duck from the water at the end of the river or receiving a chat message.
// 这就像在河的尽头从水中取回橡皮鸭或收到一条聊天消息。

// Filename: src/main.rs

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// Listing 16-8: Receiving the value “hi” in the main thread and printing it
// 示例 16-8：在主线程中接收值“hi”并打印它

// The receiver has two useful methods: recv and try_recv.
// 接收器有两个有用的方法：recv 和 try_recv。
// We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel.
// 我们正在使用 recv，receive 的缩写，它将阻塞主线程的执行并等待直到一个值被发送到通道中。
// Once a value is sent, recv will return it in a Result<T, E>.
// 一旦发送了一个值，recv 将在 Result<T, E> 中返回它。
// When the transmitter closes, recv will return an error to signal that no more values will be coming.
// 当发送器关闭时，recv 将返回一个错误，表示不会有更多的值到来。

// The try_recv method doesn’t block, but will instead return a Result<T, E> immediately:
// try_recv 方法不会阻塞，而是会立即返回一个 Result<T, E>：
//  an Ok value holding a message if one is available and an Err value if there aren’t any messages this time.
//  如果有一条消息可用，则为 Ok 值，如果这次没有任何消息，则为 Err 值。
// Using try_recv is useful if this thread has other work to do while waiting for messages:
// 如果此线程在等待消息时有其他工作要做，则使用 try_recv 很有用：
//  we could write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does other work for a little while until checking again.
//  我们可以编写一个循环，每隔一段时间调用 try_recv，如果消息可用则处理一条消息，否则做其他工作一段时间直到再次检查。

// We’ve used recv in this example for simplicity; we don’t have any other work for the main thread to do other than wait for messages, so blocking the main thread is appropriate.
// 为简单起见，我们在此示例中使用了 recv； 除了等待消息，我们没有任何其他工作需要主线程做，所以阻塞主线程是合适的。

// When we run the code in Listing 16-8, we’ll see the value printed from the main thread:
// 当我们运行示例 16-8 中的代码时，我们将看到从主线程打印的值：

Got: hi

// Perfect!

// Channels and Ownership Transference
// 通道和所有权转移
// The ownership rules play a vital role in message sending because they help you write safe, concurrent code.
// 所有权规则在消息发送中起着至关重要的作用，因为它们可以帮助您编写安全的并发代码。
// Preventing errors in concurrent programming is the advantage of thinking about ownership throughout your Rust programs.
// 防止并发编程中的错误是在整个 Rust 程序中考虑所有权的优势。
// Let’s do an experiment to show how channels and ownership work together to prevent problems: we’ll try to use a val value in the spawned thread after we’ve sent it down the channel.
// 让我们做一个实验来展示通道和所有权如何协同工作以防止出现问题：我们将在将其发送到通道后尝试在生成的线程中使用 val 值。
// Try compiling the code in Listing 16-9 to see why this code isn’t allowed:
// 尝试编译清单 16-9 中的代码，看看为什么不允许使用此代码：

// Filename: src/main.rs

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// Listing 16-9: Attempting to use val after we’ve sent it down the channel
// 示例 16-9：在将 val 发送到通道后尝试使用它

// Here, we try to print val after we’ve sent it down the channel via tx.send.
// 在这里，我们尝试在通过 tx.send 将其发送到通道后打印 val。
// Allowing this would be a bad idea: once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again.
// 允许这样做不是一个好主意：一旦值被发送到另一个线程，该线程可以在我们尝试再次使用该值之前修改或删除它。
// Potentially, the other thread’s modifications could cause errors or unexpected results due to inconsistent or nonexistent data.
// 其他线程的修改可能会由于数据不一致或不存在而导致错误或意外结果。
// However, Rust gives us an error if we try to compile the code in Listing 16-9:
// 但是，如果我们尝试编译示例 16-9 中的代码，Rust 会报错：

// $ cargo run
//    Compiling message-passing v0.1.0 (file:///projects/message-passing)
// error[E0382]: borrow of moved value: `val`
//   --> src/main.rs:10:31
//    |
// 8  |         let val = String::from("hi");
//    |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
// 9  |         tx.send(val).unwrap();
//    |                 --- value moved here
// 10 |         println!("val is {}", val);
//    |                               ^^^ value borrowed here after move
//    |
//    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
//
// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `message-passing` due to previous error

// Our concurrency mistake has caused a compile time error. The send function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it.
// 我们的并发错误导致了编译时错误。 发送函数获得其参数的所有权，当值被移动时，接收者获得它的所有权。
// This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.
// 这可以防止我们在发送后不小心再次使用该值； 所有权系统检查一切是否正常。

// Sending Multiple Values and Seeing the Receiver Waiting
// 发送多个值并看到接收者在等待
// The code in Listing 16-8 compiled and ran, but it didn’t clearly show us that two separate threads were talking to each other over the channel.
// 清单 16-8 中的代码编译并运行了，但它没有清楚地向我们展示两个独立的线程正在通过通道相互通信。
// In Listing 16-10 we’ve made some modifications that will prove the code in Listing 16-8 is running concurrently:
// 在示例 16-10 中，我们做了一些修改，以证明示例 16-8 中的代码正在并发运行：
//  the spawned thread will now send multiple messages and pause for a second between each message.
//  生成的线程现在将发送多条消息并在每条消息之间暂停一秒钟。

// Filename: src/main.rs

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// Listing 16-10: Sending multiple messages and pausing between each
// 示例 16-10：发送多条消息并在每条消息之间暂停

// This time, the spawned thread has a vector of strings that we want to send to the main thread.
// 这一次，生成的线程有一个我们要发送到主线程的字符串向量。
// We iterate over them, sending each individually, and pause between each by calling the thread::sleep function with a Duration value of 1 second.
// 我们遍历它们，单独发送每个，并通过调用 thread::sleep 函数在每个之间暂停，持续时间值为 1 秒。

// In the main thread, we’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator.
// 在主线程中，我们不再显式调用 recv 函数：相反，我们将 rx 视为迭代器。
// For each value received, we’re printing it. When the channel is closed, iteration will end.
// 对于接收到的每个值，我们都将其打印出来。 当通道关闭时，迭代将结束。

// When running the code in Listing 16-10, you should see the following output with a 1-second pause in between each line:
// 当运行清单 16-10 中的代码时，您应该看到以下输出，每行之间有 1 秒的停顿：

Got: hi
Got: from
Got: the
Got: thread

// Because we don’t have any code that pauses or delays in the for loop in the main thread, we can tell that the main thread is waiting to receive values from the spawned thread.
// 因为我们在主线程的 for 循环中没有任何暂停或延迟的代码，所以我们可以知道主线程正在等待从派生线程接收值。

// Creating Multiple Producers by Cloning the Transmitter
// 通过克隆传输器创建多个生产者
// Earlier we mentioned that mpsc was an acronym for multiple producer, single consumer.
// 前面我们提到 mpsc 是 multiple producer, single consumer 的缩写。
// Let’s put mpsc to use and expand the code in Listing 16-10 to create multiple threads that all send values to the same receiver.
// 让我们使用 mpsc 并扩展清单 16-10 中的代码来创建多个线程，这些线程都将值发送到同一个接收器。
// We can do so by cloning the transmitter, as shown in Listing 16-11:
// 我们可以通过克隆发送器来做到这一点，如清单 16-11 所示：

// Filename: src/main.rs

    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --snip--

// Listing 16-11: Sending multiple messages from multiple producers
// 示例 16-11：从多个生产者发送多条消息

// This time, before we create the first spawned thread, we call clone on the transmitter.
// 这一次，在我们创建第一个派生线程之前，我们在发送器上调用克隆。
// This will give us a new transmitter we can pass to the first spawned thread.
// 这将为我们提供一个新的发送器，我们可以将其传递给第一个生成的线程。
// We pass the original transmitter to a second spawned thread.
// 我们将原始发送器传递给第二个生成的线程。
// This gives us two threads, each sending different messages to the one receiver.
// 这给了我们两个线程，每个线程向一个接收者发送不同的消息。

// When you run the code, your output should look something like this:
// 运行代码时，输出应如下所示：

Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you

// You might see the values in another order, depending on your system.
// 您可能会看到其他顺序的值，具体取决于您的系统。
// This is what makes concurrency interesting as well as difficult.
// 这就是使并发变得既有趣又困难的原因。
// If you experiment with thread::sleep, giving it various values in the different threads, each run will be more nondeterministic and create different output each time.
// 如果你试验 thread::sleep，在不同的线程中给它不同的值，每次运行都会更加不确定，每次都会产生不同的输出。

// Now that we’ve looked at how channels work, let’s look at a different method of concurrency.
// 现在我们已经了解了通道是如何工作的，让我们看看另一种不同的并发方法。


// Shared-State Concurrency
// 共享状态并发
// Message passing is a fine way of handling concurrency, but it’s not the only one. Another method would be for multiple threads to access the same shared data.
// 消息传递是处理并发的一种很好的方式，但它不是唯一的方式。 另一种方法是让多个线程访问相同的共享数据。
// Consider this part of the slogan from the Go language documentation again: “do not communicate by sharing memory.”
// 再次考虑 Go 语言文档中的这部分标语：“不要通过共享内存进行通信。”

// What would communicating by sharing memory look like? In addition, why would message-passing enthusiasts caution not to use memory sharing?
// 通过共享内存进行通信会是什么样子？ 此外，为什么消息传递爱好者会警告不要使用内存共享？

// In a way, channels in any programming language are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value.
// 在某种程度上，任何编程语言中的通道都类似于单一所有权，因为一旦你将一个值传递到通道中，你就不应再使用该值。
// Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.
// 共享内存并发就像多重所有权：多个线程可以同时访问同一个内存位置。
// As you saw in Chapter 15, where smart pointers made multiple ownership possible, multiple ownership can add complexity because these different owners need managing.
// 正如您在第 15 章中看到的，智能指针使多重所有权成为可能，多重所有权会增加复杂性，因为这些不同的所有者需要管理。
// Rust’s type system and ownership rules greatly assist in getting this management correct.
// Rust 的类型系统和所有权规则极大地帮助了这种管理的正确性。
// For an example, let’s look at mutexes, one of the more common concurrency primitives for shared memory.
// 例如，让我们看一下互斥锁，它是共享内存的一种更常见的并发原语。

// Using Mutexes to Allow Access to Data from One Thread at a Time
// 使用互斥量允许一次从一个线程访问数据
// Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time.
// Mutex 是 mutual exclusion 的缩写，互斥锁在任何给定时间只允许一个线程访问某些数据。
// To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock.
// 要访问互斥锁中的数据，线程必须首先通过请求获取互斥锁的锁来发出它想要访问的信号。
// The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.
// 锁是一种数据结构，它是互斥锁的一部分，用于跟踪谁当前对数据具有独占访问权限。
// Therefore, the mutex is described as guarding the data it holds via the locking system.
// 因此，互斥量被描述为通过锁定系统保护它所持有的数据。

// Mutexes have a reputation for being difficult to use because you have to remember two rules:
// 互斥锁以难以使用而著称，因为您必须记住两条规则：
// You must attempt to acquire the lock before using the data.
// 在使用数据之前必须尝试获取锁。
// When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
// 当您处理完互斥量保护的数据后，您必须解锁数据以便其他线程可以获得锁。

// For a real-world metaphor for a mutex, imagine a panel discussion at a conference with only one microphone.
// 对于互斥量的真实隐喻，想象一下在只有一个麦克风的会议上进行小组讨论。
// Before a panelist can speak, they have to ask or signal that they want to use the microphone.
// 在小组成员可以发言之前，他们必须先询问或示意他们想使用麦克风。
// When they get the microphone, they can talk for as long as they want to and then hand the microphone to the next panelist who requests to speak.
// 当他们拿到麦克风时，他们想说多久就说多久，然后将麦克风交给下一位要求发言的小组成员。
// If a panelist forgets to hand the microphone off when they’re finished with it, no one else is able to speak.
// 如果小组成员在完成麦克风后忘记将其移开，则其他人将无法发言。
// If management of the shared microphone goes wrong, the panel won’t work as planned!
// 如果共享麦克风管理出错，小组讨论将无法正常工作！

// Management of mutexes can be incredibly tricky to get right, which is why so many people are enthusiastic about channels.
// 互斥锁的管理非常棘手，这就是为什么这么多人热衷于通道的原因。
// However, thanks to Rust’s type system and ownership rules, you can’t get locking and unlocking wrong.
// 然而，由于 Rust 的类型系统和所有权规则，你不能错误地锁定和解锁。

// The API of Mutex<T>
// Mutex<T>的API
// As an example of how to use a mutex, let’s start by using a mutex in a single-threaded context, as shown in Listing 16-12:
// 作为如何使用互斥锁的示例，让我们从在单线程上下文中使用互斥锁开始，如示例 16-12 所示：

// Filename: src/main.rs

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// Listing 16-12: Exploring the API of Mutex<T> in a single-threaded context for simplicity
// 示例 16-12：为简单起见，在单线程上下文中探索 Mutex<T> 的 API

// As with many types, we create a Mutex<T> using the associated function new.
// 与许多类型一样，我们使用关联的函数 new 创建一个 Mutex<T>。
// To access the data inside the mutex, we use the lock method to acquire the lock.
// 要访问互斥体中的数据，我们使用 lock 方法来获取锁。
// This call will block the current thread so it can’t do any work until it’s our turn to have the lock.
// 此调用将阻塞当前线程，因此在轮到我们获取锁之前它无法执行任何操作。

// The call to lock would fail if another thread holding the lock panicked.
// 如果持有锁的另一个线程发生恐慌，则对锁的调用将失败。
// In that case, no one would ever be able to get the lock, so we’ve chosen to unwrap and have this thread panic if we’re in that situation.
// 在那种情况下，没有人能够获得锁，所以如果我们处于那种情况，我们选择解包并让这个线程恐慌。

// After we’ve acquired the lock, we can treat the return value, named num in this case, as a mutable reference to the data inside.
// 获得锁后，我们可以将返回值（在本例中名为 num）视为对内部数据的可变引用。
// The type system ensures that we acquire a lock before using the value in m.
// 类型系统确保我们在使用 m 中的值之前获取锁。
// The type of m is Mutex<i32>, not i32, so we must call lock to be able to use the i32 value.
// m 的类型是 Mutex<i32>，不是 i32，所以我们必须调用 lock 才能使用 i32 的值。
// We can’t forget; the type system won’t let us access the inner i32 otherwise.
// 我们不能忘记； 否则类型系统不允许我们访问内部 i32。

// As you might suspect, Mutex<T> is a smart pointer.
// 您可能会怀疑，Mutex<T> 是一个智能指针。
// More accurately, the call to lock returns a smart pointer called MutexGuard, wrapped in a LockResult that we handled with the call to unwrap.
// 更准确地说，对 lock 的调用返回一个名为 MutexGuard 的智能指针，它包装在我们通过调用 unwrap 处理的 LockResult 中。
// The MutexGuard smart pointer implements Deref to point at our inner data;
// MutexGuard 智能指针实现 Deref 指向我们的内部数据；
// the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope, which happens at the end of the inner scope.
// 智能指针还有一个 Drop 实现，当 MutexGuard 超出范围时自动释放锁，这发生在内部范围的末尾。
// As a result, we don’t risk forgetting to release the lock and blocking the mutex from being used by other threads, because the lock release happens automatically.
// 因此，我们不会冒险忘记释放锁并阻止互斥锁被其他线程使用，因为锁释放是自动发生的。

// After dropping the lock, we can print the mutex value and see that we were able to change the inner i32 to 6.
// 释放锁后，我们可以打印互斥量值并看到我们能够将内部 i32 更改为 6。

// Sharing a Mutex<T> Between Multiple Threads
// 在多个线程之间共享一个 Mutex<T>
// Now, let’s try to share a value between multiple threads using Mutex<T>.
// 现在，让我们尝试使用 Mutex<T> 在多个线程之间共享一个值。
// We’ll spin up 10 threads and have them each increment a counter value by 1, so the counter goes from 0 to 10.
// 我们将启动 10 个线程并让它们各自将计数器值递增 1，因此计数器从 0 变为 10。
// The next example in Listing 16-13 will have a compiler error, and we’ll use that error to learn more about using Mutex<T> and how Rust helps us use it correctly.
// 清单 16-13 中的下一个示例将出现编译器错误，我们将使用该错误来了解有关使用 Mutex<T> 以及 Rust 如何帮助我们正确使用它的更多信息。

// Filename: src/main.rs

use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Listing 16-13: Ten threads each increment a counter guarded by a Mutex<T>
// 示例 16-13：10 个线程各自递增一个由 Mutex<T> 保护的计数器

// We create a counter variable to hold an i32 inside a Mutex<T>, as we did in Listing 16-12.
// 我们创建了一个计数器变量来保存 Mutex<T> 中的 i32，就像我们在示例 16-12 中所做的那样。
// Next, we create 10 threads by iterating over a range of numbers.
// 接下来，我们通过迭代一系列数字来创建 10 个线程。
// We use thread::spawn and give all the threads the same closure:
// 我们使用 thread::spawn 并为所有线程提供相同的闭包：
//  one that moves the counter into the thread, acquires a lock on the Mutex<T> by calling the lock method, and then adds 1 to the value in the mutex.
//  将计数器移入线程，通过调用 lock 方法获取 Mutex<T> 上的锁，然后将 mutex 中的值加 1。
// When a thread finishes running its closure, num will go out of scope and release the lock so another thread can acquire it.
// 当一个线程结束运行它的闭包时，num 将超出范围并释放锁，以便另一个线程可以获取它。

// In the main thread, we collect all the join handles.
// 在主线程中，我们收集所有的连接句柄。
// Then, as we did in Listing 16-2, we call join on each handle to make sure all the threads finish.
// 然后，就像我们在清单 16-2 中所做的那样，我们在每个句柄上调用 join 以确保所有线程都完成。
// At that point, the main thread will acquire the lock and print the result of this program.
// 此时，主线程将获取锁并打印该程序的结果。

// We hinted that this example wouldn’t compile. Now let’s find out why!
// 我们暗示这个例子不会编译。 现在让我们找出原因！

// $ cargo run
//    Compiling shared-state v0.1.0 (file:///projects/shared-state)
// error[E0382]: use of moved value: `counter`
//   --> src/main.rs:9:36
//    |
// 5  |     let counter = Mutex::new(0);
//    |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
// ...
// 9  |         let handle = thread::spawn(move || {
//    |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
// 10 |             let mut num = counter.lock().unwrap();
//    |                           ------- use occurs due to use in closure
//
// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `shared-state` due to previous error

// The error message states that the counter value was moved in the previous iteration of the loop.
// 错误消息表明计数器值在循环的前一次迭代中被移动。
// Rust is telling us that we can’t move the ownership of lock counter into multiple threads.
// Rust 告诉我们不能将锁计数器的所有权转移到多个线程中。
// Let’s fix the compiler error with a multiple-ownership method we discussed in Chapter 15.
// 让我们用我们在第 15 章讨论过的多重所有权方法来修复编译器错误。

// Multiple Ownership with Multiple Threads
// 多线程的多重所有权
// In Chapter 15, we gave a value multiple owners by using the smart pointer Rc<T> to create a reference counted value.
// 在第 15 章中，我们通过使用智能指针 Rc<T> 来创建一个引用计数值，从而赋予一个值多个所有者。
// Let’s do the same here and see what happens. We’ll wrap the Mutex<T> in Rc<T> in Listing 16-14 and clone the Rc<T> before moving ownership to the thread.
// 让我们在这里做同样的事情，看看会发生什么。 在示例 16-14 中，我们将 Mutex<T> 包装在 Rc<T> 中，并在将所有权移至线程之前克隆 Rc<T>。

// Filename: src/main.rs

use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Listing 16-14: Attempting to use Rc<T> to allow multiple threads to own the Mutex<T>
// 示例 16-14：尝试使用 Rc<T> 允许多个线程拥有 Mutex<T>

// Once again, we compile and get... different errors! The compiler is teaching us a lot.
// 再一次，我们编译并得到...不同的错误！ 编译器教会了我们很多东西。

// $ cargo run
//    Compiling shared-state v0.1.0 (file:///projects/shared-state)
// error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
//    --> src/main.rs:11:22
//     |
// 11  |           let handle = thread::spawn(move || {
//     |  ______________________^^^^^^^^^^^^^_-
//     | |                      |
//     | |                      `Rc<Mutex<i32>>` cannot be sent between threads safely
// 12  | |             let mut num = counter.lock().unwrap();
// 13  | |
// 14  | |             *num += 1;
// 15  | |         });
//     | |_________- within this `[closure@src/main.rs:11:36: 15:10]`
//     |
//     = help: within `[closure@src/main.rs:11:36: 15:10]`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
//     = note: required because it appears within the type `[closure@src/main.rs:11:36: 15:10]`
// note: required by a bound in `spawn`
//
// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `shared-state` due to previous error

// Wow, that error message is very wordy! Here’s the important part to focus on: `Rc<Mutex<i32>>` cannot be sent between threads safely.
// 哇，那条错误信息很啰嗦！ 这是需要关注的重要部分：`Rc<Mutex<i32>>` 不能在线程之间安全地发送。
// The compiler is also telling us the reason why: the trait `Send` is not implemented for `Rc<Mutex<i32>>` .
// 编译器还告诉我们原因：特性 `Send` 没有为 `Rc<Mutex<i32>>` 实现。
// We’ll talk about Send in the next section: it’s one of the traits that ensures the types we use with threads are meant for use in concurrent situations.
// 我们将在下一节中讨论发送：它是确保我们与线程一起使用的类型适用于并发情况的特征之一。

// Unfortunately, Rc<T> is not safe to share across threads.
// 不幸的是，Rc<T> 不能安全地跨线程共享。
// When Rc<T> manages the reference count, it adds to the count for each call to clone and subtracts from the count when each clone is dropped.
// 当 Rc<T> 管理引用计数时，它会在每次调用克隆时添加计数，并在删除每个克隆时从计数中减去。
// But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread.
// 但它不使用任何并发原语来确保对计数的更改不会被另一个线程中断。
// This could lead to wrong counts—subtle bugs that could in turn lead to memory leaks or a value being dropped before we’re done with it.
// 这可能会导致错误的计数——细微的错误可能反过来导致内存泄漏或在我们完成之前丢弃一个值。
// What we need is a type exactly like Rc<T> but one that makes changes to the reference count in a thread-safe way.
// 我们需要的是一种与 Rc<T> 完全相同的类型，但它以线程安全的方式更改引用计数。

// Atomic Reference Counting with Arc<T>
// 使用 Arc<T> 的原子引用计数
// Fortunately, Arc<T> is a type like Rc<T> that is safe to use in concurrent situations.
// 幸运的是，Arc<T> 是一种类似于 Rc<T> 的类型，可以在并发情况下安全使用。
// The a stands for atomic, meaning it’s an atomically reference counted type.
// a 代表 atomic，意味着它是一个原子引用计数类型。
// Atomics are an additional kind of concurrency primitive that we won’t cover in detail here: see the standard library documentation for std::sync::atomic for more details.
// 原子是另一种并发原语，我们不会在这里详细介绍：有关更多详细信息，请参阅 std::sync::atomic 的标准库文档。
// At this point, you just need to know that atomics work like primitive types but are safe to share across threads.
// 此时，您只需要知道原子像原始类型一样工作，但可以安全地跨线程共享。

// You might then wonder why all primitive types aren’t atomic and why standard library types aren’t implemented to use Arc<T> by default.
// 然后您可能想知道为什么不是所有基本类型都是原子的，为什么标准库类型没有实现为默认使用 Arc<T>。
// The reason is that thread safety comes with a performance penalty that you only want to pay when you really need to.
// 原因是线程安全会带来性能损失，您只希望在真正需要时付出代价。
// If you’re just performing operations on values within a single thread, your code can run faster if it doesn’t have to enforce the guarantees atomics provide.
// 如果您只是在单个线程中对值执行操作，那么如果您的代码不必强制执行原子提供的保证，则它可以运行得更快。

// Let’s return to our example: Arc<T> and Rc<T> have the same API, so we fix our program by changing the use line, the call to new, and the call to clone.
// 让我们回到我们的示例：Arc<T> 和 Rc<T> 具有相同的 API，因此我们通过更改 use 行、对 new 的调用和对 clone 的调用来修复我们的程序。
// The code in Listing 16-15 will finally compile and run:
// 清单 16-15 中的代码最终将编译并运行：

// Filename: src/main.rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Listing 16-15: Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across multiple threads
// 示例 16-15：使用 Arc<T> 包装 Mutex<T> 以便能够跨多个线程共享所有权

// This code will print the following:
// 此代码将打印以下内容：

Result: 10

// We did it! We counted from 0 to 10, which may not seem very impressive, but it did teach us a lot about Mutex<T> and thread safety.
// 我们做到了！ 我们从 0 数到 10，这可能看起来不是很令人印象深刻，但它确实教会了我们很多有关 Mutex<T> 和线程安全的知识。
// You could also use this program’s structure to do more complicated operations than just incrementing a counter.
// 你也可以使用这个程序的结构来做比增加计数器更复杂的操作。
// Using this strategy, you can divide a calculation into independent parts, split those parts across threads,
// 使用这种策略，您可以将计算分成独立的部分，将这些部分拆分到线程中，
//  and then use a Mutex<T> to have each thread update the final result with its part.
//  然后使用 Mutex<T> 让每个线程用它的部分更新最终结果。

// Note that if you are doing simple numerical operations, there are types simpler than Mutex<T> types provided by the std::sync::atomic module of the standard library.
// 请注意，如果您正在执行简单的数值运算，则有比标准库的 std::sync::atomic 模块提供的 Mutex<T> 类型更简单的类型。
// These types provide safe, concurrent, atomic access to primitive types.
// 这些类型提供对基本类型的安全、并发、原子访问。
// We chose to use Mutex<T> with a primitive type for this example so we could concentrate on how Mutex<T> works.
// 在这个例子中，我们选择使用具有基本类型的 Mutex<T>，这样我们就可以专注于 Mutex<T> 的工作原理。


// Similarities Between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>
// RefCell<T>/Rc<T> 和 Mutex<T>/Arc<T> 之间的相似之处
// You might have noticed that counter is immutable but we could get a mutable reference to the value inside it;
// 你可能已经注意到 counter 是不可变的，但我们可以获得对其中值的可变引用；
// this means Mutex<T> provides interior mutability, as the Cell family does.
// 这意味着 Mutex<T> 提供内部可变性，就像 Cell 家族所做的那样。
// In the same way we used RefCell<T> in Chapter 15 to allow us to mutate contents inside an Rc<T>, we use Mutex<T> to mutate contents inside an Arc<T>.
// 与我们在第 15 章中使用 RefCell<T> 允许我们改变 Rc<T> 内的内容的方式相同，我们使用 Mutex<T> 改变 Arc<T> 内的内容。

// Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use Mutex<T>.
// 另一个需要注意的细节是，当您使用 Mutex<T> 时，Rust 无法保护您免受各种逻辑错误的影响。
// Recall in Chapter 15 that using Rc<T> came with the risk of creating reference cycles, where two Rc<T> values refer to each other, causing memory leaks.
// 回想一下第 15 章，使用 Rc<T> 会带来创建引用循环的风险，其中两个 Rc<T> 值相互引用，导致内存泄漏。
// Similarly, Mutex<T> comes with the risk of creating deadlocks.
// 类似地，Mutex<T> 具有创建死锁的风险。
// These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.
// 当一个操作需要锁定两个资源并且两个线程各自获得了一个锁，导致它们永远等待对方时，就会发生这种情况。
// If you’re interested in deadlocks, try creating a Rust program that has a deadlock;
// 如果你对死锁感兴趣，尝试创建一个有死锁的 Rust 程序；
// then research deadlock mitigation strategies for mutexes in any language and have a go at implementing them in Rust.
// 然后研究任何语言中互斥体的死锁缓解策略，并尝试在 Rust 中实现它们。
// The standard library API documentation for Mutex<T> and MutexGuard offers useful information.
// Mutex<T> 和 MutexGuard 的标准库 API 文档提供了有用的信息。

// We’ll round out this chapter by talking about the Send and Sync traits and how we can use them with custom types.
// 我们将通过讨论 Send 和 Sync 特征以及我们如何将它们与自定义类型一起使用来结束本章。

// Extensible Concurrency with the Sync and Send Traits
// 具有同步和发送特征的可扩展并发
// Interestingly, the Rust language has very few concurrency features.
// 有趣的是，Rust 语言的并发特性很少。
// Almost every concurrency feature we’ve talked about so far in this chapter has been part of the standard library, not the language.
// 本章到目前为止我们讨论的几乎所有并发特性都是标准库的一部分，而不是语言。
// Your options for handling concurrency are not limited to the language or the standard library; you can write your own concurrency features or use those written by others.
// 您处理并发的选项不限于语言或标准库； 您可以编写自己的并发功能或使用其他人编写的功能。

// However, two concurrency concepts are embedded in the language: the std::marker traits Sync and Send.
// 但是，语言中嵌入了两个并发概念：std::marker 特性 Sync 和 Send。

// Allowing Transference of Ownership Between Threads with Send
// 允许通过发送在线程之间转移所有权
// The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads.
// Send 标记特征表示实现 Send 的类型的值的所有权可以在线程之间转移。
// Almost every Rust type is Send, but there are some exceptions, including Rc<T>:
// 几乎每个 Rust 类型都是 Send，但也有一些例外，包括 Rc<T>：
//  this cannot be Send because if you cloned an Rc<T> value and tried to transfer ownership of the clone to another thread, both threads might update the reference count at the same time.
//  这不能发送，因为如果你克隆了一个 Rc<T> 值并试图将克隆的所有权转移到另一个线程，两个线程可能同时更新引用计数。
// For this reason, Rc<T> is implemented for use in single-threaded situations where you don’t want to pay the thread-safe performance penalty.
// 出于这个原因，Rc<T> 被实现用于您不想支付线程安全性能损失的单线程情况。

// Therefore, Rust’s type system and trait bounds ensure that you can never accidentally send an Rc<T> value across threads unsafely.
// 因此，Rust 的类型系统和特征边界确保您永远不会意外地不安全地跨线程发送 Rc<T> 值。
// When we tried to do this in Listing 16-14, we got the error the trait Send is not implemented for Rc<Mutex<i32>>.
// 当我们尝试在示例 16-14 中执行此操作时，我们得到了一个错误，即没有为 Rc<Mutex<i32>> 实现 trait Send。
// When we switched to Arc<T>, which is Send, the code compiled.
// 当我们切换到 Arc<T>，即 Send，代码编译。

// Any type composed entirely of Send types is automatically marked as Send as well.
// 任何完全由 Send 类型组成的类型也会自动标记为 Send。
// Almost all primitive types are Send, aside from raw pointers, which we’ll discuss in Chapter 19.
// 除了我们将在第 19 章讨论的原始指针外，几乎所有原始类型都是 Send。

// Allowing Access from Multiple Threads with Sync
// 允许多个线程同步访问
// The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.
// Sync 标记特征表示从多个线程引用实现 Sync 的类型是安全的。
// In other words, any type T is Sync if &T (an immutable reference to T) is Send, meaning the reference can be sent safely to another thread.
// 换句话说，如果 &T（对 T 的不可变引用）是 Send，则任何类型 T 都是 Sync 的，这意味着引用可以安全地发送到另一个线程。
// Similar to Send, primitive types are Sync, and types composed entirely of types that are Sync are also Sync.
// 和Send类似，原始类型是Sync，完全由Sync类型组成的类型也是Sync。

// The smart pointer Rc<T> is also not Sync for the same reasons that it’s not Send.
// 智能指针 Rc<T> 也不是 Sync 的，原因与它不是 Send 的原因相同。
// The RefCell<T> type (which we talked about in Chapter 15) and the family of related Cell<T> types are not Sync.
// RefCell<T> 类型（我们在第 15 章中讨论过）和相关 Cell<T> 类型系列不是 Sync。
// The implementation of borrow checking that RefCell<T> does at runtime is not thread-safe.
// RefCell<T> 在运行时执行的借用检查的实现不是线程安全的。
// The smart pointer Mutex<T> is Sync and can be used to share access with multiple threads as you saw in the “Sharing a Mutex<T> Between Multiple Threads” section.
// 智能指针 Mutex<T> 是 Sync 的，可用于与多个线程共享访问，如您在“在多个线程之间共享 Mutex<T>”部分中看到的那样。

// Implementing Send and Sync Manually Is Unsafe
// 手动实现Send和Sync是不安全的
// Because types that are made up of Send and Sync traits are automatically also Send and Sync, we don’t have to implement those traits manually.
// 因为由 Send 和 Sync 特征组成的类型自动也是 Send 和 Sync，所以我们不必手动实现这些特征。
// As marker traits, they don’t even have any methods to implement.
// 作为标记特征，它们甚至没有任何方法可以实现。
// They’re just useful for enforcing invariants related to concurrency.
// 它们仅用于强制执行与并发相关的不变量。

// Manually implementing these traits involves implementing unsafe Rust code.
// 手动实现这些特征涉及实现 unsafe 的 Rust 代码。
// We’ll talk about using unsafe Rust code in Chapter 19;
// 我们将在第 19 章讨论使用 unsafe 的 Rust 代码；
// for now, the important information is that building new concurrent types not made up of Send and Sync parts requires careful thought to uphold the safety guarantees.
// 目前，重要的信息是构建新的并发类型而不是由 Send 和 Sync 部分组成需要仔细考虑以维护安全保证。
// “The Rustonomicon” has more information about these guarantees and how to uphold them.
// “The Rustonomicon”有更多关于这些保证以及如何维护它们的信息。

// Summary
// 概括
// This isn’t the last you’ll see of concurrency in this book:
// 这不是你在本书中最后一次看到并发：
//  the project in Chapter 20 will use the concepts in this chapter in a more realistic situation than the smaller examples discussed here.
//  第 20 章中的项目将在比此处讨论的较小示例更现实的情况下使用本章中的概念。

// As mentioned earlier, because very little of how Rust handles concurrency is part of the language, many concurrency solutions are implemented as crates.
// 如前所述，由于 Rust 处理并发的方式很少是语言的一部分，因此许多并发解决方案都是作为 crates 实现的。
// These evolve more quickly than the standard library, so be sure to search online for the current, state-of-the-art crates to use in multithreaded situations.
// 这些比标准库发展得更快，所以一定要在线搜索当前最先进的 crate，以便在多线程情况下使用。

// The Rust standard library provides channels for message passing and smart pointer types, such as Mutex<T> and Arc<T>, that are safe to use in concurrent contexts.
// Rust 标准库为消息传递和智能指针类型（例如 Mutex<T> 和 Arc<T>）提供通道，可以在并发上下文中安全使用。
// The type system and the borrow checker ensure that the code using these solutions won’t end up with data races or invalid references.
// 类型系统和借用检查器确保使用这些解决方案的代码不会以数据竞争或无效引用而告终。
// Once you get your code to compile, you can rest assured that it will happily run on multiple threads without the kinds of hard-to-track-down bugs common in other languages.
//  一旦您编译了代码，您就可以放心，它会愉快地在多线程上运行，而不会出现其他语言中常见的难以追踪的错误。
// Concurrent programming is no longer a concept to be afraid of: go forth and make your programs concurrent, fearlessly!
// 并发编程不再是一个让人害怕的概念：勇敢地让你的程序并发吧！

// Next, we’ll talk about idiomatic ways to model problems and structure solutions as your Rust programs get bigger.
// 接下来，我们将讨论随着 Rust 程序变得更大而对问题建模和构建解决方案的惯用方法。
// In addition, we’ll discuss how Rust’s idioms relate to those you might be familiar with from object-oriented programming.
// 此外，我们将讨论 Rust 的习语如何与您可能熟悉的面向对象编程的习语相关联。
