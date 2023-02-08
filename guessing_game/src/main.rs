use rand::Rng;
// First, we add the line use rand::Rng.
// 首先，我们添加行 use rand::Rng。
// The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
// Rng 特征（trait）定义了随机数生成器实现的方法，这个特征（trait）必须在我们使用这些方法的范围内。
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // Next, we’re adding two lines in the middle.
    // 接下来，我们在中间添加两行。
    // In the first line, we call the rand::thread_rng function that gives us the particular random number generator that
    // 在第一行中，我们调用了 rand::thread_rng 函数，该函数为我们提供了特定的随机数生成器
    // we’re going to use: one that is local to the current thread of execution and seeded by the operating system.
    // 我们将使用：一个在当前执行线程本地并由操作系统播种的线程。
    // Then we call the gen_range method on the random number generator.
    // 然后我们调用随机数生成器的 gen_range 方法。
    // This method is defined by the Rng trait that we brought into scope with the use rand::Rng statement.
    // 此方法由我们使用 use rand::Rng 语句引入作用域的 Rng 特征定义。
    // The gen_range method takes a range expression as an argument and generates a random number in the range.
    // gen_range 方法以一个范围表达式作为参数，并生成一个范围内的随机数。
    // The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds,
    // 我们在这里使用的这种范围表达式采用 start..=end 的形式，并且包含下限和上限，
    // so we need to specify 1..=100 to request a number between 1 and 100.
    // 所以我们需要指定 1..=100 来请求 1 到 100 之间的数字。

    // Note: You won’t just know which traits to use and which methods and functions to call from a crate,
    // 注意：您不可能凭空就会知道要使用哪些特征以及要从 crate 调用哪些方法和函数，
    // so each crate has documentation with instructions for using it.
    // 所以每个 crate 都有文档和使用说明。
    // Another neat feature of Cargo is that running the cargo doc --open command will build documentation provided by all of your dependencies locally and open it in your browser.
    // Cargo 的另一个巧妙功能是运行 cargo doc --open 命令将在本地构建所有依赖项提供的文档，并在浏览器中打开它。
    // If you’re interested in other functionality in the rand crate, for example, run cargo doc --open and click rand in the sidebar on the left.
    // 如果您对 rand crate 中的其他功能感兴趣，例如，运行 cargo doc --open 并单击左侧边栏中的 rand。

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // The :: syntax in the ::new line indicates that new is an associated function of the String type.
        // ::new 那一行的 :: 语法表明 new 是 String 类型的一个 关联函数（associated function）。
        // 关联函数是针对类型实现的，在这个例子中是 String，而不是 String 的某个特定实例。一些语言中把它称为 静态方法（static method）。
        let mut guess = String::new();

        // By default, Rust has a set of items defined in the standard library that it brings into the scope of every program.
        // 默认情况下，Rust 在标准库中定义了一组项目，并将其引入每个程序的范围。
        // This set is called the prelude, and you can see everything in it in the standard library documentation.
        // 这个集合叫做序曲（prelude），你可以在标准库文档中看到里面的所有内容。
        // If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement.
        // 如果您要使用的类型不在序曲（prelude）中，则必须使用 use 语句显式将该类型引入作用域。
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // If we hadn’t imported the io library with use std::io at the beginning of the program, we could still use the function by writing this function call as std::io::stdin.
        // 如果我们没有在程序开头使用 use std::io 导入 io 库，我们仍然可以通过将此函数调用编写为 std::io::stdin 来使用该函数。
        // The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
        // stdin 函数返回 std::io::Stdin 的一个实例，它是一种表示终端标准输入句柄的类型。

        // The & indicates that this argument is a reference,
        // & 表示这个参数是一个引用，
        // which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        // 这为您提供了一种方法，让您的代码的多个部分访问一个数据，而无需多次将该数据复制到内存中。
        // References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references.
        // 引用是一个复杂的特性，Rust 的主要优势之一是使用引用是多么安全和容易。
        // You don’t need to know a lot of those details to finish this program.
        // 你不需要知道很多细节来完成这个程序。
        // For now, all you need to know is that like variables, references are immutable by default.
        // 现在，您需要知道的是，与变量一样，<!引用在默认情况下是不可变的!>。
        // Hence, you need to write &mut guess rather than &guess to make it mutable. (Chapter 4 will explain references more thoroughly.)
        // 因此，您需要编写 &mut guess 而不是 &guess 来使其可变。 （第 4 章将更详尽地解释参考文献。）

        // We’re still working on this line of code. We’re now discussing a third line of text, but note that it’s still part of a single logical line of code. The next part is this method:
        // 我们仍在处理这行代码。 我们现在正在讨论第三行文本，但请注意，它仍然是<!>单个逻辑代码行的一部分<!>。 下一部分是这个方法：`.expect("Failed to read line");`

        // As mentioned earlier, read_line puts whatever the user enters into the string we pass to it, but it also returns a Result value.
        // 如前所述，read_line 将用户输入的任何内容放入我们传递给它的字符串中，但它也会返回一个结果值。
        // Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states.
        // Result 是一个枚举（enumeration），通常称为枚举(enum)，它是一种可以处于多种可能状态之一的类型。
        // We call each possible state a variant.
        // 我们称每个可能的状态为变体（枚举成员）（variant）。

        // Values of the Result type, like values of any type, have methods defined on them.
        // Result 类型的值，与任何类型的值一样，都定义了方法。
        // An instance of Result has an expect method that you can call.
        // Result 的实例有一个可以调用的 expect 方法。
        // If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user’s input.
        // 如果这个 Result 实例是一个 Err 值，expect 将导致程序崩溃并显示您作为参数传递给 expect 的消息。
        // 如果 read_line 方法返回一个 Err，它很可能是来自底层操作系统的错误的结果。
        // 如果 Result 的这个实例是一个 Ok 值，expect 将获取 Ok 持有的返回值并将该值返回给您，以便您可以使用它。 在这种情况下，该值是用户输入中的字节数。

        // The core of the error states that there are mismatched types.
        // 错误的核心是存在不匹配的类型。
        // Rust has a strong, static type system. However, it also has type inference.
        // Rust 有一个强大的静态类型系统。 但是，它也有类型推断。
        // When we wrote let mut guess = String::new(), Rust was able to infer that guess should be a String and didn’t make us write the type.
        // 当我们写 let mut guess = String::new() 时，Rust 能够推断出 guess 应该是一个字符串，不需要让我们写出类型。
        // The secret_number, on the other hand, is a number type.
        // 另一方面，secret_number 是数字类型。
        // A few of Rust’s number types can have a value between 1 and 100: i32, a 32-bit number; u32, an unsigned 32-bit number; i64, a 64-bit number; as well as others.
        // 一些 Rust 的数字类型可以有 1 到 100 之间的值：i32，一个 32 位数字； u32，一个无符号的 32 位数字； i64，一个 64 位数字； 以及其他。
        // Unless otherwise specified, Rust defaults to an i32, which is the type of secret_number unless you add type information elsewhere that would cause Rust to infer a different numerical type.
        // 除非另有说明，否则 Rust 默认为 i32，它是 secret_number 的类型，除非你在其他地方添加类型信息，这会导致 Rust 推断出不同的数字类型。
        // The reason for the error is that Rust cannot compare a string and a number type.
        // 错误的原因是 Rust 无法比较字符串和数字类型。

        // Ultimately, we want to convert the String the program reads as input into a real number type so we can compare it numerically to the secret number.
        // 最终，我们希望将程序读取的字符串转换为实数类型，以便我们可以将其与 secret number 进行数值比较。
        // We do so by adding this line to the main function body:
        // 我们通过将这一行添加到主函数体来实现：
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            // The underscore, _, is a catchall value; in this example,
            // 下划线 _ 是一个包罗万象的值； 在这个例子中，
            // we’re saying we want to match all Err values, no matter what information they have inside them.
            // 我们是说我们想要匹配所有的 Err 值，不管它们里面有什么信息。
            // So the program will execute the second arm’s code, continue,
            // 所以程序会执行第二个 arm 的代码，continue，
            // which tells the program to go to the next iteration of the loop and ask for another guess.
            // 这告诉程序转到循环的下一次迭代并要求进行另一次猜测。
        };

        // We create a variable named guess.
        // 我们创建一个名为 guess 的变量。
        // But wait, doesn’t the program already have a variable named guess?
        // 但是等等，程序不是已经有一个名为 guess 的变量了吗？
        // It does, but helpfully Rust allows us to shadow the previous value of guess with a new one.
        // 它确实如此，但 Rust 允许我们用一个新值来遮盖（shadow） guess 的先前值。
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess for example.
        // Shadowing 让我们重用 guess 的变量名，而不是强迫我们创建两个唯一的变量，例如 guess_str 和 guess。
        // We’ll cover this in more detail in Chapter 3, but for now know that this feature is often used when you want to convert a value from one type to another type.
        // 我们将在第 3 章中对此进行更详细的介绍，但现在要知道，当您要将值从一种类型转换为另一种类型时，通常会用到此功能。

        // We bind this new variable to the expression guess.trim().parse().
        // 我们将这个新变量绑定到表达式 guess.trim().parse()。
        // The guess in the expression refers to the original guess variable that contained the input as a string.
        // 表达式中的 guess 指的是包含输入字符串的原始 guess 变量。
        // The trim method on a String instance will eliminate any whitespace at the beginning and end,
        // String 实例上的 trim 方法将消除开头和结尾的任何空格，
        // which we must do to be able to compare the string to the u32, which can only contain numerical data.
        // 我们必须这样做才能将字符串与只能包含数字数据的 u32 进行比较。
        // The user must press enter to satisfy read_line and input their guess, which adds a newline character to the string.
        // 用户必须按回车键以满足 read_line 并输入他们的猜测，这会向字符串添加一个换行符。
        // For example, if the user types 5 and presses enter, guess looks like this: 5\n. The \n represents “newline”.
        // 例如，如果用户键入 5 并按回车键，则猜测如下所示：5\n。 \n 代表“换行符”。
        // (On Windows, pressing enter results in a carriage return and a newline, \r\n).
        //（在 Windows 上，按回车键会产生一个回车符和一个换行符，\r\n）。
        // The trim method eliminates \n or \r\n, resulting in just 5.
        // trim 方法消除了 \n 或 \r\n，结果只有 5。

        // The parse method on strings converts a string to another type.
        // 字符串的 parse 方法将字符串转换为另一种类型。
        // Here, we use it to convert from a string to a number. We need to tell Rust the exact number type we want by using let guess: u32.
        // 在这里，我们使用它将字符串转换为数字。 我们需要使用 let guess: u32 告诉 Rust 我们想要的确切数字类型。
        // The colon (:) after guess tells Rust we’ll annotate the variable’s type.
        // guess 之后的冒号 (:) 告诉 Rust 我们将注释变量的类型。
        // Rust has a few built-in number types; the u32 seen here is an unsigned, 32-bit integer.
        // Rust 有一些内置的数字类型； 此处看到的 u32 是一个无符号的 32 位整数。
        // It’s a good default choice for a small positive number.
        // 对于较小的正数，这是一个很好的默认选择。
        // You’ll learn about other number types in Chapter 3.
        // 您将在第 3 章中了解其他数字类型。
        // Additionally, the u32 annotation in this example program and the comparison with secret_number means that Rust will infer that secret_number should be a u32 as well.
        // 此外，此示例程序中的 u32 注释以及与 secret_number 的比较意味着 Rust 将推断 secret_number 也应该是 u32。
        // So now the comparison will be between two values of the same type!
        // 所以现在比较将在相同类型的两个值之间进行！

        // The parse method will only work on characters that can logically be converted into numbers and so can easily cause errors.
        // parse 方法只适用于逻辑上可以转换为数字的字符，因此很容易出错。
        // If, for example, the string contained A👍%, there would be no way to convert that to a number.
        // 例如，如果字符串包含 A👍%，则无法将其转换为数字。
        // Because it might fail, the parse method returns a Result type, much as the read_line method does (discussed earlier in “Handling Potential Failure with the Result Type”).
        // 因为它可能会失败，所以 parse 方法返回一个结果类型，就像 read_line 方法所做的那样（在前面的“使用结果类型（Result Type）处理潜在的失败”中讨论过）。
        // We’ll treat this Result the same way by using the expect method again.
        // 我们将再次使用 expect 方法以同样的方式处理这个结果。
        // If parse returns an Err Result variant because it couldn’t create a number from the string, the expect call will crash the game and print the message we give it.
        // 如果 parse 返回一个 Err Result variant，因为它无法从字符串中创建一个数字，则 expect 调用将使游戏崩溃并打印我们给它的消息。
        // If parse can successfully convert the string to a number, it will return the Ok variant of Result, and expect will return the number that we want from the Ok value.
        // 如果 parse 能成功将字符串转为数字，就会返回 Result 的 Ok variant，expect 会从 Ok 值中返回我们想要的数字。

        println!("You guessed: {guess}");

        // Remember that a crate is a collection of Rust source code files.
        // 请记住，crate 是 Rust 源代码文件的集合。
        // The project we’ve been building is a binary crate, which is an executable.
        // 我们一直在构建的项目是一个二进制包（binary crate），它是一个可执行文件。
        // https://crates.io/crates/rand
        // The rand crate is a library crate, which contains code intended to be used in other programs and can't be executed on its own.
        // rand crate 是一个库 crate（library crate），其中包含旨在用于其他程序且不能单独执行的代码。

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        // First we add another use statement, bringing a type called std::cmp::Ordering into scope from the standard library.
        // 首先，我们添加另一个 use 语句，将标准库中名为 std::cmp::Ordering 的类型引入范围。
        // The Ordering type is another enum and has the variants Less, Greater, and Equal.
        // Ordering 类型是另一个枚举类型，具有 Less、Greater 和 Equal 变体。
        // These are the three outcomes that are possible when you compare two values.
        // 这些是比较两个值时可能出现的三种结果。

        // Then we add five new lines at the bottom that use the Ordering type.
        // 然后我们在底部添加五个使用 Ordering 类型的新行。
        // The cmp method compares two values and can be called on anything that can be compared.
        // cmp 方法比较两个值，可以在任何可以比较的情况下调用。
        // It takes a reference to whatever you want to compare with: here it’s comparing the guess to the secret_number.
        // 它引用了你想要比较的任何内容：这里是将 guess 与 secret_number 进行比较。
        // Then it returns a variant of the Ordering enum we brought into scope with the use statement.
        // 然后它返回我们使用 use 语句引入范围的 Ordering 枚举的变体。
        // We use a match expression to decide what to do next based on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.
        // 我们使用 match 表达式来决定下一步要做什么，这取决于调用 cmp 时返回的 Ordering 枚举成员（变体）（variant）。

        // A match expression is made up of arms. An arm consists of a pattern to match against,
        // 匹配表达式（match expression）由 arms 组成。 arm 由要匹配（match）的模式（pattern）组成，
        // and the code that should be run if the value given to match fits that arm’s pattern.
        // 如果匹配的值符合该 arm 的模式，则应该运行的代码。
        // Rust takes the value given to match and looks through each arm’s pattern in turn.
        // Rust 接受给定的值来匹配并依次查看每个 arm 的模式。
        // Patterns and the match construct are powerful Rust features that let you express a variety of situations your code might encounter and make sure that you handle them all.
        // Patterns 和 match construct 是强大的 Rust 特性，可让您表达代码可能遇到的各种情况，并确保您处理所有这些情况。
        // These features will be covered in detail in Chapter 6 and Chapter 18, respectively.
        // 这些功能将分别在第 6 章和第 18 章中详细介绍。
    }
}
