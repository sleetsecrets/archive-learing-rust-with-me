fn main() {

    // Error Handling
    // 错误处理
    // Errors are a fact of life in software, so Rust has a number of features for handling situations in which something goes wrong.
    // 错误是软件中不可避免的事实，因此 Rust 具有许多功能来处理出现问题的情况。
    // In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile.
    // 在许多情况下，Rust 要求您承认错误的可能性并在您的代码编译之前采取一些措施。
    // This requirement makes your program more robust by ensuring that you’ll discover errors and handle them appropriately before you’ve deployed your code to production!
    // 此要求通过确保您在将代码部署到生产环境之前发现错误并适当地处理它们，使您的程序更加健壮！

    // Rust groups errors into two major categories: recoverable and unrecoverable errors.
    // Rust 将错误分为两大类：可恢复错误和不可恢复错误。
    // For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation.
    // 对于可恢复的错误，例如文件未找到错误，我们很可能只想将问题报告给用户并重试操作。
    // Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program.
    // 不可恢复的错误总是bug出现的征兆，比如试图访问数组末尾以外的位置，所以我们想立即停止程序。

    // Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions.
    // 大多数语言不区分这两种错误，而是使用异常等机制以相同的方式处理它们。
    // Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the `panic!` macro that stops execution when the program encounters an unrecoverable error.
    // Rust 没有例外。 相反，它具有用于可恢复错误的类型 Result<T, E> 和当程序遇到不可恢复错误时停止执行的 `panic!` 宏。
    // This chapter covers calling `panic!` first and then talks about returning Result<T, E> values.
    // 本章首先介绍调用 `panic!`，然后讨论返回 Result<T, E> 值。
    // Additionally, we’ll explore considerations when deciding whether to try to recover from an error or to stop execution.
    // 此外，我们将探讨在决定是尝试从错误中恢复还是停止执行时的注意事项。

    // Unrecoverable Errors with `panic!`
    // 无法恢复的错误 `panic!`
    // Sometimes, bad things happen in your code, and there’s nothing you can do about it.
    // 有时，你的代码中会发生不好的事情，而你对此无能为力。
    // In these cases, Rust has the `panic!` macro. There are two ways to cause a panic in practice:
    // 在这些情况下，Rust 有 `panic!` 宏。 在实践中有两种方式可以引起恐慌：
    // |- by taking an action that causes our code to panic (such as accessing an array past the end) or by explicitly calling the `panic!` macro.
    // |- 通过采取导致我们的代码崩溃的操作（例如访问结束后的数组）或通过显式调用 `panic!` 宏。
    // In both cases, we cause a panic in our program.
    // 在这两种情况下，我们都会在程序中造成恐慌。
    // By default, these panics will print a failure message, unwind, clean up the stack, and quit.
    // 默认情况下，这些恐慌将打印失败消息、展开、清理堆栈并退出。
    // Via an environment variable, you can also have Rust display the call stack when a panic occurs to make it easier to track down the source of the panic.
    // 通过环境变量，您还可以让 Rust 在发生恐慌时显示调用堆栈，以便更容易追踪恐慌的来源。

    // Unwinding the Stack or Aborting in Response to a Panic
    // 展开堆栈或中止响应恐慌
    // By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters.
    // 默认情况下，当出现 panic 时，程序开始展开，这意味着 Rust 返回堆栈并清理它遇到的每个函数的数据。
    // However, this walking back and cleanup is a lot of work.
    // 但是，这种返回和清理工作量很大。
    // Rust, therefore, allows you to choose the alternative of immediately aborting, which ends the program without cleaning up.
    // 因此，Rust 允许您选择立即中止的替代方案，这会在不清理的情况下结束程序。

    // Memory that the program was using will then need to be cleaned up by the operating system.
    // 程序正在使用的内存将需要由操作系统清理。
    // If in your project you need to make the resulting binary as small as possible,
    // 如果在你的项目中你需要使生成的二进制文件尽可能小，
    // |- you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file.
    // |- 您可以通过将 panic = 'abort' 添加到 Cargo.toml 文件中适当的 [profile] 部分来从展开切换到中止。
    // For example, if you want to abort on panic in release mode, add this:
    // 例如，如果你想在 release mode 下出现 panic 时中止，请添加：

    // [profile.release]
    // panic = 'abort'

    // Let’s try calling `panic!` in a simple program:

    // Filename: src/main.rs

    fn main() {
        panic!("crash and burn");
    }

    // When you run the program, you’ll see something like this:

    // $ cargo run
    // Compiling panic v0.1.0 (file:///projects/panic)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.25s
    //     Running `target/debug/panic`
    // thread 'main' panicked at 'crash and burn', src/main.rs:2:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // The call to `panic!` causes the error message contained in the last two lines.
    // 对 `panic!` 的调用导致最后两行中包含的错误消息。
    // The first line shows our panic message and the place in our source code where the panic occurred: src/main.rs:2:5 indicates that it’s the second line,
    // 第一行显示了我们的 panic 消息和源代码中发生 panic 的位置：src/main.rs:2:5 表示它是第二行，
    // |- fifth character of our src/main.rs file.
    // |- src/main.rs 文件的第五个字符。

    // In this case, the line indicated is part of our code, and if we go to that line, we see the `panic!` macro call.
    // 在这种情况下，指示的行是我们代码的一部分，如果我们转到该行，我们会看到 `panic!` 宏调用。
    // In other cases, the `panic!` call might be in code that our code calls,
    // 在其他情况下，`panic!` 调用可能在我们的代码调用的代码中，
    // |- and the filename and line number reported by the error message will be someone else’s code where the `panic!` macro is called,
    // |- 错误消息报告的文件名和行号将是调用 `panic!` 宏的其他人的代码，
    // not the line of our code that eventually led to the `panic!` call.
    // 不是我们最终导致 `panic!` 调用的代码行。
    // We can use the backtrace of the functions the `panic!` call came from to figure out the part of our code that is causing the problem.
    // 我们可以使用调用 `panic!` 的函数的回溯来找出导致问题的代码部分。
    // We’ll discuss backtraces in more detail next.
    // 接下来我们将更详细地讨论回溯。

    // Using a `panic!` Backtrace
    // 使用 `panic!` 回溯
    // Let’s look at another example to see what it’s like when a `panic!` call comes from a library because of a bug in our code instead of from our code calling the macro directly.
    // 让我们看另一个例子，看看当 `panic!` 调用来自库时是什么感觉，因为我们的代码中有错误，而不是我们的代码直接调用宏。
    // Listing 9-1 has some code that attempts to access an index in a vector beyond the range of valid indexes.
    // 清单 9-1 中有一些代码试图访问超出有效索引范围的向量中的索引。

    // Filename: src/main.rs

    // This code panics!
    fn main() {
        let v = vec![1, 2, 3];

        v[99];
    }

    // Listing 9-1: Attempting to access an element beyond the end of a vector, which will cause a call to `panic!`
    // 示例 9-1：尝试访问向量末尾之外的元素，这将导致调用 `panic!`

    // Here, we’re attempting to access the 100th element of our vector (which is at index 99 because indexing starts at zero), but the vector has only 3 elements.
    // 这里，我们试图访问向量的第 100 个元素（索引为 99，因为索引从零开始），但向量只有 3 个元素。
    // In this situation, Rust will panic. Using [] is supposed to return an element, but if you pass an invalid index, there’s no element that Rust could return here that would be correct.
    // 在这种情况下，Rust 会 panic。 使用 [] 应该会返回一个元素，但是如果你传递了一个无效的索引，那么 Rust 就不会在这里返回正确的元素。

    // In C, attempting to read beyond the end of a data structure is undefined behavior.
    // 在 C 中，尝试读取超出数据结构末尾的内容是未定义的行为。
    // You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn’t belong to that structure.
    // 你可能会得到内存中与数据结构中的那个元素相对应的位置，即使内存不属于那个结构。
    // This is called a buffer overread and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the data structure.
    // 这称为缓冲区重读，如果攻击者能够操纵索引以读取数据结构之后不应允许的数据，则可能会导致安全漏洞。

    // To protect your program from this sort of vulnerability, if you try to read an element at an index that doesn’t exist, Rust will stop execution and refuse to continue.
    // 为了保护您的程序免受此类漏洞的影响，如果您尝试读取不存在的索引处的元素，Rust 将停止执行并拒绝继续。
    // Let’s try it and see:
    // 让我们试试看：

    // $ cargo run
    // Compiling panic v0.1.0 (file:///projects/panic)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.27s
    //     Running `target/debug/panic`
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // This error points at line 4 of our main.rs where we attempt to access index 99.
    // 这个错误指向我们 main.rs 的第 4 行，我们试图访问索引 99。
    // The next note line tells us that we can set the RUST_BACKTRACE environment variable to get a backtrace of exactly what happened to cause the error.
    // 下一个注释行告诉我们，我们可以设置 RUST_BACKTRACE 环境变量来获取导致错误的确切原因的回溯。
    // A backtrace is a list of all the functions that have been called to get to this point.
    // 回溯是为达到这一点而调用的所有函数的列表。
    // Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote.
    // Rust 中的回溯就像在其他语言中一样工作：阅读回溯的关键是从顶部开始阅读，直到看到您编写的文件。
    // That’s the spot where the problem originated.
    // 这就是问题产生的地方。
    // The lines above that spot are code that your code has called; the lines below are code that called your code.
    // 该点上方的行是您的代码调用的代码； 以下几行是调用您的代码的代码。
    // These before-and-after lines might include core Rust code, standard library code, or crates that you’re using.
    // 这些前后行可能包括核心 Rust 代码、标准库代码或您正在使用的 crate。
    // Let’s try getting a backtrace by setting the RUST_BACKTRACE environment variable to any value except 0.
    // 让我们尝试通过将 RUST_BACKTRACE 环境变量设置为 0 以外的任何值来获取回溯。
    // Listing 9-2 shows output similar to what you’ll see.
    // 清单 9-2 显示了类似于您将看到的输出。

    // $ RUST_BACKTRACE=1 cargo run
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
    // stack backtrace:
    // 0: rust_begin_unwind
    //             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
    // 1: core::panicking::panic_fmt
    //             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
    // 2: core::panicking::panic_bounds_check
    //             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
    // 3: <usize as core::slice::index::SliceIndex<[T]>>::index
    //             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
    // 4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
    //             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
    // 5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
    //             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
    // 6: panic::main
    //             at ./src/main.rs:4
    // 7: core::ops::function::FnOnce::call_once
    //             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

    // Listing 9-2: The backtrace generated by a call to `panic!` displayed when the environment variable RUST_BACKTRACE is set
    // 清单 9-2：设置环境变量 RUST_BACKTRACE 时显示的调用 `panic!` 生成的回溯

    // That’s a lot of output! The exact output you see might be different depending on your operating system and Rust version.
    // 这是很多输出！ 您看到的确切输出可能因您的操作系统和 Rust 版本而异。
    // In order to get backtraces with this information, debug symbols must be enabled.
    // 为了使用此信息获取回溯，必须启用调试符号。
    // Debug symbols are enabled by default when using cargo build or cargo run without the --release flag, as we have here.
    // 当使用不带 --release 标志的 cargo build 或 cargo run 时，调试符号默认启用，就像我们在这里所做的那样。

    // In the output in Listing 9-2, line 6 of the backtrace points to the line in our project that’s causing the problem: line 4 of src/main.rs.
    // 在清单 9-2 的输出中，回溯的第 6 行指向我们项目中导致问题的行：src/main.rs 的第 4 行。
    // If we don’t want our program to panic, we should start our investigation at the location pointed to by the first line mentioning a file we wrote.
    // 如果我们不希望我们的程序出现 panic，我们应该从第一行提到我们编写的文件所指向的位置开始调查。
    // In Listing 9-1, where we deliberately wrote code that would panic, the way to fix the panic is to not request an element beyond the range of the vector indexes.
    // 在示例 9-1 中，我们故意编写了会发生 panic 的代码，解决这种 panic 的方法是不要请求超出向量索引范围的元素。
    // When your code panics in the future, you’ll need to figure out what action the code is taking with what values to cause the panic and what the code should do instead.
    // 当你的代码将来出现 panic 时，你需要弄清楚代码正在采取什么操作，哪些值会导致 panic 以及代码应该做什么。

    // We’ll come back to `panic!` and when we should and should not use `panic!` to handle error conditions in the “To `panic!` or Not to `panic!`” section later in this chapter.
    // 我们将回到 `panic!` 以及何时应该和不应该使用 `panic!` 来处理错误情况，这将在本章后面的“To `panic!` or Not to `panic!`”部分中进行。
    // Next, we’ll look at how to recover from an error using Result.
    // 接下来，我们将看看如何使用 Result 从错误中恢复。

    // Recoverable Errors with `Result`
    // 带有 `Result` 的可恢复错误
    // Most errors aren’t serious enough to require the program to stop entirely.
    // 大多数错误都没有严重到需要程序完全停止的程度。
    // Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.
    // 有时，当函数失败时，原因很容易解释和响应。
    // For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.
    // 例如，如果您尝试打开一个文件，但由于文件不存在而导致操作失败，您可能希望创建文件而不是终止进程。

    // Recall from “Handling Potential Failure with the `Result` Type” in Chapter 2 that the `Result` enum is defined as having two variants, `Ok` and `Err`, as follows:
    // 回忆一下第 2 章中的“使用 `Result` 类型处理潜在的失败”，`Result` 枚举被定义为具有两个变体，`Ok` 和 `Err`，如下所示：

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // The T and E are generic type parameters: we’ll discuss generics in more detail in Chapter 10.
    // T 和 E 是泛型类型参数：我们将在第 10 章中更详细地讨论泛型。
    // What you need to know right now is that T represents the type of the value that will be returned in a success case within the `Ok` variant,
    // 你现在需要知道的是，T 表示在 `Ok` 变体中成功的情况下将返回的值的类型，
    // |- and E represents the type of the error that will be returned in a failure case within the Err variant.
    // |- 和 E 表示将在 Err 变体中的失败情况下返回的错误类型。
    // Because `Result` has these generic type parameters,
    // 因为 `Result` 有这些通用类型参数，
    // |- we can use the `Result` type and the functions defined on it in many different situations where the successful value and error value we want to return may differ.
    // |- 我们可以在许多不同的情况下使用 `Result` 类型和在其上定义的函数，在这些情况下，我们想要返回的成功值和错误值可能不同。

    // Let’s call a function that returns a `Result` value because the function could fail.
    // 让我们调用一个返回 `Result` 值的函数，因为该函数可能会失败。
    // In Listing 9-3 we try to open a file.
    // 在清单 9-3 中，我们尝试打开一个文件。

    // Filename: src/main.rs

    use std::fs::File;

    fn main() {
        let greeting_file_result = File::open("hello.txt");
    }

    // Listing 9-3: Opening a file
    // 示例 9-3：打开一个文件

    // The return type of File::open is a `Result<T, E>`.
    // File::open 的返回类型是 `Result<T, E>`。
    // The generic parameter T has been filled in by the implementation of File::open with the type of the success value, std::fs::File, which is a file handle.
    // 通用参数 T 已由 File::open 的实现填充为成功值的类型，std::fs::File，它是一个文件句柄。
    // The type of E used in the error value is std::io::Error.
    // 错误值中使用的 E 类型是 std::io::Error。
    // This return type means the call to File::open might succeed and return a file handle that we can read from or write to.
    // 此返回类型意味着对 File::open 的调用可能会成功并返回一个我们可以读取或写入的文件句柄。
    // The function call also might fail: for example, the file might not exist, or we might not have permission to access the file.
    // 函数调用也可能失败：例如，文件可能不存在，或者我们可能没有访问该文件的权限。
    // The File::open function needs to have a way to tell us whether it succeeded or failed and at the same time give us either the file handle or error information.
    // File::open 函数需要有一种方法来告诉我们它是成功还是失败，同时给我们文件句柄或错误信息。
    // This information is exactly what the `Result` enum conveys.
    // 此信息正是 `Result` 枚举传达的内容。

    // In the case where File::open succeeds, the value in the variable greeting_file_result will be an instance of Ok that contains a file handle.
    // 在 File::open 成功的情况下，变量 greeting_file_result 中的值将是包含文件句柄的 Ok 实例。
    // In the case where it fails, the value in greeting_file_result will be an instance of Err that contains more information about the kind of error that happened.
    // 在失败的情况下，greeting_file_result 中的值将是 Err 的一个实例，其中包含有关发生的错误类型的更多信息。

    // We need to add to the code in Listing 9-3 to take different actions depending on the value File::open returns.
    // 我们需要添加到清单 9-3 中的代码以根据 File::open 返回的值采取不同的操作。
    // Listing 9-4 shows one way to handle the `Result` using a basic tool, the match expression that we discussed in Chapter 6.
    // 清单 9-4 展示了一种使用基本工具处理 `Result` 的方法，我们在第 6 章讨论过的匹配表达式。

    // Filename: src/main.rs
    use std::fs::File;

    fn main() {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }

    // Listing 9-4: Using a match expression to handle the `Result` variants that might be returned
    // 示例 9-4：使用match表达式处理可能返回的 `Result` 变体

    // Note that, like the `Option` enum, the `Result` enum and its variants have been brought into scope by the `prelude`,
    // 请注意，与 `Option` 枚举一样，`Result` 枚举及其变体已被 `prelude` 纳入范围，
    // |- so we don’t need to specify `Result::` before the Ok and Err variants in the match arms.
    // |- 所以我们不需要在匹配臂中的 Ok 和 Err 变体之前指定 `Result::`。

    // When the result is `Ok`, this code will return the inner file value out of the `Ok` variant, and we then assign that file handle value to the variable greeting_file.
    // 当结果为 `Ok` 时，此代码将从 `Ok` 变体中返回内部文件值，然后我们将该文件句柄值分配给变量 greeting_file。
    // After the match, we can use the file handle for reading or writing.
    // 匹配后，我们可以使用文件句柄进行读取或写入。

    // The other `arm` of the `match` handles the case where we get an Err value from File::open.
    // `match` 的另一个 `arm` 处理我们从 File::open 获取 Err 值的情况。
    // In this example, we’ve chosen to call the `panic!` macro.
    // 在这个例子中，我们选择调用 `panic!` 宏。
    // If there’s no file named `hello.txt` in our current directory and we run this code, we’ll see the following output from the `panic!` macro:
    // 如果当前目录中没有名为 `hello.txt` 的文件并且我们运行这段代码，我们将看到 `panic!` 宏的以下输出：

    // $ cargo run
    //     Compiling error-handling v0.1.0 (file:///projects/error-handling)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.73s
    //     Running `target/debug/error-handling`
    // thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // As usual, this output tells us exactly what has gone wrong.
    // 像往常一样，这个输出告诉我们究竟出了什么问题。

    // Matching on Different Errors
    // 匹配不同的错误
    // The code in Listing 9-4 will `panic!` no matter why File::open failed.
    // 无论 File::open 为何失败，清单 9-4 中的代码都会 `panic!`。
    // However, we want to take different actions for different failure reasons:
    // 但是，我们希望针对不同的失败原因采取不同的操作：
    // |- if File::open failed because the file doesn’t exist, we want to create the file and return the handle to the new file.
    // |- 如果 File::open 因为文件不存在而失败，我们要创建文件并返回新文件的句柄。
    // If File::open failed for any other reason—for example, because we didn’t have permission to open the file—we still want the code to `panic!` in the same way as it did in Listing 9-4.
    // 如果 File::open 由于任何其他原因失败——例如，因为我们没有打开文件的权限——我们仍然希望代码像示例 9-4 中那样“崩溃！” .
    // For this we add an inner match expression, shown in Listing 9-5.
    // 为此，我们添加了一个内部匹配表达式，如清单 9-5 所示。

    // Filename: src/main.rs

    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }

    // Listing 9-5: Handling different kinds of errors in different ways
    // 示例 9-5：以不同方式处理不同类型的错误

    // The type of the value that File::open returns inside the Err variant is io::Error, which is a struct provided by the standard library.
    // File::open 在 Err 变体内部返回的值的类型是 io::Error，这是标准库提供的结构。
    // This struct has a method kind that we can call to get an io::ErrorKind value.
    // 这个结构有一个方法 kind，我们可以调用它来获取 io::ErrorKind 值。
    // The enum io::ErrorKind is provided by the standard library and has variants representing the different kinds of errors that might result from an io operation.
    // 枚举 io::ErrorKind 由标准库提供，具有表示 io 操作可能导致的不同类型错误的变体。
    // The variant we want to use is ErrorKind::NotFound, which indicates the file we’re trying to open doesn’t exist yet.
    // 我们要使用的变体是 ErrorKind::NotFound，它表示我们尝试打开的文件尚不存在。
    // So we `match` on greeting_file_result, but we also have an inner `match` on error.kind().
    // 因此我们在 greeting_file_result 上进行了“匹配”，但我们在 error.kind() 上也有一个内部“匹配”。

    // The condition we want to check in the inner `match` is whether the value returned by error.kind() is the NotFound variant of the ErrorKind enum.
    // 我们要在内部 `match` 中检查的条件是 error.kind() 返回的值是否是 ErrorKind 枚举的 NotFound 变体。
    // If it is, we try to create the file with File::create.
    // 如果是，我们尝试使用 File::create 创建文件。
    // However, because File::create could also fail, we need a second arm in the inner `match` expression.
    // 但是，因为 File::create 也可能失败，所以我们需要在内部 `match` 表达式中添加第二个分支。
    // When the file can’t be created, a different error message is printed.
    // 当无法创建文件时，打印不同的错误消息。
    // The second arm of the outer `match` stays the same, so the program panics on any error besides the missing file error.
    // 外部 `match` 的第二个分支保持不变，因此程序会在出现除丢失文件错误之外的任何错误时崩溃。

    // Alternatives to Using `match` with `Result<T, E>`
    // 将 `match` 与 `Result<T, E>` 结合使用的替代方案
    // That’s a lot of `match`!
    // 这里有很多 `match`！
    // The `match` expression is very useful but also very much a primitive.
    // `match` 表达式非常有用，但也非常原始。
    // In Chapter 13, you’ll learn about closures, which are used with many of the methods defined on `Result<T, E>`.
    // 在第 13 章中，您将了解 closures，它与定义在 `Result<T, E>` 上的许多方法一起使用。
    // These methods can be more concise than using `match` when handling `Result<T, E>` values in your code.
    // 在代码中处理 `Result<T, E>` 值时，这些方法比使用 `match` 更简洁。

    // For example, here’s another way to write the same logic as shown in Listing 9-5, this time using closures and the unwrap_or_else method:
    // 例如，这是编写与清单 9-5 中所示相同逻辑的另一种方法，这次使用 closures 和 unwrap_or_else 方法：

    use std::fs::File;
    use std::io::ErrorKind;

    fn main() {
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    }

    // Although this code has the same behavior as Listing 9-5, it doesn’t contain any `match` expressions and is cleaner to read.
    // 尽管此代码与示例 9-5 具有相同的行为，但它不包含任何 `match` 表达式并且更易读。
    // Come back to this example after you’ve read Chapter 13, and look up the unwrap_or_else method in the standard library documentation.
    // 读完第 13 章后回到这个例子，并在标准库文档中查找 unwrap_or_else 方法。
    // Many more of these methods can clean up huge nested `match` expressions when you’re dealing with errors.
    // 在处理错误时，更多这些方法可以清理巨大的嵌套 `match` 表达式。

    // Shortcuts for Panic on Error: unwrap and expect
    // Panic on Error 的快捷方式：unwrap and expect
    // Using `match` works well enough, but it can be a bit verbose and doesn’t always communicate intent well.
    // 使用 `match` 效果很好，但它可能有点冗长并且并不总是能很好地传达意图。
    // The `Result<T, E>` type has many helper methods defined on it to do various, more specific tasks.
    // `Result<T, E>` 类型定义了许多辅助方法来执行各种更具体的任务。
    // The unwrap method is a shortcut method implemented just like the `match` expression we wrote in Listing 9-4.
    // unwrap 方法是一个快捷方法，就像我们在示例 9-4 中编写的 `match` 表达式一样实现。
    // If the `Result` value is the Ok variant, unwrap will return the value inside the Ok.
    // 如果 `Result` 值是 Ok 变体，unwrap 将返回 Ok 中的值。
    // If the `Result` is the Err variant, unwrap will call the `panic!` macro for us. Here is an example of unwrap in action:
    // 如果 `Result` 是 Err 变体，unwrap 将为我们调用 `panic!` 宏。 以下是展开操作的示例：

    // Filename: src/main.rs

    use std::fs::File;

    fn main() {
        let greeting_file = File::open("hello.txt").unwrap();
    }

    // If we run this code without a `hello.txt` file, we’ll see an error message from the `panic!` call that the unwrap method makes:
    // 如果我们在没有 `hello.txt` 文件的情况下运行这段代码，我们将看到来自 unwrap 方法调用的 `panic!` 的错误消息：

    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os {
    //     code: 2, kind: NotFound, message: "No such file or directory" }',
    //     src/main.rs:4:49

    // Similarly, the expect method lets us also choose the `panic!` error message.
    // 同样，expect 方法让我们也可以选择 `panic!` 错误信息。
    // Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.
    // 使用 expect 而不是 unwrap 并提供良好的错误消息可以传达您的意图并使追踪恐慌的根源变得更加容易。
    // The syntax of expect looks like this:
    // expect 的语法如下所示：

    // Filename: src/main.rs

    use std::fs::File;

    fn main() {
        let greeting_file = File::open("hello.txt")
            .expect("hello.txt should be included in this project");
    }

    // We use expect in the same way as unwrap: to return the file handle or call the `panic!` macro.
    // 我们以与 unwrap 相同的方式使用 expect：返回文件句柄或调用 `panic!` 宏。
    // The error message used by expect in its call to `panic!` will be the parameter that we pass to expect, rather than the default `panic!` message that unwrap uses.
    // expect 在调用 `panic!` 时使用的错误消息将是我们传递给 expect 的参数，而不是 unwrap 使用的默认 `panic!` 消息。
    // Here’s what it looks like:
    // 这是它的样子：

    // thread 'main' panicked at 'hello.txt should be included in this project: Os {
    //     code: 2, kind: NotFound, message: "No such file or directory" }',
    //     src/main.rs:5:10

    // In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed.
    // 在生产质量代码中，大多数 Rustaceans 选择 expect 而不是 unwrap 并给出更多关于为什么期望操作总是成功的上下文。
    // That way, if your assumptions are ever proven wrong, you have more information to use in debugging.
    // 这样，如果您的假设被证明是错误的，您就有更多信息可用于调试。

    // Propagating Errors
    // 传播错误
    // When a function’s implementation calls something that might fail,
    // 当函数的实现调用可能失败的东西时，
    // |- instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do.
    // |- 您可以将错误返回给调用代码，而不是在函数本身内处理错误，以便它可以决定要做什么。
    // This is known as propagating the error and gives more control to the calling code,
    // 这被称为传播错误并为调用代码提供更多控制，
    // |- where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.
    // |- 那里可能有更多的信息或逻辑来指示应该如何处理错误，而不是您在代码上下文中可用的信息或逻辑。

    // For example, Listing 9-6 shows a function that reads a username from a file.
    // 例如，清单 9-6 显示了一个从文件中读取用户名的函数。
    // If the file doesn’t exist or can’t be read, this function will return those errors to the code that called the function.
    // 如果文件不存在或无法读取，此函数会将这些错误返回给调用该函数的代码。

    // Filename: src/main.rs
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // Listing 9-6: A function that returns errors to the calling code using match
    // 示例 9-6：使用 match 向调用代码返回错误的函数

    // This function can be written in a much shorter way, but we’re going to start by doing a lot of it manually in order to explore error handling; at the end, we’ll show the shorter way.
    // 这个函数可以用更短的方式编写，但我们将首先手动完成很多，以探索错误处理； 最后，我们将展示更短的方法。
    // Let’s look at the return type of the function first: `Result<String, io::Error>`.
    // 先看函数的返回类型：`Result<String, io::Error>`。
    // This means the function is returning a value of the type `Result<T, E>` where the generic parameter T has been filled in with the concrete type String,
    // 这意味着该函数正在返回一个类型为 `Result<T, E>` 的值，其中通用参数 T 已填充为具体类型 String，
    // |- and the generic type E has been filled in with the concrete type io::Error.
    // |- 泛型类型 E 已填充为具体类型 io::Error。

    // If this function succeeds without any problems, the code that calls this function will receive an Ok value that holds a String—the username that this function read from the file.
    // 如果这个函数没有任何问题成功，调用这个函数的代码将收到一个 Ok 值，它包含一个字符串——这个函数从文件中读取的用户名。
    // If this function encounters any problems, the calling code will receive an Err value that holds an instance of io::Error that contains more information about what the problems were.
    // 如果此函数遇到任何问题，调用代码将收到一个 Err 值，该值包含 io::Error 的实例，其中包含有关问题所在的更多信息。
    // We chose io::Error as the return type of this function because that happens to be the type of the error value returned from both of the operations we’re calling in this function’s body that might fail: the File::open function and the read_to_string method.
    // 我们选择 io::Error 作为这个函数的返回类型，因为它恰好是我们在这个函数主体中调用的两个可能失败的操作返回的错误值的类型：File::open 函数 和 read_to_string 方法。

    // The body of the function starts by calling the File::open function.
    // 函数体以调用 File::open 函数开始。
    // Then we handle the `Result` value with a match similar to the match in Listing 9-4.
    // 然后我们用类似于清单 9-4 中的匹配处理 `Result` 值。
    // If File::open succeeds, the file handle in the pattern variable file becomes the value in the mutable variable username_file and the function continues.
    // 如果 File::open 成功，则模式变量 file 中的文件句柄成为可变变量 username_file 中的值，函数继续。
    // In the Err case, instead of calling `panic!`,
    // 在 Err 情况下，不调用 `panic!`，
    // |- we use the return keyword to return early out of the function entirely and pass the error value from File::open,
    // |- 我们使用 return 关键字从函数中完全返回并传递来自 File::open 的错误值，
    // |- now in the pattern variable e, back to the calling code as this function’s error value.
    // |- 现在在模式变量 e 中，返回调用代码作为此函数的错误值。

    // So if we have a file handle in username_file,
    // 所以如果我们在 username_file 中有一个文件句柄，
    // |- the function then creates a new String in variable username and calls the read_to_string method on the file handle in username_file to read the contents of the file into username.
    // |- 该函数然后在变量 username 中创建一个新的 String 并在 username_file 中的文件句柄上调用 read_to_string 方法以将文件的内容读入 username。
    // The read_to_string method also returns a `Result` because it might fail, even though File::open succeeded.
    // read_to_string 方法也返回一个 `Result`，因为它可能失败，即使 File::open 成功。
    // So we need another match to handle that `Result`: if read_to_string succeeds,
    // 所以我们需要另一个匹配来处理那个 `Result`：如果 read_to_string 成功，
    // |- then our function has succeeded, and we return the username from the file that’s now in username wrapped in an Ok.
    // |- 那么我们的函数就成功了，我们从文件中返回用户名，该文件现在包含在一个 Ok 中的用户名中。
    // If read_to_string fails, we return the error value in the same way that we returned the error value in the match that handled the return value of File::open.
    // 如果 read_to_string 失败，我们返回错误值的方式与我们在处理 File::open 的返回值的匹配中返回错误值的方式相同。
    // However, we don’t need to explicitly say return, because this is the last expression in the function.
    // 但是，我们不需要显式地说 return，因为这是函数中的最后一个表达式。

    // The code that calls this code will then handle getting either an Ok value that contains a username or an Err value that contains an io::Error.
    // 调用此代码的代码将处理获取包含用户名的 Ok 值或包含 io::Error 的 Err 值。
    // It’s up to the calling code to decide what to do with those values.
    // 由调用代码决定如何处理这些值。
    // If the calling code gets an Err value, it could call `panic!` and crash the program, use a default username, or look up the username from somewhere other than a file, for example.
    // 如果调用代码得到一个 Err 值，它可能会调用 `panic!` 并使程序崩溃，使用默认用户名，或者从文件以外的其他地方查找用户名，例如。
    // We don’t have enough information on what the calling code is actually trying to do, so we propagate all the success or error information upward for it to handle appropriately.
    // 我们没有足够的信息来了解调用代码实际尝试做什么，因此我们向上传播所有成功或错误信息，以便它进行适当处理。

    // This pattern of propagating errors is so common in Rust that Rust provides the question mark operator `?` to make this easier.
    // 这种传播错误的模式在 Rust 中非常常见，以至于 Rust 提供了问号运算符 `?` 来简化这一过程。

    // A Shortcut for Propagating Errors: the `?` Operator
    // 传播错误的捷径：`?` 运算符
    // Listing 9-7 shows an implementation of read_username_from_file that has the same functionality as in Listing 9-6, but this implementation uses the `?` operator.
    // 清单 9-7 显示了 read_username_from_file 的一个实现，它具有与清单 9-6 相同的功能，但是这个实现使用了 `?` 运算符。

    // Filename: src/main.rs

    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // Listing 9-7: A function that returns errors to the calling code using the `?` operator
    // 示例 9-7：使用 `?` 运算符将错误返回给调用代码的函数

    // The `?` placed after a `Result` value is defined to work in almost the same way as the `match` expressions we defined to handle the `Result` values in Listing 9-6.
    // 放置在 `Result` 值之后的 `?` 定义为与我们定义的用于处理清单 9-6 中的 `Result` 值的 `match` 表达式几乎相同的工作方式。
    // If the value of the `Result` is an Ok, the value inside the Ok will get returned from this expression, and the program will continue.
    // 如果 `Result` 的值为 Ok，Ok 中的值将从该表达式返回，程序将继续。
    // If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.
    // 如果值为 Err，则 Err 将从整个函数返回，就好像我们使用了 return 关键字一样，因此错误值会传播到调用代码。

    // There is a difference between what the `match` expression from Listing 9-6 does and what the `?` operator does:
    // 清单 9-6 中的 `match` 表达式的作用与 `?` 运算符的作用不同：
    // |- error values that have the `?` operator called on them go through the from function,
    // |- 调用了 `?` 运算符的错误值通过 from 函数，
    // |- defined in the From trait in the standard library,
    // |- 在标准库的 From 特征中定义，
    // |- which is used to convert values from one type into another.
    // |- 用于将值从一种类型转换为另一种类型。
    // When the `?` operator calls the from function, the error type received is converted into the error type defined in the return type of the current function.
    // 当`?`操作符调用from函数时，接收到的错误类型被转换为当前函数返回类型中定义的错误类型。
    // This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
    // 当一个函数返回一种错误类型来表示一个函数可能失败的所有方式时，这很有用，即使部分可能由于许多不同的原因而失败。

    // For example, we could change the read_username_from_file function in Listing 9-7 to return a custom error type named OurError that we define.
    // 例如，我们可以更改清单 9-7 中的 read_username_from_file 函数以返回我们定义的名为 OurError 的自定义错误类型。
    // If we also define impl From<io::Error> for OurError to construct an instance of OurError from an io::Error,
    // 如果我们还为 OurError 定义 impl From<io::Error> 以从 io::Error 构造 OurError 的实例，
    // |- then the `?` operator calls in the body of read_username_from_file will call from and convert the error types without needing to add any more code to the function.
    // |- 然后 read_username_from_file 主体中的 `?` 运算符调用将从错误类型调用并转换错误类型，而无需向函数添加任何更多代码。

    // In the context of Listing 9-7, the `?` at the end of the File::open call will return the value inside an Ok to the variable username_file.
    // 在示例 9-7 的上下文中，File::open 调用末尾的 `?` 会将 Ok 中的值返回给变量 username_file。
    // If an error occurs, the `?` operator will return early out of the whole function and give any Err value to the calling code.
    // 如果发生错误，`?` 运算符将提前从整个函数中返回并将任何 Err 值赋给调用代码。
    // The same thing applies to the `?` at the end of the read_to_string call.
    // 同样的事情也适用于 read_to_string 调用末尾的 `?`。

    // The `?` operator eliminates a lot of boilerplate and makes this function’s implementation simpler.
    // `?` 运算符消除了很多样板文件并使此函数的实现更简单。
    // We could even shorten this code further by chaining method calls immediately after the `?`, as shown in Listing 9-8.
    // 我们甚至可以通过在 `?` 之后立即链接方法调用来进一步缩短这段代码，如清单 9-8 所示。

    // Filename: src/main.rs

    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // Listing 9-8: Chaining method calls after the `?` operator
    // 示例 9-8：在 `?` 运算符之后链接方法调用

    // We’ve moved the creation of the new String in username to the beginning of the function; that part hasn’t changed.
    // 我们已经将用户名中新字符串的创建移到了函数的开头； 那部分没有改变。
    // Instead of creating a variable username_file, we’ve chained the call to read_to_string directly onto the result of `File::open("hello.txt")?`.
    // 我们没有创建变量 username_file，而是将对 read_to_string 的调用直接链接到 `File::open("hello.txt")?` 的结果。
    // We still have a `?` at the end of the read_to_string call,
    // 我们在 read_to_string 调用的末尾仍然有一个 `?`，
    // |- and we still return an Ok value containing username when both File::open and read_to_string succeed rather than returning errors.
    // |- 当 File::open 和 read_to_string 都成功时，我们仍然返回一个包含用户名的 Ok 值，而不是返回错误。
    // The functionality is again the same as in Listing 9-6 and Listing 9-7; this is just a different, more ergonomic way to write it.
    // 功能与清单 9-6 和清单 9-7 相同； 这只是一种不同的、更符合人体工学的书写方式。

    // Listing 9-9 shows a way to make this even shorter using fs::read_to_string.
    // 清单 9-9 显示了一种使用 fs::read_to_string 使它更短的方法。

    // Filename: src/main.rs
    use std::fs;
    use std::io;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
    // Listing 9-9: Using fs::read_to_string instead of opening and then reading the file
    // 示例 9-9：使用 fs::read_to_string 而不是打开然后读取文件

    // Reading a file into a string is a fairly common operation,
    // 将文件读入字符串是一个相当常见的操作，
    // |- so the standard library provides the convenient fs::read_to_string function that opens the file,
    // |- 因此标准库提供了方便的 fs::read_to_string 函数来打开文件，
    // |- creates a new `String`, reads the contents of the file, puts the contents into that `String`, and returns it.
    // |- 创建一个新的 `String`，读取文件的内容，将内容放入那个 `String`，然后返回它。
    // Of course, using `fs::read_to_string` doesn’t give us the opportunity to explain all the error handling, so we did it the longer way first.
    // 当然，使用 `fs::read_to_string` 并没有给我们解释所有错误处理的机会，所以我们最初选择了艰苦的道路。

    // Where The `?` Operator Can Be Used
    // 哪里可以使用 `?` 运算符
    // The `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on.
    // `?` 运算符只能用在返回类型与使用 `?` 的值兼容的函数中。
    // This is because the `?` operator is defined to perform an early return of a value out of the function, in the same manner as the `match` expression we defined in Listing 9-6.
    // 这是因为 `?` 运算符被定义为执行函数的早期返回值（提早返回（一个）值），其方式与我们在清单 9-6 中定义的 `match` 表达式相同。
    // In Listing 9-6, the `match` was using a `Result` value, and the early return arm returned an Err(e) value.
    // 在示例 9-6 中，`match` 使用了一个 `Result` 值，而早期返回臂（return arm）返回了一个 Err(e) 值。
    // The return type of the function has to be a `Result` so that it’s compatible with this `return`.
    // 函数的返回类型必须是 `Result`，以便与这个 `return` 兼容。

    // In Listing 9-10, let’s look at the error we’ll get if we use the `?` operator in a main function with a return type incompatible with the type of the value we use `?` on:
    // 在示例 9-10 中，让我们看看如果我们在 main 函数中使用 `?` 运算符，返回类型与我们在上面使用 `?` 的值的类型不兼容时会得到的错误：

    // Filename: src/main.rs

    // This code does not compile!
    use std::fs::File;

    fn main() {
        let greeting_file = File::open("hello.txt")?;
    }
    // Listing 9-10: Attempting to use the `?` in the main function that returns () won’t compile
    // 示例 9-10：尝试在返回 () 的主函数中使用 `?` 不会编译

    // This code opens a file, which might fail. The `?` operator follows the Result value returned by File::open, but this main function has the return type of (), not Result.
    // 此代码打开一个文件，该文件可能会失败。 `?` 运算符跟在 File::open 返回的 Result 值之后，但是这个 main 函数的返回类型是 ()，而不是 Result。
    // When we compile this code, we get the following error message:
    // 当我们编译这段代码时，我们得到以下错误信息：

    // $ cargo run
    // Compiling error-handling v0.1.0 (file:///projects/error-handling)
    // error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
    // --> src/main.rs:4:48
    // |
    // 3 | / fn main() {
    // 4 | |     let greeting_file = File::open("hello.txt")?;
    // | |                                                ^ cannot use the `?` operator in a function that returns `()`
    // 5 | | }
    // | |_- this function should return `Result` or `Option` to accept `?`
    // |
    // = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`
    //
    // For more information about this error, try `rustc --explain E0277`.
    // error: could not compile `error-handling` due to previous error

    // This error points out that we’re only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements FromResidual.
    // 这个错误指出我们只允许在返回 `Result`、`Option` 或实现 FromResidual 的其他类型的函数中使用 `?` 运算符。

    // To fix the error, you have two choices.
    // 要修复错误，您有两种选择。
    // One choice is to change the return type of your function to be compatible with the value you’re using the `?` operator on as long as you have no restrictions preventing that.
    // 一种选择是更改函数的返回类型，使其与使用 `?` 运算符的值兼容，只要您没有任何限制即可。
    // The other technique is to use a match or one of the `Result<T, E>` methods to handle the `Result<T, E>` in whatever way is appropriate.
    // 另一种技术是使用 match 或 `Result<T, E>` 方法之一以任何适当的方式处理 `Result<T, E>`。

    // The error message also mentioned that `?` can be used with `Option<T>` values as well. As with using `?` on `Result`, you can only use `?` on `Option` in a function that returns an `Option`.
    // 错误消息还提到 `?` 也可以与 `Option<T>` 值一起使用。 与在 `Result` 上使用 `?` 一样，您只能在返回 `Option` 的函数中的 `Option` 上使用 `?`。
    // The behavior of the `?` operator when called on an `Option<T>` is similar to its behavior when called on a `Result<T, E>`:
    // 在 `Option<T>` 上调用时 `?` 运算符的行为类似于在 `Result<T, E>` 上调用时的行为：
    // |- if the value is None, the None will be returned early from the function at that point.
    // |- 如果值为 None，则 None 将在此时从函数中提前返回。
    // If the value is Some, the value inside the Some is the resulting value of the expression and the function continues.
    // 如果值为 Some，则 Some 内的值为表达式的结果值，函数继续。
    // Listing 9-11 has an example of a function that finds the last character of the first line in the given text:
    // 清单 9-11 中有一个函数示例，它查找给定文本中第一行的最后一个字符：

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    // Listing 9-11: Using the `?` operator on an `Option<T>` value
    // 示例 9-11：在 `Option<T>` 值上使用 `?` 运算符

    // This function returns `Option<char>` because it’s possible that there is a character there, but it’s also possible that there isn’t.
    // 这个函数返回 `Option<char>` 因为那里可能有一个字符，但也有可能没有。
    // This code takes the text string slice argument and calls the lines method on it, which returns an iterator over the lines in the string.
    // 此代码采用文本字符串切片参数并对其调用 lines 方法，该方法返回字符串中行的迭代器。
    // Because this function wants to examine the first line, it calls next on the iterator to get the first value from the iterator.
    // 因为这个函数想要检查第一行，所以它在迭代器上调用 next 以从迭代器中获取第一个值。
    // If text is the empty string, this call to next will return None, in which case we use ? to stop and return None from last_char_of_first_line.
    // 如果 text 是空字符串，对 next 的调用将返回 None，在这种情况下我们使用 ? 从 last_char_of_first_line 停止并返回 None。
    // If text is not the empty string, next will return a Some value containing a string slice of the first line in text.
    // 如果 text 不是空字符串，next 将返回一个 Some 值，其中包含文本中第一行的字符串切片。

    // The ? extracts the string slice, and we can call chars on that string slice to get an iterator of its characters.
    // 这 ？ 提取字符串切片，我们可以在该字符串切片上调用 chars 以获取其字符的迭代器。
    // We’re interested in the last character in this first line, so we call last to return the last item in the iterator.
    // 我们对第一行的最后一个字符感兴趣，所以我们调用 last 返回迭代器中的最后一项。
    // This is an `Option` because it’s possible that the first line is the empty string, for example if text starts with a blank line but has characters on other lines, as in "\nhi".
    // 这是一个 `Option`，因为第一行可能是空字符串，例如，如果文本以空行开头但其他行有字符，如“\nhi”。
    // However, if there is a last character on the first line, it will be returned in the Some variant.
    // 但是，如果第一行有最后一个字符，它将在 Some 变体中返回。
    // The ? operator in the middle gives us a concise way to express this logic, allowing us to implement the function in one line.
    // 这 ？ 中间的运算符为我们提供了一种简洁的方式来表达这种逻辑，使我们能够在一行中实现该功能。
    // If we couldn’t use the ? operator on `Option`, we’d have to implement this logic using more method calls or a match expression.
    // 如果我们不能使用 ? Option 上的运算符，我们必须使用更多方法调用或匹配表达式来实现此逻辑。

    // Note that you can use the `?` operator on a `Result` in a function that returns `Result`,
    // 请注意，您可以在返回 `Result` 的函数中的 `Result` 上使用 `?` 运算符，
    // |- and you can use the `?` operator on an `Option` in a function that returns `Option`,
    // |- 并且您可以在返回 `Option` 的函数中的 `Option` 上使用 `?` 运算符，
    // |- but you can’t mix and `match`.
    // |- 但你不能混合和“匹配”。
    // The `?` operator won’t automatically convert a `Result` to an `Option` or vice versa;
    // `?` 运算符不会自动将 `Result` 转换为 `Option`，反之亦然；
    // in those cases, you can use methods like the ok method on `Result` or the ok_or method on `Option` to do the conversion explicitly.
    // 在这些情况下，您可以使用诸如 `Result` 上的 ok 方法或 `Option` 上的 ok_or 方法之类的方法来显式地进行转换。

    // So far, all the main functions we’ve used return ().
    // 到目前为止，我们使用的所有主要函数都是 return ()。
    // The main function is special because it’s the entry and exit point of executable programs,
    // main 函数比较特殊，因为它是可执行程序的入口和出口点，
    // |- and there are restrictions on what its return type can be for the programs to behave as expected.
    // |- 并且为了使程序能正常工作，其可以返回的类型是有限制的。

    // Luckily, main can also return a `Result<(), E>`.
    // 幸运的是，main 还可以返回一个 `Result<(), E>`。
    // Listing 9-12 has the code from Listing 9-10 but we’ve changed the return type of main to be `Result<(), Box<dyn Error>>` and added a return value `Ok(())` to the end.
    // 示例 9-12 包含示例 9-10 中的代码，但我们将 main 的返回类型更改为 `Result<(), Box<dyn Error>>` 并添加了一个返回值 `Ok(()) ` 到最后。
    // This code will now compile:
    // 这段代码现在可以编译：

    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;

        Ok(())
    }

    // Listing 9-12: Changing main to return `Result<(), E>` allows the use of the `?` operator on `Result` values
    // 示例 9-12：将 main 更改为返回 `Result<(), E>` 允许在 `Result` 值上使用 `?` 运算符

    // The `Box<dyn Error>` type is a trait object, which we’ll talk about in the “Using Trait Objects that Allow for Values of Different Types” section in Chapter 17.
    // `Box<dyn Error>` 类型是一个特征对象，我们将在第 17 章的“使用允许不同类型值的特征对象”一节中讨论。
    // For now, you can read `Box<dyn Error>` to mean “any kind of error.” Using `?` on a `Result` value in a main function with the error type `Box<dyn Error>` is allowed,
    // 现在，您可以将 `Box<dyn Error>` 理解为“任何类型的错误”。 在错误类型为 Box<dyn Error> 的主函数中对 Result 值使用 `?` 是允许的，
    // |- because it allows any Err value to be returned early.
    // |- 因为它允许提前返回任何 Err 值。
    // Even though the body of this main function will only ever return errors of type std::io::Error,
    // 即使这个 main 函数的主体只会返回 std::io::Error 类型的错误，
    // |- by specifying `Box<dyn Error>`, this signature will continue to be correct even if more code that returns other errors is added to the body of main.
    // |- 通过指定 `Box<dyn Error>`，即使在 main 的主体中添加了更多返回其他错误的代码，此签名也将继续正确。

    // When a main function returns a `Result<(), E>`, the executable will exit with a value of 0 if main returns `Ok(())` and will exit with a nonzero value if main returns an Err value.
    // 当 main 函数返回 `Result<(), E>` 时，如果 main 返回 `Ok(())`，可执行文件将以 0 值退出，如果 main 返回 Err 值，则以非零值退出 .
    // Executables written in C return integers when they exit: programs that exit successfully return the integer 0, and programs that error return some integer other than 0.
    // 用 C 编写的可执行文件在退出时返回整数：成功退出的程序返回整数 0，出错的程序返回一些非 0 的整数。
    // Rust also returns integers from executables to be compatible with this convention.
    // Rust 还从可执行文件返回整数以与此约定兼容。

    // The main function may return any types that implement the std::process::Termination trait, which contains a function report that returns an ExitCode.
    // main 函数可以返回任何实现 std::process::Termination 特性的类型，其中包含一个返回 ExitCode 的函数报告。
    // Consult the standard library documentation for more information on implementing the Termination trait for your own types.
    // 有关为您自己的类型实现终止特征的更多信息，请参阅标准库文档。

    // Now that we’ve discussed the details of calling `panic!` or returning `Result`, let’s return to the topic of how to decide which is appropriate to use in which cases.
    // 现在我们已经讨论了调用 panic! 的细节 或返回 `Result`，让我们回到如何决定在哪些情况下适合使用哪个的主题。

    // To `panic!` or Not to `panic!`
    // `panic!` 或 Not to `panic!`
    // So how do you decide when you should call `panic!` and when you should return Result?
    // 那么你如何决定什么时候应该调用 `panic!` 以及什么时候应该返回结果？
    // When code panics, there’s no way to recover.
    // 当代码 panic 时，没有办法恢复。
    // You could call `panic!` for any error situation, whether there’s a possible way to recover or not,
    // 你可以为任何错误情况调用 `panic!`，无论是否有可能的恢复方法，
    // |- but then you’re making the decision that a situation is unrecoverable on behalf of the calling code.
    // |- 但随后您决定代表调用代码的情况是不可恢复的。
    // When you choose to return a `Result` value, you give the calling code options.
    // 当你选择返回一个 Result 值时，你给了调用代码选项。
    // The calling code could choose to attempt to recover in a way that’s appropriate for its situation,
    // 调用代码可以选择尝试以适合其情况的方式恢复，
    // |- or it could decide that an Err value in this case is unrecoverable, so it can call `panic!` and turn your recoverable error into an unrecoverable one.
    // |- 或者它可以决定在这种情况下 Err 值是不可恢复的，因此它可以调用 `panic!` 并将您的可恢复错误变成不可恢复的错误。
    // Therefore, returning Result is a good default choice when you’re defining a function that might fail.
    // 因此，当您定义一个可能会失败的函数时，返回 Result 是一个很好的默认选择。

    // In situations such as examples, prototype code, and tests, it’s more appropriate to write code that panics instead of returning a `Result`.
    // 在示例、原型代码和测试等情况下，编写 panic 代码比返回 `Result` 更合适。
    // Let’s explore why, then discuss situations in which the compiler can’t tell that failure is impossible, but you as a human can.
    // 让我们探讨原因，然后讨论编译器无法判断失败是不可能发生的情况，但作为人类的你可以。
    // The chapter will conclude with some general guidelines on how to decide whether to panic in library code.
    // 本章将以一些关于如何决定是否在库代码中恐慌的一般准则作为结尾。

    // Examples, Prototype Code, and Tests
    // 示例、原型代码和测试
    // When you’re writing an example to illustrate some concept, also including robust error-handling code can make the example less clear.
    // 当你写一个例子来说明一些概念时，还包括健壮的错误处理代码会使例子不那么清晰。
    // In examples, it’s understood that a call to a method like unwrap that could panic is meant as a placeholder for the way you’d want your application to handle errors,
    // 在示例中，可以理解调用 unwrap 之类的方法可能会导致恐慌，这意味着您希望应用程序处理错误的方式的占位符，
    // |- which can differ based on what the rest of your code is doing.
    // |- 这可能会根据您的其余代码所做的事情而有所不同。

    // Similarly, the unwrap and expect methods are very handy when prototyping, before you’re ready to decide how to handle errors.
    // 同样，在您准备好决定如何处理错误之前，unwrap 和 expect 方法在制作原型时非常方便。
    // They leave clear markers in your code for when you’re ready to make your program more robust.
    // 当你准备好让你的程序更健壮时，它们会在你的代码中留下清晰的标记。

    // If a method call fails in a test, you’d want the whole test to fail, even if that method isn’t the functionality under test.
    // 如果测试中的方法调用失败，您会希望整个测试失败，即使该方法不是被测功能。
    // Because `panic!` is how a test is marked as a failure, calling unwrap or expect is exactly what should happen.
    // 因为 `panic!` 是将测试标记为失败的方式，调用 unwrap 或 expect 正是应该发生的事情。

    // Cases in Which You Have More Information Than the Compiler
    // 你比编译器掌握更多信息的情况
    // It would also be appropriate to call unwrap or expect when you have some other logic that ensures the `Result` will have an Ok value, but the logic isn’t something the compiler understands.
    // 当您有一些其他逻辑可以确保 `Result` 具有 Ok 值时，调用 unwrap 或 expect 也是合适的，但编译器无法理解这些逻辑。
    // You’ll still have a `Result` value that you need to handle:
    // 你仍然有一个需要处理的 `Result` 值：
    // |- whatever operation you’re calling still has the possibility of failing in general, even though it’s logically impossible in your particular situation.
    // |- 无论你调用什么操作，一般来说仍然有可能失败，即使在你的特定情况下这在逻辑上是不可能的。
    // If you can ensure by manually inspecting the code that you’ll never have an Err variant, it’s perfectly acceptable to call unwrap,
    // 如果您可以通过手动检查代码来确保永远不会有 Err 变体，那么调用 unwrap 是完全可以接受的，
    // |- and even better to document the reason you think you’ll never have an Err variant in the expect text.
    // |- 甚至更好地记录您认为在期望文本中永远不会有 Err 变体的原因。
    // Here’s an example:
    // 这是一个例子：

    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    // We’re creating an `IpAddr` instance by parsing a hardcoded string.
    // 我们通过解析硬编码字符串来创建一个 `IpAddr` 实例。
    // We can see that `127.0.0.1` is a valid IP address, so it’s acceptable to use expect here.
    // 我们可以看到 `127.0.0.1` 是一个有效的 IP 地址，所以在这里使用 expect 是可以接受的。
    // However, having a hardcoded, valid string doesn’t change the return type of the parse method:
    // 但是，硬编码的有效字符串不会改变解析方法的返回类型：
    // |- we still get a `Result` value,
    // |- 我们仍然得到一个 `Result` 值，
    // |- and the compiler will still make us handle the `Result` as if the Err variant is a possibility because the compiler isn’t smart enough to see that this string is always a valid IP address.
    // |- 编译器仍然会让我们处理 `Result`，就好像 Err 变体是可能的一样，因为编译器不够聪明，无法看到这个字符串始终是有效的 IP 地址。
    // If the IP address string came from a user rather than being hardcoded into the program and therefore did have a possibility of failure,
    // 如果 IP 地址字符串来自用户而不是硬编码到程序中，因此确实有可能失败，
    // |- we’d definitely want to handle the `Result` in a more robust way instead.
    // |- 我们肯定希望以更稳健的方式处理 `Result`。
    // Mentioning the assumption that this IP address is hardcoded will prompt us to change expect to better error handling code if in the future, we need to get the IP address from some other source instead.
    // 提到这个 IP 地址是硬编码的假设将提示我们更改期望以更好的错误处理代码，如果将来我们需要从其他来源获取 IP 地址。

    // Guidelines for Error Handling
    // 错误处理指南
    // It’s advisable to have your code panic when it’s possible that your code could end up in a bad state.
    // 当您的代码可能最终处于有害状态时，建议让您的代码恐慌。
    // In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:
    // 在此上下文中，有害状态是指某些假设、保证、契约或不变量已被破坏，例如当无效值、矛盾值或缺失值传递给您的代码时——加上以下一项或多项：

    // The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
    // 有害状态是意料之外的事情，而不是偶尔会发生的事情，比如用户以错误的格式输入数据。
    // Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
    // 在此之后的代码需要依赖于不处于这种有害状态，而不是每一步都检查问题。
    // There’s not a good way to encode this information in the types you use.
    // 在你使用的类型中没有一个好的方法来编码这些信息。
    // We’ll work through an example of what we mean in the “Encoding States and Behavior as Types” section of Chapter 17.
    // 我们将通过第 17 章“将状态和行为编码为类型”部分中的示例来说明我们的意思。

    // If someone calls your code and passes in values that don’t make sense, it’s best to return an error if you can so the user of the library can decide what they want to do in that case.
    // 如果有人调用你的代码并传入没有意义的值，如果可以的话最好返回一个错误，这样库的用户可以决定在这种情况下他们想做什么。
    // However, in cases where continuing could be insecure or harmful, the best choice might be to call `panic!` and alert the person using your library to the bug in their code so they can fix it during development.
    // 但是，在继续可能不安全或有害的情况下，最好的选择可能是调用 `panic!` 并提醒使用您的库的人注意他们代码中的错误，以便他们可以在开发过程中修复它。
    // Similarly, `panic!` is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.
    // 同样，如果您正在调用不受控制的外部代码并且它返回您无法修复的无效状态，则 `panic!` 通常是合适的。

    // However, when failure is expected, it’s more appropriate to return a `Result` than to make a `panic!` call.
    // 然而，当预期失败时，返回 `Result` 比调用 `panic!` 更合适。
    // Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit.
    // 示例包括向解析器提供格式错误的数据或返回指示您已达到速率限制的状态的 HTTP 请求。
    // In these cases, returning a `Result` indicates that failure is an expected possibility that the calling code must decide how to handle.
    // 在这些情况下，返回 `Result` 表示失败是调用代码必须决定如何处理的预期可能性。

    // When your code performs an operation that could put a user at risk if it’s called using invalid values,
    // 如果您的代码执行的操作使用无效值调用，可能会使用户面临风险，
    // |- your code should verify the values are valid first and panic if the values aren’t valid.
    // |- 你的代码应该首先验证值是否有效，如果值无效则恐慌。
    // This is mostly for safety reasons: attempting to operate on invalid data can expose your code to vulnerabilities.
    // 这主要是出于安全原因：尝试对无效数据进行操作会使您的代码暴露于漏洞。
    // This is the main reason the standard library will call `panic!` if you attempt an out-of-bounds memory access:
    // 这是标准库在您尝试越界内存访问时调用 `panic!` 的主要原因：
    // |- trying to access memory that doesn’t belong to the current data structure is a common security problem.
    // |- 试图访问不属于当前数据结构的内存是一个常见的安全问题。
    // Functions often have contracts: their behavior is only guaranteed if the inputs meet particular requirements.
    // 函数通常有契约：它们的行为只有在输入满足特定要求时才能得到保证。
    // Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug and it’s not a kind of error you want the calling code to have to explicitly handle.
    // 当契约被违反时恐慌是有道理的，因为契约违反总是表示调用方错误，这不是您希望调用代码必须显式处理的错误。
    // In fact, there’s no reasonable way for calling code to recover; the calling programmers need to fix the code.
    // 实际上调用代码恢复没有合理的方式； 调用的程序员需要修复代码。
    // Contracts for a function, especially when a violation will cause a panic, should be explained in the API documentation for the function.
    // 函数的契约，尤其是当违反会导致恐慌时，应该在函数的 API 文档中进行解释。

    // However, having lots of error checks in all of your functions would be verbose and annoying.
    // 但是，在所有函数中进行大量错误检查会显得冗长且烦人。
    // Fortunately, you can use Rust’s type system (and thus the type checking done by the compiler) to do many of the checks for you.
    // 幸运的是，您可以使用 Rust 的类型系统（以及编译器完成的类型检查）为您完成许多检查。
    // If your function has a particular type as a parameter, you can proceed with your code’s logic knowing that the compiler has already ensured you have a valid value.
    // 如果你的函数有一个特定的类型作为参数，你可以继续你的代码逻辑，因为编译器已经确保你有一个有效的值。
    // For example, if you have a type rather than an `Option`, your program expects to have something rather than nothing.
    // 例如，如果您有一个类型而不是一个 `Option`，您的程序希望有一些东西而不是什么都没有。
    // Your code then doesn’t have to handle two cases for the Some and None variants: it will only have one case for definitely having a value.
    // 那么你的代码就不必处理 Some 和 None 变体的两种情况：它只会有一种情况肯定有一个值。
    // Code trying to pass nothing to your function won’t even compile, so your function doesn’t have to check for that case at runtime.
    // 试图不向您的函数传递任何内容的代码甚至不会编译，因此您的函数不必在运行时检查这种情况。
    // Another example is using an unsigned integer type such as `u32`, which ensures the parameter is never negative.
    // 另一个例子是使用无符号整数类型，例如 `u32`，它确保参数永远不会为负数。

    // Creating Custom Types for Validation
    // 创建用于验证的自定义类型
    // Let’s take the idea of using Rust’s type system to ensure we have a valid value one step further and look at creating a custom type for validation.
    // 让我们进一步了解使用 Rust 的类型系统来确保我们有一个有效值，并查看创建自定义类型以进行验证。
    // Recall the guessing game in Chapter 2 in which our code asked the user to guess a number between 1 and 100.
    // 回想第 2 章中的猜谜游戏，我们的代码要求用户猜测 1 到 100 之间的数字。
    // We never validated that the user’s guess was between those numbers before checking it against our secret number; we only validated that the guess was positive.
    // 在根据我们的秘密数字进行检查之前，我们从未验证用户的猜测是否在这些数字之间； 我们只验证了猜测是肯定的。
    // In this case, the consequences were not very dire: our output of “Too high” or “Too low” would still be correct.
    // 在这种情况下，后果不是很可怕：我们的“太高”或“太低”的输出仍然是正确的。
    // But it would be a useful enhancement to guide the user toward valid guesses and have different behavior when a user guesses a number that’s out of range versus when a user types,
    // 但这将是一个有用的增强功能，可以引导用户进行有效的猜测，并在用户猜测超出范围的数字时与用户键入时有不同的行为，
    // |- for example, letters instead.
    // |- 例如，改为字母。

    // One way to do this would be to parse the guess as an i32 instead of only a u32 to allow potentially negative numbers, and then add a check for the number being in range, like so:
    // 一种方法是将猜测解析为 i32 而不是仅 u32 以允许潜在的负数，然后添加对数字是否在范围内的检查，如下所示：

    loop {
        // --snip--

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
    }

    // The if expression checks whether our value is out of range, tells the user about the problem, and calls continue to start the next iteration of the loop and ask for another guess.
    // if 表达式检查我们的值是否超出范围，将问题告诉用户，并继续调用以开始循环的下一次迭代并要求再次猜测。
    // After the if expression, we can proceed with the comparisons between guess and the secret number knowing that guess is between 1 and 100.
    // 在 if 表达式之后，我们可以继续进行猜测和秘密数字之间的比较，知道猜测在 1 到 100 之间。

    // However, this is not an ideal solution:
    // 但是，这不是一个理想的解决方案：
    // |- if it was absolutely critical that the program only operated on values between 1 and 100,
    // |- 如果程序只对 1 到 100 之间的值进行操作是绝对关键的，
    // |- and it had many functions with this requirement, having a check like this in every function would be tedious (and might impact performance).
    // |- 它有很多函数都满足这个要求，在每个函数中进行这样的检查会很乏味（并且可能会影响性能）。

    // Instead, we can make a new type and put the validations in a function to create an instance of the type rather than repeating the validations everywhere.
    // 相反，我们可以创建一个新类型并将验证放在函数中以创建该类型的实例，而不是在所有地方重复验证。
    // That way, it’s safe for functions to use the new type in their signatures and confidently use the values they receive.
    // 这样，函数就可以安全地在其签名中使用新类型并自信地使用它们接收到的值。
    // Listing 9-13 shows one way to define a Guess type that will only create an instance of Guess if the new function receives a value between 1 and 100.
    // 清单 9-13 展示了一种定义 Guess 类型的方法，如果新函数接收到 1 到 100 之间的值，它只会创建 Guess 的实例。

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    // Listing 9-13: A Guess type that will only continue with values between 1 and 100
    // 示例 9-13：Guess 类型只会继续使用 1 到 100 之间的值

    // First, we define a struct named Guess that has a field named value that holds an i32.
    // 首先，我们定义一个名为 Guess 的结构，它有一个名为 value 的字段，其中包含一个 i32。
    // This is where the number will be stored.
    // 这是存储数字的地方。

    // Then we implement an associated function named new on Guess that creates instances of Guess values.
    // 然后我们在 Guess 上实现一个名为 new 的关联函数，它创建 Guess 值的实例。
    // The new function is defined to have one parameter named value of type i32 and to return a Guess.
    // 新函数被定义为具有一个名为 value 类型的 i32 的参数并返回一个 Guess。
    // The code in the body of the new function tests value to make sure it’s between 1 and 100.
    // 新函数主体中的代码测试值以确保它在 1 到 100 之间。
    // If value doesn’t pass this test, we make a `panic!` call,
    // 如果值没有通过这个测试，我们会调用 `panic!`，
    // |- which will alert the programmer who is writing the calling code that they have a bug they need to fix,
    // |- 这将提醒正在编写调用代码的程序员他们有一个需要修复的错误，
    // |- because creating a Guess with a value outside this range would violate the contract that Guess::new is relying on.
    // |- 因为创建一个值超出此范围的 Guess 会违反 Guess::new 所依赖的契约。
    // The conditions in which Guess::new might panic should be discussed in its public-facing API documentation;
    // Guess::new 可能出现 panic 的情况应该在其面向公众的 API 文档中讨论；
    // we’ll cover documentation conventions indicating the possibility of a `panic!` in the API documentation that you create in Chapter 14.
    // 我们将在您在第 14 章创建的 API 文档中介绍指示 `panic!` 可能性的文档约定。
    // If value does pass the test, we create a new Guess with its value field set to the value parameter and return the Guess.
    // 如果值确实通过了测试，我们创建一个新的 Guess，其值字段设置为值参数并返回 Guess。

    // Next, we implement a method named value that borrows self, doesn’t have any other parameters, and returns an i32.
    // 接下来，我们实现一个名为 value 的方法，它借用了 self，没有任何其他参数，并返回一个 i32。
    // This kind of method is sometimes called a getter, because its purpose is to get some data from its fields and return it.
    // 这种方法有时被称为getter，因为它的目的是从它的字段中获取一些数据并返回它。
    // This public method is necessary because the value field of the Guess struct is private.
    // 这个公共方法是必需的，因为 Guess 结构的值字段是私有的。
    // It’s important that the value field be private so code using the Guess struct is not allowed to set value directly:
    // 值字段是私有的很重要，因此不允许使用 Guess 结构的代码直接设置值：
    // |- code outside the module must use the Guess::new function to create an instance of Guess, thereby ensuring there’s no way for a Guess to have a value that hasn’t been checked by the conditions in the Guess::new function.
    // |- 模块外的代码必须使用 Guess::new 函数来创建 Guess 的实例，从而确保 Guess 无法获得未经 Guess::new 中的条件检查的值 功能。

    // A function that has a parameter or returns only numbers between 1 and 100 could then declare in its signature that it takes or returns a Guess rather than an i32 and wouldn’t need to do any additional checks in its body.
    // 具有参数或仅返回 1 到 100 之间的数字的函数然后可以在其签名中声明它采用或返回 Guess 而不是 i32，并且不需要在其主体中进行任何额外检查。

    // Summary
    // 概括
    // Rust’s error handling features are designed to help you write more robust code.
    // Rust 的错误处理功能旨在帮助您编写更健壮的代码。
    // The `panic!` macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values.
    // panic! 宏表示您的程序处于无法处理的状态，并让您告诉进程停止，而不是尝试继续处理无效或不正确的值。
    // The `Result` enum uses Rust’s type system to indicate that operations might fail in a way that your code could recover from.
    // `Result` 枚举使用 Rust 的类型系统来指示操作可能以您的代码可以从中恢复的方式失败。
    // You can use `Result` to tell code that calls your code that it needs to handle potential success or failure as well.
    // 您可以使用 `Result` 告诉调用您的代码的代码它也需要处理潜在的成功或失败。
    // Using `panic!` and `Result` in the appropriate situations will make your code more reliable in the face of inevitable problems.
    // 使用 panic! 和 Result 在适当的情况下将使您的代码在面对不可避免的问题时更加可靠。

    // Now that you’ve seen useful ways that the standard library uses generics with the `Option` and `Result` enums, we’ll talk about how generics work and how you can use them in your code.
    // 既然您已经了解了标准库将泛型与 `Option` 和 `Result` 枚举一起使用的有用方式，我们将讨论泛型的工作原理以及如何在代码中使用它们。
}
