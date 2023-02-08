// Object-Oriented Programming Features of Rust
// Rust 的面向对象编程特性
// Object-oriented programming (OOP) is a way of modeling programs.
// 面向对象编程 (OOP) 是一种建模程序的方式。
// Objects as a programmatic concept were introduced in the programming language Simula in the 1960s.
// 对象作为一个编程概念在 1960 年代引入了编程语言 Simula。
// Those objects influenced Alan Kay’s programming architecture in which objects pass messages to each other.
// 这些对象影响了 Alan Kay 的编程架构，在该架构中，对象相互传递消息。
// To describe this architecture, he coined the term object-oriented programming in 1967.
// 为了描述这种架构，他在 1967 年创造了面向对象编程这个术语。
// Many competing definitions describe what OOP is, and by some of these definitions Rust is object-oriented, but by others it is not.
// 许多相互竞争的定义描述了 OOP 是什么，根据其中一些定义，Rust 是面向对象的，但其他定义则不是。
// In this chapter, we’ll explore certain characteristics that are commonly considered object-oriented and how those characteristics translate to idiomatic Rust.
// 在本章中，我们将探讨通常被认为是面向对象的某些特征，以及这些特征如何转化为惯用的 Rust。
// We’ll then show you how to implement an object-oriented design pattern in Rust and discuss the trade-offs of doing so versus implementing a solution using some of Rust’s strengths instead.
// 然后，我们将向您展示如何在 Rust 中实现面向对象的设计模式，并讨论这样做与使用 Rust 的一些优势实现解决方案之间的权衡。


// Characteristics of Object-Oriented Languages
// 面向对象语言的特点
// There is no consensus in the programming community about what features a language must have to be considered object-oriented.
// 对于语言必须具有哪些特性才能被认为是面向对象的，编程社区没有达成共识。
// Rust is influenced by many programming paradigms, including OOP; for example, we explored the features that came from functional programming in Chapter 13.
// Rust 受到许多编程范式的影响，包括 OOP； 例如，我们在第 13 章探讨了函数式编程的特性。
// Arguably, OOP languages share certain common characteristics, namely objects, encapsulation, and inheritance.
// 可以说，OOP 语言具有某些共同特征，即对象、封装和继承。
// Let’s look at what each of those characteristics means and whether Rust supports it.
// 让我们看看每个特征的含义以及 Rust 是否支持它。

// Objects Contain Data and Behavior
// 对象包含数据和行为
// The book Design Patterns: Elements of Reusable Object-Oriented Software by Erich Gamma, Richard Helm, Ralph Johnson,
// 和 John Vlissides（Addison-Wesley Professional，1994 年），通俗地称为“四人帮”一书，是面向对象设计模式的目录。
//  and John Vlissides (Addison-Wesley Professional, 1994), colloquially referred to as The Gang of Four book, is a catalog of object-oriented design patterns.
// 它以这种方式定义 OOP：
// It defines OOP this way:
// 面向对象的程序是由对象组成的。

// Object-oriented programs are made up of objects.
// 一个对象封装了数据和操作该数据的过程。
// An object packages both data and the procedures that operate on that data.
// 这些过程通常称为方法或操作。
// The procedures are typically called methods or operations.
// Erich Gamma、Richard Helm、Ralph Johnson 的书设计模式：可重用面向对象软件的元素，

// Using this definition, Rust is object-oriented: structs and enums have data, and impl blocks provide methods on structs and enums.
// 使用这个定义，Rust 是面向对象的：结构和枚举有数据，impl 块提供结构和枚举的方法。
// Even though structs and enums with methods aren’t called objects, they provide the same functionality, according to the Gang of Four’s definition of objects.
// 尽管带有方法的结构和枚举不称为对象，但根据四人帮对对象的定义，它们提供相同的功能。

// Encapsulation that Hides Implementation Details
// 隐藏实现细节的封装
// Another aspect commonly associated with OOP is the idea of encapsulation, which means that the implementation details of an object aren’t accessible to code using that object.
// 通常与 OOP 相关的另一个方面是封装的概念，这意味着使用该对象的代码无法访问对象的实现细节。
// Therefore, the only way to interact with an object is through its public API;
// 因此，与对象交互的唯一方法是通过其公共 API；
// code using the object shouldn’t be able to reach into the object’s internals and change data or behavior directly.
// 使用该对象的代码不应该能够进入对象的内部并直接更改数据或行为。
// This enables the programmer to change and refactor an object’s internals without needing to change the code that uses the object.
// 这使程序员能够更改和重构对象的内部结构，而无需更改使用该对象的代码。

// We discussed how to control encapsulation in Chapter 7:
// 我们在第 7 章讨论了如何控制封装：
//  we can use the pub keyword to decide which modules, types, functions, and methods in our code should be public, and by default everything else is private.
//  我们可以使用 pub 关键字来决定我们代码中的哪些模块、类型、函数和方法应该是公共的，默认情况下其他一切都是私有的。
// For example, we can define a struct AveragedCollection that has a field containing a vector of i32 values.
// 例如，我们可以定义一个结构体 AveragedCollection，它有一个包含 i32 值向量的字段。
// The struct can also have a field that contains the average of the values in the vector, meaning the average doesn’t have to be computed on demand whenever anyone needs it.
// 该结构还可以有一个包含向量中值的平均值的字段，这意味着平均值不必在任何人需要时按需计算。
// In other words, AveragedCollection will cache the calculated average for us.
// 也就是说，AveragedCollection 会为我们缓存计算出的平均值。
// Listing 17-1 has the definition of the AveragedCollection struct:
// 清单 17-1 包含 AveragedCollection 结构的定义：

// Filename: src/lib.rs

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// Listing 17-1: An AveragedCollection struct that maintains a list of integers and the average of the items in the collection
// 示例 17-1：一个 AveragedCollection 结构，它维护一个整数列表和集合中各项的平均值

// The struct is marked pub so that other code can use it, but the fields within the struct remain private.
// 该结构被标记为 pub 以便其他代码可以使用它，但结构中的字段保持私有。
// This is important in this case because we want to ensure that whenever a value is added or removed from the list,
// 这在这种情况下很重要，因为我们要确保无论何时从列表中添加或删除值，
//  the average is also updated. We do this by implementing add, remove, and average methods on the struct, as shown in Listing 17-2:
//  平均值也被更新。 我们通过在结构上实现 add、remove 和 average 方法来做到这一点，如清单 17-2 所示：

