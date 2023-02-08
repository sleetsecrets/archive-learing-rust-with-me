// Generic Types, Traits, and Lifetimes
// 泛型、特征和生命周期
// Every programming language has tools for effectively handling the duplication of concepts.
// 每种编程语言都有有效处理概念重复的工具。
// In Rust, one such tool is generics: abstract stand-ins for concrete types or other properties.
// 在 Rust 中，一种这样的工具是泛型：具体类型或其他属性的抽象替代品。
// We can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.
// 我们可以表达泛型的行为或它们与其他泛型的关系，而无需知道在编译和运行代码时它们的位置是什么。

// Functions can take parameters of some generic type, instead of a concrete type like i32 or String, in the same way a function takes parameters with unknown values to run the same code on multiple concrete values.
// 函数可以接受一些通用类型的参数，而不是像 i32 或 String 这样的具体类型，就像函数接受具有未知值的参数以在多个具体值上运行相同的代码一样。
// In fact, we’ve already used generics in Chapter 6 with Option<T>, Chapter 8 with Vec<T> and HashMap<K, V>, and Chapter 9 with Result<T, E>.
// 事实上，我们已经在第 6 章的 Option<T>、第 8 章的 Vec<T> 和 HashMap<K, V> 以及第 9 章的 Result<T, E> 中使用了泛型。
// In this chapter, you’ll explore how to define your own types, functions, and methods with generics!
// 在本章中，您将探索如何使用泛型定义您自己的类型、函数和方法！

// First, we’ll review how to extract a function to reduce code duplication.
// 首先，我们将回顾如何提取函数以减少代码重复。
// We’ll then use the same technique to make a generic function from two functions that differ only in the types of their parameters.
// 然后我们将使用相同的技术从两个仅参数类型不同的函数创建一个泛型函数。
// We’ll also explain how to use generic types in struct and enum definitions.
// 我们还将解释如何在结构和枚举定义中使用泛型类型。

// Then you’ll learn how to use traits to define behavior in a generic way.
// 然后您将学习如何使用特征以通用方式（泛型）定义行为。
// You can combine traits with generic types to constrain a generic type to accept only those types that have a particular behavior, as opposed to just any type.
// 您可以将 traits 与泛型类型结合起来，以限制泛型类型只接受那些具有特定行为的类型，而不是只接受任何类型。

// Finally, we’ll discuss lifetimes: a variety of generics that give the compiler information about how references relate to each other.
// 最后，我们将讨论生命周期：各种泛型，它们为编译器提供有关引用如何相互关联的信息。
// Lifetimes allow us to give the compiler enough information about borrowed values so that it can ensure references will be valid in more situations than it could without our help.
// 生命周期功能允许在很多场景下借用值的同时仍然使编译器能够检查这些引用的有效性。

// Removing Duplication by Extracting a Function
// 通过提取函数去除重复
// Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication.
// 泛型允许我们用代表多种类型的占位符替换特定类型，以消除代码重复。
// Before diving into generics syntax, then,
// 在深入研究泛型语法之前，
// |- let’s first look at how to remove duplication in a way that doesn’t involve generic types by extracting a function that replaces specific values with a placeholder that represents multiple values.
// |- 让我们首先看看如何通过提取一个函数来以不涉及泛型类型的方式去除重复，该函数将特定值替换为表示多个值的占位符。
// Then we’ll apply the same technique to extract a generic function!
// 然后我们将应用相同的技术来提取泛型函数！
// By looking at how to recognize duplicated code you can extract into a function, you’ll start to recognize duplicated code that can use generics.
// 通过了解如何识别可以提取到函数中的重复代码，您将开始识别可以使用泛型的重复代码。

// We begin with the short program in Listing 10-1 that finds the largest number in a list.
// 我们从清单 10-1 中的短程序开始，它找到列表中的最大数字。

// Filename: src/main.rs

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

// Listing 10-1: Finding the largest number in a list of numbers
// 示例 10-1：查找数字列表中的最大数字

// We store a list of integers in the variable number_list and place a reference to the first number in the list in a variable named largest.
// 我们将整数列表存储在变量 number_list 中，并将对列表中第一个数字的引用放置在名为 largest 的变量中。
// We then iterate through all the numbers in the list, and if the current number is greater than the number stored in largest, replace the reference in that variable.
// 然后我们遍历列表中的所有数字，如果当前数字大于 largest 中存储的数字，则替换该变量中的引用。
// However, if the current number is less than or equal to the largest number seen so far, the variable doesn’t change, and the code moves on to the next number in the list.
// 但是，如果当前数字小于或等于目前为止看到的最大数字，则变量不会改变，代码会移至列表中的下一个数字。
// After considering all the numbers in the list, largest should refer to the largest number, which in this case is 100.
// 在考虑了列表中的所有数字后，largest 应该是指最大的数字，在本例中为 100。

// We've now been tasked with finding the largest number in two different lists of numbers.
// 我们现在的任务是在两个不同的数字列表中找到最大的数字。
// To do so, we can choose to duplicate the code in Listing 10-1 and use the same logic at two different places in the program, as shown in Listing 10-2.
// 为此，我们可以选择复制清单 10-1 中的代码，并在程序的两个不同位置使用相同的逻辑，如清单 10-2 所示。

// Filename: src/main.rs

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

// Listing 10-2: Code to find the largest number in two lists of numbers
// 示例 10-2：查找两个数字列表中最大数字的代码

// Although this code works, duplicating code is tedious and error prone.
// 尽管此代码有效，但复制代码很乏味且容易出错。
// We also have to remember to update the code in multiple places when we want to change it.
// 我们还必须记住，当我们要更改它时，要在多个地方更新代码。

// To eliminate this duplication, we’ll create an abstraction by defining a function that operates on any list of integers passed in a parameter.
// 为了消除这种重复，我们将通过定义一个函数来创建一个抽象，该函数对传入参数的任何整数列表进行操作。
// This solution makes our code clearer and lets us express the concept of finding the largest number in a list abstractly.
// 这个解决方案让我们的代码更清晰，让我们抽象地表达了寻找列表中最大数的概念。

// In Listing 10-3, we extract the code that finds the largest number into a function named largest.
// 在示例 10-3 中，我们将查找最大数的代码提取到名为 largest 的函数中。
// Then we call the function to find the largest number in the two lists from Listing 10-2.
// 然后我们调用函数来查找示例 10-2 中两个列表中的最大数。
// We could also use the function on any other list of i32 values we might have in the future.
// 我们也可以在未来可能拥有的任何其他 i32 值列表上使用该函数。

// Filename: src/main.rs

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

// Listing 10-3: Abstracted code to find the largest number in two lists
// 示例 10-3：在两个列表中查找最大数的抽象代码

// The largest function has a parameter called list, which represents any concrete slice of i32 values we might pass into the function.
// largest 函数有一个名为 list 的参数，它表示我们可能传递给函数的任何具体的 i32 值切片。
// As a result, when we call the function, the code runs on the specific values that we pass in.
// 因此，当我们调用该函数时，代码会根据我们传入的特定值运行。

// In summary, here are the steps we took to change the code from Listing 10-2 to Listing 10-3:
// 总之，这里是我们将代码从清单 10-2 更改为清单 10-3 所采取的步骤：

// 1.Identify duplicate code.
// 1.识别重复代码。
// 2.Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
// 2.将重复代码提取到函数体中，并在函数签名中指定该代码的输入和返回值。
// 3.Update the two instances of duplicated code to call the function instead.
// 3.更新重复代码的两个实例以调用该函数。

// Next, we’ll use these same steps with generics to reduce code duplication.
// 接下来，我们将对泛型使用这些相同的步骤来减少代码重复。
// In the same way that the function body can operate on an abstract list instead of specific values, generics allow code to operate on abstract types.
// 就像函数体可以对抽象列表而不是特定值进行操作一样，泛型允许代码对抽象类型进行操作。

// For example, say we had two functions: one that finds the largest item in a slice of i32 values and one that finds the largest item in a slice of char values.
// 例如，假设我们有两个函数：一个在 i32 值切片中查找最大项，另一个在 char 值切片中查找最大项。
// How would we eliminate that duplication? Let’s find out!
// 我们如何消除重复？ 让我们找出答案！


// Generic Data Types
// 通用数据类型
// We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.
// 我们使用泛型为函数签名或结构等项目创建定义，然后我们可以将其用于许多不同的具体数据类型。
// Let’s first look at how to define functions, structs, enums, and methods using generics.
// 让我们先看看如何使用泛型定义函数、结构、枚举和方法。
// Then we’ll discuss how generics affect code performance.
// 然后我们将讨论泛型如何影响代码性能。

// In Function Definitions
// 在函数定义中
// When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value.
// 当定义一个使用泛型的函数时，我们将泛型放在函数的签名中，我们通常会在函数的签名中指定参数和返回值的数据类型。
// Doing so makes our code more flexible and provides more functionality to callers of our function while preventing code duplication.
// 这样做使我们的代码更加灵活，并为函数的调用者提供更多功能，同时防止代码重复。

// Continuing with our largest function, Listing 10-4 shows two functions that both find the largest value in a slice.
// 继续我们的 largest 函数，清单 10-4 显示了两个函数，它们都在一个切片中找到最大值。
// We'll then combine these into a single function that uses generics.
// 然后我们将把它们组合成一个使用泛型的函数。

// Filename: src/main.rs

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

// Listing 10-4: Two functions that differ only in their names and the types in their signatures
// 示例 10-4：仅名称和签名类型不同的两个函数

// The largest_i32 function is the one we extracted in Listing 10-3 that finds the largest i32 in a slice.
// largest_i32 函数是我们在清单 10-3 中提取的函数，它在一个切片中找到最大的 i32。
// The largest_char function finds the largest char in a slice.
// largest_char 函数查找切片中最大的字符。
// The function bodies have the same code, so let’s eliminate the duplication by introducing a generic type parameter in a single function.
// 函数体具有相同的代码，所以让我们通过在单个函数中引入泛型类型参数来消除重复。

// To parameterize the types in a new single function, we need to name the type parameter, just as we do for the value parameters to a function.
// 要在新的单个函数中参数化类型，我们需要命名类型参数，就像我们为函数的值参数所做的那样。
// You can use any identifier as a type parameter name.
// 您可以使用任何标识符作为类型参数名称。
// But we’ll use T because, by convention, type parameter names in Rust are short, often just a letter, and Rust’s type-naming convention is CamelCase.
// 但我们将使用 T，因为按照惯例，Rust 中的类型参数名称很短，通常只是一个字母，而 Rust 的类型命名约定是 CamelCase。
// Short for “type,” T is the default choice of most Rust programmers.
// “类型”的缩写 type，T 是大多数 Rust 程序员的默认选择。

// When we use a parameter in the body of the function, we have to declare the parameter name in the signature so the compiler knows what that name means.
// 当我们在函数体中使用参数时，我们必须在签名中声明参数名称，以便编译器知道该名称的含义。
// Similarly, when we use a type parameter name in a function signature, we have to declare the type parameter name before we use it.
// 同样，当我们在函数签名中使用类型参数名称时，我们必须在使用它之前声明类型参数名称。
// To define the generic largest function, place type name declarations inside angle brackets, <>, between the name of the function and the parameter list, like this:
// 要定义通用最大函数（generic largest function），请将类型名称声明放在尖括号 <> 内，位于函数名称和参数列表之间，如下所示：

