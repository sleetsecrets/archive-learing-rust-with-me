// Getting Started
// 入门
// Welcome to Asynchronous Programming in Rust! If you're looking to start writing asynchronous Rust code, you've come to the right place.
// 欢迎使用 Rust 中的异步编程！ 如果您希望开始编写异步 Rust 代码，那么您来对地方了。
// Whether you're building a web server, a database, or an operating system, this book will show you how to use Rust's asynchronous programming tools to get the most out of your hardware.
// 无论你是在构建网络服务器、数据库还是操作系统，本书都会向你展示如何使用 Rust 的异步编程工具来充分利用你的硬件。

// What This Book Covers
// 本书涵盖的内容
// This book aims to be a comprehensive, up-to-date guide to using Rust's async language features and libraries, appropriate for beginners and old hands alike.
// 本书旨在成为使用 Rust 的异步语言功能和库的全面、最新的指南，适合初学者和老手。

// The early chapters provide an introduction to async programming in general, and to Rust's particular take on it.
// 前面的章节介绍了一般的异步编程，以及 Rust 对它的特殊处理。
// The middle chapters discuss key utilities and control-flow tools you can use when writing async code, and describe best-practices for structuring libraries and applications to maximize performance and reusability.
// 中间章节讨论了编写异步代码时可以使用的关键实用程序和控制流工具，并描述了构建库和应用程序以最大化性能和可重用性的最佳实践。
// The last section of the book covers the broader async ecosystem, and provides a number of examples of how to accomplish common tasks.
// 本书的最后一部分涵盖了更广泛的异步生态系统，并提供了一些如何完成常见任务的示例。

// With that out of the way, let's explore the exciting world of Asynchronous Programming in Rust!
// 说完这些，让我们探索 Rust 中令人兴奋的异步编程世界！


// Why Async?
// 为什么是异步？
// We all love how Rust empowers us to write fast, safe software. But how does asynchronous programming fit into this vision?
// 我们都喜欢 Rust 如何使我们能够编写快速、安全的软件。 但是异步编程如何适应这个愿景呢？

// Asynchronous programming, or async for short, is a concurrent programming model supported by an increasing number of programming languages.
// 异步编程，简称async，是越来越多的编程语言支持的并发编程模型。
// It lets you run a large number of concurrent tasks on a small number of OS threads,
// 它可以让你在少量操作系统线程上运行大量并发任务，
//  while preserving much of the look and feel of ordinary synchronous programming, through the async/await syntax.
//  同时通过 async/await 语法保留普通同步编程的大部分外观和感觉。

// Async vs other concurrency models
// 异步与其他并发模型
// Concurrent programming is less mature and "standardized" than regular, sequential programming.
// 并发编程不如常规的顺序编程成熟和“标准化”。
// As a result, we express concurrency differently depending on which concurrent programming model the language is supporting.
// 因此，我们根据语言支持的并发编程模型以不同方式表达并发性。
// A brief overview of the most popular concurrency models can help you understand how asynchronous programming fits within the broader field of concurrent programming:
// 最流行的并发模型的简要概述可以帮助您了解异步编程如何适应更广泛的并发编程领域：