// Filename: src/lib.rs

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Listing 17-2: Implementations of the public methods add, remove, and average on AveragedCollection
// 示例 17-2：AveragedCollection 上公共方法 add、remove 和 average 的实现

// The public methods add, remove, and average are the only ways to access or modify data in an instance of AveragedCollection.
// 公共方法 add、remove 和 average 是访问或修改 AveragedCollection 实例中数据的唯一方法。
// When an item is added to list using the add method or removed using the remove method,
// 当使用 add 方法将项目添加到列表或使用 remove 方法删除时，
//  the implementations of each call the private update_average method that handles updating the average field as well.
//  each 的实现调用私有的 update_average 方法来处理平均字段的更新。

// We leave the list and average fields private so there is no way for external code to add or remove items to or from the list field directly;
// 我们将列表和平均字段保留为私有，因此外部代码无法直接向列表字段添加或从列表字段中删除项目；
// otherwise, the average field might become out of sync when the list changes.
// 否则，当列表更改时，平均字段可能会变得不同步。
// The average method returns the value in the average field, allowing external code to read the average but not modify it.
// average 方法返回平均值字段中的值，允许外部代码读取平均值但不能修改它。

// Because we’ve encapsulated the implementation details of the struct AveragedCollection, we can easily change aspects, such as the data structure, in the future.
// 因为我们封装了 struct AveragedCollection 的实现细节，以后我们可以很容易地更改数据结构等方面。
// For instance, we could use a HashSet<i32> instead of a Vec<i32> for the list field.
// 例如，我们可以为列表字段使用 HashSet<i32> 而不是 Vec<i32>。
// As long as the signatures of the add, remove, and average public methods stay the same, code using AveragedCollection wouldn’t need to change.
// 只要 add、remove 和 average 公共方法的签名保持不变，使用 AveragedCollection 的代码就不需要更改。
// If we made list public instead, this wouldn’t necessarily be the case:
// 如果我们改为公开列表，情况就不一定是这样了：
//  HashSet<i32> and Vec<i32> have different methods for adding and removing items, so the external code would likely have to change if it were modifying list directly.
//  HashSet<i32> 和 Vec<i32> 有不同的添加和删除项目的方法，因此如果直接修改列表，外部代码可能必须更改。

// If encapsulation is a required aspect for a language to be considered object-oriented, then Rust meets that requirement.
// 如果封装是一种语言被认为是面向对象的必需方面，那么 Rust 满足该要求。
// The option to use pub or not for different parts of code enables encapsulation of implementation details.
// 对代码的不同部分使用或不使用 pub 的选项启用了实现细节的封装。

// Inheritance as a Type System and as Code Sharing
// 作为类型系统和代码共享的继承
// Inheritance is a mechanism whereby an object can inherit elements from another object’s definition,
// 继承是一种机制，通过该机制，一个对象可以从另一个对象的定义中继承元素，
//  thus gaining the parent object’s data and behavior without you having to define them again.
//  从而获得父对象的数据和行为，而无需再次定义它们。

// If a language must have inheritance to be an object-oriented language, then Rust is not one.
// 如果一门语言必须有继承才能成为面向对象的语言，那么 Rust 不是。
// There is no way to define a struct that inherits the parent struct’s fields and method implementations without using a macro.
// 如果不使用宏，则无法定义继承父结构的字段和方法实现的结构。

// However, if you’re used to having inheritance in your programming toolbox, you can use other solutions in Rust, depending on your reason for reaching for inheritance in the first place.
// 但是，如果您习惯于在编程工具箱中使用继承，则可以使用 Rust 中的其他解决方案，具体取决于您首先达到继承的原因。

// You would choose inheritance for two main reasons.
// 你会选择继承有两个主要原因。
// One is for reuse of code: you can implement particular behavior for one type, and inheritance enables you to reuse that implementation for a different type.
// 一种是为了重用代码：您可以为一种类型实现特定的行为，而继承使您能够为不同的类型重用该实现。
// You can do this in a limited way in Rust code using default trait method implementations,
// 您可以使用默认特征方法实现在 Rust 代码中以有限的方式执行此操作，
//  which you saw in Listing 10-14 when we added a default implementation of the summarize method on the Summary trait.
//  当我们在 Summary 特征上添加 summarize 方法的默认实现时，您会在示例 10-14 中看到。
// Any type implementing the Summary trait would have the summarize method available on it without any further code.
// 任何实现了 Summary 特性的类型都可以使用 summarize 方法，而无需任何进一步的代码。
// This is similar to a parent class having an implementation of a method and an inheriting child class also having the implementation of the method.
// 这类似于具有方法实现的父类和也具有该方法实现的继承子类。
// We can also override the default implementation of the summarize method when we implement the Summary trait,
// 我们也可以在实现 Summary trait 时覆盖 summarize 方法的默认实现，
//  which is similar to a child class overriding the implementation of a method inherited from a parent class.
// 这类似于子类覆盖从父类继承的方法的实现。

// The other reason to use inheritance relates to the type system: to enable a child type to be used in the same places as the parent type.
// 使用继承的另一个原因与类型系统有关：使子类型能够在与父类型相同的地方使用。
// This is also called polymorphism, which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.
// 这也称为多态性，这意味着您可以在运行时将多个对象相互替换，如果它们具有某些共同的特征。

// Polymorphism
// 多态性
// To many people, polymorphism is synonymous with inheritance.
// 对很多人来说，多态就是继承的同义词。
// But it’s actually a more general concept that refers to code that can work with data of multiple types.
// 但它实际上是一个更笼统的概念，指的是可以处理多种类型数据的代码。
// For inheritance, those types are generally subclasses.
// 对于继承来说，那些类型一般都是子类。

// Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide.
// Rust 使用泛型来抽象不同的可能类型和特征边界，以对这些类型必须提供的内容施加约束。
// This is sometimes called bounded parametric polymorphism.
// 这有时称为有界参数多态性。