fn largest<T>(list: &[T]) -> &T {}

// We read this definition as: the function largest is generic over some type T.
// 我们将这个定义解读为：函数 largest 是某种类型 T 上的泛型。
// This function has one parameter named list, which is a slice of values of type T.
// 这个函数有一个名为 list 的参数，它是 T 类型值的一部分。
// The largest function will return a reference to a value of the same type T.
// largest 函数将返回对相同类型 T 的值的引用。

// Listing 10-5 shows the combined largest function definition using the generic data type in its signature.
// 清单 10-5 显示了在其签名中使用泛型数据类型的组合最大函数定义。
// The listing also shows how we can call the function with either a slice of i32 values or char values.
// 该清单还显示了我们如何使用 i32 值或 char 值的切片调用该函数。
// Note that this code won’t compile yet, but we’ll fix it later in this chapter.
// 请注意，这段代码还不能编译，但我们会在本章后面修复它。

// Filename: src/main.rs

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// Listing 10-5: The largest function using generic type parameters; this doesn’t yet compile
// 示例 10-5：使用泛型类型参数的最大函数； 这还没有编译

// If we compile this code right now, we’ll get this error:
// 如果我们现在编译这段代码，我们会得到这个错误：

// $ cargo run
//    Compiling chapter10 v0.1.0 (file:///projects/chapter10)
// error[E0369]: binary operation `>` cannot be applied to type `&T`
//  --> src/main.rs:5:17
//   |
// 5 |         if item > largest {
//   |            ---- ^ ------- &T
//   |            |
//   |            &T
//   |
// help: consider restricting type parameter `T`
//   |
// 1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//   |             ++++++++++++++++++++++
//
// For more information about this error, try `rustc --explain E0369`.
// error: could not compile `chapter10` due to previous error

// The help text mentions std::cmp::PartialOrd, which is a trait, and we’re going to talk about traits in the next section.
// 帮助文中提到了std::cmp::PartialOrd，这是一个trait，我们下一节再讲trait。
// For now, know that this error states that the body of largest won’t work for all possible types that T could be.
// 现在，知道这个错误表明 largest 的主体不适用于 T 可能是的所有可能类型。
// Because we want to compare values of type T in the body, we can only use types whose values can be ordered.
// 因为我们要比较主体中类型 T 的值，所以我们只能使用值可以排序的类型。
// To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types (see Appendix C for more on this trait).
// 为了启用比较，标准库具有 std::cmp::PartialOrd 特性，您可以在类型上实现该特性（有关此特性的更多信息，请参阅附录 C）。
// By following the help text's suggestion, we restrict the types valid for T to only those that implement PartialOrd and this example will compile,
// 按照帮助文本的建议，我们将对 T 有效的类型限制为仅实现 PartialOrd 的类型，并且此示例将编译，
// |- because the standard library implements PartialOrd on both i32 and char.
// |- 因为标准库在 i32 和 char 上都实现了 PartialOrd。

// In Struct Definitions
// 在结构定义中
// We can also define structs to use a generic type parameter in one or more fields using the <> syntax.
// 我们还可以使用 <> 语法定义结构以在一个或多个字段中使用泛型类型参数。
// Listing 10-6 defines a Point<T> struct to hold x and y coordinate values of any type.
// 清单 10-6 定义了一个 Point<T> 结构来保存任何类型的 x 和 y 坐标值。

// Filename: src/main.rs

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

// Listing 10-6: A Point<T> struct that holds x and y values of type T
// 示例 10-6：保存类型 T 的 x 和 y 值的 Point<T> 结构

// The syntax for using generics in struct definitions is similar to that used in function definitions.
// 在结构定义中使用泛型的语法类似于在函数定义中使用的语法。
// First, we declare the name of the type parameter inside angle brackets just after the name of the struct.
// 首先，我们在结构名称之后的尖括号内声明类型参数的名称。
// Then we use the generic type in the struct definition where we would otherwise specify concrete data types.
// 然后我们在结构定义中使用泛型类型，否则我们将指定具体的数据类型。

// Note that because we’ve used only one generic type to define Point<T>, this definition says that the Point<T> struct is generic over some type T,
// 请注意，因为我们只使用了一种泛型类型来定义 Point<T>，所以这个定义表示 Point<T> 结构在某些类型 T 上是泛型的，
// |- and the fields x and y are both that same type, whatever that type may be.
// |- 并且字段 x 和 y 都是相同的类型，无论是什么类型。
// If we create an instance of a Point<T> that has values of different types, as in Listing 10-7, our code won’t compile.
// 如果我们创建一个具有不同类型值的 Point<T> 实例，如示例 10-7 所示，我们的代码将无法编译。

// Filename: src/main.rs

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}

// Listing 10-7: The fields x and y must be the same type because both have the same generic data type T.
// 示例 10-7：字段 x 和 y 必须是同一类型，因为它们具有相同的通用数据类型 T。

// In this example, when we assign the integer value 5 to x, we let the compiler know that the generic type T will be an integer for this instance of Point<T>.
// 在此示例中，当我们将整数值 5 分配给 x 时，我们让编译器知道泛型类型 T 将是此 Point<T> 实例的整数。
// Then when we specify 4.0 for y, which we’ve defined to have the same type as x, we’ll get a type mismatch error like this:
// 然后当我们为 y 指定 4.0 时，我们定义为与 x 具有相同的类型，我们将得到一个类型不匹配错误，如下所示：

// $ cargo run
//    Compiling chapter10 v0.1.0 (file:///projects/chapter10)
// error[E0308]: mismatched types
//  --> src/main.rs:7:38
//   |
// 7 |     let wont_work = Point { x: 5, y: 4.0 };
//   |                                      ^^^ expected integer, found floating-point number
//
// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `chapter10` due to previous error

// To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters.
// 要定义一个点结构，其中 x 和 y 都是泛型但可以有不同的类型，我们可以使用多个泛型类型参数。
// For example, in Listing 10-8, we change the definition of Point to be generic over types T and U where x is of type T and y is of type U.
// 例如，在清单 10-8 中，我们将 Point 的定义更改为对类型 T 和 U 的泛型，其中 x 是 T 类型，y 是 U 类型。

// Filename: src/main.rs

struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

// Listing 10-8: A Point<T, U> generic over two types so that x and y can be values of different types
// 示例 10-8：一个 Point<T, U> 对两种类型的泛型使得 x 和 y 可以是不同类型的值

// Now all the instances of Point shown are allowed! You can use as many generic type parameters in a definition as you want, but using more than a few makes your code hard to read.
// 现在显示的所有 Point 实例都是允许的！ 您可以根据需要在定义中使用任意数量的泛型类型参数，但使用过多会使您的代码难以阅读。
// If you're finding you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces.
// 如果您发现代码中需要大量泛型类型，则可能表明您的代码需要重组为更小的部分。

// In Enum Definitions
// 在枚举定义中
// As we did with structs, we can define enums to hold generic data types in their variants.
// 正如我们对结构所做的那样，我们可以定义枚举以在其变体中保存通用数据类型。
// Let’s take another look at the Option<T> enum that the standard library provides, which we used in Chapter 6:
// 让我们再看一下标准库提供的 Option<T> 枚举，我们在第 6 章中使用了它：

enum Option<T> {
    Some(T),
    None,
}

// This definition should now make more sense to you.
// 这个定义现在对你来说应该更有意义了。
// As you can see, the Option<T> enum is generic over type T and has two variants:
// 如您所见，Option<T> 枚举是 T 类型的泛型，有两个变体：
// |- Some, which holds one value of type T, and a None variant that doesn’t hold any value.
// |- Some，其中包含一个 T 类型的值，以及一个不包含任何值的 None 变体。
// By using the Option<T> enum, we can express the abstract concept of an optional value, and because Option<T> is generic,
// 通过使用 Option<T> 枚举，我们可以表达可选值的抽象概念，并且因为 Option<T> 是通用的，
// |- we can use this abstraction no matter what the type of the optional value is.
// |- 无论可选值的类型是什么，我们都可以使用这个抽象。

// Enums can use multiple generic types as well.
// 枚举也可以使用多个泛型类型。
// The definition of the `Result` enum that we used in Chapter 9 is one example:
// 我们在第 9 章中使用的 `Result` 枚举的定义就是一个例子：

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// The `Result` enum is generic over two types, T and E, and has two variants: Ok, which holds a value of type T, and Err, which holds a value of type E.
// `Result` 枚举在 T 和 E 两种类型上是通用的，并且有两个变体：Ok，它包含类型 T 的值，和 Err，它包含类型 E 的值。
// This definition makes it convenient to use the `Result` enum anywhere we have an operation that might succeed (return a value of some type T) or fail (return an error of some type E).
// 这个定义使得我们可以在任何可能成功（返回某种类型 T 的值）或失败（返回某种类型 E 的错误）的操作的任何地方方便地使用 `Result` 枚举。
// In fact, this is what we used to open a file in Listing 9-3,
// 事实上，这就是我们在清单 9-3 中用来打开文件的方式，
// |- where T was filled in with the type std::fs::File when the file was opened successfully and E was filled in with the type std::io::Error when there were problems opening the file.
// |- 当文件打开成功时，T 填充为 std::fs::File 类型，当打开文件出现问题时，E 填充为 std::io::Error 类型。

// When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types instead.
// 当您发现代码中的多个结构或枚举定义仅在它们所持有的值的类型不同时，您可以通过使用泛型类型来避免重复。

// In Method Definitions
// 在方法定义中
// We can implement methods on structs and enums (as we did in Chapter 5) and use generic types in their definitions, too.
// 我们可以在结构和枚举上实现方法（就像我们在第 5 章中所做的那样）并在它们的定义中也使用泛型类型。
// Listing 10-9 shows the Point<T> struct we defined in Listing 10-6 with a method named x implemented on it.
// 清单 10-9 显示了我们在清单 10-6 中定义的 Point<T> 结构，其中实现了一个名为 x 的方法。

// Filename: src/main.rs

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

// Listing 10-9: Implementing a method named x on the Point<T> struct that will return a reference to the x field of type T
// 示例 10-9：在 Point<T> 结构上实现一个名为 x 的方法，它将返回对类型 T 的 x 字段的引用

// Here, we’ve defined a method named x on Point<T> that returns a reference to the data in the field x.
// 在这里，我们在 Point<T> 上定义了一个名为 x 的方法，它返回对字段 x 中数据的引用。

// Note that we have to declare T just after impl so we can use T to specify that we’re implementing methods on the type Point<T>.
// 请注意，我们必须在 impl 之后声明 T，这样我们就可以使用 T 来指定我们在 Point<T> 类型上实现方法。
// By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
// 通过在 impl 之后将 T 声明为泛型类型，Rust 可以识别出 Point 中尖括号中的类型是泛型类型，而不是具体类型。
// We could have chosen a different name for this generic parameter than the generic parameter declared in the struct definition, but using the same name is conventional.
// 我们可以为此泛型参数选择与结构定义中声明的泛型参数不同的名称，但使用相同的名称是约定俗成的。
// Methods written within an impl that declares the generic type will be defined on any instance of the type, no matter what concrete type ends up substituting for the generic type.
// 在声明泛型类型的 impl 中编写的方法将在该类型的任何实例上定义，无论最终用什么具体类型替代泛型类型。

