// An Example Program Using Structs
// 一个使用结构体的示例程序
// To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle.
// 为了理解什么时候需要使用结构体，让我们编写一个计算矩形面积的程序。
// We’ll start by using single variables, and then refactor the program until we’re using structs instead.
// 我们将从使用单变量开始，然后重构程序直到使用结构体。

// Let’s make a new binary project with Cargo called rectangles that will take the width and height of a rectangle specified in pixels and calculate the area of the rectangle.
// 让我们创建一个新的二进制项目，其中包含rectangle对象，它将获取一个以像素为单位的矩形的宽度和高度，并计算矩形的面积。
// Listing 5-8 shows a short program with one way of doing exactly that in our project’s src/main.rs.
// 代码清单5-8展示了一个简短的程序，其中的一种方法正是在项目的src/main.rs中实现的。
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
// Listing 5-8: Calculating the area of a rectangle specified by separate width and height variables
//代码清单5-8:计算由单独的width和height变量指定的矩形的面积

// Now, run this program using cargo run:
// 现在，使用cargo run运行这个程序:

// $ cargo run
//    Compiling rectangles v0.1.0 (file:///projects/rectangles)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.42s
//      Running `target/debug/rectangles`
// The area of the rectangle is 1500 square pixels.

// This code succeeds in figuring out the area of the rectangle by calling the area function with each dimension, but we can do more to make this code clear and readable.
// 这段代码通过对每个维度调用面积函数，成功地计算出了矩形的面积，但我们还可以做更多的工作，使代码更加清晰和可读。

// The issue with this code is evident in the signature of area:
// 这段代码的问题在area的签名中很明显:

fn area(width: u32, height: u32) -> u32 {}

// The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters,
// 面积函数应该计算一个矩形的面积，但我们编写的函数有两个参数，
// and it’s not clear anywhere in our program that the parameters are related.
// 在我们的程序中，参数之间没有明显的关联。
// It would be more readable and more manageable to group width and height together.
// 将宽度和高度组合在一起会更具有可读性和可管理性
// We’ve already discussed one way we might do that in “The Tuple Type” section of Chapter 3: by using tuples.
// 我们已经在第3章的“元组类型”一节讨论过一种方法:使用元组。

// Refactoring with Tuples
// 使用元组重构
// Listing 5-9 shows another version of our program that uses tuples.
// 清单5-9展示了使用元组的程序的另一个版本。

fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Listing 5-9: Specifying the width and height of the rectangle with a tuple
// 清单5-9:用元组指定矩形的宽度和高度

// In one way, this program is better. Tuples let us add a bit of structure, and we’re now passing just one argument.
// 从某种意义上说，这个程序更好。元组让我们添加一些结构，我们现在只传递一个参数。
// But in another way, this version is less clear: tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious.
// 但从另一方面来说，这个版本不太清楚:元组不命名它们的元素，因此我们必须索引元组的每个部分，使我们的计算不太明显。

// Mixing up the width and height wouldn’t matter for the area calculation, but if we want to draw the rectangle on the screen, it would matter!
// 混合使用宽度和高度对面积计算没有影响，但如果我们想在屏幕上绘制矩形，这就有关系了!
// We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1.
// 我们必须记住，width是元组索引0,height是元组索引1。
// This would be even harder for someone else to figure out and keep in mind if they were to use our code.
// 如果其他人使用我们的代码，这将更加难以理解和记住。
// Because we haven’t conveyed the meaning of our data in our code, it’s now easier to introduce errors.
// 因为我们没有在代码中传达数据的含义，所以现在更容易引入错误。

// Refactoring with Structs: Adding More Meaning
// 用结构体重构:添加更多含义
// We use structs to add meaning by labeling the data.
// 我们使用结构体通过标记数据来添加含义。
// We can transform the tuple we’re using into a struct with a name for the whole as well as names for the parts, as shown in Listing 5-10.
// 可以将元组转换为一个结构体，既包含部分名称，也包含整体名称，如代码清单5-10所示。
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// Listing 5-10: Defining a Rectangle struct
// 清单5-10:定义一个Rectangle结构体

