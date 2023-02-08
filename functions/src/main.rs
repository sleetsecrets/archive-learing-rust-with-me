fn five() -> i32 {
    5
}

// Functions are prevalent in Rust code.
//函数在Rust代码中很常见。
// You’ve already seen one of the most important functions in the language: the main function, which is the entry point of many programs.
//你已经见过这门语言中最重要的函数之一:main函数，它是许多程序的入口。
// You’ve also seen the fn keyword, which allows you to declare new functions.
//你也见过fn关键字，它允许你声明新函数。
fn main() {
    println!("Hello, world!");

    // We can call any function we’ve defined by entering its name followed by a set of parentheses.
    // 我们可以通过输入函数名和一对括号来调用任何定义的函数。
    // Because another_function is defined in the program, it can be called from inside the main function.
    // 因为在程序中定义了another_function，所以可以在主函数中调用它
    // Note that we defined another_function after the main function in the source code; we could have defined it before as well.
    // 注意，在源代码中，我们在main函数之后定义了another_function;我们之前也可以定义它。
    // Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
    // Rust不关心你在哪里定义你的函数，只关心它们定义在调用者可以看到的作用域中的某处。
    another_function(5);

    // The lines execute in the order in which they appear in the main function.
    // 这些行按照它们在main函数中出现的顺序执行
    // First, the “Hello, world!” message prints, and then another_function is called and its message is printed.
    // 首先是" Hello, world! "，然后调用another_function并打印它的消息。

    print_labeled_measurement(5, 'h');

    // Statements and Expressions
    // 语句和表达式
    // Function bodies are made up of a series of statements optionally ending in an expression.
    // 函数体由一系列可选，以表达式结尾的语句组成
    // So far, the functions we’ve covered haven’t included an ending expression, but you have seen an expression as part of a statement.
    // 到目前为止，我们介绍的函数都没有包含结束表达式，但你已经看到表达式是语句的一部分。
    // Because Rust is an expression-based language, this is an important distinction to understand.
    // 因为Rust是一种基于表达式的语言，这是一个需要理解的重要区别。
    // Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.
    // 其他语言没有这样的区别，所以让我们看看什么是语句和表达式，以及它们的差异如何影响函数体。

    // Statements are instructions that perform some action and do not return a value.
    // 语句（Statements）是执行某种操作但不返回值的指令。
    // Expressions evaluate to a resulting value.
    // 表达式（Expressions）计算并产生一个值。
    // Let’s look at some examples.
    // 让我们看一些例子。

    // We’ve actually already used statements and expressions.
    // 实际上我们已经使用了语句和表达式
    // Creating a variable and assigning a value to it with the let keyword is a statement.
    // 使用let关键字创建变量并赋值是一个语句。
    // In Listing 3-1, let y = 6; is a statement.
    // 在代码清单3-1中，let y = 6;是一个语句。

    // Listing 3-1: A main function declaration containing one statement
    // fn main() {
    //     let y = 6;
    // }

    // Function definitions are also statements; the entire preceding example is a statement in itself.
    // 函数定义也是语句;前面的整个例子本身就是一条语句。

    // Statements do not return values.
    // 语句不会返回值
    // Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:
    // 因此，你不能像下面的代码那样将let语句赋值给另一个变量。你会得到一个错误:

    // fn main() {
    //     let x = (let y = 6);
    // }

    // When you run this program, the error you’ll get looks like this:
    // 运行这个程序时，你会得到这样的错误:

    // $ cargo run
    // Compiling functions v0.1.0 (file:///projects/functions)
    // error: expected expression, found statement (`let`)
    // --> src/main.rs:2:14
    // |
    // 2 |     let x = (let y = 6);
    // |              ^^^^^^^^^
    // |
    // = note: variable declaration using `let` is a statement

    // error[E0658]: `let` expressions in this position are unstable
    // --> src/main.rs:2:14
    // |
    // 2 |     let x = (let y = 6);
    // |              ^^^^^^^^^
    // |
    // = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

    // warning: unnecessary parentheses around assigned value
    // --> src/main.rs:2:13
    // |
    // 2 |     let x = (let y = 6);
    // |             ^         ^
    // |
    // = note: `#[warn(unused_parens)]` on by default
    // help: remove these parentheses
    // |
    // 2 -     let x = (let y = 6);
    // 2 +     let x = let y = 6;
    // |

    // For more information about this error, try `rustc --explain E0658`.
    // warning: `functions` (bin "functions") generated 1 warning
    // error: could not compile `functions` due to 2 previous errors; 1 warning emitted

    // The let y = 6 statement does not return a value, so there isn’t anything for x to bind to.
    // let y = 6语句没有返回值，因此没有任何东西可以绑定x
    // This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment.
    // 这与其他语言(如C和Ruby)不同，在其他语言中，赋值操作会返回赋值操作的值。
    // In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.
    // 在这些语言中，你可以写x = y = 6，并且x和y的值都是6;但在Rust中不是这样的。

    // Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust.
    // 表达式求值为一个值，并构成了你将在Rust中编写的大部分代码。
    // Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11.
    // 考虑一个数学运算，例如5 + 6，它是一个计算值为11的表达式。
    // Expressions can be part of statements: in Listing 3-1, the 6 in the statement let y = 6; is an expression that evaluates to the value 6.
    // 表达式可以作为语句的一部分:在代码清单3-1中，语句let y = 6;是一个计算值为6的表达式。
    // Calling a function is an expression. Calling a macro is an expression.
    // 调用函数是一个表达式。调用宏是一个表达式。
    // A new scope block created with curly brackets is an expression, for example:
    // 用大括号创建的作用域块是一个表达式，例如:

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // This expression:
    // {
    //     let x = 3;
    //     x + 1
    // }

    // is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement.
    // 在这个例子中，计算结果为4。作为let语句的一部分，这个值绑定到y。
    // Note that the x + 1 line doesn’t have a semicolon at the end, unlike most of the lines you’ve seen so far.
    // 请注意，x + 1行末尾没有分号，这与你迄今为止看到的大多数行不同。
    // Expressions do not include ending semicolons.
    // 表达式中不包含结束分号
    // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    // 如果在表达式末尾添加分号，则将其转换为语句，然后它将不会返回值。
    // Keep this in mind as you explore function return values and expressions next.
    // 在接下来探索函数返回值和表达式时，请记住这一点。

    // Functions with Return Values
    // 有返回值的函数
    // Functions can return values to the code that calls them.
    // 函数可以向调用它们的代码返回值
    // We don’t name return values, but we must declare their type after an arrow (->).
    // 我们不命名返回值，但我们必须在箭头之后声明它们的类型(->)。
    // In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
    // 在Rust中，函数的返回值等同于函数体块中最终表达式的值。
    // You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
    // 使用return关键字并指定一个值，可以提前从函数返回，但大多数函数都会隐式返回最后一个表达式。
    // Here’s an example of a function that returns a value:
    // 下面是一个有返回值的函数示例:
    let x = five();

    println!("The value of x is: {x}");

    // There are no function calls, macros, or even let statements in the five function—just the number 5 by itself.
    // 在five函数中没有函数调用、宏，甚至没有let语句，只有数字5本身。
    // That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as -> i32.
    // 这在Rust中是一个完全有效的函数。请注意，函数的返回类型也指定了，为-> i32。
    // Try running this code; the output should look like this:
    // 试着运行这段代码;输出应该像这样:

    // sleetsecrets@lolita functions % cargo run
    //    Compiling functions v0.1.0 (/Users/sleetsecrets/Desktop/learn-rust-with-me/functions)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.86s
    //      Running `target/debug/functions`
    // ----snip----
    // The value of x is: 5
    // sleetsecrets@lolita functions %

    // The 5 in five is the function’s return value, which is why the return type is i32.
    // 5中的5是函数的返回值，这也是返回类型为i32的原因。
    // Let’s examine this in more detail.
    // 让我们更详细地研究一下。
    // There are two important bits: first, the line let x = five(); shows that we’re using the return value of a function to initialize a variable.
    // 有两个重要的比特位:第一个是let x = five();说明我们使用函数的返回值来初始化变量。
    // Because the function five returns a 5, that line is the same as the following:
    // 因为函数five的返回值是5，所以这一行代码和下面一样:
    let x = 5;

    println!("The value of x is: {x}");

    // Second, the five function has no parameters and defines the type of the return value,
    // 其次，five函数没有参数，并且定义了返回值的类型，
    // but the body of the function is a lonely 5 with no semicolon because it’s an expression whose value we want to return.
    // 但是函数体是一个没有分号的5，因为它是一个我们想要返回值的表达式。

    // Let’s look at another example:
    // 来看另一个例子:

    let y = plus_one(5);

    println!("The value of y is: {y}");

    // Running this code will print The value of x is: 6.
    // 运行这段代码将打印x的值为:6
    // But if we place a semicolon at the end of the line containing x + 1, changing it from an expression to a statement, we’ll get an error.
    // 但是，如果我们在包含x + 1的行末尾放置一个分号，将它从表达式更改为语句，我们将得到一个错误。

    // fn main() {
    //     let x = plus_one(5);

    //     println!("The value of x is: {x}");
    // }

    // fn plus_one(x: i32) -> i32 {
    //     x + 1;
    // }

    // Compiling this code produces an error, as follows:
    // 编译这段代码会产生一个错误，如下:
    // $ cargo run
    //     Compiling functions v0.1.0 (file:///projects/functions)
    // error[E0308]: mismatched types
    // --> src/main.rs:7:24
    // |
    // 7 | fn plus_one(x: i32) -> i32 {
    // |    --------            ^^^ expected `i32`, found `()`
    // |    |
    // |    implicitly returns `()` as its body has no tail or `return` expression
    // 8 |     x + 1;
    // |          - help: remove this semicolon

    // For more information about this error, try `rustc --explain E0308`.
    // error: could not compile `functions` due to previous error

    // The main error message, “mismatched types,” reveals the core issue with this code.
    // 主要的错误信息" mismatch types "揭示了这段代码的核心问题。
    // The definition of the function plus_one says that it will return an i32, but statements don’t evaluate to a value, which is expressed by (), the unit type.
    // 根据plus_one函数的定义，它将返回i32，不过语句并不会返回值，使用单位类型 () 表示不返回值。
    // Therefore, nothing is returned, which contradicts the function definition and results in an error.
    // 因此，没有返回任何东西，这与函数定义相矛盾，并导致错误。
    // In this output, Rust provides a message to possibly help rectify this issue: it suggests removing the semicolon, which would fix the error.
    // 在此输出中，Rust提供了一条消息，可能有助于纠正此问题:它建议删除分号，这将修复错误。

}