// We can also specify constraints on generic types when defining methods on the type.
// 我们也可以在类型上定义方法时指定对泛型类型的约束。
// We could, for example, implement methods only on Point<f32> instances rather than on Point<T> instances with any generic type.
// 例如，我们可以仅在 Point<f32> 实例上实现方法，而不是在具有任何泛型类型的 Point<T> 实例上实现方法。
// In Listing 10-10 we use the concrete type f32, meaning we don’t declare any types after impl.
// 在示例 10-10 中，我们使用了具体类型 f32，这意味着我们没有在 impl 之后声明任何类型。

// Filename: src/main.rs

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Listing 10-10: An impl block that only applies to a struct with a particular concrete type for the generic type parameter T
// 示例 10-10：仅适用于具有泛型类型参数 T 的特定具体类型的结构的 impl 块

// This code means the type Point<f32> will have a distance_from_origin method; other instances of Point<T> where T is not of type f32 will not have this method defined.
// 这段代码意味着类型 Point<f32> 将有一个 distance_from_origin 方法； T 不是 f32 类型的其他 Point<T> 实例将不会定义此方法。
// The method measures how far our point is from the point at coordinates (0.0, 0.0) and uses mathematical operations that are available only for floating point types.
// 该方法测量我们的点距离坐标 (0.0, 0.0) 处的点有多远，并使用仅适用于浮点类型的数学运算。

// Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures.
// 结构定义中的通用类型参数并不总是与您在同一结构的方法签名中使用的参数相同。
// Listing 10-11 uses the generic types X1 and Y1 for the Point struct and X2 Y2 for the mixup method signature to make the example clearer.
// 清单 10-11 将泛型类型 X1 和 Y1 用于 Point 结构，将 X2 Y2 用于混合方法签名以使示例更清晰。
// The method creates a new Point instance with the x value from the self Point (of type X1) and the y value from the passed-in Point (of type Y2).
// 该方法创建一个新的 Point 实例，其 x 值来自自身 Point（类型 X1），y 值来自传入的 Point（类型 Y2）。

// Filename: src/main.rs

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// Listing 10-11: A method that uses generic types different from its struct’s definition
// 示例 10-11：一个使用不同于其结构定义的泛型类型的方法

// In main, we’ve defined a Point that has an i32 for x (with value 5) and an f64 for y (with value 10.4).
// 在 main 中，我们定义了一个 Point，它具有 x 的 i32（值为 5）和 y 的 f64（值为 10.4）。
// The p2 variable is a Point struct that has a string slice for x (with value "Hello") and a char for y (with value c).
// p2 变量是一个 Point 结构，它有一个用于 x 的字符串切片（值为“Hello”）和一个用于 y 的字符（值为 c）。
// Calling mixup on p1 with the argument p2 gives us p3, which will have an i32 for x, because x came from p1.
// 使用参数 p2 在 p1 上调用 mixup 会得到 p3，它将具有 x 的 i32，因为 x 来自 p1。
// The p3 variable will have a char for y, because y came from p2.
// p3 变量将有一个代表 y 的字符，因为 y 来自 p2。
// The println! macro call will print p3.x = 5, p3.y = c.
// println! 宏调用将打印 p3.x = 5, p3.y = c。

// The purpose of this example is to demonstrate a situation in which some generic parameters are declared with impl and some are declared with the method definition.
// 这个例子的目的是为了演示一种情况，其中一些泛型参数是用impl声明的，一些是用方法定义声明的。
// Here, the generic parameters X1 and Y1 are declared after impl because they go with the struct definition.
// 这里，通用参数 X1 和 Y1 在 impl 之后声明，因为它们与结构定义一致。
// The generic parameters X2 and Y2 are declared after fn mixup, because they’re only relevant to the method.
// 泛型参数 X2 和 Y2 在 fn mixup 之后声明，因为它们只与方法相关。

// Performance of Code Using Generics
// 使用泛型的代码性能
// You might be wondering whether there is a runtime cost when using generic type parameters.
// 你可能想知道使用泛型类型参数时是否会产生运行时成本。
// The good news is that using generic types won't make your program run any slower than it would with concrete types.
// 好消息是使用泛型类型不会使您的程序运行速度比使用具体类型慢。

// Rust accomplishes this by performing monomorphization of the code using generics at compile time.
// Rust 通过在编译时使用泛型对代码进行单态化来实现这一点。
// Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
// Monomorphization 是通过填充编译时使用的具体类型，将通用代码变成特定代码的过程。
// In this process, the compiler does the opposite of the steps we used to create the generic function in Listing 10-5:
// 在此过程中，编译器执行与我们在示例 10-5 中创建泛型函数的步骤相反的操作：
// |- the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.
// |- 编译器查看所有调用泛型代码的地方，并为调用泛型代码的具体类型生成代码。

// Let’s look at how this works by using the standard library’s generic Option<T> enum:
// 让我们通过使用标准库的通用 Option<T> 枚举看看它是如何工作的：

let integer = Some(5);
let float = Some(5.0);

// When Rust compiles this code, it performs monomorphization.
// 当 Rust 编译这段代码时，它会执行单态化。
// During that process, the compiler reads the values that have been used in Option<T> instances and identifies two kinds of Option<T>: one is i32 and the other is f64.
// 在此过程中，编译器读取已在 Option<T> 实例中使用的值并识别两种 Option<T>：一种是 i32，另一种是 f64。
// As such, it expands the generic definition of Option<T> into two definitions specialized to i32 and f64, thereby replacing the generic definition with the specific ones.
// 因此，它将 Option<T> 的通用定义扩展为两个专门针对 i32 和 f64 的定义，从而用特定的定义替换通用定义。

// The monomorphized version of the code looks similar to the following (the compiler uses different names than what we’re using here for illustration):
// 代码的单态化版本类似于以下内容（编译器使用的名称与我们在此处用于说明的名称不同）：

// Filename: src/main.rs

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

// The generic Option<T> is replaced with the specific definitions created by the compiler.
// 通用 Option<T> 替换为编译器创建的特定定义。
// Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics.
// 因为 Rust 将泛型代码编译成在每个实例中指定类型的代码，所以我们不会为使用泛型支付任何运行时成本。
// When the code runs, it performs just as it would if we had duplicated each definition by hand.
// 当代码运行时，它的执行就像我们手动复制每个定义一样。
// The process of monomorphization makes Rust’s generics extremely efficient at runtime.
// 单态化的过程使得 Rust 的泛型在运行时非常高效。

// Traits: Defining Shared Behavior
// 特征：定义共享行为
// A trait defines functionality a particular type has and can share with other types.
// 特征定义特定类型具有并可以与其他类型共享的功能。
// We can use traits to define shared behavior in an abstract way.
// 我们可以使用特征以抽象的方式定义共享行为。
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.
// 我们可以使用 trait bounds 来指定泛型类型可以是任何具有特定行为的类型。

// Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
// 注意：Traits 类似于在其他语言中通常称为接口的功能，尽管有一些差异。

// Defining a Trait
// 定义特征
// A type’s behavior consists of the methods we can call on that type.
// 类型的行为由我们可以对该类型调用的方法组成。
// Different types share the same behavior if we can call the same methods on all of those types.
// 如果我们可以对所有这些类型调用相同的方法，则不同的类型会共享相同的行为。
// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
// 特征定义是一种将方法签名组合在一起以定义完成某些目的所需的一组行为的方法。

// For example, let’s say we have multiple structs that hold various kinds and amounts of text:
// 例如，假设我们有多个结构来保存各种类型和数量的文本：
// |- a NewsArticle struct that holds a news story filed in a particular location and a Tweet that can have at most 280 characters along with metadata that indicates whether it was a new tweet,
// |- 一个 NewsArticle 结构，它包含一个在特定位置归档的新闻故事和一个最多包含 280 个字符的推文以及指示它是否是新推文的元数据，
// |- a retweet, or a reply to another tweet.
// |- 转发或回复另一条推文。

// We want to make a media aggregator library crate named aggregator that can display summaries of data that might be stored in a NewsArticle or Tweet instance.
// 我们想要制作一个名为 aggregator 的媒体聚合器库 crate，它可以显示可能存储在 NewsArticle 或 Tweet 实例中的数据摘要。
// To do this, we need a summary from each type, and we’ll request that summary by calling a summarize method on an instance.
// 为此，我们需要每种类型的摘要，我们将通过在实例上调用摘要方法来请求该摘要。
// Listing 10-12 shows the definition of a public Summary trait that expresses this behavior.
// 清单 10-12 显示了表达此行为的公共摘要特征的定义。

// Filename: src/lib.rs

pub trait Summary {
    fn summarize(&self) -> String;
}

// Listing 10-12: A Summary trait that consists of the behavior provided by a summarize method
// 示例 10-12：由 summarize 方法提供的行为组成的 Summary trait

// Here, we declare a trait using the trait keyword and then the trait’s name, which is Summary in this case.
// 在这里，我们使用 trait 关键字声明一个特征，然后是特征的名称，在本例中为 Summary。
// We’ve also declared the trait as pub so that crates depending on this crate can make use of this trait too, as we’ll see in a few examples.
// 我们还声明了 trait 为 pub，这样依赖于这个 crate 的 crate 也可以使用这个 trait，正如我们将在几个例子中看到的那样。
// Inside the curly brackets, we declare the method signatures that describe the behaviors of the types that implement this trait, which in this case is fn summarize(&self) -> String.
// 在大括号内，我们声明了描述实现此特征的类型的行为的方法签名，在本例中为 fn summarize(&self) -> String。

// After the method signature, instead of providing an implementation within curly brackets, we use a semicolon.
// 在方法签名之后，我们没有在大括号内提供实现，而是使用分号。
// Each type implementing this trait must provide its own custom behavior for the body of the method.
// 实现此特征的每个类型都必须为方法主体提供自己的自定义行为。
// The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.
// 编译器将强制任何具有 Summary 特征的类型都将精确地使用此签名定义方法 summarize。

// A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.
// 一个 trait 的主体中可以有多个方法：方法签名每行列出一个，每行以分号结尾。

// Implementing a Trait on a Type
// 在类型上实现特征
// Now that we’ve defined the desired signatures of the Summary trait’s methods, we can implement it on the types in our media aggregator.
// 现在我们已经定义了 Summary trait 方法所需的签名，我们可以在我们的媒体聚合器中的类型上实现它。
// Listing 10-13 shows an implementation of the Summary trait on the NewsArticle struct that uses the headline, the author, and the location to create the return value of summarize.
// 清单 10-13 显示了 NewsArticle 结构上 Summary 特性的实现，它使用标题、作者和位置来创建 summarize 的返回值。
// For the Tweet struct, we define summarize as the username followed by the entire text of the tweet, assuming that tweet content is already limited to 280 characters.
// 对于推文结构，我们将摘要定义为用户名后跟推文的整个文本，假设推文内容已限制为 280 个字符。

// Filename: src/lib.rs

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Listing 10-13: Implementing the Summary trait on the NewsArticle and Tweet types
// 示例 10-13：在 NewsArticle 和 Tweet 类型上实现 Summary trait

