// Smart Pointers
// 智能指针
// A pointer is a general concept for a variable that contains an address in memory.
// 指针是包含内存地址的变量的一般概念。
// This address refers to, or “points at,” some other data.
// 此地址引用或“指向”其他一些数据。
// The most common kind of pointer in Rust is a reference, which you learned about in Chapter 4.
// Rust 中最常见的指针类型是引用，您在第 4 章中了解过。
// References are indicated by the & symbol and borrow the value they point to.
// 引用由 & 符号表示，并借用它们指向的值。
// They don’t have any special capabilities other than referring to data, and have no overhead.
// 除了引用数据之外，它们没有任何特殊功能，并且没有开销。

// Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.
// 另一方面，智能指针是一种数据结构，其行为类似于指针，但还具有额外的元数据和功能。
// The concept of smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist in other languages as well.
// 智能指针的概念并不是 Rust 独有的：智能指针起源于 C++，也存在于其他语言中。
// Rust has a variety of smart pointers defined in the standard library that provide functionality beyond that provided by references.
// Rust 在标准库中定义了多种智能指针，它们提供的功能超出了引用所提供的功能。
// To explore the general concept, we’ll look at a couple of different examples of smart pointers, including a reference counting smart pointer type.
// 为了探索一般概念，我们将看几个不同的智能指针示例，包括引用计数智能指针类型。
// This pointer enables you to allow data to have multiple owners by keeping track of the number of owners and, when no owners remain, cleaning up the data.
// 此指针使您能够通过跟踪所有者的数量并在没有所有者时清理数据来允许数据拥有多个所有者。

// Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers:
// Rust 具有所有权和借用的概念，在引用和智能指针之间有一个额外的区别：
//  while references only borrow data, in many cases, smart pointers own the data they point to.
//  虽然引用只是借用数据，但在许多情况下，智能指针拥有它们指向的数据。

// Though we didn’t call them as such at the time, we’ve already encountered a few smart pointers in this book,
// 虽然我们当时没有这样称呼它们，但我们已经在本书中遇到过一些智能指针，
//  including String and Vec<T> in Chapter 8.
//  包括第 8 章中的 String 和 Vec<T>。
// Both these types count as smart pointers because they own some memory and allow you to manipulate it.
// 这两种类型都算作智能指针，因为它们拥有一些内存并允许您对其进行操作。
// They also have metadata and extra capabilities or guarantees.
// 它们也有元数据和额外的能力或保证。
// String, for example, stores its capacity as metadata and has the extra ability to ensure its data will always be valid UTF-8.
// 例如，String 将其容量存储为元数据，并具有确保其数据始终为有效 UTF-8 的额外能力。

// Smart pointers are usually implemented using structs.
// 智能指针通常使用结构体来实现。
// Unlike an ordinary struct, smart pointers implement the Deref and Drop traits.
// 与普通结构不同，智能指针实现了 Deref 和 Drop 特性。
// The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers.
// Deref 特性允许智能指针结构的实例表现得像引用，因此您可以编写代码以使用引用或智能指针。
// The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.
// Drop 特性允许您自定义在智能指针实例超出范围时运行的代码。
// In this chapter, we’ll discuss both traits and demonstrate why they’re important to smart pointers.
// 在本章中，我们将讨论这两个特征并演示为什么它们对智能指针很重要。

// Given that the smart pointer pattern is a general design pattern used frequently in Rust, this chapter won’t cover every existing smart pointer.
// 鉴于智能指针模式是 Rust 中经常使用的通用设计模式，本章不会涵盖所有现有的智能指针。
// Many libraries have their own smart pointers, and you can even write your own.
// 很多库都有自己的智能指针，你甚至可以自己写。
// We’ll cover the most common smart pointers in the standard library:
// 我们将介绍标准库中最常见的智能指针：

// * Box<T> for allocating values on the heap
//   Box<T> 用于在堆上分配值
// * Rc<T>, a reference counting type that enables multiple ownership
//   Rc<T>，一种支持多重所有权的引用计数类型
// * Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
//   Ref<T> 和 RefMut<T>，通过 RefCell<T> 访问，这是一种在运行时而非编译时强制执行借用规则的类型

// In addition, we’ll cover the interior mutability pattern where an immutable type exposes an API for mutating an interior value.
// 此外，我们将介绍内部可变性模式，其中不可变类型公开用于改变内部值的 API。
// We’ll also discuss reference cycles: how they can leak memory and how to prevent them.
// 我们还将讨论引用循环：它们如何泄漏内存以及如何防止它们。

// Let’s dive in!

// Using Box<T> to Point to Data on the Heap
// 使用 Box<T> 指向堆上的数据
// The most straightforward smart pointer is a box, whose type is written Box<T>.
// 最直接的智能指针是一个box，类型写成Box<T>。
// Boxes allow you to store data on the heap rather than the stack.
// Boxes 允许您将数据存储在堆上而不是栈上。
// What remains on the stack is the pointer to the heap data.
// 留在栈上的是指向堆数据的指针。
// Refer to Chapter 4 to review the difference between the stack and the heap.
// 参考第四章回顾栈和堆的区别。

// Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
// box没有性能开销，除了将它们的数据存储在堆上而不是栈上。
// But they don’t have many extra capabilities either.
// 它们也没有很多额外的功能。
// You’ll use them most often in these situations:
// 你会在这些情况下最常使用它们：

// 1.When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
//   当你有一个在编译时无法知道其大小的类型并且你想在需要精确大小的上下文中使用该类型的值时
// 2.When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
//   当你有大量数据并且你想转移所有权但确保数据不会被复制时
// 3.When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
//   当你想要拥有一个值并且你只关心它是一个实现特定特征的类型而不是特定类型时

// We’ll demonstrate the first situation in the “Enabling Recursive Types with Boxes” section.
// 我们将在“使用Boxes启用递归类型”部分演示第一种情况。
// In the second case, transferring ownership of a large amount of data can take a long time because the data is copied around on the stack.
// 在第二种情况下，转移大量数据的所有权可能需要很长时间，因为数据是在堆上复制的。
// To improve performance in this situation, we can store the large amount of data on the heap in a box.
// 为了在这种情况下提高性能，我们可以将堆上的大量数据存储在一个盒子中。
// Then, only the small amount of pointer data is copied around on the stack, while the data it references stays in one place on the heap.
// 然后，只有少量的指针数据被复制到栈上，而它引用的数据留在堆上的一个地方。
// The third case is known as a trait object, and Chapter 17 devotes an entire section, “Using Trait Objects That Allow for Values of Different Types,” just to that topic.
// 第三种情况称为 trait 对象，第 17 章用一整节“使用允许不同类型值的 Trait 对象”来讨论该主题。
// So what you learn here you’ll apply again in Chapter 17!
// 所以你在这里学到的东西将在第 17 章再次应用！

// Using a Box<T> to Store Data on the Heap
// 使用 Box<T> 在堆上存储数据
// Before we discuss the heap storage use case for Box<T>, we’ll cover the syntax and how to interact with values stored within a Box<T>.
// 在我们讨论 Box<T> 的堆存储用例之前，我们将介绍语法以及如何与存储在 Box<T> 中的值进行交互。

// Listing 15-1 shows how to use a box to store an i32 value on the heap:
// 清单 15-1 展示了如何使用一个盒子在堆上存储一个 i32 值：

// Filename: src/main.rs

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Listing 15-1: Storing an i32 value on the heap using a box
// 示例 15-1：使用盒子在堆上存储 i32 值

// We define the variable b to have the value of a Box that points to the value 5, which is allocated on the heap.
// 我们将变量 b 定义为具有指向值 5 的 Box 的值，该值分配在堆上。
// This program will print b = 5; in this case, we can access the data in the box similar to how we would if this data were on the stack.
// 这个程序会打印 b = 5; 在这种情况下，我们可以访问box中的数据，就像这些数据在栈上一样。
// Just like any owned value, when a box goes out of scope, as b does at the end of main, it will be deallocated.
// 就像任何拥有的值一样，当一个盒子超出范围时，就像 b 在 main 的末尾所做的那样，它将被释放。
// The deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).
// 释放发生在盒子（存储在栈中）和它指向的数据（存储在堆中）上。

// Putting a single value on the heap isn’t very useful, so you won’t use boxes by themselves in this way very often.
// 将单个值放在堆上不是很有用，因此您不会经常以这种方式单独使用boxes。
// Having values like a single i32 on the stack, where they’re stored by default, is more appropriate in the majority of situations.
// 在大多数情况下，在栈上拥有像单个 i32 这样的值（它们默认存储在此处）更合适。
// Let’s look at a case where boxes allow us to define types that we wouldn’t be allowed to if we didn’t have boxes.
// 让我们看一个案例，其中盒子允许我们定义（这种）类型，如果我们没有盒子，我们将不允许定义这些类型。

// Enabling Recursive Types with Boxes
// 启用带Boxes的递归类型
// A value of recursive type can have another value of the same type as part of itself.
// 递归类型的值可以有另一个相同类型的值作为它自身的一部分。
// Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up.
// 递归类型会带来问题，因为在编译时 Rust 需要知道一个类型占用了多少空间。
// However, the nesting of values of recursive types could theoretically continue infinitely, so Rust can’t know how much space the value needs.
// 然而，递归类型的值的嵌套理论上可以无限继续，因此 Rust 无法知道值需要多少空间。
// Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.
// 因为Box的大小已知（一个引用），所以我们可以通过在递归类型定义中插入一个Box来启用递归类型。

// As an example of a recursive type, let’s explore the cons list. This is a data type commonly found in functional programming languages.
// 作为递归类型的示例，让我们探索cons list。 这是函数式编程语言中常见的数据类型。
// The cons list type we’ll define is straightforward except for the recursion;
// 除了递归之外，我们将定义的 cons 列表类型很简单；
// therefore, the concepts in the example we’ll work with will be useful any time you get into more complex situations involving recursive types.
// 因此，每当您遇到涉及递归类型的更复杂情况时，我们将使用的示例中的概念将很有用。

// More Information About the Cons List
// 有关 Cons 列表的更多信息
// A cons list is a data structure that comes from the Lisp programming language and its dialects and is made up of nested pairs, and is the Lisp version of a linked list.
// Cons list 是一种数据结构，它来自 Lisp 编程语言及其方言，由嵌套对组成，是链表的 Lisp 版本。
// Its name comes from the cons function (short for “construct function”) in Lisp that constructs a new pair from its two arguments.
// 它的名字来自 Lisp 中的 cons 函数（“construct function”的缩写），它从它的两个参数构造一个新的对（a new pair）。
// By calling cons on a pair consisting of a value and another pair, we can construct cons lists made up of recursive pairs.
// 通过对由一个值和另一对（another pair）组成的对（a pair）调用 cons，我们可以构建由递归对组成的 cons list。

// For example, here’s a pseudocode representation of a cons list containing the list 1, 2, 3 with each pair in parentheses:
// 例如，这是包含 列表 1、2、3 的 cons 列表的伪代码表示，每一对都在括号中：

(1, (2, (3, Nil)))

// Each item in a cons list contains two elements: the value of the current item and the next item.
// cons 列表中的每个项目都包含两个元素：当前项目的值和下一个项目（的值，项目=自身）。
// The last item in the list contains only a value called Nil without a next item.
// 列表中的最后一项仅包含一个名为 Nil 的值，没有下一项。
// A cons list is produced by recursively calling the cons function.
// 通过递归调用 cons 函数生成一个 cons 列表。
// The canonical name to denote the base case of the recursion is Nil.
// 表示递归基本情况的规范名称是 Nil。
// Note that this is not the same as the “null” or “nil” concept in Chapter 6, which is an invalid or absent value.
// 请注意，这与第 6 章中的“null”或“nil”概念不同，后者是无效或不存在的值。

// The cons list isn’t a commonly used data structure in Rust. Most of the time when you have a list of items in Rust, Vec<T> is a better choice to use.
// Cons list 不是 Rust 中常用的数据结构。 大多数时候，当你在 Rust 中有一个项目列表时，Vec<T> 是一个更好的选择。
// Other, more complex recursive data types are useful in various situations,
// 其他更复杂的递归数据类型在各种情况下都很有用，
// |- but by starting with the cons list in this chapter, we can explore how boxes let us define a recursive data type without much distraction.
// |- 但从本章的 cons list 开始，我们可以探索 boxes 如何让我们定义递归数据类型。

// Listing 15-2 contains an enum definition for a cons list.
// 清单 15-2 包含一个 cons 列表的枚举定义。
// Note that this code won’t compile yet because the List type doesn’t have a known size, which we’ll demonstrate.
// 请注意，此代码尚未编译，因为 List 类型没有已知大小，我们将对此进行演示。

// Filename: src/main.rs

enum List {
    Cons(i32, List),
    Nil,
}

// Listing 15-2: The first attempt at defining an enum to represent a cons list data structure of i32 values
// 示例 15-2：第一次尝试定义枚举来表示 i32 值的 cons 列表数据结构

// Note: We’re implementing a cons list that holds only i32 values for the purposes of this example.
// 注意：出于本示例的目的，我们正在实现一个仅包含 i32 值的 cons 列表。
// We could have implemented it using generics, as we discussed in Chapter 10, to define a cons list type that could store values of any type.
// 我们可以使用泛型来实现它，正如我们在第 10 章中讨论的那样，定义一个可以存储任何类型值的 cons 列表类型。

// Using the List type to store the list 1, 2, 3 would look like the code in Listing 15-3:
// 使用 List 类型存储列表 1、2、3 类似于清单 15-3 中的代码：

// Filename: src/main.rs

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}

// Listing 15-3: Using the List enum to store the list 1, 2, 3
// 示例 15-3：使用 List 枚举存储列表 1、2、3

// The first Cons value holds 1 and another List value. This List value is another Cons value that holds 2 and another List value.
// 第一个 Cons 值包含 1 和另一个 List 值。 此 List 值是另一个包含 2 和另一个 List 值的 Cons 值。
// This List value is one more Cons value that holds 3 and a List value, which is finally Nil, the non-recursive variant that signals the end of the list.
// 这个 List 值是一个包含 3 的 Cons 值和一个 List 值，它最终是 Nil，表示列表结束的非递归变体。

// If we try to compile the code in Listing 15-3, we get the error shown in Listing 15-4:
// 如果我们尝试编译清单 15-3 中的代码，我们会得到如清单 15-4 所示的错误：