fn plus_one(y: i32) -> i32 {
    y + 1
}

// Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.
// Rust代码使用snake case作为函数和变量名的常规风格，其中所有字母都是小写，下划线分隔单词。
// Here’s a program that contains an example function definition:
//下面是一个包含函数定义示例的程序:


// Parameters
// We can define functions to have parameters, which are special variables that are part of a function’s signature.
// 我们可以定义具有参数（parameter）的函数，这些参数是函数签名中的特殊变量。
// When a function has parameters, you can provide it with concrete values for those parameters.
// 当函数有参数时，你可以为这些参数提供具体的值。
// Technically, the concrete values are called arguments, but in casual conversation, people tend to use the words parameter and argument interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.
// 从技术上讲，具体的值称为实参（argument），但在日常交流中，人们倾向于将形参（parameter）和实参（argument）互换表示函数定义中的变量或调用函数时传入的具体值。
fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

// We define a function in Rust by entering fn followed by a function name and a set of parentheses.
// 在Rust中，我们通过输入fn，后跟函数名称和一组圆括号来定义函数。
// The curly brackets tell the compiler where the function body begins and ends.
// 大括号告诉编译器函数体在哪里开始和结束。

// The declaration of another_function has one parameter named x.
// another_function的声明中有一个名为x的参数
// The type of x is specified as i32. When we pass 5 in to another_function,
// 将x的类型指定为i32。当我们将5传递给another_function时，
// the println! macro puts 5 where the pair of curly brackets containing x was in the format string.
// println!宏将5放在格式字符串中包含x的大括号对的地方。

// In function signatures, you must declare the type of each parameter.
// 在函数签名中，必须声明每个参数的类型。
// This is a deliberate decision in Rust’s design:
// 这是Rust设计中经过深思熟虑的决定:
//  requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean.
//  在函数定义中使用类型注解意味着编译器几乎不需要你在代码的其他地方使用它们来确定你想要的类型。
// The compiler is also able to give more helpful error messages if it knows what types the function expects.
// 如果编译器知道函数需要什么类型，它也能够提供更有帮助的错误消息。

// When defining multiple parameters, separate the parameter declarations with commas, like this:
// 定义多个参数时，参数声明之间用逗号分隔，如下所示:
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// This example creates a function named print_labeled_measurement with two parameters.
// 这个例子创建了一个名为print_labeled_measurement的函数，它有两个参数。
// The first parameter is named value and is an i32.
// 第一个参数名为value，是i32类型。
// The second is named unit_label and is type char.
// 第二个变量名为unit_label，类型为char。
// The function then prints text containing both the value and the unit_label.
// 然后函数打印包含值和unit_label的文本。