// Implementing a trait on a type is similar to implementing regular methods.
// 在类型上实现特征类似于实现常规方法。
// The difference is that after impl, we put the trait name we want to implement, then use the for keyword, and then specify the name of the type we want to implement the trait for.
// 不同的是，在impl之后，我们放上了我们要实现的trait名称，然后使用for关键字，然后指定我们要为其实现trait的类型的名称。
// Within the impl block, we put the method signatures that the trait definition has defined.
// 在 impl 块中，我们放置了特征定义定义的方法签名。
// Instead of adding a semicolon after each signature,
// 而不是在每个签名后添加分号，
// |- we use curly brackets and fill in the method body with the specific behavior that we want the methods of the trait to have for the particular type.
// |- 我们使用大括号并在方法主体中填写我们希望特性的方法针对特定类型具有的特定行为。

// Now that the library has implemented the Summary trait on NewsArticle and Tweet,
// 现在库已经在 NewsArticle 和 Tweet 上实现了 Summary 特性，
// |- users of the crate can call the trait methods on instances of NewsArticle and Tweet in the same way we call regular methods.
// |- crate 的用户可以像调用常规方法一样调用 NewsArticle 和 Tweet 实例的特征方法。
// The only difference is that the user must bring the trait into scope as well as the types.
// 唯一的区别是用户必须将特征和类型一起带入范围。
// Here’s an example of how a binary crate could use our aggregator library crate:
// 这是一个二进制包如何使用我们的聚合器库包的示例：

use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

// This code prints 1 new tweet: horse_ebooks: of course, as you probably already know, people.
// 此代码打印 1 条新推文：horse_ebooks：当然，正如您可能已经知道的那样，人们。


// Other crates that depend on the `aggregator crate` can also bring the `Summary` trait into scope to implement `Summary` on their own `types`.
// One restriction to note is that we can implement a `trait` on a `type` only if at least one of the `trait` or the `type` is local to our crate.
// For example, we can implement `standard library traits` like `Display` on a `custom type` like `Tweet` as part of our `aggregator crate` functionality,
// |- because the `type Tweet` is local to our `aggregator crate`.
// We can also implement `Summary` on Vec<T> in our `aggregator crate`, because the `trait Summary` is local to our `aggregator crate`.
// But we can’t implement external `traits` on external `types`.
// For example, we can’t implement the `Display` trait on Vec<T> within our `aggregator crate`,
// |- because `Display` and Vec<T> are both defined in the `standard library` and aren’t local to our `aggregator crate`.
// This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent `type` is not present.
// This rule ensures that other people’s code can’t break your code and vice versa.
// Without the rule, two crates could implement the same trait for the same `type`, and Rust wouldn’t know which implementation to use.

// 其他依赖于 `aggregator crate` 的 crate 也可以将 `Summary` trait 引入作用域以在它们自己的 `types` 上实现 `Summary`。
// 需要注意的一个限制是，只有在 `trait` 或 `type` 中至少有一个是我们的 crate 本地的情况下，我们才能在 `type` 上实现 `trait`。
// 例如，我们可以在 `Tweet` 等 `custom type` 上实现 `standard library traits` ，如 `Display` 作为我们 `aggregator crate` 功能的一部分，
// |- 因为 `type Tweet` 对于我们的 `aggregator crate` 来说是本地的。
// 我们也可以在我们的 `aggregator crate` 中的 Vec<T> 上实现 `Summary`，因为 `trait Summary` 是我们的 `aggregator crate` 的本地 trait。
// 但我们不能在外部 `types` 上实现外部 `traits`。
// 例如，我们不能在我们的 `aggregator crate` 中实现 Vec<T> 上的 `Display` trait，
// |- 因为 `Display` 和 Vec<T> 都是在 `standard library` 中定义的，而不是我们的 `aggregator crate` 本地的。
// 此限制是称为连贯性的属性的一部分，更具体地说是孤儿规则，之所以如此命名是因为父“类型”不存在。
// 这个规则确保其他人的代码不能破坏你的代码，反之亦然。
// 如果没有这条规则，两个 crate 可以为相同的 `type` 实现相同的特征，Rust 不知道要使用哪个实现。


// Default Implementations
// 默认实现
// Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type.
// 有时，为 trait 中的部分或所有方法设置默认行为而不是要求为每种类型的所有方法都实现默认行为很有用。
// Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.
// 然后，当我们在特定类型上实现特征时，我们可以保留或覆盖每个方法的默认行为。

// In Listing 10-14 we specify a default string for the summarize method of the Summary trait instead of only defining the method signature, as we did in Listing 10-12.
// 在示例 10-14 中，我们为 Summary 特性的总结方法指定了一个默认字符串，而不是像示例 10-12 中那样只定义方法签名。

// Filename: src/lib.rs

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Listing 10-14: Defining a `Summary trait` with a default implementation of the `summarize method`
// 示例 10-14：使用 `summarize 方法` 的默认实现定义 `Summary trait`

// To use a default implementation to `summarize` instances of `NewsArticle`, we specify an empty impl block with impl `Summary` for `NewsArticle` {}.
// 要使用默认实现对 `NewsArticle` 的 `summarize` 实例，我们指定一个空的 impl 块，其中包含 impl `Summary` for `NewsArticle` {}。

// Even though we’re no longer defining the `summarize` method on `NewsArticle` directly, we’ve provided a default implementation and specified that `NewsArticle` implements the `Summary trait`.
// 尽管我们不再直接在 `NewsArticle` 上定义 `summarize` 方法，但我们提供了默认实现并指定 `NewsArticle` 实现了 `Summary trait`。
// As a result, we can still call the `summarize` method on an instance of `NewsArticle`, like this:
// 因此，我们仍然可以在 `NewsArticle` 的实例上调用 `summarize` 方法，如下所示：

let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
    ),
};

println!("New article available! {}", article.summarize());

// This code prints `New article available! (Read more...).`
// 此代码打印 `有新文章可用！ （阅读更多...）。`

// Creating a default implementation doesn’t require us to change anything about the implementation of `Summary` on `Tweet` in Listing 10-13.
// 创建一个默认实现并不需要我们更改清单 10-13 中关于 `Tweet` 的 `Summary` 实现的任何内容。
// The reason is that the syntax for overriding a default implementation is the same as the syntax for implementing a `trait` method that doesn’t have a default implementation.
// 原因是覆盖默认实现的语法与实现没有默认实现的 `trait` 方法的语法相同。

// Default implementations can call other methods in the same `trait`, even if those other methods don’t have a default implementation.
// 默认实现可以调用相同 `trait` 中的其他方法，即使那些其他方法没有默认实现。
// In this way, a `trait` can provide a lot of useful functionality and only require implementors to specify a small part of it.
// 这样，一个 `trait` 可以提供很多有用的功能，而只需要实现者指定其中的一小部分。
// For example, we could define the `Summary trait` to have a summarize_author method whose implementation is required,
// 例如，我们可以将 `Summary trait` 定义为具有需要实现的 summarize_author 方法，
// |- and then define a summarize method that has a default implementation that calls the summarize_author method:
// |- 然后定义一个 summarize 方法，该方法具有调用 summarize_author 方法的默认实现：

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// To use this version of `Summary`, we only need to define `summarize_author` when we implement the trait on a type:
// 要使用这个版本的 `Summary`，我们只需要在 type 上实现 trait 时定义 `summarize_author`：

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// After we define `summarize_author`, we can call `summarize` on instances of the `Tweet struct`,
// 在我们定义了 `summarize_author` 之后，我们可以在 `Tweet struct` 的实例上调用 `summarize`，
// |- and the default implementation of `summarize` will call the definition of `summarize_author` that we’ve provided.
// |- `summarize` 的默认实现将调用我们提供的 `summarize_author` 的定义。
// Because we’ve implemented `summarize_author`, the `Summary trait` has given us the behavior of the `summarize` method without requiring us to write any more code.
// 因为我们已经实现了 `summarize_author`，所以 `Summary trait` 已经为我们提供了 `summarize` 方法的行为，而不需要我们编写更多代码。

let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());

// This code prints `1 new tweet: (Read more from @horse_ebooks...).`
// 此代码打印 `1 条新推文：（从@horse_ebooks 阅读更多...）。`

// Note that it isn’t possible to call the default implementation from an overriding implementation of that same method.
// 请注意，无法从同一方法的覆盖实现中调用默认实现。

// Traits as Parameters
// 特征作为参数
// Now that you know how to define and implement `traits`, we can explore how to use `traits` to define functions that accept many different types.
// 现在您已经知道如何定义和实现 `traits`，我们可以探索如何使用 `traits` 来定义接受多种不同类型的函数。
// We'll use the `Summary trait` we implemented on the `NewsArticle` and `Tweet` `types` in Listing 10-13 to define a `notify function` that calls the `summarize method` on its item parameter, which is of some type that implements the `Summary trait`.
// 我们将使用我们在清单 10-13 中的 `NewsArticle` 和 `Tweet` `types` 上实现的 `Summary trait` 来定义一个 `notify function`，它在其 item 参数上调用 `summarize 方法`，它 是某种实现了“Summary trait”的类型。
// To do this, we use the impl `Trait syntax`, like this:
// 为此，我们使用 impl `Trait syntax`，如下所示：

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name.
// 我们指定 impl 关键字和特征名称，而不是 item 参数的具体类型。
// This parameter accepts any type that implements the specified trait.
// 此参数接受任何实现指定特征的类型。
// In the body of notify, we can call any methods on item that come from the `Summary` trait, such as summarize.
// 在 notify 的主体中，我们可以调用来自 `Summary` 特征的 item 上的任何方法，例如 summarize。
// We can call notify and pass in any instance of NewsArticle or Tweet.
// 我们可以调用通知并传入 NewsArticle 或 Tweet 的任何实例。
// Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement `Summary`.
// 使用任何其他类型（例如 String 或 i32）调用函数的代码将无法编译，因为这些类型未实现 `Summary`。

// Trait Bound Syntax
// 特征绑定语法
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound; it looks like this:
// impl Trait 语法适用于简单的情况，但实际上是一种较长形式的语法糖，称为特征边界； 它看起来像这样：

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// This longer form is equivalent to the example in the previous section but is more verbose.
// 这种较长的形式与上一节中的示例等效，但更加冗长。
// We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.
// 我们将 trait bounds 与泛型类型参数的声明放在冒号之后和尖括号内。

// The impl Trait syntax is convenient and makes for more concise code in simple cases, while the fuller trait bound syntax can express more complexity in other cases.
// impl Trait 语法很方便，在简单的情况下代码更简洁，而更完整的 trait bound 语法可以在其他情况下表达更多的复杂性。
// For example, we can have two parameters that implement `Summary`.
// 例如，我们可以有两个实现 `Summary` 的参数。
// Doing so with the `impl Trait syntax` looks like this:
// 使用 `impl Trait syntax` 这样做看起来像这样：

pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

// Using impl Trait is appropriate if we want this function to allow item1 and item2 to have different types (as long as both types implement Summary).
// 如果我们希望此函数允许 item1 和 item2 具有不同的类型（只要两种类型都实现 Summary），则使用 impl Trait 是合适的。
// If we want to force both parameters to have the same type, however, we must use a trait bound, like this:
// 但是，如果我们想强制两个参数具有相同的类型，我们必须使用特征绑定，如下所示：

pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// The generic type T specified as the type of the item1 and item2 parameters constrains the function such that the concrete type of the value passed as an argument for item1 and item2 must be the same.
// 指定为 item1 和 item2 参数类型的泛型 T 约束函数，使得作为参数传递给 item1 和 item2 的值的具体类型必须相同。