// $ cargo run
//    Compiling cons-list v0.1.0 (file:///projects/cons-list)
// error[E0072]: recursive type `List` has infinite size
//  --> src/main.rs:1:1
//   |
// 1 | enum List {
//   | ^^^^^^^^^ recursive type has infinite size
// 2 |     Cons(i32, List),
//   |               ---- recursive without indirection
//   |
// help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
//   |
// 2 |     Cons(i32, Box<List>),
//   |               ++++    +
//
// error[E0391]: cycle detected when computing drop-check constraints for `List`
//  --> src/main.rs:1:1
//   |
// 1 | enum List {
//   | ^^^^^^^^^
//   |
//   = note: ...which immediately requires computing drop-check constraints for `List` again
//   = note: cycle used when computing dropck types for `Canonical { max_universe: U0, variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, constness: NotConst }, value: List } }`
//
// Some errors have detailed explanations: E0072, E0391.
// For more information about an error, try `rustc --explain E0072`.
// error: could not compile `cons-list` due to 2 previous errors

// Listing 15-4: The error we get when attempting to define a recursive enum
// 示例 15-4：尝试定义递归枚举时出现的错误

// The error shows this type “has infinite size.” The reason is that we’ve defined List with a variant that is recursive: it holds another value of itself directly.
// 错误显示此类型“具有无限大小”。 原因是我们用递归的变体定义了 List：它直接持有自身的另一个值。
// As a result, Rust can’t figure out how much space it needs to store a List value.
// 因此，Rust 无法计算出它需要多少空间来存储一个 List 值。
// Let’s break down why we get this error. First, we’ll look at how Rust decides how much space it needs to store a value of a non-recursive type.
// 让我们分析一下为什么会出现这个错误。 首先，我们将看看 Rust 如何决定它需要多少空间来存储一个非递归类型的值。

// Computing the Size of a Non-Recursive Type
// 计算非递归类型的大小
// Recall the Message enum we defined in Listing 6-2 when we discussed enum definitions in Chapter 6:
// 回想我们在第 6 章中讨论枚举定义时在清单 6-2 中定义的 Message 枚举：

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// To determine how much space to allocate for a Message value, Rust goes through each of the variants to see which variant needs the most space.
// 为了确定为消息值分配多少空间，Rust 遍历每个变体以查看哪个变体需要最多空间。
// Rust sees that Message::Quit doesn’t need any space, Message::Move needs enough space to store two i32 values, and so forth.
// Rust 认为 Message::Quit 不需要任何空间，Message::Move 需要足够的空间来存储两个 i32 值，等等。
// Because only one variant will be used, the most space a Message value will need is the space it would take to store the largest of its variants.
// 因为只会使用一个变体，所以 Message 值所需的最大空间是存储其最大变体所需的空间。

// Contrast this with what happens when Rust tries to determine how much space a recursive type like the List enum in Listing 15-2 needs.
// 将此与 Rust 尝试确定递归类型（如清单 15-2 中的 List 枚举）需要多少空间时发生的情况进行对比。
// The compiler starts by looking at the Cons variant, which holds a value of type i32 and a value of type List.
// 编译器首先查看 Cons 变量，它包含一个 i32 类型的值和一个 List 类型的值。
// Therefore, Cons needs an amount of space equal to the size of an i32 plus the size of a List.
// 因此，Cons 需要的空间量等于 i32 的大小加上 List 的大小。
// To figure out how much memory the List type needs, the compiler looks at the variants, starting with the Cons variant.
// 为了弄清楚 List 类型需要多少内存，编译器会查看变体，从 Cons 变体开始。
// The Cons variant holds a value of type i32 and a value of type List, and this process continues infinitely, as shown in Figure 15-1.
// Cons 变量保存了一个 i32 类型的值和一个 List 类型的值，这个过程无限继续下去，如图 15-1 所示。

//   Cons
// ---------------------------------------------
// |     | Cons                              | |
// |     |-------------------------------- | | |
// |     |     | Cons                    | | | |
// |     |     |------------------------ | | | |
// | i32 |     |     | Cons            | | | | |
// |     | i32 |     |---------------- | | | | |
// |     |     | i32 |     | Cons    | | | | | |
// |     |     |     | i32 |---------| | | | | |
// |     |     |     |     | i32 | ∞ | | | | | |
// |     |     |     |     |---------- | | | | |
// |     |     |     |-------------------| | | |
// |     |     |---------------------------- | |
// |     |------------------------------------ |
// ---------------------------------------------
// Figure 15-1: An infinite List consisting of infinite Cons variants
// 图 15-1：由无限 Cons 变体组成的无限列表

// Using Box<T> to Get a Recursive Type with a Known Size
// 使用 Box<T> 获取已知大小的递归类型
// Because Rust can’t figure out how much space to allocate for recursively defined types, the compiler gives an error with this helpful suggestion:
// 因为 Rust 无法计算出为递归定义的类型分配多少空间，编译器给出了一个错误并给出了这个有用的建议：

// help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
//   |
// 2 |     Cons(i32, Box<List>),
//   |               ++++    +

// In this suggestion, “indirection” means that instead of storing a value directly,
// 在此建议中，“间接”意味着不直接存储值，
// |- we should change the data structure to store the value indirectly by storing a pointer to the value instead.
// |- 我们应该更改数据结构以通过存储 指向值的指针 来间接存储值。

// Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to.
// 因为 Box<T> 是一个指针，Rust 总是知道 Box<T> 需要多少空间：指针的大小不会根据它指向的数据量而改变。
// This means we can put a Box<T> inside the Cons variant instead of another List value directly.
// 这意味着我们可以将 Box<T> 放入 Cons 变体中，而不是直接放入另一个 List 值。
// The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant.
// Box<T> 将指向堆上的下一个 List 值，而不是 Cons 变体中的值。
// Conceptually, we still have a list, created with lists holding other lists, but this implementation is now more like placing the items next to one another rather than inside one another.
// 从概念上讲，我们仍然有一个列表，它是用包含其他列表的列表创建的，但是这种实现现在更像是将项目一个接一个地放置，而不是一个包含另一个地放置。

// We can change the definition of the List enum in Listing 15-2 and the usage of the List in Listing 15-3 to the code in Listing 15-5, which will compile:
// 我们可以将示例 15-2 中 List 枚举的定义和示例 15-3 中 List 的用法更改为示例 15-5 中的代码，它将编译：

// Filename: src/main.rs

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// Listing 15-5: Definition of List that uses Box<T> in order to have a known size
// 示例 15-5：使用 Box<T> 来获得已知大小的 List 的定义

// The Cons variant needs the size of an i32 plus the space to store the box’s pointer data.
// Cons 变体需要 i32 的大小加上存储 box 的指针数据的空间。
// The Nil variant stores no values, so it needs less space than the Cons variant.
// Nil 变体不存储任何值，因此它比 Cons 变体需要更少的空间。
// We now know that any List value will take up the size of an i32 plus the size of a box’s pointer data.
// 我们现在知道，任何 List 值都将占用 i32 的大小加上box的指针数据的大小。
// By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a List value.
// 通过使用盒子，我们打破了无限递归链，因此编译器可以计算出存储列表值所需的大小。
// Figure 15-2 shows what the Cons variant looks like now.
// 图 15-2 显示了 Cons 变体现在的样子。

//   Cons
// -------------------
// |     |   Box     |
// |     | --------- |
// | i32 | | usize | |
// |     | --------- |
// -------------------

// Figure 15-2: A List that is not infinitely sized because Cons holds a Box
// 图 15-2：一个不是无限大的列表，因为 Cons 包含一个 Box

// Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types.
// Boxes 只提供间接和堆分配； 它们没有任何其他特殊功能，就像我们将在其他智能指针类型中看到的那样。
// They also don’t have the performance overhead that these special capabilities incur, so they can be useful in cases like the cons list where the indirection is the only feature we need.
// 它们也没有这些特殊功能带来的性能开销，因此它们在像 cons 列表这样的情况下非常有用，在这种情况下，间接是我们唯一需要的功能。
// We’ll look at more use cases for boxes in Chapter 17, too.
// 我们也会在第 17 章中看到更多盒子的用例。

// The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references.
// Box<T> 类型是智能指针，因为它实现了 Deref 特性，允许将 Box<T> 值视为引用。
// When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.
// 当 Box<T> 值超出范围时，由于 Drop 特性的实现，box 指向的堆数据也会被清除。
// These two traits will be even more important to the functionality provided by the other smart pointer types we’ll discuss in the rest of this chapter.
// 这两个特性对于我们将在本章其余部分讨论的其他智能指针类型所提供的功能更为重要。
// Let’s explore these two traits in more detail.
// 让我们更详细地探讨这两个特征。


// Treating Smart Pointers Like Regular References with the Deref Trait
// 使用 Deref 特性将智能指针视为常规引用
// Implementing the Deref trait allows you to customize the behavior of the dereference operator * (not to be confused with the multiplication or glob operator).
// 实现 Deref 特性允许您自定义解引用运算符 * 的行为（不要与乘法或 glob 运算符混淆）。
// By implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.
// 通过以可以将智能指针视为常规引用的方式实现 Deref，您可以编写对引用进行操作的代码，并将该代码与智能指针一起使用。

// Let’s first look at how the dereference operator works with regular references.
// 让我们首先看看解引用运算符如何处理常规引用。
// Then we’ll try to define a custom type that behaves like Box<T>, and see why the dereference operator doesn’t work like a reference on our newly defined type.
// 然后我们将尝试定义一个行为类似于 Box<T> 的自定义类型，并了解为什么解引用运算符不像我们新定义的类型上的引用那样工作。
// We’ll explore how implementing the Deref trait makes it possible for smart pointers to work in ways similar to references.
// 我们将探讨实现 Deref 特性如何使智能指针以类似于引用的方式工作。
// Then we’ll look at Rust’s deref coercion feature and how it lets us work with either references or smart pointers.
// 然后我们将看看 Rust 的 deref 强制转换功能，以及它如何让我们使用引用或智能指针。

// Note: there’s one big difference between the MyBox<T> type we’re about to build and the real Box<T>: our version will not store its data on the heap.
// 注意：我们将要构建的 MyBox<T> 类型与真正的 Box<T> 之间有一个很大的区别：我们的版本不会将其数据存储在堆上。
// We are focusing this example on Deref, so where the data is actually stored is less important than the pointer-like behavior.
// 我们将此示例重点放在 Deref 上，因此实际存储数据的位置不如类似指针的行为重要。

// Following the Pointer to the Value
// 跟随指向值的指针
// A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.
// 常规引用是一种指针，将指针视为指向存储在其他地方的值的一种方式。
// In Listing 15-6, we create a reference to an i32 value and then use the dereference operator to follow the reference to the value:
// 在清单 15-6 中，我们创建了一个 i32 值的引用，然后使用解引用运算符跟随对该值的引用：

// Filename: src/main.rs

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Listing 15-6: Using the dereference operator to follow a reference to an i32 value
// 示例 15-6：使用解引用运算符跟随对 i32 值的引用

// The variable x holds an i32 value 5. We set y equal to a reference to x. We can assert that x is equal to 5.
// 变量 x 的 i32 值为 5。我们将 y 设置为对 x 的引用。 我们可以断言 x 等于 5。
// However, if we want to make an assertion about the value in y,
// 但是，如果我们想对 y 中的值进行断言，
//  we have to use *y to follow the reference to the value it’s pointing to (hence dereference) so the compiler can compare the actual value.
// 我们必须使用 *y 来跟随对它指向的值的引用（因此解引用），以便编译器可以比较实际值。
// Once we dereference y, we have access to the integer value y is pointing to that we can compare with 5.
// 一旦我们解引用 y，我们就可以访问 y 指向的整数值，我们可以将其与 5 进行比较。

// If we tried to write assert_eq!(5, y); instead, we would get this compilation error:
// 如果我们尝试写 assert_eq!(5, y); 相反，我们会得到这个编译错误：

// $ cargo run
//    Compiling deref-example v0.1.0 (file:///projects/deref-example)
// error[E0277]: can't compare `{integer}` with `&{integer}`
//  --> src/main.rs:6:5
//   |
// 6 |     assert_eq!(5, y);
//   |     ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
//   |
//   = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
//   = help: the following other types implement trait `PartialEq<Rhs>`:
//             f32
//             f64
//             i128
//             i16
//             i32
//             i64
//             i8
//             isize
//           and 6 others
//   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
//
// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `deref-example` due to previous error

// Comparing a number and a reference to a number isn’t allowed because they’re different types.
// 比较数字和对数字的引用是不允许的，因为它们是不同的类型。
// We must use the dereference operator to follow the reference to the value it’s pointing to.
// 我们必须使用解引用运算符来跟随对它指向的值的引用。

// Using Box<T> Like a Reference
// 像引用一样使用 Box<T>
// We can rewrite the code in Listing 15-6 to use a Box<T> instead of a reference;
// 我们可以重写示例 15-6 中的代码以使用 Box<T> 而不是引用；
// the dereference operator used on the Box<T> in Listing 15-7 functions in the same way as the dereference operator used on the reference in Listing 15-6:
// 清单 15-7 中 Box<T> 上使用的解引用运算符与清单 15-6 中引用中使用的解引用运算符的功能相同：

// Filename: src/main.rs

fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Listing 15-7: Using the dereference operator on a Box<i32>
// 示例 15-7：在 Box<i32> 上使用解引用运算符

// The main difference between Listing 15-7 and Listing 15-6 is that here we set y to be an instance of a Box<T> pointing to a copied value of x rather than a reference pointing to the value of x.
// 清单 15-7 和清单 15-6 之间的主要区别在于，这里我们将 y 设置为指向 x 的复制值的 Box<T> 实例，而不是指向 x 值的引用。
// In the last assertion, we can use the dereference operator to follow the pointer of the Box<T> in the same way that we did when y was a reference.
// 在最后一个断言中，我们可以使用解引用运算符来跟随 Box<T> 的指针，就像我们在 y 是引用时所做的那样。
// Next, we’ll explore what is special about Box<T> that enables us to use the dereference operator by defining our own type.
// 接下来，我们将探讨 Box<T> 的特殊之处，它使我们能够通过定义自己的类型来使用解引用运算符。

// Defining Our Own Smart Pointer
// 定义我们自己的智能指针
// Let’s build a smart pointer similar to the Box<T> type provided by the standard library to experience how smart pointers behave differently from references by default.
// 让我们构建一个类似于标准库提供的 Box<T> 类型的智能指针，以体验智能指针在默认情况下与引用的行为有何不同。
// Then we’ll look at how to add the ability to use the dereference operator.
// 然后我们将看看如何添加使用解引用运算符的能力。

// The Box<T> type is ultimately defined as a tuple struct with one element, so Listing 15-8 defines a MyBox<T> type in the same way.
// Box<T> 类型最终定义为具有一个元素的元组结构，因此清单 15-8 以相同的方式定义了 MyBox<T> 类型。
// We’ll also define a new function to match the new function defined on Box<T>.

