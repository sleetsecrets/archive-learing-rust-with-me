// Advanced Features
// By now, you’ve learned the most commonly used parts of the Rust programming language.
// 到目前为止，您已经学习了 Rust 编程语言中最常用的部分。
// Before we do one more project in Chapter 20, we’ll look at a few aspects of the language you might run into every once in a while,
// 在第 20 章再做一个项目之前，我们将看看你可能偶尔会遇到的语言的几个方面，
//  but may not use every day. You can use this chapter as a reference for when you encounter any unknowns.
//  但可能不会每天都使用。 当您遇到任何未知内容时，您可以将本章作为参考。
// The features covered here are useful in very specific situations.
// 此处介绍的功能在非常特殊的情况下很有用。
// Although you might not reach for them often, we want to make sure you have a grasp of all the features Rust has to offer.
// 尽管您可能不会经常接触到它们，但我们希望确保您掌握 Rust 必须提供的所有功能。

// In this chapter, we’ll cover:

// Unsafe Rust: how to opt out of some of Rust’s guarantees and take responsibility for manually upholding those guarantees
// 不安全的 Rust：如何选择退出 Rust 的一些保证并负责手动维护这些保证
// Advanced traits: associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern in relation to traits
// 高级特征：关联类型、默认类型参数、完全限定语法、超特征以及与特征相关的新类型模式
// Advanced types: more about the newtype pattern, type aliases, the never type, and dynamically sized types
// 高级类型：更多关于 newtype 模式、类型别名、never 类型和动态大小类型
// Advanced functions and closures: function pointers and returning closures
// 高级函数和闭包：函数指针和返回闭包
// Macros: ways to define code that defines more code at compile time
// 宏：定义代码的方式，在编译时定义更多的代码
// It’s a panoply of Rust features with something for everyone! Let’s dive in!
// 这是一整套 Rust 功能，适合每个人！ 让我们开始吧！


// Unsafe Rust
// 不安全的 Rust
// All the code we’ve discussed so far has had Rust’s memory safety guarantees enforced at compile time.
// 到目前为止我们讨论的所有代码都在编译时强制执行了 Rust 的内存安全保证。
// However, Rust has a second language hidden inside it that doesn’t enforce these memory safety guarantees:
// 然而，Rust 隐藏了另一种语言，它不强制执行这些内存安全保证：
//  it’s called unsafe Rust and works just like regular Rust, but gives us extra superpowers.
//  它被称为不安全的 Rust，就像普通的 Rust 一样工作，但给了我们额外的超能力。

// Unsafe Rust exists because, by nature, static analysis is conservative.
// 不安全的 Rust 存在是因为静态分析本质上是保守的。
// When the compiler tries to determine whether or not code upholds the guarantees, it’s better for it to reject some valid programs than to accept some invalid programs.
// 当编译器试图确定代码是否支持保证时，拒绝一些有效程序比接受一些无效程序更好。
// Although the code might be okay, if the Rust compiler doesn’t have enough information to be confident, it will reject the code.
// 尽管代码可能没问题，但如果 Rust 编译器没有足够的信息来确定，它会拒绝代码。
// In these cases, you can use unsafe code to tell the compiler, “Trust me, I know what I’m doing.”
// 在这些情况下，您可以使用不安全代码告诉编译器，“相信我，我知道我在做什么。”
//  Be warned, however, that you use unsafe Rust at your own risk:
//  但是请注意，使用不安全的 Rust 需要您自担风险：
//  if you use unsafe code incorrectly, problems can occur due to memory unsafety, such as null pointer dereferencing.
//  不安全代码使用不当，会出现内存不安全的问题，比如空指针解引用。


// Another reason Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe.
// Rust 具有不安全的另一个原因是底层计算机硬件本质上是不安全的。
// If Rust didn’t let you do unsafe operations, you couldn’t do certain tasks.
// 如果 Rust 不允许你做不安全的操作，你就不能做某些任务。
// Rust needs to allow you to do low-level systems programming, such as directly interacting with the operating system or even writing your own operating system.
// Rust 需要允许您进行低级系统编程，例如直接与操作系统交互，甚至编写您自己的操作系统。
// Working with low-level systems programming is one of the goals of the language.
// 使用低级系统编程是该语言的目标之一。
// Let’s explore what we can do with unsafe Rust and how to do it.
// 让我们探讨一下我们可以用不安全的 Rust 做什么以及如何做。

// Unsafe Superpowers
// 不安全的超能力
// To switch to unsafe Rust, use the unsafe keyword and then start a new block that holds the unsafe code.
// 要切换到不安全的 Rust，请使用不安全关键字，然后启动一个包含不安全代码的新块。
// You can take five actions in unsafe Rust that you can’t in safe Rust, which we call unsafe superpowers.
// 你可以在不安全的 Rust 中采取在安全的 Rust 中不能做的五个动作，我们称之为不安全的超能力。
// Those superpowers include the ability to:
// 这些超能力包括：

// * Dereference a raw pointer
// * 解引用原始指针
// * Call an unsafe function or method
// * 调用不安全的函数或方法
// * Access or modify a mutable static variable
// * 访问或修改可变静态变量
// * Implement an unsafe trait
// * 实现一个不安全的特征
// * Access fields of unions
// * 访问联合字段

// It’s important to understand that unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks:
// 重要的是要了解 unsafe 不会关闭借用检查器或禁用任何其他 Rust 的安全检查：
//  if you use a reference in unsafe code, it will still be checked.
//  如果你在不安全的代码中使用引用，它仍然会被检查。
// The unsafe keyword only gives you access to these five features that are then not checked by the compiler for memory safety.
// 另外，unsafe并不代表block里面的代码就一定是危险的或者一定会存在内存安全问题：
// You’ll still get some degree of safety inside of an unsafe block.
// 目的是作为程序员，您将确保不安全块内的代码将以有效方式访问内存。

// In addition, unsafe does not mean the code inside the block is necessarily dangerous or that it will definitely have memory safety problems:
// unsafe 关键字只允许您访问这五个功能，然后编译器不会检查内存安全。
//  the intent is that as the programmer, you’ll ensure the code inside an unsafe block will access memory in a valid way.
//  你仍然会在不安全的块中获得一定程度的安全。

// People are fallible, and mistakes will happen,
// 人是容易犯错的，错误会发生，
//  but by requiring these five unsafe operations to be inside blocks annotated with unsafe you’ll know that any errors related to memory safety must be within an unsafe block.
//  但通过要求这五个不安全操作位于带有不安全注释的块内，您将知道与内存安全相关的任何错误都必须位于不安全块内。
// Keep unsafe blocks small; you’ll be thankful later when you investigate memory bugs.
// 保持不安全块小； 稍后当您调查内存错误时，您会感激不尽。

// To isolate unsafe code as much as possible, it’s best to enclose unsafe code within a safe abstraction and provide a safe API,
// 为了尽可能隔离不安全的代码，最好将不安全的代码封装在安全的抽象中并提供安全的 API，
//  which we’ll discuss later in the chapter when we examine unsafe functions and methods.
// 我们将在本章后面检查不安全的函数和方法时讨论。
// Parts of the standard library are implemented as safe abstractions over unsafe code that has been audited.
// 部分标准库实现为对已审计的不安全代码的安全抽象。
// Wrapping unsafe code in a safe abstraction prevents uses of unsafe from leaking out into all the places that you or your users might want to use the functionality implemented with unsafe code,
// 将不安全代码包装在安全抽象中可以防止不安全的使用泄漏到您或您的用户可能想要使用不安全代码实现的功能的所有地方，
//  because using a safe abstraction is safe.
//  因为使用安全抽象是安全的。

// Let’s look at each of the five unsafe superpowers in turn.
// 让我们依次看看五个不安全的超级大国。
// We’ll also look at some abstractions that provide a safe interface to unsafe code.
// 我们还将研究一些为不安全代码提供安全接口的抽象。

// Dereferencing a Raw Pointer
// 解引用原始指针
// In Chapter 4, in the “Dangling References” section, we mentioned that the compiler ensures references are always valid.
// 在第 4 章的“悬挂引用”部分，我们提到编译器确保引用始终有效。
// Unsafe Rust has two new types called raw pointers that are similar to references.
// 不安全 Rust 有两种新类型，称为原始指针，类似于引用。
// As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively.
// 与引用一样，原始指针可以是不可变的或可变的，分别写为 *const T 和 *mut T。
// The asterisk isn’t the dereference operator; it’s part of the type name.
// 星号不是解引用运算符； 它是类型名称的一部分。
// In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.
// 在原始指针的上下文中，不可变意味着指针在解引用后不能直接赋值。

// Different from references and smart pointers, raw pointers:
// 不同于引用和智能指针，原始指针：

// * Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
// * 通过将不可变指针和可变指针或多个可变指针指向同一位置，允许忽略借用规则
// * Aren’t guaranteed to point to valid memory
// * 不保证指向有效内存
// * Are allowed to be null
// * 允许为空
// * Don’t implement any automatic cleanup
// * 没有实现任何自动清理

// By opting out of having Rust enforce these guarantees,
// 通过选择不让 Rust 强制执行这些保证，
//  you can give up guaranteed safety in exchange for greater performance or the ability to interface with another language or hardware where Rust’s guarantees don’t apply.
//  你可以放弃有保证的安全性以换取更高的性能或与 Rust 的保证不适用的另一种语言或硬件接口的能力。

// Listing 19-1 shows how to create an immutable and a mutable raw pointer from references.
// 清单 19-1 显示了如何从引用创建一个不可变和可变的原始指针。

let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

// Listing 19-1: Creating raw pointers from references
// 示例 19-1：从引用创建原始指针

// Notice that we don’t include the unsafe keyword in this code.
// 请注意，我们没有在此代码中包含 unsafe 关键字。
// We can create raw pointers in safe code; we just can’t dereference raw pointers outside an unsafe block, as you’ll see in a bit.
// 我们可以在安全代码中创建原始指针； 我们只是不能在不安全块之外解引用原始指针，稍后您会看到。

// We’ve created raw pointers by using as to cast an immutable and a mutable reference into their corresponding raw pointer types.
// 我们通过使用 as 将不可变和可变引用转换为相应的原始指针类型来创建原始指针。
// Because we created them directly from references guaranteed to be valid, we know these particular raw pointers are valid, but we can’t make that assumption about just any raw pointer.
// 因为我们直接从保证有效的引用创建它们，所以我们知道这些特定的原始指针是有效的，但我们不能对任何原始指针做出这样的假设。

// To demonstrate this, next we’ll create a raw pointer whose validity we can’t be so certain of.
// 为了证明这一点，接下来我们将创建一个原始指针，我们无法确定其有效性。
// Listing 19-2 shows how to create a raw pointer to an arbitrary location in memory.
// 清单 19-2 展示了如何创建指向内存中任意位置的原始指针。
// Trying to use arbitrary memory is undefined:
// 尝试使用任意内存是未定义的：
//  there might be data at that address or there might not, the compiler might optimize the code so there is no memory access, or the program might error with a segmentation fault.
//  该地址可能有数据，也可能没有，编译器可能会优化代码，因此没有内存访问，或者程序可能会因分段错误而出错。
// Usually, there is no good reason to write code like this, but it is possible.
// 通常，没有充分的理由编写这样的代码，但这是可能的。

let address = 0x012345usize;
let r = address as *const i32;

// Listing 19-2: Creating a raw pointer to an arbitrary memory address
// 示例 19-2：创建指向任意内存地址的原始指针

// Recall that we can create raw pointers in safe code, but we can’t dereference raw pointers and read the data being pointed to.
// 回想一下，我们可以在安全代码中创建原始指针，但我们不能解引用原始指针并读取所指向的数据。
// In Listing 19-3, we use the dereference operator * on a raw pointer that requires an unsafe block.
// 在示例 19-3 中，我们在需要不安全块的原始指针上使用解引用运算符 *。

let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

// Listing 19-3: Dereferencing raw pointers within an unsafe block
// 示例 19-3：在不安全块中解引用原始指针

// Creating a pointer does no harm; it’s only when we try to access the value that it points at that we might end up dealing with an invalid value.
// 创建一个指针没有坏处； 只有当我们尝试访问它指向的值时，我们才可能最终处理一个无效值。

// Note also that in Listing 19-1 and 19-3, we created *const i32 and *mut i32 raw pointers that both pointed to the same memory location, where num is stored.
// 另请注意，在清单 19-1 和 19-3 中，我们创建了 *const i32 和 *mut i32 原始指针，它们都指向相同的内存位置，其中存储了 num。
// If we instead tried to create an immutable and a mutable reference to num,
// 如果我们改为尝试创建对 num 的不可变和可变引用，
//  the code would not have compiled because Rust’s ownership rules don’t allow a mutable reference at the same time as any immutable references.
//  代码不会被编译，因为 Rust 的所有权规则不允许同时有可变引用和任何不可变引用。
// With raw pointers, we can create a mutable pointer and an immutable pointer to the same location and change data through the mutable pointer, potentially creating a data race. Be careful!
// 使用原始指针，我们可以创建一个指向同一位置的可变指针和一个不可变指针，并通过可变指针更改数据，这可能会造成数据竞争。 当心！

// With all of these dangers, why would you ever use raw pointers?
// 有了所有这些危险，你为什么还要使用原始指针？
// One major use case is when interfacing with C code, as you’ll see in the next section, “Calling an Unsafe Function or Method.”
// 一个主要用例是与 C 代码交互，您将在下一节“调用不安全的函数或方法”中看到。
// Another case is when building up safe abstractions that the borrow checker doesn’t understand.
// 另一种情况是建立借用检查器不理解的安全抽象。
// We’ll introduce unsafe functions and then look at an example of a safe abstraction that uses unsafe code.
// 我们将介绍不安全函数，然后查看使用不安全代码的安全抽象示例。

// Calling an Unsafe Function or Method
// 调用不安全的函数或方法
// The second type of operation you can perform in an unsafe block is calling unsafe functions.
// 您可以在不安全块中执行的第二种操作是调用不安全函数。
// Unsafe functions and methods look exactly like regular functions and methods, but they have an extra unsafe before the rest of the definition.
// 不安全的函数和方法看起来和普通的函数和方法一模一样，但是它们在定义的其余部分之前多了一个不安全的。
// The unsafe keyword in this context indicates the function has requirements we need to uphold when we call this function, because Rust can’t guarantee we’ve met these requirements.
// 这个上下文中的 unsafe 关键字表示函数有我们在调用这个函数时需要坚持的要求，因为 Rust 不能保证我们已经满足这些要求。
// By calling an unsafe function within an unsafe block, we’re saying that we’ve read this function’s documentation and take responsibility for upholding the function’s contracts.
// 通过在不安全块中调用不安全函数，我们表示我们已经阅读了该函数的文档并负责维护该函数的契约。

// Here is an unsafe function named dangerous that doesn’t do anything in its body:
// 这是一个名为 dangerous 的不安全函数，它在其主体中不执行任何操作：

unsafe fn dangerous() {}

unsafe {
    dangerous();
}

// We must call the dangerous function within a separate unsafe block.
// 我们必须在单独的不安全块中调用危险函数。
// If we try to call dangerous without the unsafe block, we’ll get an error:
// 如果我们尝试在没有 unsafe 块的情况下调用 dangerous，我们会得到一个错误：

// $ cargo run
//    Compiling unsafe-example v0.1.0 (file:///projects/unsafe-example)
// error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
//  --> src/main.rs:4:5
//   |
// 4 |     dangerous();
//   |     ^^^^^^^^^^^ call to unsafe function
//   |
//   = note: consult the function's documentation for information on how to avoid undefined behavior
//
// For more information about this error, try `rustc --explain E0133`.
// error: could not compile `unsafe-example` due to previous error

// With the unsafe block, we’re asserting to Rust that we’ve read the function’s documentation, we understand how to use it properly,
// 使用不安全块，我们向 Rust 断言我们已经阅读了该函数的文档，我们了解如何正确使用它，
//  and we’ve verified that we’re fulfilling the contract of the function.
//  并且我们已经验证了我们正在履行函数的契约。

