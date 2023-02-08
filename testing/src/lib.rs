// Writing Automated Tests
// 编写自动化测试
// In his 1972 essay “The Humble Programmer,” Edsger W.
// 在他 1972 年的论文“谦逊的程序员”中，Edsger W.
// Dijkstra said that “Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.”
// Dijkstra 说“程序测试可以是显示错误存在的一种非常有效的方法，但它不足以证明它们不存在。”
// That doesn’t mean we shouldn’t try to test as much as we can!
// 这并不意味着我们不应该尽可能多地进行测试！

// Correctness in our programs is the extent to which our code does what we intend it to do.
// 我们程序的正确性是我们的代码在多大程度上完成了我们想要它做的事情。
// Rust is designed with a high degree of concern about the correctness of programs, but correctness is complex and not easy to prove.
// Rust 的设计高度关注程序的正确性，但正确性很复杂，不易证明。
// Rust’s type system shoulders a huge part of this burden, but the type system cannot catch everything.
// Rust 的类型系统承担了这个负担的很大一部分，但类型系统并不能包揽一切。
// As such, Rust includes support for writing automated software tests.
// 因此，Rust 包括对编写自动化软件测试的支持。

// Say we write a function add_two that adds 2 to whatever number is passed to it.
// 假设我们编写了一个函数 add_two，它将 2 加到传递给它的任何数字上。
// This function’s signature accepts an integer as a parameter and returns an integer as a result.
// 这个函数的签名接受一个整数作为参数并返回一个整数作为结果。
// When we implement and compile that function, Rust does all the type checking and borrow checking that you’ve learned so far to ensure that,
// 当我们实现并编译该函数时，Rust 会执行您目前所学的所有类型检查和借用检查，以确保，
// |- for instance, we aren’t passing a String value or an invalid reference to this function.
// |- 例如，我们没有传递字符串值或对此函数的无效引用。
// But Rust can’t check that this function will do precisely what we intend, which is return the parameter plus 2 rather than,
// 但 Rust 无法检查此函数是否会准确地执行我们想要的操作，即返回参数加 2 而不是，
// |- say, the parameter plus 10 or the parameter minus 50! That’s where tests come in.
// |- 例如，参数加 10 或参数减 50！ 这就是测试的用武之地。

// We can write tests that assert, for example, that when we pass 3 to the add_two function, the returned value is 5.
// 我们可以编写断言测试，例如，当我们将 3 传递给 add_two 函数时，返回值为 5。
// We can run these tests whenever we make changes to our code to make sure any existing correct behavior has not changed.
// 每当我们对代码进行更改时，我们都可以运行这些测试，以确保任何现有的正确行为都没有改变。

// Testing is a complex skill: although we can’t cover every detail about how to write good tests in one chapter, we’ll discuss the mechanics of Rust’s testing facilities.
// 测试是一项复杂的技能：虽然我们无法在一章中涵盖有关如何编写良好测试的所有细节，但我们将讨论 Rust 测试工具的机制。
// We’ll talk about the annotations and macros available to you when writing your tests, the default behavior and options provided for running your tests,
// 我们将讨论编写测试时可用的注释和宏，为运行测试提供的默认行为和选项，
// |- and how to organize tests into unit tests and integration tests.
// |- 以及如何将测试组织成单元测试和集成测试。

// How to Write Tests
// 如何编写测试
// Tests are Rust functions that verify that the non-test code is functioning in the expected manner.
// 测试是验证非测试代码是否以预期方式运行的 Rust 函数。
// The bodies of test functions typically perform these three actions:
// 测试函数的主体通常执行以下三个操作：

// 1.Set up any needed data or state.
// 1.设置任何需要的数据或状态。
// 2.Run the code you want to test.
// 2.运行你要测试的代码。
// 3.Assert the results are what you expect.
// 3.断言结果如你所料。

// Let’s look at the features Rust provides specifically for writing tests that take these actions, which include the test attribute, a few macros, and the should_panic attribute.
// 让我们看看 Rust 专门为编写执行这些操作的测试提供的功能，其中包括测试属性、一些宏和 should_panic 属性。

// The Anatomy of a Test Function
// 测试函数剖析
// At its simplest, a test in Rust is a function that’s annotated with the test attribute.
// 在最简单的情况下，Rust 中的测试是一个用 test 属性注释的函数。
// Attributes are metadata about pieces of Rust code; one example is the derive attribute we used with structs in Chapter 5.
// 属性是关于 Rust 代码片段的元数据； 一个例子是我们在第 5 章中与 struct 一起使用的 derive 属性。
// To change a function into a test function, add #[test] on the line before fn.
// 要将函数更改为测试函数，请在 fn 之前的行中添加 #[test]。
// When you run your tests with the cargo test command, Rust builds a test runner binary that runs the annotated functions and reports on whether each test function passes or fails.
// 当您使用 cargo test 命令运行测试时，Rust 会构建一个测试运行器二进制文件来运行带注释的函数并报告每个测试函数是通过还是失败。

// Whenever we make a new library project with Cargo, a test module with a test function in it is automatically generated for us.
// 每当我们用 Cargo 创建一个新的库项目时，都会自动为我们生成一个带有测试功能的测试模块。
// This module gives you a template for writing your tests so you don’t have to look up the exact structure and syntax every time you start a new project.
// 此模块为您提供了一个用于编写测试的模板，因此您不必在每次开始新项目时都查找确切的结构和语法。
// You can add as many additional test functions and as many test modules as you want!
// 你可以添加任意多的附加测试函数和任意多的测试模块！

// We’ll explore some aspects of how tests work by experimenting with the template test before we actually test any code.
// 在我们实际测试任何代码之前，我们将通过模板测试来探索测试如何工作的某些方面。
// Then we’ll write some real-world tests that call some code that we’ve written and assert that its behavior is correct.
// 然后我们将编写一些真实世界的测试来调用我们编写的一些代码并断言它的行为是正确的。

// Let’s create a new library project called adder that will add two numbers:
// 让我们创建一个名为 adder 的新库项目，它将添加两个数字：

// $ cargo new adder --lib
//      Created library `adder` project
// $ cd adder

// The contents of the src/lib.rs file in your `adder library` should look like Listing 11-1.
// `adder library` 中的 src/lib.rs 文件的内容应该如清单 11-1 所示。

// Filename: src/lib.rs

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// Listing 11-1: The test module and function generated automatically by cargo new
// 示例 11-1：cargo new 自动生成的测试模块和函数

// For now, let’s ignore the top two lines and focus on the function.
// 现在，让我们忽略前两行并关注函数。
// Note the #[test] annotation: this attribute indicates this is a test function, so the test runner knows to treat this function as a test.
// 注意 #[test] 注释：此属性表示这是一个测试函数，因此测试运行器知道将此函数视为测试。
// We might also have non-test functions in the tests module to help set up common scenarios or perform common operations, so we always need to indicate which functions are tests.
// 我们可能在测试模块中也有非测试功能，以帮助设置常见场景或执行常见操作，因此我们始终需要指出哪些功能是测试。

// The example function body uses the assert_eq! macro to assert that result, which contains the result of adding 2 and 2, equals 4.
// 示例函数体使用 assert_eq! 宏来断言结果，其中包含 2 和 2 相加的结果，等于 4。
// This assertion serves as an example of the format for a typical test.
// 此断言用作典型测试格式的示例。
// Let’s run it to see that this test passes.
// 让我们运行它看看这个测试是否通过。

// The cargo test command runs all tests in our project, as shown in Listing 11-2.
// cargo test 命令运行我们项目中的所有测试，如清单 11-2 所示。

// $ cargo test
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.57s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 1 test
// test tests::it_works ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//    Doc-tests adder
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// Listing 11-2: The output from running the automatically generated test
// 示例 11-2：运行自动生成的测试的输出

// Cargo compiled and ran the test.
// Cargo 编译并运行测试。
// We see the line running 1 test.
// 我们看到该行运行 1 个测试。
// The next line shows the name of the generated test function, called it_works, and that the result of running that test is ok.
// 下一行显示生成的测试函数的名称，称为 it_works，运行该测试的结果是 ok。
// The overall summary test result: ok. means that all the tests passed, and the portion that reads `1 passed`; `0 failed` totals the number of tests that passed or failed.
// 整体总结测试结果：ok。 意味着所有的测试都通过了.


// It’s possible to mark a test as ignored so it doesn’t run in a particular instance; we’ll cover that in the “Ignoring Some Tests Unless Specifically Requested” section later in this chapter.
// 可以将测试标记为忽略，这样它就不会在特定实例中运行； 我们将在本章后面的“除非特别要求，否则忽略一些测试”一节中介绍这一点。
// Because we haven’t done that here, the summary shows 0 ignored.
// 因为我们在这里没有这样做，所以摘要显示 0 被忽略了。
// We can also pass an argument to the cargo test command to run only tests whose name matches a string; this is called filtering and we’ll cover that in the “Running a Subset of Tests by Name” section.
// 我们还可以将参数传递给 cargo test 命令以仅运行名称与字符串匹配的测试； 这称为过滤，我们将在“按名称运行测试子集”部分中介绍。
// We also haven’t filtered the tests being run, so the end of the summary shows 0 filtered out.
// 我们也没有过滤正在运行的测试，所以摘要的末尾显示 0 过滤掉了。

