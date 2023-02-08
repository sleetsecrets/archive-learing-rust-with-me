fn main() {

    // Common Collections
    // Rust’s standard library includes a number of very useful data structures called collections.
    // Rust 的标准库包含许多非常有用的数据结构，称为集合。
    // Most other data types represent one specific value, but collections can contain multiple values.
    // 大多数其他数据类型表示一个特定值，但集合可以包含多个值。
    // Unlike the built-in array and tuple types, the data these collections point to is stored on the heap,
    // 与内置的数组和元组类型不同，这些集合指向的数据存储在堆上，
    // |- which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.
    // |- 这意味着数据量不需要在编译时知道，并且可以随着程序运行而增加或减少。
    // Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time.
    // 每种集合都有不同的功能和成本，选择适合您当前情况的集合是一项您会随着时间的推移而培养的技能。
    // In this chapter, we’ll discuss three collections that are used very often in Rust programs:
    // 在本章中，我们将讨论 Rust 程序中经常使用的三个集合：

    // |- A vector allows you to store a variable number of values next to each other.
    // |- 向量允许您将可变数量的值彼此相邻存储。
    // |- A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
    // |- 字符串是字符的集合。 我们之前提到过 String 类型，但在本章中我们将深入讨论它。
    // |- A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.
    // |- 哈希映射允许您将值与特定键相关联。 它是更通用的数据结构（称为映射）的特定实现。

    // To learn about the other kinds of collections provided by the standard library, see the documentation.
    // 要了解标准库提供的其他类型的集合，请参阅文档。

    // We’ll discuss how to create and update vectors, strings, and hash maps, as well as what makes each special.
    // 我们将讨论如何创建和更新向量、字符串和哈希映射，以及它们的特殊之处。

    // Storing Lists of Values with Vectors
    // 用向量存储值列表
    // The first collection type we’ll look at is Vec<T>, also known as a vector.
    // 我们要看的第一个集合类型是 Vec<T>，也称为向量。
    // Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.
    // 向量允许您在单个数据结构中存储多个值，该数据结构将所有值在内存中并排放置。
    // Vectors can only store values of the same type.
    // 向量只能存储相同类型的值。
    // They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.
    // 当你有一个项目列表时，它们很有用，例如文件中的文本行或购物车中的项目价格。

    // Creating a New Vector
    // 创建一个新向量
    // To create a new empty vector, we call the Vec::new function, as shown in Listing 8-1.
    // 要创建一个新的空向量，我们调用 Vec::new 函数，如清单 8-1 所示。

    let v: Vec<i32> = Vec::new();

    // Listing 8-1: Creating a new, empty vector to hold values of type i32
    // 示例 8-1：创建一个新的空向量来保存 i32 类型的值

    // Note that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store.
    // 请注意，我们在此处添加了类型注释。 因为我们没有向这个向量中插入任何值，Rust 不知道我们打算存储什么样的元素。
    // This is an important point. Vectors are implemented using generics; we’ll cover how to use generics with your own types in Chapter 10.
    // 这是很重要的一点。 向量是使用泛型实现的； 我们将在第 10 章介绍如何将泛型与您自己的类型一起使用。
    // For now, know that the Vec<T> type provided by the standard library can hold any type.
    // 现在，知道标准库提供的 Vec<T> 类型可以容纳任何类型。
    // When we create a vector to hold a specific type, we can specify the type within angle brackets.
    // 当我们创建一个向量来保存特定类型时，我们可以在尖括号内指定类型。
    // In Listing 8-1, we’ve told Rust that the Vec<T> in v will hold elements of the i32 type.
    // 在清单 8-1 中，我们告诉 Rust v 中的 Vec<T> 将保存 i32 类型的元素。

    // More often, you’ll create a Vec<T> with initial values and Rust will infer the type of value you want to store, so you rarely need to do this type annotation.
    // 更多时候，你会创建一个带有初始值的 Vec<T>，Rust 会推断出你想要存储的值的类型，所以你很少需要做这种类型注释。
    // Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it.
    // Rust 方便地提供了 vec! 宏，它将创建一个新的向量来保存你给它的值。
    // Listing 8-2 creates a new Vec<i32> that holds the values 1, 2, and 3.
    // 清单 8-2 创建了一个新的 Vec<i32>，它包含值 1、2 和 3。
    // The integer type is i32 because that’s the default integer type, as we discussed in the “Data Types” section of Chapter 3.
    // 整数类型是 i32，因为它是默认的整数类型，正如我们在第 3 章的“数据类型”一节中讨论的那样。

    let v = vec![1, 2, 3];

    // Listing 8-2: Creating a new vector containing values
    // 示例 8-2：创建一个包含值的新向量

    // Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t necessary.
    // 因为我们已经给出了初始的 i32 值，Rust 可以推断出 v 的类型是 Vec<i32>，并且不需要类型注释。
    // Next, we’ll look at how to modify a vector.
    // 接下来，我们将看看如何修改向量。

    // Updating a Vector
    // 更新向量
    // To create a vector and then add elements to it, we can use the push method, as shown in Listing 8-3.
    // 要创建一个向量然后向其添加元素，我们可以使用 push 方法，如清单 8-3 所示。

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Listing 8-3: Using the push method to add values to a vector
    // 示例 8-3：使用 push 方法向向量添加值

    // As with any variable, if we want to be able to change its value, we need to make it mutable using the mut keyword, as discussed in Chapter 3.
    // 与任何变量一样，如果我们希望能够更改它的值，我们需要使用 mut 关键字使其可变，如第 3 章所述。
    // The numbers we place inside are all of type i32, and Rust infers this from the data, so we don’t need the Vec<i32> annotation.
    // 我们放在里面的数字都是 i32 类型，Rust 从数据中推断出这一点，所以我们不需要 Vec<i32> 注释。

    // Reading Elements of Vectors
    // 读取向量元素
    // There are two ways to reference a value stored in a vector: via indexing or using the get method.
    // 有两种方法可以引用存储在向量中的值：通过索引或使用 get 方法。
    // In the following examples, we’ve annotated the types of the values that are returned from these functions for extra clarity.
    // 在下面的示例中，为了更加清晰，我们注释了从这些函数返回的值的类型。

    // Listing 8-4 shows both methods of accessing a value in a vector, with indexing syntax and the get method.
    // 清单 8-4 显示了访问向量中的值的两种方法，使用索引语法和 get 方法。

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Listing 8-4: Using indexing syntax or the get method to access an item in a vector
    // 示例 8-4：使用索引语法或 get 方法访问向量中的项目

    // Note a few details here. We use the index value of 2 to get the third element because vectors are indexed by number, starting at zero.
    // 注意这里的一些细节。 我们使用索引值 2 来获取第三个元素，因为向量是按数字索引的，从零开始。
    // Using & and [] gives us a reference to the element at the index value.
    // 使用 & 和 [] 为我们提供了对索引值处元素的引用。
    // When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.
    // 当我们使用 get 方法并将索引作为参数传递时，我们会得到一个可以与 match 一起使用的 Option<&T>。

    // The reason Rust provides these two ways to reference an element is so you can choose how the program behaves when you try to use an index value outside the range of existing elements.
    // Rust 提供这两种方式来引用元素的原因是，当您尝试使用现有元素范围之外的索引值时，您可以选择程序的行为方式。
    // As an example, let’s see what happens when we have a vector of five elements and then we try to access an element at index 100 with each technique, as shown in Listing 8-5.
    // 例如，让我们看看当我们有一个包含五个元素的向量时会发生什么，然后我们尝试使用每种技术访问索引 100 处的元素，如清单 8-5 所示。

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    // Listing 8-5: Attempting to access the element at index 100 in a vector containing five elements
    // 示例 8-5：尝试访问包含五个元素的向量中索引为 100 的元素

    // When we run this code, the first [] method will cause the program to panic because it references a nonexistent element.
    // 当我们运行这段代码时，第一个 [] 方法会导致程序崩溃，因为它引用了一个不存在的元素。
    // This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.
    // 如果您希望程序在尝试访问超出向量末尾的元素时崩溃，则最好使用此方法。

    // When the get method is passed an index that is outside the vector, it returns None without panicking.
    // 当向 get 方法传递一个向量外部的索引时，它会返回 None 而不会出现恐慌。
    // You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.
    // 如果在正常情况下偶尔会访问超出 vector 范围的元素，则可以使用此方法。
    // Your code will then have logic to handle having either Some(&element) or None, as discussed in Chapter 6. For example, the index could be coming from a person entering a number.
    // 然后，您的代码将具有处理 Some(&element) 或 None 的逻辑，如第 6 章所述。例如，索引可能来自输入数字的人。
    // If they accidentally enter a number that’s too large and the program gets a None value,
    // 如果他们不小心输入了一个太大的数字并且程序得到一个 None 值，
    // |- you could tell the user how many items are in the current vector and give them another chance to enter a valid value.
    // |- 你可以告诉用户当前向量中有多少项，并给他们另一次输入有效值的机会。
    // That would be more user-friendly than crashing the program due to a typo!
    // 这比由于打字错误导致程序崩溃更方便用户使用！

    // When the program has a valid reference,
    // 当程序有一个有效的引用时，
    // |- the borrow checker enforces the ownership and borrowing rules (covered in Chapter 4) to ensure this reference and any other references to the contents of the vector remain valid.
    // |- 借用检查器执行所有权和借用规则（在第 4 章中介绍）以确保此引用和对向量内容的任何其他引用保持有效。
    // Recall the rule that states you can’t have mutable and immutable references in the same scope.
    // 回想一下声明在同一范围内不能有可变和不可变引用的规则。
    // That rule applies in Listing 8-6, where we hold an immutable reference to the first element in a vector and try to add an element to the end.
    // 该规则适用于示例 8-6，其中我们持有对 vector 中第一个元素的不可变引用，并尝试将一个元素添加到末尾。
    // This program won’t work if we also try to refer to that element later in the function:
    // 如果我们稍后在函数中也尝试引用该元素，则此程序将无法运行：

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);

    // Listing 8-6: Attempting to add an element to a vector while holding a reference to an item
    // 示例 8-6：尝试将元素添加到 vector 中，同时持有对项目的引用

    // Compiling this code will result in this error:
    // 编译此代码将导致此错误：

    // $ cargo run
    // Compiling collections v0.1.0 (file:///projects/collections)
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    // --> src/main.rs:6:5
    // |
    // 4 |     let first = &v[0];
    // |                  - immutable borrow occurs here
    // 5 |
    // 6 |     v.push(6);
    // |     ^^^^^^^^^ mutable borrow occurs here
    // 7 |
    // 8 |     println!("The first element is: {}", first);
    // |                                          ----- immutable borrow later used here
    //
    // For more information about this error, try `rustc --explain E0502`.
    // error: could not compile `collections` due to previous error

    // The code in Listing 8-6 might look like it should work: why should a reference to the first element care about changes at the end of the vector?
    // 清单 8-6 中的代码看起来应该可以工作：为什么对第一个元素的引用应该关心向量末尾的变化？
    // This error is due to the way vectors work: because vectors put the values next to each other in memory,
    // 这个错误是由于向量的工作方式造成的：因为向量在内存中把值放在一起，
    // |- adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space,
    // |- if there isn’t enough room to put all the elements next to each other where the vector is currently stored.
    // |- 如果没有足够的空间将所有元素彼此相邻地放置在当前存储向量的位置，则将新元素添加到向量的末尾可能需要分配新内存并将旧元素复制到新空间。
    // In that case, the reference to the first element would be pointing to deallocated memory.
    // 在这种情况下，对第一个元素的引用将指向已释放的内存。
    // The borrowing rules prevent programs from ending up in that situation.
    // 借用规则防止程序在那种情况下结束。

    // Note: For more on the implementation details of the Vec<T> type, see “The Rustonomicon”.
    // 注意：有关 Vec<T> 类型的更多实现细节，请参阅“The Rustonomicon”。

    // Iterating over the Values in a Vector
    // 遍历 Vector 中的值
    // To access each element in a vector in turn, we would iterate through all of the elements rather than use indices to access one at a time.
    // 要依次访问向量中的每个元素，我们将遍历所有元素，而不是使用索引一次访问一个元素。
    // Listing 8-7 shows how to use a for loop to get immutable references to each element in a vector of i32 values and print them.
    // 清单 8-7 显示了如何使用 for 循环获取 i32 值向量中每个元素的不可变引用并打印它们。

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Listing 8-7: Printing each element in a vector by iterating over the elements using a for loop
    // 示例 8-7：通过使用 for 循环遍历元素来打印向量中的每个元素

    // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
    // 我们还可以迭代对可变向量中每个元素的可变引用，以便对所有元素进行更改。
    // The for loop in Listing 8-8 will add 50 to each element.
    // 清单 8-8 中的 for 循环将为每个元素加 50。

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Listing 8-8: Iterating over mutable references to elements in a vector
    // 示例 8-8：迭代向量中元素的可变引用
    // To change the value that the mutable reference refers to, we have to use the * dereference operator to get to the value in i before we can use the += operator.
    // 要更改可变引用引用的值，我们必须先使用 * 解引用运算符获取 i 中的值，然后才能使用 += 运算符。
    // We’ll talk more about the dereference operator in the “Following the Pointer to the Value with the Dereference Operator” section of Chapter 15.
    // 我们将在第 15 章的“使用解引用运算符跟随指向值的指针”部分详细讨论解引用运算符。

    // Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker's rules.
    // 由于借用检查器的规则，迭代一个向量，无论是不变的还是可变的，都是安全的。
    // If we attempted to insert or remove items in the for loop bodies in Listing 8-7 and Listing 8-8, we would get a compiler error similar to the one we got with the code in Listing 8-6.
    // 如果我们试图在清单 8-7 和清单 8-8 中的 for 循环体中插入或删除项目，我们将得到一个类似于清单 8-6 中的代码的编译器错误。
    // The reference to the vector that the for loop holds prevents simultaneous modification of the whole vector.
    // 对 for 循环持有的向量的引用可防止同时修改整个向量。

    // Using an Enum to Store Multiple Types
    // 使用枚举存储多种类型
    // Vectors can only store values that are the same type.
    // 向量只能存储相同类型的值。
    // This can be inconvenient; there are definitely use cases for needing to store a list of items of different types.
    // 这可能很不方便； 肯定有需要存储不同类型项目列表的用例。
    // Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!
    // 幸运的是，一个枚举的变体定义在同一个枚举类型下，所以当我们需要一个类型来表示不同类型的元素时，我们可以定义并使用一个枚举！

    // For example, say we want to get values from a row in a spreadsheet in which some of the columns in the row contain integers, some floating-point numbers, and some strings.
    // 例如，假设我们要从电子表格的一行中获取值，其中行中的某些列包含整数、一些浮点数和一些字符串。
    // We can define an enum whose variants will hold the different value types, and all the enum variants will be considered the same type: that of the enum.
    // 我们可以定义一个枚举，其变体将包含不同的值类型，并且所有枚举变体将被视为同一类型：枚举的类型。
    // Then we can create a vector to hold that enum and so, ultimately, holds different types.
    // 然后我们可以创建一个向量来保存该枚举，因此最终保存不同的类型。
    // We’ve demonstrated this in Listing 8-9.
    // 我们已经在示例 8-9 中演示了这一点。

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Listing 8-9: Defining an enum to store values of different types in one vector
    // 示例 8-9：定义一个枚举以将不同类型的值存储在一个向量中

    // Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element.
    // Rust 需要在编译时知道 vector 中的类型，因此它确切地知道存储每个元素需要堆上的多少内存。
    // We must also be explicit about what types are allowed in this vector.
    // 我们还必须明确说明此向量中允许使用哪些类型。
    // If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector.
    // 如果 Rust 允许向量包含任何类型，那么一种或多种类型可能会导致对向量元素执行的操作出错。
    // Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled, as discussed in Chapter 6.
    // 使用枚举加上匹配表达式意味着 Rust 将确保在编译时处理所有可能的情况，如第 6 章所述。

    // If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work.
    // 如果您不知道程序将在运行时获取并存储在向量中的详尽类型集，则枚举技术将不起作用。
    // Instead, you can use a trait object, which we’ll cover in Chapter 17.
    // 相反，您可以使用特征对象，我们将在第 17 章中介绍。

    // Now that we’ve discussed some of the most common ways to use vectors, be sure to review the API documentation for all the many useful methods defined on Vec<T> by the standard library.
    // 既然我们已经讨论了使用向量的一些最常见方法，请务必查看 API 文档以了解标准库在 Vec<T> 上定义的所有许多有用方法。
    // For example, in addition to push, a pop method removes and returns the last element.
    // 例如，除了 push 之外，还有一个 pop 方法移除并返回最后一个元素。

    // Dropping a Vector Drops Its Elements
    // 删除一个向量会删除它的元素
    // Like any other struct, a vector is freed when it goes out of scope, as annotated in Listing 8-10.
    // 与任何其他结构一样，向量在超出范围时会被释放，如清单 8-10 中所示。

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    // Listing 8-10: Showing where the vector and its elements are dropped
    // 示例 8-10：显示向量及其元素的放置位置

    // When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up.
    // 当 vector 被丢弃时，它的所有内容也被丢弃，这意味着它持有的整数将被清理。
    // The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.
    // 借用检查器确保仅在向量本身有效时才使用对向量内容的任何引用。

    // Let’s move on to the next collection type: String!
    // 让我们继续下一个集合类型：String！

    // Storing UTF-8 Encoded Text with Strings
    // 使用字符串存储 UTF-8 编码的文本
    // We talked about strings in Chapter 4, but we’ll look at them in more depth now.
    // 我们在第 4 章中讨论了字符串，但现在我们将更深入地研究它们。
    // New Rustaceans commonly get stuck on strings for a combination of three reasons:
    // 新的 Rustaceans 通常会因为三个原因的组合而卡在字符串上：
    // |- Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8.
    // |- Rust 暴露可能错误的倾向，字符串是一种比许多程序员认为的更复杂的数据结构，以及 UTF-8。
    // These factors combine in a way that can seem difficult when you’re coming from other programming languages.
    // 当您来自其他编程语言时，这些因素以一种看似困难的方式结合在一起。

    // We discuss strings in the context of collections because strings are implemented as a collection of bytes,
    // 我们在集合的上下文中讨论字符串，因为字符串是作为字节集合实现的，
    // |- plus some methods to provide useful functionality when those bytes are interpreted as text.
    // |- 加上一些方法以在这些字节被解释为文本时提供有用的功能。
    // In this section, we’ll talk about the operations on String that every collection type has, such as creating, updating, and reading.
    // 在这一节中，我们将讨论每个集合类型对 String 的操作，例如创建、更新和读取。
    // We’ll also discuss the ways in which String is different from the other collections,
    // 我们还将讨论 String 与其他集合的不同之处，
    // |- namely how indexing into a String is complicated by the differences between how people and computers interpret String data.
    // |- 即索引到 String 是如何因人和计算机解释 String 数据的方式之间的差异而变得复杂的。

    // What Is a String?
    // 什么是字符串？
    // We’ll first define what we mean by the term string. Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str.
    // 我们将首先定义术语字符串的含义。 Rust 在核心语言中只有一种字符串类型，即通常以其借用形式 &str 形式出现的字符串切片 str。
    // In Chapter 4, we talked about string slices, which are references to some UTF-8 encoded string data stored elsewhere.
    // 在第 4 章中，我们谈到了字符串切片，它是对存储在别处的一些 UTF-8 编码字符串数据的引用。
    // String literals, for example, are stored in the program’s binary and are therefore string slices.
    // 例如，字符串文字存储在程序的二进制文件中，因此是字符串切片。

    // The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.
    // String 类型由 Rust 的标准库提供，而不是编码到核心语言中，是一种可增长的、可变的、自有的、UTF-8 编码的字符串类型。
    // When Rustaceans refer to “strings” in Rust, they might be referring to either the String or the string slice &str types, not just one of those types.
    // 当 Rustaceans 在 Rust 中提到“字符串”时，他们可能指的是 String 或字符串切片 &str 类型，而不仅仅是其中一种类型。
    // Although this section is largely about String, both types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.
    // 虽然这部分主要是关于 String 的，但这两种类型在 Rust 的标准库中被大量使用，并且 String 和字符串切片都是 UTF-8 编码的。

    // Creating a New String
    // 创建一个新字符串
    // Many of the same operations available with Vec<T> are available with String as well,
    // 许多可用于 Vec<T> 的相同操作也可用于 String，
    // |- because String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
    // |- 因为 String 实际上是作为字节向量的包装器实现的，具有一些额外的保证、限制和功能。
    // An example of a function that works the same way with Vec<T> and String is the new function to create an instance, shown in Listing 8-11.
    // 与 Vec<T> 和 String 以相同方式工作的函数示例是创建实例的新函数，如清单 8-11 所示。

    let mut s = String::new();

    // Listing 8-11: Creating a new, empty String
    // 示例 8-11：创建一个新的空字符串

    // This line creates a new empty string called s, which we can then load data into.
    // 此行创建一个名为 s 的新空字符串，然后我们可以将数据加载到其中。
    // Often, we’ll have some initial data that we want to start the string with.
    // 通常，我们会有一些初始数据来作为字符串的开头。
    // For that, we use the to_string method, which is available on any type that implements the Display trait, as string literals do. Listing 8-12 shows two examples.
    // 为此，我们使用 to_string 方法，该方法可用于实现 Display 特性的任何类型，就像字符串文字一样。 清单 8-12 显示了两个示例。

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    // 该方法也直接作用于字面量：
    let s = "initial contents".to_string();

    // Listing 8-12: Using the to_string method to create a String from a string literal
    // 示例 8-12：使用 to_string 方法从字符串文字创建 String

    // This code creates a string containing initial contents.
    // 此代码创建一个包含初始内容的字符串。

    // We can also use the function String::from to create a String from a string literal.
    // 我们还可以使用函数 String::from 从字符串文字创建一个 String。
    // The code in Listing 8-13 is equivalent to the code from Listing 8-12 that uses to_string.
    // 清单 8-13 中的代码等同于清单 8-12 中使用 to_string 的代码。

    let s = String::from("initial contents");

    // Listing 8-13: Using the String::from function to create a String from a string literal
    // 示例 8-13：使用 String::from 函数从字符串文字创建一个 String

    // Because strings are used for so many things, we can use many different generic APIs for strings, providing us with a lot of options.
    // 因为字符串有很多用途，所以我们可以对字符串使用很多不同的通用 API，为我们提供了很多选择。
    // Some of them can seem redundant, but they all have their place!
    // 其中一些看似多余，但它们都有其用武之地！
    // In this case, String::from and to_string do the same thing, so which you choose is a matter of style and readability.
    // 在这种情况下，String::from 和 to_string 做同样的事情，所以你选择哪个是风格和可读性的问题。

    // Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them, as shown in Listing 8-14.
    // 请记住，字符串是 UTF-8 编码的，因此我们可以在其中包含任何正确编码的数据，如清单 8-14 所示。

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Listing 8-14: Storing greetings in different languages in strings
    // 示例 8-14：在字符串中存储不同语言的问候语

    // All of these are valid String values.
    // 所有这些都是有效的字符串值。

    // Updating a String
    // 更新一个字符串
    // A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you push more data into it.
    // String 的大小可以增加，它的内容也可以改变，就像 Vec<T> 的内容一样，如果你向它压入更多的数据。
    // In addition, you can conveniently use the + operator or the format! macro to concatenate String values.
    // 另外，可以方便的使用+操作符或者 format! 用于连接字符串值的宏。

    // Appending to a String with push_str and push
    // 使用 push_str 和 push 附加到一个字符串
    // We can grow a String by using the push_str method to append a string slice, as shown in Listing 8-15.
    // 我们可以通过使用 push_str 方法附加一个字符串切片来增长一个字符串，如清单 8-15 所示。

    let mut s = String::from("foo");
    s.push_str("bar");

    // Listing 8-15: Appending a string slice to a String using the push_str method
    // 示例 8-15：使用 push_str 方法将字符串切片附加到 String

    // After these two lines, s will contain foobar.
    // 在这两行之后，s 将包含 foobar。
    // The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter.
    // push_str 方法接受一个字符串切片，因为我们不一定要获取参数的所有权。
    // For example, in the code in Listing 8-16, we want to be able to use s2 after appending its contents to s1.
    // 例如，在清单 8-16 的代码中，我们希望能够在将其内容附加到 s1 之后使用 s2。

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // Listing 8-16: Using a string slice after appending its contents to a String
    // 示例 8-16：在将其内容附加到 String 后使用字符串切片

    // If the push_str method took ownership of s2, we wouldn’t be able to print its value on the last line. However, this code works as we’d expect!
    // 如果 push_str 方法获得了 s2 的所有权，我们将无法在最后一行打印它的值。 然而，这段代码正如我们所期望的那样工作！

    // The push method takes a single character as a parameter and adds it to the String.
    // push 方法将单个字符作为参数并将其添加到 String 中。
    // Listing 8-17 adds the letter “l” to a String using the push method.
    // 示例 8-17 使用 push 方法将字母“l”添加到一个字符串中。

    let mut s = String::from("lo");
    s.push('l');

    // Listing 8-17: Adding one character to a String value using push
    // 示例 8-17：使用 push 向 String 值添加一个字符

    // As a result, s will contain lol.
    // 因此，s 将包含 lol。

    // Concatenation with the + Operator or the format! Macro
    // 使用 + 运算符或 format! 宏连接
    // Often, you’ll want to combine two existing strings. One way to do so is to use the + operator, as shown in Listing 8-18.
    // 通常，你会想要组合两个现有的字符串。 一种方法是使用 + 运算符，如清单 8-18 所示。

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       // 注意 s1 已移至此处，不能再使用

    // Listing 8-18: Using the + operator to combine two String values into a new String value
    // 示例 8-18：使用 + 运算符将两个 String 值组合成一个新的 String 值

    // The string s3 will contain Hello, world!.
    // 字符串 s3 将包含 Hello, world!。
    // The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator.
    // 添加后 s1 不再有效的原因，以及我们使用对 s2 的引用的原因，与我们使用 + 运算符时调用的方法的签名有关。
    // The + operator uses the add method, whose signature looks something like this:
    // + 运算符使用 add 方法，其签名如下所示：

    fn add(self, s: &str) -> String {}

    // In the standard library, you'll see add defined using generics and associated types.
    // 在标准库中，您将看到使用泛型和关联类型定义的添加。
    // Here, we’ve substituted in concrete types, which is what happens when we call this method with String values.
    // 在这里，我们用具体类型进行了替换，这就是我们用字符串值调用此方法时发生的情况。
    // We’ll discuss generics in Chapter 10. This signature gives us the clues we need to understand the tricky bits of the + operator.
    // 我们将在第 10 章讨论泛型。这个签名为我们提供了理解 + 运算符的棘手部分所需的线索。

    // First, s2 has an &, meaning that we’re adding a reference of the second string to the first string.
    // 首先，s2 有一个 &，这意味着我们将第二个字符串的引用添加到第一个字符串。
    // This is because of the s parameter in the add function: we can only add a &str to a String; we can’t add two String values together.
    // 这是因为 add 函数中的 s 参数：我们只能将 &str 添加到 String; 我们不能将两个字符串值加在一起。
    // But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add.
    // 但是等等——&s2 的类型是 &String，而不是 &str，正如在要添加的第二个参数中指定的那样。
    // So why does Listing 8-18 compile?
    // 那么为什么示例 8-18 可以编译呢？

    // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str.
    // 我们能够在对 add 的调用中使用 &s2 的原因是编译器可以将 &String 参数强制转换为 &str。
    // When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..].
    // 当我们调用 add 方法时，Rust 使用 deref 强制转换，在这里将 &s2 变成 &s2[..]。
    // We’ll discuss deref coercion in more depth in Chapter 15.
    // Because add does not take ownership of the s parameter, s2 will still be a valid String after this operation.
    // 因为 add 没有取得 s 参数的所有权，所以 s2 在这个操作之后仍然是一个有效的 String。

    // Second, we can see in the signature that add takes ownership of self, because self does not have an &.
    // 其次，我们可以在签名中看到 add 取得了 self 的所有权，因为 self 没有 &。
    // This means s1 in Listing 8-18 will be moved into the add call and will no longer be valid after that.
    // 这意味着示例 8-18 中的 s1 将被移入 add 调用中，之后将不再有效。
    // So although let s3 = s1 + &s2; looks like it will copy both strings and create a new one,
    // 所以尽管让 s3 = s1 + &s2; 看起来它会复制两个字符串并创建一个新字符串，
    // |- this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result.
    // |- 该语句实际上获取了 s1 的所有权，附加了 s2 内容的副本，然后返回结果的所有权。
    // In other words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.
    // 换句话说，看起来它正在制作很多副本，但实际上并没有； 这个实现比复制更高效。

    // If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:
    // 如果我们需要连接多个字符串，+ 运算符的行为会变得笨拙：

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // At this point, s will be tic-tac-toe. With all of the + and " characters, it’s difficult to see what’s going on.
    // 此时，s 将是tic-tac-toe。 对于所有 + 和 " 字符，很难看出发生了什么。
    // For more complicated string combining, we can instead use the format! macro:
    // 对于更复杂的字符串组合，我们可以改用 format! 宏：

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // This code also sets s to tic-tac-toe.
    // 此代码还将 s 设置为tic-tac-toe。
    // The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents.
    // format! 宏的工作方式类似于 println!，但它不是将输出打印到屏幕上，而是返回一个包含内容的字符串。
    // The version of the code using format! is much easier to read, and the code generated by the format! macro uses references so that this call doesn’t take ownership of any of its parameters.
    // 使用 format! 的代码版本 更容易阅读，并且由 format! macro 使用引用，因此此调用不会取得其任何参数的所有权。

    // Indexing into Strings
    // 索引到字符串
    // In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation.
    // 在许多其他编程语言中，通过索引引用来访问字符串中的单个字符是一种有效且常见的操作。
    // However, if you try to access parts of a String using indexing syntax in Rust, you’ll get an error.
    // 但是，如果您尝试在 Rust 中使用索引语法访问字符串的一部分，您将收到错误。
    // Consider the invalid code in Listing 8-19.
    // 考虑清单 8-19 中的无效代码。

    let s1 = String::from("hello");
    let h = s1[0];

    // Listing 8-19: Attempting to use indexing syntax with a String
    // 示例 8-19：尝试对 String 使用索引语法

    // This code will result in the following error:
    // 此代码将导致以下错误：

    // $ cargo run
    // Compiling collections v0.1.0 (file:///projects/collections)
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    // --> src/main.rs:3:13
    // |
    // 3 |     let h = s1[0];
    // |             ^^^^^ `String` cannot be indexed by `{integer}`
    // |
    // = help: the trait `Index<{integer}>` is not implemented for `String`
    // = help: the following other types implement trait `Index<Idx>`:
    //             <String as Index<RangeFrom<usize>>>
    //             <String as Index<RangeFull>>
    //             <String as Index<RangeInclusive<usize>>>
    //             <String as Index<RangeTo<usize>>>
    //             <String as Index<RangeToInclusive<usize>>>
    //             <String as Index<std::ops::Range<usize>>>
    //             <str as Index<I>>
    //
    // For more information about this error, try `rustc --explain E0277`.
    // error: could not compile `collections` due to previous error

    // The error and the note tell the story: Rust strings don’t support indexing.
    // 错误和注释说明了这个问题：Rust 字符串不支持索引。
    // But why not? To answer that question, we need to discuss how Rust stores strings in memory.
    // 但为什么不呢？ 要回答这个问题，我们需要讨论 Rust 如何在内存中存储字符串。

    // Internal Representation
    // 内部表示
    // A String is a wrapper over a Vec<u8>.
    // String 是 Vec<u8> 的包装器。
    // Let’s look at some of our properly encoded UTF-8 example strings from Listing 8-14.
    // 让我们看看示例 8-14 中一些正确编码的 UTF-8 示例字符串。
    // First, this one:
    // 首先，这个：

    let hello = String::from("Hola");

    // In this case, len will be 4, which means the vector storing the string “Hola” is 4 bytes long.
    // 在这种情况下，len 将为 4，这意味着存储字符串“Hola”的向量有 4 个字节长。
    // Each of these letters takes 1 byte when encoded in UTF-8. The following line, however, may surprise you.
    // 当以 UTF-8 编码时，这些字母中的每一个都占用 1 个字节。 然而，下面一行可能会让您大吃一惊。
    // |- (Note that this string begins with the capital Cyrillic letter Ze, not the Arabic number 3.)
    // |-（请注意，此字符串以大写西里尔字母 Ze 开头，而不是阿拉伯数字 3。）

    let hello = String::from("Здравствуйте");

    // Asked how long the string is, you might say 12. In fact, Rust’s answer is 24:
    // 问字符串有多长，你可能会说 12。事实上，Rust 的答案是 24：
    // |- that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value in that string takes 2 bytes of storage.
    // |- 这是用 UTF-8 编码“Здравствуйте”所需的字节数，因为该字符串中的每个 Unicode 标量值占用 2 个字节的存储空间。
    // Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value.
    // 因此，字符串字节的索引并不总是与有效的 Unicode 标量值相关联。
    // To demonstrate, consider this invalid Rust code:
    // 为了演示，考虑这个无效的 Rust 代码：

    let hello = "Здравствуйте";
    let answer = &hello[0];

    // You already know that answer will not be З, the first letter.
    // 你已经知道答案不会是 З，第一个字母。
    // When encoded in UTF-8, the first byte of З is 208 and the second is 151, so it would seem that answer should in fact be 208, but 208 is not a valid character on its own.
    // 当以 UTF-8 编码时，З 的第一个字节是 208，第二个字节是 151，所以看起来 answer 实际上应该是 208，但 208 本身并不是一个有效字符。
    // Returning 208 is likely not what a user would want if they asked for the first letter of this string; however, that’s the only data that Rust has at byte index 0.
    // 返回 208 可能不是用户想要的，如果他们要求这个字符串的第一个字母； 然而，这是 Rust 在字节索引 0 处唯一的数据。
    // Users generally don’t want the byte value returned, even if the string contains only Latin letters: if &"hello"[0] were valid code that returned the byte value, it would return 104, not h.
    // 用户通常不希望返回字节值，即使字符串只包含拉丁字母：如果 &"hello"[0] 是返回字节值的有效代码，它将返回 104，而不是 h。

    // The answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately,
    // 答案是，为了避免返回意外值并导致可能无法立即发现的错误，
    // |- Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.
    // |- Rust 根本不编译这段代码，防止在开发过程的早期出现误解。

    // Bytes and Scalar Values and Grapheme Clusters! Oh My!
    // 字节和标量值以及字素簇！ 天啊！
    // Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters
    // 关于 UTF-8 的另一点是，从 Rust 的角度来看，实际上有三种相关的方式来看待字符串：作为字节、标量值和字素簇
    // |- (the closest thing to what we would call letters).
    // |-（最接近我们所说的字母）。

    // If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:
    // 如果我们看一下用梵文脚本编写的印地语单词“नमस्ते”，它存储为一个 u8 值的向量，如下所示：

    [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224,
     165, 135]

    // That’s 18 bytes and is how computers ultimately store this data.
    // 那是 18 个字节，这就是计算机最终存储这些数据的方式。
    // If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:
    // 如果我们将它们视为 Unicode 标量值，即 Rust 的 char 类型，这些字节如下所示：

    ['न', 'म', 'स', '्', 'त', 'े']

    // There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own.
    // 这里有六个字符值，但第四个和第六个不是字母：它们是变音符号，本身没有意义。
    // Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:
    // 最后，如果我们将它们视为字素簇，我们会得到人们所说的组成印地语单词的四个字母：

    ["न", "म", "स्", "ते"]

    // Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs,
    // Rust 提供了不同的方式来解释计算机存储的原始字符串数据，以便每个程序可以选择它需要的解释，
    // |- no matter what human language the data is in.
    // |- 无论数据使用何种人类语言。

    // A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)).
    // Rust 不允许我们索引一个字符串来获取一个字符的最后一个原因是索引操作应该总是花费常数时间 (O(1))。
    // But it isn’t possible to guarantee that performance with a String,
    // 但是不能保证字符串的性能，
    // |- because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.
    // |- 因为 Rust 必须遍历从头到索引的内容以确定有多少个有效字符。

    // Slicing Strings
    // 切片字符串
    // Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be:
    // 对字符串进行索引通常不是一个好主意，因为不清楚字符串索引操作的返回类型应该是什么：
    // |- a byte value, a character, a grapheme cluster, or a string slice.
    // |- 字节值、字符、字素簇或字符串切片。
    // If you really need to use indices to create string slices, therefore, Rust asks you to be more specific.
    // 如果你真的需要使用索引来创建字符串切片，那么 Rust 要求你更加具体。

    // Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:
    // 您可以使用带有范围的 [] 来创建包含特定字节的字符串切片，而不是使用带有单个数字的 [] 进行索引：

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    // Here, s will be a &str that contains the first 4 bytes of the string.
    // 在这里，s 将是一个 &str，它包含字符串的前 4 个字节。
    // Earlier, we mentioned that each of these characters was 2 bytes, which means s will be Зд.
    // 早些时候，我们提到这些字符中的每一个都是 2 个字节，这意味着 s 将是 Зд。

    // If we were to try to slice only part of a character’s bytes with something like &hello[0..1],
    // 如果我们试图用 &hello[0..1] 之类的东西只分割字符的一部分字节，
    // Rust would panic at runtime in the same way as if an invalid index were accessed in a vector:
    // Rust 会在运行时出现恐慌，就像在向量中访问无效索引一样：

    // $ cargo run
    //     Compiling collections v0.1.0 (file:///projects/collections)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.43s
    //     Running `target/debug/collections`
    // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', library/core/src/str/mod.rs:127:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // You should use ranges to create string slices with caution, because doing so can crash your program.
    // 你应该谨慎地使用范围来创建字符串切片，因为这样做会使你的程序崩溃。

    // Methods for Iterating Over Strings
    // 迭代字符串的方法
    // The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.
    // 对字符串片段进行操作的最佳方法是明确说明您需要字符还是字节。
    // For individual Unicode scalar values, use the chars method.
    // 对于单个 Unicode 标量值，使用 chars 方法。
    // Calling chars on “Зд” separates out and returns two values of type char, and you can iterate over the result to access each element:
    // 在“Зд”上调用 chars 分离并返回两个 char 类型的值，您可以迭代结果以访问每个元素：

    for c in "Зд".chars() {
        println!("{}", c);
    }

    // This code will print the following:
    // 此代码将打印以下内容：

    // З
    // д

    // Alternatively, the bytes method returns each raw byte, which might be appropriate for your domain:
    // 或者，bytes 方法返回每个原始字节，这可能适合您的域：

    for b in "Зд".bytes() {
        println!("{}", b);
    }

    // This code will print the four bytes that make up this string:
    // 此代码将打印构成此字符串的四个字节：

    // 208
    // 151
    // 208
    // 180

    // But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.
    // 但一定要记住，有效的 Unicode 标量值可能由超过 1 个字节组成。

    // Getting grapheme clusters from strings as with the Devanagari script is complex, so this functionality is not provided by the standard library.
    // 与梵文脚本一样，从字符串中获取字素簇很复杂，因此标准库不提供此功能。
    // Crates are available on crates.io if this is the functionality you need.
    // 如果这是你需要的功能，crates 可以在 crates.io 上找到。

    // Strings Are Not So Simple
    // 字符串没那么简单
    // To summarize, strings are complicated. Different programming languages make different choices about how to present this complexity to the programmer.
    // 总而言之，字符串很复杂。 不同的编程语言对如何向程序员呈现这种复杂性做出不同的选择。
    // Rust has chosen to make the correct handling of String data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data upfront.
    // Rust 选择将正确处理 String 数据作为所有 Rust 程序的默认行为，这意味着程序员必须更多地考虑预先处理 UTF-8 数据。
    // This trade-off exposes more of the complexity of strings than is apparent in other programming languages,
    // 与其他编程语言相比，这种权衡暴露了字符串的更多复杂性，
    // |- but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.
    // |- 但它可以防止您在开发生命周期的后期处理涉及非 ASCII 字符的错误。

    // The good news is that the standard library offers a lot of functionality built off the String and &str types to help handle these complex situations correctly.
    // 好消息是标准库提供了许多基于 String 和 &str 类型构建的功能，以帮助正确处理这些复杂情况。
    // Be sure to check out the documentation for useful methods like contains for searching in a string and replace for substituting parts of a string with another string.
    // 请务必查看文档以了解有用的方法，例如用于在字符串中搜索的 contains 以及用于将字符串的一部分替换为另一个字符串的 replace。

    // Let’s switch to something a bit less complex: hash maps!
    // 让我们切换到不那么复杂的东西：hash map！

    // Storing Keys with Associated Values in Hash Maps
    // 在哈希映射中存储具有关联值的键
    // The last of our common collections is the hash map.
    // 我们最后一个常见的集合是哈希映射。
    // The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function,
    // 类型 HashMap<K, V> 使用散列函数存储类型 K 的键到类型 V 的值的映射，
    // which determines how it places these keys and values into memory.
    // 这决定了它如何将这些键和值放入内存中。
    // Many programming languages support this kind of data structure,
    // 许多编程语言都支持这种数据结构，
    // ｜- but they often use a different name, such as hash, map, object, hash table, dictionary, or associative array, just to name a few.
    // ｜- 但他们经常使用不同的名称，例如 hash、map、object、hash table、dictionary 或 associative array，仅举几例。

    // Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.
    // 当您不想像使用向量那样使用索引来查找数据，而是使用可以是任何类型的键来查找数据时，哈希映射很有用。
    // For example, in a game, you could keep track of each team’s score in a hash map in which each key is a team’s name and the values are each team’s score.
    // 例如，在游戏中，您可以在哈希映射中跟踪每个团队的分数，其中每个键是团队的名称，值是每个团队的分数。
    // Given a team name, you can retrieve its score.
    // 给定一个团队名称，你可以检索它的分数。

    // We’ll go over the basic API of hash maps in this section, but many more goodies are hiding in the functions defined on HashMap<K, V> by the standard library.
    // 我们将在本节中介绍哈希映射的基本 API，但标准库在 HashMap<K, V> 上定义的函数中隐藏了更多好东西。
    // As always, check the standard library documentation for more information.
    // 一如既往，查看标准库文档以获取更多信息。

    // Creating a New Hash Map
    // 创建一个新的哈希映射
    // One way to create an empty hash map is using new and adding elements with insert.
    // 创建空哈希映射的一种方法是使用 new 并使用 insert 添加元素。
    // In Listing 8-20, we’re keeping track of the scores of two teams whose names are Blue and Yellow.
    // 在示例 8-20 中，我们跟踪了两支名为 Blue 和 Yellow 的球队的得分。
    // The Blue team starts with 10 points, and the Yellow team starts with 50.
    // 蓝队以 10 分开始，黄队以 50 分开始。

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Listing 8-20: Creating a new hash map and inserting some keys and values
    // 示例 8-20：创建一个新的哈希映射并插入一些键和值

    // Note that we need to first use the HashMap from the collections portion of the standard library.
    // 请注意，我们需要首先使用标准库集合部分的 HashMap。
    // Of our three common collections, this one is the least often used, so it’s not included in the features brought into scope automatically in the prelude.
    // 在我们的三个常用集合中，这个是最不常用的，因此它不包含在前奏（prelude）中自动引入范围的功能中。
    // Hash maps also have less support from the standard library; there’s no built-in macro to construct them, for example.
    // 哈希映射在标准库中的支持也较少； 例如，没有内置宏来构造它们。

    // Just like vectors, hash maps store their data on the heap.
    // 就像向量一样，哈希映射将它们的数据存储在堆上。
    // This HashMap has keys of type String and values of type i32.
    // 这个 HashMap 有 String 类型的键和 i32 类型的值。
    // Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.
    // 与向量一样，哈希映射是同质的：所有键必须彼此具有相同类型，并且所有值必须具有相同类型。

    // Accessing Values in a Hash Map
    // 访问哈希映射中的值
    // We can get a value out of the hash map by providing its key to the get method, as shown in Listing 8-21.
    // 我们可以通过将其键提供给 get 方法来从哈希映射中获取一个值，如清单 8-21 所示。

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Listing 8-21: Accessing the score for the Blue team stored in the hash map
    // 示例 8-21：访问存储在哈希映射中的蓝队得分

    // Here, score will have the value that’s associated with the Blue team, and the result will be 10.
    // 此处，score 将具有与蓝队关联的值，结果将为 10。
    // The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None.
    // get 方法返回一个 Option<&V>; 如果哈希映射中该键没有值，则 get 将返回 None。
    // This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the key.
    // 此程序通过调用 copied 来处理 Option 以获取 Option<i32> 而不是 Option<&i32>，然后 unwrap_or 将 score 设置为零（如果 scores 没有键的条目）。

    // We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a for loop:
    // 我们可以使用 for 循环以与处理向量类似的方式遍历哈希映射中的每个键/值对：

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // This code will print each pair in an arbitrary order:
    // 此代码将以任意顺序打印每一对：

    // Yellow: 50
    // Blue: 10

    // Hash Maps and Ownership
    // 哈希映射和所有权
    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // 对于实现 Copy 特性的类型，比如 i32，值被复制到哈希映射中。
    // For owned values like String, the values will be moved and the hash map will be the owner of those values, as demonstrated in Listing 8-22.
    // 对于像 String 这样的拥有值，这些值将被移动并且哈希映射将成为这些值的所有者，如清单 8-22 所示。

    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // field_name 和 field_value 在这一点上是无效的，尝试使用它们，看看你得到什么编译器错误！

    // Listing 8-22: Showing that keys and values are owned by the hash map once they’re inserted
    // 示例 8-22：显示键和值在插入后归哈希映射所有

    // We aren’t able to use the variables field_name and field_value after they’ve been moved into the hash map with the call to insert.
    // 在调用 insert 将变量 field_name 和 field_value 移入哈希映射后，我们无法使用它们。

    // If we insert references to values into the hash map, the values won’t be moved into the hash map.
    // 如果我们将对值的引用插入到哈希映射中，这些值将不会被移动到哈希映射中。
    // The values that the references point to must be valid for at least as long as the hash map is valid.
    // 引用指向的值必须至少在哈希映射有效时有效。
    // We’ll talk more about these issues in the “Validating References with Lifetimes” section in Chapter 10.
    // 我们将在第 10 章的“使用生命周期验证引用”部分详细讨论这些问题。

    // Updating a Hash Map
    // 更新哈希映射
    // Although the number of key and value pairs is growable, each unique key can only have one value associated with it at a time (but not vice versa:
    // 尽管键和值对的数量是可以增长的，但是每个唯一键一次只能有一个与之关联的值（反之亦然：
    // ｜- for example, both the Blue team and the Yellow team could have value 10 stored in the scores hash map).
    // ｜- 例如，蓝队和黄队都可以将值 10 存储在分数哈希映射中）。

    // When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned.
    // 当你想改变哈希映射中的数据时，你必须决定如何处理键已经分配了值的情况。
    // You could replace the old value with the new value, completely disregarding the old value.
    // 你可以用新值替换旧值，完全忽略旧值。
    // You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value.
    // 您可以保留旧值并忽略新值，只有在键还没有值时才添加新值。
    // Or you could combine the old value and the new value.
    // 或者你可以结合旧值和新值。
    // Let’s look at how to do each of these!
    // 让我们看看如何做每一个！

    // Overwriting a Value
    // 覆盖一个值
    // If we insert a key and a value into a hash map and then insert that same key with a different value, the value associated with that key will be replaced.
    // 如果我们将一个键和一个值插入到哈希映射中，然后用不同的值插入相同的键，则与该键关联的值将被替换。
    // Even though the code in Listing 8-23 calls insert twice, the hash map will only contain one key/value pair because we’re inserting the value for the Blue team’s key both times.
    // 尽管清单 8-23 中的代码调用了两次插入，哈希映射将只包含一个键/值对，因为我们两次都为蓝队的键插入值。

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Listing 8-23: Replacing a value stored with a particular key
    // 示例 8-23：用特定键替换存储的值

    // This code will print {"Blue": 25}. The original value of 10 has been overwritten.
    // 此代码将打印 {"Blue": 25}。 原始值 10 已被覆盖。

    // Adding a Key and Value Only If a Key Isn’t Present
    // 仅在键不存在时添加键和值
    // It’s common to check whether a particular key already exists in the hash map with a value then take the following actions:
    // 检查特定键是否已存在于具有值的哈希映射中是很常见的，然后执行以下操作：
    // ｜- if the key does exist in the hash map, the existing value should remain the way it is. If the key doesn’t exist, insert it and a value for it.
    // ｜- 如果键确实存在于哈希映射中，则现有值应保持原样。 如果键不存在，则插入它并为其赋值。

    // Hash maps have a special API for this called entry that takes the key you want to check as a parameter.
    // 哈希映射有一个特殊的 API 用于此称为entry，它将您要检查的键作为参数。
    // The return value of the entry method is an enum called Entry that represents a value that might or might not exist.
    // entry 方法的返回值是一个名为 Entry 的枚举，表示可能存在或可能不存在的值。
    // Let’s say we want to check whether the key for the Yellow team has a value associated with it.
    // 假设我们要检查 Yellow team 的键是否有与之关联的值。
    // If it doesn’t, we want to insert the value 50, and the same for the Blue team. Using the entry API, the code looks like Listing 8-24.
    // 如果没有，我们要插入值 50，蓝队也一样。 使用entry API，代码如清单 8-24 所示。

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Listing 8-24: Using the entry method to only insert if the key does not already have a value
    // 示例 8-24：使用 entry 方法仅在键还没有值时插入

    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists,
    // Entry 上的 or_insert 方法被定义为返回对相应 Entry 键值的可变引用（如果该键存在），
    // ｜- and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.
    // ｜- 如果不是，则插入参数作为该键的新值并返回对新值的可变引用。
    // This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.
    // 这种技术比我们自己编写逻辑要干净得多，此外，与借用检查器的配合也更好。

    // Running the code in Listing 8-24 will print {"Yellow": 50, "Blue": 10}.
    // 运行清单 8-24 中的代码将打印 {"Yellow": 50, "Blue": 10}。
    // The first call to entry will insert the key for the Yellow team with the value 50 because the Yellow team doesn’t have a value already.
    // 第一次调用 entry 将为黄色队插入值为 50 的键，因为黄色队还没有值。
    // The second call to entry will not change the hash map because the Blue team already has the value 10.
    // 第二次调用 entry 不会改变 hash 映射，因为蓝队已经有了值 10。

    // Updating a Value Based on the Old Value
    // 根据旧值更新值
    // Another common use case for hash maps is to look up a key’s value and then update it based on the old value.
    // 哈希映射的另一个常见用例是查找键的值，然后根据旧值更新它。
    // For instance, Listing 8-25 shows code that counts how many times each word appears in some text.
    // 例如，清单 8-25 显示的代码计算每个单词在某些文本中出现的次数。
    // We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word.
    // 我们使用以单词作为键的哈希映射并增加值以跟踪我们看到该单词的次数。
    // If it’s the first time we’ve seen a word, we’ll first insert the value 0.
    // 如果这是我们第一次看到一个词，我们将首先插入值 0。

    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // Listing 8-25: Counting occurrences of words using a hash map that stores words and counts
    // 示例 8-25：使用存储单词和计数的哈希映射计算单词的出现次数

    // This code will print {"world": 2, "hello": 1, "wonderful": 1}.
    // 此代码将打印 {"world": 2, "hello": 1, "wonderful": 1}。
    // You might see the same key/value pairs printed in a different order: recall from the “Accessing Values in a Hash Map” section that iterating over a hash map happens in an arbitrary order.
    // 您可能会看到以不同顺序打印的相同键/值对：回想一下“访问哈希映射中的值”部分，迭代哈希映射以任意顺序发生。

    // The split_whitespace method returns an iterator over sub-slices, separated by whitespace, of the value in text.
    // split_whitespace 方法返回文本值的子切片上的迭代器，由空格分隔。
    // The or_insert method returns a mutable reference (&mut V) to the value for the specified key.
    // or_insert 方法返回一个指向指定键值的可变引用 (&mut V)。
    // Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*).
    // 这里我们将可变引用存储在 count 变量中，因此为了分配给该值，我们必须首先使用星号 (*) 取消引用 count。
    // The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.
    // 可变引用在 for 循环结束时超出范围（移出作用域），因此所有这些更改都是安全的，并且是借用规则允许的。

    // Hashing Functions
    // 哈希函数
    // By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables^1.
    // 默认情况下，HashMap 使用称为 SipHash 的哈希函数，可以抵抗涉及哈希表的拒绝服务 (DoS) 攻击^1。
    // This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.
    // 这不是可用的最快的哈希算法，但性能下降带来的更好安全性的权衡是值得的。
    // If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher.
    // 如果您分析代码并发现默认哈希函数对于您的目的来说太慢，您可以通过指定不同的哈希器来切换到另一个函数。
    // A hasher is a type that implements the BuildHasher trait.
    // 哈希器是一种实现了 BuildHasher 特性的类型。
    // We’ll talk about traits and how to implement them in Chapter 10.
    // 我们将在第 10 章讨论特征以及如何实现它们。
    // You don’t necessarily have to implement your own hasher from scratch; crates.io has libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.
    // 你不一定要从头开始实现你自己的哈希器； crates.io 有其他 Rust 用户共享的库，这些库提供实现许多常见哈希算法的哈希器。

    // 1 https://en.wikipedia.org/wiki/SipHash

    // Summary
    // 概括
    // Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data.
    // 当您需要存储、访问和修改数据时，向量、字符串和哈希映射将提供程序中所需的大量功能。

    // We’re getting into more complex programs in which operations can fail, so, it’s a perfect time to discuss error handling. We’ll do that next!
    // 我们正在进入更复杂的程序，其中的操作可能会失败，因此，现在是讨论错误处理的最佳时机。 我们接下来会这样做！

    println!("Hello, world!");
}