// Inheritance has recently fallen out of favor as a programming design solution in many programming languages because it’s often at risk of sharing more code than necessary.
// 继承作为一种编程设计解决方案在许多编程语言中最近已经失宠，因为它经常面临共享不必要的代码的风险。
// Subclasses shouldn’t always share all characteristics of their parent class but will do so with inheritance.
// 子类不应该总是共享其父类的所有特征，但会通过继承来实现。
// This can make a program’s design less flexible.
// 这会降低程序设计的灵活性。
// It also introduces the possibility of calling methods on subclasses that don’t make sense or that cause errors because the methods don’t apply to the subclass.
// 它还引入了在子类上调用无意义或导致错误的方法的可能性，因为这些方法不适用于子类。
// In addition, some languages will only allow single inheritance (meaning a subclass can only inherit from one class), further restricting the flexibility of a program’s design.
// 另外，有些语言会只允许单继承（即一个子类只能继承一个类），进一步限制了程序设计的灵活性。

// For these reasons, Rust takes the different approach of using trait objects instead of inheritance.
// 出于这些原因，Rust 采用不同的方法使用特征对象而不是继承。
// Let’s look at how trait objects enable polymorphism in Rust.
// 让我们看看 trait 对象如何在 Rust 中启用多态性。


// Using Trait Objects That Allow for Values of Different Types
// 使用允许不同类型值的特征对象
// In Chapter 8, we mentioned that one limitation of vectors is that they can store elements of only one type.
// 在第 8 章中，我们提到向量的一个限制是它们只能存储一种类型的元素。
// We created a workaround in Listing 8-9 where we defined a SpreadsheetCell enum that had variants to hold integers, floats, and text.
// 我们在清单 8-9 中创建了一个解决方法，我们在其中定义了一个 SpreadsheetCell 枚举，它具有用于保存整数、浮点数和文本的变体。
// This meant we could store different types of data in each cell and still have a vector that represented a row of cells.
// 这意味着我们可以在每个单元格中存储不同类型的数据，并且仍然有一个表示一行单元格的向量。
// This is a perfectly good solution when our interchangeable items are a fixed set of types that we know when our code is compiled.
// 当我们的可互换项是我们在编译代码时知道的一组固定类型时，这是一个非常好的解决方案。

// However, sometimes we want our library user to be able to extend the set of types that are valid in a particular situation.
// 但是，有时我们希望我们的库用户能够扩展在特定情况下有效的类型集。
// To show how we might achieve this, we’ll create an example graphical user interface (GUI) tool that iterates through a list of items,
// 为了展示我们如何实现这一点，我们将创建一个示例图形用户界面 (GUI) 工具，它遍历项目列表，
//  calling a draw method on each one to draw it to the screen—a common technique for GUI tools.
//  在每个对象上调用一个绘制方法以将其绘制到屏幕上——这是 GUI 工具的一种常用技术。
// We’ll create a library crate called gui that contains the structure of a GUI library.
// 我们将创建一个名为 gui 的库 crate，其中包含 GUI 库的结构。
// This crate might include some types for people to use, such as Button or TextField.
// 这个 crate 可能包含一些供人们使用的类型，例如 Button 或 TextField。
// In addition, gui users will want to create their own types that can be drawn: for instance, one programmer might add an Image and another might add a SelectBox.
// 此外，gui 用户会希望创建自己的可以绘制的类型：例如，一个程序员可能会添加一个图像，另一个可能会添加一个选择框。

// We won’t implement a fully fledged GUI library for this example but will show how the pieces would fit together.
// 我们不会为这个例子实现一个完全成熟的 GUI 库，但会展示这些部分如何组合在一起。
// At the time of writing the library, we can’t know and define all the types other programmers might want to create.
// 在编写库时，我们无法知道和定义其他程序员可能想要创建的所有类型。
// But we do know that gui needs to keep track of many values of different types, and it needs to call a draw method on each of these differently typed values.
// 但我们确实知道 gui 需要跟踪许多不同类型的值，并且它需要对这些不同类型的值中的每一个调用 draw 方法。
// It doesn’t need to know exactly what will happen when we call the draw method, just that the value will have that method available for us to call.
// 它不需要确切地知道当我们调用 draw 方法时会发生什么，只是该值将具有可供我们调用的方法。

// To do this in a language with inheritance, we might define a class named Component that has a method named draw on it.
// 要在具有继承的语言中执行此操作，我们可以定义一个名为 Component 的类，该类具有一个名为 draw 的方法。
// The other classes, such as Button, Image, and SelectBox, would inherit from Component and thus inherit the draw method.
// 其他的类，比如Button、Image、SelectBox，都继承自Component，从而继承了draw方法。
// They could each override the draw method to define their custom behavior, but the framework could treat all of the types as if they were Component instances and call draw on them.
// 它们都可以覆盖 draw 方法来定义它们的自定义行为，但是框架可以将所有类型视为 Component 实例并在它们上调用 draw。
// But because Rust doesn’t have inheritance, we need another way to structure the gui library to allow users to extend it with new types.
// 但因为 Rust 没有继承，我们需要另一种方式来构建 gui 库，以允许用户使用新类型扩展它。

// Defining a Trait for Common Behavior
// 为常见行为定义特征
// To implement the behavior we want gui to have, we’ll define a trait named Draw that will have one method named draw.
// 为了实现我们希望 gui 具有的行为，我们将定义一个名为 Draw 的特征，该特征将有一个名为 draw 的方法。
// Then we can define a vector that takes a trait object.
// 然后我们可以定义一个带有特征对象的向量。
// A trait object points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime.
// 一个 trait 对象既指向一个实现我们指定 trait 的类型的实例，也指向一个用于在运行时查找该类型的 trait 方法的表。
// We create a trait object by specifying some sort of pointer, such as a & reference or a Box<T> smart pointer, then the dyn keyword, and then specifying the relevant trait.
// 我们通过指定某种指针来创建特征对象，例如 & 引用或 Box<T> 智能指针，然后是 dyn 关键字，然后指定相关特征。
//  (We’ll talk about the reason trait objects must use a pointer in Chapter 19 in the section “Dynamically Sized Types and the Sized Trait.”)
//（我们将在第 19 章的“动态大小类型和大小特征”部分讨论特征对象必须使用指针的原因。）
//  We can use trait objects in place of a generic or concrete type.
// 我们可以使用 trait 对象来代替泛型或具体类型。
// Wherever we use a trait object, Rust’s type system will ensure at compile time that any value used in that context will implement the trait object’s trait.
// 无论我们在哪里使用特征对象，Rust 的类型系统都会在编译时确保在该上下文中使用的任何值都将实现特征对象的特征。
// Consequently, we don’t need to know all the possible types at compile time.
// 因此，我们不需要在编译时知道所有可能的类型。