// Specifying Multiple Trait Bounds with the + Syntax
// 使用 + 语法指定多个特征边界
// We can also specify more than one trait bound.
// 我们也可以指定多个 trait bound。
// Say we wanted `notify` to use display formatting as well as `summarize` on `item`: we specify in the `notify` definition that item must implement both `Display` and `Summary`.
// 假设我们希望 `notify` 在 `item` 上使用 display formatting 和 `summarize`：我们在 `notify` 定义中指定 item 必须同时实现 `Display` 和 `Summary`。
// We can do so using the + syntax:
// 我们可以使用 + 语法来做到这一点：

pub fn notify(item: &(impl Summary + Display)) {}

// The + syntax is also valid with trait bounds on generic types:
// + 语法对于泛型类型的 trait bounds 也是有效的：

pub fn notify<T: Summary + Display>(item: &T) {}

// With the two trait bounds specified, the body of notify can call summarize and use {} to format item.
// 指定两个 trait bounds 后，notify 的主体可以调用 summarize 并使用 {} 格式化 item。

// Clearer Trait Bounds with where Clauses
// 更清晰的 Trait Bounds 与 where 子句
// Using too many trait bounds has its downsides.
// 使用太多的 trait bounds 有它的缺点。
// Each generic has its own trait bounds,
// 每个泛型都有自己的特征边界，
// |- so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list,
// |- 因此具有多个泛型类型参数的函数可以在函数名称及其参数列表之间包含大量特征绑定信息，
// |- making the function signature hard to read.
// |- 使函数签名难以阅读。
// For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature.
// 出于这个原因，Rust 在函数签名之后的 where 子句中使用替代语法来指定特征边界。
// So instead of writing this:
// 所以不要这样写：

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// we can use a where clause, like this:
// 我们可以使用 where 子句，如下所示：

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}

// This function’s signature is less cluttered: the function name, parameter list, and return type are close together, similar to a function without lots of trait bounds.
// 这个函数的签名不那么混乱：函数名、参数列表和返回类型靠得很近，类似于没有很多特征边界的函数。

// Returning Types that Implement Traits
// 返回实现特征的类型
// We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait, as shown here:
// 我们还可以在返回位置使用 impl Trait 语法来返回实现特征的某种类型的值，如下所示：

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// By using impl `Summary` for the return type, we specify that the `returns_summarizable` function returns some type that implements the `Summary trait` without naming the concrete type.
// 通过对返回类型使用 impl `Summary`，我们指定 `returns_summarizable` 函数返回某种实现 `Summary trait` 的类型，而不命名具体类型。
// In this case, `returns_summarizable` returns a `Tweet`, but the code calling this function doesn’t need to know that.
// 在这种情况下，`returns_summarizable` 返回一条 `Tweet`，但调用此函数的代码不需要知道这一点。

// The ability to specify a return type only by the trait it implements is especially useful in the context of closures and iterators, which we cover in Chapter 13.
// 仅通过它实现的特征来指定返回类型的能力在 closures and iterators 的上下文中特别有用，我们将在第 13 章中介绍。
// Closures and iterators create types that only the compiler knows or types that are very long to specify.
// Closures and iterators 创建只有编译器知道的类型或需要指定很长的类型。
// The impl Trait syntax lets you concisely specify that a function returns some type that implements the Iterator trait without needing to write out a very long type.
// impl Trait 语法可以让你简洁地指定一个函数返回某种实现 Iterator trait 的类型，而不需要写出很长的类型。

// However, you can only use impl Trait if you’re returning a single type.
// 但是，如果您只能使用 impl Trait返回单一类型。
// For example, this code that returns either a `NewsArticle` or a `Tweet` with the return type specified as impl `Summary` wouldn’t work:
// 例如，返回 `NewsArticle` 或 `Tweet` 且返回类型指定为 impl `Summary` 的代码将不起作用：

fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

// Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler.
// 由于在编译器中实现 impl Trait 语法的限制，不允许返回 `NewsArticle` 或 `Tweet`。
// We’ll cover how to write a function with this behavior in the “Using Trait Objects That Allow for Values of Different Types” section of Chapter 17.
// 我们将在第 17 章的“使用允许不同类型值的特征对象”部分介绍如何编写具有此行为的函数。

// Using Trait Bounds to Conditionally Implement Methods
// 使用 Trait Bounds 有条件地实现方法
// By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
// 通过使用与使用泛型类型参数的 a trait bound with an impl block ，我们可以有条件地为实现指定特征的类型实现方法。
// For example, the type Pair<T> in Listing 10-15 always implements the new function to return a new instance of Pair<T>
// 例如，示例 10-15 中的类型 Pair<T> 始终实现新函数以返回 Pair<T> 的新实例
// |- (recall from the “Defining Methods” section of Chapter 5 that Self is a type alias for the type of the impl block, which in this case is Pair<T>).
// |-（回想第 5 章的“定义方法”部分，Self 是 impl 块类型的类型别名，在本例中为 Pair<T>）。
// But in the next impl block, Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.
// 但在下一个 impl 块中，如果 Pair<T> 的内部类型 T 实现了启用比较的 PartialOrd 特性和启用打印的 Display 特性，则仅实现 cmp_display 方法。

// Filename: src/lib.rs

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Listing 10-15: Conditionally implementing methods on a generic type depending on trait bounds
// 示例 10-15：根据 trait bounds 在泛型类型上有条件地实现方法

// We can also conditionally implement a trait for any type that implements another trait.
// 我们还可以有条件地为实现另一个特征的任何类型实现一个特征。
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the `Rust standard library`.
// 对满足特征边界的任何类型的特征实现称为一揽子实现，并在 `Rust 标准库` 中广泛使用。
// For example, the standard library implements the ToString trait on any type that implements the Display trait.
// 例如，标准库在任何实现 Display 特性的类型上实现 ToString 特性。
// The impl block in the standard library looks similar to this code:
// 标准库中的 impl 块看起来类似于以下代码：

impl<T: Display> ToString for T {
    // --snip--
}

// Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait.
// 因为标准库有这个全面的实现，我们可以在任何实现 Display 特性的类型上调用由 ToString 特性定义的 to_string 方法。
// For example, we can turn integers into their corresponding String values like this because integers implement Display:
// 例如，我们可以像这样将整数转换为它们对应的字符串值，因为整数实现了 Display：

let s = 3.to_string();

// Blanket implementations appear in the documentation for the trait in the “Implementors” section.
// 一揽子实现出现在“实现者”部分的特征文档中。

// Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior.
// 特征和特征边界让我们编写的代码使用泛型类型参数来减少重复，但也向编译器指定我们希望泛型类型具有特定行为。
// The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior.
// 然后，编译器可以使用特征绑定信息来检查代码中使用的所有具体类型是否提供了正确的行为。
// In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t define the method.
// 在动态类型语言中，如果我们调用一个没有定义方法的类型的方法，我们会在运行时得到一个错误。
// But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run.
// 但 Rust 将这些错误移至编译时，因此我们不得不在我们的代码能够运行之前修复这些问题。
// Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time.
// 此外，我们不必编写在运行时检查行为的代码，因为我们已经在编译时进行了检查。
// Doing so improves performance without having to give up the flexibility of generics.
// 这样做可以提高性能，而不必放弃泛型的灵活性。

// Validating References with Lifetimes
// 使用生命周期验证引用
// Lifetimes are another kind of generic that we’ve already been using.
// 生命周期是我们已经使用过的另一种泛型。
// Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.
// 生命周期不是确保类型具有我们想要的行为，而是确保引用在我们需要时有效。

// One detail we didn’t discuss in the “References and Borrowing” section in Chapter 4 is that every reference in Rust has a lifetime, which is the scope for which that reference is valid.
// 我们在第 4 章的“引用和借用”部分没有讨论的一个细节是 Rust 中的每个引用都有生命周期，即该引用有效的范围。
// Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.
// 大多数时候，生命周期是隐式的和推断的，就像大多数时候，类型是推断的一样。
// We only must annotate types when multiple types are possible.
// 只有在可能存在多种类型时，我们才必须注释类型。
// In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.
// 以类似的方式，当引用的生命周期可以通过几种不同的方式相关时，我们必须注释生命周期。
// Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.
// Rust 要求我们使用通用生命周期参数来注释关系，以确保在运行时使用的实际引用肯定有效。

// Annotating lifetimes is not even a concept most other programming languages have, so this is going to feel unfamiliar.
// 注释生命周期甚至不是大多数其他编程语言的概念，所以这会让人感到陌生。
// Although we won’t cover lifetimes in their entirety in this chapter, we’ll discuss common ways you might encounter lifetime syntax so you can get comfortable with the concept.
// 虽然我们不会在本章中完整介绍生命周期，但我们将讨论您可能会遇到生命周期语法的常见方式，以便您熟悉这个概念。

// Preventing Dangling References with Lifetimes
// 用生命周期防止悬挂引用
// The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.
// 生命周期的主要目的是防止悬挂引用，悬挂引用会导致程序引用它打算引用的数据以外的数据。
// Consider the program in Listing 10-16, which has an outer scope and an inner scope.
// 考虑清单 10-16 中的程序，它有一个外部作用域和一个内部作用域。

fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

// Listing 10-16: An attempt to use a reference whose value has gone out of scope
// 示例 10-16：尝试使用其值超出范围的引用

// Note: The examples in Listings 10-16, 10-17, and 10-23 declare variables without giving them an initial value, so the variable name exists in the outer scope.
// 注意：清单 10-16、10-17 和 10-23 中的示例在声明变量时没有给它们初始值，因此变量名存在于外部作用域中。
// At first glance, this might appear to be in conflict with Rust’s having no null values.
// 乍一看，这似乎与 Rust 没有空值相冲突。
// However, if we try to use a variable before giving it a value, we’ll get a compile-time error, which shows that Rust indeed does not allow null values.
// 但是，如果我们在给变量赋值之前尝试使用它，我们会得到一个编译时错误，这表明 Rust 确实不允许空值。

// The outer scope declares a variable named r with no initial value, and the inner scope declares a variable named x with the initial value of 5.
// 外层作用域声明了一个没有初值的变量r，内层作用域声明了一个初值为5的变量x。
// Inside the inner scope, we attempt to set the value of r as a reference to x.
// 在内部范围内，我们尝试将 r 的值设置为对 x 的引用。
// Then the inner scope ends, and we attempt to print the value in r.
// 然后内部范围结束，我们尝试打印 r 中的值。
// This code won’t compile because the value r is referring to has gone out of scope before we try to use it.
// 这段代码无法编译，因为 r 所指的值在我们尝试使用它之前已经超出范围。
// Here is the error message:
// 这是错误信息：

// $ cargo run
//    Compiling chapter10 v0.1.0 (file:///projects/chapter10)
// error[E0597]: `x` does not live long enough
//  --> src/main.rs:6:13
//   |
// 6 |         r = &x;
//   |             ^^ borrowed value does not live long enough
// 7 |     }
//   |     - `x` dropped here while still borrowed
// 8 |
// 9 |     println!("r: {}", r);
//   |                       - borrow later used here
//
// For more information about this error, try `rustc --explain E0597`.
// error: could not compile `chapter10` due to previous error

