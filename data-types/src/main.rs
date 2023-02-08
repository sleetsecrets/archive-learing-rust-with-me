use std::io;

fn main() {
    // Every value in Rust is of a certain data type,
    // Rust中的每个值都是特定的数据类型，
    // which tells Rust what kind of data is being specified so it knows how to work with that data.
    // 它告诉Rust正在指定什么类型的数据，以便它知道如何处理该数据。
    // We’ll look at two data type subsets: scalar and compound.
    // 我们将看到两个数据类型的子集:标量和复合类型。

    // Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.
    // 请记住，Rust是一种静态类型语言，这意味着它必须在编译时知道所有变量的类型。
    // The compiler can usually infer what type we want to use based on the value and how we use it.
    // 编译器通常可以根据值及其使用方式推断出我们想使用的类型。
    // In cases when many types are possible,
    // 在可能有多种类型的情况下，
    // such as when we converted a String to a numeric type using parse in the “Comparing the Guess to the Secret Number” section in Chapter 2,
    // 比如在第2章的“比较猜的数字与秘密数字”一节中，我们使用parse将字符串转换为数字类型，
    // we must add a type annotation, like this:
    // 我们必须添加一个类型注解，如下所示:
    // ```rust
    // let guess: u32 = "42".parse().expect("Not a number!");
    // ```
    // If we don’t add the : u32 type annotation above,
    // 如果我们不添加上面的:u32类型注解，
    // Rust will display the following error, which means the compiler needs more information from us to know which type we want to use:
    // Rust将显示以下错误，这意味着编译器需要我们提供更多信息来知道我们想要使用哪种类型:
    // $ cargo build
    // Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
    // error[E0282]: type annotations needed
    // --> src/main.rs:2:9
    // |
    // 2 |     let guess = "42".parse().expect("Not a number!");
    // |         ^^^^^ consider giving `guess` a type

    // For more information about this error, try `rustc --explain E0282`.
    // error: could not compile `no_type_annotations` due to previous error

    // Scalar Types
    // 标量类型
    // A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    // 标量类型表示单个值Rust有四种主要的标量类型:整数、浮点数、布尔值和字符。
    // You may recognize these from other programming languages. Let’s jump into how they work in Rust.
    // 你可能在其他编程语言中见过它们。让我们来看看它们在Rust中是如何工作的。

    // Integer Types
    // 整数类型
    // An integer is a number without a fractional component. We used one integer type in Chapter 2, the u32 type.
    // 整数是没有小数部分的数我们在第2章中使用了一个整数类型——u32。
    // This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with i, instead of u) that takes up 32 bits of space.
    // 这个类型声明表明它关联的值应该是无符号整数(有符号整数类型以i开头，而不是u)，占用32位空间。
    // Table 3-1 shows the built-in integer types in Rust. We can use any of these variants to declare the type of an integer value.
    // 表3-1展示了Rust内置的整数类型。我们可以使用这些变量来声明整型值的类型。

    // Table 3-1: Integer Types in Rust
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize

    // Each variant can be either signed or unsigned and has an explicit size.
    // 每个变量都可以是有符号的或无符号的，并且有明确的大小。
    // Signed and unsigned refer to whether it’s possible for the number to be negative—in other words,
    // 有符号数(signed)和无符号数(unsigned)表示这个数是否可能为负数，换句话说，
    // whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned).
    // 这个数是否需要有符号(有符号)，还是只需要一个正数就可以表示(无符号)。
    // It’s like writing numbers on paper: when the sign matters, a number is shown with a plus sign or a minus sign; however, when it’s safe to assume the number is positive, it’s shown with no sign.
    // 这就像在纸上写数字一样:当符号很重要时，数字显示为加号或减号;然而，当可以放心地假设数字是正数时，它没有显示符号。
    // Signed numbers are stored using two’s complement representation.
    // 有符号数以二进制的补码形式存储

    // Each signed variant can store numbers from -(2^{n - 1}) to 2^{n - 1} - 1 inclusive, where n is the number of bits that variant uses.
    // 每个有符号变体可以存储从-(2^{n - 1})到2^{n - 1} - 1(包括2^{n - 1})的数字，其中n是变体使用的位数。
    // So an i8 can store numbers from -(2^7) to 2^7 - 1, which equals -128 to 127.
    // 所以i8可以存储从-(2^7)到2^7 - 1的数字，也就是-128到127
    // Unsigned variants can store numbers from 0 to 2^{n - 1}, so a u8 can store numbers from 0 to 2^{8 - 1}, which equals 0 to 255.
    // 无符号变量可以存储0到2^{n - 1}之间的数字，因此u8可以存储0到2^{8 - 1}之间的数字，即0到255。

    // Additionally, the isize and usize types depend on the architecture of the computer your program is running on,
    // 此外，isize和usize类型取决于运行程序的计算机的体系结构（架构），
    // which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
    // 在表中用"arch"表示: 64位体系结构（架构）是64位，32位是32位。

    // You can write integer literals in any of the forms shown in Table 3-2.
    // 整数字面量可以写成表3-2中的任何一种形式
    // Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type.
    // 注意，可以是多个数字类型的数字字面量允许使用类型后缀来指定类型，例如57u8。
    // Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.
    // 数字字面量也可以使用_作为视觉分隔符，使数字更容易阅读，例如1_000，它的值与指定1000的值相同。

    // Table 3-2: Integer Literals in Rust
    // Number literals	Example
    // Decimal	        98_222
    // Hex	            0xff
    // Octal	        0o77
    // Binary	        0b1111_0000
    // Byte (u8 only)	b'A'

    // Integer Overflow
    // 整数类型溢出
    // Let’s say you have a variable of type u8 that can hold values between 0 and 255.
    // 假设你有一个类型为u8的变量，可以保存0到255之间的值。
    // If you try to change the variable to a value outside of that range, such as 256, integer overflow will occur, which can result in one of two behaviors.
    // 如果你试图将变量更改为超出范围的值，例如256，就会发生整数溢出，这可能导致两种行为之一。
    // When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs.
    // 当你以调试模式编译时，Rust会检查是否有整数溢出，如果发生这种行为，会导致程序在运行时出现错误。
    // Rust uses the term panicking when a program exits with an error; we’ll discuss panics in more depth in the “Unrecoverable Errors with panic!” section in Chapter 9.
    // 当程序出现错误退出时，Rust使用恐慌（panics）这个术语;我们将在12.4节中更深入地讨论恐慌（panics）。第9章的一节。

    // When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics.
    // 当你使用——release标志在发布模式下编译时，Rust不会检查导致严重错误的整数溢出。
    // Instead, if overflow occurs, Rust performs two’s complement wrapping.
    // 相反，如果发生溢出，Rust会执行补码包装。
    // In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold.
    // 简而言之，大于类型所能容纳的最大值的值会被“换行”到类型所能容纳的最小值。
    // In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on.
    // 在u8中，值256变成0，值257变成1，以此类推。
    // The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have.
    // 程序不会出错，但变量的值可能和你预期的不一样
    // Relying on integer overflow’s wrapping behavior is considered an error.
    // 依赖整数溢出的包装行为被认为是错误的。

    // 依赖整型回绕被认为是一种错误，即便可能出现这种行为。如果你确实需要这种行为，标准库中有一个类型显式提供此功能，Wrapping。

    // Floating-Point Types
    // 浮点类型
    // Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.
    // Rust也有两种用于浮点数的基本类型，即带有小数点的数字。
    // Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.
    // Rust的浮点类型是f32和f64，它们分别是32位和64位。
    // The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
    // 默认类型是f64，因为在现代cpu上，它的速度与f32大致相同，但精度更高。
    // All floating-point types are signed.
    // 所有浮点类型都有符号

    // Here’s an example that shows floating-point numbers in action:
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Floating-point numbers are represented according to the IEEE-754 standard.
    // 浮点数按照IEEE-754标准表示。
    // The f32 type is a single-precision float, and f64 has double precision.
    // f32是单精度浮点数，而f64是双精度浮点数。

    // Numeric Operations
    // 整数值运算
    // Rust supports the basic mathematical operations you’d expect for all of the number types:
    // Rust支持所有数字类型的基本数学操作:
    // addition, subtraction, multiplication, division, and remainder.
    // 加，减，乘，除，余数
    // Integer division rounds down to the nearest integer.
    // 整数除法向下舍入为最接近的整数
    // The following code shows how you’d use each numeric operation in a let statement:
    // 下面的代码展示了如何在let语句中使用数值运算:

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;

    // Each expression in these statements uses a mathematical operator and evaluates to a single value, which is then bound to a variable.
    // 这些语句中的每个表达式都使用一个数学运算符，求值为一个值，然后绑定到一个变量。
    // Appendix B contains a list of all operators that Rust provides.
    // 附录B包含了Rust提供的所有操作符。

    // The Boolean Type
    // 布尔型
    // As in most other programming languages, a Boolean type in Rust has two possible values: true and false.
    // 与大多数其他编程语言一样，Rust中的布尔类型有两个可能的值:true和false。
    // Booleans are one byte in size.
    // 布尔值的大小为1字节。
    // The Boolean type in Rust is specified using bool. For example:
    // Rust中的布尔类型是用bool指定的。例如:

    let _t = true;

    let _f: bool = false; // with explicit type annotation

    // The Character Type
    // 字符类型
    // Rust’s char type is the language’s most primitive alphabetic type.
    // Rust的char类型是语言中最原始的字母类型
    // Here’s some examples of declaring char values:
    // 下面是一些声明char类型值的例子:

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes.
    // 注意，字符字面量使用单引号，而字符串字面量使用双引号。
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
    // Rust的char类型是4字节大小，表示Unicode标量值，这意味着它可以表示比ASCII多得多的东西。
    // Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.
    // 重音字母;中文、日文、韩文;emoji;零宽度空格在Rust中都是有效的字符值。
    // Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
    // Unicode标量值的范围从U+0000到U+D7FF，从U+E000到U+10FFFF(包括U+10FFFF)。
    // However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust.
    // 然而，“字符”在Unicode中并不是一个真正的概念，所以你对“字符”的直觉可能与Rust中的字符并不匹配。
    // We’ll discuss this topic in detail in “Storing UTF-8 Encoded Text with Strings” in Chapter 8.
    // 我们将在第8章“用字符串存储UTF-8编码的文本”中详细讨论这个主题。

    // Compound Types
    // 复合类型
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
    // 复合类型可以将多个值组合为一种类型。Rust有两种基本的复合类型：元组和数组。

    // The Tuple Type
    // 元组类型
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // 元组是一种将多种类型的值组合成一种复合类型的通用方法。
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // 元组有固定长度:一旦声明，它们就不能变大或变小。

    // We create a tuple by writing a comma-separated list of values inside parentheses.
    // 我们通过在括号中编写逗号分隔的值列表来创建元组。
    // Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
    // 元组中的每个位置都有一个类型，元组中不同值的类型不必相同。
    // We’ve added optional type annotations in this example:
    // 我们在这个例子中添加了可选类型注解:
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // The variable tup binds to the entire tuple, because a tuple is considered a single compound element.
    // 变量tup绑定到整个元组，因为元组被视为一个单一的复合元素。
    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
    // 为了从元组中获取单个值，我们可以使用模式匹配来解构元组值，如下所示:
    let tup = (500, 6.4, 1); // 这里不是为了遮蔽（shadow）而遮蔽，仅仅是解释上面的话。

    let (_x, _y, _z) = tup;

    // This program first creates a tuple and binds it to the variable tup.
    // 这个程序首先创建了一个元组并将其绑定到变量tup。
    // It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z.
    // 然后它使用let模式来获取tup并将其转换为三个独立的变量:x、y和z。
    // This is called destructuring, because it breaks the single tuple into three parts.
    // 这被称为解构(destructuring)，因为它将单个元组分解为三个部分。
    // Finally, the program prints the value of y, which is 6.4.
    // 最后，程序打印y的值，它是6.4。

    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:
    // 我们也可以通过句点(.)后跟我们想要访问的值的索引直接访问元组元素。例如:
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    // This program creates the tuple x and then accesses each element of the tuple using their respective indices.
    // 这个程序创建元组x，然后使用各自的索引访问元组中的每个元素。
    // As with most programming languages, the first index in a tuple is 0.
    // 和大多数编程语言一样，元组的第一个索引是0。

    // The tuple without any values has a special name, unit.
    // 没有值的元组有一个特殊的名称unit。
    // This value and its corresponding type are both written () and represent an empty value or an empty return type.
    // 这个值及其对应的类型都写为()，表示空值或空返回类型
    // Expressions implicitly return the unit value if they don’t return any other value.
    // 如果表达式不返回任何其他值，则隐式返回单位值（unit value）

    // The Array Type
    // 数组类型
    // Another way to have a collection of multiple values is with an array.
    // 另一种保存多个值的集合的方法是使用数组。
    // Unlike a tuple, every element of an array must have the same type.
    // 与元组不同的是，数组中的每个元素必须具有相同的类型
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.
    // 与其他一些语言中的数组不同，Rust中的数组具有固定长度。

    // We write the values in an array as a comma-separated list inside square brackets:
    // 我们将数组中的值写成方括号中逗号分隔的列表:
    let _a = [1, 2, 3, 4, 5];

    // Arrays are useful when you want your data allocated on the stack rather than the heap
    // 当你想在栈而不是堆上分配数据时，数组很有用
    // (we will discuss the stack and the heap more in Chapter 4) or when you want to ensure you always have a fixed number of elements.
    // (我们将在第4章详细讨论栈和堆)或者当你想确保元素的数量总是固定的时候。
    // An array isn’t as flexible as the vector type, though.
    // 不过，数组没有vector类型灵活。
    // A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.
    // vector是标准库提供的一种类似的集合类型，允许增大或缩小大小。
    // If you’re unsure whether to use an array or a vector, chances are you should use a vector.
    // 如果你不确定应该使用数组还是向量，那么你应该使用向量。
    // Chapter 8 discusses vectors in more detail.
    // 第8章会更详细地讨论向量

    // However, arrays are more useful when you know the number of elements will not need to change.
    // 然而，当你知道元素的数量不需要改变时，数组更有用。
    // For example, if you were using the names of the month in a program,
    // 例如，如果你在程序中使用月份的名称，
    // you would probably use an array rather than a vector because you know it will always contain 12 elements:
    // 你可能会使用数组而不是向量，因为你知道它总是包含12个元素:
    let _months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    // You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array,
    //  like so:
    // 你可以使用方括号来编写数组的类型，方括号中包含每个元素的类型，一个分号，然后是数组中的元素数量，
    //  像这样:
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    // Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.
    // 这里i32是每个元素的类型分号后面的数字5表示数组包含5个元素。

    // You can also initialize an array to contain the same value for each element by specifying the initial value,
    // 你也可以通过指定初始值来初始化数组，使每个元素都包含相同的值，
    // followed by a semicolon, and then the length of the array in square brackets, as shown here:
    // 后面是一个分号，然后是方括号中的数组长度，如下所示:
    let _a = [3; 5];
    // The array named a will contain 5 elements that will all be set to the value 3 initially.
    // 名为a的数组包含5个元素，它们最初的值都是3
    // This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
    // 这等同于写let a = [3, 3, 3, 3, 3];但是用一种更简洁的方式。

    // Accessing Array Elements
    // 访问数组元素
    // An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.
    // 数组是可以在栈上分配的固定大小的单个内存块。
    // You can access elements of an array using indexing, like this:
    // 你可以使用索引访问数组中的元素，如下所示:
    let a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];
    // In this example, the variable named first will get the value 1, because that is the value at index [0] in the array.
    // 在这个例子中，变量first的值为1，因为它是数组中索引[0]处的值。
    // The variable named second will get the value 2 from index [1] in the array.
    // 变量second将从数组的索引[1]中获取值2

    // Invalid Array Element Access
    // 无效的数组元素访问
    // Let’s see what happens if you try to access an element of an array that is past the end of the array.
    // 让我们看看如果你试图访问数组末尾的元素会发生什么。
    // Say you run this code, similar to the guessing game in Chapter 2, to get an array index from the user:
    // 假设你运行这段代码(类似于第2章的猜谜游戏)，从用户那里获取数组索引:

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // This code compiles successfully.
    // 这段代码能成功编译
    // If you run this code using cargo run and enter 0, 1, 2, 3, or 4, the program will print out the corresponding value at that index in the array.
    // 如果使用cargo run运行这段代码，并输入0、1、2、3或4，程序将打印出数组中该索引处对应的值。
    // If you instead enter a number past the end of the array, such as 10, you’ll see output like this:
    // 如果你输入一个超出数组末尾的数字，例如10，输出将如下所示:

    // sleetsecrets@lolita data-types % cargo run
    //     Compiling data-types v0.1.0 (/Users/sleetsecrets/Desktop/learn-rust-with-me/data-types)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.67s
    //     Running `target/debug/data-types`
    // Please enter an array index.
    // 6
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 6', src/main.rs:366:19
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // sleetsecrets@lolita data-types %

    // The program resulted in a runtime error at the point of using an invalid value in the indexing operation.
    // 程序在索引操作中使用了无效值，导致运行时错误。
    // The program exited with an error message and didn’t execute the final println! statement.
    // 程序退出并报错，没有执行最后的println!声明。
    // When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length.
    // 当你尝试使用索引访问元素时，Rust将检查你指定的索引是否小于数组长度。
    // If the index is greater than or equal to the length, Rust will panic.
    // 如果索引大于或等于长度，Rust会出错。
    // This check has to happen at runtime, especially in this case, because the compiler can’t possibly know what value a user will enter when they run the code later.
    // 这种检查必须在运行时进行，特别是在这种情况下，因为编译器不可能知道用户稍后运行代码时将输入什么值。

    // This is an example of Rust’s memory safety principles in action.
    // 这是Rust内存安全原则的一个例子。
    // In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed.
    // 在许多低级语言中，不会进行这种检查，当你提供了不正确的索引时，会访问无效的内存。
    // Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing.
    // Rust通过立即退出而不是允许内存访问并继续来防止这种错误。
    // Chapter 9 discusses more of Rust’s error handling and how you can write readable, safe code that neither panics nor allows invalid memory access.
    // 第9章更多地讨论Rust的错误处理，以及如何编写既不出错也不允许非法内存访问的可读、安全的代码。
}
