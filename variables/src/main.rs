fn main() {
    // Variables and Mutability

    // The error message indicates that the cause of the error is that you cannot assign twice to immutable variable `x`,
    // 错误信息表明，错误的原因是不能两次给不可变变量' x '赋值，
    // because you tried to assign a second value to the immutable x variable.
    // 因为你试图给不可变的x变量赋第二个值

    // It’s important that we get compile-time errors when we attempt to change a value that’s designated as immutable because this very situation can lead to bugs.
    // 当我们试图更改指定为不可变的值时，必须得到编译时错误，这很重要，因为这种情况可能会导致错误。
    // If one part of our code operates on the assumption that a value will never change and another part of our code changes that value,
    // 如果代码的一部分假设某个值永远不会改变，而代码的另一部分会改变这个值，
    // it’s possible that the first part of the code won’t do what it was designed to do.
    // 代码的第一部分可能不会按照设计的方式执行。
    // The cause of this kind of bug can be difficult to track down after the fact, especially when the second piece of code changes the value only sometimes.
    // 这种错误的原因很难在事后找到，特别是当第二段代码只是偶尔修改了值的时候。
    // The Rust compiler guarantees that when you state a value won’t change, it really won’t change, so you don’t have to keep track of it yourself.
    // Rust编译器保证当你声明一个值不会改变时，它真的不会改变，所以你不必自己跟踪它。
    // Your code is thus easier to reason through.
    // 因此你的代码更容易理解

    // But mutability can be very useful, and can make code more convenient to write.
    // 但是可变性非常有用，可以让代码更方便编写。
    // Although variables are immutable by default, you can make them mutable by adding mut in front of the variable name as you did in Chapter 2.
    // 虽然默认情况下变量是不可变的，但你可以像第2章那样，在变量名前加上mut，让它们可变。
    // Adding mut also conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable’s value.
    // 添加mut也可以通过指示代码的其他部分将改变该变量的值，来向代码的未来读者传递意图（mut 也向读者表明了其他代码将会改变这个变量值的意图）。
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    // Like immutable variables, constants are values that are bound to a name and are not allowed to change,
    // 和不可变变量一样，常量是绑定到名称的值，不允许更改，
    // but there are a few differences between constants and variables.
    // 但是常量和变量之间有一些区别

    // First, you aren’t allowed to use mut with constants.
    // 首先，不能对常量使用mut。
    // Constants aren’t just immutable by default—they’re always immutable.
    // 常量不仅在默认情况下是不可变的，它们永远是不可变的
    // You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.
    // 使用const关键字而不是let关键字声明常量，而且必须注释值的类型。
    // We’re about to cover types and type annotations in the next section, “Data Types,” so don’t worry about the details right now.
    // 我们将在下一节“数据类型”中介绍类型和类型注解，所以现在不要担心细节。
    // Just know that you must always annotate the type.
    // 只要知道你必须始终注释类型即可。

    // Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
    // 常量可以在任何作用域中声明，包括全局作用域，这使得它们对于很多代码都需要知道的值很有用。
    // The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
    // 最后一个区别是，常量只能设置为常量表达式，而不能设置为只能在运行时计算的值的结果。

    // Here’s an example of a constant declaration:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // The constant’s name is THREE_HOURS_IN_SECONDS and
    // 常量的名称是THREE_HOURS_IN_SECONDS
    // its value is set to the result of multiplying 60 (the number of seconds in a minute) by 60 (the number of minutes in an hour) by 3 (the number of hours we want to count in this program).
    // 它的值被设置为60(一分钟的秒数)乘以60(一小时的分钟数)乘以3(我们要计算的小时数)的结果。
    // Rust’s naming convention for constants is to use all uppercase with underscores between words.
    // Rust对常量的命名约定是在单词之间使用全大写和下划线。
    // The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand and verify,
    // 编译器能够在编译时执行有限的操作，这让我们选择以更容易理解和验证的方式写出这个值，
    // rather than setting this constant to the value 10,800.
    // 而不是将这个常量设置为10,800
    // See the Rust Reference’s section on constant evaluation for more information on what operations can be used when declaring constants.
    // 有关声明常量时可以使用哪些操作的更多信息，请参阅Rust参考资料中关于常量求值的部分。

    // Constants are valid for the entire time a program runs, within the scope they were declared in.
    // 常量在程序运行的整个时间内都是有效的，在它们被声明的作用域中
    // This property makes constants useful for values in your application domain that multiple parts of the program might need to know about,
    // 这个属性使得常量对于应用程序域中多个部分可能需要知道的值很有用，
    // such as the maximum number of points any player of a game is allowed to earn or the speed of light.
    // 例如游戏中任何玩家可以获得的最大点数或光速。

    // Naming hardcoded values used throughout your program as constants is useful in conveying the meaning of that value to future maintainers of the code.
    // 将程序中使用的硬编码值命名为常量，有助于向代码的维护者传达该值的含义。
    // It also helps to have only one place in your code you would need to change if the hardcoded value needed to be updated in the future.
    // 如果硬编码的值在未来需要更新时，在代码中只有一个地方需要更改，这也有帮助。
    println!(" THe value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // As you saw in the guessing game tutorial in Chapter 2, you can declare a new variable with the same name as a previous variable.
    // 就像在第2章的猜谜游戏教程中看到的那样，你可以声明一个与之前变量同名的新变量。
    // Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable.
    // Rustaceans们称之为第一个变量被第二个变量遮蔽了，这意味着当你使用变量名时，编译器将看到第二个变量。
    // In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends.
    // 实际上，第二个变量遮蔽了第一个变量，任何使用该变量名的操作都会被遮蔽，直到它自身被遮蔽，或者作用域结束。
    // We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows:
    // 我们可以通过使用相同的变量名并重复使用let关键字来屏蔽变量:
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
    // This program first binds x to a value of 5.
    // 这个程序首先将x绑定到值5
    // Then it creates a new variable x by repeating let x =, taking the original value and adding 1 so the value of x is then 6.
    // 然后它通过重复let x =创建一个新变量x，取原始值并加上1，因此x的值是6。
    // Then, within an inner scope created with the curly brackets, the third let statement also shadows x and creates a new variable, multiplying the previous value by 2 to give x a value of 12.
    // 然后，在用大括号创建的内部作用域中，第三条let语句也对x进行了遮盖（shadow），并创建了一个新变量，将之前的值乘以2，得到x的值为12。
    // When that scope is over, the inner shadowing ends and x returns to being 6.
    // 当作用域结束时，内部遮盖结束，x返回6。
    // When we run this program, it will output the following:
    // 运行这个程序，输出如下:
    // sleetsecrets@lolita variables % cargo run
    // Compiling variables v0.1.0 (/Users/sleetsecrets/Desktop/learn-rust-with-me/variables)
    // Finished dev [unoptimized + debuginfo] target(s) in 0.21s
    // Running `target/debug/variables`
    // --snip--
    // The value of y in the inner scope is: 12
    // The value of y is: 6
    // sleetsecrets@lolita variables %

    // Shadowing is different from marking a variable as mut,
    // 遮蔽与将变量标记为mut不同，
    // because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.
    // 因为如果不小心尝试给这个变量重新赋值而不使用let关键字，我们会得到编译时错误。
    // By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    // 通过使用let，我们可以对一个值进行一些变换（计算），但在这些变换（计算）完成后，变量是不可变的。

    // The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again,
    // mut和shadowing的另一个区别在于，当再次使用let关键字时，实际上是创建了一个新变量，
    // we can change the type of the value but reuse the same name.
    // 我们可以改变值的类型，但重用相同的名称
    // For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:
    // 例如，假设我们的程序要求用户输入空格，以显示他们希望在文本之间有多少空格，然后我们希望将输入存储为数字:
    let spaces = "   ";
    let spaces = spaces.len();
    // The first spaces variable is a string type and the second spaces variable is a number type.
    // 第一个spaces变量是字符串类型，第二个spaces变量是数字类型。
    // Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num;
    // 因此，遮蔽使我们不必使用不同的名称，例如spaces_str和spaces_num;
    // instead, we can reuse the simpler spaces name. However, if we try to use mut for this, as shown here, we’ll get a compile-time error:
    // 相反，我们可以重用更简单的spaces名称但是，如果像下面这样使用mut，就会得到一个编译时错误:
    let mut spaces = "   ";
    spaces = spaces.len();
    // The error says we’re not allowed to mutate a variable’s type:
    // 错误提示不允许改变变量的类型:
    // sleetsecrets@lolita variables % cargo run
    // Compiling variables v0.1.0 (/Users/sleetsecrets/Desktop/learn-rust-with-me/variables)
    // error[E0308]: mismatched types
    // --> src/main.rs:144:14
    //     |
    // 143 |     let mut spaces = "   ";
    //     |                      ----- expected due to this value
    // 144 |     spaces = spaces.len();
    //     |              ^^^^^^^^^^^^ expected `&str`, found `usize`

    // For more information about this error, try `rustc --explain E0308`.
    // error: could not compile `variables` due to previous error
    // sleetsecrets@lolita variables %
}