// Filename: src/main.rs

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Listing 15-8: Defining a MyBox<T> type
// 示例 15-8：定义 MyBox<T> 类型

// We define a struct named MyBox and declare a generic parameter T, because we want our type to hold values of any type.
// 我们定义了一个名为 MyBox 的结构并声明了一个泛型参数 T，因为我们希望我们的类型包含任何类型的值。
// The MyBox type is a tuple struct with one element of type T. The MyBox::new function takes one parameter of type T and returns a MyBox instance that holds the value passed in.
// MyBox 类型是一个包含一个 T 类型元素的元组结构。MyBox::new 函数接受一个 T 类型的参数，并返回一个保存传入值的 MyBox 实例。

// Let’s try adding the main function in Listing 15-7 to Listing 15-8 and changing it to use the MyBox<T> type we’ve defined instead of Box<T>.
// 让我们尝试将清单 15-7 中的 main 函数添加到清单 15-8 并将其更改为使用我们定义的 MyBox<T> 类型而不是 Box<T>。
// The code in Listing 15-9 won’t compile because Rust doesn’t know how to dereference MyBox.
// 清单 15-9 中的代码无法编译，因为 Rust 不知道如何解引用 MyBox。

// Filename: src/main.rs

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Listing 15-9: Attempting to use MyBox<T> in the same way we used references and Box<T>
// 示例 15-9：尝试以与使用引用和 Box<T> 相同的方式使用 MyBox<T>

// Here’s the resulting compilation error:
// 这是产生的编译错误：

// $ cargo run
//    Compiling deref-example v0.1.0 (file:///projects/deref-example)
// error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
//   --> src/main.rs:14:19
//    |
// 14 |     assert_eq!(5, *y);
//    |                   ^^
//
// For more information about this error, try `rustc --explain E0614`.
// error: could not compile `deref-example` due to previous error

// Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type.
// 我们的 MyBox<T> 类型不能被解引用，因为我们还没有在我们的类型上实现那个能力。
// To enable dereferencing with the * operator, we implement the Deref trait.
// 为了使用 * 运算符启用解引用，我们实现了 Deref 特性。

// Treating a Type Like a Reference by Implementing the Deref Trait
// 通过实现 Deref 特性将类型视为引用
// As discussed in the “Implementing a Trait on a Type” section of Chapter 10, to implement a trait, we need to provide implementations for the trait’s required methods.
// 正如第 10 章“在类型上实现特征”部分所讨论的，要实现特征，我们需要为特征所需的方法提供实现。
// The Deref trait, provided by the standard library, requires us to implement one method named deref that borrows self and returns a reference to the inner data.
// 标准库提供的 Deref trait 要求我们实现一个名为 deref 的方法，该方法借用 self 并返回对内部数据的引用。
// Listing 15-10 contains an implementation of Deref to add to the definition of MyBox:
// 清单 15-10 包含要添加到 MyBox 定义的 Deref 的实现：

// Filename: src/main.rs

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Listing 15-10: Implementing Deref on MyBox<T>
// 示例 15-10：在 MyBox<T> 上实现 Deref

// The type Target = T; syntax defines an associated type for the Deref trait to use.
// 类型 Target = T; 语法为要使用的 Deref 特性定义关联类型。
// Associated types are a slightly different way of declaring a generic parameter, but you don’t need to worry about them for now; we’ll cover them in more detail in Chapter 19.
// 关联类型是声明泛型参数的一种稍微不同的方式，但你现在不需要担心它们； 我们将在第 19 章中更详细地介绍它们。

// We fill in the body of the deref method with &self.0 so deref returns a reference to the value we want to access with the * operator;
// 我们用 &self.0 填充 deref 方法的主体，因此 deref 返回对我们要使用 * 运算符访问的值的引用；
// recall from the “Using Tuple Structs without Named Fields to Create Different Types” section of Chapter 5 that .0 accesses the first value in a tuple struct.
// 回忆一下第 5 章的“使用不带命名字段的元组结构来创建不同类型”部分，.0 访问元组结构中的第一个值。
// The main function in Listing 15-9 that calls * on the MyBox<T> value now compiles, and the assertions pass!
// 清单 15-9 中在 MyBox<T> 值上调用 * 的主函数现在可以编译，并且断言通过！

// Without the Deref trait, the compiler can only dereference & references.
// 如果没有 Deref 特性，编译器只能解引用 & 引用。
// The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.
// deref 方法使编译器能够获取实现 Deref 的任何类型的值，并调用 deref 方法来获取它知道如何解引用的 & 引用。

// When we entered *y in Listing 15-9, behind the scenes Rust actually ran this code:
// 当我们在示例 15-9 中输入 *y 时，Rust 在幕后实际运行了这段代码：

*(y.deref())

// Rust substitutes the * operator with a call to the deref method and then a plain dereference so we don’t have to think about whether or not we need to call the deref method.
// Rust 将 * 运算符替换为对 deref 方法的调用，然后是一个简单的解引用，因此我们不必考虑是否需要调用 deref 方法。
// This Rust feature lets us write code that functions identically whether we have a regular reference or a type that implements Deref.
// 这个 Rust 特性让我们可以编写功能相同的代码，无论我们有一个常规引用还是一个实现 Deref 的类型。

// The reason the deref method returns a reference to a value, and that the plain dereference outside the parentheses in *(y.deref()) is still necessary, is to do with the ownership system.
// deref 方法返回对值的引用的原因，以及 *(y.deref()) 中括号外的普通取消引用仍然是必要的，这是与所有权系统有关。
// If the deref method returned the value directly instead of a reference to the value, the value would be moved out of self.
// 如果 deref 方法直接返回值而不是对该值的引用，该值将被移出 self.
// We don’t want to take ownership of the inner value inside MyBox<T> in this case or in most cases where we use the dereference operator.
// （在这种情况下或）在我们使用解引用运算符的大多数情况下，我们不想获得 MyBox<T> 内部值的所有权。

// Note that the * operator is replaced with a call to the deref method and then a call to the * operator just once, each time we use a * in our code.
// 请注意，每次我们在代码中使用 * 时，* 运算符被替换为对 deref 方法的调用，然后仅调用一次 * 运算符。
// Because the substitution of the * operator does not recurse infinitely, we end up with data of type i32, which matches the 5 in assert_eq! in Listing 15-9.
// 因为 * 运算符的替换不会无限递归，所以我们最终得到类型为 i32 的数据，它与 assert_eq 中的 5 匹配！ 在清单 15-9 中。

// Implicit Deref Coercions with Functions and Methods
// 函数和方法的隐式 Deref 强制转换
// Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type.
// Deref 强制将对实现 Deref 特性的类型的引用转换为对另一种类型的引用。
// For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str.
// 例如，deref 强制转换可以将 &String 转换为 &str，因为 String 实现了 Deref 特性，因此它返回 &str。
// Deref coercion is a convenience Rust performs on arguments to functions and methods, and works only on types that implement the Deref trait.
// Deref 强制转换是 Rust 对函数和方法的参数执行的一种便利，并且仅适用于实现 Deref 特征的类型。
// It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition.
// 当我们将对特定类型值的引用作为参数传递给与函数或方法定义中的参数类型不匹配的函数或方法时，它会自动发生。
// A sequence of calls to the deref method converts the type we provided into the type the parameter needs.
// 对 deref 方法的一系列调用将我们提供的类型转换为参数需要的类型。

// Deref coercion was added to Rust so that programmers writing function and method calls don’t need to add as many explicit references and dereferences with & and *.
// 在 Rust 中添加了 Deref 强制，这样编写函数和方法调用的程序员就不需要使用 & 和 * 添加尽可能多的明确引用和解引用。
// The deref coercion feature also lets us write more code that can work for either references or smart pointers.
// deref 强制转换功能还让我们可以编写更多可用于引用或智能指针的代码。

// To see deref coercion in action, let’s use the MyBox<T> type we defined in Listing 15-8 as well as the implementation of Deref that we added in Listing 15-10.
// 要查看实际的 deref 强制转换，让我们使用我们在示例 15-8 中定义的 MyBox<T> 类型以及我们在示例 15-10 中添加的 Deref 的实现。
// Listing 15-11 shows the definition of a function that has a string slice parameter:
// 清单 15-11 显示了一个函数的定义，它有一个字符串切片参数：

// Filename: src/main.rs

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Listing 15-11: A hello function that has the parameter name of type &str
// 示例 15-11：一个参数名称为 &str 类型的 hello 函数

// We can call the hello function with a string slice as an argument, such as hello("Rust"); for example.
// 我们可以用字符串切片作为参数调用 hello 函数，例如 hello("Rust"); 例如。
// Deref coercion makes it possible to call hello with a reference to a value of type MyBox<String>, as shown in Listing 15-12:
// Deref 强制可以通过引用类型为 MyBox<String> 的值来调用 hello，如示例 15-12 所示：

// Filename: src/main.rs

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

// Listing 15-12: Calling hello with a reference to a MyBox<String> value, which works because of deref coercion
// 示例 15-12：使用对 MyBox<String> 值的引用调用 hello，由于 deref 强制，它可以工作

// Here we’re calling the hello function with the argument &m, which is a reference to a MyBox<String> value.
// 这里我们使用参数 &m 调用 hello 函数，它是对 MyBox<String> 值的引用。
// Because we implemented the Deref trait on MyBox<T> in Listing 15-10, Rust can turn &MyBox<String> into &String by calling deref.
// 因为我们在示例 15-10 中对 MyBox<T> 实现了 Deref 特性，Rust 可以通过调用 deref 将 &MyBox<String> 转换为 &String。
// The standard library provides an implementation of Deref on String that returns a string slice, and this is in the API documentation for Deref.
// 标准库提供了 Deref on String 的实现，它返回一个字符串切片，这在 Deref 的 API 文档中。
// Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.
// Rust 再次调用 deref 将 &String 转换为 &str，这与 hello 函数的定义相匹配。

// If Rust didn’t implement deref coercion, we would have to write the code in Listing 15-13 instead of the code in Listing 15-12 to call hello with a value of type &MyBox<String>.
// 如果 Rust 没有实现 deref 强制转换，我们将不得不编写示例 15-13 中的代码而不是示例 15-12 中的代码来调用 hello 并使用 &MyBox<String> 类型的值。

// Filename: src/main.rs

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

// Listing 15-13: The code we would have to write if Rust didn’t have deref coercion
// 示例 15-13：如果 Rust 没有解引用强制，我们必须编写的代码

// The (*m) dereferences the MyBox<String> into a String. Then the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello.
// (*m) 将 MyBox<String> 解引用为字符串。 然后 & 和 [..] 获取 String 的一个字符串切片，该切片等于整个字符串以匹配 hello 的签名。
// This code without deref coercions is harder to read, write, and understand with all of these symbols involved.
// 没有 deref 强制的这段代码更难阅读、编写和理解所有这些涉及的符号。
// Deref coercion allows Rust to handle these conversions for us automatically.
// Deref 强制允许 Rust 自动为我们处理这些转换。

// When the Deref trait is defined for the types involved, Rust will analyze the types and use Deref::deref as many times as necessary to get a reference to match the parameter’s type.
// 当为涉及的类型定义 Deref 特性时，Rust 将分析类型并根据需要多次使用 Deref::deref 以获得与参数类型匹配的引用。
// The number of times that Deref::deref needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!
// Deref::deref 需要插入的次数在编译时确定，因此利用 deref 强制转换没有运行时惩罚！

// How Deref Coercion Interacts with Mutability
// Deref 强制转换如何与可变性交互
// Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.
// 与使用 Deref 特性覆盖不可变引用上的 * 运算符的方式类似，您可以使用 DerefMut 特性覆盖可变引用上的 * 运算符。

// Rust does deref coercion when it finds types and trait implementations in three cases:
// Rust 在三种情况下找到类型和特征实现时会解引用强制转换：

// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>

// The first two cases are the same as each other except that the second implements mutability.
// 前两种情况彼此相同，只是第二种情况实现了可变性。
// The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently.
// 第一种情况表明，如果你有一个 &T，并且 T 实现了对某种类型 U 的 Deref，你可以透明地得到一个 &U。
// The second case states that the same deref coercion happens for mutable references.
// 第二种情况表明相同的解引用强制发生在可变引用上。

// The third case is trickier: Rust will also coerce a mutable reference to an immutable one.
// 第三种情况比较棘手：Rust 还会将可变引用强制转换为不可变引用。
// But the reverse is not possible: immutable references will never coerce to mutable references.
// 但反过来是不可能的：不可变引用永远不会强制转换为可变引用。
// Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile).
// 由于借用规则，如果你有一个可变引用，该可变引用必须是对该数据的唯一引用（否则，程序将无法编译）。
// Converting one mutable reference to one immutable reference will never break the borrowing rules.
// 将一个可变引用转换为一个不可变引用永远不会违反借用规则。
// Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don’t guarantee that.
// 将不可变引用转换为可变引用需要初始不可变引用是对该数据的唯一不可变引用，但借用规则不能保证这一点。
// Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.
// 因此，Rust 不能假设将不可变引用转换为可变引用是可能的。

// Running Code on Cleanup with the Drop Trait
// 使用 Drop Trait 运行清理代码
// The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope.
// 智能指针模式的第二个重要特征是 Drop，它允许您自定义值即将超出范围时发生的情况。
// You can provide an implementation for the Drop trait on any type, and that code can be used to release resources like files or network connections.
// 您可以为任何类型的 Drop 特性提供一个实现，该代码可用于释放文件或网络连接等资源。

// We’re introducing Drop in the context of smart pointers because the functionality of the Drop trait is almost always used when implementing a smart pointer.
// 我们在智能指针的上下文中引入 Drop，因为在实现智能指针时几乎总是使用 Drop 特性的功能。
// For example, when a Box<T> is dropped it will deallocate the space on the heap that the box points to.
// 例如，当放下一个 Box<T> 时，它将释放该box指向的堆上的空间。

// In some languages, for some types, the programmer must call code to free memory or resources every time they finish using an instance of those types.
// 在某些语言中，对于某些类型，程序员必须在每次使用完这些类型的实例时调用代码来释放内存或资源。
// Examples include file handles, sockets, or locks. If they forget, the system might become overloaded and crash.
// 示例包括文件句柄、套接字或锁。 如果他们忘记了，系统可能会过载并崩溃。
// In Rust, you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically.
// 在 Rust 中，您可以指定每当值超出范围时运行一段特定的代码，编译器将自动插入这段代码。
// As a result, you don’t need to be careful about placing cleanup code everywhere in a program that an instance of a particular type is finished with—you still won’t leak resources!
// 因此，您无需小心地将清理代码放置在特定类型的实例已完成的程序中的任何地方——您不会泄漏资源！