// Bodies of unsafe functions are effectively unsafe blocks, so to perform other unsafe operations within an unsafe function, we don’t need to add another unsafe block.
// 不安全函数体实际上是不安全块，因此要在不安全函数内执行其他不安全操作，我们不需要添加另一个不安全块。

// Creating a Safe Abstraction over Unsafe Code
// 在不安全代码上创建安全抽象
// Just because a function contains unsafe code doesn’t mean we need to mark the entire function as unsafe.
// 仅仅因为函数包含不安全代码并不意味着我们需要将整个函数标记为不安全。
// In fact, wrapping unsafe code in a safe function is a common abstraction.
// 事实上，将不安全代码包装在安全函数中是一种常见的抽象。
// As an example, let’s study the split_at_mut function from the standard library, which requires some unsafe code.
// 例如，让我们研究标准库中的 split_at_mut 函数，它需要一些不安全的代码。
// We’ll explore how we might implement it. This safe method is defined on mutable slices:
// 我们将探索如何实现它。 这个安全方法是在可变切片上定义的：
//  it takes one slice and makes it two by splitting the slice at the index given as an argument.
// 它需要一个切片，并通过在作为参数给出的索引处拆分切片来将其分成两个。
// Listing 19-4 shows how to use split_at_mut.
// 清单 19-4 展示了如何使用 split_at_mut。

let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);

// Listing 19-4: Using the safe split_at_mut function
// 示例 19-4：使用安全的 split_at_mut 函数

// We can’t implement this function using only safe Rust.
// 我们不能只使用安全的 Rust 来实现这个功能。
// An attempt might look something like Listing 19-5, which won’t compile.
// 尝试可能类似于示例 19-5，但无法编译。
// For simplicity, we’ll implement split_at_mut as a function rather than a method and only for slices of i32 values rather than for a generic type T.
// 为简单起见，我们将 split_at_mut 作为函数而不是方法来实现，并且仅用于 i32 值的切片而不是泛型类型 T。

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    (&mut values[..mid], &mut values[mid..])
}

// Listing 19-5: An attempted implementation of split_at_mut using only safe Rust
// 示例 19-5：尝试仅使用安全 Rust 实现 split_at_mut

// This function first gets the total length of the slice.
// 该函数首先获取切片的总长度。
// Then it asserts that the index given as a parameter is within the slice by checking whether it’s less than or equal to the length.
// 然后它通过检查它是否小于或等于长度来断言作为参数给出的索引在切片内。
// The assertion means that if we pass an index that is greater than the length to split the slice at, the function will panic before it attempts to use that index.
// 该断言意味着如果我们传递的索引大于分割切片的长度，该函数将在尝试使用该索引之前发生恐慌。

// Then we return two mutable slices in a tuple: one from the start of the original slice to the mid index and another from mid to the end of the slice.
// 然后我们在一个元组中返回两个可变切片：一个从原始切片的开始到中间索引，另一个从中间到切片的结尾。

// When we try to compile the code in Listing 19-5, we’ll get an error.
// 当我们尝试编译示例 19-5 中的代码时，我们会得到一个错误。

// $ cargo run
//    Compiling unsafe-example v0.1.0 (file:///projects/unsafe-example)
// error[E0499]: cannot borrow `*values` as mutable more than once at a time
//  --> src/main.rs:6:31
//   |
// 1 | fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//   |                         - let's call the lifetime of this reference `'1`
// ...
// 6 |     (&mut values[..mid], &mut values[mid..])
//   |     --------------------------^^^^^^--------
//   |     |     |                   |
//   |     |     |                   second mutable borrow occurs here
//   |     |     first mutable borrow occurs here
//   |     returning this value requires that `*values` is borrowed for `'1`
//
// For more information about this error, try `rustc --explain E0499`.
// error: could not compile `unsafe-example` due to previous error

// Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; it only knows that we’re borrowing from the same slice twice.
// Rust 的借用检查器无法理解我们在借用切片的不同部分； 它只知道我们从同一个切片中借用了两次。
// Borrowing different parts of a slice is fundamentally okay because the two slices aren’t overlapping, but Rust isn’t smart enough to know this.
// 借用切片的不同部分基本上是可以的，因为这两个切片不重叠，但 Rust 不够聪明，无法知道这一点。
// When we know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.
// 当我们知道代码是可以的，但 Rust 不是，是时候处理不安全的代码了。

// Listing 19-6 shows how to use an unsafe block, a raw pointer, and some calls to unsafe functions to make the implementation of split_at_mut work.
// 清单 19-6 展示了如何使用一个不安全块、一个原始指针和一些对不安全函数的调用来使 split_at_mut 的实现工作。

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Listing 19-6: Using unsafe code in the implementation of the split_at_mut function
// 示例 19-6：在 split_at_mut 函数的实现中使用不安全代码

// Recall from “The Slice Type” section in Chapter 4 that slices are a pointer to some data and the length of the slice.
// 回忆第 4 章的“切片类型”部分，切片是指向某些数据的指针和切片的长度。
// We use the len method to get the length of a slice and the as_mut_ptr method to access the raw pointer of a slice. In this case,
// 我们使用 len 方法获取切片的长度，使用 as_mut_ptr 方法访问切片的原始指针。 在这种情况下，
//  because we have a mutable slice to i32 values, as_mut_ptr returns a raw pointer with the type *mut i32, which we’ve stored in the variable ptr.
//  因为我们有一个 i32 值的可变切片，as_mut_ptr 返回一个类型为 *mut i32 的原始指针，我们将其存储在变量 ptr 中。

// We keep the assertion that the mid index is within the slice.
// 我们保留中间索引在切片内的断言。
// Then we get to the unsafe code: the slice::from_raw_parts_mut function takes a raw pointer and a length,
// 然后我们得到不安全的代码：slice::from_raw_parts_mut 函数接受一个原始指针和一个长度，
//  and it creates a slice. We use this function to create a slice that starts from ptr and is mid items long.
//  它创建了一个切片。 我们使用此函数创建一个从 ptr 开始且长度为中间项的切片。
// Then we call the add method on ptr with mid as an argument to get a raw pointer that starts at mid,
// 然后我们以 mid 作为参数调用 ptr 上的 add 方法来获得一个从 mid 开始的原始指针，
//  and we create a slice using that pointer and the remaining number of items after mid as the length.
//  然后我们使用该指针创建一个切片，并将 mid 之后的剩余项目数作为长度。

// The function slice::from_raw_parts_mut is unsafe because it takes a raw pointer and must trust that this pointer is valid.
// 函数 slice::from_raw_parts_mut 是不安全的，因为它接受一个原始指针并且必须相信这个指针是有效的。
// The add method on raw pointers is also unsafe, because it must trust that the offset location is also a valid pointer.
// 原始指针上的 add 方法也是不安全的，因为它必须相信偏移位置也是一个有效的指针。
// Therefore, we had to put an unsafe block around our calls to slice::from_raw_parts_mut and add so we could call them.
// 因此，我们必须在对 slice::from_raw_parts_mut 的调用周围放置一个不安全的块并添加，以便我们可以调用它们。
// By looking at the code and by adding the assertion that mid must be less than or equal to len,
// 通过查看代码并添加 mid 必须小于或等于 len 的断言，
//  we can tell that all the raw pointers used within the unsafe block will be valid pointers to data within the slice.
//  我们可以看出，不安全块中使用的所有原始指针都是指向切片中数据的有效指针。
// This is an acceptable and appropriate use of unsafe.
// 这是对不安全的可接受且适当的使用。

// Note that we don’t need to mark the resulting split_at_mut function as unsafe, and we can call this function from safe Rust.
// 请注意，我们不需要将生成的 split_at_mut 函数标记为不安全，我们可以从安全的 Rust 中调用此函数。
// We’ve created a safe abstraction to the unsafe code with an implementation of the function that uses unsafe code in a safe way,
// 我们已经通过以安全方式使用不安全代码的函数实现创建了对不安全代码的安全抽象，
//  because it creates only valid pointers from the data this function has access to.
// 因为它仅从该函数有权访问的数据创建有效指针。

// In contrast, the use of slice::from_raw_parts_mut in Listing 19-7 would likely crash when the slice is used.
// 相反，在使用切片时，清单 19-7 中使用 slice::from_raw_parts_mut 可能会崩溃。
// This code takes an arbitrary memory location and creates a slice 10,000 items long.
// 此代码采用任意内存位置并创建一个 10,000 项长的切片。

use std::slice;

let address = 0x01234usize;
let r = address as *mut i32;

let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

// Listing 19-7: Creating a slice from an arbitrary memory location
// 示例 19-7：从任意内存位置创建切片

// We don’t own the memory at this arbitrary location, and there is no guarantee that the slice this code creates contains valid i32 values.
// 我们不拥有这个任意位置的内存，并且不能保证这段代码创建的切片包含有效的 i32 值。
// Attempting to use values as though it’s a valid slice results in undefined behavior.
// 尝试将值当作有效切片使用会导致未定义的行为。

// Using extern Functions to Call External Code
// 使用extern函数调用外部代码
// Sometimes, your Rust code might need to interact with code written in another language.
// 有时，您的 Rust 代码可能需要与用另一种语言编写的代码进行交互。
// For this, Rust has the keyword extern that facilitates the creation and use of a Foreign Function Interface (FFI).
// 为此，Rust 有关键字 extern 来促进外部函数接口 (FFI) 的创建和使用。
// An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.
// FFI 是编程语言定义函数并使不同的（外部）编程语言能够调用这些函数的一种方式。

// Listing 19-8 demonstrates how to set up an integration with the abs function from the C standard library.
// 清单 19-8 演示了如何设置与 C 标准库中的 abs 函数的集成。
// Functions declared within extern blocks are always unsafe to call from Rust code.
// 在 extern 块中声明的函数从 Rust 代码调用总是不安全的。
// The reason is that other languages don’t enforce Rust’s rules and guarantees, and Rust can’t check them, so responsibility falls on the programmer to ensure safety.
// 原因是其他语言不执行 Rust 的规则和保证，Rust 无法检查它们，因此责任落在程序员身上以确保安全。

// Filename: src/main.rs

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Listing 19-8: Declaring and calling an extern function defined in another language
// 示例 19-8：声明和调用用另一种语言定义的外部函数

// Within the extern "C" block, we list the names and signatures of external functions from another language we want to call.
// 在 extern "C" 块中，我们列出了我们要调用的另一种语言的外部函数的名称和签名。
// The "C" part defines which application binary interface (ABI) the external function uses: the ABI defines how to call the function at the assembly level.
// “C”部分定义外部函数使用哪个应用程序二进制接口 (ABI)：ABI 定义如何在汇编级别调用函数。
// The "C" ABI is the most common and follows the C programming language’s ABI.
// “C”ABI 是最常见的，它遵循 C 编程语言的 ABI。

// Calling Rust Functions from Other Languages
// 从其他语言调用 Rust 函数
// We can also use extern to create an interface that allows other languages to call Rust functions.
// 我们也可以使用 extern 来创建一个允许其他语言调用 Rust 函数的接口。
// Instead of creating a whole extern block, we add the extern keyword and specify the ABI to use just before the fn keyword for the relevant function.
// 我们没有创建整个 extern 块，而是添加了 extern 关键字并指定了要在相关函数的 fn 关键字之前使用的 ABI。
// We also need to add a #[no_mangle] annotation to tell the Rust compiler not to mangle the name of this function.
// 我们还需要添加一个#[no_mangle] 注释来告诉 Rust 编译器不要破坏这个函数的名称。
// Mangling is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of
// Mangling 是指编译器将我们为函数指定的名称更改为包含其他部分更多信息的不同名称
//  the compilation process to consume but is less human readable.
//  要消耗的编译过程，但人类可读性较差。
// Every programming language compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, we must disable the Rust compiler’s name mangling.
// 每种编程语言编译器对名称的处理略有不同，因此要使 Rust 函数可以被其他语言命名，我们必须禁用 Rust 编译器的名称处理。

// In the following example, we make the call_from_c function accessible from C code, after it’s compiled to a shared library and linked from C:
// 在以下示例中，我们将 call_from_c 函数编译为共享库并从 C 链接后，使其可从 C 代码访问：

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// This usage of extern does not require unsafe.
// extern 的这种用法不需要不安全。

// Accessing or Modifying a Mutable Static Variable
// 访问或修改可变静态变量
// In this book, we’ve not yet talked about global variables, which Rust does support but can be problematic with Rust’s ownership rules.
// 在这本书中，我们还没有讨论全局变量，Rust 确实支持全局变量，但 Rust 的所有权规则可能会有问题。
// If two threads are accessing the same mutable global variable, it can cause a data race.
// 如果两个线程正在访问同一个可变全局变量，可能会导致数据竞争。

// In Rust, global variables are called static variables.
// 在 Rust 中，全局变量被称为静态变量。
// Listing 19-9 shows an example declaration and use of a static variable with a string slice as a value.
// 清单 19-9 显示了一个示例声明和使用一个以字符串切片作为值的静态变量。

// Filename: src/main.rs

static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}

// Listing 19-9: Defining and using an immutable static variable
// 示例 19-9：定义和使用不可变静态变量

// Static variables are similar to constants, which we discussed in the “Differences Between Variables and Constants” section in Chapter 3.
// 静态变量类似于常量，我们在第 3 章“变量和常量的区别”部分讨论过。
// The names of static variables are in SCREAMING_SNAKE_CASE by convention.
// 按照惯例，静态变量的名称在 SCREAMING_SNAKE_CASE 中。
// Static variables can only store references with the 'static lifetime, which means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly.
// 静态变量只能存储具有 'static 生命周期的引用，这意味着 Rust 编译器可以计算出生命周期，我们不需要显式注释它。
// Accessing an immutable static variable is safe.
// 访问不可变的静态变量是安全的。

// A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory.
// 常量和不可变静态变量之间的细微差别是静态变量中的值在内存中具有固定地址。
// Using the value will always access the same data. Constants, on the other hand, are allowed to duplicate their data whenever they’re used.
// 使用该值将始终访问相同的数据。 另一方面，常量允许在使用时复制它们的数据。
// Another difference is that static variables can be mutable. Accessing and modifying mutable static variables is unsafe.
// 另一个区别是静态变量可以是可变的。 访问和修改可变静态变量是不安全的。
// Listing 19-10 shows how to declare, access, and modify a mutable static variable named COUNTER.
// 清单 19-10 展示了如何声明、访问和修改名为 COUNTER 的可变静态变量。

// Filename: src/main.rs

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Listing 19-10: Reading from or writing to a mutable static variable is unsafe
// 示例 19-10：读取或写入可变静态变量是不安全的

// As with regular variables, we specify mutability using the mut keyword.
// 与常规变量一样，我们使用 mut 关键字指定可变性。
// Any code that reads or writes from COUNTER must be within an unsafe block.
// 任何从 COUNTER 读取或写入的代码都必须在不安全的块中。
// This code compiles and prints COUNTER: 3 as we would expect because it’s single threaded.
// 此代码编译并打印 COUNTER: 3 正如我们所期望的那样，因为它是单线程的。
// Having multiple threads access COUNTER would likely result in data races.
// 让多个线程访问 COUNTER 可能会导致数据竞争。

// With mutable data that is globally accessible, it’s difficult to ensure there are no data races, which is why Rust considers mutable static variables to be unsafe.
// 对于全局可访问的可变数据，很难确保没有数据竞争，这就是 Rust 认为可变静态变量不安全的原因。
// Where possible,
// 在可能的情况，
//  it’s preferable to use the concurrency techniques and thread-safe smart pointers we discussed in Chapter 16 so the compiler checks that data accessed from different threads is done safely.
//  最好使用我们在第 16 章中讨论的并发技术和线程安全智能指针，这样编译器会检查从不同线程访问的数据是否安全完成。