// The variable x doesn’t “live long enough.” The reason is that x will be out of scope when the inner scope ends on line 7.
// 变量 x 没有“活得足够长”。 原因是当内部范围在第 7 行结束时 x 将超出范围。
// But r is still valid for the outer scope; because its scope is larger, we say that it “lives longer.” If Rust allowed this code to work,
// 但 r 对外部作用域仍然有效； 因为它的范围更大，我们说它“寿命更长”。 如果 Rust 允许这段代码工作，
// |- r would be referencing memory that was deallocated when x went out of scope, and anything we tried to do with r wouldn’t work correctly.
// |- r 将引用 x 超出范围时释放的内存，我们尝试用 r 做的任何事情都不会正常工作。
// So how does Rust determine that this code is invalid? It uses a borrow checker.
// 那么 Rust 是如何确定这段代码无效的呢？ 它使用借用检查器。

// The Borrow Checker
// 借用检查器
// The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
// Rust 编译器有一个借用检查器，它比较范围以确定所有借用是否有效。
// Listing 10-17 shows the same code as Listing 10-16 but with annotations showing the lifetimes of the variables.
// 清单 10-17 显示了与清单 10-16 相同的代码，但带有显示变量生命周期的注释。

fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+

// Listing 10-17: Annotations of the lifetimes of r and x, named 'a and 'b, respectively
// 示例 10-17：r 和 x 的生命周期注解，分别命名为 'a 和 'b

// Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b.
// 在这里，我们用 'a 注释了 r 的生命周期，用 'b 注释了 x 的生命周期。
// As you can see, the inner 'b block is much smaller than the outer 'a lifetime block.
// 如您所见，内部 'b 块比外部 'a 生命周期块小得多。
// At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b.
// 在编译时，Rust 比较两个生命周期的大小，发现 r 的生命周期为 'a，但它指的是生命周期为 'b 的内存。
// The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.
// 该程序被拒绝，因为 'b 的生命周期 比 'a 的生命周期短。

// Listing 10-18 fixes the code so it doesn’t have a dangling reference and compiles without any errors.
// 清单 10-18 修复了代码，因此它没有悬空引用并且编译没有任何错误。

fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+

// Listing 10-18: A valid reference because the data has a longer lifetime than the reference
// 示例 10-18：一个有效的引用，因为数据的生命周期比引用长

// Here, x has the lifetime 'b, which in this case is larger than 'a.
// 此处，x 的生命周期为 'b，在本例中大于 'a。
// This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.
// 这意味着 r 可以引用 x，因为 Rust 知道 r 中的引用在 x 有效时始终有效。

// Now that you know where the lifetimes of references are and how Rust analyzes lifetimes to ensure references will always be valid,
// 现在您知道引用的生命周期在哪里以及 Rust 如何分析生命周期以确保引用始终有效，
// let’s explore generic lifetimes of parameters and return values in the context of functions.
// 让我们在函数的上下文中探索参数和返回值的通用生命周期。

// Generic Lifetimes in Functions
// 函数中的通用生命周期
// We’ll write a function that returns the longer of two string slices.
// 我们将编写一个函数，返回两个字符串切片中较长的一个。
// This function will take two string slices and return a single string slice.
// 此函数将获取两个字符串切片并返回一个字符串切片。
// After we’ve implemented the longest function, the code in Listing 10-19 should print The longest string is abcd.
// 在我们实现了 longest 函数之后，示例 10-19 中的代码应该打印 The longest string is abcd。

// Filename: src/main.rs

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// Listing 10-19: A main function that calls the longest function to find the longer of two string slices
// 示例 10-19：调用 longest 函数以找到两个字符串切片中较长者的主函数

// Note that we want the function to take string slices, which are references, rather than strings, because we don’t want the longest function to take ownership of its parameters.
// 请注意，我们希望函数采用字符串切片，它们是引用，而不是字符串，因为我们不希望 longest 函数获得其参数的所有权。
// Refer to the “String Slices as Parameters” section in Chapter 4 for more discussion about why the parameters we use in Listing 10-19 are the ones we want.
// 请参阅第 4 章中的“字符串切片作为参数”部分，以详细讨论为什么我们在示例 10-19 中使用的参数是我们想要的。

// If we try to implement the longest function as shown in Listing 10-20, it won’t compile.
// 如果我们尝试实现清单 10-20 中所示的 longest 函数，它不会编译。

// Filename: src/main.rs

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Listing 10-20: An implementation of the longest function that returns the longer of two string slices but does not yet compile
// 示例 10-20：longest 函数的实现，它返回两个字符串切片中较长的一个但尚未编译

// Instead, we get the following error that talks about lifetimes:
// 相反，我们得到以下关于生命周期的错误：

// $ cargo run
//    Compiling chapter10 v0.1.0 (file:///projects/chapter10)
// error[E0106]: missing lifetime specifier
//  --> src/main.rs:9:33
//   |
// 9 | fn longest(x: &str, y: &str) -> &str {
//   |               ----     ----     ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// help: consider introducing a named lifetime parameter
//   |
// 9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//   |           ++++     ++          ++          ++
//
// For more information about this error, try `rustc --explain E0106`.
// error: could not compile `chapter10` due to previous error

// The help text reveals that the return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y.
// 帮助文本显示返回类型需要一个泛型生命周期参数，因为 Rust 无法判断返回的引用是指向 x 还是指向 y。
// Actually, we don’t know either, because the if block in the body of this function returns a reference to x and the else block returns a reference to y!
// 实际上，我们也不知道，因为这个函数体中的 if 块返回对 x 的引用，而 else 块返回对 y 的引用！

// When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the if case or the else case will execute.
// 当我们定义这个函数的时候，我们并不知道传递给这个函数的具体值，所以我们不知道是 if case 还是 else case 会执行。
// We also don’t know the concrete lifetimes of the references that will be passed in,
// 我们也不知道将要传入的引用的具体生命周期，
// |- so we can’t look at the scopes as we did in Listings 10-17 and 10-18 to determine whether the reference we return will always be valid.
// |- 所以我们不能像在清单 10-17 和 10-18 中那样查看范围来确定我们返回的引用是否始终有效。
// The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y relate to the lifetime of the return value.
// 借用检查器也无法确定这一点，因为它不知道 x 和 y 的生命周期与返回值的生命周期有何关系。
// To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.
// 为了修复这个错误，我们将添加定义引用之间关系的通用生命周期参数，以便借用检查器可以执行其分析。

// Lifetime Annotation Syntax
// 生命周期注解语法
// Lifetime annotations don’t change how long any of the references live.
// 生命周期注解不会改变任何引用的存活时间。
// Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
// 相反，它们描述了多个引用的生命周期相互之间的关系，而不影响生命周期。
// Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.
// 正如函数在签名指定泛型类型参数时可以接受任何类型一样，函数可以通过指定泛型生命周期参数来接受具有任何生命周期的引用。

// Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types.
// 生命周期注释有一个稍微不寻常的语法：生命周期参数的名称必须以撇号 (') 开头，并且通常都是小写且非常短，就像泛型类型一样。
// Most people use the name 'a for the first lifetime annotation.
// 大多数人使用名称 'a 作为第一个生命周期注解。
// We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s type.
// 我们将生命周期参数注释放在引用的 & 之后，使用空格将注释与引用的类型分开。

// Here are some examples: a reference to an i32 without a lifetime parameter, a reference to an i32 that has a lifetime parameter named 'a, and a mutable reference to an i32 that also has the lifetime 'a.
// 下面是一些示例：对没有生命周期参数的 i32 的引用，对具有名为 'a 的生命周期参数的 i32 的引用，以及对具有生命周期 'a 的 i32 的可变引用。

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

// One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other.
// 一个生命周期注解本身没有多大意义，因为注解是为了告诉 Rust 多个引用的通用生命周期参数如何相互关联。
// Let’s examine how the lifetime annotations relate to each other in the context of the longest function.
// 让我们检查一下生命周期注解在 longest 函数的上下文中是如何相互关联的。

// Lifetime Annotations in Function Signatures
// 函数签名中的生命周期注解
// To use lifetime annotations in function signatures, we need to declare the generic lifetime parameters inside angle brackets between the function name and the parameter list,
// 要在函数签名中使用生命周期注释，我们需要在函数名称和参数列表之间的尖括号内声明通用生命周期参数，
// |- just as we did with generic type parameters.
// |- 就像我们对泛型类型参数所做的那样。

// We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid.
// 我们希望签名表达以下约束：只要两个参数都有效，返回的引用就有效（交集）。
// This is the relationship between lifetimes of the parameters and the return value.
// 这就是参数的生命周期和返回值的关系。
// We’ll name the lifetime 'a and then add it to each reference, as shown in Listing 10-21.
// 我们将生命周期命名为 'a，然后将其添加到每个引用中，如示例 10-21 所示。

// Filename: src/main.rs

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Listing 10-21: The longest function definition specifying that all the references in the signature must have the same lifetime 'a
// 示例 10-21：longest 的函数定义指定签名中的所有引用必须具有相同的生命周期 'a

// This code should compile and produce the result we want when we use it with the main function in Listing 10-19.
// 当我们将它与示例 10-19 中的主函数一起使用时，这段代码应该编译并产生我们想要的结果。

// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a.
// 函数签名现在告诉 Rust 对于某个生命周期 'a，该函数有两个参数，这两个参数都是至少与生命周期 'a 一样长的字符串切片。
// The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.
// 函数签名还告诉 Rust 从函数返回的字符串切片将至少与生命周期 'a.
// In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments.
// 实际上，这意味着 longest 函数返回的引用的生命周期与函数参数引用的值的生命周期中的较小者相同。
// These relationships are what we want Rust to use when analyzing this code.
// 这些关系是我们希望 Rust 在分析这段代码时使用的。

// Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned.
// 请记住，当我们在此函数签名中指定生命周期参数时，我们不会更改传入或返回的任何值的生命周期。
// Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints.
// 相反，我们指定借用检查器应该拒绝任何不遵守这些约束的值。
// Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.
// 请注意，longest 函数不需要确切地知道 x 和 y 将存在多长时间，只需要知道某个范围（作用域）可以替换满足此签名的 'a。

// When annotating lifetimes in functions, the annotations go in the function signature, not in the function body.
// 在函数中注解生命周期时，注解放在函数签名中，而不是函数体中。
// The lifetime annotations become part of the contract of the function, much like the types in the signature.
// 生命周期注释成为函数契约的一部分，就像签名中的类型一样。
// Having function signatures contain the lifetime contract means the analysis the Rust compiler does can be simpler.
// 让函数签名包含生命周期契约意味着 Rust 编译器所做的分析可以更简单。
// If there’s a problem with the way a function is annotated or the way it is called, the compiler errors can point to the part of our code and the constraints more precisely.
// 如果函数的注释方式或调用方式有问题，编译器错误可以更准确地指向我们的代码部分和约束。
// If, instead, the Rust compiler made more inferences about what we intended the relationships of the lifetimes to be,
// 相反，如果 Rust 编译器对我们预期的生命周期关系做出更多推断，
// |- the compiler might only be able to point to a use of our code many steps away from the cause of the problem.
// |- 编译器可能只能指出我们代码的使用距离问题的原因有很多步骤。

// When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y.
// 当我们将具体引用传递给 longest 时，替换为 'a 的具体生命周期是 x 的范围与 y 的范围重叠的部分。
// In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y.
// 换句话说，通用生命周期 'a 将获得等于 x 和 y 生命周期中较小者的具体生命周期。
// Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.
// 因为我们用相同的生命周期参数 'a 注释了返回的引用，所以返回的引用也将在 x 和 y 的生命周期中较小的那个的长度内有效。