// You specify the code to run when a value goes out of scope by implementing the Drop trait.
// 通过实现 Drop 特性，您可以指定在值超出范围时运行的代码。
// The Drop trait requires you to implement one method named drop that takes a mutable reference to self.
// Drop 特性要求您实现一个名为 drop 的方法，该方法采用对 self 的可变引用。
// To see when Rust calls drop, let’s implement drop with println! statements for now.
// 要查看 Rust 何时调用 drop，让我们用 println 实现 drop！ 现在的声明。

// Listing 15-14 shows a CustomSmartPointer struct whose only custom functionality is that it will print Dropping CustomSmartPointer!
// 清单 15-14 显示了一个 CustomSmartPointer 结构，其唯一的自定义功能是它将打印 Dropping CustomSmartPointer！
// when the instance goes out of scope, to show when Rust runs the drop function.
// 当实例超出范围时，显示 Rust 何时运行 drop 函数。

// Filename: src/main.rs

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

// Listing 15-14: A CustomSmartPointer struct that implements the Drop trait where we would put our cleanup code
// 示例 15-14：实现 Drop 特性的 CustomSmartPointer 结构，我们将在其中放置清理代码

// The Drop trait is included in the prelude, so we don’t need to bring it into scope.
// Drop trait 包含在 prelude 中，因此我们不需要将其引入作用域。
// We implement the Drop trait on CustomSmartPointer and provide an implementation for the drop method that calls println!.
// The body of the drop function is where you would place any logic that you wanted to run when an instance of your type goes out of scope.
// We’re printing some text here to demonstrate visually when Rust will call drop.

// In main, we create two instances of CustomSmartPointer and then print CustomSmartPointers created.
// At the end of main, our instances of CustomSmartPointer will go out of scope, and Rust will call the code we put in the drop method, printing our final message.
// Note that we didn’t need to call the drop method explicitly.

// When we run this program, we’ll see the following output:

// 我们在 CustomSmartPointer 上实现了 Drop 特性，并为调用 println! 的 drop 方法提供了一个实现。
// drop 函数的主体是放置当您的类型的实例超出范围时要运行的任何逻辑的地方。
// 我们在这里打印一些文本以直观地演示 Rust 何时调用 drop。

// 在 main 中，我们创建了两个 CustomSmartPointer 实例，然后打印创建的 CustomSmartPointers。
// 在 main 的末尾，我们的 CustomSmartPointer 实例将超出范围，Rust 将调用我们放在 drop 方法中的代码，打印我们的最终消息。
// 请注意，我们不需要显式调用 drop 方法。

// 当我们运行这个程序时，我们会看到以下输出：

// $ cargo run
//    Compiling drop-example v0.1.0 (file:///projects/drop-example)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.60s
//      Running `target/debug/drop-example`
// CustomSmartPointers created.
// Dropping CustomSmartPointer with data `other stuff`!
// Dropping CustomSmartPointer with data `my stuff`!

// Rust automatically called drop for us when our instances went out of scope, calling the code we specified.
// 当我们的实例超出范围时，Rust 自动为我们调用 drop，调用我们指定的代码。
// Variables are dropped in the reverse order of their creation, so d was dropped before c.
// 变量的删除顺序与其创建顺序相反，因此 d 在 c 之前被删除。
// This example’s purpose is to give you a visual guide to how the drop method works;
// 这个例子的目的是给你一个直观的指导，让你了解 drop 方法是如何工作的；
// usually you would specify the cleanup code that your type needs to run rather than a print message.
// 通常你会指定你的类型需要运行的清理代码而不是打印消息。

// Dropping a Value Early with std::mem::drop
// 使用 std::mem::drop 提前删除一个值
// Unfortunately, it’s not straightforward to disable the automatic drop functionality.
// 不幸的是，禁用自动放置功能（自动插入drop函数的代码）并不简单。
// Disabling drop isn’t usually necessary; the whole point of the Drop trait is that it’s taken care of automatically.
// 通常不需要禁用 drop； Drop trait 的全部意义在于它是自动处理的。
// Occasionally, however, you might want to clean up a value early.
// 但是，有时您可能希望尽早清理一个值。
// One example is when using smart pointers that manage locks: you might want to force the drop method that releases the lock so that other code in the same scope can acquire the lock.
// 一个例子是当使用管理锁的智能指针时：您可能想要强制释放锁的 drop 方法，以便同一范围内的其他代码可以获得锁。
// Rust doesn’t let you call the Drop trait’s drop method manually;
// Rust 不允许您手动调用 Drop 特性的 drop 方法；
// instead you have to call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.
// 相反，如果你想强制一个值在其范围结束之前被删除，你必须调用标准库提供的 std::mem::drop 函数。

// If we try to call the Drop trait’s drop method manually by modifying the main function from Listing 15-14, as shown in Listing 15-15, we’ll get a compiler error:
// 如果我们尝试通过修改示例 15-14 中的 main 函数来手动调用 Drop trait 的 drop 方法，如示例 15-15 所示，我们将得到一个编译器错误：

// Filename: src/main.rs

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}

// Listing 15-15: Attempting to call the drop method from the Drop trait manually to clean up early
// 示例 15-15：尝试从 Drop 特性中手动调用 drop 方法以提前清理

// When we try to compile this code, we’ll get this error:
// 当我们尝试编译这段代码时，我们会得到这个错误：

// $ cargo run
//    Compiling drop-example v0.1.0 (file:///projects/drop-example)
// error[E0040]: explicit use of destructor method
//   --> src/main.rs:16:7
//    |
// 16 |     c.drop();
//    |     --^^^^--
//    |     | |
//    |     | explicit destructor calls not allowed
//    |     help: consider using `drop` function: `drop(c)`
//
// For more information about this error, try `rustc --explain E0040`.
// error: could not compile `drop-example` due to previous error

// This error message states that we’re not allowed to explicitly call drop.
// 此错误消息指出我们不允许显式调用 drop。
// The error message uses the term destructor, which is the general programming term for a function that cleans up an instance.
// 错误消息使用术语析构函数，这是用于清理实例的函数的通用编程术语。
// A destructor is analogous to a constructor, which creates an instance. The drop function in Rust is one particular destructor.
// 析构函数类似于构造函数，它创建一个实例。 Rust 中的 drop 函数是一个特殊的析构函数。

// Rust doesn’t let us call drop explicitly because Rust would still automatically call drop on the value at the end of main.
// Rust 不允许我们显式地调用 drop，因为 Rust 仍然会在 main 的末尾自动调用 drop。
// This would cause a double free error because Rust would be trying to clean up the same value twice.
// 这会导致双重释放错误，因为 Rust 会尝试两次清理相同的值。

// We can’t disable the automatic insertion of drop when a value goes out of scope, and we can’t call the drop method explicitly.
// 当值超出范围时，我们不能禁用 drop 的自动插入，也不能显式调用 drop 方法。
// So, if we need to force a value to be cleaned up early, we use the std::mem::drop function.
// 所以，如果我们需要强制提前清理一个值，我们使用 std::mem::drop 函数。

// The std::mem::drop function is different from the drop method in the Drop trait. We call it by passing as an argument the value we want to force drop.
// std::mem::drop 函数不同于 Drop trait 中的 drop 方法。 我们通过将要强制删除的值作为参数传递来调用它。
// The function is in the prelude, so we can modify main in Listing 15-15 to call the drop function, as shown in Listing 15-16:
// 该函数在序曲中，因此我们可以修改示例 15-15 中的 main 来调用 drop 函数，如示例 15-16 所示：

// Filename: src/main.rs

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

// Listing 15-16: Calling std::mem::drop to explicitly drop a value before it goes out of scope
// 示例 15-16：调用 std::mem::drop 以在值超出范围之前显式删除该值

// Running this code will print the following:
// 运行此代码将打印以下内容：

// $ cargo run
//    Compiling drop-example v0.1.0 (file:///projects/drop-example)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.73s
//      Running `target/debug/drop-example`
// CustomSmartPointer created.
// Dropping CustomSmartPointer with data `some data`!
// CustomSmartPointer dropped before the end of main.

// The text Dropping CustomSmartPointer with data `some data`! is printed between the CustomSmartPointer created.
// 文本 Dropping CustomSmartPointer with data `some data`! 在创建的 CustomSmartPointer 之间打印。
// and CustomSmartPointer dropped before the end of main. text, showing that the drop method code is called to drop c at that point.
// 和 CustomSmartPointer 在 main 结束之前下降。 文本，表明此时调用了 drop 方法代码来删除 c。

// You can use code specified in a Drop trait implementation in many ways to make cleanup convenient and safe: for instance, you could use it to create your own memory allocator!
// 您可以通过多种方式使用 Drop trait 实现中指定的代码来方便和安全地进行清理：例如，您可以使用它来创建自己的内存分配器！
// With the Drop trait and Rust’s ownership system, you don’t have to remember to clean up because Rust does it automatically.
// 使用 Drop trait 和 Rust 的所有权系统，你不必记得清理，因为 Rust 会自动完成。

// You also don’t have to worry about problems resulting from accidentally cleaning up values still in use:
// 您也不必担心因意外清理仍在使用的值而导致的问题：
//  the ownership system that makes sure references are always valid also ensures that drop gets called only once when the value is no longer being used.
//  确保引用始终有效的所有权系统还确保在不再使用该值时仅调用一次 drop。

// Now that we’ve examined Box<T> and some of the characteristics of smart pointers, let’s look at a few other smart pointers defined in the standard library.
// 现在我们已经研究了 Box<T> 和智能指针的一些特性，让我们看看标准库中定义的其他一些智能指针。


// Rc<T>, the Reference Counted Smart Pointer
// Rc<T>，引用计数智能指针
// In the majority of cases, ownership is clear: you know exactly which variable owns a given value.
// 在大多数情况下，所有权是明确的：您确切地知道哪个变量拥有给定值。
// However, there are cases when a single value might have multiple owners.
// 但是，在某些情况下，单个值可能有多个所有者。
// For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it.
// 例如，在图形数据结构中，多条边可能指向同一个节点，并且该节点在概念上由所有指向它的边拥有。
// A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it and so has no owners.
// 一个节点不应该被清理，除非它没有任何边指向它，因此没有所有者。

// You have to enable multiple ownership explicitly by using the Rust type Rc<T>, which is an abbreviation for reference counting.
// 您必须使用 Rust 类型 Rc<T> 显式启用多重所有权，Rc<T> 是引用计数的缩写。
// The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use.
// Rc<T> 类型跟踪对值的引用次数以确定该值是否仍在使用中。
// If there are zero references to a value, the value can be cleaned up without any references becoming invalid.
// 如果对某个值的引用为零，则可以在没有任何引用变为无效的情况下清除该值。

// Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they turn it on. Others can come into the room and watch the TV.
// 将 Rc<T> 想象成家庭房间里的电视。 当一个人进来看电视时，他们会打开电视。 其他人可以进入房间看电视。
// When the last person leaves the room, they turn off the TV because it’s no longer being used.
// 当最后一个人离开房间时，他们关掉了电视，因为它不再被使用了。
// If someone turns off the TV while others are still watching it, there would be uproar from the remaining TV watchers!
// 如果有人在其他人还在看电视的情况下关掉电视，剩下的电视观众就会哗然！

// We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last.
// 当我们想在堆上分配一些数据供程序的多个部分读取并且我们无法在编译时确定哪一部分最后使用数据时，我们使用 Rc<T> 类型。
// If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.
// 如果我们知道哪个部分最后完成，我们就可以让那个部分成为数据的所有者，编译时强制执行的正常所有权规则就会生效。

// Note that Rc<T> is only for use in single-threaded scenarios.
// 注意 Rc<T> 仅用于单线程场景。
// When we discuss concurrency in Chapter 16, we’ll cover how to do reference counting in multithreaded programs.
// 当我们在第 16 章讨论并发时，我们将介绍如何在多线程程序中进行引用计数。

// Using Rc<T> to Share Data
// 使用 Rc<T> 共享数据
// Let’s return to our cons list example in Listing 15-5. Recall that we defined it using Box<T>.
// 让我们回到示例 15-5 中的 cons 列表示例。 回想一下，我们使用 Box<T> 定义了它。
// This time, we’ll create two lists that both share ownership of a third list.
// 这一次，我们将创建两个列表，它们都共享第三个列表的所有权。
// Conceptually, this looks similar to Figure 15-3:
// 从概念上讲，这看起来类似于图 15-3：

// b -> 3 -->-
//           |
//      a -> 5 -> 10 -> Nil
//           |
// c -> 4 -->-

// Figure 15-3: Two lists, b and c, sharing ownership of a third list, a
// 图 15-3：两个列表 b 和 c，共享第三个列表 a 的所有权

// We’ll create list a that contains 5 and then 10. Then we’ll make two more lists: b that starts with 3 and c that starts with 4.
// 我们将创建包含 5 和 10 的列表 a。然后我们将创建另外两个列表：以 3 开头的 b 和以 4 开头的 c。
// Both b and c lists will then continue on to the first a list containing 5 and 10.
// 然后 b 和 c 列表将继续到第一个包含 5 和 10 的列表。
// In other words, both lists will share the first list containing 5 and 10.
// 换句话说，两个列表将共享包含 5 和 10 的第一个列表。

// Trying to implement this scenario using our definition of List with Box<T> won’t work, as shown in Listing 15-17:
// 尝试使用带有 Box<T> 的 List 定义来实现这个场景是行不通的，如示例 15-17 所示：

// Filename: src/main.rs

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}

// Listing 15-17: Demonstrating we’re not allowed to have two lists using Box<T> that try to share ownership of a third list
// 示例 15-17：演示我们不允许使用 Box<T> 的两个列表尝试共享第三个列表的所有权

// When we compile this code, we get this error:
// 当我们编译这段代码时，我们得到这个错误：

// $ cargo run
//    Compiling cons-list v0.1.0 (file:///projects/cons-list)
// error[E0382]: use of moved value: `a`
//   --> src/main.rs:11:30
//    |
// 9  |     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//    |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
// 10 |     let b = Cons(3, Box::new(a));
//    |                              - value moved here
// 11 |     let c = Cons(4, Box::new(a));
//    |                              ^ value used here after move
//
// For more information about this error, try `rustc --explain E0382`.
// error: could not compile `cons-list` due to previous error

// The Cons variants own the data they hold, so when we create the b list, a is moved into b and b owns a.
// Cons 变体拥有它们持有的数据，因此当我们创建 b 列表时，a 被移动到 b 并且 b 拥有 a。
// Then, when we try to use a again when creating c, we’re not allowed to because a has been moved.
// 然后，当我们尝试在创建 c 时再次使用 a 时，我们不被允许，因为 a 已经被移动了。