// Here we’ve defined a struct and named it Rectangle.
// 这里我们定义了一个结构体，并将其命名为Rectangle。
// Inside the curly brackets, we defined the fields as width and height, both of which have type u32.
// 在大括号内，我们将字段定义为width和height，它们的类型都是u32。
// Then in main, we created a particular instance of Rectangle that has a width of 30 and a height of 50.
// 然后在main中，我们创建了一个width为30,height为50的Rectangle实例。

// Our area function is now defined with one parameter, which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance.
// 面积函数现在定义了一个参数rectangle，它的类型是一个不可变的借用struct rectangle实例。
// As mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it.
// 如第4章所述，我们想借用这个结构体，而不是拥有它。
// This way, main retains its ownership and can continue using rect1, which is the reason we use the & in the function signature and where we call the function.
// 这样，main保留了它的所有权，可以继续使用rect1，这就是我们在函数签名中使用&的原因，也是我们调用该函数的地方。

// The area function accesses the width and height fields of the Rectangle instance (note that accessing fields of a borrowed struct instance does not move the field values,
// area函数访问Rectangle实例的width和height字段(注意，访问借用的struct实例的字段并不会移动字段的值，
// which is why you often see borrows of structs).
// 这就是为什么你经常看到借用结构体)。
// Our function signature for area now says exactly what we mean: calculate the area of Rectangle, using its width and height fields.
// 我们的area函数签名现在正好表达了我们的意思:使用它的width和height字段计算矩形的面积。
// This conveys that the width and height are related to each other, and it gives descriptive names to the values rather than using the tuple index values of 0 and 1.
// 这表明宽度和高度是相互关联的，并且它为值提供了描述性的名称，而不是使用元组索引值0和1。
// This is a win for clarity.
// 这是清晰的胜利。

// Adding Useful Functionality with Derived Traits
// 使用派生trait添加有用的功能
// It’d be useful to be able to print an instance of Rectangle while we’re debugging our program and see the values for all its fields.
// 在调试程序时，能够打印一个Rectangle实例，并查看其所有字段的值，这很有用。
// Listing 5-11 tries using the println! macro as we have used in previous chapters. This won’t work, however.
// 代码清单5-11尝试使用println!宏，正如我们在前几章中使用的那样。然而，这行不通。

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}

// Listing 5-11: Attempting to print a Rectangle instance
// 清单5-11:尝试打印一个Rectangle实例

// When we compile this code, we get an error with this core message:
// 当我们编译这段代码时，我们会得到一个包含以下核心信息的错误:

// error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`

// The println! macro can do many kinds of formatting, and by default, the curly brackets tell println!
// println!宏可以执行多种格式，默认情况下，大括号告诉println!
// to use formatting known as Display: output intended for direct end user consumption.
// 使用名为Display:输出的格式，供最终用户直接使用。
// The primitive types we’ve seen so far implement Display by default, because there’s only one way you’d want to show a 1 or any other primitive type to a user.
// 到目前为止，我们看到的基本类型都默认实现了Display，因为只有一种方法可以向用户显示1或其他基本类型。
// But with structs, the way println! should format the output is less clear because there are more display possibilities: Do you want commas or not?
// 但是对于结构体，println!是否因为有更多的显示方式，所以输出的格式不太清楚:你想要逗号还是不要逗号?
// Do you want to print the curly brackets? Should all the fields be shown?
// 打印大括号吗?所有字段都应该显示吗?
// Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of Display to use with println! and the {} placeholder.
// 由于这种模糊性，Rust不会尝试猜测我们想要什么，结构体也没有提供用于println的Display实现!以及{}占位符。

// If we continue reading the errors, we’ll find this helpful note:
// 如果我们继续阅读错误，会发现以下有用的提示:

// = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
// = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
// =注意:在格式字符串中，你可以使用' {:?}`(或者{:#?}以方便打印)