// The 0 measured statistic is for benchmark tests that measure performance.
// 0 measured 统计数据用于衡量性能的基准测试。
// Benchmark tests are, as of this writing, only available in nightly Rust.
// 在撰写本文时，基准测试仅在 nightly Rust 中可用。
// See the documentation about benchmark tests to learn more.
// 请参阅有关基准测试的文档以了解更多信息。

// The next part of the test output starting at Doc-tests adder is for the results of any documentation tests.
// 从 Doc-tests adder 开始的测试输出的下一部分用于任何文档测试的结果。
// We don’t have any documentation tests yet, but Rust can compile any code examples that appear in our API documentation.
// 我们还没有任何文档测试，但 Rust 可以编译出现在我们的 API 文档中的任何代码示例。
// This feature helps keep your docs and your code in sync!
// 此功能有助于使您的文档和代码保持同步！
// We’ll discuss how to write documentation tests in the “Documentation Comments as Tests” section of Chapter 14.
// 我们将在第 14 章的“作为测试的文档注释”部分讨论如何编写文档测试。
// For now, we’ll ignore the Doc-tests output.
// 现在，我们将忽略 Doc-tests 输出。

// Let’s start to customize the test to our own needs.
// 让我们开始根据自己的需要自定义测试。
// First change the name of the it_works function to a different name, such as exploration, like so:
// 先把it_works函数的名字改成不同的名字，比如exploration，像这样：

// Filename: src/lib.rs

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}

// Then run `cargo test` again. The output now shows `exploration` instead of `it_works`:
// 然后再次运行 `cargo test`。 输出现在显示“exploration”而不是“it_works”：

// $ cargo test
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.59s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 1 test
// test tests::exploration ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//    Doc-tests adder
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// Now we’ll add another test, but this time we’ll make a test that fails!
// 现在我们将添加另一个测试，但这次我们将进行一个失败的测试！
// Tests fail when something in the test function panics.
// 当测试函数中的某些东西出现 panic 时，测试会失败。
// Each test is run in a new thread,
// 每个测试都在一个新线程中运行，
// |- and when the main thread sees that a test thread has died,
// |- 当主线程发现测试线程已经死亡时，
// |- the test is marked as failed.
// |- 测试被标记为失败。
// In Chapter 9, we talked about how the simplest way to panic is to call the `panic!` macro.
// 在第 9 章中，我们谈到了最简单的 panic 方法是调用 `panic!` 宏。
// Enter the new test as a function named another, so your src/lib.rs file looks like Listing 11-3.
// 将新测试作为名为 another 的函数输入，因此您的 src/lib.rs 文件如清单 11-3 所示。

// Filename: src/lib.rs

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

// Listing 11-3: Adding a second test that will fail because we call the panic! macro
// 示例 11-3：添加第二个会失败的测试，因为我们调用了 panic! 宏观

// Run the tests again using cargo test.
// 使用 cargo test 再次运行测试。
// The output should look like Listing 11-4, which shows that our exploration test passed and another failed.
// 输出应该如清单 11-4 所示，这表明我们的 exploration 测试通过了，而另一个失败了。

// $ cargo test
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.72s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 2 tests
// test tests::another ... FAILED
// test tests::exploration ... ok
//
// failures:
//
// ---- tests::another stdout ----
// thread 'main' panicked at 'Make this test fail', src/lib.rs:10:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
//
// failures:
//     tests::another
//
// test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass '--lib'

// Listing 11-4: Test results when one test passes and one test fails
// 示例 11-4：一个测试通过，一个测试失败时的测试结果

// Instead of ok, the line test tests::another shows FAILED.
// 而不是 ok，行测试 tests::another 显示 FAILED。
// Two new sections appear between the individual results and the summary: the first displays the detailed reason for each test failure.
// 在单个结果和摘要之间出现两个新部分：第一个部分显示每个测试失败的详细原因。
// In this case, we get the details that another failed because it panicked at 'Make this test fail' on line 10 in the src/lib.rs file.
// 在这种情况下，我们获得了另一个失败的详细信息，因为它在 src/lib.rs 文件第 10 行的“使此测试失败”处恐慌。
// The next section lists just the names of all the failing tests, which is useful when there are lots of tests and lots of detailed failing test output.
// 下一节仅列出所有失败测试的名称，这在有很多测试和大量详细的失败测试输出时很有用。
// We can use the name of a failing test to run just that test to more easily debug it; we’ll talk more about ways to run tests in the “Controlling How Tests Are Run” section.
// 我们可以使用失败测试的名称来运行该测试以更轻松地调试它； 我们将在“控制测试的运行方式”部分详细讨论运行测试的方法。

// The summary line displays at the end: overall, our test result is FAILED. We had one test pass and one test fail.
// 摘要行显示在最后：总的来说，我们的测试结果是失败的。 我们有一项测试通过，一项测试失败。

// Now that you’ve seen what the test results look like in different scenarios, let’s look at some macros other than panic! that are useful in tests.
// 既然你已经看到了不同场景下的测试结果是什么样的，那么让我们看看 panic 之外的一些宏！ 这在测试中很有用。

// Checking Results with the `assert!` Macro
// 使用 `assert!` 宏检查结果
// The `assert!` macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true.
// 标准库提供的 `assert!` 宏在您想要确保测试中的某些条件的计算结果为真时非常有用。
// We give the `assert!` macro an argument that evaluates to a Boolean.
// 我们给 `assert!` 宏一个评估为布尔值的参数。
// If the value is true, nothing happens and the test passes.
// 如果值为 true，则什么也不会发生，测试通过。
// If the value is false, the `assert!` macro calls panic! to cause the test to fail.
// 如果值为 false，`assert!` 宏调用 panic! 导致测试失败。
// Using the `assert!` macro helps us check that our code is functioning in the way we intend.
// 使用 `assert!` 宏可以帮助我们检查我们的代码是否按照我们预期的方式运行。

// In Chapter 5, Listing 5-15, we used a Rectangle struct and a can_hold method, which are repeated here in Listing 11-5.
// 在第 5 章的清单 5-15 中，我们使用了一个 Rectangle 结构和一个 can_hold 方法，它们在清单 11-5 中重复出现。
// Let’s put this code in the src/lib.rs file, then write some tests for it using the assert! macro.
// 让我们将这段代码放在 src/lib.rs 文件中，然后使用断言为其编写一些测试！ 宏。

// Filename: src/lib.rs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Listing 11-5: Using the Rectangle struct and its can_hold method from Chapter 5
// 示例 11-5：使用第 5 章中的 Rectangle 结构及其 can_hold 方法

// The can_hold method returns a Boolean, which means it’s a perfect use case for the `assert!` macro.
// can_hold 方法返回一个布尔值，这意味着它是 `assert!` 宏的完美用例。
// In Listing 11-6, we write a test that exercises the can_hold method by creating a Rectangle instance that has a width of 8 and
// 在清单 11-6 中，我们编写了一个测试，通过创建一个宽度为 8 的 Rectangle 实例来执行 can_hold 方法
// |- a height of 7 and asserting that it can hold another Rectangle instance that has a width of 5 and a height of 1.
// |- 高度为 7 并断言它可以容纳另一个宽度为 5 高度为 1 的 Rectangle 实例。

// Filename: src/lib.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}

// Listing 11-6: A test for can_hold that checks whether a larger rectangle can indeed hold a smaller rectangle
// 示例 11-6：can_hold 的测试，检查较大的矩形是否确实可以容纳较小的矩形

// Note that we’ve added a new line inside the tests module: use super::*;.
// 请注意，我们在测试模块中添加了一个新行：use super::*;。
// The tests module is a regular module that follows the usual visibility rules we covered in Chapter 7 in the “Paths for Referring to an Item in the Module Tree” section.
// 测试模块是一个常规模块，它遵循我们在第 7 章“模块树中引用项目的路径”部分中介绍的常见可见性规则。
// Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module.
// 因为tests模块是一个内层模块，所以我们需要将外层模块中的被测代码引入到内层模块的作用域中。
// We use a glob here so anything we define in the outer module is available to this tests module.
// 我们在这里使用了一个 glob，所以我们在外部模块中定义的任何东西都可以用于这个测试模块。