// We’ve mentioned that, in Rust, we refrain from calling structs and enums “objects” to distinguish them from other languages’ objects.
// 我们已经提到，在 Rust 中，我们避免把结构和枚举称为“对象”以将它们与其他语言的对象区分开来。
// In a struct or enum, the data in the struct fields and the behavior in impl blocks are separated,
// 在结构或枚举中，结构字段中的数据和 impl 块中的行为是分开的，
// Listing 17-3 shows how to define a trait named Draw with one method named draw:
// 而在其他语言中，数据和行为组合成一个概念通常被标记为对象。
//  whereas in other languages, the data and behavior combined into one concept is often labeled an object.
//  然而，特征对象更像其他语言中的对象，因为它们结合了数据和行为。
// However, trait objects are more like objects in other languages in the sense that they combine data and behavior.
// 但 trait 对象与传统对象的不同之处在于我们不能向 trait 对象添加数据。
// But trait objects differ from traditional objects in that we can’t add data to a trait object.
// Trait 对象不像其他语言中的对象那样普遍有用：它们的特定目的是允许对常见行为进行抽象。
// Trait objects aren’t as generally useful as objects in other languages: their specific purpose is to allow abstraction across common behavior.
// 清单 17-3 展示了如何使用一个名为 draw 的方法定义一个名为 Draw 的特征：

// Filename: src/lib.rs

pub trait Draw {
    fn draw(&self);
}

// Listing 17-3: Definition of the Draw trait
// 示例 17-3：Draw 特性的定义

// This syntax should look familiar from our discussions on how to define traits in Chapter 10.
// 从我们在第 10 章中关于如何定义特征的讨论来看，这种语法应该看起来很熟悉。
// Next comes some new syntax: Listing 17-4 defines a struct named Screen that holds a vector named components.
// 接下来是一些新语法：清单 17-4 定义了一个名为 Screen 的结构，它包含一个名为 components 的向量。
// This vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.
// 这个vector是Box<dyn Draw>类型，是一个trait对象； 它是 Box 中实现 Draw 特性的任何类型的替代品。

// Filename: src/lib.rs

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// Listing 17-4: Definition of the Screen struct with a components field holding a vector of trait objects that implement the Draw trait
// 示例 17-4：Screen 结构的定义，其组件字段包含实现 Draw 特征的特征对象向量

// On the Screen struct, we’ll define a method named run that will call the draw method on each of its components, as shown in Listing 17-5:
// 在 Screen 结构上，我们将定义一个名为 run 的方法，它将在其每个组件上调用 draw 方法，如清单 17-5 所示：

// Filename: src/lib.rs

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Listing 17-5: A run method on Screen that calls the draw method on each component
// 示例 17-5：Screen 上调用每个组件上的 draw 方法的 run 方法

// This works differently from defining a struct that uses a generic type parameter with trait bounds.
// 这与定义使用具有特征边界的泛型类型参数的结构不同。
// A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
// 泛型类型参数一次只能替换为一个具体类型，而特征对象允许多个具体类型在运行时填充特征对象。
// For example, we could have defined the Screen struct using a generic type and a trait bound as in Listing 17-6:
// 例如，我们可以使用泛型类型和特征绑定定义 Screen 结构，如示例 17-6 所示：

// Filename: src/lib.rs

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Listing 17-6: An alternate implementation of the Screen struct and its run method using generics and trait bounds
// 示例 17-6：Screen 结构的替代实现及其使用泛型和特征边界的 run 方法

// This restricts us to a Screen instance that has a list of components all of type Button or all of type TextField.
// 这将我们限制为一个 Screen 实例，该实例具有所有类型为 Button 或所有类型为 TextField 的组件列表。
// If you’ll only ever have homogeneous collections, using generics and trait bounds is preferable because the definitions will be monomorphized at compile time to use the concrete types.
// 如果你只拥有同类集合，那么使用泛型和特征边界是更可取的，因为定义将在编译时单态化以使用具体类型。

// On the other hand, with the method using trait objects, one Screen instance can hold a Vec<T> that contains a Box<Button> as well as a Box<TextField>.
// 另一方面，通过使用 trait 对象的方法，一个 Screen 实例可以包含一个包含 Box<Button> 和 Box<TextField> 的 Vec<T>。
// Let’s look at how this works, and then we’ll talk about the runtime performance implications.
// 让我们看看它是如何工作的，然后我们将讨论运行时性能影响。

// Implementing the Trait
// 实现特征
// Now we’ll add some types that implement the Draw trait.
// 现在我们将添加一些实现 Draw 特性的类型。
// We’ll provide the Button type.
// 我们将提供 Button 类型。
// Again, actually implementing a GUI library is beyond the scope of this book, so the draw method won’t have any useful implementation in its body.
// 同样，实际实现 GUI 库超出了本书的范围，因此 draw 方法在其主体中不会有任何有用的实现。
// To imagine what the implementation might look like, a Button struct might have fields for width, height, and label, as shown in Listing 17-7:
// 想象一下实现的样子，一个 Button 结构可能有宽度、高度和标签字段，如清单 17-7 所示：

// Filename: src/lib.rs

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

// Listing 17-7: A Button struct that implements the Draw trait
// 示例 17-7：实现 Draw 特性的 Button 结构

// The width, height, and label fields on Button will differ from the fields on other components;
// Button 上的宽度、高度和标签字段将与其他组件上的字段不同；
// for example, a TextField type might have those same fields plus a placeholder field.
// 例如，TextField 类型可能具有相同的字段和占位符字段。
// Each of the types we want to draw on the screen will implement the Draw trait but will use different code in the draw method to define how to draw that particular type,
// 我们想要在屏幕上绘制的每种类型都将实现 Draw 特性，但会在 draw 方法中使用不同的代码来定义如何绘制特定类型，
//  as Button has here (without the actual GUI code, as mentioned).
//  就像这里的 Button 一样（没有提到的实际 GUI 代码）。
// The Button type, for instance, might have an additional impl block containing methods related to what happens when a user clicks the button.
// 例如，Button 类型可能有一个额外的 impl 块，其中包含与用户单击按钮时发生的事情相关的方法。
// These kinds of methods won’t apply to types like TextField.
// 这些方法不适用于像 TextField 这样的类型。