// Let’s try it! The println! macro call will now look like println!("rect1 is {:?}", rect1);.
// Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug.
// 放置说明符:?在大括号内告诉println!我们想使用一种名为Debug的输出格式。
// The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.
// Debug特性使我们能够以一种对开发人员有用的方式打印我们的结构体，以便我们在调试代码时可以看到它的值。

// Compile the code with this change. Drat! We still get an error:
// 修改后编译代码。见鬼!仍然会得到一个错误:
// error[E0277]: `Rectangle` doesn't implement `Debug`

// But again, the compiler gives us a helpful note:
// 但是，编译器还是给了我们有用的提示:
// = help: the trait `Debug` is not implemented for `Rectangle`
// = help: trait `Debug`没有为`Rectangle`实现
// = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
// =注意:添加' #[derive(Debug)] '到' Rectangle '或者手动添加' impl Debug for Rectangle '

// Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct.
// Rust确实包含打印调试信息的功能，但我们必须明确选择将该功能用于我们的结构体。
// To do that, we add the outer attribute #[derive(Debug)] just before the struct definition, as shown in Listing 5-12.
// 为此，我们在结构体定义之前添加了外部属性#[derive(Debug)]，如清单5-12所示。

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}

// Listing 5-12: Adding the attribute to derive the Debug trait and printing the Rectangle instance using debug formatting
// 代码清单5-12:添加属性来获取调试特征，并使用调试格式打印矩形实例

// Now when we run the program, we won’t get any errors, and we’ll see the following output:
// 现在运行程序时，不会有任何错误，输出如下:

// $ cargo run
//    Compiling rectangles v0.1.0 (file:///projects/rectangles)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.48s
//      Running `target/debug/rectangles`
// rect1 is Rectangle { width: 30, height: 50 }

// Nice! It’s not the prettiest output, but it shows the values of all the fields for this instance, which would definitely help during debugging.
// 好!这不是最漂亮的输出，但它显示了此实例的所有字段的值，这肯定有助于调试。
// When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use {:#?} instead of {:?} in the println! string.
// 当我们有更大的结构体时，让输出更容易阅读是很有用的;在这种情况下，我们可以使用{:#?}而不是{:?}在println!字符串。
// In this example, using the {:#?} style will output:
// 在这个例子中，使用{:#?} style将输出:

// $ cargo run
//    Compiling rectangles v0.1.0 (file:///projects/rectangles)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.48s
//      Running `target/debug/rectangles`
// rect1 is Rectangle {
//     width: 30,
//     height: 50,
// }

// Another way to print out a value using the Debug format is to use the dbg! macro, which takes ownership of an expression (as opposed to println! that takes a reference),
// 另一种使用Debug格式打印值的方法是使用dbg!宏，它接受表达式的所有权(与之相反的是println!这需要参考)，
// prints the file and line number of where that dbg! macro call occurs in your code along with the resulting value of that expression, and returns ownership of the value.
// 打印dbg所在的文件和行号!宏调用与表达式的结果值一起发生在代码中，并返回值的所有权。

// Note: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println! which prints to the standard output console stream (stdout).
// 注意:调用dbg!宏打印到标准错误控制流(stderr)，而不是println!它打印到标准输出控制台流(stdout)。
// We’ll talk more about stderr and stdout in the ““Writing Error Messages to Standard Error Instead of Standard Output” section in Chapter 12.
// 我们将在第12章的“将错误消息写入标准错误而不是标准输出”一节中详细讨论stderr和stdout。

// Here’s an example where we’re interested in the value that gets assigned to the width field, as well as the value of the whole struct in rect1:
// 在这个例子中，我们对赋值给width字段的值以及rect1中整个结构体的值感兴趣:

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

// We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value,
// the width field will get the same value as if we didn’t have the dbg! call there.
// We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call.
// 我们不需要dbg!取得rect1的所有权，所以我们在接下来的调用中使用了对rect1的引用。
// Here’s what the output of this example looks like:
// 这个例子的输出如下:

// $ cargo run
//    Compiling rectangles v0.1.0 (file:///projects/rectangles)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.61s
//      Running `target/debug/rectangles`
// [src/main.rs:10] 30 * scale = 60
// [src/main.rs:14] &rect1 = Rectangle {
//     width: 60,
//     height: 50,
// }

// We can see the first bit of output came from src/main.rs line 10, where we’re debugging the expression 30 * scale,
// 我们可以看到第一个输出来自src/main。Rs第10行，我们正在调试表达式30 * scale，
// and its resulting value is 60 (the Debug formatting implemented for integers is to print only their value).
// 它的结果是60(对整数实现的调试格式化是只打印它们的值)。
// The dbg! call on line 14 of src/main.rs outputs the value of &rect1, which is the Rectangle struct.
// dbg!调用src/main的第14行。rs输出&rect1的值，它是Rectangle结构体。
// This output uses the pretty Debug formatting of the Rectangle type.
// 这个输出使用了漂亮的Rectangle类型调试格式。
// The dbg! macro can be really helpful when you’re trying to figure out what your code is doing!
// dbg!当你想弄清楚你的代码在做什么时，宏非常有用!

// In addition to the Debug trait, Rust has provided a number of traits for us to use with the derive attribute that can add useful behavior to our custom types.
// 除了Debug特征，Rust还提供了许多特征供我们与derive属性一起使用，这些特征可以为我们的自定义类型添加有用的行为。
// Those traits and their behaviors are listed in Appendix C.
// 这些特征及其行为列在附录C中。
// We’ll cover how to implement these traits with custom behavior as well as how to create your own traits in Chapter 10.
// 我们将在第10章介绍如何实现这些具有自定义行为的trait，以及如何创建自己的trait。
// There are also many attributes other than derive; for more information, see the “Attributes” section of the Rust Reference.
// 除了derive之外，还有很多属性;有关更多信息，请参阅Rust参考资料的“Attributes”部分。

// Our area function is very specific: it only computes the area of rectangles.
// 面积函数非常特殊:它只计算矩形的面积。
// It would be helpful to tie this behavior more closely to our Rectangle struct, because it won’t work with any other type.
// 将这种行为更紧密地绑定到结构体Rectangle会有所帮助，因为它不能用于任何其他类型。
// Let’s look at how we can continue to refactor this code by turning the area function into an area method defined on our Rectangle type.
// 让我们看看如何继续重构这段代码，将面积函数转换为Rectangle类型定义的面积方法。

// Method Syntax
// 方法语法
// Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value,
// 方法类似于函数:我们使用fn关键字和名称声明它们，它们可以有参数和返回值，
// and they contain some code that’s run when the method is called from somewhere else.
// 当从其他地方调用该方法时，它们包含一些代码。
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object, which we cover in Chapters 6 and 17, respectively),
// 与函数不同，方法是在结构体(或枚举或trait对象，分别在第6章和第17章中介绍)的上下文中定义的，
// and their first parameter is always self, which represents the instance of the struct the method is being called on.
// 第一个参数始终是self，表示调用方法的结构体的实例。

// Defining Methods
// 定义方法
// Let’s change the area function that has a Rectangle instance as a parameter and instead make an area method defined on the Rectangle struct, as shown in Listing 5-13.
// 修改以Rectangle实例作为参数的area函数，改为在Rectangle结构体中定义一个area方法，如代码清单5-13所示。
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
// Listing 5-13: Defining an area method on the Rectangle struct
// 代码清单5-13:在Rectangle结构体上定义area方法

// To define the function within the context of Rectangle, we start an impl (implementation) block for Rectangle.
// 为了在Rectangle的上下文中定义函数，我们为Rectangle启动了一个impl (implementation)块。
// Everything within this impl block will be associated with the Rectangle type.
// impl块中的所有内容都与Rectangle类型关联
// Then we move the area function within the impl curly brackets and change the first (and in this case, only) parameter to be self in the signature and everywhere within the body.
// 然后，我们将面积函数移动到impl的大括号中，并将签名中的第一个(在本例中是唯一的)参数更改为self，以及函数体中的所有地方。
// In main, where we called the area function and passed rect1 as an argument, we can instead use method syntax to call the area method on our Rectangle instance.
// 在main中，我们调用面积函数并将rect1作为实参，我们可以使用方法语法来调用Rectangle实例的area方法。
// The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.
// 方法语法跟在实例后面:我们在方法名、圆括号和参数后面加上一个句点。