// We’ve named our test larger_can_hold_smaller, and we’ve created the two Rectangle instances that we need.
// 我们将测试命名为 larger_can_hold_smaller，并创建了我们需要的两个 Rectangle 实例。
// Then we called the `assert!` macro and passed it the result of calling larger.can_hold(&smaller).
// 然后我们调用 `assert!` 宏并将调用 larger.can_hold(&smaller) 的结果传递给它。
// This expression is supposed to return true, so our test should pass. Let’s find out!
// 这个表达式应该返回 true，所以我们的测试应该通过。 让我们找出答案！

// $ cargo test
//    Compiling rectangle v0.1.0 (file:///projects/rectangle)
//     Finished test [unoptimized + debuginfo] target(s) in 0.66s
//      Running unittests src/lib.rs (target/debug/deps/rectangle-6584c4561e48942e)
//
// running 1 test
// test tests::larger_can_hold_smaller ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//    Doc-tests rectangle
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// It does pass! Let’s add another test, this time asserting that a smaller rectangle cannot hold a larger rectangle:
// 它确实通过了！ 让我们添加另一个测试，这次断言较小的矩形不能容纳较大的矩形：

// Filename: src/lib.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

// Because the correct result of the can_hold function in this case is false, we need to negate that result before we pass it to the `assert!` macro.
// 因为在这种情况下 can_hold 函数的正确结果是 false，我们需要在将它传递给 `assert!` 宏之前取反该结果。
// As a result, our test will pass if can_hold returns false:
// 因此，如果 can_hold 返回 false，我们的测试将通过：

// $ cargo test
//    Compiling rectangle v0.1.0 (file:///projects/rectangle)
//     Finished test [unoptimized + debuginfo] target(s) in 0.66s
//      Running unittests src/lib.rs (target/debug/deps/rectangle-6584c4561e48942e)
//
// running 2 tests
// test tests::larger_can_hold_smaller ... ok
// test tests::smaller_cannot_hold_larger ... ok
//
// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//    Doc-tests rectangle
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// Two tests that pass! Now let’s see what happens to our test results when we introduce a bug in our code.
// 两个测试通过！ 现在让我们看看当我们在代码中引入一个错误时，我们的测试结果会发生什么。
// We’ll change the implementation of the can_hold method by replacing the greater-than sign with a less-than sign when it compares the widths:
// 我们将通过在比较宽度时用小于号替换大于号来更改 can_hold 方法的实现：

// --snip--
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

// Running the tests now produces the following:
// 现在运行测试会产生以下结果：

// $ cargo test
//    Compiling rectangle v0.1.0 (file:///projects/rectangle)
//     Finished test [unoptimized + debuginfo] target(s) in 0.66s
//      Running unittests src/lib.rs (target/debug/deps/rectangle-6584c4561e48942e)
//
// running 2 tests
// test tests::larger_can_hold_smaller ... FAILED
// test tests::smaller_cannot_hold_larger ... ok
//
// failures:
//
// ---- tests::larger_can_hold_smaller stdout ----
// thread 'main' panicked at 'assertion failed: larger.can_hold(&smaller)', src/lib.rs:28:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
//
// failures:
//     tests::larger_can_hold_smaller
//
// test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass '--lib'

// Our tests caught the bug! Because larger.width is 8 and smaller.width is 5, the comparison of the widths in can_hold now returns false: 8 is not less than 5.
// 我们的测试发现了 bug！ 因为 larger.width 是 8，smaller.width 是 5，所以 can_hold 中宽度的比较现在返回 false：8 不小于 5。

// Testing Equality with the `assert_eq!` and `assert_ne!` Macros
// 使用 `assert_eq!` 和 `assert_ne!` 宏测试相等性
// A common way to verify functionality is to test for equality between the result of the code under test and the value you expect the code to return.
// 验证功能的常用方法是测试被测代码的结果与您期望代码返回的值之间是否相等。
// You could do this using the `assert!` macro and passing it an expression using the == operator.
// 你可以使用 `assert!` 宏并使用 == 运算符将表达式传递给它。
// However, this is such a common test that the standard library provides a pair of macros—`assert_eq!` and `assert_ne!`—to perform this test more conveniently.
// 然而，这是一个非常常见的测试，标准库提供了一对宏——`assert_eq!` 和 `assert_ne!`——来更方便地执行这个测试。
// These macros compare two arguments for equality or inequality, respectively.
// 这些宏分别比较两个参数是否相等。
// They’ll also print the two values if the assertion fails, which makes it easier to see why the test failed;
// 如果断言失败，它们也会打印这两个值，这样更容易看出测试失败的原因；
// conversely, the `assert! macro` only indicates that it got a false value for the == expression, without printing the values that led to the false value.
// 相反，`assert! macro` 只表示它得到了 == 表达式的假值，而不打印导致假值的值。

// In Listing 11-7, we write a function named add_two that adds 2 to its parameter, then we test this function using the `assert_eq! macro`.
// 在清单 11-7 中，我们编写了一个名为 add_two 的函数，将 2 添加到它的参数中，然后我们使用 `assert_eq! 宏`。

// Filename: src/lib.rs

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

// Listing 11-7: Testing the function add_two using the `assert_eq!` macro
// 示例 11-7：使用 `assert_eq!` 宏测试函数 add_two

// Let’s check that it passes!
// 让我们检查它是否通过！

// $ cargo test
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.58s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 1 test
// test tests::it_adds_two ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//    Doc-tests adder
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// We pass 4 as the argument to `assert_eq!`, which is equal to the result of calling add_two(2).
// 我们将 4 作为参数传递给 `assert_eq!`，它等于调用 add_two(2) 的结果。
// The line for this test is test tests::it_adds_two ... ok, and the ok text indicates that our test passed!
// 此测试的行是 test tests::it_adds_two ... ok，ok 文本表示我们的测试通过了！

// Let’s introduce a bug into our code to see what `assert_eq!` looks like when it fails.
// 让我们在代码中引入一个错误，看看 `assert_eq!` 失败时的样子。
// Change the implementation of the add_two function to instead add 3:
// 将 add_two 函数的实现改为添加 3：

pub fn add_two(a: i32) -> i32 {
    a + 3
}

// Run the tests again:

// $ cargo test
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.61s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 1 test
// test tests::it_adds_two ... FAILED
//
// failures:
//
// ---- tests::it_adds_two stdout ----
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `4`,
//  right: `5`', src/lib.rs:11:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
//
// failures:
//     tests::it_adds_two
//
// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass '--lib'

// Our test caught the bug! The it_adds_two test failed, and the message tells us that the assertion that fails was assertion failed: `(left == right)` and what the left and right values are.
// 我们的测试发现了 bug！ it_adds_two 测试失败，消息告诉我们失败的断言是 assertion failed: `(left == right)` 以及左右值是什么。
// This message helps us start debugging: the left argument was 4 but the right argument, where we had add_two(2), was 5.
// 这条消息帮助我们开始调试：左边的参数是 4，但是右边的参数，我们有 add_two(2) 的地方，是 5。
// You can imagine that this would be especially helpful when we have a lot of tests going on.
// 你可以想象，当我们正在进行大量测试时，这会特别有用。

// Note that in some languages and test frameworks, the parameters to equality assertion functions are called expected and actual, and the order in which we specify the arguments matters.
// 请注意，在某些语言和测试框架中，相等断言函数的参数称为预期参数和实际参数，我们指定参数的顺序很重要。
// However, in Rust, they’re called left and right, and the order in which we specify the value we expect and the value the code produces doesn’t matter.
// 但是，在 Rust 中，它们被称为左和右，我们指定期望值和代码生成值的顺序并不重要。
// We could write the assertion in this test as assert_eq!(add_two(2), 4), which would result in the same failure message that displays assertion failed: `(left == right)`.
// 我们可以将此测试中的断言写为 assert_eq!(add_two(2), 4)，这将导致显示断言失败的相同失败消息：`(left == right)`。

// The `assert_ne!` macro will pass if the two values we give it are not equal and fail if they’re equal.
// 如果我们给它的两个值不相等，`assert_ne!` 宏将通过，如果它们相等则失败。
// This macro is most useful for cases when we’re not sure what a value will be, but we know what the value definitely shouldn’t be.
// 当我们不确定一个值是什么，但我们知道这个值绝对不应该是什么时，这个宏最有用。
// For example, if we’re testing a function that is guaranteed to change its input in some way,
// 例如，如果我们正在测试一个保证以某种方式改变其输入的函数，
// |- but the way in which the input is changed depends on the day of the week that we run our tests,
// |- 但输入更改的方式取决于我们运行测试的星期几，
// |- the best thing to assert might be that the output of the function is not equal to the input.
// |- 最好的断言可能是函数的输出不等于输入。