// If someone using our library decides to implement a SelectBox struct that has width, height, and options fields,
// 如果使用我们库的人决定实现一个具有宽度、高度和选项字段的 SelectBox 结构，
//  they implement the Draw trait on the SelectBox type as well, as shown in Listing 17-8:
//  它们也在 SelectBox 类型上实现了 Draw 特性，如清单 17-8 所示：

// Filename: src/main.rs

use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

// Listing 17-8: Another crate using gui and implementing the Draw trait on a SelectBox struct
// 示例 17-8：另一个使用 gui 并在 SelectBox 结构上实现 Draw 特性的 crate

// Our library’s user can now write their main function to create a Screen instance.
// 我们库的用户现在可以编写他们的主要函数来创建一个 Screen 实例。
// To the Screen instance, they can add a SelectBox and a Button by putting each in a Box<T> to become a trait object.
// 对于 Screen 实例，他们可以添加一个 SelectBox 和一个 Button，方法是将每个都放在 Box<T> 中以成为特征对象。
// They can then call the run method on the Screen instance, which will call draw on each of the components.
// 然后他们可以在 Screen 实例上调用 run 方法，这将在每个组件上调用 draw。
// Listing 17-9 shows this implementation:
// 清单 17-9 显示了这个实现：

// Filename: src/main.rs

use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// Listing 17-9: Using trait objects to store values of different types that implement the same trait
// 示例 17-9：使用特征对象存储实现相同特征的不同类型的值

// When we wrote the library, we didn’t know that someone might add the SelectBox type,
// 我们写库的时候并不知道可能有人会添加SelectBox类型，
//  but our Screen implementation was able to operate on the new type and draw it because SelectBox implements the Draw trait, which means it implements the draw method.
//  但我们的 Screen 实现能够对新类型进行操作并绘制它，因为 SelectBox 实现了 Draw 特性，这意味着它实现了 draw 方法。

// This concept—of being concerned only with the messages a value responds to rather than the value’s concrete type—is similar to the concept of duck typing in dynamically typed languages:
// 这个概念——只关注值响应的消息而不是值的具体类型——类似于动态类型语言中鸭子类型的概念：
//  if it walks like a duck and quacks like a duck, then it must be a duck!
//  如果它走路像鸭子，叫起来像鸭子，那么它一定是鸭子！
// In the implementation of run on Screen in Listing 17-5, run doesn’t need to know what the concrete type of each component is.
// 在示例 17-5 的 run on Screen 实现中，run 不需要知道每个组件的具体类型是什么。
// It doesn’t check whether a component is an instance of a Button or a SelectBox, it just calls the draw method on the component.
// 它不检查组件是 Button 还是 SelectBox 的实例，它只是调用组件上的 draw 方法。
// By specifying Box<dyn Draw> as the type of the values in the components vector, we’ve defined Screen to need values that we can call the draw method on.
// 通过将 Box<dyn Draw> 指定为组件向量中值的类型，我们定义了 Screen 以需要我们可以调用 draw 方法的值。

// The advantage of using trait objects and Rust’s type system to write code similar to code using duck typing is that
// 使用 trait 对象和 Rust 的类型系统来编写类似于使用 duck typing 的代码的优点是
//  we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn’t implement a method but we call it anyway.
//  我们永远不必在运行时检查一个值是否实现了特定的方法，或者担心如果一个值没有实现某个方法但我们仍然调用它会出错。
// Rust won’t compile our code if the values don’t implement the traits that the trait objects need.
// 如果值没有实现特征对象需要的特征，Rust 将不会编译我们的代码。

// For example, Listing 17-10 shows what happens if we try to create a Screen with a String as a component:
// 例如，清单 17-10 展示了如果我们尝试创建一个以字符串作为组件的屏幕会发生什么：

// Filename: src/main.rs

use gui::Screen;

fn main() {
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
}

// Listing 17-10: Attempting to use a type that doesn’t implement the trait object’s trait
// 示例 17-10：尝试使用未实现 trait 对象的 trait 的类型

// We’ll get this error because String doesn’t implement the Draw trait:
// 我们会得到这个错误，因为 String 没有实现 Draw 特性：

// $ cargo run
//    Compiling gui v0.1.0 (file:///projects/gui)
// error[E0277]: the trait bound `String: Draw` is not satisfied
//  --> src/main.rs:5:26
//   |
// 5 |         components: vec![Box::new(String::from("Hi"))],
//   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
//   |
//   = help: the trait `Draw` is implemented for `Button`
//   = note: required for the cast to the object type `dyn Draw`
//
// For more information about this error, try `rustc --explain E0277`.
// error: could not compile `gui` due to previous error

// This error lets us know that either we’re passing something to Screen we didn’t mean to pass and so should pass a different type or we should implement Draw on String so that Screen is able to call draw on it.
// 这个错误让我们知道要么我们将一些东西传递给 Screen 但我们并不打算传递它，因此应该传递一个不同的类型，或者我们应该在 String 上实现 Draw 以便 Screen 能够在其上调用 draw。

// Trait Objects Perform Dynamic Dispatch
// 特征对象执行动态调度
// Recall in the “Performance of Code Using Generics” section in Chapter 10 our discussion on the monomorphization process performed by the compiler when we use trait bounds on generics:
// 回想一下第 10 章的“使用泛型的代码性能”部分，我们讨论了当我们在泛型上使用特征边界时编译器执行的单态化过程：
//  the compiler generates nongeneric implementations of functions and methods for each concrete type that we use in place of a generic type parameter.
//  编译器为我们用来代替泛型类型参数的每个具体类型生成函数和方法的非泛型实现。
// The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.
// 单态化产生的代码正在进行静态分派，即编译器知道您在编译时调用的方法。
// This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re calling.
// 这与动态分派相反，动态分派是指编译器无法在编译时判断您正在调用哪个方法。
// In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
// 在动态调度情况下，编译器发出代码，在运行时将确定调用哪个方法。

// When we use trait objects, Rust must use dynamic dispatch.
// 当我们使用 trait 对象时，Rust 必须使用动态调度。
// The compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call.
// 编译器不知道可能与使用 trait 对象的代码一起使用的所有类型，因此它不知道要调用在哪个类型上实现的哪个方法。
// Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call.
// 相反，在运行时，Rust 使用特征对象内部的指针来知道调用哪个方法。
// This lookup incurs a runtime cost that doesn’t occur with static dispatch.
// 此查找会产生运行时成本，而静态分派不会发生这种情况。
// Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.
// 动态分派还阻止编译器选择内联方法的代码，这反过来又阻止了一些优化。
// However, we did get extra flexibility in the code that we wrote in Listing 17-5 and were able to support in Listing 17-9, so it’s a trade-off to consider.
// 但是，我们在示例 17-5 中编写的代码中确实获得了额外的灵活性，并且能够在示例 17-9 中提供支持，因此需要考虑权衡。