// Implementing an Unsafe Trait
// 实现不安全特征
// We can use unsafe to implement an unsafe trait.
// 我们可以使用 unsafe 来实现一个不安全的特征。
// A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
// 当至少有一个方法具有编译器无法验证的不变性时，特征是不安全的。
// We declare that a trait is unsafe by adding the unsafe keyword before trait and marking the implementation of the trait as unsafe too, as shown in Listing 19-11.
// 我们通过在 trait 之前添加 unsafe 关键字并将 trait 的实现也标记为不安全来声明 trait 是不安全的，如示例 19-11 所示。

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}

// Listing 19-11: Defining and implementing an unsafe trait
// 示例 19-11：定义和实现不安全特征

// By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify.
// 通过使用不安全的 impl，我们承诺我们将维护编译器无法验证的不变量。

// As an example, recall the Sync and Send marker traits we discussed in the “Extensible Concurrency with the Sync and Send Traits” section in Chapter 16:
// 例如，回想一下我们在第 16 章的“使用同步和发送特征的可扩展并发”部分讨论的同步和发送标记特征：
//  the compiler implements these traits automatically if our types are composed entirely of Send and Sync types.
//  如果我们的类型完全由 Send 和 Sync 类型组成，编译器会自动实现这些特征。
// If we implement a type that contains a type that is not Send or Sync, such as raw pointers, and we want to mark that type as Send or Sync, we must use unsafe.
// 如果我们实现一个类型，其中包含一个不是 Send 或 Sync 的类型，比如原始指针，我们想将该类型标记为 Send 或 Sync，我们必须使用 unsafe。
// Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads;
// Rust 无法验证我们的类型是否支持它可以安全地跨线程发送或从多个线程访问的保证；
// therefore, we need to do those checks manually and indicate as such with unsafe.
// 因此，我们需要手动进行这些检查，并用不安全的方式进行指示。

// Accessing Fields of a Union
// 访问联合字段
// The final action that works only with unsafe is accessing fields of a union.
// 仅适用于不安全的最后一个操作是访问联合的字段。
// A union is similar to a struct, but only one declared field is used in a particular instance at one time.
// union 类似于 struct，但一次在特定实例中只使用一个声明的字段。
// Unions are primarily used to interface with unions in C code.
// 联合主要用于与 C 代码中的联合进行交互。
// Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.
// 访问联合字段是不安全的，因为 Rust 不能保证当前存储在联合实例中的数据的类型。
// You can learn more about unions in the Rust Reference.
// 您可以在 Rust 参考中了解有关联合的更多信息。

// When to Use Unsafe Code
// 何时使用不安全代码
// Using unsafe to take one of the five actions (superpowers) just discussed isn’t wrong or even frowned upon.
// 使用 unsafe 来执行刚刚讨论的五个操作（超能力）之一并没有错，甚至是受欢迎的。
// But it is trickier to get unsafe code correct because the compiler can’t help uphold memory safety.
// 但要使不安全的代码正确更棘手，因为编译器无法帮助维护内存安全。
// When you have a reason to use unsafe code, you can do so, and having the explicit unsafe annotation makes it easier to track down the source of problems when they occur.
// 当你有理由使用不安全代码时，你可以这样做，并且拥有显式的不安全注释使得在问题发生时更容易追踪问题的根源。

// Advanced Traits
// 高级特征
// We first covered traits in the “Traits: Defining Shared Behavior” section of Chapter 10, but we didn’t discuss the more advanced details.
// 我们首先在第 10 章的“特征：定义共享行为”部分介绍了特征，但我们没有讨论更高级的细节。
// Now that you know more about Rust, we can get into the nitty-gritty.
// 既然您对 Rust 有了更多的了解，我们就可以深入了解细节了。

// Specifying Placeholder Types in Trait Definitions with Associated Types
// 在具有关联类型的特征定义中指定占位符类型
// Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.
// 关联类型将类型占位符与特征连接起来，这样特征方法定义就可以在其签名中使用这些占位符类型。
// The implementor of a trait will specify the concrete type to be used instead of the placeholder type for the particular implementation.
// 特征的实现者将指定要使用的具体类型，而不是特定实现的占位符类型。
// That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented.
// 这样，我们就可以定义一个使用某些类型的特征，而无需在实现特征之前确切知道这些类型是什么。

// We’ve described most of the advanced features in this chapter as being rarely needed.
// 我们将本章中的大部分高级功能描述为很少需要。
// Associated types are somewhere in the middle:
// 关联类型在中间的某个地方：
//  they’re used more rarely than features explained in the rest of the book but more commonly than many of the other features discussed in this chapter.
//  它们的使用比本书其余部分中解释的功能少，但比本章讨论的许多其他功能更常用。

// One example of a trait with an associated type is the Iterator trait that the standard library provides.
// 具有关联类型的特征的一个示例是标准库提供的 Iterator 特征。
// The associated type is named Item and stands in for the type of the values the type implementing the Iterator trait is iterating over.
// 关联类型名为 Item，代表实现 Iterator 特性的类型正在迭代的值的类型。
// The definition of the Iterator trait is as shown in Listing 19-12.
// Iterator trait 的定义如示例 19-12 所示。

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Listing 19-12: The definition of the Iterator trait that has an associated type Item
// 示例 19-12：具有关联类型 Item 的 Iterator trait 的定义

// The type Item is a placeholder, and the next method’s definition shows that it will return values of type Option<Self::Item>.
// Item 类型是一个占位符，下一个方法的定义表明它将返回 Option<Self::Item> 类型的值。
// Implementors of the Iterator trait will specify the concrete type for Item, and the next method will return an Option containing a value of that concrete type.
// Iterator trait 的实现者将为 Item 指定具体类型，next 方法将返回包含该具体类型值的 Option。

// Associated types might seem like a similar concept to generics, in that the latter allow us to define a function without specifying what types it can handle.
// 关联类型可能看起来像是与泛型类似的概念，因为后者允许我们定义一个函数而无需指定它可以处理的类型。
// To examine the difference between the two concepts, we’ll look at an implementation of the Iterator trait on a type named Counter that specifies the Item type is u32:
// 为了检查这两个概念之间的区别，我们将查看 Iterator 特性在名为 Counter 的类型上的实现，该类型指定 Item 类型为 u32：

// Filename: src/lib.rs
// 文件名：src/lib.rs

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
    }
    // --snip--
}

// This syntax seems comparable to that of generics.
// 这种语法似乎与泛型相当。
// So why not just define the Iterator trait with generics, as shown in Listing 19-13?
// 那么为什么不直接用泛型定义 Iterator trait，如示例 19-13 所示？

pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

// Listing 19-13: A hypothetical definition of the Iterator trait using generics
// 示例 19-13：使用泛型的 Iterator trait 的假设定义

// The difference is that when using generics, as in Listing 19-13, we must annotate the types in each implementation;
// 不同之处在于，当使用泛型时，如示例 19-13 所示，我们必须在每个实现中注释类型；
// because we can also implement Iterator<String> for Counter or any other type, we could have multiple implementations of Iterator for Counter.
// 因为我们还可以为 Counter 或任何其他类型实现 Iterator<String>，所以我们可以为 Counter 实现 Iterator 的多个实现。
// In other words, when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time.
// 换句话说，当一个 trait 有一个泛型参数时，它可以为一个类型实现多次，每次都改变泛型类型参数的具体类型。
// When we use the next method on Counter, we would have to provide type annotations to indicate which implementation of Iterator we want to use.
// 当我们在 Counter 上使用 next 方法时，我们必须提供类型注释来指示我们想要使用 Iterator 的哪个实现。

// With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times.
// 对于关联类型，我们不需要注释类型，因为我们不能多次实现一个类型的特征。
// In Listing 19-12 with the definition that uses associated types, we can only choose what the type of Item will be once,
// 在使用关联类型定义的示例 19-12 中，我们只能选择 Item 的类型一次，
//  because there can only be one impl Iterator for Counter.
//  因为 Counter 只能有一个 impl Iterator。
// We don’t have to specify that we want an iterator of u32 values everywhere that we call next on Counter.
// 我们不必指定我们想要一个 u32 值的迭代器，我们在 Counter 上调用 next。

// Associated types also become part of the trait’s contract: implementors of the trait must provide a type to stand in for the associated type placeholder.
// 关联类型也成为特征契约的一部分：特征的实现者必须提供一个类型来代表关联类型占位符。
// Associated types often have a name that describes how the type will be used, and documenting the associated type in the API documentation is good practice.
// 关联类型通常有一个描述类型将如何使用的名称，在 API 文档中记录关联类型是一种很好的做法。

// Default Generic Type Parameters and Operator Overloading
// 默认泛型类型参数和运算符重载
// When we use generic type parameters, we can specify a default concrete type for the generic type.
// 当我们使用泛型类型参数时，我们可以为泛型指定一个默认的具体类型。
// This eliminates the need for implementors of the trait to specify a concrete type if the default type works.
// 如果默认类型有效，这消除了特征实现者指定具体类型的需要。
// You specify a default type when declaring a generic type with the <PlaceholderType=ConcreteType> syntax.
// 在使用 <PlaceholderType=ConcreteType> 语法声明泛型类型时指定默认类型。

// A great example of a situation where this technique is useful is with operator overloading, in which you customize the behavior of an operator (such as +) in particular situations.
// 这种技术有用的一个很好的例子是运算符重载，在这种情况下，您可以在特定情况下自定义运算符（例如 +）的行为。

// Rust doesn’t allow you to create your own operators or overload arbitrary operators.
// Rust 不允许您创建自己的运算符或重载任意运算符。
// But you can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator.
// 但您可以通过实现与运算符关联的特征来重载 std::ops 中列出的操作和相应特征。
// For example, in Listing 19-14 we overload the + operator to add two Point instances together.
// 例如，在示例 19-14 中，我们重载了 + 运算符以将两个 Point 实例相加。
// We do this by implementing the Add trait on a Point struct:
// 我们通过在 Point 结构上实现 Add trait 来做到这一点：

// Filename: src/main.rs

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

// Listing 19-14: Implementing the Add trait to overload the + operator for Point instances
// 示例 19-14：实现 Add 特性以重载 Point 实例的 + 运算符

// The add method adds the x values of two Point instances and the y values of two Point instances to create a new Point.
// add 方法将两个 Point 实例的 x 值和两个 Point 实例的 y 值相加，创建一个新的 Point。
// The Add trait has an associated type named Output that determines the type returned from the add method.
// Add 特征有一个名为 Output 的关联类型，它确定从 add 方法返回的类型。

// The default generic type in this code is within the Add trait. Here is its definition:
// 此代码中的默认泛型类型在 Add 特性中。 这是它的定义：

trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

// This code should look generally familiar: a trait with one method and an associated type.
// 这段代码看起来应该很熟悉：一个具有一个方法和关联类型的特征。
// The new part is Rhs=Self: this syntax is called default type parameters.
// 新部分是 Rhs=Self：这种语法称为默认类型参数。
// The Rhs generic type parameter (short for “right hand side”) defines the type of the rhs parameter in the add method.
// Rhs 泛型类型参数（“right hand side”的缩写）定义了 add 方法中 rhs 参数的类型。
// If we don’t specify a concrete type for Rhs when we implement the Add trait, the type of Rhs will default to Self, which will be the type we’re implementing Add on.
// 如果我们在实现 Add trait 时没有为 Rhs 指定具体类型，Rhs 的类型将默认为 Self，这将是我们正在实现 Add on 的类型。

// When we implemented Add for Point, we used the default for Rhs because we wanted to add two Point instances.
// 当我们为 Point 实现 Add 时，我们使用 Rhs 的默认值，因为我们想添加两个 Point 实例。
// Let’s look at an example of implementing the Add trait where we want to customize the Rhs type rather than using the default.
// 让我们看一个实现 Add trait 的例子，我们想要自定义 Rhs 类型而不是使用默认值。

// We have two structs, Millimeters and Meters, holding values in different units.
// 我们有两个结构，毫米和米，以不同的单位保存值。
// This thin wrapping of an existing type in another struct is known as the newtype pattern,
// 这种在另一个结构中对现有类型的精简包装称为新类型模式，
//  which we describe in more detail in the “Using the Newtype Pattern to Implement External Traits on External Types” section.
//  我们在“使用 Newtype 模式在外部类型上实现外部特征”一节中有更详细的描述。
// We want to add values in millimeters to values in meters and have the implementation of Add do the conversion correctly.
// 我们想要将以毫米为单位的值添加到以米为单位的值，并让 Add 的实现正确地进行转换。
// We can implement Add for Millimeters with Meters as the Rhs, as shown in Listing 19-15.
// 我们可以实现以米为 Rhs 的毫米加法，如示例 19-15 所示。

// Filename: src/lib.rs

use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Listing 19-15: Implementing the Add trait on Millimeters to add Millimeters to Meters
// 示例 19-15：在 Millimeters 上实现 Add trait 以将 Millimeters 添加到 Meters

// To add Millimeters and Meters, we specify impl Add<Meters> to set the value of the Rhs type parameter instead of using the default of Self.
// 要添加毫米和米，我们指定 impl Add<Meters> 来设置 Rhs 类型参数的值，而不是使用默认值 Self。

// You’ll use default type parameters in two main ways:
// 您将以两种主要方式使用默认类型参数：

// * To extend a type without breaking existing code
// * 在不破坏现有代码的情况下扩展类型
// * To allow customization in specific cases most users won’t need
// * 允许在大多数用户不需要的特定情况下进行自定义

// The standard library’s Add trait is an example of the second purpose: usually, you’ll add two like types, but the Add trait provides the ability to customize beyond that.
// 标准库的 Add trait 是第二个目的的一个例子：通常，你会添加两个相似的类型，但是 Add trait 提供了自定义的能力。
// Using a default type parameter in the Add trait definition means you don’t have to specify the extra parameter most of the time.
// 在 Add trait 定义中使用默认类型参数意味着大多数时候你不必指定额外的参数。
// In other words, a bit of implementation boilerplate isn’t needed, making it easier to use the trait.
// 换句话说，不需要一些实现样板，这使得使用 trait 更容易。

// The first purpose is similar to the second but in reverse:
// 第一个目的与第二个相似但相反：
//  if you want to add a type parameter to an existing trait, you can give it a default to allow extension of the functionality of the trait without breaking the existing implementation code.
//  如果你想给现有的特征添加一个类型参数，你可以给它一个默认值，以允许在不破坏现有实现代码的情况下扩展特征的功能。

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
// 消除歧义的完全限定语法：调用同名方法
// Nothing in Rust prevents a trait from having a method with the same name as another trait’s method, nor does Rust prevent you from implementing both traits on one type.
// Rust 中没有任何内容可以阻止一个 trait 拥有与另一个 trait 的方法同名的方法，Rust 也不会阻止您在一个类型上实现两个 trait。
// It’s also possible to implement a method directly on the type with the same name as methods from traits.
// 也可以直接在与 traits 中的方法同名的类型上实现方法。

// When calling methods with the same name, you’ll need to tell Rust which one you want to use.
// 当调用同名方法时，你需要告诉 Rust 你想使用哪一个。
// Consider the code in Listing 19-16 where we’ve defined two traits, Pilot and Wizard, that both have a method called fly.
// 考虑示例 19-16 中的代码，其中我们定义了两个特征，Pilot 和 Wizard，它们都有一个名为 fly 的方法。
// We then implement both traits on a type Human that already has a method named fly implemented on it.
// 然后我们在一个已经实现了名为 fly 的方法的 Human 类型上实现这两个特征。
// Each fly method does something different.
// 每个 fly 方法都做一些不同的事情。

// Filename: src/main.rs

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Listing 19-16: Two traits are defined to have a fly method and are implemented on the Human type, and a fly method is implemented on Human directly
// 示例 19-16：两个 trait 定义了一个 fly 方法并在 Human 类型上实现，一个 fly 方法直接在 Human 上实现