// Under the surface, the assert_eq! and `assert_ne!` macros use the operators == and !=, respectively.
// 在表面之下，assert_eq! 和 `assert_ne!` 宏分别使用运算符 == 和 !=。
// When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits.
// 当断言失败时，这些宏使用调试格式打印它们的参数，这意味着被比较的值必须实现 PartialEq 和 Debug 特征。
// All primitive types and most of the standard library types implement these traits.
// 所有基本类型和大多数标准库类型都实现了这些特征。
// For structs and enums that you define yourself, you’ll need to implement PartialEq to assert equality of those types.
// 对于您自己定义的结构和枚举，您需要实现 PartialEq 来断言这些类型的相等性。
// You’ll also need to implement Debug to print the values when the assertion fails.
// 您还需要实现调试以在断言失败时打印值。
// Because both traits are derivable traits, as mentioned in Listing 5-12 in Chapter 5,
// 因为这两个特征都是可派生的特征，如第 5 章中的清单 5-12 所述，
// |- this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum definition.
// |- 这通常与将 #[derive(PartialEq, Debug)] 注释添加到您的结构或枚举定义一样简单。
// See Appendix C, “Derivable Traits,” for more details about these and other derivable traits.
// 有关这些和其他可派生特征的更多详细信息，请参阅附录 C，“可派生特征”。

// Adding Custom Failure Messages
// 添加自定义失败消息
// You can also add a custom message to be printed with the failure message as optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros.
// 您还可以将要与失败消息一起打印的自定义消息作为可选参数添加到 `assert!`、`assert_eq!` 和 `assert_ne!` 宏。
// Any arguments specified after the required arguments are passed along to the `format!` macro (discussed in Chapter 8 in the “Concatenation with the + Operator or the `format!` Macro” section),
// 在所需参数之后指定的任何参数都将传递给 `format!` 宏（在第 8 章的“使用 + 运算符或 `format!` 宏连接”部分中讨论），
// |- so you can pass a format string that contains {} placeholders and values to go in those placeholders.
// |- 因此您可以传递包含 {} 占位符和值的格式字符串，这些占位符将进入这些占位符。
// Custom messages are useful for documenting what an assertion means; when a test fails, you’ll have a better idea of what the problem is with the code.
// 自定义消息对于记录断言的含义很有用； 当测试失败时，您将更好地了解代码的问题所在。

// For example, let’s say we have a function that greets people by name and we want to test that the name we pass into the function appears in the output:
// 例如，假设我们有一个按名字问候人们的函数，我们想测试我们传递给函数的名字是否出现在输出中：

// Filename: src/lib.rs

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}

// The requirements for this program haven’t been agreed upon yet, and we’re pretty sure the Hello text at the beginning of the greeting will change.
// 这个程序的要求还没有达成一致，我们很确定问候语开头的 Hello 文本会改变。
// We decided we don’t want to have to update the test when the requirements change,
// 我们决定不想在需求改变时更新测试，
// |- so instead of checking for exact equality to the value returned from the greeting function,
// |- 所以不是检查问候函数返回的值是否完全相等，
// |- we’ll just assert that the output contains the text of the input parameter.
// |- 我们只是断言输出包含输入参数的文本。

// Now let’s introduce a bug into this code by changing greeting to exclude name to see what the default test failure looks like:
// 现在让我们通过将 greeting 更改为 exclude name 来在此代码中引入一个错误，以查看默认测试失败的情况：

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

// Running this test produces the following:
// 运行此测试会产生以下结果：

// $ cargo test
//    Compiling greeter v0.1.0 (file:///projects/greeter)
//     Finished test [unoptimized + debuginfo] target(s) in 0.91s
//      Running unittests src/lib.rs (target/debug/deps/greeter-170b942eb5bf5e3a)
//
// running 1 test
// test tests::greeting_contains_name ... FAILED
//
// failures:
//
// ---- tests::greeting_contains_name stdout ----
// thread 'main' panicked at 'assertion failed: result.contains(\"Carol\")', src/lib.rs:12:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
//
// failures:
//     tests::greeting_contains_name
//
// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass '--lib'

// This result just indicates that the assertion failed and which line the assertion is on.
// 这个结果只是说明断言失败，断言在哪一行。
// A more useful failure message would print the value from the greeting function.
// 更有用的失败消息是打印问候函数的值。
// Let’s add a custom failure message composed of a format string with a placeholder filled in with the actual value we got from the greeting function:
// 让我们添加一个自定义失败消息，该消息由格式字符串组成，占位符填充我们从问候函数获得的实际值：

#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}

// Now when we run the test, we’ll get a more informative error message:
// 现在当我们运行测试时，我们会得到一条信息更丰富的错误消息：

// $ cargo test
//    Compiling greeter v0.1.0 (file:///projects/greeter)
//     Finished test [unoptimized + debuginfo] target(s) in 0.93s
//      Running unittests src/lib.rs (target/debug/deps/greeter-170b942eb5bf5e3a)
//
// running 1 test
// test tests::greeting_contains_name ... FAILED
//
// failures:
//
// ---- tests::greeting_contains_name stdout ----
// thread 'main' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:12:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
//
// failures:
//     tests::greeting_contains_name
//
// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass '--lib'

// We can see the value we actually got in the test output, which would help us debug what happened instead of what we were expecting to happen.
// 我们可以在测试输出中看到我们实际得到的值，这将帮助我们调试发生的事情而不是我们期望发生的事情。

// Checking for Panics with should_panic
// 使用 should_panic 检查恐慌
// In addition to checking return values, it’s important to check that our code handles error conditions as we expect.
// 除了检查返回值之外，检查我们的代码是否按预期处理错误情况也很重要。
// For example, consider the Guess type that we created in Chapter 9, Listing 9-13.
// 例如，考虑我们在第 9 章的清单 9-13 中创建的 Guess 类型。
// Other code that uses Guess depends on the guarantee that Guess instances will contain only values between 1 and 100.
// 其他使用 Guess 的代码取决于 Guess 实例将仅包含 1 到 100 之间的值的保证。
// We can write a test that ensures that attempting to create a Guess instance with a value outside that range panics.
// 我们可以编写一个测试，确保尝试创建一个值超出该范围的 Guess 实例时会发生恐慌。

// We do this by adding the attribute should_panic to our test function. The test passes if the code inside the function panics; the test fails if the code inside the function doesn’t panic.
// 我们通过将属性 should_panic 添加到我们的测试函数来做到这一点。 如果函数内的代码出现 panic，则测试通过； 如果函数内的代码没有 panic，则测试失败。

// Listing 11-8 shows a test that checks that the error conditions of Guess::new happen when we expect them to.
// 清单 11-8 显示了一个测试，它检查 Guess::new 的错误条件是否在我们期望的时候发生。

// Filename: src/lib.rs

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// Listing 11-8: Testing that a condition will cause a panic!
// 示例 11-8：测试一个条件会导致恐慌！

// We place the #[should_panic] attribute after the #[test] attribute and before the test function it applies to.
// 我们将#[should_panic] 属性放在#[test] 属性之后和它适用的测试函数之前。
// Let’s look at the result when this test passes:
// 让我们看看这个测试通过后的结果：

// $ cargo test
//    Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
//     Finished test [unoptimized + debuginfo] target(s) in 0.58s
//      Running unittests src/lib.rs (target/debug/deps/guessing_game-57d70c3acb738f4d)
//
// running 1 test
// test tests::greater_than_100 - should panic ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//    Doc-tests guessing_game
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// Looks good! Now let’s introduce a bug in our code by removing the condition that the new function will panic if the value is greater than 100:
// 看起来不错！ 现在让我们在代码中引入一个错误，删除如果值大于 100 时新函数将崩溃的条件：

// --snip--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

// When we run the test in Listing 11-8, it will fail:
// 当我们运行示例 11-8 中的测试时，它会失败：

// $ cargo test
//    Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
//     Finished test [unoptimized + debuginfo] target(s) in 0.62s
//      Running unittests src/lib.rs (target/debug/deps/guessing_game-57d70c3acb738f4d)
//
// running 1 test
// test tests::greater_than_100 - should panic ... FAILED
//
// failures:
//
// ---- tests::greater_than_100 stdout ----
// note: test did not panic as expected
//
// failures:
//     tests::greater_than_100
//
// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass '--lib'

// We don’t get a very helpful message in this case, but when we look at the test function, we see that it’s annotated with #[should_panic].
// 在这种情况下，我们没有得到非常有用的消息，但是当我们查看测试函数时，我们看到它带有 #[should_panic] 注释。
// The failure we got means that the code in the test function did not cause a panic.
// 我们得到的失败意味着测试函数中的代码没有引起恐慌。