// Implementing an Object-Oriented Design Pattern
// The state pattern is an object-oriented design pattern.
// The crux of the pattern is that we define a set of states a value can have internally.
// The states are represented by a set of state objects, and the value’s behavior changes based on its state.
// We’re going to work through an example of a blog post struct that has a field to hold its state, which will be a state object from the set "draft", "review", or "published".

// The state objects share functionality: in Rust, of course, we use structs and traits rather than objects and inheritance.
// Each state object is responsible for its own behavior and for governing when it should change into another state.
// The value that holds a state object knows nothing about the different behavior of the states or when to transition between states.

// The advantage of using the state pattern is that, when the business requirements of the program change,
//  we won’t need to change the code of the value holding the state or the code that uses the value.
// We’ll only need to update the code inside one of the state objects to change its rules or perhaps add more state objects.

// First, we’re going to implement the state pattern in a more traditional object-oriented way, then we’ll use an approach that’s a bit more natural in Rust.
// Let’s dig in to incrementally implementing a blog post workflow using the state pattern.

// The final functionality will look like this:

// 1.A blog post starts as an empty draft.
// 2.When the draft is done, a review of the post is requested.
// 3.When the post is approved, it gets published.
// 4.Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

// Any other changes attempted on a post should have no effect.
// For example, if we try to approve a draft blog post before we’ve requested a review, the post should remain an unpublished draft.

// Listing 17-11 shows this workflow in code form: this is an example usage of the API we’ll implement in a library crate named blog.
// This won’t compile yet because we haven’t implemented the blog crate.

// Filename: src/main.rs

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

// Listing 17-11: Code that demonstrates the desired behavior we want our blog crate to have

// We want to allow the user to create a new draft blog post with Post::new.
// We want to allow text to be added to the blog post.
// If we try to get the post’s content immediately, before approval, we shouldn’t get any text because the post is still a draft.
// We’ve added assert_eq! in the code for demonstration purposes.
// An excellent unit test for this would be to assert that a draft blog post returns an empty string from the content method, but we’re not going to write tests for this example.

// Next, we want to enable a request for a review of the post, and we want content to return an empty string while waiting for the review.
// When the post receives approval, it should get published, meaning the text of the post will be returned when content is called.

// Notice that the only type we’re interacting with from the crate is the Post type.
// This type will use the state pattern and will hold a value that will be one of three state objects representing the various states a post can be in—draft,
//  waiting for review, or published.
// Changing from one state to another will be managed internally within the Post type.
// The states change in response to the methods called by our library’s users on the Post instance, but they don’t have to manage the state changes directly.
// Also, users can’t make a mistake with the states, like publishing a post before it’s reviewed.

// Defining Post and Creating a New Instance in the Draft State
// Let’s get started on the implementation of the library!
// We know we need a public Post struct that holds some content,
//  so we’ll start with the definition of the struct and an associated public new function to create an instance of Post,
//  as shown in Listing 17-12.
// We’ll also make a private State trait that will define the behavior that all state objects for a Post must have.

// Then Post will hold a trait object of Box<dyn State> inside an Option<T> in a private field named state to hold the state object.
// You’ll see why the Option<T> is necessary in a bit.

// Filename: src/lib.rs

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}

// Listing 17-12: Definition of a Post struct and a new function that creates a new Post instance, a State trait, and a Draft struct

// The State trait defines the behavior shared by different post states.
// The state objects are Draft, PendingReview, and Published, and they will all implement the State trait.
// For now, the trait doesn’t have any methods, and we’ll start by defining just the Draft state because that is the state we want a post to start in.

// When we create a new Post, we set its state field to a Some value that holds a Box.
// This Box points to a new instance of the Draft struct.
// This ensures whenever we create a new instance of Post, it will start out as a draft.
// Because the state field of Post is private, there is no way to create a Post in any other state!
// In the Post::new function, we set the content field to a new, empty String.

// Storing the Text of the Post Content
// We saw in Listing 17-11 that we want to be able to call a method named add_text and pass it a &str that is then added as the text content of the blog post.
// We implement this as a method, rather than exposing the content field as pub, so that later we can implement a method that will control how the content field’s data is read.
// The add_text method is pretty straightforward, so let’s add the implementation in Listing 17-13 to the impl Post block:

// Filename: src/lib.rs

impl Post {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

// Listing 17-13: Implementing the add_text method to add text to a post’s content

// The add_text method takes a mutable reference to self, because we’re changing the Post instance that we’re calling add_text on.
// We then call push_str on the String in content and pass the text argument to add to the saved content.
// This behavior doesn’t depend on the state the post is in, so it’s not part of the state pattern.
// The add_text method doesn’t interact with the state field at all, but it is part of the behavior we want to support.

// Ensuring the Content of a Draft Post Is Empty
// Even after we’ve called add_text and added some content to our post,
//  we still want the content method to return an empty string slice because the post is still in the draft state,
//  as shown on line 7 of Listing 17-11.
// For now, let’s implement the content method with the simplest thing that will fulfill this requirement:
//  always returning an empty string slice. We’ll change this later once we implement the ability to change a post’s state so it can be published.
// So far, posts can only be in the draft state, so the post content should always be empty.
// Listing 17-14 shows this placeholder implementation:

// Filename: src/lib.rs

impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        ""
    }
}

// Listing 17-14: Adding a placeholder implementation for the content method on Post that always returns an empty string slice

// With this added content method, everything in Listing 17-11 up to line 7 works as intended.

// Requesting a Review of the Post Changes Its State
// Next, we need to add functionality to request a review of a post, which should change its state from Draft to PendingReview.
// Listing 17-15 shows this code:

// Filename: src/lib.rs

impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// Listing 17-15: Implementing request_review methods on Post and the State trait

// We give Post a public method named request_review that will take a mutable reference to self.
// Then we call an internal request_review method on the current state of Post, and this second request_review method consumes the current state and returns a new state.