// We could change the definition of Cons to hold references instead, but then we would have to specify lifetime parameters.
// 我们可以更改 Cons 的定义来保存引用，但是我们必须指定生命周期参数。
// By specifying lifetime parameters, we would be specifying that every element in the list will live at least as long as the entire list.
// 通过指定生命周期参数，我们将指定列表中的每个元素至少与整个列表一样长。
// This is the case for the elements and lists in Listing 15-17, but not in every scenario.
// 示例 15-17 中的元素和列表就是这种情况，但并非在所有情况下都如此。

// Instead, we’ll change our definition of List to use Rc<T> in place of Box<T>, as shown in Listing 15-18.
// 相反，我们将更改 List 的定义以使用 Rc<T> 代替 Box<T>，如示例 15-18 所示。
// Each Cons variant will now hold a value and an Rc<T> pointing to a List.
// 每个 Cons 变体现在将包含一个值和一个指向列表的 Rc<T>。
// When we create b, instead of taking ownership of a, we’ll clone the Rc<List> that a is holding,
// 当我们创建 b 时，我们不会获取 a 的所有权，而是克隆 a 持有的 Rc<List>，
//  thereby increasing the number of references from one to two and letting a and b share ownership of the data in that Rc<List>.
// 从而将引用的数量从一个增加到两个，并让 a 和 b 共享 Rc<List> 中数据的所有权。
// We’ll also clone a when creating c, increasing the number of references from two to three.
// 我们还将在创建 c 时克隆 a，将引用的数量从两个增加到三个。
// Every time we call Rc::clone, the reference count to the data within the Rc<List> will increase, and the data won’t be cleaned up unless there are zero references to it.
// 每次我们调用 Rc::clone 时，Rc<List> 中数据的引用计数都会增加，除非对它的引用为零，否则数据不会被清理。

// Filename: src/main.rs

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

// Listing 15-18: A definition of List that uses Rc<T>
// 示例 15-18：使用 Rc<T> 的 List 定义

// We need to add a use statement to bring Rc<T> into scope because it’s not in the prelude.
// 我们需要添加一个 use 语句来将 Rc<T> 引入作用域，因为它不在前奏中。
// In main, we create the list holding 5 and 10 and store it in a new Rc<List> in a.
// 在 main 中，我们创建包含 5 和 10 的列表，并将其存储在 a 中的新 Rc<List> 中。
// Then when we create b and c, we call the Rc::clone function and pass a reference to the Rc<List> in a as an argument.
// 然后当我们创建 b 和 c 时，我们调用 Rc::clone 函数并将对 a 中的 Rc<List> 的引用作为参数传递。

// We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use Rc::clone in this case.
// 我们可以调用 a.clone() 而不是 Rc::clone(&a)，但 Rust 的约定是在这种情况下使用 Rc::clone。
// The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do.
// Rc::clone 的实现不像大多数类型的 clone 实现那样对所有数据进行深度复制。
// The call to Rc::clone only increments the reference count, which doesn’t take much time. Deep copies of data can take a lot of time.
// 对 Rc::clone 的调用只会增加引用计数，这不会花费太多时间。 数据的深拷贝可能会花费很多时间。
// By using Rc::clone for reference counting, we can visually distinguish between the deep-copy kinds of clones and the kinds of clones that increase the reference count.
// 通过使用Rc::clone 进行引用计数，我们可以直观的区分深拷贝克隆和增加引用计数的克隆。
// When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to Rc::clone.
// 在代码中寻找性能问题时，我们只需要考虑深拷贝克隆，可以忽略对 Rc::clone 的调用。

// Cloning an Rc<T> Increases the Reference Count
// 克隆一个 Rc<T> 增加引用计数
// Let’s change our working example in Listing 15-18 so we can see the reference counts changing as we create and drop references to the Rc<List> in a.
// 让我们更改示例 15-18 中的工作示例，以便我们可以看到引用计数在我们创建和删除对 a 中的 Rc<List> 的引用时发生变化。

// In Listing 15-19, we’ll change main so it has an inner scope around list c; then we can see how the reference count changes when c goes out of scope.
// 在示例 15-19 中，我们将更改 main，使其具有围绕列表 c 的内部作用域； 然后我们可以看到当 c 超出范围时引用计数如何变化。

// Filename: src/main.rs

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// Listing 15-19: Printing the reference count

// At each point in the program where the reference count changes, we print the reference count, which we get by calling the Rc::strong_count function.
// 在程序中引用计数发生变化的每个点，我们打印引用计数，这是通过调用 Rc::strong_count 函数获得的。
// This function is named strong_count rather than count because the Rc<T> type also has a weak_count;
// 这个函数被命名为 strong_count 而不是 count 因为 Rc<T> 类型也有一个 weak_count；
// we’ll see what weak_count is used for in the “Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>” section.
// 我们将在“防止引用循环：将 Rc<T> 转换为 Weak<T>”部分中了解 weak_count 的用途。

// This code prints the following:
// 此代码打印以下内容：

// $ cargo run
//    Compiling cons-list v0.1.0 (file:///projects/cons-list)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.45s
//      Running `target/debug/cons-list`
// count after creating a = 1
// count after creating b = 2
// count after creating c = 3
// count after c goes out of scope = 2

// We can see that the Rc<List> in a has an initial reference count of 1; then each time we call clone, the count goes up by 1. When c goes out of scope, the count goes down by 1.
// 我们可以看到a中的Rc<List>初始引用计数为1； 然后每次我们调用克隆时，计数都会增加 1。当 c 超出范围时，计数会减少 1。
// We don’t have to call a function to decrease the reference count like we have to call Rc::clone to increase the reference count:
// 我们不必像必须调用 Rc::clone 来增加引用计数那样调用函数来减少引用计数：
//  the implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.
//  当 Rc<T> 值超出范围时，Drop 特性的实现会自动减少引用计数。

// What we can’t see in this example is that when b and then a go out of scope at the end of main, the count is then 0, and the Rc<List> is cleaned up completely.
// 在这个例子中我们看不到的是，当 b 和 a 在 main 的末尾超出范围时，计数为 0，Rc<List> 被完全清理。
// Using Rc<T> allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of the owners still exist.
// 使用 Rc<T> 允许单个值拥有多个所有者，并且计数确保只要任何所有者仍然存在，该值就保持有效。

// Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only.
// 通过不可变引用，Rc<T> 允许您在程序的多个部分之间共享数据以供只读。
// If Rc<T> allowed you to have multiple mutable references too, you might violate one of the borrowing rules discussed in Chapter 4:
// 如果 Rc<T> 也允许你有多个可变引用，你可能会违反第 4 章中讨论的借用规则之一：
//  multiple mutable borrows to the same place can cause data races and inconsistencies.
//  对同一个地方的多个可变借用会导致数据竞争和不一致。
// But being able to mutate data is very useful! In the next section,
// 但是能够改变数据是非常有用的！ 在下一节中，
//  we’ll discuss the interior mutability pattern and the RefCell<T> type that you can use in conjunction with an Rc<T> to work with this immutability restriction.
//  我们将讨论内部可变性模式和 RefCell<T> 类型，您可以将其与 Rc<T> 结合使用以应对这种不可变性限制。

// RefCell<T> and the Interior Mutability Pattern
// RefCell<T> 和内部可变性模式
// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data;
// 内部可变性是 Rust 中的一种设计模式，即使存在对该数据的不可变引用，它也允许您改变数据；
// normally, this action is disallowed by the borrowing rules.
// 通常，借用规则不允许此操作。
// To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.
// 为了改变数据，该模式在数据结构中使用不安全代码来改变 Rust 管理改变和借用的通常规则。
// Unsafe code indicates to the compiler that we’re checking the rules manually instead of relying on the compiler to check them for us;
// 不安全代码向编译器表明我们正在手动检查规则，而不是依赖编译器为我们检查它们；
// we will discuss unsafe code more in Chapter 19.
// 我们将在第 19 章中更多地讨论不安全代码。

// We can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can’t guarantee that.
// 只有当我们可以确保在运行时遵循借用规则时，我们才能使用使用内部可变性模式的类型，即使编译器不能保证这一点。
// The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.
// 所涉及的不安全代码被包装在安全的 API 中，外部类型仍然是不可变的。

// Let’s explore this concept by looking at the RefCell<T> type that follows the interior mutability pattern.
// 让我们通过查看遵循内部可变性模式的 RefCell<T> 类型来探索这个概念。

// Enforcing Borrowing Rules at Runtime with RefCell<T>
// 使用 RefCell<T> 在运行时执行借用规则
// Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds.
// 与 Rc<T> 不同，RefCell<T> 类型表示对其所持有数据的单一所有权。
// So, what makes RefCell<T> different from a type like Box<T>? Recall the borrowing rules you learned in Chapter 4:
// 那么，是什么让 RefCell<T> 不同于像 Box<T> 这样的类型？ 回想一下您在第 4 章中学到的借用规则：

// At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
// 在任何给定时间，您可以拥有一个（但不能同时拥有）一个可变引用或任意数量的不可变引用。
// References must always be valid.
// 引用必须始终有效。

// With references and Box<T>, the borrowing rules’ invariants are enforced at compile time.
// 对于引用和 Box<T>，借用规则的不变量在编译时强制执行。
// With RefCell<T>, these invariants are enforced at runtime.
// 使用 RefCell<T>，这些不变量在运行时强制执行。
// With references, if you break these rules, you’ll get a compiler error. With RefCell<T>, if you break these rules, your program will panic and exit.
// 对于引用，如果你违反这些规则，你将得到一个编译器错误。 对于 RefCell<T>，如果您违反这些规则，您的程序将崩溃并退出。

// The advantages of checking the borrowing rules at compile time are that errors will be caught sooner in the development process,
// 在编译时检查借用规则的好处是在开发过程中会更快地发现错误，
//  and there is no impact on runtime performance because all the analysis is completed beforehand.
//  并且对运行时性能没有影响，因为所有分析都是预先完成的。
// For those reasons, checking the borrowing rules at compile time is the best choice in the majority of cases, which is why this is Rust’s default.
// 由于这些原因，在大多数情况下，在编译时检查借用规则是最好的选择，这就是为什么这是 Rust 的默认设置。

// The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, where they would’ve been disallowed by the compile-time checks.
// 相反，在运行时检查借用规则的优点是允许某些内存安全的场景，而编译时检查不允许这些场景。
// Static analysis, like the Rust compiler, is inherently conservative.
// 静态分析，就像 Rust 编译器一样，本质上是保守的。
// Some properties of code are impossible to detect by analyzing the code: the most famous example is the Halting Problem, which is beyond the scope of this book but is an interesting topic to research.
// 代码的某些属性无法通过分析代码来检测：最著名的例子是停机问题，这超出了本书的范围，但却是一个值得研究的有趣主题。

// Because some analysis is impossible, if the Rust compiler can’t be sure the code complies with the ownership rules, it might reject a correct program; in this way, it’s conservative.
// 因为某些分析是不可能的，如果 Rust 编译器不能确定代码是否符合所有权规则，它可能会拒绝一个正确的程序； 这样，它是保守的。
// If Rust accepted an incorrect program, users wouldn’t be able to trust in the guarantees Rust makes.
// 如果 Rust 接受了一个不正确的程序，用户将无法相信 Rust 做出的保证。
// However, if Rust rejects a correct program, the programmer will be inconvenienced, but nothing catastrophic can occur.
// 但是，如果 Rust 拒绝一个正确的程序，程序员会很不方便，但不会发生任何灾难。
// The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.
// 当您确定您的代码遵循借用规则但编译器无法理解和保证时，RefCell<T> 类型很有用。

// Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.
// 与 Rc<T> 类似，RefCell<T> 仅适用于单线程场景，如果您尝试在多线程上下文中使用它，将会出现编译时错误。
// We’ll talk about how to get the functionality of RefCell<T> in a multithreaded program in Chapter 16.
// 我们将在第 16 章讨论如何在多线程程序中获取 RefCell<T> 的功能。

// Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
// 以下是选择 Box<T>、Rc<T> 或 RefCell<T> 的原因的概述：

// Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
// Rc<T> 允许同一数据的多个所有者； Box<T> 和 RefCell<T> 具有单一所有者。
// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
// Box<T> 允许在编译时检查不可变或可变借用； Rc<T> 只允许在编译时检查不可变借用； RefCell<T> 允许在运行时检查不可变或可变借用。
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
// 因为 RefCell<T> 允许在运行时检查可变借用，所以即使 RefCell<T> 是不可变的，您也可以改变 RefCell<T> 内的值。

// Mutating the value inside an immutable value is the interior mutability pattern.
// 改变不可变值内部的值是内部可变性模式。
// Let’s look at a situation in which interior mutability is useful and examine how it’s possible.
// 让我们看看内部可变性有用的情况，并检查它是如何可能的。

// Interior Mutability: A Mutable Borrow to an Immutable Value
// 内部可变性：对不可变值的可变借用
// A consequence of the borrowing rules is that when you have an immutable value, you can’t borrow it mutably. For example, this code won’t compile:
// 借用规则的结果是当你有一个不可变的值时，你不能可变地借用它。 例如，这段代码将无法编译：

fn main() {
    let x = 5;
    let y = &mut x;
}

// If you tried to compile this code, you’d get the following error:
// 如果你试图编译这段代码，你会得到以下错误：

// $ cargo run
//    Compiling borrowing v0.1.0 (file:///projects/borrowing)
// error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
//  --> src/main.rs:3:13
//   |
// 2 |     let x = 5;
//   |         - help: consider changing this to be mutable: `mut x`
// 3 |     let y = &mut x;
//   |             ^^^^^^ cannot borrow as mutable
//
// For more information about this error, try `rustc --explain E0596`.
// error: could not compile `borrowing` due to previous error

// However, there are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code.
// 然而，在某些情况下，值在其方法中改变自身是有用的，但对其他代码来说似乎是不可变的。
// Code outside the value’s methods would not be able to mutate the value.
// 值方法之外的代码将无法改变该值。
// Using RefCell<T> is one way to get the ability to have interior mutability, but RefCell<T> doesn’t get around the borrowing rules completely:
// 使用 RefCell<T> 是获得内部可变性能力的一种方法，但 RefCell<T> 并没有完全绕过借用规则：
//  the borrow checker in the compiler allows this interior mutability, and the borrowing rules are checked at runtime instead.
// 编译器中的借用检查器允许这种内部可变性，而是在运行时检查借用规则。
// If you violate the rules, you’ll get a panic! instead of a compiler error.
// 如果你违反规则，你会感到恐慌！ 而不是编译器错误。