// Tests that use should_panic can be imprecise. A should_panic test would pass even if the test panics for a different reason from the one we were expecting.
// 使用 should_panic 的测试可能不精确。 should_panic 测试会通过，即使测试因与我们预期不同的原因而恐慌。
// To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute.
// 为了使 should_panic 测试更加精确，我们可以向 should_panic 属性添加一个可选的预期参数。
// The test harness will make sure that the failure message contains the provided text.
// 测试工具将确保失败消息包含提供的文本。
// For example, consider the modified code for Guess in Listing 11-9 where the new function panics with different messages depending on whether the value is too small or too large.
// 例如，考虑清单 11-9 中 Guess 的修改代码，其中新函数根据值是太小还是太大而出现不同的消息。

// Filename: src/lib.rs

// --snip--

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// Listing 11-9: Testing for a `panic!` with a panic message containing a specified substring
// 示例 11-9：使用包含指定子字符串的恐慌消息测试 `panic!`

// This test will pass because the value we put in the should_panic attribute’s expected parameter is a substring of the message that the Guess::new function panics with.
// 这个测试将通过，因为我们在 should_panic 属性的预期参数中输入的值是 Guess::new 函数恐慌的消息的子字符串。
// We could have specified the entire panic message that we expect, which in this case would be Guess value must be less than or equal to 100, got 200.
// 我们可以指定我们期望的整个恐慌消息，在这种情况下是 Guess value must be less than or equal to 100, got 200.
// What you choose to specify depends on how much of the panic message is unique or dynamic and how precise you want your test to be.
// 您选择指定的内容取决于恐慌消息中有多少是唯一的或动态的，以及您希望测试的精确度。
// In this case, a substring of the panic message is enough to ensure that the code in the test function executes the else if value > 100 case.
// 在这种情况下，panic 消息的一个子字符串足以确保测试函数中的代码执行 else if value > 100 的情况。

// To see what happens when a should_panic test with an expected message fails,
// 要查看当带有预期消息的 should_panic 测试失败时会发生什么，
// |- let’s again introduce a bug into our code by swapping the bodies of the if value < 1 and the else if value > 100 blocks:
// |- 让我们通过交换 if value < 1 和 else if value > 100 块的主体再次在我们的代码中引入错误：

if value < 1 {
    panic!(
        "Guess value must be less than or equal to 100, got {}.",
        value
    );
} else if value > 100 {
    panic!(
        "Guess value must be greater than or equal to 1, got {}.",
        value
    );
}

// This time when we run the should_panic test, it will fail:
// 这次当我们运行 should_panic 测试时，它会失败：

// $ cargo test
//    Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
//     Finished test [unoptimized + debuginfo] target(s) in 0.66s
//      Running unittests src/lib.rs (target/debug/deps/guessing_game-57d70c3acb738f4d)
//
// running 1 test
// test tests::greater_than_100 - should panic ... FAILED
//
// failures:
//
// ---- tests::greater_than_100 stdout ----
// thread 'main' panicked at 'Guess value must be greater than or equal to 1, got 200.', src/lib.rs:13:13
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// note: panic did not contain expected string
//       panic message: `"Guess value must be greater than or equal to 1, got 200."`,
//  expected substring: `"less than or equal to 100"`
//
// failures:
//     tests::greater_than_100
//
// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass '--lib'

// The failure message indicates that this test did indeed panic as we expected, but the panic message did not include the expected string 'Guess value must be less than or equal to 100'.
// 失败信息表明这个测试确实如我们预期的那样发生了 panic，但是 panic 信息没有包含预期的字符串 'Guess value must be less than or equal to 100'。
// The panic message that we did get in this case was Guess value must be greater than or equal to 1, got 200. Now we can start figuring out where our bug is!
// 在这种情况下我们确实得到的恐慌信息是 Guess value must be greater than or equal to 1, got 200. 现在我们可以开始找出我们的错误在哪里！

// Using Result<T, E> in Tests
// 在测试中使用 Result<T, E>
// Our tests so far all panic when they fail. We can also write tests that use Result<T, E>!
// 到目前为止，我们的测试在失败时都会恐慌。 我们还可以编写使用 Result<T, E> 的测试！
// Here’s the test from Listing 11-1, rewritten to use Result<T, E> and return an Err instead of panicking:
// 这是清单 11-1 中的测试，重写为使用 Result<T, E> 并返回 Err 而不是恐慌：

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// The it_works function now has the Result<(), String> return type.
// it_works 函数现在具有 Result<(), String> 返回类型。
// In the body of the function, rather than calling the assert_eq! macro, we return Ok(()) when the test passes and an Err with a String inside when the test fails.
// 在函数体中，而不是调用 assert_eq! 宏，我们在测试通过时返回 Ok(()) ，在测试失败时返回带有 String 的 Err 。

// Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.
// Writing tests so they return a Result<T, E> 使您能够在测试主体中使用问号运算符，这可能是编写测试的便捷方式，如果其中的任何操作返回 Err 变体，则应失败。

// You can’t use the #[should_panic] annotation on tests that use Result<T, E>.
// 你不能在使用 Result<T, E> 的测试中使用#[should_panic] 注释。
// To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value.
// 要断言操作返回 Err 变体，请不要在 Result<T, E> 值上使用问号运算符。
// Instead, use assert!(value.is_err()).
// 相反，使用 assert!(value.is_err())。

// Now that you know several ways to write tests, let’s look at what is happening when we run our tests and explore the different options we can use with cargo test.
// 现在您知道了几种编写测试的方法，让我们看看运行测试时发生了什么，并探索我们可以与 cargo test 一起使用的不同选项。

// Controlling How Tests Are Run
// 控制测试的运行方式
// Just as cargo run compiles your code and then runs the resulting binary, cargo test compiles your code in test mode and runs the resulting test binary.
// 正如 cargo run 编译您的代码然后运行生成的二进制文件一样，cargo test 在测试模式下编译您的代码并运行生成的测试二进制文件。
// The default behavior of the binary produced by cargo test is to run all the tests in parallel and capture output generated during test runs,
// cargo test 生成的二进制文件的默认行为是并行运行所有测试并捕获测试运行期间生成的输出，
// |- preventing the output from being displayed and making it easier to read the output related to the test results.
// |- 防止显示（静默）输出并使与测试结果相关的输出更容易阅读。
// You can, however, specify command line options to change this default behavior.
// 但是，您可以指定命令行选项来更改此默认行为。

// Some command line options go to cargo test, and some go to the resulting test binary.
// 一些命令行选项进入 cargo test，一些进入生成的测试二进制文件。
// To separate these two types of arguments, you list the arguments that go to cargo test followed by the separator -- and then the ones that go to the test binary.
// 为了分隔这两种类型的参数，您列出了进入 cargo test 的参数，后跟分隔符 -- 然后是进入测试二进制文件的参数。
// Running cargo test --help displays the options you can use with cargo test, and running cargo test -- --help displays the options you can use after the separator.
// 运行 cargo test --help 显示您可以与 cargo test 一起使用的选项，运行 cargo test -- --help 显示您可以在分隔符后使用的选项。

// Running Tests in Parallel or Consecutively
// 并行或连续运行测试
// When you run multiple tests, by default they run in parallel using threads, meaning they finish running faster and you get feedback quicker.
// 当你运行多个测试时，默认情况下它们使用线程并行运行，这意味着它们完成运行得更快，你也能更快地获得反馈。
// |- Because the tests are running at the same time, you must make sure your tests don’t depend on each other or on any shared state, including a shared environment,
// |- 因为测试是同时运行的，所以你必须确保你的测试不依赖于彼此或任何共享状态，包括共享环境，
// |- such as the current working directory or environment variables.
// |- 例如当前工作目录或环境变量。

// For example, say each of your tests runs some code that creates a file on disk named test-output.txt and writes some data to that file.
// 例如，假设您的每个测试都运行一些代码，这些代码在磁盘上创建一个名为 test-output.txt 的文件并将一些数据写入该文件。
// Then each test reads the data in that file and asserts that the file contains a particular value, which is different in each test.
// 然后每个测试读取该文件中的数据并断言该文件包含特定值，该值在每个测试中都不同。
// Because the tests run at the same time, one test might overwrite the file in the time between another test writing and reading the file.
// 因为测试同时运行，一个测试可能会在另一个测试写入和读取文件之间的时间覆盖文件。
// The second test will then fail, not because the code is incorrect but because the tests have interfered with each other while running in parallel.
// 第二个测试会失败，不是因为代码不正确，而是因为测试在并行运行时相互干扰。
// One solution is to make sure each test writes to a different file; another solution is to run the tests one at a time.
// 一种解决方案是确保每个测试都写入不同的文件； 另一种解决方案是一次运行一个测试。

// If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used,
// 如果您不想并行运行测试，或者如果您想要对使用的线程数进行更细粒度的控制，
// |- you can send the --test-threads flag and the number of threads you want to use to the test binary.
// |- 您可以将 --test-threads 标志和要使用的线程数发送到测试二进制文件。
// Take a look at the following example:
// 看看下面的例子：

// $ cargo test -- --test-threads=1