// We add the request_review method to the State trait; all types that implement the trait will now need to implement the request_review method.
// Note that rather than having self, &self, or &mut self as the first parameter of the method, we have self: Box<Self>.
// This syntax means the method is only valid when called on a Box holding the type.
// This syntax takes ownership of Box<Self>, invalidating the old state so the state value of the Post can transform into a new state.

// To consume the old state, the request_review method needs to take ownership of the state value.
// This is where the Option in the state field of Post comes in: we call the take method to take the Some value out of the state field and leave a None in its place,
//  because Rust doesn’t let us have unpopulated fields in structs.
// This lets us move the state value out of Post rather than borrowing it.
// Then we’ll set the post’s state value to the result of this operation.

// We need to set state to None temporarily rather than setting it directly with code like self.state = self.state.request_review(); to get ownership of the state value.
// This ensures Post can’t use the old state value after we’ve transformed it into a new state.

// The request_review method on Draft returns a new, boxed instance of a new PendingReview struct, which represents the state when a post is waiting for a review.
// The PendingReview struct also implements the request_review method but doesn’t do any transformations.
// Rather, it returns itself, because when we request a review on a post already in the PendingReview state, it should stay in the PendingReview state.

// Now we can start seeing the advantages of the state pattern: the request_review method on Post is the same no matter its state value.
// Each state is responsible for its own rules.

// We’ll leave the content method on Post as is, returning an empty string slice.
// We can now have a Post in the PendingReview state as well as in the Draft state, but we want the same behavior in the PendingReview state.
// Listing 17-11 now works up to line 10!

// Adding approve to Change the Behavior of content
// The approve method will be similar to the request_review method:
//  it will set state to the value that the current state says it should have when that state is approved, as shown in Listing 17-16:

// Filename: src/lib.rs

impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// Listing 17-16: Implementing the approve method on Post and the State trait

// We add the approve method to the State trait and add a new struct that implements State, the Published state.

// Similar to the way request_review on PendingReview works, if we call the approve method on a Draft, it will have no effect because approve will return self.
// When we call approve on PendingReview, it returns a new, boxed instance of the Published struct.
// The Published struct implements the State trait, and for both the request_review method and the approve method, it returns itself, because the post should stay in the Published state in those cases.

// Now we need to update the content method on Post.
// We want the value returned from content to depend on the current state of the Post, so we’re going to have the Post delegate to a content method defined on its state, as shown in Listing 17-17:

// Filename: src/lib.rs

impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // --snip--
}

// Listing 17-17: Updating the content method on Post to delegate to a content method on State

// Because the goal is to keep all these rules inside the structs that implement State,
//  we call a content method on the value in state and pass the post instance (that is, self) as an argument.
// Then we return the value that’s returned from using the content method on the state value.

// We call the as_ref method on the Option because we want a reference to the value inside the Option rather than ownership of the value.
// Because state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned.
// If we didn’t call as_ref, we would get an error because we can’t move state out of the borrowed &self of the function parameter.

// We then call the unwrap method, which we know will never panic, because we know the methods on Post ensure that state will always contain a Some value when those methods are done.
// This is one of the cases we talked about in the “Cases In Which You Have More Information Than the Compiler” section of Chapter 9 when we know that a None value is never possible,
//  even though the compiler isn’t able to understand that.

// At this point, when we call content on the &Box<dyn State>, deref coercion will take effect on the & and the Box so the content method will ultimately be called on the type that implements the State trait.
// That means we need to add content to the State trait definition, and that is where we’ll put the logic for what content to return depending on which state we have, as shown in Listing 17-18:

// Filename: src/lib.rs

trait State {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--
struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// Listing 17-18: Adding the content method to the State trait

// We add a default implementation for the content method that returns an empty string slice.
// That means we don’t need to implement content on the Draft and PendingReview structs.
// The Published struct will override the content method and return the value in post.content.

// Note that we need lifetime annotations on this method, as we discussed in Chapter 10.
// We’re taking a reference to a post as an argument and returning a reference to part of that post,
//  so the lifetime of the returned reference is related to the lifetime of the post argument.

// And we’re done—all of Listing 17-11 now works! We’ve implemented the state pattern with the rules of the blog post workflow.
// The logic related to the rules lives in the state objects rather than being scattered throughout Post.

// Why Not An Enum?
// You may have been wondering why we didn’t use an enum with the different possible post states as variants.
// That’s certainly a possible solution, try it and compare the end results to see which you prefer!
// One disadvantage of using an enum is every place that checks the value of the enum will need a match expression or similar to handle every possible variant.
// This could get more repetitive than this trait object solution.


// Trade-offs of the State Pattern
// We’ve shown that Rust is capable of implementing the object-oriented state pattern to encapsulate the different kinds of behavior a post should have in each state.
// The methods on Post know nothing about the various behaviors.
// The way we organized the code, we have to look in only one place to know the different ways a published post can behave: the implementation of the State trait on the Published struct.

// If we were to create an alternative implementation that didn’t use the state pattern,
//  we might instead use match expressions in the methods on Post or even in the main code that checks the state of the post and changes behavior in those places.
// That would mean we would have to look in several places to understand all the implications of a post being in the published state!
// This would only increase the more states we added: each of those match expressions would need another arm.

// With the state pattern, the Post methods and the places we use Post don’t need match expressions,
//  and to add a new state, we would only need to add a new struct and implement the trait methods on that one struct.

// The implementation using the state pattern is easy to extend to add more functionality.
// To see the simplicity of maintaining code that uses the state pattern, try a few of these suggestions:

// Add a reject method that changes the post’s state from PendingReview back to Draft.
// Require two calls to approve before the state can be changed to Published.
// Allow users to add text content only when a post is in the Draft state.
// Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.

// One downside of the state pattern is that, because the states implement the transitions between states, some of the states are coupled to each other.
// If we add another state between PendingReview and Published, such as Scheduled, we would have to change the code in PendingReview to transition to Scheduled instead.
// It would be less work if PendingReview didn’t need to change with the addition of a new state, but that would mean switching to another design pattern.

// Another downside is that we’ve duplicated some logic.
// To eliminate some of the duplication, we might try to make default implementations for the request_review and approve methods on the State trait that return self;
//  however, this would violate object safety, because the trait doesn’t know what the concrete self will be exactly.
// We want to be able to use State as a trait object, so we need its methods to be object safe.

// Other duplication includes the similar implementations of the request_review and approve methods on Post.
// Both methods delegate to the implementation of the same method on the value in the state field of Option and set the new value of the state field to the result.
// If we had a lot of methods on Post that followed this pattern, we might consider defining a macro to eliminate the repetition (see the “Macros” section in Chapter 19).

// By implementing the state pattern exactly as it’s defined for object-oriented languages, we’re not taking as full advantage of Rust’s strengths as we could.
// Let’s look at some changes we can make to the blog crate that can make invalid states and transitions into compile time errors.


// Encoding States and Behavior as Types

// We’ll show you how to rethink the state pattern to get a different set of trade-offs.
// Rather than encapsulating the states and transitions completely so outside code has no knowledge of them, we’ll encode the states into different types.
// Consequently, Rust’s type checking system will prevent attempts to use draft posts where only published posts are allowed by issuing a compiler error.

// Let’s consider the first part of main in Listing 17-11:

// Filename: src/main.rs

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}