// Let’s look at how the lifetime annotations restrict the longest function by passing in references that have different concrete lifetimes.
// 让我们看看生命周期注释是如何通过传入具有不同具体生命周期的引用来限制最长函数的。
// Listing 10-22 is a straightforward example.
// 清单 10-22 是一个简单的示例。

// Filename: src/main.rs

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// Listing 10-22: Using the longest function with references to String values that have different concrete lifetimes
// 示例 10-22：使用 longest 函数引用具有不同具体生命周期的 String 值

// In this example, string1 is valid until the end of the outer scope, string2 is valid until the end of the inner scope,
// 在本例中，string1 在外部作用域结束前有效，string2 在内部作用域结束前有效，
// |- and result references something that is valid until the end of the inner scope.
// |- 并且 result 引用了一些在内部范围结束之前有效的东西。
// Run this code, and you’ll see that the borrow checker approves; it will compile and print The longest string is long string is long.
// 运行这段代码，你会看到借用检查器批准了； 它将编译并打印 The longest string is long string is long。

// Next, let’s try an example that shows that the lifetime of the reference in result must be the smaller lifetime of the two arguments.
// 接下来，让我们尝试一个例子，表明结果中引用的生命周期必须是两个参数中较小的生命周期。
// We’ll move the declaration of the result variable outside the inner scope but leave the assignment of the value to the result variable inside the scope with string2.
// 我们将把结果变量的声明移到内部作用域之外，但将值赋值留给作用域内的结果变量，使用 string2。
// Then we’ll move the println! that uses result to outside the inner scope, after the inner scope has ended.
// 然后我们将移动 println! 在内部范围结束后，将结果用于内部范围之外。
// The code in Listing 10-23 will not compile.
// 清单 10-23 中的代码将无法编译。

// Filename: src/main.rs

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

// Listing 10-23: Attempting to use result after string2 has gone out of scope
// 示例 10-23：在 string2 超出范围后尝试使用结果

// When we try to compile this code, we get this error:
// 当我们尝试编译这段代码时，我们得到了这个错误：

// $ cargo run
//    Compiling chapter10 v0.1.0 (file:///projects/chapter10)
// error[E0597]: `string2` does not live long enough
//  --> src/main.rs:6:44
//   |
// 6 |         result = longest(string1.as_str(), string2.as_str());
//   |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
// 7 |     }
//   |     - `string2` dropped here while still borrowed
// 8 |     println!("The longest string is {}", result);
//   |                                          ------ borrow later used here
//
// For more information about this error, try `rustc --explain E0597`.
// error: could not compile `chapter10` due to previous error

// The error shows that for result to be valid for the `println!` statement, string2 would need to be valid until the end of the outer scope.
// 错误表明，要使 `println!` 语句的结果有效，string2 需要在外部作用域结束之前有效。
// Rust knows this because we annotated the lifetimes of the function parameters and return values using the same lifetime parameter 'a.
// Rust 知道这一点，因为我们使用相同的生命周期参数 'a.

// As humans, we can look at this code and see that string1 is longer than string2 and therefore result will contain a reference to string1.
// 作为人类，我们可以查看这段代码，发现 string1 比 string2 长，因此结果将包含对 string1 的引用。
// Because string1 has not gone out of scope yet, a reference to string1 will still be valid for the `println!` statement.
// 因为 string1 还没有超出范围，所以对 string1 的引用对于 `println!` 语句仍然有效。
// However, the compiler can’t see that the reference is valid in this case.
// 但是，在这种情况下，编译器看不到引用是有效的。
// We’ve told Rust that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in.
// 我们已经告诉 Rust，longest 函数返回的引用的生命周期与传入的引用的生命周期中较小的一个相同。
// Therefore, the borrow checker disallows the code in Listing 10-23 as possibly having an invalid reference.
// 因此，借用检查器不允许示例 10-23 中的代码可能具有无效引用。

// Try designing more experiments that vary the values and lifetimes of the references passed in to the longest function and how the returned reference is used.
// 尝试设计更多实验，改变传递给 longest 函数的引用的值和生命周期，以及如何使用返回的引用。
// Make hypotheses about whether or not your experiments will pass the borrow checker before you compile; then check to see if you’re right!
// 在编译之前假设你的实验是否会通过借用检查器； 然后检查你是否正确！

// Thinking in Terms of Lifetimes
// 从生命周期的角度思考
// The way in which you need to specify lifetime parameters depends on what your function is doing.
// 你需要指定生命周期参数的方式取决于你的函数在做什么。
// For example, if we changed the implementation of the longest function to always return the first parameter rather than the longest string slice,
// 例如，如果我们将 longest 函数的实现更改为始终返回第一个参数而不是最长的字符串切片，
// |- we wouldn’t need to specify a lifetime on the y parameter.
// |- 我们不需要在 y 参数上指定生命周期。
// The following code will compile:
// 以下代码将编译：

// Filename: src/main.rs

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// We’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y,
// 我们已经为参数 x 和返回类型指定了生命周期参数 'a，但没有为参数 y 指定，
// |- because the lifetime of y does not have any relationship with the lifetime of x or the return value.
// |- 因为 y 的生命周期与 x 的生命周期或返回值没有任何关系。

// When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
// 从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期参数相匹配。
// If the reference returned does not refer to one of the parameters, it must refer to a value created within this function.
// 如果返回的引用不引用其中一个参数，则它必须引用在此函数内创建的值。
// However, this would be a dangling reference because the value will go out of scope at the end of the function.
// 然而，这将是一个悬空引用，因为该值将在函数结束时超出范围。
// Consider this attempted implementation of the longest function that won’t compile:
// 考虑这个无法编译的 longest 函数的尝试实现：

// Filename: src/main.rs

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// Here, even though we’ve specified a lifetime parameter 'a for the return type,
// 在这里，即使我们为返回类型指定了生命周期参数 'a，
// |- this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all.
// |- 此实现将无法编译，因为返回值生命周期与参数的生命周期根本无关。
// Here is the error message we get:
// 这是我们得到的错误信息：

// $ cargo run
//    Compiling chapter10 v0.1.0 (file:///projects/chapter10)
// error[E0515]: cannot return reference to local variable `result`
//   --> src/main.rs:11:5
//    |
// 11 |     result.as_str()
//    |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function
//
// For more information about this error, try `rustc --explain E0515`.
// error: could not compile `chapter10` due to previous error

// The problem is that result goes out of scope and gets cleaned up at the end of the longest function.
// 问题是结果超出范围并在 longest 函数结束时被清除。
// We’re also trying to return a reference to result from the function.
// 我们还尝试从函数返回对结果的引用。
// There is no way we can specify lifetime parameters that would change the dangling reference, and Rust won’t let us create a dangling reference.
// 我们无法指定会更改悬挂引用的生命周期参数，Rust 不会让我们创建悬挂引用。
// In this case, the best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value.
// 在这种情况下，最好的解决方法是返回一个拥有的数据类型而不是一个引用，这样调用函数就负责清理该值。

// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
// 归根结底，生命周期语法是连接函数的各种参数和返回值的生命周期。
// Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.
// 一旦它们连接起来，Rust 就有足够的信息来允许内存安全操作并禁止会创建悬空指针或以其他方式违反内存安全的操作。

// Lifetime Annotations in Struct Definitions
// 结构定义中的生命周期注解
// So far, the structs we’ve defined all hold owned types.
// 到目前为止，我们定义的结构都拥有自己的类型。
// We can define structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.
// 我们可以定义结构来保存引用，但在这种情况下，我们需要在结构定义中的每个引用上添加生命周期注释。
// Listing 10-24 has a struct named ImportantExcerpt that holds a string slice.
// 清单 10-24 有一个名为 ImportantExcerpt 的结构，它包含一个字符串切片。

// Filename: src/main.rs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael.
    // Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Listing 10-24: A struct that holds a reference, requiring a lifetime annotation
// 示例 10-24：一个包含引用的结构，需要生命周期注解

// This struct has the single field part that holds a string slice, which is a reference.
// 这个结构有一个 part 字段，它包含一个字符串切片，它是一个引用。
// As with generic data types,
// 与通用数据类型一样，
// |- we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition.
// |- 我们在结构名称后的尖括号内声明通用生命周期参数的名称，以便我们可以在结构定义的主体中使用生命周期参数。
// This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
// 这个注解意味着 ImportantExcerpt 的实例不能比它在其 part 字段中持有的引用长寿。

// The main function here creates an instance of the ImportantExcerpt struct that holds a reference to the first sentence of the String owned by the variable novel.
// 这里的 main 函数创建了一个 ImportantExcerpt 结构的实例，它包含对变量 novel 所拥有的字符串的第一句的引用。
// |- The data in novel exists before the ImportantExcerpt instance is created.
// |- novel 中的数据在 ImportantExcerpt 实例创建之前就已存在。
// In addition, novel doesn’t go out of scope until after the ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt instance is valid.
// 此外，直到 ImportantExcerpt 超出范围后，novel 才会超出范围，因此 ImportantExcerpt 实例中的引用是有效的。

// Lifetime Elision
// 生命周期省略
// You’ve learned that every reference has a lifetime and that you need to specify lifetime parameters for functions or structs that use references.
// 你已经了解到每个引用都有生命周期，你需要为使用引用的函数或结构指定生命周期参数。
// However, in Chapter 4 we had a function in Listing 4-9, shown again in Listing 10-25, that compiled without lifetime annotations.
// 但是，在第 4 章中，我们在清单 4-9 中有一个函数，再次显示在清单 10-25 中，它编译时没有使用生命周期注释。

// Filename: src/lib.rs

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Listing 10-25: A function we defined in Listing 4-9 that compiled without lifetime annotations, even though the parameter and return type are references
// 示例 10-25：我们在示例 4-9 中定义的函数在编译时没有使用生命周期注解，即使参数和返回类型是引用

// The reason this function compiles without lifetime annotations is historical: in early versions (pre-1.0) of Rust,
// 这个函数在没有生命周期注解的情况下编译的原因是历史性的：在 Rust 的早期版本（pre-1.0）中，
// |- this code wouldn’t have compiled because every reference needed an explicit lifetime.
// |- 这段代码不会被编译，因为每个引用都需要一个明确的生命周期。
// At that time, the function signature would have been written like this:
// 那时候函数签名应该是这样写的：

fn first_word<'a>(s: &'a str) -> &'a str {}

// After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same lifetime annotations over and over in particular situations.
// 在编写了大量 Rust 代码后，Rust 团队发现 Rust 程序员在特定情况下一遍又一遍地输入相同的生命周期注解。
// These situations were predictable and followed a few deterministic patterns.
// 这些情况是可以预测的，并遵循一些确定性模式。
// The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.
// 开发人员将这些模式编程到编译器的代码中，因此借用检查器可以推断这些情况下的生命周期，并且不需要显式注释。

// This piece of Rust history is relevant because it’s possible that more deterministic patterns will emerge and be added to the compiler.
// 这段 Rust 历史是相关的，因为可能会出现更多确定性模式并将其添加到编译器中。
// In the future, even fewer lifetime annotations might be required.
// 在未来，可能需要更少的生命周期注解。

// The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.
// 编程到 Rust 的引用分析中的模式称为生命周期省略规则。
// These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider,
// 这些不是程序员要遵循的规则； 它们是编译器将考虑的一组特殊情况，
// |- and if your code fits these cases, you don’t need to write the lifetimes explicitly.
// |- 如果您的代码符合这些情况，则无需显式编写生命周期。