// We set the number of test threads to 1, telling the program not to use any parallelism.
// 我们将测试线程数设置为 1，告诉程序不要使用任何并行性。
// Running the tests using one thread will take longer than running them in parallel, but the tests won’t interfere with each other if they share state.
// 使用一个线程运行测试将比并行运行它们花费更长的时间，但如果它们共享状态，测试将不会相互干扰。

// Showing Function Output
// 显示函数输出
// By default, if a test passes, Rust’s test library captures anything printed to standard output.
// 默认情况下，如果测试通过，Rust 的测试库会捕获任何打印到标准输出的内容。
// For example, if we call println! in a test and the test passes, we won’t see the println! output in the terminal;
// 例如，如果我们调用 println! 在测试中并且测试通过时，我们不会看到 println! 在终端输出；
// we’ll see only the line that indicates the test passed. If a test fails, we’ll see whatever was printed to standard output with the rest of the failure message.
// 我们只会看到表明测试通过的那一行。 如果测试失败，我们将看到打印到标准输出的任何内容以及其余的失败消息。

// As an example, Listing 11-10 has a silly function that prints the value of its parameter and returns 10, as well as a test that passes and a test that fails.
// 例如，清单 11-10 有一个愚蠢的函数，它打印其参数的值并返回 10，以及一个通过的测试和一个失败的测试。

// Filename: src/lib.rs

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

// Listing 11-10: Tests for a function that calls println!
// 示例 11-10：测试调用 println 的函数！

// When we run these tests with cargo test, we’ll see the following output:
// 当我们使用 cargo test 运行这些测试时，我们将看到以下输出：

// $ cargo test
//    Compiling silly-function v0.1.0 (file:///projects/silly-function)
//     Finished test [unoptimized + debuginfo] target(s) in 0.58s
//      Running unittests src/lib.rs (target/debug/deps/silly_function-160869f38cff9166)
//
// running 2 tests
// test tests::this_test_will_fail ... FAILED
// test tests::this_test_will_pass ... ok
//
// failures:
//
// ---- tests::this_test_will_fail stdout ----
// I got the value 8
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `5`,
//  right: `10`', src/lib.rs:19:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
//
// failures:
//     tests::this_test_will_fail
//
// test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass '--lib'

// Note that nowhere in this output do we see I got the value 4, which is what is printed when the test that passes runs.
// 请注意，在此输出中的任何地方我们都看不到 I got the value 4，这是通过测试运行时打印的值。
// That output has been captured. The output from the test that failed, I got the value 8, appears in the section of the test summary output, which also shows the cause of the test failure.
// 该输出已被捕获。 测试失败的输出，I got the value 8，出现在测试摘要输出部分，其中也显示了测试失败的原因。

// If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests with --show-output.
// 如果我们还想查看通过测试的打印值，我们可以告诉 Rust 也使用 --show-output 显示成功测试的输出。

// $ cargo test -- --show-output

// When we run the tests in Listing 11-10 again with the --show-output flag, we see the following output:
// 当我们使用 --show-output 标志再次运行示例 11-10 中的测试时，我们看到以下输出：

// $ cargo test -- --show-output
//    Compiling silly-function v0.1.0 (file:///projects/silly-function)
//     Finished test [unoptimized + debuginfo] target(s) in 0.60s
//      Running unittests src/lib.rs (target/debug/deps/silly_function-160869f38cff9166)
//
// running 2 tests
// test tests::this_test_will_fail ... FAILED
// test tests::this_test_will_pass ... ok
//
// successes:
//
// ---- tests::this_test_will_pass stdout ----
// I got the value 4
//
//
// successes:
//     tests::this_test_will_pass
//
// failures:
//
// ---- tests::this_test_will_fail stdout ----
// I got the value 8
// thread 'main' panicked at 'assertion failed: `(left == right)`
//   left: `5`,
//  right: `10`', src/lib.rs:19:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
//
// failures:
//     tests::this_test_will_fail
//
// test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass '--lib'

// Running a Subset of Tests by Name
// 按名称运行测试子集
// Sometimes, running a full test suite can take a long time.
// 有时，运行完整的测试套件可能需要很长时间。
// If you’re working on code in a particular area, you might want to run only the tests pertaining to that code.
// 如果您正在处理特定区域的代码，您可能只想运行与该代码相关的测试。
// You can choose which tests to run by passing cargo test the name or names of the test(s) you want to run as an argument.
// 您可以通过将要运行的测试的名称作为参数传递给 cargo test 来选择要运行的测试。

// To demonstrate how to run a subset of tests, we’ll first create three tests for our add_two function, as shown in Listing 11-11, and choose which ones to run.
// 为了演示如何运行测试的一个子集，我们将首先为我们的 add_two 函数创建三个测试，如清单 11-11 所示，然后选择要运行的测试。

// Filename: src/lib.rs

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

// Listing 11-11: Three tests with three different names
// 示例 11-11：具有三个不同名称的三个测试

// If we run the tests without passing any arguments, as we saw earlier, all the tests will run in parallel:
// 如果我们在不传递任何参数的情况下运行测试，如我们之前所见，所有测试将并行运行：

// $ cargo test
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.62s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 3 tests
// test tests::add_three_and_two ... ok
// test tests::add_two_and_two ... ok
// test tests::one_hundred ... ok
//
// test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//    Doc-tests adder
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// Running Single Tests
// 运行单个测试
// We can pass the name of any test function to cargo test to run only that test:
// 我们可以将任何测试函数的名称传递给 cargo test 以仅运行该测试：

// $ cargo test one_hundred
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.69s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 1 test
// test tests::one_hundred ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s

// Only the test with the name one_hundred ran; the other two tests didn’t match that name.
// 只有名称为 one_hundred 的测试运行了； 其他两个测试与该名称不匹配。
// The test output lets us know we had more tests that didn’t run by displaying 2 filtered out at the end.
// 测试输出让我们知道我们有更多的测试没有运行，最后显示 2 个被过滤掉。

// We can’t specify the names of multiple tests in this way; only the first value given to cargo test will be used.
// 我们不能通过这种方式指定多个测试的名称； 只会使用给 cargo test 的第一个值。
// But there is a way to run multiple tests.
// 但是有一种方法可以运行多个测试。

// Filtering to Run Multiple Tests
// 过滤以运行多个测试
// We can specify part of a test name, and any test whose name matches that value will be run.
// 我们可以指定测试名称的一部分，任何名称与该值匹配的测试都将运行。
// For example, because two of our tests’ names contain add, we can run those two by running cargo test add:
// 例如，因为我们的两个测试名称包含 add，我们可以通过运行 cargo test add 来运行这两个：

// $ cargo test add
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.61s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 2 tests
// test tests::add_three_and_two ... ok
// test tests::add_two_and_two ... ok
//
// test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

// This command ran all tests with add in the name and filtered out the test named one_hundred.
// 此命令运行名称中包含 add 的所有测试并过滤掉名为 one_hundred 的测试。
// Also note that the module in which a test appears becomes part of the test’s name, so we can run all the tests in a module by filtering on the module’s name.
// 另请注意，出现测试的模块成为测试名称的一部分，因此我们可以通过过滤模块名称来运行模块中的所有测试。

// Ignoring Some Tests Unless Specifically Requested
// 除非特别要求，否则忽略一些测试
// Sometimes a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of cargo test.
// 有时一些特定的测试执行起来可能非常耗时，因此您可能希望在大多数 cargo test 运行期间排除它们。
// Rather than listing as arguments all tests you do want to run, you can instead annotate the time-consuming tests using the ignore attribute to exclude them, as shown here:
// 与其将您确实要运行的所有测试都列为参数，不如使用 ignore 属性注释耗时的测试以排除它们，如下所示：

// Filename: src/lib.rs

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}

// After #[test] we add the #[ignore] line to the test we want to exclude.
// 在 #[test] 之后，我们将 #[ignore] 行添加到我们要排除的测试中。
// Now when we run our tests, it_works runs, but expensive_test doesn’t:
// 现在当我们运行测试时，it_works 运行，但 expensive_test 不运行：

// $ cargo test
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.60s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 2 tests
// test expensive_test ... ignored
// test it_works ... ok
//
// test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//    Doc-tests adder
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// The expensive_test function is listed as ignored.
// expensive_test 函数被列为忽略。
// If we want to run only the ignored tests, we can use cargo test -- --ignored:
// 如果我们只想运行被忽略的测试，我们可以使用 cargo test -- --ignored:

// $ cargo test -- --ignored
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.61s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 1 test
// test expensive_test ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
//
//    Doc-tests adder
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// By controlling which tests run, you can make sure your cargo test results will be fast.
// 通过控制运行哪些测试，您可以确保您的 cargo 测试结果很快。
// When you’re at a point where it makes sense to check the results of the ignored tests and you have time to wait for the results, you can run cargo test -- --ignored instead.
// 当你处于检查忽略测试的结果有意义并且你有时间等待结果的时候，你可以运行 cargo test -- --ignored 代替。
// If you want to run all tests whether they’re ignored or not, you can run cargo test -- --include-ignored.
// 如果你想运行所有测试，不管它们是否被忽略，你可以运行 cargo test -- --include-ignored。