// We still enable the creation of new posts in the draft state using Post::new and the ability to add text to the post’s content.
// But instead of having a content method on a draft post that returns an empty string, we’ll make it so draft posts don’t have the content method at all.
// That way, if we try to get a draft post’s content, we’ll get a compiler error telling us the method doesn’t exist.
// As a result, it will be impossible for us to accidentally display draft post content in production,
//  because that code won’t even compile. Listing 17-19 shows the definition of a Post struct and a DraftPost struct, as well as methods on each:

// Filename: src/lib.rs

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

// Listing 17-19: A Post with a content method and a DraftPost without a content method

// Both the Post and DraftPost structs have a private content field that stores the blog post text.
// The structs no longer have the state field because we’re moving the encoding of the state to the types of the structs.
// The Post struct will represent a published post, and it has a content method that returns the content.

// We still have a Post::new function, but instead of returning an instance of Post, it returns an instance of DraftPost.
// Because content is private and there aren’t any functions that return Post, it’s not possible to create an instance of Post right now.

// The DraftPost struct has an add_text method, so we can add text to content as before, but note that DraftPost does not have a content method defined!
// So now the program ensures all posts start as draft posts, and draft posts don’t have their content available for display.
// Any attempt to get around these constraints will result in a compiler error.

// Implementing Transitions as Transformations into Different Types
// So how do we get a published post? We want to enforce the rule that a draft post has to be reviewed and approved before it can be published.
// A post in the pending review state should still not display any content.
// Let’s implement these constraints by adding another struct, PendingReviewPost, defining the request_review method on DraftPost to return a PendingReviewPost,
//  and defining an approve method on PendingReviewPost to return a Post, as shown in Listing 17-20:

// Filename: src/lib.rs

impl DraftPost {
    // --snip--
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

// Listing 17-20: A PendingReviewPost that gets created by calling request_review on DraftPost and an approve method that turns a PendingReviewPost into a published Post

// The request_review and approve methods take ownership of self,
//  thus consuming the DraftPost and PendingReviewPost instances and transforming them into a PendingReviewPost and a published Post, respectively.
// This way, we won’t have any lingering DraftPost instances after we’ve called request_review on them, and so forth.
// The PendingReviewPost struct doesn’t have a content method defined on it, so attempting to read its content results in a compiler error, as with DraftPost.
// Because the only way to get a published Post instance that does have a content method defined is to call the approve method on a PendingReviewPost,
//  and the only way to get a PendingReviewPost is to call the request_review method on a DraftPost, we’ve now encoded the blog post workflow into the type system.

// But we also have to make some small changes to main.
// The request_review and approve methods return new instances rather than modifying the struct they’re called on,
//  so we need to add more let post = shadowing assignments to save the returned instances.
// We also can’t have the assertions about the draft and pending review posts’ contents be empty strings, nor do we need them:
//  we can’t compile code that tries to use the content of posts in those states any longer.
// The updated code in main is shown in Listing 17-21:

// Filename: src/main.rs

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

// Listing 17-21: Modifications to main to use the new implementation of the blog post workflow

// The changes we needed to make to main to reassign post mean that this implementation doesn’t quite follow the object-oriented state pattern anymore:
//  the transformations between the states are no longer encapsulated entirely within the Post implementation.
// However, our gain is that invalid states are now impossible because of the type system and the type checking that happens at compile time!
// This ensures that certain bugs, such as display of the content of an unpublished post, will be discovered before they make it to production.

// Try the tasks suggested at the start of this section on the blog crate as it is after Listing 17-21 to see what you think about the design of this version of the code.
// Note that some of the tasks might be completed already in this design.

// We’ve seen that even though Rust is capable of implementing object-oriented design patterns, other patterns, such as encoding state into the type system, are also available in Rust.
// These patterns have different trade-offs.
// Although you might be very familiar with object-oriented patterns,
//  rethinking the problem to take advantage of Rust’s features can provide benefits, such as preventing some bugs at compile time.
// Object-oriented patterns won’t always be the best solution in Rust due to certain features, like ownership, that object-oriented languages don’t have.

// Summary
// No matter whether or not you think Rust is an object-oriented language after reading this chapter,
// 不管你看完本章后是否认为 Rust 是一门面向对象的语言，
//  you now know that you can use trait objects to get some object-oriented features in Rust.
//  你现在知道你可以使用特征对象来获得 Rust 中的一些面向对象的特性。
// Dynamic dispatch can give your code some flexibility in exchange for a bit of runtime performance.
// 动态调度可以给你的代码一些灵活性，以换取一点运行时性能。
// You can use this flexibility to implement object-oriented patterns that can help your code’s maintainability.
// 你可以利用这种灵活性来实现面向对象的模式，这有助于提高代码的可维护性。
// Rust also has other features, like ownership, that object-oriented languages don’t have.
// Rust 还具有面向对象语言所没有的其他功能，例如所有权。
// An object-oriented pattern won’t always be the best way to take advantage of Rust’s strengths, but is an available option.
// 面向对象的模式并不总是利用 Rust 优势的最佳方式，但它是一个可用的选项。

// Next, we’ll look at patterns, which are another of Rust’s features that enable lots of flexibility.
// 接下来，我们将看看模式，这是 Rust 的另一个功能，可以提供很大的灵活性。
// We’ve looked at them briefly throughout the book but haven’t seen their full capability yet. Let’s go!
// 我们已经在整本书中简要介绍了它们，但还没有看到它们的全部功能。 我们走吧！