// * OS threads don't require any changes to the programming model, which makes it very easy to express concurrency.
// * 操作系统线程不需要对编程模型进行任何更改，这使得并发表达变得非常容易。
//   However, synchronizing between threads can be difficult, and the performance overhead is large.
//   然而，线程之间的同步可能很困难，并且性能开销很大。
//   Thread pools can mitigate some of these costs, but not enough to support massive IO-bound workloads.
//   线程池可以减轻其中一些成本，但不足以支持大量 IO 绑定的工作负载。
// * Event-driven programming, in conjunction with callbacks, can be very performant, but tends to result in a verbose, "non-linear" control flow.
// * 事件驱动编程与回调相结合，可以非常高效，但往往会导致冗长的“非线性”控制流。
//   Data flow and error propagation is often hard to follow.
//   数据流和错误传播通常很难遵循。
// * Coroutines, like threads, don't require changes to the programming model, which makes them easy to use. Like async, they can also support a large number of tasks.
// * 协程和线程一样，不需要改变编程模型，这使得它们易于使用。 和 async 一样，它们也可以支持大量的任务。
//   However, they abstract away low-level details that are important for systems programming and custom runtime implementors.
//   但是，它们抽象掉了对系统编程和自定义运行时实现者很重要的低级细节。
// * The actor model divides all concurrent computation into units called actors, which communicate through fallible message passing, much like in distributed systems.
// * actor 模型将所有并发计算划分为称为 actors 的单元，它们通过易出错的消息传递进行通信，就像在分布式系统中一样。
//   The actor model can be efficiently implemented, but it leaves many practical issues unanswered, such as flow control and retry logic.
//   actor 模型可以高效实现，但是它留下了很多实际问题没有解决，比如流量控制和重试逻辑。

// In summary, asynchronous programming allows highly performant implementations that are suitable for low-level languages like Rust,
// 总之，异步编程允许适用于低级语言（如 Rust）的高性能实现，
//  while providing most of the ergonomic benefits of threads and coroutines.
//  同时提供线程和协程的大部分人体工程学优势。

// Async in Rust vs other languages
// Rust 中的异步 vs 其他语言
// Although asynchronous programming is supported in many languages, some details vary across implementations.
// 尽管许多语言都支持异步编程，但一些细节因实现而异。
// Rust's implementation of async differs from most languages in a few ways:
// Rust 的异步实现在几个方面与大多数语言不同：

// * Futures are inert in Rust and make progress only when polled. Dropping a future stops it from making further progress.
// * Futures 在 Rust 中是惰性的，只有在轮询时才会取得进展。 放弃未来会阻止它取得进一步的进展。
// * Async is zero-cost in Rust, which means that you only pay for what you use. Specifically,
// * Async 在 Rust 中是零成本的，这意味着你只需为你使用的东西付费。 具体来说，
//   you can use async without heap allocations and dynamic dispatch, which is great for performance!
//   你可以在没有堆分配和动态调度的情况下使用异步，这对性能非常有用！
//   This also lets you use async in constrained environments, such as embedded systems.
//   这也让您可以在受限环境（例如嵌入式系统）中使用异步。
// * No built-in runtime is provided by Rust. Instead, runtimes are provided by community maintained crates.
// * Rust 不提供内置运行时。 相反，运行时由社区维护的crates提供。
// * Both single- and multithreaded runtimes are available in Rust, which have different strengths and weaknesses.
// * Rust 提供单线程和多线程运行时，它们各有优缺点。

// Async vs threads in Rust
// 异步与 Rust 中的线程
// The primary alternative to async in Rust is using OS threads, either directly through std::thread or indirectly through a thread pool.
// Rust 中异步的主要替代方法是使用操作系统线程，直接通过 std::thread 或间接通过线程池。
// Migrating from threads to async or vice versa typically requires major refactoring work,
// 从线程迁移到异步或从线程迁移到异步通常需要主要的重构工作，
//  both in terms of implementation and (if you are building a library) any exposed public interfaces.
//  在实现和（如果你正在构建一个库）任何暴露的公共接口方面。
// As such, picking the model that suits your needs early can save a lot of development time.
// 因此，尽早选择适合您需求的模型可以节省大量开发时间。

// OS threads are suitable for a small number of tasks, since threads come with CPU and memory overhead.
// 操作系统线程适用于少量任务，因为线程会带来 CPU 和内存开销。
// Spawning and switching between threads is quite expensive as even idle threads consume system resources.
// 在线程之间产生和切换是非常昂贵的，因为即使是空闲线程也会消耗系统资源。
// A thread pool library can help mitigate some of these costs, but not all.
// 线程池库可以帮助减轻部分成本，但不是全部。
// However, threads let you reuse existing synchronous code without significant code changes—no particular programming model is required.
// 但是，线程允许您重用现有的同步代码而无需对代码进行重大更改——不需要特定的编程模型。
// In some operating systems, you can also change the priority of a thread, which is useful for drivers and other latency sensitive applications.
// 在某些操作系统中，您还可以更改线程的优先级，这对驱动程序和其他对延迟敏感的应用程序很有用。