// In the signature for area, we use &self instead of rectangle: &Rectangle.
// 在area的签名中，我们使用&self而不是rectangle: &Rectangle。
// The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for.
// &self实际上是self: &Self的缩写。在impl块中，类型Self是impl块所对应类型的别名。
// Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot.
// 方法的第一个参数必须有一个类型为Self的参数self，因此Rust允许您在第一个参数位置只使用名称self来缩写它。
// Note that we still need to use the & in front of the self shorthand to indicate this method borrows the Self instance, just as we did in rectangle: &Rectangle.
// 注意，我们仍然需要在self前面使用&，以表明这个方法借用了self实例，就像我们在rectangle: &Rectangle中所做的那样。
// Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.
// 方法可以获取self的所有权，可以像这里一样不可变地借用self，也可以像其他参数一样可变地借用self。

// We’ve chosen &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership,
// 这里我们选择&self的原因和在函数版本中使用&Rectangle的原因一样:我们不想拥有所有权，
// and we just want to read the data in the struct, not write to it.
// 我们只想读取结构体中的数据，而不是写入它。
// If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter.
// 如果我们想改变调用方法的实例，作为方法功能的一部分，可以使用&mut self作为第一个参数。
// Having a method that takes ownership of the instance by using just self as the first parameter is rare;
// 使用self作为第一个参数来获取实例所有权的方法很少见;
// this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
// 当方法将self转换为其他对象，并且你希望避免调用者在转换后使用原始实例时，通常使用这种技术。

// The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of self in every method’s signature, is for organization.
// 使用方法而不是函数的主要原因是为了组织，除了提供方法语法，而且不需要在每个方法签名中重复self的类型。
// We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.
// 我们把一个类型实例能做的所有事情都放在了一个impl块中，而不是让将来使用我们代码的用户在我们提供的库的不同地方搜索Rectangle的功能。

// Note that we can choose to give a method the same name as one of the struct’s fields. For example, we can define a method on Rectangle also named width:
// 注意，我们可以选择给方法提供与结构体的某个字段同名的方法。例如，我们可以为Rectangle定义一个名为width的方法:

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
// Here, we’re choosing to make the width method return true if the value in the instance’s width field is greater than 0,
// 这里，我们选择让width方法在实例的width字段的值大于0时返回true，
// and false if the value is 0: we can use a field within a method of the same name for any purpose.
// 如果值为0，则为false: 我们可以出于任何目的，在同名的方法中使用同名的字段。
// In main, when we follow rect1.width with parentheses, Rust knows we mean the method width. When we don’t use parentheses, Rust knows we mean the field width.
// 在main中，当我们执行rect1。带括号的宽度，Rust知道我们指的是方法宽度。当我们不使用括号时，Rust知道我们指的是字段宽度。

// Often, but not always, when we give methods with the same name as a field we want it to only return the value in the field and do nothing else.
// 通常情况下(但并非总是)，当我们提供与字段同名的方法时，我们希望它只返回字段的值，而不做其他任何事情。
// Methods like this are called getters, and Rust does not implement them automatically for struct fields as some other languages do.
// 这样的方法称为getter, Rust不会像其他语言那样为结构字段自动实现它们。
// Getters are useful because you can make the field private but the method public and thus enable read-only access to that field as part of the type’s public API.
// getter很有用，因为你可以把字段声明为private，但把方法声明为public，这样就可以在类型的公共API中只读地访问字段。
// We will be discussing what public and private are and how to designate a field or method as public or private in Chapter 7.
// 我们将在第7章讨论什么是public和private，以及如何将字段或方法指定为public或private。