// Test Organization
// 测试机构
// As mentioned at the start of the chapter, testing is a complex discipline, and different people use different terminology and organization.
// 如本章开头所述，测试是一门复杂的学科，不同的人使用不同的术语和组织。
// The Rust community thinks about tests in terms of two main categories: unit tests and integration tests.
// Rust 社区将测试分为两大类：单元测试和集成测试。
// Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces.
// 单元测试小而集中，一次隔离测试一个模块，可以测试私有接口。
// Integration tests are entirely external to your library and use your code in the same way any other external code would,
// 集成测试完全在您的库外部，并以与任何其他外部代码相同的方式使用您的代码，
// |- using only the public interface and potentially exercising multiple modules per test.
// |- 仅使用公共接口并可能在每个测试中使用多个模块。

// Writing both kinds of tests is important to ensure that the pieces of your library are doing what you expect them to, separately and together.
// 编写这两种测试对于确保你的库的各个部分分别和一起做你期望的事情很重要。

// Unit Tests
// 单元测试
// The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected.
// 单元测试的目的是将每个代码单元与其余代码隔离开来进行测试，以快速查明代码在哪里正常工作，哪里没有按预期工作。
// You’ll put unit tests in the src directory in each file with the code that they’re testing.
// 你会将单元测试放在每个文件的 src 目录中，其中包含他们正在测试的代码。
// The convention is to create a module named tests in each file to contain the test functions and to annotate the module with cfg(test).
// 约定是在每个文件中创建一个名为 tests 的模块来包含测试函数，并用 cfg(test) 注释模块。

// The Tests Module and #[cfg(test)]
// 测试模块和#[cfg(test)]
// The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.
// tests 模块上的#[cfg(test)] 注解告诉 Rust 仅在运行 cargo test 时编译和运行测试代码，而不是在运行 cargo build 时。
// This saves compile time when you only want to build the library and saves space in the resulting compiled artifact because the tests are not included.
// 当您只想构建库时，这可以节省编译时间，并节省生成的编译工件中的空间，因为不包括测试。
// You’ll see that because integration tests go in a different directory,
// 你会看到因为集成测试在不同的目录中，
// |- they don’t need the #[cfg(test)] annotation. However, because unit tests go in the same files as the code,
// |- 他们不需要 #[cfg(test)] 注释。 但是，因为单元测试与代码放在相同的文件中，
// |- you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.
// |- 您将使用#[cfg(test)] 指定它们不应包含在编译结果中。

// Recall that when we generated the new adder project in the first section of this chapter, Cargo generated this code for us:
// 回想一下，当我们在本章第一节生成新的 adder project 时，Cargo 为我们生成了这段代码：

// Filename: src/lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
// This code is the automatically generated test module.
// 这段代码是自动生成的测试模块。
// The attribute cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option.
// 属性 cfg 代表配置并告诉 Rust 只有在给定特定配置选项的情况下才应包含以下项目。
// In this case, the configuration option is test, which is provided by Rust for compiling and running tests.
// 在本例中，配置选项为 test，这是 Rust 提供的，用于编译和运行测试。
// By using the cfg attribute, Cargo compiles our test code only if we actively run the tests with cargo test.
// 通过使用 cfg 属性，只有当我们使用 cargo test 主动运行测试时，Cargo 才会编译我们的测试代码。
// This includes any helper functions that might be within this module, in addition to the functions annotated with #[test].
// 除了用#[test] 注释的函数之外，这包括可能在此模块中的任何辅助函数。

// Testing Private Functions
// 测试私有函数
// There’s debate within the testing community about whether or not private functions should be tested directly,
// 测试社区内部存在关于私有函数是否应该直接测试的争论，
// |- and other languages make it difficult or impossible to test private functions.
// |- 和其他语言使得测试私有函数变得困难或不可能。
// Regardless of which testing ideology you adhere to, Rust’s privacy rules do allow you to test private functions.
// 无论您遵循哪种测试思想，Rust 的隐私规则都允许您测试私有函数。
// Consider the code in Listing 11-12 with the private function internal_adder.
// 考虑清单 11-12 中带有私有函数 internal_adder 的代码。

// Filename: src/lib.rs
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

// Listing 11-12: Testing a private function
// 示例 11-12：测试私有函数

// Note that the internal_adder function is not marked as pub.
// 请注意，internal_adder 函数未标记为 pub。
// Tests are just Rust code, and the tests module is just another module.
// 测试只是 Rust 代码，测试模块只是另一个模块。
// As we discussed in the “Paths for Referring to an Item in the Module Tree” section, items in child modules can use the items in their ancestor modules.
// 正如我们在“模块树中引用项目的路径”部分中讨论的那样，子模块中的项目可以使用其祖先模块中的项目。
// In this test, we bring all of the test module’s parent’s items into scope with use super::*, and then the test can call internal_adder.
// 在这个测试中，我们使用 super::* 将所有测试模块的父项放入范围内，然后测试可以调用 internal_adder。
// If you don’t think private functions should be tested, there’s nothing in Rust that will compel you to do so.
// 如果您认为不应该测试私有函数，那么 Rust 中没有任何内容会强迫您这样做。

// Integration Tests
// 集成测试
// In Rust, integration tests are entirely external to your library.
// 在 Rust 中，集成测试完全在你的库之外。
// They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API.
// 它们以与任何其他代码相同的方式使用您的库，这意味着它们只能调用属于您库的公共 API 的函数。
// Their purpose is to test whether many parts of your library work together correctly.
// 它们的目的是测试您的库的许多部分是否可以正确协同工作。
// Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well.
// 独立运行正常的代码单元在集成时可能会出现问题，因此集成代码的测试覆盖率也很重要。
// To create integration tests, you first need a tests directory.
// 要创建集成测试，您首先需要一个测试目录。

// The tests Directory
// 测试目录
// We create a tests directory at the top level of our project directory, next to src.
// 我们在项目目录的顶层创建一个测试目录，紧挨着 src。
// Cargo knows to look for integration test files in this directory.
// Cargo 知道在此目录中查找集成测试文件。
// We can then make as many test files as we want, and Cargo will compile each of the files as an individual crate.
// 然后我们可以制作任意数量的测试文件，Cargo 会将每个文件编译为一个单独的 crate。

// Let’s create an integration test.
// 让我们创建一个集成测试。
// With the code in Listing 11-12 still in the src/lib.rs file, make a tests directory, and create a new file named tests/integration_test.rs.
// 清单 11-12 中的代码仍在 src/lib.rs 文件中，创建一个测试目录，并创建一个名为 tests/integration_test.rs 的新文件。
// Your directory structure should look like this:
// 你的目录结构应该是这样的：

// adder
// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   └── lib.rs
// └── tests
//     └── integration_test.rs

// Enter the code in Listing 11-13 into the tests/integration_test.rs file:
// 将清单 11-13 中的代码输入到 tests/integration_test.rs 文件中：

// Filename: tests/integration_test.rs

use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

// Listing 11-13: An integration test of a function in the adder crate
// 示例 11-13：adder crate 中函数的集成测试

// Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope.
// 测试目录中的每个文件都是一个单独的 crate，因此我们需要将我们的 library 放入每个测试 crate 的范围（作用域）内。
// For that reason we add use adder at the top of the code, which we didn’t need in the unit tests.
// 出于这个原因，我们在代码的顶部添加了 use adder，我们在单元测试中不需要它。

// We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)].
// 我们不需要用 #[cfg(test)] 注释 tests/integration_test.rs 中的任何代码。
// Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test.
// Cargo 特殊对待 tests 目录，只有当我们运行 cargo test 时才编译该目录中的文件。
// Run cargo test now:
// 现在运行 cargo test：

// $ cargo test
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 1.31s
//      Running unittests src/lib.rs (target/debug/deps/adder-1082c4b063a8fbe6)
//
// running 1 test
// test tests::internal ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//      Running tests/integration_test.rs (target/debug/deps/integration_test-1082c4b063a8fbe6)
//
// running 1 test
// test it_adds_two ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//    Doc-tests adder
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// The three sections of output include the unit tests, the integration test, and the doc tests.
// 输出的三部分包括单元测试、集成测试和文档测试。
// Note that if any test in a section fails, the following sections will not be run.
// 请注意，如果一个部分中的任何测试失败，则不会运行以下部分。
// For example, if a unit test fails, there won’t be any output for integration and doc tests because those tests will only be run if all unit tests are passing.
// 例如，如果单元测试失败，则不会有任何集成和文档测试的输出，因为这些测试只有在所有单元测试都通过时才会运行。