// The elision rules don’t provide full inference.
// If Rust deterministically applies the rules but there is still ambiguity as to what lifetimes the references have,
// |- the compiler won’t guess what the lifetime of the remaining references should be.
// Instead of guessing, the compiler will give you an error that you can resolve by adding the lifetime annotations.

// 省略规则不提供完整的推理。
// 如果 Rust 确定性地应用规则，但对于引用的生命周期仍然存在歧义，
// |- 编译器不会猜测剩余引用的生命周期应该是多少。
// 而不是猜测，编译器会给你一个错误，你可以通过添加生命周期注释来解决。

// Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.
// 函数或方法参数的生命周期称为输入生命周期，返回值的生命周期称为输出生命周期。

// The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations.
// 当没有显式注释时，编译器使用三个规则来确定引用的生命周期。
// The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.
// 第一条规则适用于输入生命周期，第二条和第三条规则适用于输出生命周期。
// If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error.
// 如果编译器到达三个规则的末尾并且仍然有它无法确定生命周期的引用，编译器将停止并报错。
// These rules apply to fn definitions as well as impl blocks.
// 这些规则适用于 fn 定义和 impl 块。

// The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference.
// 第一条规则是编译器为每个引用参数分配一个生命周期参数。
// In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
// 换句话说，具有一个参数的函数获得一个生命周期参数： fn foo<'a>(x: &'a i32);
// a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
// 一个有两个参数的函数有两个独立的生命周期参数： fn foo<'a, 'b>(x: &'a i32, y: &'b i32); 等等。

// The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
// 第二条规则是，如果只有一个输入生命周期参数，则该生命周期将分配给所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。

// The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
// 第三条规则是，如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self 因为这是一个方法，
// |- the lifetime of self is assigned to all output lifetime parameters.
// |- self 的生命周期分配给所有输出生命周期参数。
// This third rule makes methods much nicer to read and write because fewer symbols are necessary.
// 第三条规则使方法更易于读写，因为需要的符号更少。

// Let’s pretend we’re the compiler.
// 假设我们是编译器。
// We’ll apply these rules to figure out the lifetimes of the references in the signature of the first_word function in Listing 10-25.
// 我们将应用这些规则来计算示例 10-25 中 first_word 函数签名中引用的生命周期。
// The signature starts without any lifetimes associated with the references:
// 签名开始时没有任何与引用关联的生命周期：

fn first_word(s: &str) -> &str {}

// Then the compiler applies the first rule, which specifies that each parameter gets its own lifetime.
// 然后编译器应用第一条规则，它指定每个参数都有自己的生命周期。
// We’ll call it 'a as usual, so now the signature is this:
// 我们像往常一样称它为 'a，所以现在签名是这样的：

fn first_word<'a>(s: &'a str) -> &str {}

// The second rule applies because there is exactly one input lifetime.
// 第二条规则适用，因为只有一个输入生命周期。
// The second rule specifies that the lifetime of the one input parameter gets assigned to the output lifetime, so the signature is now this:
// 第二条规则指定一个输入参数的生命周期被分配给输出生命周期，所以签名现在是这样的：

fn first_word<'a>(s: &'a str) -> &'a str {}

// Now all the references in this function signature have lifetimes, and the compiler can continue its analysis without needing the programmer to annotate the lifetimes in this function signature.
// 现在这个函数签名中的所有引用都有生命周期，编译器可以继续分析而不需要程序员在这个函数签名中注释生命周期。

// Let’s look at another example, this time using the longest function that had no lifetime parameters when we started working with it in Listing 10-20:
// 让我们看另一个例子，这次使用的是我们在示例 10-20 中开始使用时没有生命周期参数的 longest 函数：

fn longest(x: &str, y: &str) -> &str {}

// Let’s apply the first rule: each parameter gets its own lifetime.
// 让我们应用第一条规则：每个参数都有自己的生命周期。
// This time we have two parameters instead of one, so we have two lifetimes:
// 这次我们有两个参数而不是一个，所以我们有两个生命周期：

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}

// You can see that the second rule doesn’t apply because there is more than one input lifetime.
// 你可以看到第二条规则不适用，因为输入的生命周期不止一个。
// The third rule doesn’t apply either, because longest is a function rather than a method, so none of the parameters are self.
// 第三条规则也不适用，因为 longest 是函数而不是方法，所以参数都不是 self。
// After working through all three rules, we still haven’t figured out what the return type’s lifetime is.
// 在完成所有三个规则之后，我们仍然没有弄清楚返回类型的生命周期是多少。
// This is why we got an error trying to compile the code in Listing 10-20:
// 这就是我们在尝试编译示例 10-20 中的代码时出错的原因：
// |- the compiler worked through the lifetime elision rules but still couldn’t figure out all the lifetimes of the references in the signature.
// |- 编译器完成了生命周期省略规则，但仍然无法找出签名中引用的所有生命周期。

// Because the third rule really only applies in method signatures,
// 因为第三条规则只适用于方法签名，
// |- we’ll look at lifetimes in that context next to see why the third rule means we don’t have to annotate lifetimes in method signatures very often.
// |- 接下来我们将在该上下文中查看生命周期，以了解为什么第三条规则意味着我们不必经常在方法签名中注释生命周期。

// Lifetime Annotations in Method Definitions
// 方法定义中的生命周期注解
// When we implement methods on a struct with lifetimes, we use the same syntax as that of generic type parameters shown in Listing 10-11.
// 当我们在具有生命周期的结构上实现方法时，我们使用与示例 10-11 中所示的泛型类型参数相同的语法。
// Where we declare and use the lifetime parameters depends on whether they’re related to the struct fields or the method parameters and return values.
// 我们在哪里声明和使用生命周期参数取决于它们是与结构字段相关还是与方法参数和返回值相关。

// Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.
// 结构字段的生命周期名称始终需要在 impl 关键字之后声明，然后在结构名称之后使用，因为这些生命周期是结构类型的一部分。

// In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent.
// 在 impl 块内的方法签名中，引用可能与结构字段中引用的生命周期相关联，或者它们可能是独立的。
// In addition, the lifetime elision rules often make it so that lifetime annotations aren’t necessary in method signatures.
// 此外，生命周期省略规则通常使得方法签名中不需要生命周期注释。
// Let’s look at some examples using the struct named ImportantExcerpt that we defined in Listing 10-24.
// 让我们看一些使用示例 10-24 中定义的名为 ImportantExcerpt 的结构的示例。

// First, we’ll use a method named level whose only parameter is a reference to self and whose return value is an i32, which is not a reference to anything:
// 首先，我们将使用一个名为 level 的方法，其唯一参数是对 self 的引用，其返回值是一个 i32，它不是对任何内容的引用：

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// The lifetime parameter declaration after impl and its use after the type name are required,
// impl 之后的生命周期参数声明及其在类型名称之后的使用是必需的，
// |- but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.
// |- 但由于第一个省略规则，我们不需要注释对 self 的引用的生命周期。

// Here is an example where the third lifetime elision rule applies:
// 这是一个应用第三个生命周期省略规则的例子：

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes.
// 有两个输入生命周期，所以 Rust 应用第一个生命周期省略规则并为 &self 和 announcement 提供它们自己的生命周期。
// Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
// 那么，因为其中一个参数是&self，所以返回类型得到的是&self的生命周期，所有的生命周期都算进去了。

// The Static Lifetime
// 静态生命周期
// One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program.
// 我们需要讨论的一个特殊生命周期是 'static，它表示受影响的引用可以在程序的整个持续时间内存活。
// All string literals have the 'static lifetime, which we can annotate as follows:
// 所有的字符串字面量都有 'static 生命周期，我们可以这样注解：

let s: &'static str = "I have a static lifetime.";

// The text of this string is stored directly in the program’s binary, which is always available.
// 这个字符串的文本直接存储在程序的二进制文件中，它总是可用的。
// Therefore, the lifetime of all string literals is 'static.
// 因此，所有字符串字面量的生命周期都是 'static.

// You might see suggestions to use the 'static lifetime in error messages.
// 您可能会看到在错误消息中使用 'static 生命周期的建议。
// But before specifying 'static as the lifetime for a reference,
// 但在将 'static 指定为引用的生命周期之前，
// |- think about whether the reference you have actually lives the entire lifetime of your program or not, and whether you want it to.
// |- 想想你所拥有的引用是否真的存在于你程序的整个生命周期中，以及你是否想要它。
// Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes.
// 大多数情况下，一条错误消息提示 'static 生命周期是由于尝试创建悬空引用或可用生命周期不匹配造成的。
// In such cases, the solution is fixing those problems, not specifying the 'static lifetime.
// 在这种情况下，解决方案是解决这些问题，而不是指定 'static 生命周期。

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
// 通用类型参数、特征边界和生命周期
// Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!
// 让我们简要地看一下在一个函数中指定泛型类型参数、特征边界和生命周期的语法！

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This is the longest function from Listing 10-21 that returns the longer of two string slices.
// 这是示例 10-21 中的 longest 函数，它返回两个字符串切片中较长的一个。
// But now it has an extra parameter named ann of the generic type T, which can be filled in by any type that implements the Display trait as specified by the where clause.
// 但现在它有一个额外的参数，名为 ann，泛型类型 T，可以由实现 where 子句指定的 Display 特征的任何类型填充。
// This extra parameter will be printed using {}, which is why the Display trait bound is necessary.
// 这个额外的参数将使用 {} 打印，这就是为什么 Display trait bound 是必要的。
// Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.
// 因为生命周期是泛型类型，所以生命周期参数 'a 和泛型类型参数 T 的声明位于函数名称后尖括号内的同一个列表中。

// Summary
// 概括
// We covered a lot in this chapter!
// 我们在本章中介绍了很多内容！
// Now that you know about generic type parameters, traits and trait bounds, and generic lifetime parameters, you’re ready to write code without repetition that works in many different situations.
// 现在您已经了解了泛型类型参数、特征和特征边界以及泛型生命周期参数，您已经准备好无需重复地编写适用于许多不同情况的代码。
// Generic type parameters let you apply the code to different types.
// 通用类型参数让您可以将代码应用于不同的类型。
// Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs.
// 特征和特征边界确保即使类型是通用的，它们也将具有代码所需的行为。
// You learned how to use lifetime annotations to ensure that this flexible code won’t have any dangling references.
// 你学会了如何使用生命周期注释来确保这个灵活的代码不会有任何悬空引用。
// And all of this analysis happens at compile time, which doesn’t affect runtime performance!
// 所有这些分析都发生在编译时，这不会影响运行时性能！

// Believe it or not, there is much more to learn on the topics we discussed in this chapter: Chapter 17 discusses trait objects, which are another way to use traits.
// 信不信由你，关于我们在本章中讨论的主题，还有很多东西需要学习：第 17 章讨论了特征对象，这是使用特征的另一种方式。
// There are also more complex scenarios involving lifetime annotations that you will only need in very advanced scenarios; for those, you should read the Rust Reference.
// 还有更复杂的场景涉及到生命周期注解，只有在非常高级的场景下才会用到； 对于那些，你应该阅读 Rust 参考。
// But next, you’ll learn how to write tests in Rust so you can make sure your code is working the way it should.
// 但接下来，您将学习如何在 Rust 中编写测试，以确保您的代码按应有的方式工作。

fn main() {
    println!("Hello, world!");
}