// Where’s the -> Operator?
// -> 运算符在哪里?
// In C and C++, two different operators are used for calling methods: you use . if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first.
// 在C和c++中，有两种不同的操作符用于调用方法:如果你直接在对象上调用方法->如果你在对象的指针上调用方法，并且需要先解引指针。
// In other words, if object is a pointer, object->something() is similar to (*object).something().
// 换句话说，如果object是一个指针，那么object->something()类似于(*object).something()。

// Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing.
// Rust没有与->运算符等价的运算符;相反，Rust有一个称为自动引用和解引用的功能。
// Calling methods is one of the few places in Rust that has this behavior.
// 调用方法是Rust中少数几个具有这种行为的地方之一

// Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method.
// 它是这样工作的:当你使用object.something()调用一个方法时，Rust会自动添加&、&mut或*，因此object与method的签名匹配。
// In other words, the following are the same:
// 换句话说，下面的代码是相同的:

// p1.distance(&p2);
// (&p1).distance(&p2);

// The first one looks much cleaner.
// 第一个看起来干净多了
// This automatic referencing behavior works because methods have a clear receiver—the type of self.
// 这种自动引用行为之所以有效，是因为方法有一个明确的接收器——self类型。
// Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self).
// 给定接收者和方法名，Rust可以确定该方法是在读取(&self)、修改(&mut self)还是在消耗（获得所有权）(self)。
// The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
// Rust对方法接收器的借用是在实践中使所有权符合人体工程学的很大一部分。

// Methods with More Parameters
// 带有更多参数的方法
// Let’s practice using methods by implementing a second method on the Rectangle struct.
// 让我们通过在Rectangle结构体上实现第二个方法来练习使用方法。
// This time, we want an instance of Rectangle to take another instance of Rectangle and return true if the second Rectangle can fit completely within self (the first Rectangle); otherwise it should return false.
// 这一次，我们希望Rectangle的实例接受Rectangle的另一个实例，并在第二个Rectangle的实例能够完全放入self(第一个Rectangle)时返回true;否则应该返回false。
// That is, once we’ve defined the can_hold method, we want to be able to write the program shown in Listing 5-14.
// 也就是说，定义can_hold方法后，我们希望能够编写如清单5-14所示的程序。

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
// Listing 5-14: Using the as-yet-unwritten can_hold method
// 清单5-14:使用尚未编写的can_hold方法

// And the expected output would look like the following, because both dimensions of rect2 are smaller than the dimensions of rect1 but rect3 is wider than rect1:
// 预期的输出如下所示，因为rect2的两个维度都小于rect1，而rect3的宽度大于rect1:

// Can rect1 hold rect2? true
// Can rect1 hold rect3? false

// We know we want to define a method, so it will be within the impl Rectangle block.
// 我们知道要定义一个方法，所以它将在impl Rectangle块内。
// The method name will be can_hold, and it will take an immutable borrow of another Rectangle as a parameter.
// 方法名是can_hold，它会借用另一个不可变的矩形作为参数。
// We can tell what the type of the parameter will be by looking at the code that calls the method: rect1.can_hold(&rect2) passes in &rect2,
// 我们可以通过调用rect1.can_hold(&rect2)的代码来判断参数的类型:
// which is an immutable borrow to rect2, an instance of Rectangle.
// 这是一个不可变的借用对象，它是Rectangle的一个实例rect2。
// This makes sense because we only need to read rect2 (rather than write, which would mean we’d need a mutable borrow),
// 这是有道理的，因为我们只需要读取rect2(而不是写入，这意味着我们需要一个可变的借用)，
// and we want main to retain ownership of rect2 so we can use it again after calling the can_hold method.
// 我们希望main保留rect2的所有权，以便在调用can_hold方法后再次使用它。
// The return value of can_hold will be a Boolean, and the implementation will check whether the width and height of self are both greater than the width and height of the other Rectangle, respectively.
// can_hold的返回值是布尔值，实现会检查self的宽度和高度是否分别大于另一个矩形的宽度和高度。
// Let’s add the new can_hold method to the impl block from Listing 5-13, shown in Listing 5-15.
// 让我们将新的can_hold方法添加到清单5-13中的impl块中，如清单5-15所示。

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// Listing 5-15: Implementing the can_hold method on Rectangle that takes another Rectangle instance as a parameter
// 清单5-15:在Rectangle上实现can_hold方法，它接受另一个Rectangle实例作为参数