// When we call fly on an instance of Human, the compiler defaults to calling the method that is directly implemented on the type, as shown in Listing 19-17.
// 当我们在 Human 的实例上调用 fly 时，编译器默认调用直接在该类型上实现的方法，如清单 19-17 所示。

// Filename: src/main.rs
// 文件名：src/main.rs

fn main() {
    let person = Human;
    person.fly();
}

// Listing 19-17: Calling fly on an instance of Human
// 示例 19-17：在 Human 实例上调用 fly

// Running this code will print *waving arms furiously*, showing that Rust called the fly method implemented on Human directly.
// 运行这段代码会打印 *waving arms furiously*，表明 Rust 直接调用了在 Human 上实现的 fly 方法。

// To call the fly methods from either the Pilot trait or the Wizard trait, we need to use more explicit syntax to specify which fly method we mean. Listing 19-18 demonstrates this syntax.
// 要从 Pilot trait 或 Wizard trait 调用 fly 方法，我们需要使用更明确的语法来指定我们指的是哪种 fly 方法。 清单 19-18 演示了这种语法。

// Filename: src/main.rs

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

// Listing 19-18: Specifying which trait’s fly method we want to call
// 示例 19-18：指定我们要调用哪个 trait 的 fly 方法

// Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call.
// 在方法名称之前指定特征名称向 Rust 阐明了我们要调用 fly 的哪个实现。
// We could also write Human::fly(&person), which is equivalent to the person.fly() that we used in Listing 19-18, but this is a bit longer to write if we don’t need to disambiguate.
// 我们也可以写成 Human::fly(&person)，它等同于我们在示例 19-18 中使用的 person.fly()，但是如果我们不需要消除歧义的话，这个写法会稍微长一些。

// Running this code prints the following:
// 运行此代码打印以下内容：

$ cargo run
   Compiling traits-example v0.1.0 (file:///projects/traits-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/traits-example`
This is your captain speaking.
Up!
*waving arms furiously*

// Because the fly method takes a self parameter, if we had two types that both implement one trait, Rust could figure out which implementation of a trait to use based on the type of self.
// 因为 fly 方法接受一个 self 参数，如果我们有两个类型都实现一个特征，Rust 可以根据 self 的类型确定使用哪个特征实现。

// However, associated functions that are not methods don’t have a self parameter.
// 但是，不是方法的关联函数没有 self 参数。
// When there are multiple types or traits that define non-method functions with the same function name,
// 当有多个类型或特征定义了具有相同函数名的非方法函数时，
//  Rust doesn't always know which type you mean unless you use fully qualified syntax.
// Rust 并不总是知道你指的是哪种类型，除非你使用完全限定的语法。
// For example, in Listing 19-19 we create a trait for an animal shelter that wants to name all baby dogs Spot.
// 例如，在示例 19-19 中，我们为动物收容所创建了一个特征，它希望将所有幼犬命名为 Spot。
// We make an Animal trait with an associated non-method function baby_name.
// 我们用关联的非方法函数 baby_name 创建一个 Animal 特征。
// The Animal trait is implemented for the struct Dog, on which we also provide an associated non-method function baby_name directly.
// Animal trait 是为 struct Dog 实现的，我们还直接在其上提供了一个关联的非方法函数 baby_name。

// Filename: src/main.rs

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}

// Listing 19-19: A trait with an associated function and a type with an associated function of the same name that also implements the trait
// 示例 19-19：具有关联函数的特征和具有实现该特征的同名关联函数的类型

// We implement the code for naming all puppies Spot in the baby_name associated function that is defined on Dog.
// 我们在 Dog 上定义的 baby_name 关联函数中实现了为所有小狗命名 Spot 的代码。
// The Dog type also implements the trait Animal, which describes characteristics that all animals have.
// Dog 类型还实现了 Animal 特征，它描述了所有动物都具有的特征。
// Baby dogs are called puppies, and that is expressed in the implementation of the Animal trait on Dog in the baby_name function associated with the Animal trait.
// 小狗被称为小狗，这体现在与动物特征关联的 baby_name 函数中 Dog 的动物特征的实现中。

// In main, we call the Dog::baby_name function, which calls the associated function defined on Dog directly. This code prints the following:
// 在 main 中，我们调用了 Dog::baby_name 函数，它直接调用了定义在 Dog 上的关联函数。 此代码打印以下内容：

// $ cargo run
//    Compiling traits-example v0.1.0 (file:///projects/traits-example)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.54s
//      Running `target/debug/traits-example`
// A baby dog is called a Spot

// This output isn’t what we wanted.
// 这个输出不是我们想要的。
// We want to call the baby_name function that is part of the Animal trait that we implemented on Dog so the code prints A baby dog is called a puppy.
// 我们想要调用 baby_name 函数，它是我们在 Dog 上实现的 Animal 特性的一部分，因此代码会打印 A baby dog is called a puppy。
// The technique of specifying the trait name that we used in Listing 19-18 doesn’t help here;
// 我们在示例 19-18 中使用的指定特征名称的技术在这里没有帮助；
// if we change main to the code in Listing 19-20, we’ll get a compilation error.
// 如果我们将 main 更改为示例 19-20 中的代码，我们将得到一个编译错误。

// Filename: src/main.rs

fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}

// Listing 19-20: Attempting to call the baby_name function from the Animal trait, but Rust doesn’t know which implementation to use
// 示例 19-20：尝试从 Animal 特性调用 baby_name 函数，但 Rust 不知道使用哪个实现

// Because Animal::baby_name doesn’t have a self parameter, and there could be other types that implement the Animal trait,
// 因为 Animal::baby_name 没有 self 参数，并且可能有其他类型实现了 Animal 特性，
//  Rust can’t figure out which implementation of Animal::baby_name we want.
//  Rust 无法确定我们想要 Animal::baby_name 的哪个实现。
// We’ll get this compiler error:
// 我们会得到这个编译器错误：

// $ cargo run
//    Compiling traits-example v0.1.0 (file:///projects/traits-example)
// error[E0283]: type annotations needed
//   --> src/main.rs:20:43
//    |
// 20 |     println!("A baby dog is called a {}", Animal::baby_name());
//    |                                           ^^^^^^^^^^^^^^^^^ cannot infer type
//    |
//    = note: cannot satisfy `_: Animal`
//
// For more information about this error, try `rustc --explain E0283`.
// error: could not compile `traits-example` due to previous error

// To disambiguate and tell Rust that we want to use the implementation of Animal for Dog as opposed to the implementation of Animal for some other type,
// 为了消除歧义并告诉 Rust 我们想要对 Dog 使用 Animal 的实现，而不是对其他类型使用 Animal 的实现，
//  we need to use fully qualified syntax. Listing 19-21 demonstrates how to use fully qualified syntax.
//  我们需要使用完全限定的语法。 清单 19-21 演示了如何使用完全限定语法。

// Filename: src/main.rs

fn main() {
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// Listing 19-21: Using fully qualified syntax to specify that we want to call the baby_name function from the Animal trait as implemented on Dog
// 示例 19-21：使用完全限定的语法指定我们要从在 Dog 上实现的 Animal 特征调用 baby_name 函数

// We’re providing Rust with a type annotation within the angle brackets,
// 我们在尖括号内为 Rust 提供类型注释，
//  which indicates we want to call the baby_name method from the Animal trait as implemented on Dog by saying that we want to treat the Dog type as an Animal for this function call.
// 这表示我们想从在 Dog 上实现的 Animal 特性调用 baby_name 方法，表示我们想将 Dog 类型视为此函数调用的 Animal。
// This code will now print what we want:
// 这段代码现在将打印我们想要的内容：

// $ cargo run
//    Compiling traits-example v0.1.0 (file:///projects/traits-example)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.48s
//      Running `target/debug/traits-example`
// A baby dog is called a puppy

// In general, fully qualified syntax is defined as follows:
// 通常，完全限定语法定义如下：

<Type as Trait>::function(receiver_if_method, next_arg, ...);

// For associated functions that aren’t methods, there would not be a receiver: there would only be the list of other arguments.
// 对于不是方法的关联函数，不会有接收者：只有其他参数的列表。
// You could use fully qualified syntax everywhere that you call functions or methods.
// 你可以在调用函数或方法的任何地方使用完全限定语法。
// However, you’re allowed to omit any part of this syntax that Rust can figure out from other information in the program.
// 但是，您可以省略此语法中 Rust 可以从程序中的其他信息中找出的任何部分。
// You only need to use this more verbose syntax in cases where there are multiple implementations that use the same name and Rust needs help to identify which implementation you want to call.
// 只有在多个实现使用相同名称并且 Rust 需要帮助来识别您要调用哪个实现的情况下，您才需要使用这种更冗长的语法。

// Using Supertraits to Require One Trait’s Functionality Within Another Trait
// 使用超特征来要求一个特征在另一个特征中的功能
// Sometimes, you might write a trait definition that depends on another trait: for a type to implement the first trait, you want to require that type to also implement the second trait.
// 有时，您可能会编写依赖于另一个特征的特征定义：对于实现第一个特征的类型，您希望要求该类型也实现第二个特征。
// You would do this so that your trait definition can make use of the associated items of the second trait.
// 你会这样做，以便你的特征定义可以使用第二个特征的关联项。
// The trait your trait definition is relying on is called a supertrait of your trait.
// 你的特质定义所依赖的特质被称为你的特质的超特质。

// For example, let’s say we want to make an OutlinePrint trait with an outline_print method that will print a given value formatted so that it's framed in asterisks.
// 例如，假设我们想创建一个带有 outline_print 方法的 OutlinePrint 特征，该方法将打印给定值的格式，使其以星号框起来。
// That is, given a Point struct that implements the standard library trait Display to result in (x, y),
// 也就是说，给定一个实现标准库特征 Display 的 Point 结构以产生 (x, y)，
//  when we call outline_print on a Point instance that has 1 for x and 3 for y, it should print the following:
//  当我们在 x 为 1，y 为 3 的 Point 实例上调用 outline_print 时，它应该打印以下内容：

**********
*        *
* (1, 3) *
*        *
**********

// In the implementation of the outline_print method, we want to use the Display trait’s functionality.
// 在 outline_print 方法的实现中，我们想要使用 Display 特性的功能。
// Therefore, we need to specify that the OutlinePrint trait will work only for types that also implement Display and provide the functionality that OutlinePrint needs.
// 因此，我们需要指定 OutlinePrint 特性仅适用于同样实现 Display 并提供 OutlinePrint 所需功能的类型。
// We can do that in the trait definition by specifying OutlinePrint: Display.
// 我们可以通过指定 OutlinePrint: Display 在特征定义中做到这一点。
// This technique is similar to adding a trait bound to the trait.
// 这种技术类似于添加绑定到特征的特征。
// Listing 19-22 shows an implementation of the OutlinePrint trait.
// 清单 19-22 显示了 OutlinePrint 特性的实现。

// Filename: src/main.rs

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Listing 19-22: Implementing the OutlinePrint trait that requires the functionality from Display
// 示例 19-22：实现需要 Display 功能的 OutlinePrint 特性

// Because we’ve specified that OutlinePrint requires the Display trait, we can use the to_string function that is automatically implemented for any type that implements Display.
// 因为我们已经指定 OutlinePrint 需要 Display 特性，所以我们可以使用为任何实现 Display 的类型自动实现的 to_string 函数。
// If we tried to use to_string without adding a colon and specifying the Display trait after the trait name,
// 如果我们尝试使用 to_string 而不添加冒号并在特征名称后指定 Display 特征，
//  we’d get an error saying that no method named to_string was found for the type &Self in the current scope.
//  我们会收到一条错误消息，指出在当前作用域中没有为 &Self 类型找到名为 to_string 的方法。

// Let’s see what happens when we try to implement OutlinePrint on a type that doesn’t implement Display, such as the Point struct:
// 让我们看看当我们尝试在没有实现 Display 的类型上实现 OutlinePrint 时会发生什么，例如 Point 结构：

// Filename: src/main.rs

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

// We get an error saying that Display is required but not implemented:
// 我们收到一条错误消息，指出需要显示但未实现：

// $ cargo run
//    Compiling traits-example v0.1.0 (file:///projects/traits-example)
// error[E0277]: `Point` doesn't implement `std::fmt::Display`
//   --> src/main.rs:20:6
//    |
// 20 | impl OutlinePrint for Point {}
//    |      ^^^^^^^^^^^^ `Point` cannot be formatted with the default formatter
//    |
//    = help: the trait `std::fmt::Display` is not implemented for `Point`
//    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
// note: required by a bound in `OutlinePrint`
//   --> src/main.rs:3:21
//    |
// 3  | trait OutlinePrint: fmt::Display {
//    |                     ^^^^^^^^^^^^ required by this bound in `OutlinePrint`
//
// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `traits-example` due to previous error

// To fix this, we implement Display on Point and satisfy the constraint that OutlinePrint requires, like so:
// 为了解决这个问题，我们在 Point 上实现 Display 并满足 OutlinePrint 要求的约束，如下所示：

// Filename: src/main.rs

use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Then implementing the OutlinePrint trait on Point will compile successfully, and we can call outline_print on a Point instance to display it within an outline of asterisks.
// 然后在 Point 上实现 OutlinePrint trait 将编译成功，我们可以在 Point 实例上调用 outline_print 以将其显示在星号轮廓内。

// Using the Newtype Pattern to Implement External Traits on External Types
// 使用 Newtype 模式在外部类型上实现外部特征
// In Chapter 10 in the “Implementing a Trait on a Type” section,
// 在第 10 章的“在类型上实现特征”部分，
//  we mentioned the orphan rule that states we’re only allowed to implement a trait on a type if either the trait or the type are local to our crate.
//  我们提到了孤儿规则，它规定我们只允许在类型上实现一个特征，如果特征或类型是我们 crate 的本地特征。
// It’s possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct.
// 可以使用 newtype 模式绕过此限制，它涉及在元组结构中创建新类型。
//  (We covered tuple structs in the “Using Tuple Structs without Named Fields to Create Different Types” section of Chapter 5.)
// （我们在第 5 章的“使用不带命名字段的元组结构来创建不同类型”部分介绍了元组结构。）
// The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for.
// 元组结构将有一个字段，并且是我们要为其实现特征的类型的薄包装。
// Then the wrapper type is local to our crate, and we can implement the trait on the wrapper.
// 然后包装器类型是我们包装箱的本地类型，我们可以在包装器上实现特征。
// Newtype is a term that originates from the Haskell programming language.
// Newtype 是一个源自 Haskell 编程语言的术语。
// There is no runtime performance penalty for using this pattern, and the wrapper type is elided at compile time.
// 使用此模式不会造成运行时性能损失，并且在编译时省略了包装器类型。

// As an example, let’s say we want to implement Display on Vec<T>,
// 例如，假设我们要在 Vec<T> 上实现 Display，
//  which the orphan rule prevents us from doing directly because the Display trait and the Vec<T> type are defined outside our crate.
//  孤儿规则阻止我们直接这样做，因为 Display 特征和 Vec<T> 类型是在我们的包装箱外定义的。
// We can make a Wrapper struct that holds an instance of Vec<T>; then we can implement Display on Wrapper and use the Vec<T> value, as shown in Listing 19-23.
// 我们可以创建一个包含 Vec<T> 实例的 Wrapper 结构； 然后我们可以在 Wrapper 上实现 Display 并使用 Vec<T> 值，如示例 19-23 所示。

// Filename: src/main.rs

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

// Listing 19-23: Creating a Wrapper type around Vec<String> to implement Display
// 示例 19-23：围绕 Vec<String> 创建一个 Wrapper 类型来实现 Display

// The implementation of Display uses self.0 to access the inner Vec<T>, because Wrapper is a tuple struct and Vec<T> is the item at index 0 in the tuple.
// Display 的实现使用 self.0 来访问内部 Vec<T>，因为 Wrapper 是一个元组结构，而 Vec<T> 是元组中索引为 0 的项。
// Then we can use the functionality of the Display type on Wrapper.
// 然后我们可以在 Wrapper 上使用 Display 类型的功能。

// The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value it’s holding.
// 使用这种技术的缺点是 Wrapper 是一种新类型，因此它没有它持有的值的方法。
// We would have to implement all the methods of Vec<T> directly on Wrapper such that the methods delegate to self.0, which would allow us to treat Wrapper exactly like a Vec<T>.
// 我们必须直接在 Wrapper 上实现 Vec<T> 的所有方法，这样这些方法就会委托给 self.0，这样我们就可以像对待 Vec<T> 一样对待 Wrapper。
// If we wanted the new type to have every method the inner type has,
// 如果我们希望新类型拥有内部类型拥有的所有方法，
//  implementing the Deref trait (discussed in Chapter 15 in the “Treating Smart Pointers Like Regular References with the Deref Trait” section)
//  实现 Deref 特性（在第 15 章的“使用 Deref 特性将智能指针视为常规引用”部分中讨论）
//  on the Wrapper to return the inner type would be a solution.
//  在 Wrapper 上返回内部类型将是一个解决方案。
// If we don’t want the Wrapper type to have all the methods of the inner type—for example,
// 如果我们不希望 Wrapper 类型拥有内部类型的所有方法——例如，
//  to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.
//  限制 Wrapper 类型的行为——我们必须手动实现我们确实需要的方法。

// This newtype pattern is also useful even when traits are not involved.
// 即使不涉及特征，这种新类型模式也很有用。
// Let’s switch focus and look at some advanced ways to interact with Rust’s type system.
// 让我们切换焦点，看看与 Rust 类型系统交互的一些高级方法。


// Advanced Types
// 高级类型
// The Rust type system has some features that we’ve so far mentioned but haven’t yet discussed.
// Rust 类型系统有一些我们目前已经提到但尚未讨论的特性。
// We’ll start by discussing newtypes in general as we examine why newtypes are useful as types.
// 当我们检查为什么新类型作为类型有用时，我们将从一般性地讨论新类型开始。
// Then we’ll move on to type aliases, a feature similar to newtypes but with slightly different semantics.
// 然后我们将继续类型别名，这是一个类似于新类型但语义略有不同的特性。
// We’ll also discuss the ! type and dynamically sized types.
// 我们还将讨论 ! 类型和动态大小的类型。

// Using the Newtype Pattern for Type Safety and Abstraction
// 使用 Newtype 模式进行类型安全和抽象
// Note: This section assumes you’ve read the earlier section “Using the Newtype Pattern to Implement External Traits on External Types.”
// 注意：本节假设您已经阅读了前面的部分“使用 Newtype 模式在外部类型上实现外部特征”。
// The newtype pattern is also useful for tasks beyond those we’ve discussed so far, including statically enforcing that values are never confused and indicating the units of a value.
// newtype 模式对于我们目前讨论的任务之外的任务也很有用，包括静态地强制值永远不会混淆并指示值的单位。
// You saw an example of using newtypes to indicate units in Listing 19-15: recall that the Millimeters and Meters structs wrapped u32 values in a newtype.
// 您在示例 19-15 中看到了使用新类型指示单位的示例：回想一下，毫米和米结构将 u32 值包装在新类型中。
// If we wrote a function with a parameter of type Millimeters, we couldn’t compile a program that accidentally tried to call that function with a value of type Meters or a plain u32.
// 如果我们编写了一个带有 Millimeters 类型参数的函数，我们将无法编译意外尝试使用 Meters 类型或普通 u32 类型的值调用该函数的程序。

// We can also use the newtype pattern to abstract away some implementation details of a type: the new type can expose a public API that is different from the API of the private inner type.
// 我们还可以使用 newtype 模式来抽象出一个类型的一些实现细节：新类型可以公开一个不同于私有内部类型 API 的公共 API。

// Newtypes can also hide internal implementation.
// Newtypes 也可以隐藏内部实现。
// For example, we could provide a People type to wrap a HashMap<i32, String> that stores a person’s ID associated with their name.
// 例如，我们可以提供一个 People 类型来包装一个 HashMap<i32, String>，它存储一个人的 ID 与其姓名相关联。
// Code using People would only interact with the public API we provide, such as a method to add a name string to the People collection;
// 使用 People 的代码只会与我们提供的公共 API 交互，例如将名称字符串添加到 People 集合的方法；
// that code wouldn’t need to know that we assign an i32 ID to names internally.
// 该代码不需要知道我们在内部为名称分配了一个 i32 ID。
// The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details,
// newtype 模式是一种实现封装以隐藏实现细节的轻量级方式，
//  which we discussed in the “Encapsulation that Hides Implementation Details” section of Chapter 17.
//  我们在第 17 章的“隐藏实现细节的封装”部分讨论过。

// Creating Type Synonyms with Type Aliases
// 使用类型别名创建类型同义词
// Rust provides the ability to declare a type alias to give an existing type another name.
// Rust 提供了声明类型别名以给现有类型另一个名称的能力。
// For this we use the type keyword. For example, we can create the alias Kilometers to i32 like so:
// 为此，我们使用 type 关键字。 例如，我们可以像这样为 i32 创建别名 Kilometers：

type Kilometers = i32;

// Now, the alias Kilometers is a synonym for i32; unlike the Millimeters and Meters types we created in Listing 19-15, Kilometers is not a separate, new type.
// 现在，别名 Kilometers 是 i32 的同义词； 与我们在示例 19-15 中创建的 Millimeters 和 Meters 类型不同，Kilometers 不是一个单独的新类型。
// Values that have the type Kilometers will be treated the same as values of type i32:
// Kilometers 类型的值将被视为与 i32 类型的值相同：

type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);