// Let’s work through a practical example where we can use RefCell<T> to mutate an immutable value and see why that is useful.
// 让我们通过一个实际的例子来工作，我们可以在其中使用 RefCell<T> 来改变一个不可变的值，看看为什么它有用。

// A Use Case for Interior Mutability: Mock Objects
// 内部可变性的用例：模拟对象
// Sometimes during testing a programmer will use a type in place of another type, in order to observe particular behavior and assert it’s implemented correctly.
// 有时在测试期间，程序员会使用一种类型代替另一种类型，以观察特定行为并断言它已正确实现。
// This placeholder type is called a test double.
// 此占位符类型称为测试替身。
// Think of it in the sense of a “stunt double” in filmmaking, where a person steps in and substitutes for an actor to do a particular tricky scene.
// 从电影制作中的“特技替身”的意义上来说，一个人介入并代替演员来完成一个特别棘手的场景。
// Test doubles stand in for other types when we’re running tests.
// 当我们运行测试时，测试替身代表其他类型。
// Mock objects are specific types of test doubles that record what happens during a test so you can assert that the correct actions took place.
// Mock 对象是特定类型的测试替身，它记录测试期间发生的事情，因此您可以断言发生了正确的操作。

// Rust doesn’t have objects in the same sense as other languages have objects, and Rust doesn’t have mock object functionality built into the standard library as some other languages do.
// Rust 没有与其他语言拥有对象相同意义上的对象，并且 Rust 没有像其他一些语言那样将模拟对象功能内置到标准库中。
// However, you can definitely create a struct that will serve the same purposes as a mock object.
// 但是，您绝对可以创建一个与模拟对象具有相同用途的结构。

// Here’s the scenario we’ll test: we’ll create a library that tracks a value against a maximum value and sends messages based on how close to the maximum value the current value is.
// 这是我们要测试的场景：我们将创建一个库来跟踪一个值与最大值的对比，并根据当前值与最大值的接近程度发送消息。
// This library could be used to keep track of a user’s quota for the number of API calls they’re allowed to make, for example.
// 例如，此库可用于跟踪用户的 API 调用数量配额。

// Our library will only provide the functionality of tracking how close to the maximum a value is and what the messages should be at what times.
// 我们的库将只提供跟踪一个值与最大值的接近程度以及在什么时间应该是什么消息的功能。
// Applications that use our library will be expected to provide the mechanism for sending the messages:
// 使用我们库的应用程序需要提供发送消息的机制：
//  the application could put a message in the application, send an email, send a text message, or something else.
//  应用程序可以在应用程序中放置消息、发送电子邮件、发送文本消息或其他内容。
// The library doesn’t need to know that detail.
// 库不需要知道那个细节。
// All it needs is something that implements a trait we’ll provide called Messenger.
// 它所需要的只是实现我们将提供的称为 Messenger 的特征的东西。
// Listing 15-20 shows the library code:
// 清单 15-20 显示了库代码：

// Filename: src/lib.rs

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// Listing 15-20: A library to keep track of how close a value is to a maximum value and warn when the value is at certain levels
// 示例 15-20：一个用于跟踪值与最大值的接近程度并在该值达到特定水平时发出警告的库

// One important part of this code is that the Messenger trait has one method called send that takes an immutable reference to self and the text of the message.
// 这段代码的一个重要部分是 Messenger trait 有一个名为 send 的方法，它采用对 self 和消息文本的不可变引用。
// This trait is the interface our mock object needs to implement so that the mock can be used in the same way a real object is.
// 这个特征是我们的模拟对象需要实现的接口，这样模拟对象就可以像真实对象一样使用。
// The other important part is that we want to test the behavior of the set_value method on the LimitTracker.
// 另一个重要部分是我们要测试 LimitTracker 上的 set_value 方法的行为。
// We can change what we pass in for the value parameter, but set_value doesn’t return anything for us to make assertions on.
// 我们可以更改为 value 参数传入的内容，但 set_value 不会返回任何内容供我们进行断言。
// We want to be able to say that if we create a LimitTracker with something that implements the Messenger trait and a particular value for max,
// 我们希望能够说，如果我们创建一个 LimitTracker，其中包含实现 Messenger 特性和特定最大值的东西，
//  when we pass different numbers for value, the messenger is told to send the appropriate messages.
//  当我们为值传递不同的数字时，信使被告知发送适当的消息。

// We need a mock object that, instead of sending an email or text message when we call send, will only keep track of the messages it’s told to send.
// 我们需要一个模拟对象，它不会在我们调用发送时发送电子邮件或短信，而只会跟踪它被告知要发送的消息。
// We can create a new instance of the mock object, create a LimitTracker that uses the mock object, call the set_value method on LimitTracker,
// 我们可以创建模拟对象的新实例，创建一个使用模拟对象的 LimitTracker，调用 LimitTracker 的 set_value 方法，
//  and then check that the mock object has the messages we expect.
//  然后检查模拟对象是否包含我们期望的消息。
// Listing 15-21 shows an attempt to implement a mock object to do just that, but the borrow checker won’t allow it:
// 示例 15-21 展示了实现模拟对象的尝试，但借用检查器不允许这样做：

// Filename: src/lib.rs

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}

// Listing 15-21: An attempt to implement a MockMessenger that isn’t allowed by the borrow checker
// 示例 15-21：尝试实现借用检查器不允许的 MockMessenger

// This test code defines a MockMessenger struct that has a sent_messages field with a Vec of String values to keep track of the messages it’s told to send.
// 这个测试代码定义了一个 MockMessenger 结构，它有一个 sent_messages 字段和一个 String 值的 Vec 来跟踪它被告知要发送的消息。
// We also define an associated function new to make it convenient to create new MockMessenger values that start with an empty list of messages.
// 我们还定义了一个关联函数 new，以便于创建以空消息列表开头的新 MockMessenger 值。
// We then implement the Messenger trait for MockMessenger so we can give a MockMessenger to a LimitTracker.
// 然后我们为 MockMessenger 实现 Messenger 特性，这样我们就可以将 MockMessenger 赋予 LimitTracker。
// In the definition of the send method, we take the message passed in as a parameter and store it in the MockMessenger list of sent_messages.
// 在send方法的定义中，我们将传入的message作为参数，存放在sent_messages的MockMessenger列表中。

// In the test, we’re testing what happens when the LimitTracker is told to set value to something that is more than 75 percent of the max value.
// 在测试中，我们正在测试当 LimitTracker 被告知将值设置为超过最大值的 75% 时会发生什么。
// First, we create a new MockMessenger, which will start with an empty list of messages.
// 首先，我们创建一个新的 MockMessenger，它将以一个空的消息列表开始。
// Then we create a new LimitTracker and give it a reference to the new MockMessenger and a max value of 100.
// 然后我们创建一个新的 LimitTracker 并为其提供对新 MockMessenger 的引用和最大值 100。
// We call the set_value method on the LimitTracker with a value of 80, which is more than 75 percent of 100.
// 我们在 LimitTracker 上调用 set_value 方法，值为 80，超过 100 的 75%。
// Then we assert that the list of messages that the MockMessenger is keeping track of should now have one message in it.
// 然后我们断言 MockMessenger 正在跟踪的消息列表现在应该有一条消息。

// However, there’s one problem with this test, as shown here:
// 然而，这个测试有一个问题，如下所示：

// $ cargo test
//    Compiling limit-tracker v0.1.0 (file:///projects/limit-tracker)
// error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
//   --> src/lib.rs:58:13
//    |
// 2  |     fn send(&self, msg: &str);
//    |             ----- help: consider changing that to be a mutable reference: `&mut self`
// ...
// 58 |             self.sent_messages.push(String::from(message));
//    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//
// For more information about this error, try `rustc --explain E0596`.
// error: could not compile `limit-tracker` due to previous error
// warning: build failed, waiting for other jobs to finish...

// We can’t modify the MockMessenger to keep track of the messages, because the send method takes an immutable reference to self.
// 我们不能修改 MockMessenger 来跟踪消息，因为发送方法采用对 self 的不可变引用。
// We also can’t take the suggestion from the error text to use &mut self instead,
// 我们也不能接受错误文本中的建议来使用 &mut self，
//  because then the signature of send wouldn’t match the signature in the Messenger trait definition (feel free to try and see what error message you get).
//  因为 send 的签名与 Messenger 特征定义中的签名不匹配（随意尝试看看你得到了什么错误消息）。

// This is a situation in which interior mutability can help!
// 这是内部可变性可以提供帮助的情况！
// We’ll store the sent_messages within a RefCell<T>,
// 我们将 sent_messages 存储在 RefCell<T> 中，
//  and then the send method will be able to modify sent_messages to store the messages we’ve seen.
//  然后 send 方法将能够修改 sent_messages 来存储我们看到的消息。
// Listing 15-22 shows what that looks like:
// 清单 15-22 显示了它的样子：

// Filename: src/lib.rs

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// Listing 15-22: Using RefCell<T> to mutate an inner value while the outer value is considered immutable
// 示例 15-22：使用 RefCell<T> 改变内部值，而外部值被认为是不可变的

// The sent_messages field is now of type RefCell<Vec<String>> instead of Vec<String>.
// sent_messages 字段现在是 RefCell<Vec<String>> 类型，而不是 Vec<String>。
// In the new function, we create a new RefCell<Vec<String>> instance around the empty vector.
// 在新函数中，我们围绕空向量创建了一个新的 RefCell<Vec<String>> 实例。

// For the implementation of the send method, the first parameter is still an immutable borrow of self, which matches the trait definition.
// send方法的实现，第一个参数仍然是不可变的借用self，符合trait定义。
// We call borrow_mut on the RefCell<Vec<String>> in self.sent_messages to get a mutable reference to the value inside the RefCell<Vec<String>>, which is the vector.
// 我们在 self.sent_messages 中的 RefCell<Vec<String>> 上调用 borrow_mut 以获取对 RefCell<Vec<String>> 内部值的可变引用，即向量。
// Then we can call push on the mutable reference to the vector to keep track of the messages sent during the test.
// 然后我们可以对 vector 的可变引用调用 push 来跟踪测试期间发送的消息。

// The last change we have to make is in the assertion: to see how many items are in the inner vector, we call borrow on the RefCell<Vec<String>> to get an immutable reference to the vector.
// 我们必须做的最后一个更改是在断言中：为了查看内部向量中有多少项，我们在 RefCell<Vec<String>> 上调用 borrow 以获得对向量的不可变引用。

// Now that you’ve seen how to use RefCell<T>, let’s dig into how it works!
// 现在您已经了解了如何使用 RefCell<T>，让我们深入了解它的工作原理！

// Keeping Track of Borrows at Runtime with RefCell<T>
// 使用 RefCell<T> 在运行时跟踪借用
// When creating immutable and mutable references, we use the & and &mut syntax, respectively.
// 当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。
// With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API that belongs to RefCell<T>.
// 对于 RefCell<T>，我们使用 borrow 和 borrow_mut 方法，它们是属于 RefCell<T> 的安全 API 的一部分。
// The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>.
// borrow方法返回智能指针类型Ref<T>，borrow_mut返回智能指针类型RefMut<T>。
// Both types implement Deref, so we can treat them like regular references.
// 这两种类型都实现了 Deref，因此我们可以将它们视为常规引用。

// The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active.
// RefCell<T> 跟踪当前有多少 Ref<T> 和 RefMut<T> 智能指针处于活动状态。
// Every time we call borrow, the RefCell<T> increases its count of how many immutable borrows are active.
// 每次我们调用借用时，RefCell<T> 都会增加其对有多少不可变借用处于活动状态的计数。
// When a Ref<T> value goes out of scope, the count of immutable borrows goes down by one.
// 当 Ref<T> 值超出范围时，不可变借用的计数减一。
// Just like the compile-time borrowing rules, RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time.
// 就像编译时借用规则一样，RefCell<T> 让我们在任何时间点都有许多不可变借用或一个可变借用。

// If we try to violate these rules, rather than getting a compiler error as we would with references, the implementation of RefCell<T> will panic at runtime.
// 如果我们试图违反这些规则，而不是像处理引用那样得到编译器错误，RefCell<T> 的实现将在运行时出现混乱。
// Listing 15-23 shows a modification of the implementation of send in Listing 15-22.
// 清单 15-23 显示了清单 15-22 中发送实现的修改。
// We’re deliberately trying to create two mutable borrows active for the same scope to illustrate that RefCell<T> prevents us from doing this at runtime.
// 我们故意尝试为同一范围创建两个活动的可变借用，以说明 RefCell<T> 阻止我们在运行时执行此操作。

// Filename: src/lib.rs

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}

// Listing 15-23: Creating two mutable references in the same scope to see that RefCell<T> will panic
// 示例 15-23：在同一范围内创建两个可变引用以查看 RefCell<T> 会崩溃

// We create a variable one_borrow for the RefMut<T> smart pointer returned from borrow_mut.
// 我们为从 borrow_mut 返回的 RefMut<T> 智能指针创建一个变量 one_borrow。
// Then we create another mutable borrow in the same way in the variable two_borrow.
// 然后我们在变量 two_borrow 中以相同的方式创建另一个可变借用。
// This makes two mutable references in the same scope, which isn’t allowed.
// 这会在同一作用域内创建两个可变引用，这是不允许的。
// When we run the tests for our library, the code in Listing 15-23 will compile without any errors, but the test will fail:
// 当我们为我们的库运行测试时，清单 15-23 中的代码将编译无任何错误，但测试将失败：

// $ cargo test
//    Compiling limit-tracker v0.1.0 (file:///projects/limit-tracker)
//     Finished test [unoptimized + debuginfo] target(s) in 0.91s
//      Running unittests src/lib.rs (target/debug/deps/limit_tracker-e599811fa246dbde)
//
// running 1 test
// test tests::it_sends_an_over_75_percent_warning_message ... FAILED
//
// failures:
//
// ---- tests::it_sends_an_over_75_percent_warning_message stdout ----
// thread 'main' panicked at 'already borrowed: BorrowMutError', src/lib.rs:60:53
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
//
//
// failures:
//     tests::it_sends_an_over_75_percent_warning_message
//
// test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//
// error: test failed, to rerun pass '--lib'

// Notice that the code panicked with the message already borrowed: BorrowMutError.
// 请注意，代码因 already borrowed 的消息而恐慌：BorrowMutError。
// This is how RefCell<T> handles violations of the borrowing rules at runtime.
// 这就是 RefCell<T> 在运行时处理违反借用规则的方式。