// The first section for the unit tests is the same as we’ve been seeing:
// 单元测试的第一部分与我们看到的一样：
// |- one line for each unit test (one named internal that we added in Listing 11-12) and then a summary line for the unit tests.
// |- 每个单元测试一行（我们在清单 11-12 中添加的一个名为 internal 的行），然后是单元测试的摘要行。

// The integration tests section starts with the line Running tests/integration_test.rs.
// 集成测试部分以 Running tests/integration_test.rs 行开始。
// Next, there is a line for each test function in that integration test and a summary line for the results of the integration test just before the Doc-tests adder section starts.
// 接下来，该集成测试中的每个测试函数都有一行，并且在 Doc-tests 加法器部分开始之前集成测试结果的摘要行。

// Each integration test file has its own section, so if we add more files in the tests directory, there will be more integration test sections.
// 每个集成测试文件都有自己的部分，所以如果我们在 tests 目录中添加更多文件，将会有更多的集成测试部分。

// We can still run a particular integration test function by specifying the test function’s name as an argument to cargo test.
// 我们仍然可以通过将测试函数的名称指定为 cargo test 的参数来运行特定的集成测试函数。
// To run all the tests in a particular integration test file, use the --test argument of cargo test followed by the name of the file:
// 要运行特定集成测试文件中的所有测试，请使用 cargo test 的 --test 参数后跟文件名：

// $ cargo test --test integration_test
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.64s
//      Running tests/integration_test.rs (target/debug/deps/integration_test-82e7799c1bc62298)
//
// running 1 test
// test it_adds_two ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// This command runs only the tests in the tests/integration_test.rs file.
// 此命令仅运行 tests/integration_test.rs 文件中的测试。
// Submodules in Integration Tests
// 集成测试中的子模块
// As you add more integration tests, you might want to make more files in the tests directory to help organize them;
// 当您添加更多集成测试时，您可能希望在测试目录中创建更多文件以帮助组织它们；
// for example, you can group the test functions by the functionality they’re testing.
// 例如，您可以根据测试的功能对测试函数进行分组。
// As mentioned earlier, each file in the tests directory is compiled as its own separate crate,
// 如前所述，tests 目录中的每个文件都被编译为自己单独的 crate，
// |- which is useful for creating separate scopes to more closely imitate the way end users will be using your crate.
// |- 这对于创建单独的范围以更接近地模仿最终用户使用您的箱子的方式很有用。
// However, this means files in the tests directory don’t share the same behavior as files in src do, as you learned in Chapter 7 regarding how to separate code into modules and files.
// 但是，这意味着 tests 目录中的文件与 src 中的文件不具有相同的行为，正如您在第 7 章中了解的有关如何将代码分成模块和文件的知识一样。

// The different behavior of tests directory files is most noticeable when you have a set of helper functions to use in multiple integration test files and
// 当你有一组辅助函数用于多个集成测试文件时，测试目录文件的不同行为是最明显的
// |- you try to follow the steps in the “Separating Modules into Different Files” section of Chapter 7 to extract them into a common module.
// |- 你尝试按照第 7 章“将模块分离到不同的文件”部分中的步骤将它们提取到一个公共模块中。
// For example, if we create tests/common.rs and place a function named setup in it, we can add some code to setup that we want to call from multiple test functions in multiple test files:
// 例如，如果我们创建 tests/common.rs 并在其中放置一个名为 setup 的函数，我们可以在 setup 中添加一些我们希望从多个测试文件中的多个测试函数调用的代码：

// Filename: tests/common.rs

pub fn setup() {
    // setup code specific to your library's tests would go here
}

// When we run the tests again, we’ll see a new section in the test output for the common.rs file,
// 当我们再次运行测试时，我们将在 common.rs 文件的测试输出中看到一个新部分，
// |- even though this file doesn’t contain any test functions nor did we call the setup function from anywhere:
// |- 即使这个文件不包含任何测试函数，我们也没有从任何地方调用 setup 函数：

// $ cargo test
//    Compiling adder v0.1.0 (file:///projects/adder)
//     Finished test [unoptimized + debuginfo] target(s) in 0.89s
//      Running unittests src/lib.rs (target/debug/deps/adder-92948b65e88960b4)
//
// running 1 test
// test tests::internal ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//      Running tests/common.rs (target/debug/deps/common-92948b65e88960b4)
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//      Running tests/integration_test.rs (target/debug/deps/integration_test-92948b65e88960b4)
//
// running 1 test
// test it_adds_two ... ok
//
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
//    Doc-tests adder
//
// running 0 tests
//
// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// Having common appear in the test results with running 0 tests displayed for it is not what we wanted.
// 让 common 出现在测试结果中并为其显示运行 0 个测试不是我们想要的。
// We just wanted to share some code with the other integration test files.
// 我们只是想与其他集成测试文件共享一些代码。

// To avoid having common appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs.
// 为了避免在测试输出中出现 common，我们将创建 tests/common/mod.rs 而不是创建 tests/common.rs。
// The project directory now looks like this:
// 项目目录现在看起来像这样：

// ├── Cargo.lock
// ├── Cargo.toml
// ├── src
// │   └── lib.rs
// └── tests
//     ├── common
//     │   └── mod.rs
//     └── integration_test.rs

// This is the older naming convention that Rust also understands that we mentioned in the “Alternate File Paths” section of Chapter 7.
// 这是 Rust 也理解的旧命名约定，我们在第 7 章的“替代文件路径”部分提到过。
// Naming the file this way tells Rust not to treat the common module as an integration test file.
// 以这种方式命名文件告诉 Rust 不要将公共模块视为集成测试文件。
// When we move the setup function code into tests/common/mod.rs and delete the tests/common.rs file, the section in the test output will no longer appear.
// 当我们将setup函数代码移到tests/common/mod.rs中，删除tests/common.rs文件后，测试输出中的部分将不再出现。
// Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.
// 测试目录子目录中的文件不会被编译为单独的箱子或在测试输出中有部分。

// After we’ve created tests/common/mod.rs, we can use it from any of the integration test files as a module.
// 在我们创建了 tests/common/mod.rs 之后，我们可以从任何集成测试文件中将它作为一个模块来使用。
// Here’s an example of calling the setup function from the it_adds_two test in tests/integration_test.rs:
// 这是从 tests/integration_test.rs 中的 it_adds_two 测试调用设置函数的示例：

// Filename: tests/integration_test.rs
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

// Note that the mod common; declaration is the same as the module declaration we demonstrated in Listing 7-21.
// 注意 mod 是通用的； 声明与我们在清单 7-21 中演示的模块声明相同。
// Then in the test function, we can call the common::setup() function.
// 然后在测试函数中，我们可以调用common::setup()函数。

// Integration Tests for Binary Crates
// 二进制包的集成测试
// If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file,
// 如果我们的项目是一个只包含 src/main.rs 文件而没有 src/lib.rs 文件的二进制 crate，
// |- we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement.
// |- 我们不能在 tests 目录中创建集成测试并使用 use 语句将 src/main.rs 文件中定义的函数引入作用域。
// Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.
// 只有库 crate 公开其他 crate 可以使用的函数； binary crates 是要自己运行的。

// This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file.
// 这是提供二进制文件的 Rust 项目有一个简单的 src/main.rs 文件的原因之一，该文件调用存在于 src/lib.rs 文件中的逻辑。
// Using that structure, integration tests can test the library crate with use to make the important functionality available.
// 使用该结构，集成测试可以使用 use 来测试库 crate，以使重要功能可用。
// If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.
// 如果重要功能正常工作，src/main.rs 文件中的少量代码也将正常工作，并且不需要测试该少量代码。

// Summary
// 概括
// Rust’s testing features provide a way to specify how code should function to ensure it continues to work as you expect, even as you make changes.
// Rust 的测试功能提供了一种方法来指定代码应该如何运行以确保它继续按预期工作，即使在您进行更改时也是如此。
// Unit tests exercise different parts of a library separately and can test private implementation details.
// 单元测试分别执行库的不同部分，并且可以测试私有实现细节。
// Integration tests check that many parts of the library work together correctly, and they use the library’s public API to test the code in the same way external code will use it.
// 集成测试检查库的许多部分是否正确协同工作，它们使用库的公共 API 以外部代码使用它的相同方式测试代码。
// Even though Rust’s type system and ownership rules help prevent some kinds of bugs, tests are still important to reduce logic bugs having to do with how your code is expected to behave.
// 尽管 Rust 的类型系统和所有权规则有助于防止某些类型的错误，但测试对于减少与代码预期行为方式有关的逻辑错误仍然很重要。

// Let’s combine the knowledge you learned in this chapter and in previous chapters to work on a project!
// 让我们结合你在本章和之前章节中学到的知识来做一个项目吧！

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        assert!(4, add(2, 2));
    }
}