// Because Kilometers and i32 are the same type, we can add values of both types and we can pass Kilometers values to functions that take i32 parameters.
// 因为 Kilometers 和 i32 是同一类型，我们可以将两种类型的值相加，我们可以将 Kilometers 值传递给采用 i32 参数的函数。
// However, using this method, we don’t get the type checking benefits that we get from the newtype pattern discussed earlier.
// 然而，使用这种方法，我们无法获得我们从前面讨论的新类型模式中获得的类型检查优势。
// In other words, if we mix up Kilometers and i32 values somewhere, the compiler will not give us an error.
// 换句话说，如果我们在某处混淆了 Kilometers 和 i32 值，编译器不会给我们一个错误。

// The main use case for type synonyms is to reduce repetition. For example, we might have a lengthy type like this:
// 类型同义词的主要用例是减少重复。 例如，我们可能有这样一个冗长的类型：

Box<dyn Fn() + Send + 'static>

// Writing this lengthy type in function signatures and as type annotations all over the code can be tiresome and error prone.
// 在函数签名中编写这种冗长的类型并在整个代码中作为类型注释可能会让人厌烦且容易出错。
// Imagine having a project full of code like that in Listing 19-24.
// 想象一下，有一个项目充满了清单 19-24 中的代码。

let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    // --snip--
}

// Listing 19-24: Using a long type in many places
// 示例 19-24：在很多地方使用 long 类型

// A type alias makes this code more manageable by reducing the repetition.
// 类型别名通过减少重复使此代码更易于管理。
// In Listing 19-25, we’ve introduced an alias named Thunk for the verbose type and can replace all uses of the type with the shorter alias Thunk.
// 在示例 19-25 中，我们为 verbose 类型引入了一个名为 Thunk 的别名，并且可以用更短的别名 Thunk 替换该类型的所有使用。

type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}

// Listing 19-25: Introducing a type alias Thunk to reduce repetition
// 示例 19-25：引入类型别名 Thunk 以减少重复

// This code is much easier to read and write!
// 这段代码更易读易写！
// Choosing a meaningful name for a type alias can help communicate your intent as well
// 为类型别名选择一个有意义的名称也有助于传达您的意图
//  (thunk is a word for code to be evaluated at a later time, so it’s an appropriate name for a closure that gets stored).
// （thunk 是一个词，表示稍后要评估的代码，因此它是存储的闭包的适当名称）。

// Type aliases are also commonly used with the Result<T, E> type for reducing repetition.
// 类型别名也常用于 Result<T, E> 类型以减少重复。
// Consider the std::io module in the standard library. I/O operations often return a Result<T, E> to handle situations when operations fail to work.
// 考虑标准库中的 std::io 模块。 I/O 操作通常会返回一个 Result<T, E> 来处理操作失败的情况。
// This library has a std::io::Error struct that represents all possible I/O errors.
// 这个库有一个 std::io::Error 结构，代表所有可能的 I/O 错误。
// Many of the functions in std::io will be returning Result<T, E> where the E is std::io::Error, such as these functions in the Write trait:
// std::io 中的许多函数将返回 Result<T, E>，其中 E 是 std::io::Error，例如 Write 特征中的这些函数：

use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

// The Result<..., Error> is repeated a lot. As such, std::io has this type alias declaration:
// Result<..., Error> 重复了很多次。 因此，std::io 具有此类型别名声明：

type Result<T> = std::result::Result<T, std::io::Error>;

// Because this declaration is in the std::io module, we can use the fully qualified alias std::io::Result<T>;
// 因为这个声明在 std::io 模块中，我们可以使用完全限定的别名 std::io::Result<T>;
// that is, a Result<T, E> with the E filled in as std::io::Error. The Write trait function signatures end up looking like this:
// 也就是说，一个 Result<T, E>，E 填充为 std::io::Error。 Write trait 函数签名最终看起来像这样：

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// The type alias helps in two ways: it makes code easier to write and it gives us a consistent interface across all of std::io.
// 类型别名有两个方面的帮助：它使代码更容易编写，并且它为我们提供了一个跨所有 std::io 的一致接口。
// Because it’s an alias, it’s just another Result<T, E>, which means we can use any methods that work on Result<T, E> with it, as well as special syntax like the ? operator.
// 因为它是一个别名，它只是另一个 Result<T, E>，这意味着我们可以使用任何适用于 Result<T, E> 的方法，以及像 ? 操作员。

// The Never Type that Never Returns
// 永不返回的 Never 类型
// Rust has a special type named ! that’s known in type theory lingo as the empty type because it has no values.
// Rust 有一个名为 ! 在类型理论术语中称为空类型，因为它没有值。
// We prefer to call it the never type because it stands in the place of the return type when a function will never return. Here is an example:
// 我们更喜欢称它为 never 类型，因为当函数永远不会返回时，它代表返回类型。 这是一个例子：

fn bar() -> ! {
    // --snip--
}

// This code is read as “the function bar returns never.” Functions that return never are called diverging functions.
// 这段代码被解读为“函数 bar 从不返回”。 永不返回的函数称为发散函数。
// We can’t create values of the type ! so bar can never possibly return.
// 我们不能创建类型的值 ! 所以 bar 永远不可能 return。

// But what use is a type you can never create values for? Recall the code from Listing 2-5, part of the number guessing game;
// 但是你永远不能为其创建值的类型有什么用？ 回想一下清单 2-5 中的代码，猜数游戏的一部分；
// we’ve reproduced a bit of it here in Listing 19-26.
// 我们在示例 19-26 中复制了一些内容。

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

// Listing 19-26: A match with an arm that ends in continue
// 示例 19-26：一个以 continue 结尾的 arm 的匹配

// At the time, we skipped over some details in this code.
// 当时，我们跳过了这段代码中的一些细节。
// In Chapter 6 in “The match Control Flow Operator” section, we discussed that match arms must all return the same type.
// 在第 6 章的“匹配控制流运算符”部分，我们讨论了匹配武器必须全部返回相同的类型。
// So, for example, the following code doesn’t work:
// 因此，例如，以下代码不起作用：

let guess = match guess.trim().parse() {
    Ok(_) => 5,
    Err(_) => "hello",
};


// The type of guess in this code would have to be an integer and a string, and Rust requires that guess have only one type.
// 此代码中的猜测类型必须是整数和字符串，而 Rust 要求猜测只有一种类型。
// So what does continue return? How were we allowed to return a u32 from one arm and have another arm that ends with continue in Listing 19-26?
// 那么 continue 返回什么？ 在示例 19-26 中，我们如何允许从一只手臂返回一个 u32 而另一只手臂以 continue 结尾？

// As you might have guessed, continue has a ! value. That is, when Rust computes the type of guess, it looks at both match arms, the former with a value of u32 and the latter with a ! value.
// 你可能已经猜到了，continue 有一个 ! 价值。 也就是说，当 Rust 计算猜测的类型时，它会查看两个匹配臂，前者的值为 u32，后者的值为 ! 价值。
// Because ! can never have a value, Rust decides that the type of guess is u32.
// 因为 ！ 永远不可能有值，Rust 决定猜测的类型是 u32。

// The formal way of describing this behavior is that expressions of type ! can be coerced into any other type.
// 描述此行为的正式方式是类型表达式 ! 可以强制转换为任何其他类型。
// We’re allowed to end this match arm with continue because continue doesn’t return a value;
// 我们可以用 continue 结束这个 match arm，因为 continue 不返回值；
// instead, it moves control back to the top of the loop, so in the Err case, we never assign a value to guess.
// 相反，它将控制移回循环的顶部，因此在 Err 的情况下，我们永远不会分配一个值来猜测。

// The never type is useful with the panic! macro as well.
// never 类型对 panic! 宏也一样很有用。
// Recall the unwrap function that we call on Option<T> values to produce a value or panic with this definition:
// 回忆一下我们在 Option<T> 值上调用的 unwrap 函数，以使用此定义生成一个值或 panic：

impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

// In this code, the same thing happens as in the match in Listing 19-26: Rust sees that val has the type T and panic! has the type !, so the result of the overall match expression is T.
// 在这段代码中，发生了与示例 19-26 中的匹配相同的事情：Rust 看到 val 的类型为 T 和 panic！ 具有类型 !，因此整体匹配表达式的结果为 T。
// This code works because panic! doesn’t produce a value; it ends the program.
// In the None case, we won’t be returning a value from unwrap, so this code is valid.
// 这段代码有效，因为恐慌！ 不产生价值； 它结束了程序。

// One final expression that has the type ! is a loop:
// 在 None 的情况下，我们不会从 unwrap 返回值，所以这段代码是有效的。

// 一个类型为 ! 是一个循环：

print!("forever ");

loop {
    print!("and ever ");
}

// Here, the loop never ends, so ! is the value of the expression.
// 在这里，循环永远不会结束，所以 ! 是表达式的值。
// However, this wouldn’t be true if we included a break, because the loop would terminate when it got to the break.
// 但是，如果我们包含一个 break，这就不是真的了，因为循环会在到达 break 时终止。

// Dynamically Sized Types and the Sized Trait
// 动态大小类型和大小特征
// Rust needs to know certain details about its types, such as how much space to allocate for a value of a particular type.
// Rust 需要知道有关其类型的某些细节，例如为特定类型的值分配多少空间。
// This leaves one corner of its type system a little confusing at first: the concept of dynamically sized types.
// 这让其类型系统的一个角落一开始有点混乱：动态大小类型的概念。
// Sometimes referred to as DSTs or unsized types, these types let us write code using values whose size we can know only at runtime.
// 有时称为 DST 或未确定大小的类型，这些类型让我们可以使用只有在运行时才能知道其大小的值来编写代码。

// Let’s dig into the details of a dynamically sized type called str, which we’ve been using throughout the book.
// 让我们深入了解一个称为 str 的动态大小类型的细节，我们在整本书中都在使用它。
// That’s right, not &str, but str on its own, is a DST.
// 没错，不是 &str，而是 str 本身就是一个 DST。
// We can’t know how long the string is until runtime, meaning we can’t create a variable of type str, nor can we take an argument of type str.
// 直到运行时我们才能知道字符串有多长，这意味着我们不能创建 str 类型的变量，也不能接受 str 类型的参数。
// Consider the following code, which does not work:
// 考虑下面的代码，它不起作用：

let s1: str = "Hello there!";
let s2: str = "How's it going?";

// Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type must use the same amount of memory.
// Rust 需要知道为特定类型的任何值分配多少内存，并且一个类型的所有值必须使用相同数量的内存。
// If Rust allowed us to write this code, these two str values would need to take up the same amount of space.
// 如果 Rust 允许我们编写这段代码，这两个 str 值将需要占用相同的空间量。
// But they have different lengths: s1 needs 12 bytes of storage and s2 needs 15.
// 但它们的长度不同：s1 需要 12 个字节的存储空间，s2 需要 15 个字节。
// This is why it’s not possible to create a variable holding a dynamically sized type.
// 这就是为什么不能创建一个包含动态大小类型的变量的原因。

// So what do we do? In this case, you already know the answer: we make the types of s1 and s2 a &str rather than a str.
// 那么我们该怎么办？ 在这种情况下，您已经知道答案：我们将 s1 和 s2 的类型设为 &str 而不是 str。
// Recall from the “String Slices” section of Chapter 4 that the slice data structure just stores the starting position and the length of the slice.
// 回忆第 4 章的“字符串切片”部分，切片数据结构只存储切片的起始位置和长度。
// So although a &T is a single value that stores the memory address of where the T is located, a &str is two values: the address of the str and its length.
// 因此虽然 &T 是一个存储 T 所在内存地址的单个值，但 &str 是两个值：str 的地址及其长度。
// As such, we can know the size of a &str value at compile time: it’s twice the length of a usize.
// 因此，我们可以在编译时知道 &str 值的大小：它是 usize 长度的两倍。
// That is, we always know the size of a &str, no matter how long the string it refers to is.
// 也就是说，我们总是知道 &str 的大小，无论它引用的字符串有多长。
// In general, this is the way in which dynamically sized types are used in Rust: they have an extra bit of metadata that stores the size of the dynamic information.
// 通常，这是在 Rust 中使用动态大小类型的方式：它们有额外的元数据位来存储动态信息的大小。
// The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.
// 动态大小类型的黄金法则是我们必须始终将动态大小类型的值放在某种指针后面。

// We can combine str with all kinds of pointers: for example, Box<str> or Rc<str>.
// 我们可以将 str 与各种指针组合：例如，Box<str> 或 Rc<str>。
// In fact, you’ve seen this before but with a different dynamically sized type: traits.
// 事实上，您之前已经看到过这个，但是使用了不同的动态大小类型：traits。
// Every trait is a dynamically sized type we can refer to by using the name of the trait.
// 每个 trait 都是一个动态大小的类型，我们可以使用 trait 的名称来引用。
// In Chapter 17 in the “Using Trait Objects That Allow for Values of Different Types” section,
// 在第 17 章的“使用允许不同类型值的特征对象”部分中，
//  we mentioned that to use traits as trait objects, we must put them behind a pointer, such as &dyn Trait or Box<dyn Trait> (Rc<dyn Trait> would work too).
//  我们提到要将特征用作特征对象，我们必须将它们放在指针后面，例如 &dyn Trait 或 Box<dyn Trait>（Rc<dyn Trait> 也可以）。

// To work with DSTs, Rust provides the Sized trait to determine whether or not a type’s size is known at compile time.
// 为了使用 DST，Rust 提供了 Sized 特性来确定类型的大小在编译时是否已知。
// This trait is automatically implemented for everything whose size is known at compile time.
// 对于在编译时已知大小的所有对象，都会自动实现此特征。
// In addition, Rust implicitly adds a bound on Sized to every generic function.
// 此外，Rust 隐式地为每个泛型函数添加了 Sized 的界限。
// That is, a generic function definition like this:
// 也就是说，像这样的通用函数定义：

fn generic<T>(t: T) {
    // --snip--
}

// is actually treated as though we had written this:
// 实际上被当作我们写过的：

fn generic<T: Sized>(t: T) {
    // --snip--
}

// By default, generic functions will work only on types that have a known size at compile time.
// 默认情况下，泛型函数仅适用于在编译时具有已知大小的类型。
// However, you can use the following special syntax to relax this restriction:
// 但是，您可以使用以下特殊语法来放宽此限制：

fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

// A trait bound on ?Sized means “T may or may not be Sized” and this notation overrides the default that generic types must have a known size at compile time.
// 绑定在 ?Sized 上的特征意味着“T 可以调整大小，也可以不调整大小”，并且此表示法覆盖了泛型类型在编译时必须具有已知大小的默认值。
// The ?Trait syntax with this meaning is only available for Sized, not any other traits.
// 具有此含义的 ?Trait 语法仅适用于 Sized，不适用于任何其他特征。

// Also note that we switched the type of the t parameter from T to &T.
// 另请注意，我们将 t 参数的类型从 T 切换为 &T。
// Because the type might not be Sized, we need to use it behind some kind of pointer.
// 因为类型可能不是 Sized，所以我们需要在某种指针后面使用它。
// In this case, we’ve chosen a reference.
// 在这种情况下，我们选择了一个引用。

// Next, we’ll talk about functions and closures!
// 接下来，我们将讨论函数和闭包！

// Advanced Functions and Closures
// 高级函数和闭包
// This section explores some advanced features related to functions and closures, including function pointers and returning closures.
// 本节探讨一些与函数和闭包相关的高级特性，包括函数指针和返回闭包。

// Function Pointers
// 函数指针
// We’ve talked about how to pass closures to functions; you can also pass regular functions to functions!
// 我们已经讨论过如何将闭包传递给函数； 您还可以将常规函数传递给函数！
// This technique is useful when you want to pass a function you’ve already defined rather than defining a new closure.
// 当你想传递一个你已经定义的函数而不是定义一个新的闭包时，这个技术很有用。
// Functions coerce to the type fn (with a lowercase f), not to be confused with the Fn closure trait.
// 函数强制转换为 fn 类型（带有小写的 f），不要与 Fn 闭包特征混淆。
// The fn type is called a function pointer.
// fn 类型称为函数指针。
// Passing functions with function pointers will allow you to use functions as arguments to other functions.
// 使用函数指针传递函数将允许您将函数用作其他函数的参数。

// The syntax for specifying that a parameter is a function pointer is similar to that of closures, as shown in Listing 19-27,
// 指定参数为函数指针的语法类似于闭包，如示例 19-27 所示，
//  where we’ve defined a function add_one that adds one to its parameter.
//  在这里我们定义了一个函数 add_one 将其参数加一。
// The function do_twice takes two parameters: a function pointer to any function that takes an i32 parameter and returns an i32, and one i32 value.
// 函数 do_twice 有两个参数：一个指向任何接受 i32 参数并返回 i32 的函数的函数指针，以及一个 i32 值。
// The do_twice function calls the function f twice, passing it the arg value, then adds the two function call results together.
// do_twice 函数调用函数 f 两次，将 arg 值传递给它，然后将两次函数调用结果相加。
// The main function calls do_twice with the arguments add_one and 5.
// 主函数使用参数 add_one 和 5 调用 do_twice。

// Filename: src/main.rs

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

// Listing 19-27: Using the fn type to accept a function pointer as an argument
// 示例 19-27：使用 fn 类型接受函数指针作为参数

// This code prints The answer is: 12. We specify that the parameter f in do_twice is an fn that takes one parameter of type i32 and returns an i32.
// 这段代码打印出 The answer is: 12. 我们指定 do_twice 中的参数 f 是一个 fn，它接受一个 i32 类型的参数并返回一个 i32。
// We can then call f in the body of do_twice. In main, we can pass the function name add_one as the first argument to do_twice.
// 然后我们可以在 do_twice 的主体中调用 f。 在 main 中，我们可以将函数名 add_one 作为第一个参数传递给 do_twice。

// Unlike closures, fn is a type rather than a trait, so we specify fn as the parameter type directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound.
// 与闭包不同，fn 是一种类型而不是特征，因此我们直接将 fn 指定为参数类型，而不是将具有 Fn 特征之一的泛型类型参数声明为trait bound。

// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), meaning you can always pass a function pointer as an argument for a function that expects a closure.
// 函数指针实现了所有三个闭包特征（Fn、FnMut 和 FnOnce），这意味着您始终可以将函数指针作为参数传递给需要闭包的函数。
// It’s best to write functions using a generic type and one of the closure traits so your functions can accept either functions or closures.
// 最好使用泛型类型和闭包特征之一编写函数，这样您的函数就可以接受函数或闭包。

// That said, one example of where you would want to only accept fn and not closures is when interfacing with external code that doesn’t have closures:
// 也就是说，您可能只想接受 fn 而不是闭包的一个示例是在与没有闭包的外部代码交互时：
//  C functions can accept functions as arguments, but C doesn’t have closures.
//  C 函数可以接受函数作为参数，但 C 没有闭包。

// As an example of where you could use either a closure defined inline or a named function, let’s look at a use of the map method provided by the Iterator trait in the standard library.
// 作为可以使用内联定义的闭包或命名函数的示例，让我们看一下标准库中 Iterator 特性提供的 map 方法的使用。
// To use the map function to turn a vector of numbers into a vector of strings, we could use a closure, like this:
// 要使用 map 函数将数字向量转换为字符串向量，我们可以使用闭包，如下所示：

let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> =
    list_of_numbers.iter().map(|i| i.to_string()).collect();

// Or we could name a function as the argument to map instead of the closure, like this:
// 或者我们可以将函数命名为 map 的参数而不是闭包，如下所示：

let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> =
    list_of_numbers.iter().map(ToString::to_string).collect();

// Note that we must use the fully qualified syntax that we talked about earlier in the “Advanced Traits” section because there are multiple functions available named to_string.
// 请注意，我们必须使用我们之前在“高级特征”部分中讨论过的完全限定语法，因为有多个名为 to_string 的函数可用。
// Here, we’re using the to_string function defined in the ToString trait, which the standard library has implemented for any type that implements Display.
// 在这里，我们使用了 ToString 特征中定义的 to_string 函数，标准库已经为任何实现 Display 的类型实现了该函数。

// Recall from the “Enum values” section of Chapter 6 that the name of each enum variant that we define also becomes an initializer function.
// 回忆第 6 章的“枚举值”部分，我们定义的每个枚举变体的名称也成为一个初始化函数。
// We can use these initializer functions as function pointers that implement the closure traits,
// 我们可以将这些初始化函数用作实现闭包特征的函数指针，
//  which means we can specify the initializer functions as arguments for methods that take closures, like so:
//  这意味着我们可以将初始化函数指定为采用闭包的方法的参数，如下所示：

enum Status {
    Value(u32),
    Stop,
}

let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

// Here we create Status::Value instances using each u32 value in the range that map is called on by using the initializer function of Status::Value.
// 在这里，我们使用 Status::Value 的初始化函数调用 map 的范围内的每个 u32 值创建 Status::Value 实例。
// Some people prefer this style, and some people prefer to use closures.
// 有些人喜欢这种风格，有些人喜欢使用闭包。
// They compile to the same code, so use whichever style is clearer to you.
// 它们编译成相同的代码，所以使用你更清楚的风格。

// Returning Closures
// 返回闭包
// Closures are represented by traits, which means you can’t return closures directly.
// 闭包由 traits 表示，这意味着你不能直接返回闭包。
// In most cases where you might want to return a trait, you can instead use the concrete type that implements the trait as the return value of the function.
// 在大多数情况下，您可能想要返回一个特征，您可以改为使用实现该特征的具体类型作为函数的返回值。
// However, you can’t do that with closures because they don’t have a concrete type that is returnable; you’re not allowed to use the function pointer fn as a return type, for example.
// 然而，你不能用闭包来做到这一点，因为它们没有可返回的具体类型； 例如，您不允许使用函数指针 fn 作为返回类型。

// The following code tries to return a closure directly, but it won’t compile:
// 下面的代码试图直接返回一个闭包，但它不会编译：

fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}

// The compiler error is as follows:
// 编译报错如下：

// $ cargo build
//    Compiling functions-example v0.1.0 (file:///projects/functions-example)
// error[E0746]: return type cannot have an unboxed trait object
//  --> src/lib.rs:1:25
//   |
// 1 | fn returns_closure() -> dyn Fn(i32) -> i32 {
//   |                         ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
//   |
//   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
// help: use `impl Fn(i32) -> i32` as the return type, as all return paths are of type `[closure@src/lib.rs:2:5: 2:14]`, which implements `Fn(i32) -> i32`
//   |
// 1 | fn returns_closure() -> impl Fn(i32) -> i32 {
//   |                         ~~~~~~~~~~~~~~~~~~~
//
// For more information about this error, try `rustc --explain E0746`.
// error: could not compile `functions-example` due to previous error

// The error references the Sized trait again! Rust doesn’t know how much space it will need to store the closure.
// 错误再次引用了 Sized 特征！ Rust 不知道需要多少空间来存储闭包。
// We saw a solution to this problem earlier. We can use a trait object:
// 我们之前看到了这个问题的解决方案。 我们可以使用特征对象：

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// This code will compile just fine. For more about trait objects, refer to the section “Using Trait Objects That Allow for Values of Different Types” in Chapter 17.
// 这段代码可以正常编译。 有关特征对象的更多信息，请参阅第 17 章中的“使用允许不同类型值的特征对象”部分。

// Next, let’s look at macros!
// 接下来，我们来看宏！


// Macros
// 宏
// We’ve used macros like println! throughout this book, but we haven’t fully explored what a macro is and how it works.
// 我们使用了像 println! 这样的宏 在整本书中，我们还没有完全探索宏是什么以及它是如何工作的。
// The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:
// 术语宏指的是 Rust 中的一系列特性：带有 macro_rules 的声明性宏！ 以及三种程序宏：

// * Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// * Attribute-like macros that define custom attributes usable on any item
// * Function-like macros that look like function calls but operate on the tokens specified as their argument

// We’ll talk about each of these in turn, but first, let’s look at why we even need macros when we already have functions.
// 我们将依次讨论其中的每一个，但首先，让我们看看为什么我们已经有了函数还需要宏。

// The Difference Between Macros and Functions
// 宏和函数的区别
// Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
// 从根本上说，宏是一种编写代码的方式，它可以编写其他代码，这被称为元编程。
// In Appendix C, we discuss the derive attribute, which generates an implementation of various traits for you.
// 在附录 C 中，我们讨论了派生属性，它为您生成各种特征的实现。
// We’ve also used the println! and vec! macros throughout the book.
// 我们也使用了 println! 和 vec! 贯穿全书的宏。
// All of these macros expand to produce more code than the code you’ve written manually.
// 所有这些宏都会扩展生成比您手动编写的代码更多的代码。

// Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one of the roles of functions.
// 元编程对于减少您必须编写和维护的代码量很有用，这也是函数的作用之一。
// However, macros have some additional powers that functions don’t.
// 然而，宏有一些函数没有的额外能力。

// A function signature must declare the number and type of parameters the function has.
// 函数签名必须声明函数具有的参数的数量和类型。
// Macros, on the other hand, can take a variable number of parameters: we can call println!("hello") with one argument or println!("hello {}", name) with two arguments.
// 另一方面，宏可以接受可变数量的参数：我们可以用一个参数调用 println!("hello") 或用两个参数调用 println!("hello {}", name)。
// Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type.
// 此外，宏在编译器解释代码的含义之前展开，因此宏可以，例如，在给定类型上实现特征。
// A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.
// 函数不能，因为它在运行时被调用，并且需要在编译时实现特征。

// The downside to implementing a macro instead of a function is that macro definitions are more complex than function definitions because you’re writing Rust code that writes Rust code.
// 实现宏而不是函数的缺点是宏定义比函数定义更复杂，因为你正在编写 Rust 代码来编写 Rust 代码。
// Due to this indirection, macro definitions are generally more difficult to read, understand, and maintain than function definitions.
// 由于这种间接性，宏定义通常比函数定义更难阅读、理解和维护。

// Another important difference between macros and functions is that you must define macros or bring them into scope before you call them in a file,
// 宏和函数之间的另一个重要区别是，在文件中调用它们之前，必须先定义宏或将它们带入作用域，
//  as opposed to functions you can define anywhere and call anywhere.
// 与函数相反，您可以在任何地方定义并在任何地方调用。