// Choosing to catch borrowing errors at runtime rather than compile time,
// 选择在运行时而不是编译时捕获借用错误，
//  as we’ve done here, means you’d potentially be finding mistakes in your code later in the development process: possibly not until your code was deployed to production.
//  正如我们在这里所做的那样，这意味着您可能会在开发过程的后期发现代码中的错误：可能直到您的代码被部署到生产环境中。
// Also, your code would incur a small runtime performance penalty as a result of keeping track of the borrows at runtime rather than compile time.
// 此外，由于在运行时而不是编译时跟踪借用，您的代码会招致小的运行时性能损失。
// However, using RefCell<T> makes it possible to write a mock object that can modify itself to keep track of the messages it has seen while you’re using it in a context where only immutable values are allowed.
// 但是，使用 RefCell<T> 可以编写一个模拟对象，该对象可以修改自身以跟踪它在仅允许不可变值的上下文中使用它时看到的消息。
// You can use RefCell<T> despite its trade-offs to get more functionality than regular references provide.
// 您可以使用 RefCell<T>，尽管它需要权衡取舍以获得比常规引用提供的更多功能。

// Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
// 通过组合 Rc<T> 和 RefCell<T> 拥有可变数据的多个所有者
// A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable access to that data.
// 使用 RefCell<T> 的常见方法是结合 Rc<T>。 回想一下，Rc<T> 允许您拥有某些数据的多个所有者，但它只提供对该数据的不可变访问。
// If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate!
// 如果你有一个包含 RefCell<T> 的 Rc<T>，你可以获得一个可以有多个所有者并且你可以改变的值！

// For example, recall the cons list example in Listing 15-18 where we used Rc<T> to allow multiple lists to share ownership of another list.
// 例如，回想一下示例 15-18 中的 cons 列表示例，其中我们使用 Rc<T> 允许多个列表共享另一个列表的所有权。
// Because Rc<T> holds only immutable values, we can’t change any of the values in the list once we’ve created them.
// 因为 Rc<T> 只包含不可变值，所以一旦我们创建列表中的值，我们就不能更改它们。
// Let’s add in RefCell<T> to gain the ability to change the values in the lists. Listing 15-24 shows that by using a RefCell<T> in the Cons definition,
// 让我们添加 RefCell<T> 以获得更改列表中的值的能力。 清单 15-24 显示，通过在 Cons 定义中使用 RefCell<T>，
//  we can modify the value stored in all the lists:
//  我们可以修改存储在所有列表中的值：

// Filename: src/main.rs

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// Listing 15-24: Using Rc<RefCell<i32>> to create a List that we can mutate
// 示例 15-24：使用 Rc<RefCell<i32>> 创建一个我们可以改变的列表

// We create a value that is an instance of Rc<RefCell<i32>> and store it in a variable named value so we can access it directly later.
// 我们创建一个值，它是 Rc<RefCell<i32>> 的一个实例，并将其存储在一个名为 value 的变量中，以便我们稍后可以直接访问它。
// Then we create a List in a with a Cons variant that holds value.
// 然后我们在 a 中创建一个列表，其中包含一个保存值的 Cons 变体。
// We need to clone value so both a and value have ownership of the inner 5 value rather than transferring ownership from value to a or having a borrow from value.
// 我们需要克隆值，这样 a 和值都拥有内部 5 值的所有权，而不是将所有权从值转移到 a 或从值中借用。

// We wrap the list a in an Rc<T> so when we create lists b and c, they can both refer to a, which is what we did in Listing 15-18.
// 我们将列表 a 包装在 Rc<T> 中，这样当我们创建列表 b 和 c 时，它们都可以引用 a，这就是我们在示例 15-18 中所做的。

// After we’ve created the lists in a, b, and c, we want to add 10 to the value in value.
// 在我们创建了 a、b 和 c 中的列表之后，我们要将 10 添加到 value 中的值。
// We do this by calling borrow_mut on value, which uses the automatic dereferencing feature we discussed in Chapter 5 (see the section “Where’s the -> Operator?”) to dereference the Rc<T> to the inner RefCell<T> value.
// 我们通过对值调用 borrow_mut 来实现这一点，它使用我们在第 5 章中讨论的自动解引用功能（请参阅“-> 运算符在哪里？”一节）将 Rc<T> 解引用到内部 RefCell<T> 值 .
// The borrow_mut method returns a RefMut<T> smart pointer, and we use the dereference operator on it and change the inner value.
// borrow_mut 方法返回一个 RefMut<T> 智能指针，我们对其使用解引用运算符并更改内部值。

// When we print a, b, and c, we can see that they all have the modified value of 15 rather than 5:
// 当我们打印 a、b 和 c 时，我们可以看到它们的修改值都是 15 而不是 5：

// $ cargo run
//    Compiling cons-list v0.1.0 (file:///projects/cons-list)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.63s
//      Running `target/debug/cons-list`
// a after = Cons(RefCell { value: 15 }, Nil)
// b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
// c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))

// This technique is pretty neat! By using RefCell<T>, we have an outwardly immutable List value.
// But we can use the methods on RefCell<T> that provide access to its interior mutability so we can modify our data when we need to.
// The runtime checks of the borrowing rules protect us from data races, and it’s sometimes worth trading a bit of speed for this flexibility in our data structures.
// Note that RefCell<T> does not work for multithreaded code! Mutex<T> is the thread-safe version of RefCell<T> and we’ll discuss Mutex<T> in Chapter 16.


// Reference Cycles Can Leak Memory
// 引用循环会泄漏内存
// Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a memory leak).
// Rust 的内存安全保证很难但并非不可能意外创建从未清理过的内存（称为内存泄漏）。
// Preventing memory leaks entirely is not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust.
// 完全防止内存泄漏不是 Rust 的保证之一，这意味着内存泄漏在 Rust 中是内存安全的。
// We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to create references where items refer to each other in a cycle.
// 我们可以看到，Rust 通过使用 Rc<T> 和 RefCell<T> 允许内存泄漏：可以在循环中的项目相互引用的地方创建引用。
// This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.
// 这会造成内存泄漏，因为循环中每个项目的引用计数永远不会达到 0，并且永远不会删除值。

// Creating a Reference Cycle
// 创建引用循环
// Let’s look at how a reference cycle might happen and how to prevent it, starting with the definition of the List enum and a tail method in Listing 15-25:
// 让我们看看引用循环是如何发生的以及如何防止它，从示例 15-25 中的 List 枚举和 tail 方法的定义开始：

// Filename: src/main.rs

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {}

// Listing 15-25: A cons list definition that holds a RefCell<T> so we can modify what a Cons variant is referring to
// 示例 15-25：包含 RefCell<T> 的 cons 列表定义，因此我们可以修改 Cons 变体所引用的内容

// We’re using another variation of the List definition from Listing 15-5.
// 我们正在使用示例 15-5 中 List 定义的另一种变体。
// The second element in the Cons variant is now RefCell<Rc<List>>,
// Cons 变体中的第二个元素现在是 RefCell<Rc<List>>,
//  meaning that instead of having the ability to modify the i32 value as we did in Listing 15-24,
//  这意味着我们不像示例 15-24 那样能够修改 i32 值，
//  we want to modify the List value a Cons variant is pointing to.
//  我们想要修改 Cons 变体指向的列表值。
// We’re also adding a tail method to make it convenient for us to access the second item if we have a Cons variant.
// 我们还添加了一个 tail 方法，以便在我们有 Cons 变体时方便地访问第二项。

// In Listing 15-26, we’re adding a main function that uses the definitions in Listing 15-25.
// 在示例 15-26 中，我们添加了一个使用示例 15-25 中定义的主函数。
// This code creates a list in a and a list in b that points to the list in a.
// 此代码在 a 中创建一个列表，并在 b 中创建一个指向 a 中列表的列表。
// Then it modifies the list in a to point to b, creating a reference cycle.
// 然后它修改 a 中的列表以指向 b，创建一个引用循环。
// There are println! statements along the way to show what the reference counts are at various points in this process.
// 有println!沿途的语句以显示此过程中各个点的引用计数。

// Filename: src/main.rs

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // 取消注释下一行以查看我们有一个循环；
    // it will overflow the stack
    // 会溢出栈
    // println!("a next item = {:?}", a.tail());
}

// Listing 15-26: Creating a reference cycle of two List values pointing to each other
// 示例 15-26：创建两个 List 值相互指向的循环引用

// We create an Rc<List> instance holding a List value in the variable a with an initial list of 5, Nil.
// 我们创建一个 Rc<List> 实例，在变量 a 中保存一个 List 值，初始列表为 5（值），Nil。
// We then create an Rc<List> instance holding another List value in the variable b that contains the value 10 and points to the list in a.
// 然后我们创建一个 Rc<List> 实例，在变量 b 中保存另一个 List 值，该值包含值 10 并指向 a 中的列表。

// We modify a so it points to b instead of Nil, creating a cycle.
// 我们修改 a 使其指向 b 而不是 Nil，从而创建一个循环。
// We do that by using the tail method to get a reference to the RefCell<Rc<List>> in a, which we put in the variable link.
// 我们通过使用 tail 方法获取对 a 中的 RefCell<Rc<List>> 的引用来做到这一点，我们将其放入变量 link 中。
// Then we use the borrow_mut method on the RefCell<Rc<List>> to change the value inside from an Rc<List> that holds a Nil value to the Rc<List> in b.
// 然后我们使用 RefCell<Rc<List>> 上的 borrow_mut 方法将里面的值从一个持有 Nil 值的 Rc<List> 更改为 b 中的 Rc<List>。

// When we run this code, keeping the last println! commented out for the moment, we’ll get this output:
// 当我们运行这段代码时，保留最后的 println! 注释，我们将得到这个输出：

// $ cargo run
//    Compiling cons-list v0.1.0 (file:///projects/cons-list)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.53s
//      Running `target/debug/cons-list`
// a initial rc count = 1
// a next item = Some(RefCell { value: Nil })
// a rc count after b creation = 2
// b initial rc count = 1
// b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
// b rc count after changing a = 2
// a rc count after changing a = 2

// The reference count of the Rc<List> instances in both a and b are 2 after we change the list in a to point to b.
// 当我们将a中的列表改为指向b后，a和b中的Rc<List>实例的引用计数都是2。
// At the end of main, Rust drops the variable b, which decreases the reference count of the b Rc<List> instance from 2 to 1.
// 在 main 的末尾，Rust 删除了变量 b，这将 b Rc<List> 实例的引用计数从 2 减少到 1。
// The memory that Rc<List> has on the heap won’t be dropped at this point, because its reference count is 1, not 0.
// 此时 Rc<List> 在堆上的内存不会被丢弃，因为它的引用计数是 1，而不是 0。
// Then Rust drops a, which decreases the reference count of the a Rc<List> instance from 2 to 1 as well.
// 然后 Rust 删除 a，这也将 a Rc<List> 实例的引用计数从 2 减少到 1。
// This instance’s memory can’t be dropped either, because the other Rc<List> instance still refers to it.
// 这个实例的内存也不能被丢弃，因为另一个 Rc<List> 实例仍然引用它。
// The memory allocated to the list will remain uncollected forever. To visualize this reference cycle, we’ve created a diagram in Figure 15-4.
// 分配给列表的内存永远不会被回收。 为了可视化这个引用循环，我们在图 15-4 中创建了一个图表。

// a -> 5 -> 10 <- b
//      |    |
//      ----<-

// Figure 15-4: A reference cycle of lists a and b pointing to each other
// 图 15-4：列表 a 和 b 相互指向的循环引用

// If you uncomment the last println! and run the program, Rust will try to print this cycle with a pointing to b pointing to a and so forth until it overflows the stack.
// 如果你取消注释最后一个 println! 并运行程序，Rust 将尝试打印这个循环，其中 a 指向 b 指向 a 等等，直到它溢出堆栈。

// Compared to a real-world program, the consequences creating a reference cycle in this example aren’t very dire: right after we create the reference cycle, the program ends.
// 与真实世界的程序相比，在这个例子中创建引用循环的后果不是很可怕：在我们创建引用循环之后，程序就结束了。
// However, if a more complex program allocated lots of memory in a cycle and held onto it for a long time,
// 但是，如果一个更复杂的程序在一个循环中分配了大量内存并长时间占用它，
//  the program would use more memory than it needed and might overwhelm the system, causing it to run out of available memory.
//  程序会使用比它需要的更多的内存，并可能使系统不堪重负，导致它用完可用内存。

// Creating reference cycles is not easily done, but it’s not impossible either.
// 创建引用循环并不容易，但也不是不可能。
// If you have RefCell<T> values that contain Rc<T> values or similar nested combinations of types with interior mutability and reference counting,
// 如果您的 RefCell<T> 值包含 Rc<T> 值或具有内部可变性和引用计数的类似嵌套类型组合，
//  you must ensure that you don’t create cycles; you can’t rely on Rust to catch them.
// 你必须确保你不会创建循环； 你不能依赖 Rust 来捕捉它们。
// Creating a reference cycle would be a logic bug in your program that you should use automated tests, code reviews, and other software development practices to minimize.
// 创建引用循环将是程序中的逻辑错误，您应该使用自动化测试、代码审查和其他软件开发实践来最小化。

// Another solution for avoiding reference cycles is reorganizing your data structures so that some references express ownership and some references don’t.
// 另一个避免引用循环的解决方案是重组你的数据结构，让一些引用表达所有权而一些引用不表达所有权。
// As a result, you can have cycles made up of some ownership relationships and some non-ownership relationships,
// 因此，您可以拥有由一些所有权关系和一些非所有权关系组成的循环，
//  and only the ownership relationships affect whether or not a value can be dropped.
//  只有所有权关系会影响一个值是否可以被删除。
// In Listing 15-25, we always want Cons variants to own their list, so reorganizing the data structure isn’t possible.
// 在示例 15-25 中，我们总是希望 Cons 变体拥有它们的列表，因此重组数据结构是不可能的。
// Let’s look at an example using graphs made up of parent nodes and child nodes to see when non-ownership relationships are an appropriate way to prevent reference cycles.
// 让我们看一个使用由父节点和子节点组成的图的例子 and child nodes to see when non-ownership relationships are an appropriate way to prevent reference cycles.

// Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>
// 防止引用循环：将 Rc<T> 变成 Weak<T>
// So far, we’ve demonstrated that calling Rc::clone increases the strong_count of an Rc<T> instance, and an Rc<T> instance is only cleaned up if its strong_count is 0.
// 到目前为止，我们已经演示了调用 Rc::clone 会增加 Rc<T> 实例的 strong_count，而 Rc<T> 实例只有在其 strong_count 为 0 时才会被清理。
// You can also create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade and passing a reference to the Rc<T>.
// 您还可以通过调用 Rc::downgrade 并传递对 Rc<T> 的引用来创建对 Rc<T> 实例中值的弱引用。
// Strong references are how you can share ownership of an Rc<T> instance.
// 强引用是您共享 Rc<T> 实例所有权的方式。
// Weak references don’t express an ownership relationship, and their count doesn’t affect when an Rc<T> instance is cleaned up.
// 弱引用不表示所有权关系，它们的计数不影响 Rc<T> 实例何时被清理。
// They won’t cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.
// 它们不会导致引用循环，因为一旦涉及的值的强引用计数为 0，任何涉及弱引用的循环都会被打破。

