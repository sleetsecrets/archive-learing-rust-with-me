
fn main() {

    // Enums and Pattern Matching
    // 枚举和模式匹配
    // In this chapter we’ll look at enumerations, also referred to as enums.
    // 在本章中，我们将学习枚举（enumerations），也称为enum。
    // Enums allow you to define a type by enumerating its possible variants.
    // 枚举允许你通过枚举可能的变量来定义一个类型。
    // First, we’ll define and use an enum to show how an enum can encode meaning along with data.
    // 首先，我们将定义并使用枚举来展示枚举如何与数据一起编码意义。
    // Next, we’ll explore a particularly useful enum, called Option, which expresses that a value can be either something or nothing.
    // 接下来，我们将探索一个特别有用的枚举，称为Option，它表示一个值可以是什么，或什么都不是。
    // Then we’ll look at how pattern matching in the match expression makes it easy to run different code for different values of an enum.
    // 然后，我们将了解match表达式中的模式匹配如何使为枚举的不同值运行不同的代码变得容易。
    // Finally, we’ll cover how the if let construct is another convenient and concise idiom available to handle enums in your code.
    // 最后，我们将介绍if let构造是如何在代码中处理枚举的另一个方便和简洁的习惯用法。

    // Defining an Enum
    // 定义一个枚举
    // Where structs give you a way of grouping together related fields and data,
    // 结构体为你提供了一种将相关字段和数据分组的方法，
    // like a Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values.
    // 与具有宽度和高度的矩形一样，枚举为我们提供了一种方式，让我们知道一个值是一组可能的值中的一个。
    // For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle.
    // 例如，我们可能想说Rectangle是一系列可能形状中的一个，这些形状还包括Circle和Triangle
    // To do this, Rust allows us to encode these possibilities as an enum.
    // 为了做到这一点，Rust允许我们将这些可能性编码为枚举。

    // Let’s look at a situation we might want to express in code and see why enums are useful and more appropriate than structs in this case.
    // 让我们来看一种可能需要用代码表达的情况，看看为什么在这种情况下枚举比结构体更有用、更合适。
    // Say we need to work with IP addresses.
    // 假设我们需要处理IP地址
    // Currently, two major standards are used for IP addresses: version four and version six.
    // 目前，IP地址使用两个主要标准:版本4和版本6。
    // Because these are the only possibilities for an IP address that our program will come across, we can enumerate all possible variants, which is where enumeration gets its name.
    // 因为我们的程序只会遇到这些可能的IP地址，所以我们可以枚举所有可能的变体，这也是枚举名称的由来。

    // Any IP address can be either a version four or a version six address, but not both at the same time.
    // 任何IP地址都可以是版本4或版本6，但不能同时是
    // That property of IP addresses makes the enum data structure appropriate, because an enum value can only be one of its variants.
    // 这个IP地址属性使枚举数据结构更合适，因为枚举的值只能是它的变体之一。
    // Both version four and version six addresses are still fundamentally IP addresses,
    // 版本4和版本6从根本上讲都是IP地址，
    // so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.
    // 所以当代码处理适用于任何类型IP地址的情况时，它们应该被视为相同的类型。

    // We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be, V4 and V6.
    // 我们可以在代码中通过定义IpAddrKind枚举并列出IP地址可能的类型来表达这个概念，V4和V6。
    // These are the variants of the enum:
    // 下面是这个枚举的变体:

    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // IpAddrKind is now a custom data type that we can use elsewhere in our code.
    // IpAddrKind现在是一个自定义数据类型，我们可以在代码的其他地方使用它。

    // Enum Values
    // 枚举值
    // We can create instances of each of the two variants of IpAddrKind like this:
    // 我们可以像下面这样创建IpAddrKind的两种变体的实例:

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two.
    // 注意，枚举的变体在其标识符下包含了命名空间，我们使用了两个冒号来分隔它们。
    // This is useful because now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind.
    // 这很有用，因为现在两个值IpAddrKind::V4和IpAddrKind::V6都是相同的类型:IpAddrKind。
    // We can then, for instance, define a function that takes any IpAddrKind:
    // 然后，我们可以定义一个接受任意IpAddrKind参数的函数:

    // fn route(ip_kind: IpAddrKind) {}

    // And we can call this function with either variant:
    // 我们可以用任意一种变体调用这个函数:

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    // Using enums has even more advantages.
    // 使用枚举还有更多优点
    // Thinking more about our IP address type, at the moment we don’t have a way to store the actual IP address data; we only know what kind it is.
    // 更多地考虑我们的IP地址类型，目前我们没有办法存储实际的IP地址数据;我们只知道是哪种。
    // Given that you just learned about structs in Chapter 5, you might be tempted to tackle this problem with structs as shown in Listing 6-1.
    // 鉴于你刚刚在第5章学习过结构体，你可能想用清单6-1所示的结构体来解决这个问题。

    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // Listing 6-1: Storing the data and IpAddrKind variant of an IP address using a struct
    // 清单6-1:使用结构体存储数据和IP地址的IpAddrKind变体

    // Here, we’ve defined a struct IpAddr that has two fields: a kind field that is of type IpAddrKind (the enum we defined previously) and an address field of type String.
    // 在这里，我们定义了一个struct IpAddr，它有两个字段:一个类型为IpAddrKind的字段(我们之前定义的枚举)和一个类型为String的address字段。
    // We have two instances of this struct.
    // 我们有这个结构体的两个实例
    // The first is home, and it has the value IpAddrKind::V4 as its kind with associated address data of 127.0.0.1. The second instance is loopback.
    // 第一个是home，它的类型值为IpAddrKind::V4，关联的地址数据为127.0.0.1。第二个实例是loopback。
    // It has the other variant of IpAddrKind as its kind value, V6, and has address ::1 associated with it.
    // 它的类型值是IpAddrKind的另一个变体V6，并与address ::1关联。
    // We’ve used a struct to bundle the kind and address values together, so now the variant is associated with the value.
    // 我们使用结构体将kind和address值捆绑在一起，所以现在变体与值关联。

    // However, representing the same concept using just an enum is more concise: rather than an enum inside a struct, we can put data directly into each enum variant.
    // 然而，使用枚举表示相同的概念更简洁:我们可以直接将数据放入每个枚举变体，而不是在结构体中使用枚举。
    // This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:
    // 这个IpAddr枚举的新定义表示V4和V6变体都有关联的字符串值:

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    // We attach data to each variant of the enum directly, so there is no need for an extra struct.
    // 我们直接将数据添加到枚举的每个变体，因此不需要额外的结构体。
    // Here it’s also easier to see another detail of how enums work: the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
    // 这里也更容易看到枚举如何工作的另一个细节:我们定义的每个枚举变体的名称也成为一个构造枚举实例的函数。
    // That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.
    // 也就是说，IpAddr::V4()是一个函数调用，它接受一个字符串参数，并返回IpAddr类型的实例。
    // We automatically get this constructor function defined as a result of defining the enum.
    // 我们自动得到这个构造函数的定义作为枚举的定义。

    // There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data.
    // 使用枚举比使用结构体还有一个好处:每个变体可以有不同的类型和数量的关联数据。
    // Version four type IP addresses will always have four numeric components that will have values between 0 and 255.
    // 四种类型的IP地址总是有四个数字分量，它们的值在0到255之间。
    // If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct.
    // 如果我们想将V4地址存储为四个u8值，但仍然将V6地址表示为一个字符串值，我们无法使用结构体。
    // Enums handle this case with ease:
    // 枚举可以轻松地处理这种情况:

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    // We’ve shown several different ways to define data structures to store version four and version six IP addresses.
    // 我们展示了几种不同的方式来定义数据结构来存储版本四和版本六的IP地址。
    // However, as it turns out, wanting to store IP addresses and encode which kind they are is so common that the standard library has a definition we can use!
    // 然而，事实证明，想要存储IP地址并编码它们的类型是如此常见，以至于我们可以使用标准库的定义!
    // Let’s look at how the standard library defines IpAddr: it has the exact enum and variants that we’ve defined and used,
    // 看看标准库是如何定义IpAddr的:它有我们定义和使用过的确切的枚举和变体，
    // but it embeds the address data inside the variants in the form of two different structs, which are defined differently for each variant:
    // 但它将地址数据以两个不同的结构体的形式嵌入到变体中，每个变体的定义都不同:

    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example.
    // 这段代码说明，可以在枚举变量中放入任何类型的数据:例如字符串、数字类型或结构体。
    // You can even include another enum! Also, standard library types are often not much more complicated than what you might come up with.
    // 你甚至可以包含另一个枚举!而且，标准库类型通常不会比你可能想到的更复杂。

    // Note that even though the standard library contains a definition for IpAddr,
    // 注意，尽管标准库中包含了IpAddr的定义，
    // we can still create and use our own definition without conflict because we haven’t brought the standard library’s definition into our scope.
    // 我们仍然可以创建和使用自己的定义，而不会产生冲突，因为我们没有将标准库的定义引入到我们的作用域中。
    // We’ll talk more about bringing types into scope in Chapter 7.
    // 我们将在第7章进一步讨论将类型引入作用域。

    // Let’s look at another example of an enum in Listing 6-2: this one has a wide variety of types embedded in its variants.
    // 让我们看一下清单6-2中枚举的另一个例子:这个例子在其变体中嵌入了各种类型。

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // Listing 6-2: A Message enum whose variants each store different amounts and types of values
    // 清单6-2:一个消息枚举，每个变量存储不同数量和类型的值

    // This enum has four variants with different types:
    // 这个枚举有四个不同类型的变量:

    // Quit has no data associated with it at all.
    // Quit没有相关的数据。
    // Move has named fields like a struct does.
    // Move像结构体一样。
    // Write includes a single String.
    // Write包含一个字符串。
    // ChangeColor includes three i32 values.
    // ChangeColor包含三个i32值。

    // Defining an enum with variants such as the ones in Listing 6-2 is similar to defining different kinds of struct definitions,
    // 用代码清单6-2中的变量定义枚举与定义不同类型的结构体类似，
    // except the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type.
    // 只是这个枚举不使用struct关键字，而且所有变体都放在Message类型下。
    // The following structs could hold the same data that the preceding enum variants hold:
    // 下面的结构体可以保存与前面的枚举变量相同的数据:

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // But if we used the different structs, which each have their own type,
    // 但如果我们使用不同的结构体，它们有自己的类型，
    // we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum defined in Listing 6-2, which is a single type.
    // 我们不能像代码清单6-2中定义的Message 枚举那样简单地定义一个函数来接收任何这种类型的消息。

    // There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, we’re also able to define methods on enums.
    // 枚举和结构体之间还有一个相似之处:正如我们可以使用impl在结构体上定义方法一样，我们也可以在枚举上定义方法。
    // Here’s a method named call that we could define on our Message enum:
    // 这是一个名为call的方法，我们可以在Message enum上定义它:

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // The body of the method would use self to get the value that we called the method on.
    // 方法的主体将使用self来获取我们调用方法时的值。
    // In this example, we’ve created a variable m that has the value Message::Write(String::from("hello")),
    // 在这个例子中，我们创建了一个变量m，其值为Message::Write(String::from("hello"))，
    // and that is what self will be in the body of the call method when m.call() runs.
    // 当m.call()运行时，这就是self在call方法体中的样子。

    // Let’s look at another enum in the standard library that is very common and useful: Option.
    // 下面来看标准库中另一个非常常用的枚举:Option。

    // The Option Enum and Its Advantages Over Null Values
    // Option枚举及其相对于Null值的优点
    // This section explores a case study of Option, which is another enum defined by the standard library.
    // 本节将探讨一个Option的案例，它是标准库定义的另一个枚举。
    // The Option type encodes the very common scenario in which a value could be something or it could be nothing.
    // Option类型编码了非常常见的情况，其中值可以是某个值，也可以什么都没有（没有值）。

    // For example, if you request the first of a list containing items, you would get a value.
    // 例如，如果你请求包含元素的列表中的第一个元素，你会得到一个值。
    // If you request the first item of an empty list, you would get nothing.
    // 如果你请求一个空列表的第一项，什么也不会得到
    // Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the cases you should be handling;
    // 用类型系统来表达这个概念意味着编译器可以检查你是否已经处理了所有你应该处理的情况;
    // this functionality can prevent bugs that are extremely common in other programming languages.
    // 这个功能可以防止在其他编程语言中非常常见的bug。

    // Programming language design is often thought of in terms of which features you include, but the features you exclude are important too.
    // 编程语言的设计通常取决于你包含哪些功能，但你排除的功能也很重要。
    // Rust doesn’t have the null feature that many other languages have.
    // Rust没有很多其他语言拥有的null特性。
    // Null is a value that means there is no value there.
    // Null是一个值，意味着里面没有值
    // In languages with null, variables can always be in one of two states: null or not-null.
    // 在有null的语言中，变量总是有两种状态:null或not-null。

    // In his 2009 presentation “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, has this to say:
    // 在2009年的演讲“Null References: The Billion Dollar Mistake”中，Null的发明者Tony Hoare这样说道:
    // I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language.
    // 我称它为我的数十亿美元的错误。那时，我正在设计第一个面向对象语言中针对引用的全面类型系统。
    // My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler.
    // 我的目标是确保所有引用的使用都是绝对安全的，由编译器自动执行检查。
    // But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement.
    // 但我还是忍不住想在代码中加入一个null引用，因为这样实现起来太容易了。
    // This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.
    // 这导致了无数的错误、漏洞和系统崩溃，在过去的40年里，这些可能已经造成了10亿美元的痛苦和损失。

    // The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind.
    // null值的问题是，如果你试图将null值用作非null值，你会得到某种错误。
    // Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.
    // 因为null或not-null属性很常见，所以很容易出现这种错误。

    // However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.
    // 然而，null试图表达的概念仍然很有用:null是一个当前无效或出于某种原因不存在的值。

    // The problem isn’t really with the concept but with the particular implementation.
    // 问题不在于概念，而在于特定的实现。
    // As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.
    // 因此，Rust没有null，但它有一个枚举，可以编码值是否存在的概念。
    // This enum is Option<T>, and it is defined by the standard library as follows:
    // 这个枚举是Option<T>，由标准库定义如下:

    enum Option<T> {
        None,
        Some(T),
    }

    // The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly.
    // Option<T>枚举非常有用，它甚至被包含在序曲（prelude）中;你不需要显式地将它引入作用域。
    // Its variants are also included in the prelude: you can use Some and None directly without the Option:: prefix.
    // 它的变体也包含在prelude中:你可以直接使用Some和None，不需要选项::前缀。
    // The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.
    // Option<T> enum仍然只是一个普通的枚举，Some(T)和None仍然是Option<T>类型的变体。

    // The <T> syntax is a feature of Rust we haven’t talked about yet.
    // <T>语法是Rust的一个特性，我们还没有讨论过。
    // It’s a generic type parameter, and we’ll cover generics in more detail in Chapter 10.
    // 这是一个泛型类型参数，第10章会更详细地介绍泛型。
    // For now, all you need to know is that <T> means the Some variant of the Option enum can hold one piece of data of any type,
    // 现在，你只需要知道<T>表示Option枚举的某个变体可以保存任何类型的数据，
    // and that each concrete type that gets used in place of T makes the overall Option<T> type a different type.
    // 每个用于替代T的具体类型使整个Option<T>类型成为不同的类型。
    // Here are some examples of using Option values to hold number types and string types:
    // 下面是一些使用Option值保存数字类型和字符串类型的例子:

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // The type of some_number is Option<i32>.
    // some_number的类型是Option<i32>
    // The type of some_char is Option<char>, which is a different type.
    // some_char的类型是Option<char>，这是另一种类型。
    // Rust can infer these types because we’ve specified a value inside the Some variant.
    // Rust可以推断这些类型，因为我们在某个变体中指定了一个值。
    // For absent_number,
    // 对于absent_number，
    // Rust requires us to annotate the overall Option type: the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value.
    // Rust要求我们注释整个选项类型:编译器不能仅通过查看None值来推断相应的变体将持有的类型。
    // Here, we tell Rust that we mean for absent_number to be of type Option<i32>.
    // 这里，我们告诉Rust，我们的意思是absent_number的类型为Option<i32>。

    // When we have a Some value, we know that a value is present and the value is held within the Some.
    // 当我们有一个Some值时，我们知道这个值存在于Some中。
    // When we have a None value, in some sense, it means the same thing as null: we don’t have a valid value.
    // 在某种意义上，None与null的含义相同:没有合法的值。
    // So why is having Option<T> any better than having null?
    // 那么为什么Option<T>比null更好呢?

    // In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t let us use an Option<T> value as if it were definitely a valid value.
    // 简而言之，因为Option<T>和T (T可以是任何类型)是不同的类型，编译器不允许就好像它肯定是一个有效的值一样使用Option<T>值，。
    // For example, this code won’t compile because it’s trying to add an i8 to an Option<i8>:
    // 例如，这段代码不能编译，因为它试图将 Option<i8> 与 i8 相加：

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

    // If we run this code, we get an error message like this:
    // 如果我们运行这段代码，我们会得到一个类似这样的错误消息:

    // $ cargo run
    // Compiling enums v0.1.0 (file:///projects/enums)
    // error[E0277]: cannot add `Option<i8>` to `i8`
    // --> src/main.rs:5:17
    // |
    // 5 |     let sum = x + y;
    // |                 ^ no implementation for `i8 + Option<i8>`
    // |
    // = help: the trait `Add<Option<i8>>` is not implemented for `i8`
    // = help: the following other types implement trait `Add<Rhs>`:
    //             <&'a f32 as Add<f32>>
    //             <&'a f64 as Add<f64>>
    //             <&'a i128 as Add<i128>>
    //             <&'a i16 as Add<i16>>
    //             <&'a i32 as Add<i32>>
    //             <&'a i64 as Add<i64>>
    //             <&'a i8 as Add<i8>>
    //             <&'a isize as Add<isize>>
    //         and 48 others
    //
    // For more information about this error, try `rustc --explain E0277`.
    // error: could not compile `enums` due to previous error

    // Intense! In effect, this error message means that Rust doesn’t understand how to add an i8 and an Option<i8>, because they’re different types.
    // 强烈!实际上，这个错误消息意味着Rust不理解如何将i8与Option<i8>相加，因为它们是不同的类型。
    // When we have a value of a type like i8 in Rust, the compiler will ensure that we always have a valid value.
    // 当我们在Rust中有一个类型为i8的值时，编译器将确保我们总是有一个有效的值。
    // We can proceed confidently without having to check for null before using that value.
    // 我们可以放心地继续，而不必在使用该值之前检查是否为空。
    // Only when we have an Option<i8> (or whatever type of value we’re working with) do we have to worry about possibly not having a value,
    // 只有当我们有一个Option<i8>(或任何我们正在处理的值类型)时，我们才需要担心可能没有值，
    // and the compiler will make sure we handle that case before using the value.
    // 编译器将确保我们在使用该值之前处理该情况。

    // In other words, you have to convert an Option<T> to a T before you can perform T operations with it.
    // 换句话说，您必须将Option<T>转换为T，然后才能对其进行T操作。
    // Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
    // 通常，这有助于捕捉null最常见的问题之一:假设某个东西不是null，而实际上它是null。

    // Eliminating the risk of incorrectly assuming a not-null value helps you to be more confident in your code.
    // 消除错误地假设非空值的风险可以帮助您对代码更有信心。
    // In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>.
    // 为了有一个可能为null的值，你必须显式地选择该值的类型Option<T>。
    // Then, when you use that value, you are required to explicitly handle the case when the value is null.
    // 然后，当你使用该值时，你需要显式地处理该值为空的情况。
    // Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.
    // 当一个值的类型不是Option<T>时，您可以安全地假设该值不是null。
    // This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.
    // 这是Rust为了限制null的普遍性和增加Rust代码的安全性而深思熟虑的设计决策。

    // So, how do you get the T value out of a Some variant when you have a value of type Option<T>
    // 那么，当你有一个类型为Option<T>的值时，你如何从Some变量中得到T值
    // so you can use that value? The Option<T> enum has a large number of methods
    // 你可以使用这个值吗?Option<T> enum有大量的方法
    // that are useful in a variety of situations; you can check them out in its documentation.
    // 在各种情况下都有用;您可以在其文档中查看它们。
    // Becoming familiar with the methods on Option<T> will be extremely useful in your journey with Rust.
    // 熟悉Option<T>上的方法将在Rust的旅程中非常有用。

    // In general, in order to use an Option<T> value, you want to have code that will handle each variant.
    // 一般来说，为了使用Option<T>值，你需要有处理每个变量的代码。
    // You want some code that will run only when you have a Some(T) value, and this code is allowed to use the inner T.
    // 你想要一些只有当你有一个some (T)值时才会运行的代码，并且这个代码允许使用内部的T。
    // You want some other code to run if you have a None value, and that code doesn’t have a T value available.
    // 如果你有一个None值，你想要一些其他的代码运行，而那个代码没有可用的T值。
    // The match expression is a control flow construct that does just this when used with enums:
    // match表达式是一个控制流结构，当它与枚举一起使用时，它就会这样做:
    // it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.
    // 它将运行不同的代码，这取决于它所拥有的枚举变量，并且该代码可以使用匹配值中的数据。
}