// When we run this code with the main function in Listing 5-14, we’ll get our desired output.
// 当我们使用清单5-14中的main函数运行这段代码时，我们将得到我们想要的输出。
// Methods can take multiple parameters that we add to the signature after the self parameter, and those parameters work just like parameters in functions.
// 方法可以接受多个参数，我们将这些参数添加到self参数的签名后，这些参数的工作方式就像函数中的参数一样。

// Associated Functions
// 关联函数
// All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.
// 在impl块中定义的所有函数都称为关联函数(associated function)，因为它们与以impl命名的类型相关联。
// We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.
// 我们可以定义没有self作为第一个参数的关联函数(因此不是方法)，因为它们不需要使用类型的实例。
// We’ve already used one function like this: the String::from function that’s defined on the String type.
// 我们已经使用过一个这样的函数: String::from，它是在String类型上定义的。

// Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
// 非方法的关联函数通常用于返回结构体新实例的构造函数。
// These are often called new, but new isn’t a special name and isn’t built into the language.
// 它们通常被称为new，但new不是一个特殊的名称，也没有内置在语言中。
// For example, we could choose to provide an associated function named square that would have one dimension parameter and use that as both width and height,
// 例如，我们可以选择提供一个名为square的关联函数，它具有一个维度参数，并使用它作为宽度和高度，
// thus making it easier to create a square Rectangle rather than having to specify the same value twice:
// 因此，创建一个方形矩形更容易，而不必两次指定相同的值:

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
// The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
// return类型中的Self关键字和函数体中的Self关键字是impl关键字后面的类型的别名，在这里是Rectangle类型。

// To call this associated function, we use the :: syntax with the struct name; let sq = Rectangle::square(3); is an example.
// 要调用关联的函数，我们使用结构体名称的::语法;let sq = Rectangle::square(3);就是一个例子。
// This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules.
// 这个函数的命名空间由struct定义:关联的函数和模块创建的命名空间都使用::语法。
// We’ll discuss modules in Chapter 7.
// 我们将在第7章讨论模块

// Multiple impl Blocks
// 多个impl块
// Each struct is allowed to have multiple impl blocks.
// 每个结构体允许有多个impl块
// For example, Listing 5-15 is equivalent to the code shown in Listing 5-16, which has each method in its own impl block.
// 例如，清单5-15等同于清单5-16中的代码，其中每个方法都在自己的impl块中。

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// Listing 5-16: Rewriting Listing 5-15 using multiple impl blocks
// 清单5-16:使用多个impl块重写清单5-15

// There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax.
// 这里没有理由将这些方法分成多个impl块，但这是有效的语法。
// We’ll see a case in which multiple impl blocks are useful in Chapter 10, where we discuss generic types and traits.
// 第10章讨论泛型类型和特征时，我们会看到使用多个impl块的情况。

// Summary
// 总结
// Structs let you create custom types that are meaningful for your domain.
// 结构体让你创建对你的域有意义的自定义类型
// By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear.
// 通过使用结构体，你可以将相关的数据片段彼此关联，并为每个片段命名，以使代码清晰。
// In impl blocks, you can define functions that are associated with your type,
// 在impl块中，你可以定义与你的类型相关的函数，
// and methods are a kind of associated function that let you specify the behavior that instances of your structs have.
// 方法是一种关联函数，让你指定结构体实例的行为。

// But structs aren’t the only way you can create custom types: let’s turn to Rust’s enum feature to add another tool to your toolbox.
// 但结构体并不是创建自定义类型的唯一方法:让我们转向Rust的enum功能，为你的工具箱添加另一个工具。