// When you call Rc::downgrade, you get a smart pointer of type Weak<T>.
// 当你调用 Rc::downgrade 时，你会得到一个 Weak<T> 类型的智能指针。
// Instead of increasing the strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the weak_count by 1.
// 不是将 Rc<T> 实例中的 strong_count 增加 1，而是调用 Rc::downgrade 将 weak_count 增加 1。
// The Rc<T> type uses weak_count to keep track of how many Weak<T> references exist, similar to strong_count.
// Rc<T> 类型使用 weak_count 来跟踪存在多少 Weak<T> 引用，类似于 strong_count。
// The difference is the weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up.
// 区别在于 weak_count 不需要为 0 来清理 Rc<T> 实例。

// Because the value that Weak<T> references might have been dropped, to do anything with the value that a Weak<T> is pointing to, you must make sure the value still exists.
// 因为 Weak<T> 引用的值可能已被删除，所以要对 Weak<T> 指向的值执行任何操作，您必须确保该值仍然存在。
// Do this by calling the upgrade method on a Weak<T> instance, which will return an Option<Rc<T>>.
// 通过在 Weak<T> 实例上调用升级方法来执行此操作，它将返回一个 Option<Rc<T>>。
// You’ll get a result of Some if the Rc<T> value has not been dropped yet and a result of None if the Rc<T> value has been dropped. Because upgrade returns an Option<Rc<T>>,
// 如果 Rc<T> 值还没有被删除，你会得到一个 Some 的结果，如果 Rc<T> 值已经被删除，你会得到一个 None 的结果。 因为升级返回一个 Option<Rc<T>>，
//  Rust will ensure that the Some case and the None case are handled, and there won’t be an invalid pointer.
// Rust 将确保处理 Some 和 None 情况，并且不会有无效指针。

// As an example, rather than using a list whose items know only about the next item, we’ll create a tree whose items know about their children items and their parent items.
// 例如，我们将创建一棵树，其项目了解其子项目和父项目，而不是使用其项目仅了解下一项的列表。

// Creating a Tree Data Structure: a Node with Child Nodes
// 创建一个树数据结构：一个有子节点的节点
// To start, we’ll build a tree with nodes that know about their child nodes.
// 首先，我们将构建一棵树，其中的节点知道它们的子节点。
// We’ll create a struct named Node that holds its own i32 value as well as references to its children Node values:
// 我们将创建一个名为 Node 的结构，它包含自己的 i32 值以及对其子节点值的引用：

// Filename: src/main.rs

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

// We want a Node to own its children, and we want to share that ownership with variables so we can access each Node in the tree directly.
// 我们希望一个 Node 拥有它的子节点，并且我们希望与变量共享该所有权，以便我们可以直接访问树中的每个 Node。
// To do this, we define the Vec<T> items to be values of type Rc<Node>.
// 为此，我们将 Vec<T> 项定义为 Rc<Node> 类型的值。
// We also want to modify which nodes are children of another node, so we have a RefCell<T> in children around the Vec<Rc<Node>>.
// 我们还想修改哪些节点是另一个节点的子节点，因此我们在 Vec<Rc<Node>> 周围的子节点中有一个 RefCell<T>。

// Next, we’ll use our struct definition and create one Node instance named leaf with the value 3 and no children,
// 接下来，我们将使用我们的结构定义并创建一个名为 leaf 的 Node 实例，其值为 3 并且没有子节点，
//  and another instance named branch with the value 5 and leaf as one of its children, as shown in Listing 15-27:
// 和另一个名为 branch 的实例，其值为 5，leaf 作为其子项之一，如示例 15-27 所示：

// Filename: src/main.rs

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}

// Listing 15-27: Creating a leaf node with no children and a branch node with leaf as one of its children
// 示例 15-27：创建一个没有子节点的叶节点和一个以叶作为其子节点之一的分支节点

// We clone the Rc<Node> in leaf and store that in branch, meaning the Node in leaf now has two owners: leaf and branch.
// 我们在 leaf 中克隆 Rc<Node> 并将其存储在 branch 中，这意味着 leaf 中的 Node 现在有两个所有者：leaf 和 branch。
// We can get from branch to leaf through branch.children, but there’s no way to get from leaf to branch.
// 我们可以通过branch.children从分支到叶子，但是没有办法从叶子到分支。
// The reason is that leaf has no reference to branch and doesn’t know they’re related.
// 原因是 leaf 没有引用 branch 并且不知道它们是相关的。
// We want leaf to know that branch is its parent. We’ll do that next.
// 我们想让 leaf 知道 branch 是它的父节点。 我们接下来会这样做。

// Adding a Reference from a Child to Its Parent
// 添加从子项到其父项的引用
// To make the child node aware of its parent, we need to add a parent field to our Node struct definition.
// 为了让子节点知道它的父节点，我们需要在我们的节点结构定义中添加一个父字段。
// The trouble is in deciding what the type of parent should be.
// 问题在于决定父对象的类型。
// We know it can’t contain an Rc<T>,
// 我们知道它不能包含 Rc<T>,
//  because that would create a reference cycle with leaf.parent pointing to branch and branch.children pointing to leaf, which would cause their strong_count values to never be 0.
//  因为这会创建一个引用循环，其中 leaf.parent 指向 branch，而 branch.children 指向 leaf，这将导致它们的 strong_count 值永远不会为 0。

// Thinking about the relationships another way, a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well.
// 从另一个角度考虑关系，父节点应该拥有它的子节点：如果父节点被删除，它的子节点也应该被删除。
// However, a child should not own its parent: if we drop a child node, the parent should still exist. This is a case for weak references!
// 然而，子节点不应该拥有它的父节点：如果我们删除一个子节点，父节点应该仍然存在。 这是弱引用的情况！

// So instead of Rc<T>, we’ll make the type of parent use Weak<T>, specifically a RefCell<Weak<Node>>.
// 因此，我们将使用 Weak<T> 代替 Rc<T>，特别是 RefCell<Weak<Node>>。
// Now our Node struct definition looks like this:
// 现在我们的节点结构定义如下所示：

// Filename: src/main.rs

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// A node will be able to refer to its parent node but doesn’t own its parent.
// 节点将能够引用其父节点但不拥有其父节点。
// In Listing 15-28, we update main to use this new definition so the leaf node will have a way to refer to its parent, branch:
// 在示例 15-28 中，我们更新了 main 以使用这个新定义，这样叶节点将有办法引用其父节点 branch：

// Filename: src/main.rs

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

// Listing 15-28: A leaf node with a weak reference to its parent node branch
// 示例 15-28：一个对其父节点分支有弱引用的叶节点

// Creating the leaf node looks similar to Listing 15-27 with the exception of the parent field: leaf starts out without a parent, so we create a new, empty Weak<Node> reference instance.
// 创建叶节点看起来类似于清单 15-27，除了父字段：叶开始时没有父节点，所以我们创建一个新的空 Weak<Node> 引用实例。

// At this point, when we try to get a reference to the parent of leaf by using the upgrade method, we get a None value.
// 此时，当我们尝试使用 upgrade 方法获取对 leaf 父级的引用时，我们得到一个 None 值。
// We see this in the output from the first println! statement:
// 我们在第一个 println 的输出中看到了这一点！ 陈述：

// leaf parent = None

// When we create the branch node, it will also have a new Weak<Node> reference in the parent field, because branch doesn’t have a parent node.
// 当我们创建 branch 节点时，它的 parent 字段中也会有一个新的 Weak<Node> 引用，因为 branch 没有父节点。
// We still have leaf as one of the children of branch. Once we have the Node instance in branch, we can modify leaf to give it a Weak<Node> reference to its parent.
// 我们仍然有 leaf 作为 branch 的孩子之一。 一旦我们在分支中有了 Node 实例，我们就可以修改 leaf 来给它一个 Weak<Node> 对其父节点的引用。
// We use the borrow_mut method on the RefCell<Weak<Node>> in the parent field of leaf, and then we use the Rc::downgrade function to create a Weak<Node> reference to branch from the Rc<Node> in branch.
// 我们在 leaf 的 parent 字段中的 RefCell<Weak<Node>> 上使用 borrow_mut 方法，然后我们使用 Rc::downgrade 函数从 Rc<Node> 中创建一个 Weak<Node> 到 branch 的引用 分支。

// When we print the parent of leaf again, this time we’ll get a Some variant holding branch: now leaf can access its parent!
// 当我们再次打印 leaf 的 parent 时，这次我们会得到一个 Some variant holding branch: now leaf can access its parent!
// When we print leaf, we also avoid the cycle that eventually ended in a stack overflow like we had in Listing 15-26; the Weak<Node> references are printed as (Weak):
// 当我们打印 leaf 时，我们也避免了最终以栈溢出结束的循环，就像示例 15-26 中那样； Weak<Node> 引用打印为（弱）：


// leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
// children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
// children: RefCell { value: [] } }] } })

// The lack of infinite output indicates that this code didn’t create a reference cycle.
// 缺少无限输出表明这段代码没有创建引用循环。
// We can also tell this by looking at the values we get from calling Rc::strong_count and Rc::weak_count.
// 我们也可以通过查看调用 Rc::strong_count 和 Rc::weak_count 得到的值来判断。

// Visualizing Changes to strong_count and weak_count
// 可视化 strong_count 和 weak_count 的变化
// Let’s look at how the strong_count and weak_count values of the Rc<Node> instances change by creating a new inner scope and moving the creation of branch into that scope.
// 让我们看看 Rc<Node> 实例的 strong_count 和 weak_count 值如何通过创建一个新的内部作用域并将分支的创建移动到该作用域中来改变。
// By doing so, we can see what happens when branch is created and then dropped when it goes out of scope.
// 通过这样做，我们可以看到创建分支时会发生什么，然后在超出范围时将其删除。
// The modifications are shown in Listing 15-29:
// 修改如清单 15-29 所示：

// Filename: src/main.rs

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

// Listing 15-29: Creating branch in an inner scope and examining strong and weak reference counts
// 示例 15-29：在内部作用域中创建分支并检查强引用计数和弱引用计数

// After leaf is created, its Rc<Node> has a strong count of 1 and a weak count of 0.
// leaf 创建后，其 Rc<Node> 的强计数为 1，弱计数为 0。
// In the inner scope, we create branch and associate it with leaf, at which point when we print the counts,
// 在内部范围内，我们创建分支并将其与叶相关联，此时当我们打印计数时，
//  the Rc<Node> in branch will have a strong count of 1 and a weak count of 1 (for leaf.parent pointing to branch with a Weak<Node>).
//  branch 中的 Rc<Node> 的强计数为 1，弱计数为 1（对于 leaf.parent 指向带有 Weak<Node> 的分支）。
// When we print the counts in leaf, we’ll see it will have a strong count of 2, because branch now has a clone of the Rc<Node> of leaf stored in branch.children,
// 当我们打印 leaf 中的计数时，我们会看到它的强计数为 2，因为 branch 现在有一个 leaf 的 Rc<Node> 的克隆存储在 branch.children 中，
//  but will still have a weak count of 0.
//  但弱计数仍为 0。

// When the inner scope ends, branch goes out of scope and the strong count of the Rc<Node> decreases to 0, so its Node is dropped.
// 当内部范围结束时，分支超出范围并且 Rc<Node> 的强计数减少到 0，因此它的节点被删除。
// The weak count of 1 from leaf.parent has no bearing on whether or not Node is dropped, so we don’t get any memory leaks!
// 来自 leaf.parent 的弱计数 1 与 Node 是否被删除无关，所以我们没有任何内存泄漏！

// If we try to access the parent of leaf after the end of the scope, we’ll get None again.
// 如果我们尝试在范围结束后访问 leaf 的父级，我们将再次获得 None。
// At the end of the program, the Rc<Node> in leaf has a strong count of 1 and a weak count of 0, because the variable leaf is now the only reference to the Rc<Node> again.
// 在程序的最后，leaf 中的 Rc<Node> 的强计数为 1，弱计数为 0，因为变量 leaf 现在再次成为对 Rc<Node> 的唯一引用。

// All of the logic that manages the counts and value dropping is built into Rc<T> and Weak<T> and their implementations of the Drop trait.
// 所有管理计数和值丢弃的逻辑都内置于 Rc<T> 和 Weak<T> 及其 Drop 特性的实现中。
// By specifying that the relationship from a child to its parent should be a Weak<T> reference in the definition of Node,
// 通过在 Node 的定义中指定从子到父的关系应该是一个 Weak<T> 引用，
//  you’re able to have parent nodes point to child nodes and vice versa without creating a reference cycle and memory leaks.
//  你可以让父节点指向子节点，反之亦然，而不会产生引用循环和内存泄漏。

// Summary
// 概括
// This chapter covered how to use smart pointers to make different guarantees and trade-offs from those Rust makes by default with regular references.
// 本章介绍了如何使用智能指针做出不同的保证和取舍，这些保证和取舍与 Rust 默认使用常规引用所做的不同。
// The Box<T> type has a known size and points to data allocated on the heap.
// Box<T> 类型具有已知大小并指向分配在堆上的数据。
// The Rc<T> type keeps track of the number of references to data on the heap so that data can have multiple owners.
// Rc<T> 类型跟踪对堆上数据的引用数，以便数据可以有多个所有者。
// The RefCell<T> type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type;
// 具有内部可变性的 RefCell<T> 类型为我们提供了一种类型，当我们需要不可变类型但需要更改该类型的内部值时可以使用该类型；
// it also enforces the borrowing rules at runtime instead of at compile time.
// 它还在运行时而不是编译时强制执行借用规则。

// Also discussed were the Deref and Drop traits, which enable a lot of the functionality of smart pointers.
// 还讨论了 Deref 和 Drop 特性，它们启用了智能指针的许多功能。
// We explored reference cycles that can cause memory leaks and how to prevent them using Weak<T>.
// 我们探讨了可能导致内存泄漏的引用循环以及如何使用 Weak<T> 来防止它们。

// If this chapter has piqued your interest and you want to implement your own smart pointers, check out “The Rustonomicon” for more useful information.
// 如果本章引起了您的兴趣并且您想实现自己的智能指针，请查看“The Rustonomicon”以获取更多有用信息。

// Next, we’ll talk about concurrency in Rust.
// 接下来，我们将讨论 Rust 中的并发性。
// You’ll even learn about a few new smart pointers.
// 你甚至会学到一些新的智能指针。