// Async provides significantly reduced CPU and memory overhead,
// 异步显着降低了 CPU 和内存开销，
//  especially for workloads with a large amount of IO-bound tasks, such as servers and databases.
//  特别适用于具有大量 IO 绑定任务的工作负载，例如服务器和数据库。
// All else equal, you can have orders of magnitude more tasks than OS threads,
// 在其他条件相同的情况下，您可以拥有比 OS 线程多几个数量级的任务，
//  because an async runtime uses a small amount of (expensive) threads to handle a large amount of (cheap) tasks.
//  因为异步运行时使用少量（昂贵的）线程来处理大量（廉价的）任务。
// However, async Rust results in larger binary blobs due to the state machines generated from async functions and since each executable bundles an async runtime.
// 但是，由于异步函数生成的状态机以及每个可执行文件都捆绑了一个异步运行时，异步 Rust 会导致更大的二进制 blob。

// On a last note, asynchronous programming is not better than threads, but different.
// 最后一点，异步编程不是比线程好，而是不同。
// If you don't need async for performance reasons, threads can often be the simpler alternative.
// 如果出于性能原因不需要异步，线程通常是更简单的选择。

// Example: Concurrent downloading
// 示例：并发下载
// In this example our goal is to download two web pages concurrently.
// 在这个例子中，我们的目标是同时下载两个网页。
// In a typical threaded application we need to spawn threads to achieve concurrency:
// 在典型的线程应用程序中，我们需要生成线程以实现并发：

fn get_two_sites() {
    // Spawn two threads to do work.
    // 生成两个线程来完成工作。
    let thread_one = thread::spawn(|| download("https://www.foo.com"));
    let thread_two = thread::spawn(|| download("https://www.bar.com"));

    // Wait for both threads to complete.
    // 等待两个线程完成。
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

// However, downloading a web page is a small task; creating a thread for such a small amount of work is quite wasteful.
// 然而，下载网页是一个小任务； 为如此少量的工作创建一个线程是相当浪费的。
// For a larger application, it can easily become a bottleneck. In async Rust, we can run these tasks concurrently without extra threads:
// 对于更大的应用程序，它很容易成为瓶颈。 在异步 Rust 中，我们可以在没有额外线程的情况下并发运行这些任务：

async fn get_two_sites_async() {
    // Create two different "futures" which, when run to completion,
    // 创建两个不同的“未来”，当运行完成时，
    // will asynchronously download the webpages.
    // 将异步下载网页。
    let future_one = download_async("https://www.foo.com");
    let future_two = download_async("https://www.bar.com");

    // Run both futures to completion at the same time.
    // 同时运行两个 futures 直到完成。
    join!(future_one, future_two);
}

// Here, no extra threads are created. Additionally, all function calls are statically dispatched, and there are no heap allocations!
// 在这里，没有创建额外的线程。 此外，所有函数调用都是静态调度的，没有堆分配！
// However, we need to write the code to be asynchronous in the first place, which this book will help you achieve.
// 然而，我们需要首先将代码编写为异步的，本书将帮助您实现这一点。

// Custom concurrency models in Rust
// Rust 中的自定义并发模型
// On a last note, Rust doesn't force you to choose between threads and async.
// 最后一点，Rust 不会强迫您在线程和异步之间做出选择。
// You can use both models within the same application, which can be useful when you have mixed threaded and async dependencies.
// 您可以在同一个应用程序中使用这两种模型，这在混合线程和异步依赖项时非常有用。
// In fact, you can even use a different concurrency model altogether, such as event-driven programming, as long as you find a library that implements it.
// 事实上，你甚至可以完全使用不同的并发模型，比如事件驱动编程，只要你找到一个实现它的库。

// MARK: next
