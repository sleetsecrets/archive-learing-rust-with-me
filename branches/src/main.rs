fn main() {
    // if Expressions
    // if表达式
    // An if expression allows you to branch your code depending on conditions.
    // if表达式允许你根据条件进行代码分支
    // You provide a condition and then state,
    // 你提供一个条件，然后声明，
    // “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”
    // "如果条件满足，运行这段代码。如果条件不满足，就不要运行这段代码。”

    // All if expressions start with the keyword if, followed by a condition.
    // 所有的if表达式都以if关键字开头，后面跟着一个条件。
    // In this case, the condition checks whether or not the variable number has a value less than 5.
    // 在这个例子中，条件检查变量number的值是否小于5。
    // We place the block of code to execute if the condition is true immediately after the condition inside curly brackets.
    // 我们将在条件为真时执行的代码块放在大括号中紧跟在条件之后。
    // Blocks of code associated with the conditions in if expressions are sometimes called arms,
    // if表达式中与条件相关联的代码块有时称为arms，
    // just like the arms in match expressions that we discussed in the “Comparing the Guess to the Secret Number” section of Chapter 2.
    // 就像我们在第2章“比较猜测和秘密数字”一节中讨论的匹配表达式中的手臂一样。

    // Optionally, we can also include an else expression, which we chose to do here,
    // 可选地，我们还可以包含一个else表达式，我们在这里选择这样做，
    // to give the program an alternative block of code to execute should the condition evaluate to false.
    // 给程序一个替代的代码块，以便在条件求值为false时执行。
    // If you don’t provide an else expression and the condition is false,
    // 如果你没有提供else表达式，并且条件为false，
    // the program will just skip the if block and move on to the next bit of code.
    // 程序将跳过if代码块，跳到下一段代码。

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // It’s also worth noting that the condition in this code must be a bool.
    // 值得注意的是，这段代码中的条件必须是布尔值。
    // If the condition isn’t a bool, we’ll get an error. For example, try running the following code:
    // 如果条件不是bool类型，我们会得到一个错误例如，试着运行以下代码:

    // let number = 3;

    // if number {
    //     println!("number was three");
    // }

    // sleetsecrets@lolita branches % cargo run
    // Compiling branches v0.1.0 (/Users/sleetsecrets/Desktop/learn-rust-with-me/branches)
    // error[E0308]: mismatched types
    // --> src/main.rs:46:8
    // |
    // 46 |     if number {
    // |        ^^^^^^ expected `bool`, found integer

    // For more information about this error, try `rustc --explain E0308`.
    // error: could not compile `branches` due to previous error
    // sleetsecrets@lolita branches %

    // Handling Multiple Conditions with else if
    // 使用else if处理多个条件
    // You can use multiple conditions by combining if and else in an else if expression. For example:
    // 你可以在else if表达式中结合if和else来使用多个条件。例如:

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // This program has four possible paths it can take.
    // 这个程序有4条可能的路径。
    // After running it, you should see the following output:
    // 运行后，应该能看到如下输出:

    // sleetsecrets@lolita branches % cargo run
    // Compiling branches v0.1.0 (/Users/sleetsecrets/Desktop/learn-rust-with-me/branches)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.75s
    //     Running `target/debug/branches`
    // condition was true
    // number is divisible by 3
    // sleetsecrets@lolita branches %

    // When this program executes, it checks each if expression in turn and executes the first body for which the condition holds true.
    // 当这个程序执行时，它依次检查每个if表达式，并执行条件为真的第一个主体。
    // Note that even though 6 is divisible by 2,
    // 注意，虽然6能被2整除，
    // we don’t see the output number is divisible by 2, nor do we see the number is not divisible by 4, 3, or 2 text from the else block.
    // 我们没有看到输出的数字能被2整除，也没有看到数字不能被else代码块中的文本4、3或2整除。
    // That’s because Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.
    // 这是因为Rust只执行第一个真实条件的代码块，一旦它找到了一个，它甚至不会检查其余的。

    // Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code.
    // 使用太多else if表达式会使代码混乱，所以如果你有多个else if表达式，你可能需要重构代码。
    // Chapter 6 describes a powerful Rust branching construct called match for these cases.
    // 第6章描述了一个强大的Rust分支结构match，用于解决这些情况。

    // Using if in a let Statement
    // 在let语句中使用if
    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable, as in Listing 3-2.
    // 因为if是一个表达式，所以可以在let语句的右侧使用它，将结果赋值给一个变量，如代码清单3-2所示。

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    // Listing 3-2: Assigning the result of an if expression to a variable
    // 代码清单3-2:将if表达式的结果赋值给变量

    // The number variable will be bound to a value based on the outcome of the if expression. Run this code to see what happens:
    // 变量number会根据if表达式的结果绑定到一个值上。运行这段代码，看看会发生什么:

    // sleetsecrets@lolita branches % cargo run
    // Compiling branches v0.1.0 (/Users/sleetsecrets/Desktop/learn-rust-with-me/branches)
    // Finished dev [unoptimized + debuginfo] target(s) in 0.83s
    // Running `target/debug/branches`
    // ----snip----
    // The value of number is: 5
    // sleetsecrets@lolita branches %

    // Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions.
    // 记住，代码块的计算结果是最后一个表达式，数字本身也是表达式。
    // In this case, the value of the whole if expression depends on which block of code executes.
    // 在这种情况下，整个if表达式的值取决于执行的代码块。
    // This means the values that have the potential to be results from each arm of the if must be the same type;
    // 这意味着可能成为if语句中每个分支的结果的值必须是相同的类型;
    // in Listing 3-2, the results of both the if arm and the else arm were i32 integers.
    // 在代码清单3-2中，if和else分支的结果都是i32个整数。
    // If the types are mismatched, as in the following example, we’ll get an error:
    // 如果类型不匹配，就像下面的例子一样，我们将得到一个错误:

    // let condition = true;

    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");

    // When we try to compile this code, we’ll get an error.
    // 当我们尝试编译这些代码时，我们会得到一个错误。
    // The if and else arms have value types that are incompatible, and Rust indicates exactly where to find the problem in the program:
    // if和else分支的值类型是不兼容的，Rust精确地指出了在程序中的何处查找问题:

    // sleetsecrets@lolita branches % cargo run
    // Compiling branches v0.1.0 (/Users/sleetsecrets/Desktop/learn-rust-with-me/branches)
    // error[E0308]: `if` and `else` have incompatible types
    // --> src/main.rs:142:44
    //     |
    // 142 |     let number = if condition { 5 } else { "six" };
    //     |                                 -          ^^^^^ expected integer, found `&str`
    //     |                                 |
    //     |                                 expected because of this

    // For more information about this error, try `rustc --explain E0308`.
    // error: could not compile `branches` due to previous error
    // sleetsecrets@lolita branches %

    // The expression in the if block evaluates to an integer, and the expression in the else block evaluates to a string.
    // if代码块中的表达式求值为整数，else代码块中的表达式求值为字符串
    // This won’t work because variables must have a single type,
    // 这行不通，因为变量必须只有一种类型，
    //  and Rust needs to know at compile time what type the number variable is, definitively.
    //  Rust需要在编译时明确知道number变量的类型。
    // Knowing the type of number lets the compiler verify the type is valid everywhere we use number.
    // 知道number的类型可以让编译器在任何使用number的地方验证类型是否有效。
    // Rust wouldn’t be able to do that if the type of number was only determined at runtime;
    // 如果number的类型仅在运行时确定，Rust将无法做到这一点;
    // the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.
    // 如果编译器必须跟踪任意变量的多个假设类型，那么它将变得更加复杂，并且对代码的保证也会更少。
}