// Declarative Macros with macro_rules! for General Metaprogramming
// 带有 macro_rules 的声明性宏！ 通用元编程
// The most widely used form of macros in Rust is the declarative macro.
// Rust 中使用最广泛的宏形式是声明式宏。
// These are also sometimes referred to as “macros by example,” “macro_rules! macros,” or just plain “macros.” At their core,
// 这些有时也称为“示例宏”、“macro_rules! 宏”，或者只是普通的“宏”。 在他们的核心，
//  declarative macros allow you to write something similar to a Rust match expression.
//  声明式宏允许您编写类似于 Rust 匹配表达式的内容。
// As discussed in Chapter 6, match expressions are control structures that take an expression, compare the resulting value of the expression to patterns,
// 如第 6 章所述，匹配表达式是采用表达式的控制结构，将表达式的结果值与模式进行比较，
//  and then run the code associated with the matching pattern.
//  然后运行与匹配模式关联的代码。
// Macros also compare a value to patterns that are associated with particular code: in this situation, the value is the literal Rust source code passed to the macro;
// 宏还将一个值与与特定代码关联的模式进行比较：在这种情况下，该值是传递给宏的文字 Rust 源代码；
// the patterns are compared with the structure of that source code; and the code associated with each pattern, when matched, replaces the code passed to the macro.
// 将模式与源代码的结构进行比较； 并且与每个模式关联的代码在匹配时会替换传递给宏的代码。
// This all happens during compilation.
// 这一切都发生在编译期间。

// To define a macro, you use the macro_rules! construct.
// 要定义宏，请使用 macro_rules! 构造。
// Let’s explore how to use macro_rules! by looking at how the vec! macro is defined.
// 让我们探索如何使用 macro_rules！ 通过查看 vec! 宏被定义。
// Chapter 8 covered how we can use the vec! macro to create a new vector with particular values.
// 第 8 章介绍了如何使用 vec！ 宏来创建具有特定值的新向量。
// For example, the following macro creates a new vector containing three integers:
// 例如，以下宏创建一个包含三个整数的新向量：

let v: Vec<u32> = vec![1, 2, 3];

// We could also use the vec! macro to make a vector of two integers or a vector of five string slices.
// 我们也可以使用 vec！ 宏来制作两个整数的向量或五个字符串切片的向量。
// We wouldn’t be able to use a function to do the same because we wouldn’t know the number or type of values up front.
// 我们无法使用函数来做同样的事情，因为我们无法预先知道值的数量或类型。

// Listing 19-28 shows a slightly simplified definition of the vec! macro.
// 清单 19-28 显示了 vec! 的稍微简化的定义 宏。

// Filename: src/lib.rs

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Listing 19-28: A simplified version of the vec! macro definition
// 示例 19-28：vec! 的简化版本 宏定义

// Note: The actual definition of the vec! macro in the standard library includes code to preallocate the correct amount of memory up front.
// 注意：vec! 的实际定义 标准库中的宏包含预先分配正确内存量的代码。
// That code is an optimization that we don’t include here to make the example simpler.
// 该代码是为了使示例更简单而未包含在此处的优化。

// The #[macro_export] annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope.
// #[macro_export] 注释表示只要定义宏的 crate 进入范围，就应该使该宏可用。
// Without this annotation, the macro can’t be brought into scope.
// 没有这个注解，宏就不能被引入作用域。

// We then start the macro definition with macro_rules! and the name of the macro we’re defining without the exclamation mark.
// 然后我们用 macro_rules! 开始宏定义以及我们正在定义的宏的名称，不带感叹号。
// The name, in this case vec, is followed by curly brackets denoting the body of the macro definition.
// 名称（在本例中为 vec）后跟大括号，表示宏定义的主体。

// The structure in the vec! body is similar to the structure of a match expression.
// vec! 中的结构 body 类似于匹配表达式的结构。
// Here we have one arm with the pattern ( $( $x:expr ),* ), followed by => and the block of code associated with this pattern.
// 这里我们有一个带有模式 ( $( $x:expr ),* ) 的手臂，后面跟着 => 和与这个模式相关的代码块。
// If the pattern matches, the associated block of code will be emitted.
// 如果模式匹配，将发出关联的代码块。
// Given that this is the only pattern in this macro, there is only one valid way to match; any other pattern will result in an error.
// 鉴于这是此宏中唯一的模式，因此只有一种有效的匹配方式； 任何其他模式都会导致错误。
// More complex macros will have more than one arm.
// 更复杂的宏会有不止一只手臂。

// Valid pattern syntax in macro definitions is different than the pattern syntax covered in Chapter 18 because macro patterns are matched against Rust code structure rather than values.
// 宏定义中的有效模式语法与第 18 章中涵盖的模式语法不同，因为宏模式与 Rust 代码结构而不是值匹配。
// Let’s walk through what the pattern pieces in Listing 19-28 mean; for the full macro pattern syntax, see the Rust Reference.
// 让我们来看看示例 19-28 中的模式片段是什么意思； 有关完整的宏模式语法，请参阅 Rust 参考。

// First, we use a set of parentheses to encompass the whole pattern.
// 首先，我们使用一组括号来包含整个模式。
// We use a dollar sign ($) to declare a variable in the macro system that will contain the Rust code matching the pattern.
// 我们使用美元符号 ($) 在宏系统中声明一个变量，该变量将包含与模式匹配的 Rust 代码。
// The dollar sign makes it clear this is a macro variable as opposed to a regular Rust variable.
// 美元符号清楚地表明这是一个宏变量，而不是常规的 Rust 变量。
// Next comes a set of parentheses that captures values that match the pattern within the parentheses for use in the replacement code.
// 接下来是一组括号，用于捕获与括号内的模式匹配的值，以用于替换代码。
// Within $() is $x:expr, which matches any Rust expression and gives the expression the name $x.
// 在 $() 中是 $x:expr，它匹配任何 Rust 表达式并赋予表达式名称 $x。

// The comma following $() indicates that a literal comma separator character could optionally appear after the code that matches the code in $().
// $() 后面的逗号表示可以选择在与 $() 中的代码匹配的代码之后出现文字逗号分隔符。
// The * specifies that the pattern matches zero or more of whatever precedes the *.
// * 指定模式匹配零个或多个 * 之前的内容。

// When we call this macro with vec![1, 2, 3];, the $x pattern matches three times with the three expressions 1, 2, and 3.
// 当我们用 vec![1, 2, 3]; 调用这个宏时，$x 模式与三个表达式 1、2 和 3 匹配三次。

// Now let’s look at the pattern in the body of the code associated with this arm:
// 现在让我们看看与此手臂相关的代码主体中的模式：
//  temp_vec.push() within $()* is generated for each part that matches $() in the pattern zero or more times depending on how many times the pattern matches.
//  $()* 中的 temp_vec.push() 是为匹配模式中的 $() 零次或多次的每个部分生成的，具体取决于模式匹配的次数。
// The $x is replaced with each expression matched. When we call this macro with vec![1, 2, 3];, the code generated that replaces this macro call will be the following:
// $x 被每个匹配的表达式替换。 当我们用 vec![1, 2, 3]; 调用这个宏时，生成的替换这个宏调用的代码如下：

{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}

// We’ve defined a macro that can take any number of arguments of any type and can generate code to create a vector containing the specified elements.
// 我们已经定义了一个宏，它可以接受任意数量的任意类型的参数，并可以生成代码来创建一个包含指定元素的向量。

// To learn more about how to write macros, consult the online documentation or other resources, such as “The Little Book of Rust Macros” started by Daniel Keep and continued by Lukas Wirth.
// 要了解有关如何编写宏的更多信息，请参阅在线文档或其他资源，例如由 Daniel Keep 开始并由 Lukas Wirth 继续的“The Little Book of Rust Macros”。

// Procedural Macros for Generating Code from Attributes
// 用于从属性生成代码的过程宏
// The second form of macros is the procedural macro, which acts more like a function (and is a type of procedure).
// 宏的第二种形式是过程宏，它的行为更像一个函数（并且是一种过程）。
// Procedural macros accept some code as an input, operate on that code,
// 过程宏接受一些代码作为输入，对该代码进行操作，
//  and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do.
//  并生成一些代码作为输出，而不是像声明性宏那样匹配模式并将代码替换为其他代码。
// The three kinds of procedural macros are custom derive, attribute-like, and function-like, and all work in a similar fashion.
// 这三种过程宏是自定义派生的、类属性的和类函数的，它们都以类似的方式工作。

// When creating procedural macros, the definitions must reside in their own crate with a special crate type.
// 当创建过程宏时，定义必须驻留在它们自己的具有特殊包装箱类型的包装箱中。
// This is for complex technical reasons that we hope to eliminate in the future.
// 这是出于我们希望在未来消除的复杂技术原因。
// In Listing 19-29, we show how to define a procedural macro, where some_attribute is a placeholder for using a specific macro variety.
// 在示例 19-29 中，我们展示了如何定义过程宏，其中 some_attribute 是使用特定宏种类的占位符。

// Filename: src/lib.rs

use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}

// Listing 19-29: An example of defining a procedural macro
// 示例 19-29：定义过程宏的示例

// The function that defines a procedural macro takes a TokenStream as an input and produces a TokenStream as an output.
// 定义过程宏的函数将 TokenStream 作为输入并生成 TokenStream 作为输出。
// The TokenStream type is defined by the proc_macro crate that is included with Rust and represents a sequence of tokens.
// TokenStream 类型由包含在 Rust 中的 proc_macro crate 定义，代表一系列标记。
// This is the core of the macro: the source code that the macro is operating on makes up the input TokenStream, and the code the macro produces is the output TokenStream.
// 这是宏的核心：宏运行的源代码构成输入 TokenStream，宏产生的代码是输出 TokenStream。
// The function also has an attribute attached to it that specifies which kind of procedural macro we’re creating.
// 该函数还附加了一个属性，用于指定我们正在创建哪种过程宏。
// We can have multiple kinds of procedural macros in the same crate.
// 我们可以在同一个 crate 中有多种过程宏。

// Let’s look at the different kinds of procedural macros.
// 让我们看看不同种类的过程宏。
// We’ll start with a custom derive macro and then explain the small dissimilarities that make the other forms different.
// 我们将从自定义派生宏开始，然后解释使其他形式不同的小差异。

// How to Write a Custom derive Macro
// 如何编写自定义派生宏
// Let’s create a crate named hello_macro that defines a trait named HelloMacro with one associated function named hello_macro.
// 让我们创建一个名为 hello_macro 的箱子，它定义了一个名为 HelloMacro 的特征以及一个名为 hello_macro 的关联函数。
// Rather than making our users implement the HelloMacro trait for each of their types,
// 而不是让我们的用户为他们的每个类型实现 HelloMacro 特性，
//  we’ll provide a procedural macro so users can annotate their type with #[derive(HelloMacro)] to get a default implementation of the hello_macro function.
//  我们将提供一个过程宏，以便用户可以使用 #[derive(HelloMacro)] 注释他们的类型以获得 hello_macro 函数的默认实现。
// The default implementation will print Hello, Macro! My name is TypeName! where TypeName is the name of the type on which this trait has been defined.
// 默认实现会打印 Hello, Macro! 我的名字是TypeName！ 其中 TypeName 是定义了此特征的类型的名称。
// In other words, we’ll write a crate that enables another programmer to write code like Listing 19-30 using our crate.
// 换句话说，我们将编写一个 crate，使另一个程序员能够使用我们的 crate 编写如清单 19-30 的代码。

// Filename: src/main.rs

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}

// Listing 19-30: The code a user of our crate will be able to write when using our procedural macro
// 示例 19-30：我们 crate 的用户在使用我们的过程宏时可以编写的代码

// This code will print Hello, Macro! My name is Pancakes! when we’re done. The first step is to make a new library crate, like this:
// 此代码将打印 Hello, Macro! 我叫煎饼！ 当我们完成时。 第一步是制作一个新的库箱，如下所示：

// $ cargo new hello_macro --lib

// Next, we’ll define the HelloMacro trait and its associated function:
// 接下来，我们将定义 HelloMacro 特征及其相关函数：

// Filename: src/lib.rs

pub trait HelloMacro {
    fn hello_macro();
}

// We have a trait and its function. At this point, our crate user could implement the trait to achieve the desired functionality, like so:
// 我们有一个特征及其函数。 在这一点上，我们的板条箱用户可以实现特性来实现所需的功能，如下所示：

use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}

// However, they would need to write the implementation block for each type they wanted to use with hello_macro; we want to spare them from having to do this work.
// 然而，他们需要为他们想要与 hello_macro 一起使用的每种类型编写实现块； 我们想让他们不必做这项工作。

// Additionally, we can’t yet provide the hello_macro function with default implementation that will print the name of the type the trait is implemented on:
// 此外，我们还不能为 hello_macro 函数提供默认实现，该函数将打印实现特征的类型的名称：
//  Rust doesn’t have reflection capabilities, so it can’t look up the type’s name at runtime.
//  Rust 没有反射功能，所以它无法在运行时查找类型的名称。
// We need a macro to generate code at compile time.
// 我们需要一个宏来在编译时生成代码。

// The next step is to define the procedural macro. At the time of this writing, procedural macros need to be in their own crate.
// 下一步是定义程序宏。 在撰写本文时，程序宏需要放在自己的箱子中。
// Eventually, this restriction might be lifted.
// 最终，这个限制可能会被解除。
// The convention for structuring crates and macro crates is as follows: for a crate named foo, a custom derive procedural macro crate is called foo_derive.
// 构造 crate 和宏 crate 的约定如下：对于名为 foo 的 crate，自定义派生过程宏 crate 称为 foo_derive。
// Let’s start a new crate called hello_macro_derive inside our hello_macro project:
// 让我们在 hello_macro 项目中启动一个名为 hello_macro_derive 的新 crate：

// $ cargo new hello_macro_derive --lib

// Our two crates are tightly related, so we create the procedural macro crate within the directory of our hello_macro crate.
// 我们的两个 crate 紧密相关，因此我们在 hello_macro crate 的目录中创建过程宏 crate。
// If we change the trait definition in hello_macro, we’ll have to change the implementation of the procedural macro in hello_macro_derive as well.
// 如果我们更改 hello_macro 中的特征定义，我们也必须更改 hello_macro_derive 中过程宏的实现。
// The two crates will need to be published separately, and programmers using these crates will need to add both as dependencies and bring them both into scope.
// 这两个 crate 需要单独发布，使用这些 crate 的程序员需要将它们添加为依赖项并将它们都纳入范围。
// We could instead have the hello_macro crate use hello_macro_derive as a dependency and re-export the procedural macro code.
// 我们可以改为让 hello_macro crate 使用 hello_macro_derive 作为依赖项并重新导出过程宏代码。
// However, the way we’ve structured the project makes it possible for programmers to use hello_macro even if they don’t want the derive functionality.
// 但是，我们构建项目的方式使程序员可以使用 hello_macro，即使他们不想要派生功能。

// We need to declare the hello_macro_derive crate as a procedural macro crate.
// 我们需要将 hello_macro_derive crate 声明为过程宏 crate。
// We’ll also need functionality from the syn and quote crates, as you’ll see in a moment, so we need to add them as dependencies.
// 我们还需要 syn 和 quote 包中的功能，稍后您会看到，因此我们需要将它们添加为依赖项。
// Add the following to the Cargo.toml file for hello_macro_derive:
// 将以下内容添加到 hello_macro_derive 的 Cargo.toml 文件中：

// Filename: hello_macro_derive/Cargo.toml

