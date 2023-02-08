fn main() {
    // Understanding Ownership
    // 理解所有权
    // Ownership is Rust’s most unique feature and has deep implications for the rest of the language.
    // 所有权是Rust最独特的特性，对语言的其他部分有深刻的影响。
    // It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works.
    // 它使Rust能够在不需要垃圾收集器的情况下保证内存安全，因此了解所有权的工作原理很重要。
    // In this chapter, we’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.
    // 在本章中，我们将讨论所有权以及几个相关的特性:借用、切片以及Rust如何在内存中布局数据。

    // What Is Ownership?
    // 什么是所有权?
    // Ownership is a set of rules that governs how a Rust program manages memory.
    // 所有权是一组规则，用于管理Rust程序如何管理内存。
    // All programs have to manage the way they use a computer’s memory while running.
    // 所有程序都必须管理运行时使用计算机内存的方式。
    // Some languages have garbage collection that regularly looks for no-longer used memory as the program runs;
    // 有些语言有垃圾回收机制，会在程序运行时定期寻找不再使用的内存;
    // in other languages, the programmer must explicitly allocate and free the memory.
    // 在其他语言中，程序员必须显式分配和释放内存。
    // Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks.
    // Rust使用第三种方法:内存通过所有权系统进行管理，该系统具有一组编译器检查的规则。
    // If any of the rules are violated, the program won’t compile.
    // 如果违反了任何规则，程序将无法编译
    // None of the features of ownership will slow down your program while it’s running.
    // 所有权的任何特性都不会降低程序运行的速度。

    // Because ownership is a new concept for many programmers, it does take some time to get used to.
    // 因为所有权对许多程序员来说是一个新概念，确实需要一些时间来适应。
    // The good news is that the more experienced you become with Rust and the rules of the ownership system,
    // 好消息是，你对Rust和所有权系统规则的经验越丰富，
    // the easier you’ll find it to naturally develop code that is safe and efficient.
    // 你就会更容易自然地开发出安全高效的代码
    // Keep at it!
    // 持之以恒!

    // When you understand ownership, you’ll have a solid foundation for understanding the features that make Rust unique.
    // 当你理解了所有权，你就有了一个坚实的基础来理解Rust的独特特性。
    // In this chapter, you’ll learn ownership by working through some examples that focus on a very common data structure: strings.
    // 本章通过几个例子来学习所有权，这些例子关注的是一种非常常见的数据结构:字符串。

    // The Stack and the Heap
    // 栈和堆
    // Many programming languages don’t require you to think about the stack and the heap very often.
    // 很多编程语言都不需要你经常考虑栈和堆。
    // But in a systems programming language like Rust,
    // 但是在像Rust这样的系统编程语言中，
    // whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions.
    // 一个值是在栈还是堆上影响了语言的行为以及为什么你必须做出某些决定。
    // Parts of ownership will be described in relation to the stack and the heap later in this chapter, so here is a brief explanation in preparation.
    // 所有权的部分内容将在本章后面的内容中与栈和堆相关，因此这里是一个简要的说明。

    // Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.
    // 栈和堆都是内存的一部分，代码可以在运行时使用，但它们的结构方式不同。
    // The stack stores values in the order it gets them and removes the values in the opposite order.
    // 栈以获取值的顺序存储值，并以相反的顺序删除值
    // This is referred to as last in, first out.
    // 这被称为后进先出
    // Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate,
    // 想象一下一堆盘子:当你添加更多盘子时，将它们放在这堆盘子的最上面，而当你需要一个盘子时，
    // you take one off the top.
    // 你从顶部拿掉一个。
    // Adding or removing plates from the middle or bottom wouldn’t work as well!
    // 不能从中间或底部添加或删除盘子!
    // Adding data is called pushing onto the stack, and removing data is called popping off the stack.
    // 添加数据被称为压入栈，删除数据被称为弹出栈。
    // All data stored on the stack must have a known, fixed size.
    // 存储在栈中的所有数据必须具有已知的、固定的大小。
    // Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
    // 编译时大小未知或大小可能会改变的数据必须存储在堆中。

    // The heap is less organized: when you put data on the heap, you request a certain amount of space.
    // 堆没有那么有组织:当你把数据放在堆上时，你需要一定的空间。
    // The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
    // 内存分配器在堆中找到一个足够大的空位置，标记为正在使用，并返回一个指针，即该位置的地址。
    // This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating).
    // 这个过程称为堆上分配，有时简写为分配(将值压入栈不视为分配)。
    // Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
    // 因为指向堆的指针是已知的、固定大小的，你可以将指针存储在栈上，但当你想要实际的数据时，你必须跟随指针。
    // Think of being seated at a restaurant. When you enter, you state the number of people in your group, and the staff finds an empty table that fits everyone and leads you there.
    // 想象一下坐在一家餐馆里。当你进去的时候，你说出你所在小组的人数，工作人员会找到一张空桌子，让每个人都坐下，然后带你进去。
    // If someone in your group comes late, they can ask where you’ve been seated to find you.
    // 如果你的团队中有人来晚了，他们可以询问你的座位。

    // Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
    // 压入栈比在堆上分配要快，因为分配器永远不需要寻找存储新数据的地方;该位置始终位于堆栈的顶部。
    // Comparatively, allocating space on the heap requires more work,
    // 相比之下，在堆上分配空间需要更多的工作，
    // because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
    // 因为分配器首先必须找到足够大的空间来保存数据，然后进行簿记，为下一次分配做准备。

    // Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there.
    // 访问堆中的数据比访问栈中的数据慢，因为你必须跟随指针到达栈中的数据。
    // Contemporary processors are faster if they jump around less in memory.
    // 当代处理器在内存中跳跃越少，速度越快。
    // Continuing the analogy, consider a server at a restaurant taking orders from many tables.
    // 继续类比，假设餐厅的服务员从多个桌子上点菜。
    // It’s most efficient to get all the orders at one table before moving on to the next table.
    // 在移动到下一个表之前获取一个表中的所有订单是最高效的。
    // Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process.
    // 从表A中获取订单，然后从表B中获取订单，然后再从表A中获取订单，然后再从表B中获取订单，这将是一个非常慢的过程。
    // By the same token, a processor can do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap).
    // 同理，如果处理器处理的数据离其他数据很近(比如栈上的数据)，而不是离其他数据很远(比如堆上的数据)，那么它可以做得更好。

    // When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack.
    // 当代码调用函数时，传递给函数的值(可能包括指向堆数据的指针)和函数的局部变量会被压入栈中。
    // When the function is over, those values get popped off the stack.
    // 当函数结束时，这些值从栈中弹出

    // Keeping track of what parts of code are using what data on the heap,
    // 跟踪哪些代码部分使用了堆中的哪些数据，
    // minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.
    // 最小化堆中重复数据的数量，清理堆中未使用的数据，以免耗尽空间，这些都是所有权解决的问题。
    // Once you understand ownership, you won’t need to think about the stack and the heap very often,
    // 一旦你理解了所有权，你就不需要经常考虑栈和堆了
    // but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.
    // 但是知道所有权的主要目的是管理堆数据可以帮助解释为什么它是这样工作的。

    // Ownership Rules
    // 所有权规则
    // First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:
    // 首先，让我们看一下所有权规则。在学习示例时，请牢记这些规则。

    // Each value in Rust has an owner.
    // Rust中的每个值都有一个所有者。
    // There can only be one owner at a time.
    // 同一时间只能有一个所有者
    // When the owner goes out of scope, the value will be dropped.
    // 当所有者离开作用域时，值将被删除。

    // Variable Scope
    // 变量作用域
    // Now that we’re past basic Rust syntax, we won’t include all the fn main() { code in examples,
    // 现在我们已经了解了Rust的基本语法，我们不会在示例中包含所有的fn main(){代码，
    // so if you’re following along, make sure to put the following examples inside a main function manually.
    // 作为所有权的第一个例子，我们来看看一些变量的作用域。
    // As a result, our examples will be a bit more concise, letting us focus on the actual details rather than boilerplate code.
    // 因此，如果你正在学习，请确保手动将以下示例放在main函数中。

    // As a first example of ownership, we’ll look at the scope of some variables.
    // 这样，我们的示例将更简洁，让我们专注于实际的细节，而不是样板代码。
    // A scope is the range within a program for which an item is valid. Take the following variable:
    // 作用域是程序中元素有效的范围以下面的变量为例:

    let s = "hello";
    // The variable s refers to a string literal, where the value of the string is hardcoded into the text of our program.
    // 变量s指向一个字符串字面量，这个字符串的值被硬编码到程序的文本中。
    // The variable is valid from the point at which it’s declared until the end of the current scope.
    // 变量从声明点开始有效，直到当前作用域结束。
    // Listing 4-1 shows a program with comments annotating where the variable s would be valid.
    // 代码清单4-1是一个带有注释的程序，注释了变量s在什么地方是有效的。

    //     {                      // s is not valid here, it’s not yet declared
    //         let s = "hello";   // s is valid from this point forward

    //         // do stuff with s
    //     }                      // this scope is now over, and s is no longer valid(这个作用域现在结束了，s不再有效)
    // Listing 4-1: A variable and the scope in which it is valid
    // 清单4-1:一个变量以及它的有效作用域

    // In other words, there are two important points in time here:
    // 换句话说，这里有两个重要的时间点:

    // When s comes into scope, it is valid.
    // 当s进入作用域时，它是有效的
    // It remains valid until it goes out of scope.
    // 它在超出作用域之前一直有效
    // At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages.
    // 在这一点上，作用域和变量何时有效的关系与其他编程语言类似。
    // Now we’ll build on top of this understanding by introducing the String type.
    // 现在我们将在此基础上引入String类型。

    // The String Type
    // 字符串类型
    // To illustrate the rules of ownership, we need a data type that is more complex than those we covered in the “Data Types” section of Chapter 3.
    // 为了说明所有权规则，我们需要一个比第3章数据类型更复杂的数据类型。
    // The types covered previously are all a known size, can be stored on the stack and popped off the stack when their scope is over,
    // 前面介绍的类型都是已知大小的，可以存储在栈中，当它们的作用域结束时弹出栈，
    // and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope.
    // 如果另一部分代码需要在不同的作用域中使用相同的值，可以快速地复制它，创建一个新的独立实例。
    // But we want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, and the String type is a great example.
    // 但是我们想看看存储在堆中的数据，并探索Rust如何知道何时清理该数据，String类型就是一个很好的例子。

    // We’ll concentrate on the parts of String that relate to ownership.
    // 我们将专注于String中与所有权相关的部分。
    // These aspects also apply to other complex data types, whether they are provided by the standard library or created by you.
    // 这些方面也适用于其他复杂的数据类型，无论它们是由标准库提供的还是由您创建的。
    // We’ll discuss String in more depth in Chapter 8.
    // 我们将在第8章深入讨论字符串。

    // We’ve already seen string literals, where a string value is hardcoded into our program.
    // 我们已经见过字符串字面量，在我们的程序中硬编码了字符串值。
    // String literals are convenient, but they aren’t suitable for every situation in which we may want to use text.
    // 字符串字面量很方便，但并不是适用于所有需要使用文本的情况。
    // One reason is that they’re immutable.
    // 一个原因是它们是不可变的
    // Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it?
    // 另一个问题是，在编写代码时，不是每个字符串的值都是已知的:例如，如果我们想获取用户输入并存储它，该怎么办?
    // For these situations, Rust has a second string type, String.
    // 对于这些情况，Rust有第二种字符串类型string。
    // This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
    // 这种类型管理分配在堆上的数据，因此能够存储大量在编译时未知的文本。
    // You can create a String from a string literal using the from function, like so:
    // 你可以使用from函数从一个字符串字面量创建一个字符串，像这样:
    let s = String::from("hello");

    // The double colon :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from.
    // 双冒号::操作符允许我们将这个from函数命名为String类型，而不是使用string_from之类的名称。
    // We’ll discuss this syntax more in the “Method Syntax” section of Chapter 5 and when we talk about namespacing with modules in “Paths for Referring to an Item in the Module Tree” in Chapter 7.
    // 我们会在第5章的“方法语法”一节和第7章的“模块树中引用项的路径”一节中讨论模块命名空间时进一步讨论这种语法。

    // This kind of string can be mutated:
    // 这种字符串是可以改变的:
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // So, what’s the difference here? Why can String be mutated but literals cannot? The difference is how these two types deal with memory.
    //那么，这里有什么区别呢?为什么字符串可以改变而字面量不能?不同之处在于这两种类型如何处理内存。

    // Memory and Allocation
    // 内存分配
    // In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.
    // 在字符串字面量的情况下，我们在编译时就知道内容，因此文本直接硬编码到最终的可执行文件中。
    // This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability.
    // 这就是字符串字面量快速高效的原因但是这些属性只来自于字符串字面量的不可变性。
    // Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.
    // 不幸的是，我们不能为每一段在编译时大小未知、并且在程序运行时大小可能会改变的文本，都给它分配一个内存blob。

    // With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
    // 使用String类型，为了支持可变的、可增长的文本，我们需要在堆上分配一定数量的内存，在编译时未知，以保存内容。这意味着:

    // The memory must be requested from the memory allocator at runtime.
    // 必须在运行时从内存分配器请求内存。
    // We need a way of returning this memory to the allocator when we’re done with our String.
    // 当我们使用完字符串时，我们需要一种方法将这些内存返回给分配器。

    // That first part is done by us: when we call String::from, its implementation requests the memory it needs.
    // 第一部分由我们完成:当我们调用String::from时，它的实现会请求所需的内存。
    // This is pretty much universal in programming languages.
    // 这在编程语言中非常普遍。

    // However, the second part is different.
    // 然而，第二部分有所不同。
    // In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it.
    // 在带有垃圾收集器(GC)的语言中，GC会跟踪并清理不再使用的内存，我们不需要考虑这一点。
    // In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and call code to explicitly free it, just as we did to request it.
    // 在大多数没有GC的语言中，我们有责任识别何时不再使用内存，并调用代码显式释放内存，就像我们请求它一样。
    // Doing this correctly has historically been a difficult programming problem.
    // 历史上，正确地执行此操作一直是一个编程难题。
    // If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable.
    // 如果我们忘记了，就会浪费内存如果我们太早这样做，我们会得到一个无效的变量。
    // If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.
    // 如果我们执行两次，这也是一个bug我们需要将一个分配对象和一个空闲对象配对。

    // Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.
    // Rust采用不同的路径:一旦拥有内存的变量超出作用域，内存将自动返回。
    // Here’s a version of our scope example from Listing 4-1 using a String instead of a string literal:
    // 下面是代码清单4-1中作用域示例的一个版本，使用字符串代替字符串字面量:

    // {
    //     let s = String::from("hello"); // s is valid from this point forward

    //     // do stuff with s
    // }                                  // this scope is now over, and s is no
    //                                    // longer valid(不再有效)

    // There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope.
    // 当s超出作用域时，我们可以将字符串所需的内存返回给分配器。
    // When a variable goes out of scope, Rust calls a special function for us.
    // 当变量超出作用域时，Rust为我们调用一个特殊的函数。
    // This function is called drop, and it’s where the author of String can put the code to return the memory.
    // 这个函数名为drop，字符串的作者可以把代码放在这里，以返回内存。
    // Rust calls drop automatically at the closing curly bracket.
    // Rust调用在右大括号中自动删除。

    // This pattern has a profound impact on the way Rust code is written.
    // 这种模式对Rust代码的编写方式有深刻的影响。
    // It may seem simple right now, but the behavior of code can be unexpected in more complicated situations when we want to have multiple variables use the data we’ve allocated on the heap.
    // 现在看起来很简单，但在更复杂的情况下，当我们想让多个变量使用我们在堆上分配的数据时，代码的行为可能会出乎意料。
    // Let’s explore some of those situations now.
    // 现在让我们来探索其中的一些情况。

    // Ways Variables and Data Interact: Move
    // 变量和数据交互的方式:Move
    // Multiple variables can interact with the same data in different ways in Rust. Let’s look at an example using an integer in Listing 4-2.
    // 在Rust中，多个变量可以以不同的方式与相同的数据交互。下面来看一个使用整数的例子，如代码清单4-2所示。

    let x = 5;
    let y = x;
    // Listing 4-2: Assigning the integer value of variable x to y
    // 代码清单4-2:将变量x的整数值赋值给y

    // We can probably guess what this is doing: “bind the value 5 to x; then make a copy of the value in x and bind it to y.”
    // 我们大概可以猜到这是在做什么：" bind the value 5 to x;然后复制x中的值并将其绑定到y。”
    // We now have two variables, x and y, and both equal 5.
    // 现在我们有两个变量，x和y，都等于5。
    // This is indeed what is happening, because integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.
    // 这就是正在发生的事情，因为整数是已知的、固定大小的简单值，这两个5被压入栈。

    // Now let’s look at the String version:
    let s1 = String::from("hello");
    let s2 = s1;

    // This looks very similar,
    // 看起来很像，
    // so we might assume that the way it works would be the same: that is, the second line would make a copy of the value in s1 and bind it to s2.
    // 所以我们可以假设它的工作方式是一样的:也就是说，第二行代码复制了s1中的值，并将其绑定到s2。
    // But this isn’t quite what happens.
    // 但实际情况并非如此

    // Take a look at Figure 4-1 to see what is happening to String under the covers.
    // 看一下图4-1，看看String背后发生了什么。
    // A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity.
    // 字符串由三部分组成，如左侧所示:指向存储字符串内容的内存的指针、长度和容量。
    // This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.
    // 这组数据存储在栈中，右边是堆中保存内容的内存。

    // s1
    // name      value        index  value
    // ptr       ---------->  0      h
    // len       5            1      e
    // capacity  5            2      l
    //                        3      l
    //                        4      o
    //
    // Figure 4-1: Representation in memory of a String holding the value "hello" bound to s1
    // 图4-1:在内存中表示的字符串包含绑定到s1的值"hello

    // The length is how much memory, in bytes, the contents of the String is currently using.
    // 长度是字符串的内容当前占用了多少内存，以字节为单位
    // The capacity is the total amount of memory, in bytes, that the String has received from the allocator.
    // 容量是字符串从分配器接收到的总内存大小，以字节为单位。
    // The difference between length and capacity matters, but not in this context, so for now, it’s fine to ignore the capacity.
    // length和capacity之间的差异很重要，但在这里无关紧要，所以现在可以忽略capacity。

    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack.
    // 当我们将s1赋值给s2时，字符串数据被复制，这意味着我们复制了栈上的指针、长度和容量。
    // We do not copy the data on the heap that the pointer refers to.
    // 我们不会复制指针指向的堆上的数据
    // In other words, the data representation in memory looks like Figure 4-2.
    // 换句话说，内存中的数据表示如图4-2所示。

    // s1
    // name      value
    // ptr       -----------
    // len       5         |        index  value
    // capacity  5         ------>  0      h
    //                     |        1      e
    // s2                  |        2      l
    // name      value     |        3      l
    // ptr       -----------        4      o
    // len       5
    // capacity  5
    //
    // Figure 4-2: Representation in memory of the variable s2 that has a copy of the pointer, length, and capacity of s1
    // 图4-2变量s2在内存中的表示，它包含了s1的指针、长度和容量的副本

    // The representation does not look like Figure 4-3, which is what memory would look like if Rust instead copied the heap data as well.
    // 表现形式不像图4-3所示，图4-3是Rust复制堆数据时内存的样子。
    // If Rust did this, the operation s2 = s1 could be very expensive in terms of runtime performance if the data on the heap were large.
    // 如果Rust这样做了，那么如果堆上的数据很大，那么s2 = s1操作在运行时性能方面可能非常昂贵。

    // s1
    // name      value        index  value
    // ptr       ---------->  0      h
    // len       5            1      e
    // capacity  5            2      l
    //                        3      l
    //                        4      o
    //
    // s2
    // name      value        index  value
    // ptr       ---------->  0      h
    // len       5            1      e
    // capacity  5            2      l
    //                        3      l
    //                        4      o
    //
    // Figure 4-3: Another possibility for what s2 = s1 might do if Rust copied the heap data as well
    // 图4-3:如果Rust也复制了堆数据，那么s2 = s1可能会执行的另一种操作

    // Earlier, we said that when a variable goes out of scope,
    // 前面我们说过，当变量超出作用域时，
    // Rust automatically calls the drop function and cleans up the heap memory for that variable.
    // Rust自动调用drop函数并为该变量清理堆内存。
    // But Figure 4-2 shows both data pointers pointing to the same location.
    // 但是图4-2显示了两个数据指针指向同一个位置。
    // This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory.
    // 这里有一个问题:当s2和s1超出作用域时，它们都将尝试释放相同的内存。
    // This is known as a double free error and is one of the memory safety bugs we mentioned previously.
    // 这被称为double free error，是我们之前提到的内存安全bug之一。
    // Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
    // 两次释放内存会导致内存损坏，从而可能导致安全漏洞。

    // To ensure memory safety, after the line let s2 = s1, Rust considers s1 as no longer valid.
    // 为了确保内存安全，在行let s2 = s1之后，Rust认为s1不再有效。
    // Therefore, Rust doesn’t need to free anything when s1 goes out of scope.
    // 因此，当s1超出作用域时，Rust不需要释放任何东西。
    // Check out what happens when you try to use s1 after s2 is created; it won’t work:
    // 检查在创建s2之后尝试使用s1时会发生什么;它不会工作:

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);

    // You’ll get an error like this because Rust prevents you from using the invalidated reference:
    // 你会得到类似这样的错误，因为Rust阻止你使用无效引用:

    // sleetsecrets@lolita ownership % cargo run
    //     Compiling ownership v0.1.0 (/Users/sleetsecrets/Desktop/learn-rust-with-me/ownership)
    // warning: unused variable: `s2`
    //     --> src/main.rs:373:9
    //     |
    // 373 |     let s2 = s1;
    //     |         ^^ help: if this is intentional, prefix it with an underscore: `_s2`
    //     |
    //     = note: `#[warn(unused_variables)]` on by default

    // error[E0382]: borrow of moved value: `s1`
    //     --> src/main.rs:375:28
    //     |
    // 372 |     let s1 = String::from("hello");
    //     |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    // 373 |     let s2 = s1;
    //     |              -- value moved here
    // 374 |
    // 375 |     println!("{}, world!", s1);
    //     |                            ^^ value borrowed here after move
    //     |
    //     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

    // For more information about this error, try `rustc --explain E0382`.
    // warning: `ownership` (bin "ownership") generated 1 warning
    // error: could not compile `ownership` due to previous error; 1 warning emitted
    // sleetsecrets@lolita ownership %

    // If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy.
    // 如果你在使用其他语言时听过浅复制和深复制这两个术语，那么只复制指针、长度和容量而不复制数据的概念听起来可能像做浅复制。
    // But because Rust also invalidates the first variable, instead of calling it a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2.
    // 但是因为Rust也会使第一个变量失效，所以它不是叫它浅复制，而是叫它移动。在这个例子中，我们可以说s1被移到了s2中。
    // So what actually happens is shown in Figure 4-4.
    // 所以实际发生的情况如图4-4所示。

    //&&& s1 (s1 is the gray area)
    //&&& name      value
    //&&& ptr       --------
    //&&& len       5      |        index  value
    //&&& capacity  5      ------>  0      h
    //                     |        1      e
    // s2                  |        2      l
    // name      value     |        3      l
    // ptr       -----------        4      o
    // len       5
    // capacity  5
    //
    // Figure 4-4: Representation in memory after s1 has been invalidated
    // 图4-4:s1失效后在内存中的表示

    // That solves our problem! With only s2 valid, when it goes out of scope, it alone will free the memory, and we’re done.
    // 解决了我们的问题!只有s2有效，当它超出作用域时，它会单独释放内存，我们就完成了。

    // In addition, there’s a design choice that’s implied by this: Rust will never automatically create “deep” copies of your data.
    // 此外，这暗示了一个设计选择:Rust永远不会自动创建数据的“深”副本。
    // Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.
    // 因此，就运行时性能而言，任何自动复制都可以被认为是开销较小的。

    // Ways Variables and Data Interact: Clone
    // 变量和数据交互的方式:克隆
    // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone.
    // 如果我们想深度复制字符串的堆数据，而不仅仅是栈数据，我们可以使用一个名为clone的通用方法。
    // We’ll discuss method syntax in Chapter 5, but because methods are a common feature in many programming languages, you’ve probably seen them before.
    // 我们将在第5章讨论方法语法，但由于方法是许多编程语言的共同特性，你可能见过它们。

    // Here’s an example of the clone method in action:
    // 这是一个clone方法的例子:
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // This works just fine and explicitly produces the behavior shown in Figure 4-3, where the heap data does get copied.
    // 这样做效果很好，会显式地产生如图4-3所示的行为，即复制堆数据。

    // When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive.
    // 当你看到clone的调用时，你就知道正在执行一些任意代码，并且这些代码可能是开销昂贵的。
    // It’s a visual indicator that something different is going on.
    // 这是一个视觉指示器，表明发生了一些不同的事情。

    // Stack-Only Data: Copy
    // 仅栈数据:复制
    // There’s another wrinkle we haven’t talked about yet.
    // 还有一个问题我们还没有讨论。
    // This code using integers – part of which was shown in Listing 4-2 – works and is valid:
    // 这段使用整数(部分代码如代码清单4-2所示)的代码是有效的:


    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.
    // 但这段代码似乎与我们刚刚学到的内容相矛盾:我们没有调用clone，但x仍然有效，并且没有被移动到y中。

    // The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
    // 原因是，像整数这样在编译时大小已知的类型完全存储在栈中，因此可以快速地复制实际值。
    // That means there’s no reason we would want to prevent x from being valid after we create the variable y.
    // 这意味着我们没有理由在创建变量y后阻止x的有效性。
    // In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying and we can leave it out.
    // 换句话说，这里的深复制和浅复制没有区别，所以调用clone不会做任何与通常的浅复制不同的事情，我们可以忽略它。

    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10).
    // Rust有一个名为Copy trait的特殊注释，我们可以将其放在存储在栈中的类型上，就像整数一样(我们将在第10章详细讨论traits)。
    // If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
    // 如果一个类型实现了复制特征（Copy trait），使用它的变量不会移动，而是简单地复制，使它们在赋值给另一个变量后仍然有效。

    // Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait.
    // 如果类型或其任何部分实现了Drop trait, Rust将不允许我们使用Copy注解类型。
    // If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.
    // 如果类型需要在值超出作用域时发生特殊情况，并且我们为该类型添加复制注释（Copy annotation），我们将得到编译时错误。
    // To learn about how to add the Copy annotation to your type to implement the trait, see “Derivable Traits” in Appendix C.
    // 要了解如何为你的类型添加复制注释（Copy annotation）以实现trait，请参阅附录C的“Derivable Traits”。

    // So what types implement the Copy trait? You can check the documentation for the given type to be sure,
    // 那么哪些类型实现了Copy trait?你可以查看指定类型的文档来确定，
    // but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy.
    // 但作为一般规则，任何简单的标量值组都可以实现复制，而任何需要分配或某种形式的资源都不能实现复制。
    // Here are some of the types that implement Copy:
    // 下面是一些实现了Copy的类型:

    // All the integer types, such as u32.
    // 所有的整数类型，例如u32
    // The Boolean type, bool, with values true and false.
    // 布尔类型bool，值为true和false。
    // All the floating point types, such as f64.
    // 所有浮点类型，例如f64
    // The character type, char.
    // 字符类型char
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    // Tuple，如果它们只包含也实现了复制的类型例如，(i32, i32)实现了Copy，而(i32, String)没有。

    // Ownership and Functions
    // 所有权和函数
    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
    // 向函数传递值的机制与给变量赋值的机制类似。
    // Passing a variable to a function will move or copy, just as assignment does.
    // 将变量传递给函数会移动或复制，就像赋值一样
    // Listing 4-3 has an example with some annotations showing where variables go into and out of scope.
    // 代码清单4-3是一个示例，其中有一些注释，显示了变量在什么地方进入和离开作用域。

    fn main() {
        let s = String::from("hello");  // s comes into scope (s进入作用域)

        takes_ownership(s);             // s's value moves into the function... (s的值移动到函数中…)
                                        // ... and so is no longer valid here (所以在这里不再有效)

        let x = 5;                      // x comes into scope (x进入作用域)

        makes_copy(x);                  // x would move into the function, (x会移动到函数中，)
                                        // but i32 is Copy, so it's okay to still
                                        // use x afterward (但 i32 是 Copy 的，所以在后面仍然可以使用 x)

    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.
      // 这里，x移出了作用域，然后是s。但是因为s的值被移动了，所以没有什么特别的事情发生。

    fn takes_ownership(some_string: String) { // some_string comes into scope (some_string进入作用域)
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.
      // 这里，some_string移出了作用域，`drop`被调用。占用的内存被释放。

    fn makes_copy(some_integer: i32) { // some_integer comes into scope (some_integer进入作用域)
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
      // 这里，some_integer移出了作用域。（`drop`被调用）。没有什么特别的事发生。

    // Listing 4-3: Functions with ownership and scope annotated
    // 代码清单4-3:标注了所有权和作用域的函数

    // If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error.
    // 如果我们试图在调用takes_ownership之后使用s, Rust将抛出编译时错误。
    // These static checks protect us from mistakes.
    // 这些静态检查防止我们出错
    // Try adding code to main that uses s and x to see where you can use them and where the ownership rules prevent you from doing so.
    // 尝试在main中添加使用s和x的代码，看看哪些地方可以使用它们，哪些所有权规则不允许这样做。

    // Return Values and Scope
    // 返回值和作用域
    // Returning values can also transfer ownership.
    // 返回值也可以转移所有权。
    // Listing 4-4 shows an example of a function that returns some value, with similar annotations as those in Listing 4-3.
    // 代码清单4-4展示了一个返回值的函数示例，其中使用了与代码清单4-3类似的注释。

    fn main() {
        let s1 = gives_ownership();         // gives_ownership moves its return
                                            // value into s1 （gives_ownership将其返回值移动到s1）

        let s2 = String::from("hello");     // s2 comes into scope (s2进入作用域)

        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3 (s2被移动到takes_and_gives_back中，后者也将其返回值移动到s3中)
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.
      // 在这里，s3移出了作用域并被删除。S2被移动了，所以什么都没发生。S1移出了作用域并被删除。

    fn gives_ownership() -> String {             // gives_ownership will move its
                                                 // return value into the function
                                                 // that calls it (gives_ownership将把它的返回值移动到调用它的函数中)

        let some_string = String::from("yours"); // some_string comes into scope (some_string进入作用域)

        some_string                              // some_string is returned and
                                                 // moves out to the calling
                                                 // function (返回some_string并移到调用函数)
    }

    // This function takes a String and returns one
    // 这个函数接受一个字符串并返回一个
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                          // scope (a_string进入作用域)

        a_string  // a_string is returned and moves out to the calling function (a_string被返回并移到调用函数)
    }

    // Listing 4-4: Transferring ownership of return values
    // 清单4-4:转移返回值的所有权

    // The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.
    // 变量的所有权每次都遵循相同的模式:将一个值赋给另一个变量会移动它。
    // When a variable that includes data on the heap goes out of scope,
    // 当包含堆中的数据的变量超出作用域时
    // the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
    // 除非数据的所有权已被移动到另一个变量，否则值将被drop清理

    // While this works, taking ownership and then returning ownership with every function is a bit tedious.
    // 虽然这样可以工作，但对每个函数获取所有权然后返回所有权是有点繁琐的。
    // What if we want to let a function use a value but not take ownership?
    // 如果我们想让函数使用一个值但不拥有所有权怎么办?
    // It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again,
    // 非常烦人的是，如果我们想再次使用它，我们传入的任何东西都需要传递回来，
    // in addition to any data resulting from the body of the function that we might want to return as well.
    // 除了函数体中可能需要返回的数据

    // Rust does let us return multiple values using a tuple, as shown in Listing 4-5.
    // Rust允许我们使用元组返回多个值，如清单4-5所示。

    fn main() {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String

        (s, length)
    }

    // Listing 4-5: Returning ownership of parameters
    // 代码清单4-5:返回参数的所有权

    // But this is too much ceremony and a lot of work for a concept that should be common.
    // 但是对于一个应该是通用的概念来说，这有太多的仪式和工作。
    // Luckily for us, Rust has a feature for using a value without transferring ownership, called references.
    // 幸运的是，Rust有一个特性可以在不转移所有权的情况下使用值，称为引用。

    // References and Borrowing
    // 引用和借用（（引用）借用）
    // The issue with the tuple code in Listing 4-5 is that we have to return the String to the calling function so we can still use the String after the call to calculate_length,
    // 代码清单4-5中的元组代码的问题在于，我们必须将字符串返回给调用函数，以便在调用calculate_length后仍然可以使用字符串，
    // because the String was moved into calculate_length.
    // 因为字符串被移到了calculate_length中
    // Instead, we can provide a reference to the String value.
    // 相反，我们可以提供一个字符串值的引用。
    // A reference is like a pointer in that it’s an address we can follow to access the data stored at that address;
    // 引用类似于指针，它是一个地址，我们可以跟随它访问存储在该地址的数据;
    // that data is owned by some other variable.
    // 该数据属于其他变量
    // Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
    // 与指针不同，引用在其生命周期内保证指向特定类型的有效值。

    // Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:
    // 下面是定义和使用calculate_length函数的方式，它的参数是对象的引用，而不是值的所有权:

    fn main() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    // First, notice that all the tuple code in the variable declaration and the function return value is gone.
    // 首先，注意变量声明中的所有元组代码和函数返回值都不见了。
    // Second, note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String.
    // 其次，请注意，我们将&s1传递给calculate_length，在它的定义中，我们取&String而不是String。
    // These ampersands represent references, and they allow you to refer to some value without taking ownership of it.
    // 这些&符号表示引用，它们允许你引用某个值而不拥有该值
    // Figure 4-5 depicts this concept.
    // 图4-5描述了这个概念。

    // s ref s1
    // s                   s1
    // name  value         name      value           index  value
    // ptr   ----------->  ptr       ------------->  0      h
    //                     len       5               1      e
    //                     capacity  5               2      l
    //                                               3      l
    //                                               4      0
    // Figure 4-5: A diagram of &String s pointing at String s1
    // 图4-5:&String s指向字符串s1的图表

    // Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.
    // 注意:与使用&进行引用相反的是解引用(dereferencing)，它是用解引用操作符*完成的。
    // We’ll see some uses of the dereference operator in Chapter 8 and discuss details of dereferencing in Chapter 15.
    // 第8章会介绍解引用运算符的一些用法，第15章会详细讨论解引用运算符。

    // Let’s take a closer look at the function call here:
    // 让我们仔细看看这里的函数调用:
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
    // &s1语法允许我们创建一个引用，指向s1的值，但不拥有它
    // Because it does not own it, the value it points to will not be dropped when the reference stops being used.
    // 因为它不拥有它，所以当这个引用停止使用时，它所指向的值不会被删除

    // Likewise, the signature of the function uses & to indicate that the type of the parameter s is a reference.
    // 同样，函数的签名使用&表示形参s是引用类型。
    // Let’s add some explanatory annotations:
    // 添加一些解释性注解:
    fn calculate_length(s: &String) -> usize { // s is a reference to a String (s是一个指向字符串的引用)
        s.len()
    } // Here, s goes out of scope. But because it does not have ownership of what
      // it refers to, it is not dropped.
      // 这里，s离开了作用域。但是因为它不拥有它所指的东西的所有权，所以它不会被删除。

    // The scope in which the variable s is valid is the same as any function parameter’s scope,
    // 变量s有效的作用域与任何函数参数的作用域相同，
    // but the value pointed to by the reference is not dropped when s stops being used because s doesn’t have ownership.
    // 但是当s停止使用时，引用指向的值不会被删除，因为s没有所有权
    // When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership.
    // 当函数的参数是引用而不是实际值时，我们不需要返回值来返回所有权，因为我们从未拥有过所有权。

    // We call the action of creating a reference borrowing.
    // 我们将创建一个引用的行为称为 借用（borrowing）
    // As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.
    // 和现实生活中一样，如果某人拥有某物，你可以向他借当你用完的时候，你必须把它还给我。你又不拥有它。

    // So what happens if we try to modify something we’re borrowing? Try the code in Listing 4-6. Spoiler alert: it doesn’t work!
    // 那么，如果我们试图修改正在借用的东西会发生什么?请尝试代码清单4-6中的代码。剧透:它不起作用!


    fn main() {
        let s = String::from("hello");

        change(&s);
    }

    fn change(some_string: &String) {
        some_string.push_str(", world");
    }
    // Listing 4-6: Attempting to modify a borrowed value
    // 代码清单4-6:试图修改借用的值
    // Here’s the error:
    // 错误代码如下:
    // $ cargo run
    //     Compiling ownership v0.1.0 (file:///projects/ownership)
    // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
    // --> src/main.rs:8:5
    // |
    // 7 | fn change(some_string: &String) {
    // |                        ------- help: consider changing this to be a mutable reference: `&mut String`
    // 8 |     some_string.push_str(", world");
    // |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

    // For more information about this error, try `rustc --explain E0596`.
    // error: could not compile `ownership` due to previous error

    // Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
    // 就像变量默认是不可变的一样，引用也是我们不能修改引用的对象。

    // Mutable References
    // 可变引用
    // We can fix the code from Listing 4-6 to allow us to modify a borrowed value with just a few small tweaks that use, instead, a mutable reference:
    // 我们可以修复清单4-6中的代码，只需要做一些小调整，就可以修改借用的值，使用可变引用:

    fn main() {
        let mut s = String::from("hello");

        change(&mut s);
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // First, we change s to be mut.
    // 首先，我们将s改为mut。
    // Then we create a mutable reference with &mut s where we call the change function, and update the function signature to accept a mutable reference with some_string: &mut String.
    // 然后我们使用&mut s创建一个可变引用，我们调用change函数，并更新函数签名，使其接受一个可变引用，使用some_string: &mut String。
    // This makes it very clear that the change function will mutate the value it borrows.
    // 这就非常清楚地表明，change 函数将改变它所借用的值。

    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
    // 可变引用有一个很大的限制:如果你有一个值的可变引用，那么这个值就不能有其他引用。
    // This code that attempts to create two mutable references to s will fail:
    // 这段代码试图创建两个对s的可变引用，将会失败:
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

    // Here’s the error:
    // 错误代码如下:
    // $ cargo run
    //     Compiling ownership v0.1.0 (file:///projects/ownership)
    // error[E0499]: cannot borrow `s` as mutable more than once at a time
    // --> src/main.rs:5:14
    // |
    // 4 |     let r1 = &mut s;
    // |              ------ first mutable borrow occurs here
    // 5 |     let r2 = &mut s;
    // |              ^^^^^^ second mutable borrow occurs here
    // 6 |
    // 7 |     println!("{}, {}", r1, r2);
    // |                        -- first borrow later used here

    // For more information about this error, try `rustc --explain E0499`.
    // error: could not compile `ownership` due to previous error

    // This error says that this code is invalid because we cannot borrow s as mutable more than once at a time.
    // 这个错误说明这段代码无效，因为我们不能多次借用可变的s
    // The first mutable borrow is in r1 and must last until it’s used in the println!,
    // 第一个可变借用在r1中，它必须持续到在println中使用!，
    // but between the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.
    // 但是在这个可变引用的创建和使用之间，我们尝试在r2中创建另一个可变引用，它借用了与r1相同的数据。

    // The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion.
    // 防止多个可变引用同时指向同一个数据的限制允许以一种非常可控的方式进行修改。
    // It’s something that new Rustaceans struggle with, because most languages let you mutate whenever you’d like.
    // 这是新Rustacean们难以适应，因为大多数语言都允许你随时修改。
    // The benefit of having this restriction is that Rust can prevent data races at compile time.
    // 有这个限制的好处是Rust可以在编译时防止数据竞争。
    // A data race is similar to a race condition and happens when these three behaviors occur:
    // 数据竞争类似于竞态条件，发生在以下三种情况下:

    // Two or more pointers access the same data at the same time.
    // 两个或多个指针同时访问相同的数据
    // At least one of the pointers is being used to write to the data.
    // 至少有一个指针用于写入数据
    // There’s no mechanism being used to synchronize access to the data.
    // 没有使用机制同步对数据的访问

    // Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime;
    // 数据竞争会导致未定义的行为，并且当你试图在运行时跟踪它们时，很难诊断和修复;
    // Rust prevents this problem by refusing to compile code with data races!
    // Rust通过拒绝编译带有数据竞争的代码来防止这种问题!

    // As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
    // 和往常一样，我们可以使用大括号来创建一个新的作用域，允许多个可变引用，只是不能同时引用:

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
      // r1在这里移出了作用域，所以我们可以创建一个新引用，不会有问题。

    let r2 = &mut s;

    // Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:
    // Rust对可变和不可变引用的组合实施了类似的规则。这段代码会导致一个错误:

    let mut s = String::from("hello");

    let r1 = &s; // no problem 没问题
    let r2 = &s; // no problem 没问题
    let r3 = &mut s; // BIG PROBLEM 大问题

    println!("{}, {}, and {}", r1, r2, r3);

    // Here’s the error:
    // 错误代码如下:
    // $ cargo run
    //     Compiling ownership v0.1.0 (file:///projects/ownership)
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // --> src/main.rs:6:14
    // |
    // 4 |     let r1 = &s; // no problem
    // |              -- immutable borrow occurs here
    // 5 |     let r2 = &s; // no problem
    // 6 |     let r3 = &mut s; // BIG PROBLEM
    // |              ^^^^^^ mutable borrow occurs here
    // 7 |
    // 8 |     println!("{}, {}, and {}", r1, r2, r3);
    // |                                -- immutable borrow later used here

    // For more information about this error, try `rustc --explain E0502`.
    // error: could not compile `ownership` due to previous error

    // Whew! We also cannot have a mutable reference while we have an immutable one to the same value.
    // 哇哦! 我们 也 不能在拥有不可变引用的同时拥有可变引用。

    // Users of an immutable reference don’t expect the value to suddenly change out from under them!
    // 不可变引用的用户不会期望其下面的值突然改变!
    // However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
    // 然而，多个不可变引用是允许的，因为读取数据的任何人都没有能力影响其他人读取数据。

    // Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
    // 注意，引用的作用域从它被引入的地方开始，一直持续到最后一次使用该引用的时候。
    // For instance, this code will compile because the last usage of the immutable references, the println!, occurs before the mutable reference is introduced:
    // 例如，这段代码可以编译，因为最后一次使用不可变引用println!，在引入可变引用之前发生:

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    // 变量r1和r2在此之后将不再使用

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created.
    // 不变引用r1和r2的作用域在println!它们最后一次被使用的地方，也就是可变引用r3被创建之前。
    // These scopes don’t overlap, so this code is allowed.
    // 这些作用域不重叠，所以这段代码是允许的
    // The ability of the compiler to tell that a reference is no longer being used at a point before the end of the scope is called Non-Lexical Lifetimes (NLL for short),
    // 编译器判断一个引用在作用域结束之前是否不再被使用的能力称为非词法生命周期(Non-Lexical lifetime，简称NLL)，
    // and you can read more about it in The Edition Guide.
    // 你可以在版本指南中阅读更多有关它的信息。

    // Even though borrowing errors may be frustrating at times,
    // 尽管借用错误有时令人沮丧，
    // remember that it’s the Rust compiler pointing out a potential bug early (at compile time rather than at runtime) and showing you exactly where the problem is.
    // 请记住，这是Rust编译器在早期(在编译时而不是运行时)指出潜在的错误，并准确地向您显示问题所在。
    // Then you don’t have to track down why your data isn’t what you thought it was.
    // 这样，你就不必追踪为什么数据和你想象的不一样了。

    // Dangling References
    // 悬空引用
    // In languages with pointers,
    // 在使用指针的语言中，
    // it’s easy to erroneously create a dangling pointer--a pointer that references a location in memory that may have been given to someone else--by freeing some memory while preserving a pointer to that memory.
    // 通过释放一些内存，同时保留指向该内存的指针，很容易错误地创建一个悬空指针——一个引用内存中某个位置的指针，该位置可能已经被赋予了其他人。
    // In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
    // 相比之下，在Rust中，编译器保证引用永远不会是悬空引用:如果你有对某些数据的引用，编译器将确保数据不会在对数据的引用超出范围之前超出范围。

    // Let’s try to create a dangling reference to see how Rust prevents them with a compile-time error:
    // 让我们尝试创建一个悬空引用，看看Rust是如何通过编译时错误防止它们的:

    fn main() {
        let reference_to_nothing = dangle();
    }

    fn dangle() -> &String {
        let s = String::from("hello");

        &s
    }

    // Here’s the error:
    // 错误代码如下:
    // $ cargo run
    //     Compiling ownership v0.1.0 (file:///projects/ownership)
    // error[E0106]: missing lifetime specifier
    // --> src/main.rs:5:16
    // |
    // 5 | fn dangle() -> &String {
    // |                ^ expected named lifetime parameter
    // |
    // = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // help: consider using the `'static` lifetime
    // |
    // 5 | fn dangle() -> &'static String {
    // |                ~~~~~~~~

    // For more information about this error, try `rustc --explain E0106`.
    // error: could not compile `ownership` due to previous error

    // This error message refers to a feature we haven’t covered yet: lifetimes.
    // 这个错误消息引用了一个我们还没有介绍的功能:生命周期。
    // We’ll discuss lifetimes in detail in Chapter 10. But, if you disregard the parts about lifetimes, the message does contain the key to why this code is a problem:
    // 我们将在第10章详细讨论生命周期但是，如果你忽略生命周期的部分，则消息中确实包含了为什么这段代码是一个问题的关键:
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    // 这个函数的返回类型包含一个借用值，但没有借用值

    // Let’s take a closer look at exactly what’s happening at each stage of our dangle code:
    // 让我们仔细看看dangle代码的每个阶段到底发生了什么:

    fn dangle() -> &String { // dangle returns a reference to a String (dangle返回一个指向字符串的引用)

        let s = String::from("hello"); // s is a new String (s是一个新字符串)

        &s // we return a reference to the String, s (我们返回一个指向字符串s的引用)
    } // Here, s goes out of scope, and is dropped. Its memory goes away.
      // Danger!
      // //这里，s超出了作用域，并被删除。它的记忆消失了。危险!

    // Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it.
    // 因为s是在dangle中创建的，当dangle的代码完成时，s将被释放。但我们试图返回它的引用。
    // That means this reference would be pointing to an invalid String.
    // 这意味着这个引用将指向一个无效的字符串
    // That’s no good! Rust won’t let us do this.
    // 这样不行!Rust不会让我们这么做的。

    // The solution here is to return the String directly:
    // 这里的解决方案是直接返回字符串:
    fn no_dangle() -> String {
        let s = String::from("hello");

        s
    }

    // This works without any problems. Ownership is moved out, and nothing is deallocated.
    // 没有任何问题所有权被移出，不释放任何东西。

    // The Rules of References
    // 引用规则
    // Let’s recap what we’ve discussed about references:
    // 回顾一下我们讨论过的引用:

    // At any given time, you can have either one mutable reference or any number of immutable references.
    // 在任何时候，你可以拥有一个可变引用，也可以拥有任意数量的不可变引用。
    // References must always be valid.
    // 引用必须始终有效

    // Next, we’ll look at a different kind of reference: slices.
    // 接下来，我们来看看另一种引用:slices

    // The Slice Type
    // 切片类型
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    // 切片允许你引用集合中元素的连续序列，而不是整个集合
    // A slice is a kind of reference, so it does not have ownership.
    // 切片是一种引用，所以它没有所有权

    // Here’s a small programming problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
    // 这是一个小的编程问题:编写一个函数，接收一个由空格分隔的单词组成的字符串，并返回它在该字符串中找到的第一个单词。
    // If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.
    // 如果函数在字符串中没有找到空格，那么整个字符串必须是一个单词，因此应该返回整个字符串

    // Let’s work through how we’d write the signature of this function without using slices, to understand the problem that slices will solve:
    // 让我们来看看在不使用slices的情况下如何编写这个函数的签名，以理解slices将解决的问题:
    fn first_word(s: &String) -> ?

    // The first_word function has a &String as a parameter. We don’t want ownership, so this is fine.
    // first_word函数有一个&String参数。我们不想要所有权，所以这没问题。
    // But what should we return? We don’t really have a way to talk about part of a string. However,
    // 但是我们应该返回什么呢?我们没有办法讨论字符串的一部分。然而,
    // we could return the index of the end of the word, indicated by a space. Let’s try that, as shown in Listing 4-7.
    // 我们可以返回单词末尾的索引，用空格表示让我们试一下，如清单4-7所示。

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    // Listing 4-7: The first_word function that returns a byte index value into the String parameter
    // 代码清单4-7:first_word函数返回一个字节索引值给String参数

    // Because we need to go through the String element by element and check whether a value is a space,
    // 因为我们需要逐个元素遍历字符串并检查值是否为空格，
    // we’ll convert our String to an array of bytes using the as_bytes method:
    // 使用as_bytes方法将字符串转换为字节数组:

    let bytes = s.as_bytes();

    // Next, we create an iterator over the array of bytes using the iter method:
    // 接下来，我们使用iter方法在字节数组上创建迭代器:

    for (i, &item) in bytes.iter().enumerate() {

    // We’ll discuss iterators in more detail in Chapter 13.
    // 第13章会详细讨论迭代器
    // For now, know that iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead.
    // 现在只需要知道，iter是一个返回集合中每个元素的方法，enumerate封装了iter的结果，并将每个元素作为元组的一部分返回。
    // The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element.
    // 从enumerate返回的元组的第一个元素是索引，第二个元素是对元素的引用。
    // This is a bit more convenient than calculating the index ourselves.
    // 这比我们自己计算索引更方便

    // Because the enumerate method returns a tuple, we can use patterns to destructure that tuple.
    // 因为enumerate方法返回一个元组，我们可以使用模式来解构这个元组。
    // We’ll be discussing patterns more in Chapter 6. In the for loop, we specify a pattern that has i for the index in the tuple and &item for the single byte in the tuple.
    // 我们将在第6章进一步讨论模式在for循环中，我们指定一个模式，i是元组的索引，&item是元组中的单个字节。
    // Because we get a reference to the element from .iter().enumerate(), we use & in the pattern.
    // 因为通过。iter().enumerate()获得了元素的引用，所以在模式中使用&。

    // Inside the for loop, we search for the byte that represents the space by using the byte literal syntax.
    // 在for循环中，我们使用字节字面量语法查找表示空格的字节。
    // If we find a space, we return the position. Otherwise, we return the length of the string by using s.len():
    // 如果我们找到一个空格，就返回位置。否则，使用s.len()返回字符串的长度:

        if item == b' ' {
            return i;
        }
    }

    s.len()

    // We now have a way to find out the index of the end of the first word in the string, but there’s a problem.
    // 现在我们可以找到字符串中第一个单词末尾的下标了，但有个问题
    // We’re returning a usize on its own, but it’s only a meaningful number in the context of the &String.
    // 返回的是usize对象本身，但它在&字符串中只是一个有意义的数字。
    // In other words, because it’s a separate value from the String, there’s no guarantee that it will still be valid in the future.
    // 换句话说，因为它是一个与字符串独立的值，所以无法保证它在未来仍然有效。
    // Consider the program in Listing 4-8 that uses the first_word function from Listing 4-7.
    // 考虑清单4-8中的程序，它使用了清单4-7中的first_word函数。

    fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s); // word will get the value 5 (word的值为5)

        s.clear(); // this empties the String, making it equal to "" (这会清空字符串，使其等于"")

        // word still has the value 5 here, but there's no more string that
        // word在这里的值仍然是5，但没有字符串了
        // we could meaningfully use the value 5 with. word is now totally invalid!
        // 我们可以有意义地使用5这个值。word现在完全无效!
    }
    // Listing 4-8: Storing the result from calling the first_word function and then changing the String contents
    // 清单4-8:存储调用first_word函数的结果，然后修改字符串的内容

    // This program compiles without any errors and would also do so if we used word after calling s.clear().
    // 这个程序不会编译错误，如果我们在调用s.clear()之后使用word，也会编译错误。
    // Because word isn’t connected to the state of s at all, word still contains the value 5.
    // 因为word根本没有连接到s的状态，所以word仍然包含值5
    // We could use that value 5 with the variable s to try to extract the first word out, but this would be a bug because the contents of s have changed since we saved 5 in word.
    // 我们可以将这个值5与变量s一起使用，尝试提取第一个单词，但这将是一个bug，因为我们将5保存在word中后，s的内容已经改变了。

    // Having to worry about the index in word getting out of sync with the data in s is tedious and error prone!
    // 担心word中的索引与s中的数据不同步既乏味又容易出错!
    // Managing these indices is even more brittle if we write a second_word function.
    // 如果我们编写second_word函数，管理这些索引将更加脆弱。

    // Its signature would have to look like this:
    // 它的签名应该是这样的:
    fn second_word(s: &String) -> (usize, usize) {

    // Now we’re tracking a starting and an ending index, and we have even more values that were calculated from data in a particular state but aren’t tied to that state at all.
    // 现在我们正在跟踪起始和结束索引，并且我们从特定状态的数据中计算出了更多的值，但这些值完全与该状态无关。
    // We have three unrelated variables floating around that need to be kept in sync.
    // 我们有三个不相关的变量需要保持同步

    // Luckily, Rust has a solution to this problem: string slices.
    // 幸运的是，Rust对这个问题有一个解决方案:字符串切片。

    // String Slices
    // 字符串切片
    // A string slice is a reference to part of a String, and it looks like this:
    // 字符串切片是对字符串一部分的引用，它看起来像这样:
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // Rather than a reference to the entire String, hello is a reference to a portion of the String, specified in the extra [0..5] bit.
    // hello不是指向整个字符串的引用，而是指向字符串的一部分，通过额外的[0..5]。
    // We create slices using a range within brackets by specifying [starting_index..ending_index],
    // 我们通过指定[starting_index. ending_index]，在括号内使用范围创建切片，
    // where starting_index is the first position in the slice and ending_index is one more than the last position in the slice.
    // 其中starting_index是切片中的第一个位置，ending_index比切片中的最后一个位置大1。
    // Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index.
    // 在内部，slice数据结构存储了切片的起始位置和长度，对应于ending_index减去starting_index。
    // So in the case of let world = &s[6..11];, world would be a slice that contains a pointer to the byte at index 6 of s with a length value of 5.
    // 所以在let world = &s[6..11];的情况下，world将是一个切片，其中包含一个指向s的索引6处的字节的指针，长度值为5。

    // Figure 4-6 shows this in a diagram.
    // 图4-6展示了这一点。

    // s
    // name      value                         index  value
    // ptr       --------------------------->  0      h
    // len       11                            1      e
    // capacity  11                            2      l
    //                                         3      l
    // world                                   4      o
    // name      value                         5
    // prt       --------------------------->  6      w
    // len       5                             7      o
    //                                         8      r
    //                                         9      l
    //                                         10     d
    // Figure 4-6: String slice referring to part of a String
    // 图4-6:字符串切片，指向字符串的一部分

    // With Rust’s .. range syntax, if you want to start at index zero, you can drop the value before the two periods. In other words, these are equal:
    // 使用Rust的..范围语法，如果想从索引0开始，可以删除两个句点之前的值。换句话说，这两个函数是相等的:

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];

    // By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:
    // 同样，如果切片包含字符串的最后一个字节，则可以删除末尾的数字。这意味着它们是相等的:

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    // You can also drop both values to take a slice of the entire string. So these are equal:
    // 你也可以删除这两个值来获取整个字符串的切片。这些等于:

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

    // Note: String slice range indices must occur at valid UTF-8 character boundaries.
    // 注意:字符串切片范围索引必须出现在有效的UTF-8字符边界。
    // If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.
    // 如果你试图在多字节字符的中间创建字符串切片，程序将退出并报错。
    // For the purposes of introducing string slices,
    // 为了介绍字符串切片，
    // we are assuming ASCII only in this section; a more thorough discussion of UTF-8 handling is in the “Storing UTF-8 Encoded Text with Strings” section of Chapter 8.
    // 我们只在本节中使用ASCII;关于UTF-8处理的更深入的讨论可以参考第8章的“用字符串存储UTF-8编码的文本”一节。

    // With all this information in mind, let’s rewrite first_word to return a slice.
    // 考虑到所有这些信息，让我们重写first_word以返回一个切片。
    // The type that signifies “string slice” is written as &str:
    // 表示" string slice "的类型写成&str:

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // We get the index for the end of the word in the same way as we did in Listing 4-7, by looking for the first occurrence of a space.
    // 使用与代码清单4-7相同的方法，通过查找第一个出现的空格来获取单词末尾的索引。
    // When we find a space, we return a string slice using the start of the string and the index of the space as the starting and ending indices.
    // 当我们找到一个空格时，我们返回一个字符串切片，使用字符串的开头和空格的索引作为起始和结束索引。

    // Now when we call first_word, we get back a single value that is tied to the underlying data.
    // 现在当我们调用first_word时，我们得到一个与底层数据绑定的值。
    // The value is made up of a reference to the starting point of the slice and the number of elements in the slice.
    // 这个值由切片的起始点和切片中元素的个数组成

    // Returning a slice would also work for a second_word function:
    // 对于second_word函数，也可以返回切片:

    fn second_word(s: &String) -> &str {

    // We now have a straightforward API that’s much harder to mess up,
    // 我们现在有了一个简单的API，更不容易搞砸了，
    // because the compiler will ensure the references into the String remain valid.
    // 因为编译器会确保对字符串的引用仍然有效。
    // Remember the bug in the program in Listing 4-8, when we got the index to the end of the first word but then cleared the string so our index was invalid?
    // 还记得代码清单4-8中的bug吗?我们得到了第一个单词末尾的索引，但随后清除了字符串，因此索引无效。
    // That code was logically incorrect but didn’t show any immediate errors.
    // 这段代码逻辑不正确，但没有立即显示任何错误。
    // The problems would show up later if we kept trying to use the first word index with an emptied string.
    // 如果我们继续尝试使用空字符串的第一个单词索引，问题将在稍后出现。
    // Slices make this bug impossible and let us know we have a problem with our code much sooner.
    // 切片让这个bug变得不可能，并且让我们更快地知道代码有问题。
    // Using the slice version of first_word will throw a compile-time error:
    // 使用切片版本的first_word将抛出编译时错误:

    fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear(); // error!

        println!("the first word is: {}", word);
    }

    // Here’s the compiler error:
    // 下面是编译错误:

    // $ cargo run
    //     Compiling ownership v0.1.0 (file:///projects/ownership)
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // --> src/main.rs:18:5
    //     |
    // 16 |     let word = first_word(&s);
    //     |                           -- immutable borrow occurs here
    // 17 |
    // 18 |     s.clear(); // error!
    //     |     ^^^^^^^^^ mutable borrow occurs here
    // 19 |
    // 20 |     println!("the first word is: {}", word);
    //     |                                       ---- immutable borrow later used here

    // For more information about this error, try `rustc --explain E0502`.
    // error: could not compile `ownership` due to previous error

    // Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference.
    // 回忆一下借用规则，如果我们有一个不可变的引用，我们就不能同时接受一个可变的引用。
    // Because clear needs to truncate the String, it needs to get a mutable reference.
    // 因为clear需要截断字符串，所以它需要一个可变的引用
    // The println! after the call to clear uses the reference in word, so the immutable reference must still be active at that point.
    // println!在调用clear之后，会在word中使用引用，因此不可变引用在此时必须仍然是活动的。
    // Rust disallows the mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails.
    // Rust不允许clear中的可变引用和word中的不可变引用同时存在，因此编译失败。
    // Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!
    // Rust不仅使我们的API更易于使用，而且还消除了编译时的一整类错误!

    // String Literals Are Slices
    // 字符串字面量是切片
    // Recall that we talked about string literals being stored inside the binary.
    // 我们讨论过存储在二进制文件中的字符串字面量。
    // Now that we know about slices, we can properly understand string literals:
    // 现在我们知道了切片，我们可以正确地理解字符串字面量:

    let s = "Hello, world!";

    // The type of s here is &str: it’s a slice pointing to that specific point of the binary.
    // 这里s的类型是&str:它是指向二进制文件特定位置的切片。
    // This is also why string literals are immutable; &str is an immutable reference.
    // 这也是字符串字面量不可变的原因;&str是一个不可变引用。

    // String Slices as Parameters
    // 作为参数的字符串切片
    // Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that’s its signature:
    // 知道可以对字面量和字符串值进行切片后，我们对first_word做了另一个改进，那就是它的签名:

    fn first_word(s: &String) -> &str {

    // A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the same function on both &String values and &str values.
    // 更有经验的Rustacean会编写如清单4-9所示的签名，因为它允许我们对&String值和&str值使用相同的函数。

    fn first_word(s: &str) -> &str {

    // Listing 4-9: Improving the first_word function by using a string slice for the type of the s parameter
    // 代码清单4-9:通过使用一个字符串切片作为s参数的类型来改进first_word函数

    // If we have a string slice, we can pass that directly.
    // 如果我们有一个字符串切片，我们可以直接传递它。
    // If we have a String, we can pass a slice of the String or a reference to the String.
    // 如果我们有一个字符串，我们可以传递字符串的切片或对该字符串的引用。
    // This flexibility takes advantage of deref coercions, a feature we will cover in the “Implicit Deref Coercions with Functions and Methods” section of Chapter 15.
    // 这种灵活性利用了强制转换，我们将在第15章的“函数和方法的隐式强制转换”一节中介绍这个特性。
    // Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:
    // 定义一个接受字符串切片而不是字符串引用的函数，使我们的API更通用、更有用，同时不会失去任何功能:

    fn main() {
        let my_string = String::from("hello world");

        // `first_word` works on slices of `String`s, whether partial or whole
        // `first_word`适用于`String`的切片，无论是部分的还是完整的
        let word = first_word(&my_string[0..6]);
        let word = first_word(&my_string[..]);
        // `first_word` also works on references to `String`s, which are equivalent
        // `first_word`也适用于对`String`的引用，两者是等价的
        // to whole slices of `String`s
        // 转换为`String`的整个切片
        let word = first_word(&my_string);

        let my_string_literal = "hello world";

        // `first_word` works on slices of string literals, whether partial or whole
        // `first_word`适用于字符串字面量的切片，无论是部分的还是完整的
        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);

        // Because string literals *are* string slices already,
        // 因为字符串字面量已经是字符串切片了，
        // this works too, without the slice syntax!
        // 即使没有切片语法，也可以正常工作!
        let word = first_word(my_string_literal);
    }

    // Other Slices
    // 其他切片
    // String slices, as you might imagine, are specific to strings. But there’s a more general slice type, too. Consider this array:
    // 如你所想，字符串切片是特定于字符串的。但还有一种更一般的切片类型。请看下面的数组:

    let a = [1, 2, 3, 4, 5];

    // Just as we might want to refer to a part of a string, we might want to refer to part of an array. We’d do so like this:
    // 正如我们可能想引用字符串的一部分一样，我们也可能想引用数组的一部分我们可以这样做:

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    // This slice has the type &[i32]. It works the same way as string slices do, by storing a reference to the first element and a length.
    // 这个切片的类型是&[i32]它的工作方式与字符串切片相同，都是存储对第一个元素的引用和长度。
    // You’ll use this kind of slice for all sorts of other collections.
    // 你可以将这种切片用于所有其他集合
    // We’ll discuss these collections in detail when we talk about vectors in Chapter 8.
    // 第8章讨论向量时会详细讨论这些集合。

    // Summary
    // 总结
    // The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time.
    // 所有权、借用和切片的概念确保了Rust程序在编译时的内存安全。
    // The Rust language gives you control over your memory usage in the same way as other systems programming languages,
    // Rust语言让你可以像其他系统编程语言一样控制内存使用，
    // but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.
    // 但是当数据的所有者超出范围时，数据的所有者会自动清理数据，这意味着你不需要编写和调试额外的代码来获得这种控制。

    // Ownership affects how lots of other parts of Rust work, so we’ll talk about these concepts further throughout the rest of the book.
    // 所有权会影响Rust其他部分的工作方式，因此我们将在本书的其余部分进一步讨论这些概念。
    // Let’s move on to Chapter 5 and look at grouping pieces of data together in a struct.
    // 让我们进入第5章，学习如何将数据分组到结构体中。
}