[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"

// To start defining the procedural macro, place the code in Listing 19-31 into your src/lib.rs file for the hello_macro_derive crate.
// 要开始定义过程宏，请将清单 19-31 中的代码放入 hello_macro_derive crate 的 src/lib.rs 文件中。
// Note that this code won’t compile until we add a definition for the impl_hello_macro function.
// 请注意，在我们为 impl_hello_macro 函数添加定义之前，这段代码不会编译。

// Filename: hello_macro_derive/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// Listing 19-31: Code that most procedural macro crates will require in order to process Rust code
// 示例 19-31：大多数过程宏包需要处理 Rust 代码的代码

// Notice that we’ve split the code into the hello_macro_derive function, which is responsible for parsing the TokenStream, and the impl_hello_macro function,
// 请注意，我们将代码拆分为负责解析 TokenStream 的 hello_macro_derive 函数和 impl_hello_macro 函数，
//  which is responsible for transforming the syntax tree: this makes writing a procedural macro more convenient.
//  负责转换语法树：这使得编写过程宏更加方便。
// The code in the outer function (hello_macro_derive in this case) will be the same for almost every procedural macro crate you see or create.
// 外部函数中的代码（在本例中为 hello_macro_derive）对于您看到或创建的几乎每个过程宏包都是相同的。
// The code you specify in the body of the inner function (impl_hello_macro in this case) will be different depending on your procedural macro’s purpose.
// 您在内部函数主体中指定的代码（在本例中为 impl_hello_macro）将根据您的过程宏的用途而有所不同。

// We’ve introduced three new crates: proc_macro, syn, and quote. The proc_macro crate comes with Rust, so we didn’t need to add that to the dependencies in Cargo.toml.
// 我们引入了三个新的 crate：proc_macro、syn 和 quote。 proc_macro crate 随 Rust 一起提供，因此我们不需要将其添加到 Cargo.toml 中的依赖项中。
// The proc_macro crate is the compiler’s API that allows us to read and manipulate Rust code from our code.
// proc_macro crate 是编译器的 API，它允许我们从代码中读取和操作 Rust 代码。

// The syn crate parses Rust code from a string into a data structure that we can perform operations on.
// syn crate 将 Rust 代码从字符串解析为我们可以对其执行操作的数据结构。
// The quote crate turns syn data structures back into Rust code.
// quote crate 将 syn 数据结构转回 Rust 代码。
// These crates make it much simpler to parse any sort of Rust code we might want to handle: writing a full parser for Rust code is no simple task.
// 这些 crate 使得解析我们可能想要处理的任何类型的 Rust 代码变得更加简单：为 Rust 代码编写一个完整的解析器并不是一项简单的任务。

// The hello_macro_derive function will be called when a user of our library specifies #[derive(HelloMacro)] on a type.
// 当我们库的用户在类型上指定#[derive(HelloMacro)] 时，将调用 hello_macro_derive 函数。
// This is possible because we’ve annotated the hello_macro_derive function here with proc_macro_derive and specified the name HelloMacro, which matches our trait name;
// 这是可能的，因为我们在这里用 proc_macro_derive 注释了 hello_macro_derive 函数并指定了名称 HelloMacro，这与我们的特征名称相匹配；
// this is the convention most procedural macros follow.
// 这是大多数过程宏遵循的约定。

// The hello_macro_derive function first converts the input from a TokenStream to a data structure that we can then interpret and perform operations on.
// hello_macro_derive 函数首先将输入从 TokenStream 转换为数据结构，然后我们可以对其进行解释和执行操作。
// This is where syn comes into play.
// 这就是 syn 发挥作用的地方。
// The parse function in syn takes a TokenStream and returns a DeriveInput struct representing the parsed Rust code.
// syn 中的解析函数采用 TokenStream 并返回表示已解析 Rust 代码的 DeriveInput 结构。
// Listing 19-32 shows the relevant parts of the DeriveInput struct we get from parsing the struct Pancakes; string:
// 清单 19-32 显示了我们从解析结构体 Pancakes 得到的 DeriveInput 结构体的相关部分； 细绳：

DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}

// Listing 19-32: The DeriveInput instance we get when parsing the code that has the macro’s attribute in Listing 19-30
// 示例 19-32：我们在解析示例 19-30 中具有宏属性的代码时得到的 DeriveInput 实例

// The fields of this struct show that the Rust code we’ve parsed is a unit struct with the ident (identifier, meaning the name) of Pancakes.
// 这个结构体的字段表明我们解析的 Rust 代码是一个单元结构体，带有 Pancakes 的 ident（标识符，意思是名字）。
// There are more fields on this struct for describing all sorts of Rust code; check the syn documentation for DeriveInput for more information.
// 这个结构体上有更多字段用于描述各种 Rust 代码； 查看 DeriveInput 的 syn 文档以获取更多信息。

// Soon we’ll define the impl_hello_macro function, which is where we’ll build the new Rust code we want to include.
// 很快我们将定义 impl_hello_macro 函数，我们将在其中构建我们想要包含的新 Rust 代码。
// But before we do, note that the output for our derive macro is also a TokenStream.
// 但在我们这样做之前，请注意我们的派生宏的输出也是一个 TokenStream。
// The returned TokenStream is added to the code that our crate users write, so when they compile their crate, they’ll get the extra functionality that we provide in the modified TokenStream.
// 返回的 TokenStream 被添加到我们的 crate 用户编写的代码中，因此当他们编译他们的 crate 时，他们将获得我们在修改后的 TokenStream 中提供的额外功能。

// You might have noticed that we’re calling unwrap to cause the hello_macro_derive function to panic if the call to the syn::parse function fails here.
// 你可能已经注意到，如果在这里调用 syn::parse 函数失败，我们调用 unwrap 会导致 hello_macro_derive 函数崩溃。
// It’s necessary for our procedural macro to panic on errors because proc_macro_derive functions must return TokenStream rather than Result to conform to the procedural macro API.
// 我们的过程宏有必要对错误进行恐慌，因为 proc_macro_derive 函数必须返回 TokenStream 而不是 Result 以符合过程宏 API。
// We’ve simplified this example by using unwrap; in production code, you should provide more specific error messages about what went wrong by using panic! or expect.
// 我们通过使用 unwrap 简化了这个例子； 在生产代码中，您应该提供更具体的错误消息，说明使用 panic! 或 expect。

// Now that we have the code to turn the annotated Rust code from a TokenStream into a DeriveInput instance,
// 现在我们有了将带注释的 Rust 代码从 TokenStream 转换为 DeriveInput 实例的代码，
//  let’s generate the code that implements the HelloMacro trait on the annotated type, as shown in Listing 19-33.
// 让我们生成在注释类型上实现 HelloMacro 特性的代码，如清单 19-33 所示。

// Filename: hello_macro_derive/src/lib.rs

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

// Listing 19-33: Implementing the HelloMacro trait using the parsed Rust code
// 示例 19-33：使用解析后的 Rust 代码实现 HelloMacro 特性

// We get an Ident struct instance containing the name (identifier) of the annotated type using ast.ident.
// 我们得到一个 Ident 结构实例，其中包含使用 ast.ident 的注释类型的名称（标识符）。
// The struct in Listing 19-32 shows that when we run the impl_hello_macro function on the code in Listing 19-30, the ident we get will have the ident field with a value of "Pancakes".
// 清单 19-32 中的结构表明，当我们在清单 19-30 中的代码上运行 impl_hello_macro 函数时，我们获得的 ident 将具有值为“Pancakes”的 ident 字段。
// Thus, the name variable in Listing 19-33 will contain an Ident struct instance that, when printed, will be the string "Pancakes", the name of the struct in Listing 19-30.
// 因此，示例 19-33 中的 name 变量将包含一个 Ident 结构实例，打印时该实例将是字符串“Pancakes”，即示例 19-30 中结构的名称。

// The quote! macro lets us define the Rust code that we want to return.
// quote! 宏让我们定义我们想要返回的 Rust 代码。
// The compiler expects something different to the direct result of the quote! macro’s execution, so we need to convert it to a TokenStream.
// 编译器期望与 quote! 宏执行的直接结果不同，因此我们需要将其转换为 TokenStream。
// We do this by calling the into method, which consumes this intermediate representation and returns a value of the required TokenStream type.
// 我们通过调用 into 方法来执行此操作，该方法使用此中间表示并返回所需 TokenStream 类型的值。

// The quote! macro also provides some very cool templating mechanics: we can enter #name, and quote! will replace it with the value in the variable name.
// quote! 宏还提供了一些非常酷的模板机制：我们可以输入#name，然后 quote! 将用变量名中的值替换它。
// You can even do some repetition similar to the way regular macros work.
// 您甚至可以像常规宏的工作方式一样进行一些重复。
// Check out the quote crate’s docs for a thorough introduction.
// 查看 quote crate 的文档以获得全面的介绍。

// We want our procedural macro to generate an implementation of our HelloMacro trait for the type the user annotated, which we can get by using #name.
// 我们希望我们的过程宏为用户注释的类型生成 HelloMacro 特性的实现，我们可以使用 #name 获得它。
// The trait implementation has the one function hello_macro, whose body contains the functionality we want to provide: printing Hello, Macro! My name is and then the name of the annotated type.
// trait 实现有一个函数 hello_macro，它的主体包含我们想要提供的功能：打印 Hello，Macro！ 我的名字是，然后是注释类型的名称。

// The stringify! macro used here is built into Rust.
// stringify! 此处使用的宏内置于 Rust 中。
// It takes a Rust expression, such as 1 + 2, and at compile time turns the expression into a string literal, such as "1 + 2".
// 它采用 Rust 表达式，例如 1 + 2，并在编译时将表达式转换为字符串文字，例如“1 + 2”。
// This is different than format! or println!, macros which evaluate the expression and then turn the result into a String.
// 这与 format! 或 println! 不同，计算表达式然后将结果转换为字符串的宏。
// There is a possibility that the #name input might be an expression to print literally, so we use stringify!.
// #name 输入可能是一个按字面打印的表达式，所以我们使用 stringify!。
// Using stringify! also saves an allocation by converting #name to a string literal at compile time.
// 使用 string! 还通过在编译时将 #name 转换为字符串文字来节省分配。

// At this point, cargo build should complete successfully in both hello_macro and hello_macro_derive.
// 此时，cargo build 应该在 hello_macro 和 hello_macro_derive 中成功完成。
// Let’s hook up these crates to the code in Listing 19-30 to see the procedural macro in action!
// 让我们将这些箱子连接到清单 19-30 中的代码，以查看过程宏的运行情况！
// Create a new binary project in your projects directory using cargo new pancakes.
// 使用 cargo new pancakes 在你的项目目录中创建一个新的二进制项目。
// We need to add hello_macro and hello_macro_derive as dependencies in the pancakes crate’s Cargo.toml.
// 我们需要在 pancakes crate 的 Cargo.toml 中添加 hello_macro 和 hello_macro_derive 作为依赖项。
// If you’re publishing your versions of hello_macro and hello_macro_derive to crates.io, they would be regular dependencies;
// 如果你将你的 hello_macro 和 hello_macro_derive 版本发布到 crates.io，它们将是常规依赖项；
// if not, you can specify them as path dependencies as follows:
// 如果没有，您可以将它们指定为路径依赖，如下所示：

hello_macro = { path = "../hello_macro" }
hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }

// Put the code in Listing 19-30 into src/main.rs, and run cargo run: it should print Hello, Macro! My name is Pancakes!
// 将示例 19-30 中的代码放入 src/main.rs，然后运行 cargo run：它应该打印 Hello, Macro! 我叫煎饼！
// The implementation of the HelloMacro trait from the procedural macro was included without the pancakes crate needing to implement it;
// 程序宏中 HelloMacro trait 的实现被包括在内，而 pancakes crate 不需要实现它；
// the #[derive(HelloMacro)] added the trait implementation.
// #[derive(HelloMacro)] 添加了特征实现。

// Next, let’s explore how the other kinds of procedural macros differ from custom derive macros.
// 接下来，让我们探讨一下其他类型的过程宏与自定义派生宏有何不同。

// Attribute-like macros
// 类似属性的宏
// Attribute-like macros are similar to custom derive macros, but instead of generating code for the derive attribute, they allow you to create new attributes.
// 类属性宏类似于自定义派生宏，但它们不是为派生属性生成代码，而是允许您创建新属性。
// They’re also more flexible: derive only works for structs and enums; attributes can be applied to other items as well, such as functions.
// 它们也更灵活：派生仅适用于结构和枚举； 属性也可以应用于其他项目，例如函数。
// Here’s an example of using an attribute-like macro: say you have an attribute named route that annotates functions when using a web application framework:
// 这是一个使用类似属性的宏的示例：假设您有一个名为 route 的属性，它在使用 Web 应用程序框架时注释函数：

#[route(GET, "/")]
fn index() {}

// This #[route] attribute would be defined by the framework as a procedural macro.
// 这个#[route] 属性将由框架定义为过程宏。
// The signature of the macro definition function would look like this:
// 宏定义函数的签名如下所示：

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// Here, we have two parameters of type TokenStream. The first is for the contents of the attribute: the GET, "/" part.
// 在这里，我们有两个 TokenStream 类型的参数。 第一个是属性的内容：GET，“/”部分。
// The second is the body of the item the attribute is attached to: in this case, fn index() {} and the rest of the function’s body.
// 第二个是属性附加到的项目的主体：在这种情况下，fn index() {} 和函数主体的其余部分。

// Other than that, attribute-like macros work the same way as custom derive macros: you create a crate with the proc-macro crate type and implement a function that generates the code you want!
// 除此之外，类属性宏的工作方式与自定义派生宏的工作方式相同：您创建一个具有 proc-macro crate 类型的 crate 并实现一个生成您想要的代码的函数！

// Function-like macros
// 类似函数的宏
// Function-like macros define macros that look like function calls.
// Function-like macros 定义看起来像函数调用的宏。
// Similarly to macro_rules! macros, they’re more flexible than functions; for example, they can take an unknown number of arguments.
// 类似于 macro_rules! 宏，它们比函数更灵活； 例如，他们可以接受未知数量的参数。
// However, macro_rules! macros can be defined only using the match-like syntax we discussed in the section “Declarative Macros with macro_rules! for General Metaprogramming” earlier.
// 然而，macro_rules! 宏只能使用我们在“带有 macro_rules 的声明性宏！ 对于通用元编程”较早。
// Function-like macros take a TokenStream parameter and their definition manipulates that TokenStream using Rust code as the other two types of procedural macros do.
// 类似函数的宏采用 TokenStream 参数，它们的定义使用 Rust 代码操纵 TokenStream，就像其他两种类型的过程宏所做的那样。
// An example of a function-like macro is an sql! macro that might be called like so:
// 类函数宏的一个例子是 sql! 可以这样调用的宏：

let sql = sql!(SELECT * FROM posts WHERE id=1);

// This macro would parse the SQL statement inside it and check that it’s syntactically correct, which is much more complex processing than a macro_rules! macro can do.
// 这个宏会解析其中的 SQL 语句并检查它的语法是否正确，这比 macro_rules 复杂得多！ 宏可以做到。
// The sql! macro would be defined like this:
// sql! 宏定义如下：

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}

// This definition is similar to the custom derive macro’s signature: we receive the tokens that are inside the parentheses and return the code we wanted to generate.
// 这个定义类似于自定义派生宏的签名：我们接收括号内的标记并返回我们想要生成的代码。

// Summary
// 概括
// Whew! Now you have some Rust features in your toolbox that you likely won’t use often, but you’ll know they’re available in very particular circumstances.
// 哇！ 现在您的工具箱中有一些您可能不会经常使用的 Rust 功能，但您会知道它们在非常特殊的情况下可用。
// We’ve introduced several complex topics so that when you encounter them in error message suggestions or in other peoples’ code, you’ll be able to recognize these concepts and syntax.
// 我们引入了几个复杂的主题，这样当你在错误信息提示或其他人的代码中遇到它们时，你将能够识别这些概念和语法。
// Use this chapter as a reference to guide you to solutions.
// 使用本章作为参考来指导您找到解决方案。

// Next, we’ll put everything we’ve discussed throughout the book into practice and do one more project!
// 接下来，我们将把我们在本书中讨论的所有内容付诸实践，再做一个项目！
