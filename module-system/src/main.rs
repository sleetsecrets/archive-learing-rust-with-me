// Managing Growing Projects with Packages, Crates, and Modules
// 使用Packages, Crates, 和 Modules管理不断增长的项目
// As you write large programs, organizing your code will become increasingly important.
// 当你编写大型程序时，组织代码变得越来越重要。
// By grouping related functionality and separating code with distinct features,
// 通过将相关功能分组，并将具有不同特征的代码分开，
// you’ll clarify where to find code that implements a particular feature and where to go to change how a feature works.
// 你将阐明在哪里可以找到实现特定功能的代码，以及在哪里可以改变功能的工作方式。

// The programs we’ve written so far have been in one module in one file.
// 到目前为止，我们编写的程序都在一个文件的一个模块中。
// As a project grows, you should organize code by splitting it into multiple modules and then multiple files.
// 随着项目的发展，你应该把代码分成多个模块，然后再分成多个文件。
// A package can contain multiple binary crates and optionally one library crate.
// 一个包可以包含多个binary crates和一个library crate。
// As a package grows, you can extract parts into separate crates that become external dependencies.
// 随着一个package的增长，你可以将各个部分提取到单独的crates中，成为外部依赖项。
// This chapter covers all these techniques.
// 本章涵盖了所有这些技术。
// For very large projects comprising a set of interrelated packages that evolve together, Cargo provides workspaces,
// 对于由一组相互关联的包组成的非常大的项目，Cargo提供工作空间，
// which we’ll cover in the “Cargo Workspaces” section in Chapter 14.
// 我们将在第14章的“Cargo Workspaces”一节中讨论。

// We’ll also discuss encapsulating implementation details, which lets you reuse code at a higher level:
// 我们还将讨论封装实现细节，这让你可以在更高的级别重用代码:
// once you’ve implemented an operation, other code can call your code via its public interface without having to know how the implementation works.
// 一旦你实现了一个操作，其他代码可以通过它的公共接口调用你的代码，而不需要知道实现是如何工作的。
// The way you write code defines which parts are public for other code to use and which parts are private implementation details that you reserve the right to change.
// 你写代码的方式定义了哪些部分是公共的供其他代码使用，哪些部分是私有的实现细节，你保留更改的权利。
// This is another way to limit the amount of detail you have to keep in your head.
// 这是另一种减少记忆细节的方法。

// A related concept is scope:
// 一个相关的概念是scope:
// the nested context in which code is written has a set of names that are defined as “in scope.” When reading, writing,
// 编写代码的嵌套上下文具有一组定义为"in scope"的名称。在reading、writing时，
// and compiling code, programmers and compilers need to know whether a particular name at a particular spot refers to a variable, function, struct, enum, module, constant, or other item and what that item means.
// 在编译代码时，programmers和编译器需要知道特定位置的特定名称是否指向变量、函数、结构、枚举、模块、常量或其他项，以及该项的含义。
// You can create scopes and change which names are in or out of scope.
// 您可以创建作用域并更改哪些名称在作用域内或外。
// You can’t have two items with the same name in the same scope; tools are available to resolve name conflicts.
// 你不能在同一个作用域中有两个同名的项目;可以使用工具来解决名称冲突。

// Rust has a number of features that allow you to manage your code’s organization,
// Rust有很多功能，可以让你管理代码的组织，
// including which details are exposed, which details are private, and what names are in each scope in your programs.
// 包括哪些细节是公开的，哪些细节是私有的，以及程序中每个作用域中的名称是什么。
// These features, sometimes collectively referred to as the module system, include:
// 这些特性有时统称为模块系统，包括:

// Packages: A Cargo feature that lets you build, test, and share crates
// 包:一个货物功能，让你建立，测试，和共享crates
// Crates: A tree of modules that produces a library or executable
// crate:生成库或可执行文件的模块树
// Modules and use: Let you control the organization, scope, and privacy of paths
// 模块和用途:让你控制路径的组织、作用域和私密性
// Paths: A way of naming an item, such as a struct, function, or module
// 路径:一种命名项的方式，如结构、函数或模块

// In this chapter, we’ll cover all these features, discuss how they interact, and explain how to use them to manage scope.
// 在本章中，我们将讨论所有这些特性，讨论它们如何相互作用，并解释如何使用它们来管理作用域。
// By the end, you should have a solid understanding of the module system and be able to work with scopes like a pro!
// 最后，你应该对模块系统有一个扎实的理解，并且能够像专业人士一样使用作用域!

// Packages and Crates
// The first parts of the module system we’ll cover are packages and crates.
// 我们将要介绍的模块系统的第一部分是 packages and crates.

// A crate is the smallest amount of code that the Rust compiler considers at a time.
// 一个crate是Rust编译器一次考虑的最小数量的代码（编译最小单位）。
// Even if you run rustc rather than cargo and pass a single source code file (as we did all the way back in the “Writing and Running a Rust Program” section of Chapter 1),
// 即使你运行rustc而不是cargo并传递一个源代码文件(就像我们在第一章的“编写和运行Rust程序”一节中所做的那样)，
// the compiler considers that file to be a crate.
// 编译器认为该文件是一个crate。
// Crates can contain modules, and the modules may be defined in other files that get compiled with the crate, as we’ll see in the coming sections.
//  crate可以包含模块，模块可以定义在与crate一起编译的其他文件中，我们将在接下来的章节中看到。

// A crate can come in one of two forms: a binary crate or a library crate.
// 一个crate可以有两种形式:binary crate或library crate。
// Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server.
// binary crate是可以编译为可执行文件的程序，例如命令行程序或服务器。
// Each must have a function called main that defines what happens when the executable runs.
// 每个文件都必须有一个名为main的函数，用于定义可执行文件运行时发生的事情。
// All the crates we’ve created so far have been binary crates.
// 到目前为止我们创建的所有箱子都是binary crate。

// Library crates don’t have a main function, and they don’t compile to an executable.
// Library crate没有main函数，也没有编译成可执行文件。
// Instead, they define functionality intended to be shared with multiple projects.
// 相反，它们定义了与多个项目共享的功能。
// For example, the rand crate we used in Chapter 2 provides functionality that generates random numbers.
// 例如，我们在第二章中使用的rand crate提供了生成随机数的功能。
// Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library".
// 大多数时候，Rustaceans说“crate”时，他们指的是library crate，并且他们可以将“crate”与一般的编程概念“library”互换使用。

// The crate root is a source file that the Rust compiler starts from and
// crate root是Rust编译器启动的源文件
//  makes up the root module of your crate (we’ll explain modules in depth in the “Defining Modules to Control Scope and Privacy” section).
//  构成了你的crate的根模块(我们将在“定义模块来控制范围和隐私”一节中深入解释模块)。

// A package is a bundle of one or more crates that provides a set of functionality.
// 一个package是一个或多个crates的捆绑，提供了一组功能。
// A package contains a Cargo.toml file that describes how to build those crates.
// 一个package包含一个Crago.toml文件，描述了如何构建这些crates.
// Cargo is actually a package that contains the binary crate for the command-line tool you’ve been using to build your code.
// Cargo实际上是一个包，其中包含用于构建代码的命令行工具的binary crate.
// The Cargo package also contains a library crate that the binary crate depends on.
// Cargo包还包含二进制crate所依赖的库crate。
// Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool uses.
// 其他项目可以依赖于Cargo library crate来实现Cargo command-line tool使用的相同逻辑。

// A package can contain as many binary crates as you like, but at most only one library crate.
// 一个package可以包含任意多的binary create，但最多只能包含一个library create。
// A package must contain at least one crate, whether that’s a library or binary crate.
// 一个包必须包含至少一个crate，无论是库还是二进制crate。

// Let’s walk through what happens when we create a package.
// 让我们来看看创建包时会发生什么。
// First, we enter the command cargo new:
// 首先，输入命令cargo new:

// $ cargo new my-project
//      Created binary (application) `my-project` package
// $ ls my-project
// Cargo.toml
// src
// $ ls my-project/src
// main.rs

// After we run cargo new, we use ls to see what Cargo creates.
// 运行cargo new后，使用ls查看cargo创建了什么。
// In the project directory, there’s a Cargo.toml file, giving us a package.
// 在项目目录中，有一个Cargo.toml的文件，给了我们一个包裹。
// There’s also a src directory that contains main.rs.
// 还有一个src目录包含main.rs。
// Open Cargo.toml in your text editor, and note there’s no mention of src/main.rs.
// 打开Cargo.toml，注意里边没有提到src/main.rs。
// Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package.
// Cargo 遵循的一个约定：src/main.rs 就是一个与包同名的binary crate 的 crate root。
// Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.
// 同样的，Cargo 知道如果包目录中包含 src/lib.rs，则包带有与其同名的linrary crate，且 src/lib.rs 是 crate root。
// Cargo passes the crate root files to rustc to build the library or binary.
// Cargo将crate根文件传递给rustc以构建库或二进制文件。

// Here, we have a package that only contains src/main.rs, meaning it only contains a binary crate named my-project.
// 这里，我们有一个只包含src/main.rs的包。意味着它只包含一个名为my-project的binary crate。
// If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package.
// 如果一个package包含 src/main.rs 和 src/lib.rs，它有两个crates：一个binary文件和一个library文件，两者都与包同名。
// A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.
// 通过将文件放在 src/bin 目录中，一个包可以有多个binary creates：每个文件都是一个单独的binary create。

// Defining Modules to Control Scope and Privacy
// 定义模块以控制范围和隐私
// In this section, we’ll talk about modules and other parts of the module system, namely paths that allow you to name items;
// 在本节中，我们将讨论模块和模块系统的其他部分，即允许您命名项目的路径；
// the use keyword that brings a path into scope; and the pub keyword to make items public.
// 将路径引入作用域的 use 关键字； 和 pub 关键字使项目公开。
// We’ll also discuss the as keyword, external packages, and the glob operator.
// 我们还将讨论 as 关键字、外部包和 glob 运算符。

// First, we’re going to start with a list of rules for easy reference when you’re organizing your code in the future.
// 首先，我们将从一个规则列表开始，以便您将来组织代码时方便参考。
// Then we’ll explain each of the rules in detail.
// 然后我们将详细解释每条规则。

// Modules Cheat Sheet
// 模块备忘单
// Here we provide a quick reference on how modules, paths, the use keyword, and the pub keyword work in the compiler, and how most developers organize their code.
// 在这里，我们提供了有关模块、路径、use 关键字和 pub 关键字在编译器中的工作方式以及大多数开发人员如何组织代码的快速参考。
// We’ll be going through examples of each of these rules throughout this chapter, but this is a great place to refer to as a reminder of how modules work.
// 我们将在本章中逐一介绍这些规则的示例，但这是一个很好的参考地方，可以提醒您模块是如何工作的。

// Start from the crate root: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
// 从 crate root 开始：编译 crate 时，编译器首先在 crate 根文件中查找要编译的代码（对于库 crate 通常是 src/lib.rs，对于二进制 crate 通常是 src/main.rs）。

// Declaring modules: In the crate root file, you can declare new modules; say, you declare a “garden” module with mod garden;.
// 声明模块：在 crate 根文件中，你可以声明新的模块； 比如说，你用 mod garden; 声明了一个“花园”模块。
// The compiler will look for the module’s code in these places:
// 编译器会在这些地方寻找模块的代码：
// ├── Inline, within curly brackets that replace the semicolon following mod garden
// ├── 内联，在替换 mod garden 后面分号的大括号内
// ├── In the file src/garden.rs
// ├── 在文件 src/garden.rs 中
// ├── In the file src/garden/mod.rs
// ├── 在文件 src/garden/mod.rs 中


// Declaring submodules: In any file other than the crate root, you can declare submodules.
// 声明子模块：在 crate 根以外的任何文件中，您都可以声明子模块。
// For example, you might declare mod vegetables; in src/garden.rs.
// 例如，您可以声明 mod vegetables； 在 src/garden.rs 中。
// The compiler will look for the submodule’s code within the directory named for the parent module in these places:
// 编译器将在以父模块命名的目录中的这些地方查找子模块的代码：
// ├── Inline, directly following mod vegetables, within curly brackets instead of the semicolon
// ├── 内联，直接跟在 mod vegetables 之后，在大括号内而不是分号
// ├── In the file src/garden/vegetables.rs
// ├── 在文件 src/garden/vegetables.rs 中
// ├── In the file src/garden/vegetables/mod.rs
// ├── 在文件 src/garden/vegetables/mod.rs 中

// Paths to code in modules: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate,
// 模块中代码的路径：一旦module（模块）成为你的 crate 的一部分，你就可以从同一个 crate 中的任何其他地方引用该模块中的代码，
// as long as the privacy rules allow, using the path to the code.
// 只要隐私规则允许，使用代码的路径。
// For example, an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus.
// 例如，花园蔬菜模块中的 Asparagus 类型可以在 crate::garden::vegetables::Asparagus 中找到。

// Private vs public: Code within a module is private from its parent modules by default.
// 私有 vs 公共：默认情况下，模块中的代码对其父模块是私有的。
// To make a module public, declare it with pub mod instead of mod.
// 要使模块公开，请使用 pub mod 而不是 mod 来声明它。
// To make items within a public module public as well, use pub before their declarations.
// 要使公共模块中的项目也公开，请在它们的声明之前使用 pub。

// The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths.
// use 关键字：在一个范围内，use 关键字创建项目的快捷方式以减少长路径的重复。
// In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with use crate::garden::vegetables::Asparagus;
// 在任何可以引用 crate::garden::vegetables::Asparagus 的范围内，你可以使用 use crate::garden::vegetables::Asparagus 创建快捷方式；
// and from then on you only need to write Asparagus to make use of that type in the scope.
// 从那时起，您只需编写 Asparagus 即可在范围内使用该类型。

// Here we create a binary crate named backyard that illustrates these rules.
// 在这里，我们创建了一个名为 backyard 的二进制包来说明这些规则。
// The crate’s directory, also named backyard, contains these files and directories:
// crate 的目录，也称为 backyard，包含以下文件和目录：

// backyard
// ├── Cargo.lock
// ├── Cargo.toml
// └── src
//     ├── garden
//     │   └── vegetables.rs
//     ├── garden.rs
//     └── main.rs

// The crate root file in this case is src/main.rs, and it contains:
// 本例中的 crate 根文件是 src/main.rs，它包含：

// Filename: src/main.rs
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

// The pub mod garden; line tells the compiler to include the code it finds in src/garden.rs, which is:
// The pub mod garden; 行告诉编译器包含它在 src/garden.rs 中找到的代码，它是：

// Filename: src/garden.rs
pub mod vegetables;

// Here, pub mod vegetables; means the code in src/garden/vegetables.rs is included too. That code is:
// 在这里，pub mod vegetables; 意味着 src/garden/vegetables.rs 中的代码也包含在内。 该代码是：

#[derive(Debug)]
pub struct Asparagus {}

// Now let’s get into the details of these rules and demonstrate them in action!
// 现在让我们深入了解这些规则的细节并在行动中演示它们！

// Grouping Related Code in Modules
// 将相关代码分组到模块中
// Modules let us organize code within a crate for readability and easy reuse.
// 模块让我们可以将代码组织在一个 crate 中，以提高可读性和重用性。
// Modules also allow us to control the privacy of items, because code within a module is private by default.
// 模块还允许我们控制项目的隐私，因为默认情况下模块内的代码是私有的。
// Private items are internal implementation details not available for outside use.
// 私有项是内部实现细节，不可用于外部。
// We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.
// 我们可以选择将模块和其中的项公开，这样可以将它们公开以允许外部代码使用和依赖它们。

// As an example, let’s write a library crate that provides the functionality of a restaurant.
// 例如，让我们编写一个提供餐厅功能的库 crate。
// We’ll define the signatures of functions but leave their bodies empty to concentrate on the organization of the code, rather than the implementation of a restaurant.
// 在餐饮业中，餐厅的某些部分称为前台，而其他部分称为后台。

// In the restaurant industry, some parts of a restaurant are referred to as front of house and others as back of house.
// 我们将定义函数的签名，但将函数体留空以专注于代码的组织，而不是餐厅的实现。
// Front of house is where customers are; this encompasses where the hosts seat customers, servers take orders and payment, and bartenders make drinks.
// 前台是顾客所在的地方； 这包括主人为顾客安排座位、服务员接受订单和付款以及调酒师调酒的地方。
// Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.
// 后台是厨师和厨师在厨房工作的地方，洗碗机负责清洁工作，管理人员负责行政工作。

// To structure our crate in this way, we can organize its functions into nested modules.
// 要以这种方式构建我们的箱子（create），我们可以将其功能组织到嵌套模块中。
// Create a new library named restaurant by running cargo new restaurant --lib;
// 通过运行 cargo new restaurant --lib 创建一个名为 restaurant 的新库；
// then enter the code in Listing 7-1 into src/lib.rs to define some modules and function signatures.
// 然后将清单 7-1 中的代码输入到 src/lib.rs 中以定义一些模块和函数签名。
// Here’s the front of house section:
// 这是房子前面的部分：

// Filename: src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Listing 7-1: A front_of_house module containing other modules that then contain functions
// 清单 7-1：一个包含其他模块的 front_of_house 模块，然后包含函数

// We define a module with the mod keyword followed by the name of the module (in this case, front_of_house).
// 我们使用 mod 关键字定义一个模块，后跟模块名称（在本例中为 front_of_house）。
// The body of the module then goes inside curly brackets. Inside modules, we can place other modules, as in this case with the modules hosting and serving.
// 然后模块的主体放在大括号内。 在模块内部，我们可以放置其他模块，如本例中的托管和服务模块。
// Modules can also hold definitions for other items, such as structs, enums, constants, traits, and—as in Listing 7-1—functions.
// 模块还可以保存其他项的定义，例如结构、枚举、常量、特征和——如清单 7-1 所示——函数。

// By using modules, we can group related definitions together and name why they’re related.
// 通过使用模块，我们可以将相关的定义组合在一起并说明它们相关的原因。
// Programmers using this code can navigate the code based on the groups rather than having to read through all the definitions, making it easier to find the definitions relevant to them.
// 使用此代码的程序员可以根据组导航代码，而不必通读所有定义，从而更容易找到与其相关的定义。
// Programmers adding new functionality to this code would know where to place the code to keep the program organized.
// 向此代码添加新功能的程序员将知道将代码放在哪里以保持程序的组织性。

// Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots.
// 早些时候，我们提到 src/main.rs 和 src/lib.rs 被称为 crate roots。
// The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree.
// 之所以命名，是因为这两个文件中的任何一个的内容在 crate 模块结构的根部形成了一个名为 crate 的模块，称为模块树。

// Listing 7-2 shows the module tree for the structure in Listing 7-1.
// 清单 7-2 显示了清单 7-1 中结构的模块树。

// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// Listing 7-2: The module tree for the code in Listing 7-1
// 清单 7-2：清单 7-1 中代码的模块树

// This tree shows how some of the modules nest inside one another; for example, hosting nests inside front_of_house.
// 这棵树显示了一些模块是如何嵌套在另一个模块中的； 例如，在 front_of_house 内托管巢穴。
// The tree also shows that some modules are siblings to each other, meaning they’re defined in the same module; hosting and serving are siblings defined within front_of_house.
// 树还显示一些模块是彼此的兄弟，这意味着它们在同一个模块中定义； hosting 和 serving 是 front_of_house 中定义的兄弟姐妹。
// If module A is contained inside module B, we say that module A is the child of module B and that module B is the parent of module A.
// 如果模块 A 包含在模块 B 中，我们说模块 A 是模块 B 的子模块，模块 B 是模块 A 的父模块。
// Notice that the entire module tree is rooted under the implicit module named crate.
// 请注意，整个模块树都以名为 crate 的隐式模块为根。

// The module tree might remind you of the filesystem’s directory tree on your computer; this is a very apt comparison!
// 模块树可能会让您想起计算机上文件系统的目录树； 这是一个非常贴切的类比！
// Just like directories in a filesystem, you use modules to organize your code.
// 就像文件系统中的目录一样，您使用模块来组织代码。
// And just like files in a directory, we need a way to find our modules.
// 就像目录中的文件一样，我们需要一种方法来找到我们的模块。

// Paths for Referring to an Item in the Module Tree
// 引用模块树中项目的路径
// To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem.
// 为了向 Rust 显示在模块树中的何处查找项目，我们使用路径的方式与我们在导航文件系统时使用路径的方式相同。
// To call a function, we need to know its path.
// 要调用一个函数，我们需要知道它的路径。

// A path can take two forms:
// 路径可以有两种形式：
// An absolute path is the full path starting from a crate root;
// 绝对路径是从 crate root 开始的完整路径；
//  for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
//  对于来自外部 crate 的代码，绝对路径以 crate 名称开头，对于来自当前 crate 的代码，它以文字 crate 开头。
// A relative path starts from the current module and uses self, super, or an identifier in the current module.
// 相对路径从当前模块开始，使用self、super或当前模块中的标识符。

// Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).
// 绝对路径和相对路径后跟一个或多个由双冒号 (::) 分隔的标识符。

// Returning to Listing 7-1, say we want to call the add_to_waitlist function.
// 回到示例 7-1，假设我们要调用 add_to_waitlist 函数。
// This is the same as asking: what’s the path of the add_to_waitlist function? Listing 7-3 contains Listing 7-1 with some of the modules and functions removed.
// 这等同于询问：add_to_waitlist 函数的路径是什么？ 清单 7-3 包含清单 7-1，但删除了一些模块和函数。

// We’ll show two ways to call the add_to_waitlist function from a new function eat_at_restaurant defined in the crate root.
// 我们将展示两种从包根中定义的新函数 eat_at_restaurant 调用 add_to_waitlist 函数的方法。
// These paths are correct, but there’s another problem remaining that will prevent this example from compiling as-is.
// 这些路径是正确的，但还有另一个问题会阻止此示例按原样编译。
// We’ll explain why in a bit.
// 我们稍后会解释原因。

// The eat_at_restaurant function is part of our library crate’s public API, so we mark it with the pub keyword.
// eat_at_restaurant 函数是我们库 crate 的公共 API 的一部分，所以我们用 pub 关键字标记它。
// In the “Exposing Paths with the pub Keyword” section, we’ll go into more detail about pub.
// 在“使用 pub 关键字公开路径”部分，我们将详细介绍 pub。

// Filename: src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
// Listing 7-3: Calling the add_to_waitlist function using absolute and relative paths
// 示例 7-3：使用绝对路径和相对路径调用 add_to_waitlist 函数

// The first time we call the add_to_waitlist function in eat_at_restaurant, we use an absolute path.
// 第一次调用eat_at_restaurant中的add_to_waitlist函数，我们使用的是绝对路径。
// The add_to_waitlist function is defined in the same crate as eat_at_restaurant, which means we can use the crate keyword to start an absolute path.
// add_to_waitlist 函数定义在与 eat_at_restaurant 相同的 crate 中，这意味着我们可以使用 crate 关键字来启动绝对路径。
// We then include each of the successive modules until we make our way to add_to_waitlist.
// 然后我们包括每个连续的模块，直到我们进入 add_to_waitlist。
// You can imagine a filesystem with the same structure: we’d specify the path /front_of_house/hosting/add_to_waitlist to run the add_to_waitlist program;
// 你可以想象一个具有相同结构的文件系统：我们指定路径 /front_of_house/hosting/add_to_waitlist 来运行 add_to_waitlist 程序；
// using the crate name to start from the crate root is like using / to start from the filesystem root in your shell.
// 使用 crate 名称从 crate root 开始就像使用 / 从 shell 中的文件系统根开始一样。

// The second time we call add_to_waitlist in eat_at_restaurant, we use a relative path.
// 第二次调用eat_at_restaurant中的add_to_waitlist，我们使用的是相对路径。
// The path starts with front_of_house, the name of the module defined at the same level of the module tree as eat_at_restaurant.
// 路径以 front_of_house 开头，与 eat_at_restaurant 在模块树的同一层定义的模块名称。
// Here the filesystem equivalent would be using the path front_of_house/hosting/add_to_waitlist.
// 此处等效的文件系统将使用路径 front_of_house/hosting/add_to_waitlist。
// Starting with a module name means that the path is relative.
// 以模块名称开头意味着路径是相对的。

// Choosing whether to use a relative or absolute path is a decision you’ll make based on your project,
// 选择使用相对路径还是绝对路径是您根据项目做出的决定，
// and depends on whether you’re more likely to move item definition code separately from or together with the code that uses the item.
// 并且取决于您是否更有可能将项目定义代码与使用该项目的代码分开或一起移动。
// For example, if we move the front_of_house module and the eat_at_restaurant function into a module named customer_experience,
// 例如，如果我们将 front_of_house 模块和 eat_at_restaurant 函数移动到名为 customer_experience 的模块中，
// we’d need to update the absolute path to add_to_waitlist, but the relative path would still be valid.
// 我们需要更新 add_to_waitlist 的绝对路径，但相对路径仍然有效。
// However, if we moved the eat_at_restaurant function separately into a module named dining, the absolute path to the add_to_waitlist call would stay the same,
// 但是，如果我们将 eat_at_restaurant 函数单独移动到名为 dining 的模块中，add_to_waitlist 调用的绝对路径将保持不变，
// but the relative path would need to be updated.
// 但相对路径需要更新。
// Our preference in general is to specify absolute paths because it’s more likely we’ll want to move code definitions and item calls independently of each other.
// 我们通常倾向于指定绝对路径，因为我们更有可能希望彼此独立地移动代码定义和项目调用。

// Let’s try to compile Listing 7-3 and find out why it won’t compile yet!
// 让我们尝试编译示例 7-3 并找出为什么它还不能编译！
// The error we get is shown in Listing 7-4.
// 我们得到的错误如清单 7-4 所示。

// $ cargo build
//    Compiling restaurant v0.1.0 (file:///projects/restaurant)
// error[E0603]: module `hosting` is private
//  --> src/lib.rs:9:28
//   |
// 9 |     crate::front_of_house::hosting::add_to_waitlist();
//   |                            ^^^^^^^ private module
//   |
// note: the module `hosting` is defined here
//  --> src/lib.rs:2:5
//   |
// 2 |     mod hosting {
//   |     ^^^^^^^^^^^
//
// error[E0603]: module `hosting` is private
//   --> src/lib.rs:12:21
//    |
// 12 |     front_of_house::hosting::add_to_waitlist();
//    |                     ^^^^^^^ private module
//    |
// note: the module `hosting` is defined here
//   --> src/lib.rs:2:5
//    |
// 2  |     mod hosting {
//    |     ^^^^^^^^^^^
//
// For more information about this error, try `rustc --explain E0603`.
// error: could not compile `restaurant` due to 2 previous errors

// Listing 7-4: Compiler errors from building the code in Listing 7-3
// 清单 7-4：构建清单 7-3 中代码的编译器错误

// The error messages say that module hosting is private.
// 错误消息说 hosting 模块是私有的。
// In other words, we have the correct paths for the hosting module and the add_to_waitlist function, but Rust won’t let us use them because it doesn’t have access to the private sections.
// 换句话说，我们有 hosting 模块和 add_to_waitlist 函数的正确路径，但 Rust 不允许我们使用它们，因为它无法访问私有部分。
// In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.
// 在 Rust 中，所有项（函数、方法、结构、枚举、模块和常量）默认情况下对父模块都是私有的。
// If you want to make an item like a function or struct private, you put it in a module.
// 如果你想让一个项目像一个函数或结构私有，你把它放在一个模块中。

// Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.
// 父模块中的项不能使用子模块中的私有项，但子模块中的项可以使用其祖先模块中的项。
// This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined. To continue with our metaphor,
// 这是因为子模块包装并隐藏了它们的实现细节，但是子模块可以看到它们定义的上下文。 继续我们的比喻，
// think of the privacy rules as being like the back office of a restaurant: what goes on in there is private to restaurant customers,
// 将隐私规则想象成餐馆的后台：那里发生的事情对餐馆顾客来说是私人的，
// but office managers can see and do everything in the restaurant they operate.
// 但办公室经理可以在他们经营的餐厅中看到并执行所有操作。

// Rust chose to have the module system function this way so that hiding inner implementation details is the default.
// Rust 选择以这种方式让模块系统发挥作用，以便默认隐藏内部实现细节。
// That way, you know which parts of the inner code you can change without breaking outer code.
// 这样，您就知道可以在不破坏外部代码的情况下更改内部代码的哪些部分。
// However, Rust does give you the option to expose inner parts of child modules’ code to outer ancestor modules by using the pub keyword to make an item public.
// 但是，Rust 确实为您提供了通过使用 pub 关键字公开项目来将子模块代码的内部部分公开给外部祖先模块的选项。

// Exposing Paths with the pub Keyword
// 使用 pub 关键字公开路径
// Let’s return to the error in Listing 7-4 that told us the hosting module is private.
// 让我们回到清单 7-4 中告诉我们 hosting 模块是私有的错误。
// We want the eat_at_restaurant function in the parent module to have access to the add_to_waitlist function in the child module, so we mark the hosting module with the pub keyword,
// 我们希望父模块中的 eat_at_restaurant 函数可以访问子模块中的 add_to_waitlist 函数，所以我们用 pub 关键字标记 hosting 模块，
//  as shown in Listing 7-5.
// 如清单 7-5 所示。

// Filename: src/lib.rs

mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Listing 7-5: Declaring the hosting module as pub to use it from eat_at_restaurant
// 示例 7-5：将 hosting 模块声明为 pub 以便在 eat_at_restaurant 中使用它

// Unfortunately, the code in Listing 7-5 still results in an error, as shown in Listing 7-6.
// 不幸的是，清单 7-5 中的代码仍然会导致错误，如清单 7-6 所示。

// $ cargo build
//    Compiling restaurant v0.1.0 (file:///projects/restaurant)
// error[E0603]: function `add_to_waitlist` is private
//  --> src/lib.rs:9:37
//   |
// 9 |     crate::front_of_house::hosting::add_to_waitlist();
//   |                                     ^^^^^^^^^^^^^^^ private function
//   |
// note: the function `add_to_waitlist` is defined here
//  --> src/lib.rs:3:9
//   |
// 3 |         fn add_to_waitlist() {}
//   |         ^^^^^^^^^^^^^^^^^^^^
//
// error[E0603]: function `add_to_waitlist` is private
//   --> src/lib.rs:12:30
//    |
// 12 |     front_of_house::hosting::add_to_waitlist();
//    |                              ^^^^^^^^^^^^^^^ private function
//    |
// note: the function `add_to_waitlist` is defined here
//   --> src/lib.rs:3:9
//    |
// 3  |         fn add_to_waitlist() {}
//    |         ^^^^^^^^^^^^^^^^^^^^
//
// For more information about this error, try `rustc --explain E0603`.
// error: could not compile `restaurant` due to 2 previous errors

// Listing 7-6: Compiler errors from building the code in Listing 7-5
// 示例 7-6：构建示例 7-5 中代码的编译器错误

// What happened? Adding the pub keyword in front of mod hosting makes the module public.
// 发生了什么？ 在 mod hosting 前添加 pub 关键字使模块公开。
// With this change, if we can access front_of_house, we can access hosting.
// 有了这个改变，如果我们可以访问 front_of_house，我们就可以访问托管。
// But the contents of hosting are still private; making the module public doesn’t make its contents public.
// 但是托管的内容仍然是私有的； 公开模块并不会公开其内容。
// The pub keyword on a module only lets code in its ancestor modules refer to it, not access its inner code.
// 模块上的 pub 关键字只允许其祖先模块中的代码引用它，而不能访问其内部代码。
// Because modules are containers, there’s not much we can do by only making the module public;
// 因为模块是容器，所以我们只能通过公开模块来做很多事情；
// we need to go further and choose to make one or more of the items within the module public as well.
// 我们需要更进一步，选择公开模块中的一项或多项。

// The errors in Listing 7-6 say that the add_to_waitlist function is private.
// 清单 7-6 中的错误表明 add_to_waitlist 函数是私有的。
// The privacy rules apply to structs, enums, functions, and methods as well as modules.
// 隐私规则适用于结构、枚举、函数和方法以及模块。

// Let’s also make the add_to_waitlist function public by adding the pub keyword before its definition, as in Listing 7-7.
// 让我们也通过在其定义前添加 pub 关键字来使 add_to_waitlist 函数公开，如清单 7-7 所示。

// Filename: src/lib.rs

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// Listing 7-7: Adding the pub keyword to mod hosting and fn add_to_waitlist lets us call the function from eat_at_restaurant
// 示例 7-7：将 pub 关键字添加到 mod hosting 和 fn add_to_waitlist 让我们从 eat_at_restaurant 调用函数

// Now the code will compile! To see why adding the pub keyword lets us use these paths in add_to_waitlist with respect to the privacy rules, let’s look at the absolute and the relative paths.
// 现在代码可以编译了！ 要了解为什么添加 pub 关键字允许我们根据隐私规则在 add_to_waitlist 中使用这些路径，让我们看一下绝对路径和相对路径。

// In the absolute path, we start with crate, the root of our crate’s module tree.
// 在绝对路径中，我们从 crate 开始，它是 crate 模块树的根。
// The front_of_house module is defined in the crate root.
// front_of_house 模块在 crate root 中定义。
// While front_of_house isn’t public, because the eat_at_restaurant function is defined in the same module as front_of_house (that is, eat_at_restaurant and front_of_house are siblings),
// 虽然 front_of_house 不是公共的，因为 eat_at_restaurant 函数与 front_of_house 定义在同一模块中（也就是说，eat_at_restaurant 和 front_of_house 是兄弟姐妹），
// we can refer to front_of_house from eat_at_restaurant.
// 我们可以参考 eat_at_restaurant 中的 front_of_house。
// Next is the hosting module marked with pub.
// 接下来是标有 pub 的托管模块。
// We can access the parent module of hosting, so we can access hosting.
// 我们可以访问hosting的父模块，所以我们可以访问hosting。
// Finally, the add_to_waitlist function is marked with pub and we can access its parent module, so this function call works!
// 最后，add_to_waitlist 函数被标记为 pub，我们可以访问它的父模块，所以这个函数调用有效！

// In the relative path, the logic is the same as the absolute path except for the first step: rather than starting from the crate root, the path starts from front_of_house.
// 在相对路径中，逻辑与绝对路径相同，除了第一步：路径不是从 crate root 开始，而是从 front_of_house 开始。
// The front_of_house module is defined within the same module as eat_at_restaurant, so the relative path starting from the module in which eat_at_restaurant is defined works.
// front_of_house 模块定义在与 eat_at_restaurant 相同的模块中，因此从定义 eat_at_restaurant 的模块开始的相对路径有效。
// Then, because hosting and add_to_waitlist are marked with pub, the rest of the path works, and this function call is valid!
// 那么，因为hosting和add_to_waitlist都标有pub，剩下的路径就可以了，这个函数调用有效！

// If you plan on sharing your library crate so other projects can use your code, your public API is your contract with users of your crate that determines how they can interact with your code.
// 如果您计划共享您的库 crate，以便其他项目可以使用您的代码，那么您的公共 API 就是您与您的 crate 用户的合同，它决定了他们如何与您的代码交互。
// There are many considerations around managing changes to your public API to make it easier for people to depend on your crate.
// 管理对公共 API 的更改有很多考虑因素，以使人们更容易依赖您的箱子（crate）。
// These considerations are out of the scope of this book; if you’re interested in this topic, see The Rust API Guidelines.
// 这些注意事项超出了本书的范围； 如果您对此主题感兴趣，请参阅 Rust API 指南。

// Best Practices for Packages with a Binary and a Library
// 包含二进制文件和库的包的最佳实践
// We mentioned a package can contain both a src/main.rs binary crate root as well as a src/lib.rs library crate root, and both crates will have the package name by default.
// 我们提到一个包可以同时包含一个 src/main.rs 二进制包根和一个 src/lib.rs 库包根，默认情况下这两个包都有包名。
// Typically, packages with this pattern of containing both a library and a binary crate will have just enough code in the binary crate to start an executable that calls code with the library crate.
// 通常，具有这种同时包含库和二进制 crate 的模式的包将在二进制 crate 中包含足够的代码来启动一个可执行文件，该可执行文件使用库 crate 调用代码。
// This lets other projects benefit from the most functionality that the package provides, because the library crate’s code can be shared.
// 这让其他项目可以从包提供的大部分功能中受益，因为库 crate 的代码可以共享。

// The module tree should be defined in src/lib.rs.
// 模块树应该在 src/lib.rs 中定义。
// Then, any public items can be used in the binary crate by starting paths with the name of the package.
// 然后，任何公共项都可以通过以包名开头的路径在二进制包中使用。
// The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: it can only use the public API.
// binary crate 成为 library crate 的用户，就像一个完全外部的 crate 会使用 library crate：它只能使用公共 API。
// This helps you design a good API; not only are you the author, you’re also a client!
// 这有助于你设计一个好的 API； 您不仅是作者，还是客户！

// In Chapter 12, we’ll demonstrate this organizational practice with a command-line program that will contain both a binary crate and a library crate.
// 在第 12 章中，我们将使用包含二进制包和库包的命令行程序来演示这种组织实践。

// Starting Relative Paths with super
// 使用 super 启动相对路径
// We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using super at the start of the path.
// 我们可以通过在路径的开头使用 super 来构造从父模块开始的相对路径，而不是当前模块或 crate 根目录。
// This is like starting a filesystem path with the .. syntax. Using super allows us to reference an item that we know is in the parent module,
// 这就像使用 .. 语法开始一个文件系统路径。 使用 super 允许我们引用我们知道在父模块中的项目，
// which can make rearranging the module tree easier when the module is closely related to the parent, but the parent might be moved elsewhere in the module tree someday.
// 当模块与父模块密切相关时，这可以使重新排列模块树更容易，但有一天父模块可能会移动到模块树中的其他位置。

// Consider the code in Listing 7-8 that models the situation in which a chef fixes an incorrect order and personally brings it out to the customer.
// 考虑清单 7-8 中的代码，它模拟了厨师修正错误订单并亲自将其带给顾客的情况。
// The function fix_incorrect_order defined in the back_of_house module calls the function deliver_order defined in the parent module by specifying the path to deliver_order starting with super:
// back_of_house 模块中定义的函数 fix_incorrect_order 调用父模块中定义的函数 deliver_order 指定以 super 开头的 deliver_order 路径：

// Filename: src/lib.rs
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
// Listing 7-8: Calling a function using a relative path starting with super
// 示例 7-8：使用以 super 开头的相对路径调用函数

// The fix_incorrect_order function is in the back_of_house module, so we can use super to go to the parent module of back_of_house, which in this case is crate, the root.
// fix_incorrect_order 函数在 back_of_house 模块中，因此我们可以使用 super 转到 back_of_house 的父模块，在本例中是 crate，root。
// From there, we look for deliver_order and find it.
// 从那里，我们寻找 deliver_order 并找到它。
// Success! We think the back_of_house module and the deliver_order function are likely to stay in the same relationship to each other and get moved together should we decide to reorganize the crate’s module tree.
// 成功！ 我们认为 back_of_house 模块和 deliver_order 函数很可能保持彼此相同的关系并一起移动，如果我们决定重新组织 crate 的模块树的话。
// Therefore, we used super so we’ll have fewer places to update code in the future if this code gets moved to a different module.
// 因此，我们使用了 super，这样如果这段代码被移动到不同的模块，我们将来更新代码的地方就会更少。

// Making Structs and Enums Public
// 公开结构和枚举
// We can also use pub to designate structs and enums as public, but there are a few details extra to the usage of pub with structs and enums.
// 我们也可以使用 pub 将结构和枚举指定为公共的，但是对于将 pub 与结构和枚举一起使用还有一些额外的细节。
// If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private.
// 如果我们在结构定义之前使用 pub，我们将结构公开，但结构的字段仍然是私有的。
// We can make each field public or not on a case-by-case basis.
// 我们可以根据具体情况将每个字段公开或不公开。
// In Listing 7-9, we’ve defined a public back_of_house::Breakfast struct with a public toast field but a private seasonal_fruit field.
// 在清单 7-9 中，我们定义了一个公共的 back_of_house::Breakfast 结构，它有一个公共的 toast 字段和一个私有的 seasonal_fruit 字段。
// This models the case in a restaurant where the customer can pick the type of bread that comes with a meal,
// 这模拟了餐厅的情况，顾客可以选择随餐提供的面包类型，
// but the chef decides which fruit accompanies the meal based on what’s in season and in stock.
// 但厨师会根据时令和库存来决定搭配哪种水果。
// The available fruit changes quickly, so customers can’t choose the fruit or even see which fruit they’ll get.
// 可用的水果变化很快，所以客户无法选择水果，甚至看不到他们会得到哪种水果。

// Filename: src/lib.rs
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    // 在夏天订购黑麦吐司早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    // 改变我们对想要什么面包的想法
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // 如果我们取消注释，下一行将不会编译； 我们不被允许
    // to see or modify the seasonal fruit that comes with the meal
    // 查看或修改随餐提供的时令水果
    // meal.seasonal_fruit = String::from("blueberries");
}

// Listing 7-9: A struct with some public fields and some private fields
// 示例 7-9：具有一些公共字段和一些私有字段的结构

// Because the toast field in the back_of_house::Breakfast struct is public, in eat_at_restaurant we can write and read to the toast field using dot notation.
// 因为 back_of_house::Breakfast 结构中的 toast 字段是公共的，所以在 eat_at_restaurant 中我们可以使用点符号写入和读取 toast 字段。
// Notice that we can’t use the seasonal_fruit field in eat_at_restaurant because seasonal_fruit is private.
// 请注意，我们不能在 eat_at_restaurant 中使用 seasonal_fruit 字段，因为 seasonal_fruit 是私有的。
// Try uncommenting the line modifying the seasonal_fruit field value to see what error you get!
// 尝试取消注释修改 seasonal_fruit 字段值的行，看看你得到了什么错误！

// Also, note that because back_of_house::Breakfast has a private field,
// 另外，请注意因为 back_of_house::Breakfast 有一个私有字段，
// the struct needs to provide a public associated function that constructs an instance of Breakfast (we’ve named it summer here).
// 该结构需要提供一个公共关联函数来构造 Breakfast 的实例（我们在这里将其命名为 summer）。
// If Breakfast didn’t have such a function, we couldn’t create an instance of Breakfast in eat_at_restaurant because we couldn’t set the value of the private seasonal_fruit field in eat_at_restaurant.
// 如果 Breakfast 没有这个函数，我们就无法在 eat_at_restaurant 中创建 Breakfast 的实例，因为我们无法设置 eat_at_restaurant 中私有 seasonal_fruit 字段的值。

// In contrast, if we make an enum public, all of its variants are then public.
// 相反，如果我们公开一个枚举，那么它的所有变体都是公开的。
// We only need the pub before the enum keyword, as shown in Listing 7-10.
// 我们只需要在 enum 关键字之前加上 pub，如示例 7-10 所示。

// Filename: src/lib.rs
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
// Listing 7-10: Designating an enum as public makes all its variants public
// 示例 7-10：将一个枚举指定为 public 会使它的所有变体都公开

// Because we made the Appetizer enum public, we can use the Soup and Salad variants in eat_at_restaurant.
// 因为我们公开了 Appetizer 枚举，所以我们可以在 eat_at_restaurant 中使用 Soup 和 Salad 变体。

// Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public.
// 如果枚举成员不是公有的，那么枚举会显得用处不大； 在每种情况下都必须用 pub 注释所有枚举变体会很烦人，因此枚举变体的默认值是公开的。
// Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.
// 结构通常在其字段不公开的情况下很有用，因此结构字段遵循默认情况下所有内容都是私有的一般规则，除非用 pub 注释。

// There’s one more situation involving pub that we haven’t covered, and that is our last module system feature: the use keyword.
// 还有一种涉及 pub 的情况我们没有涉及，那就是我们最后一个模块系统特性：use 关键字。
// We’ll cover use by itself first, and then we’ll show how to combine pub and use.
// 我们将首先介绍 use 本身，然后我们将展示如何结合 pub 和 use。

// Bringing Paths into Scope with the use Keyword
// 使用 use 关键字将路径引入范围
// Having to write out the paths to call functions can feel inconvenient and repetitive.
// 必须写出调用函数的路径会让人感到不便和重复。
// In Listing 7-7, whether we chose the absolute or relative path to the add_to_waitlist function, every time we wanted to call add_to_waitlist we had to specify front_of_house and hosting too.
// 在清单 7-7 中，无论我们选择 add_to_waitlist 函数的绝对路径还是相对路径，每次我们想要调用 add_to_waitlist 时，我们都必须指定 front_of_house 和 hosting。
// Fortunately, there’s a way to simplify this process: we can create a shortcut to a path with the use keyword once, and then use the shorter name everywhere else in the scope.
// 幸运的是，有一种方法可以简化这个过程：我们可以使用 use 关键字创建一个路径的快捷方式，然后在范围内的其他地方使用更短的名称。

// In Listing 7-11,
// we bring the crate::front_of_house::hosting module into the scope of the eat_at_restaurant function so we only have to specify hosting::add_to_waitlist to call the add_to_waitlist function in eat_at_restaurant.
// 我们将 crate::front_of_house::hosting 模块引入到 eat_at_restaurant 函数的范围内，因此我们只需指定 hosting::add_to_waitlist 即可调用 eat_at_restaurant 中的 add_to_waitlist 函数。

// Filename: src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
// Listing 7-11: Bringing a module into scope with use
// 示例 7-11：使用 use 将模块引入作用域

// Adding use and a path in a scope is similar to creating a symbolic link in the filesystem.
// 在作用域中添加使用和路径类似于在文件系统中创建符号链接。
// By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope, just as though the hosting module had been defined in the crate root.
// 通过在 crate root 中添加 use crate::front_of_house::hosting，hosting 现在是该范围内的一个有效名称，就像在 crate root 中定义了 hosting 模块一样。
// Paths brought into scope with use also check privacy, like any other paths.
// 与其他路径一样，使用范围内的路径也会检查隐私。

// Note that use only creates the shortcut for the particular scope in which the use occurs.
// 请注意，使用只会为使用发生的特定范围创建快捷方式。
// Listing 7-12 moves the eat_at_restaurant function into a new child module named customer,
// 清单 7-12 将 eat_at_restaurant 函数移动到名为 customer 的新子模块中，
// which is then a different scope than the use statement, so the function body won’t compile:
// 这与 use 语句的作用域不同，因此函数体不会编译：

// Filename: src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
// Listing 7-12: A use statement only applies in the scope it’s in
// 示例 7-12：use 语句仅适用于它所在的范围

// The compiler error shows that the shortcut no longer applies within the customer module:
// 编译器错误表明快捷方式不再适用于客户模块：

// $ cargo build
//    Compiling restaurant v0.1.0 (file:///projects/restaurant)
// error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
//   --> src/lib.rs:11:9
//    |
// 11 |         hosting::add_to_waitlist();
//    |         ^^^^^^^ use of undeclared crate or module `hosting`

// warning: unused import: `crate::front_of_house::hosting`
//  --> src/lib.rs:7:5
//   |
// 7 | use crate::front_of_house::hosting;
//   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//   |
//   = note: `#[warn(unused_imports)]` on by default
//
// For more information about this error, try `rustc --explain E0433`.
// warning: `restaurant` (lib) generated 1 warning
// error: could not compile `restaurant` due to previous error; 1 warning emitted

// Notice there’s also a warning that the use is no longer used in its scope!
// 注意还有一个警告，表示 use 不再在其范围内使用！
// To fix this problem, move the use within the customer module too,
// 要解决此问题，请将 use 也移到 customer 模块中，
// or reference the shortcut in the parent module with super::hosting within the child customer module.
// 或者在子customer模块中 use super::hosting 引用父模块中的快捷方式。

// Creating Idiomatic use Paths
// 创建惯用的使用路径
// In Listing 7-11, you might have wondered why we specified use crate::front_of_house::hosting and
// 在示例 7-11 中，您可能想知道为什么我们指定使用 crate::front_of_house::hosting 和
// then called hosting::add_to_waitlist in eat_at_restaurant rather than specifying the use path all the way out to the add_to_waitlist function to achieve the same result,
// 然后在 eat_at_restaurant 中调用 hosting::add_to_waitlist 而不是指定使用路径一直到 add_to_waitlist 函数以获得相同的结果，
// as in Listing 7-13.
// 如清单 7-13 所示。

// Filename: src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
// Listing 7-13: Bringing the add_to_waitlist function into scope with use, which is unidiomatic
// 示例 7-13：使用 use 将 add_to_waitlist 函数带入作用域，这是不惯用的

// Although both Listing 7-11 and 7-13 accomplish the same task, Listing 7-11 is the idiomatic way to bring a function into scope with use.
// 尽管清单 7-11 和 7-13 完成了相同的任务，但清单 7-11 是将函数带入作用域的惯用方式。
// Bringing the function’s parent module into scope with use means we have to specify the parent module when calling the function.
// 使用 use 将函数的父模块引入作用域意味着我们必须在调用函数时指定父模块。
// Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.
// 在调用函数时指定父模块可以清楚地表明该函数不是本地定义的，同时仍然最大限度地减少了完整路径的重复。
// The code in Listing 7-13 is unclear as to where add_to_waitlist is defined.
// 清单 7-13 中的代码不清楚 add_to_waitlist 的定义位置。

// On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path.
// 另一方面，当使用 use 引入结构、枚举和其他项时，习惯上指定完整路径。
// Listing 7-14 shows the idiomatic way to bring the standard library’s HashMap struct into the scope of a binary crate.
// 清单 7-14 展示了将标准库的 HashMap 结构引入二进制 crate 范围的惯用方法。

// Filename: src/main.rs
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
// Listing 7-14: Bringing HashMap into scope in an idiomatic way
// 示例 7-14：以惯用的方式将 HashMap 引入作用域

// There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.
// 这个惯用语背后没有充分的理由：它只是已经出现的约定，人们已经习惯了以这种方式阅读和编写 Rust 代码。

// The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that.
// 这个习语的例外是，如果我们使用 use 语句将两个同名的项目带入作用域，因为 Rust 不允许这样做。
// Listing 7-15 shows how to bring two Result types into scope that have the same name but different parent modules and how to refer to them.
// 清单 7-15 展示了如何将两个名称相同但父模块不同的 Result 类型放入作用域，以及如何引用它们。

// Filename: src/lib.rs
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
// Listing 7-15: Bringing two types with the same name into the same scope requires using their parent modules.
// 示例 7-15：将两个同名类型放入同一个作用域需要使用它们的父模块。

// As you can see, using the parent modules distinguishes the two Result types.
// 如您所见，使用父模块区分了两种 Result 类型。
// If instead we specified use std::fmt::Result and use std::io::Result,
// 相反，如果我们指定使用 std::fmt::Result 并使用 std::io::Result，
// we’d have two Result types in the same scope and Rust wouldn’t know which one we meant when we used Result.
// 我们在同一个作用域中有两个 Result 类型，当我们使用 Result 时，Rust 不知道我们指的是哪一个。

// Providing New Names with the as Keyword
// 使用 as 关键字提供新名称
// There’s another solution to the problem of bringing two types of the same name into the same scope with use: after the path,
// 对于将两种同名的类型带入同一作用域的问题，还有另一种解决方法：在路径之后，
// we can specify as and a new local name, or alias, for the type.
// 我们可以为该类型指定一个新的本地名称或别名。
// Listing 7-16 shows another way to write the code in Listing 7-15 by renaming one of the two Result types using as.
// 清单 7-16 显示了另一种编写清单 7-15 中代码的方法，方法是使用 as 重命名两个结果类型之一。

// Filename: src/lib.rs
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
// Listing 7-16: Renaming a type when it’s brought into scope with the as keyword
// 示例 7-16：使用 as 关键字重命名一个类型

// In the second use statement, we chose the new name IoResult for the std::io::Result type,
// 在第二个 use 语句中，我们为 std::io::Result 类型选择了新名称 IoResult，
// which won’t conflict with the Result from std::fmt that we’ve also brought into scope.
// 这不会与我们也纳入作用域的 std::fmt 的结果冲突。
// Listing 7-15 and Listing 7-16 are considered idiomatic, so the choice is up to you!
// 清单 7-15 和清单 7-16 被认为是惯用的，所以选择由你决定！

// Re-exporting Names with pub use
// 使用 pub use 重新导出名称
// When we bring a name into scope with the use keyword, the name available in the new scope is private.
// 当我们使用 use 关键字将名称引入作用域时，新作用域中可用的名称是私有的。
// To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use.
// 为了使调用我们代码的代码能够引用该名称，就好像它已在该代码的作用域内定义一样，我们可以结合使用 pub 和 use。
// This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.
// 这种技术被称为重新导出，因为我们将一个项目引入到作用域，同时也让其他人可以将该项目引入他们的作用域。

// Listing 7-17 shows the code in Listing 7-11 with use in the root module changed to pub use.
// 清单 7-17 显示了清单 7-11 中根模块中的 use 更改为 pub use 的代码。

// Filename: src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
// Listing 7-17: Making a name available for any code to use from a new scope with pub use
// 示例 7-17：使用 pub use 使名称可用于任何代码以在新作用域中使用

// Before this change, external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist().
// 在此更改之前，外部代码必须使用路径 restaurant::front_of_house::hosting::add_to_waitlist() 来调用 add_to_waitlist 函数。
// Now that this pub use has re-exported the hosting module from the root module, external code can now use the path restaurant::hosting::add_to_waitlist() instead.
// 现在这个 pub use 已经从根模块中重新导出了托管模块，外部代码现在可以使用路径 restaurant::hosting::add_to_waitlist() 来代替。

// Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain.
// 当您的代码的内部结构与调用您的代码的程序员考虑域的方式不同时，重新导出很有用。
// For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.”
// 例如，在这个餐厅的比喻中，经营餐厅的人会考虑“前台”和“后台”。
// But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. With pub use,
// 但光顾餐厅的顾客可能不会用这些术语来考虑餐厅的各个部分。 随着pub的使用，
// we can write our code with one structure but expose a different structure.
// 我们可以用一种结构编写代码，但公开不同的结构。
// Doing so makes our library well organized for programmers working on the library and programmers calling the library.
// 这样做可以使我们的库为在库上工作的程序员和调用库的程序员组织得很好。
// We’ll look at another example of pub use and how it affects your crate’s documentation in the “Exporting a Convenient Public API with pub use” section of Chapter 14.
// 我们将在第 14 章的“使用 pub use 导出一个方便的公共 API”部分中查看 pub use 的另一个示例以及它如何影响您的 crate 文档。

// Using External Packages
// 使用外部包
// In Chapter 2, we programmed a guessing game project that used an external package called rand to get random numbers.
// 在第 2 章中，我们编写了一个猜谜游戏项目，该项目使用名为 rand 的外部包来获取随机数。
// To use rand in our project, we added this line to Cargo.toml:
// 要在我们的项目中使用 rand，我们将这一行添加到 Cargo.toml：

// Filename: Cargo.toml
// rand = "0.8.3"

// Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from crates.io and make rand available to our project.
// 在 Cargo.toml 中添加 rand 作为依赖项告诉 Cargo 从 crates.io 下载 rand 包和任何依赖项，并使 rand 可用于我们的项目。

// Then, to bring rand definitions into the scope of our package, we added a use line starting with the name of the crate, rand, and listed the items we wanted to bring into scope.
// 然后，为了将 rand 定义引入我们的包的作用域，我们添加了一个以 crate 名称 rand 开头的 use 行，并列出了我们想要引入作用域的项目。
// Recall that in the “Generating a Random Number” section in Chapter 2, we brought the Rng trait into scope and called the rand::thread_rng function:
// 回想一下，在第 2 章的“生成随机数”部分，我们将 Rng 特性引入作用域并调用了 rand::thread_rng 函数：
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
// Members of the Rust community have made many packages available at crates.io,
// Rust 社区的成员在 crates.io 上提供了许多包，
// |- and pulling any of them into your package involves these same steps:
// |- 并将它们中的任何一个拉到你的包中都涉及这些相同的步骤：
// |- listing them in your package’s Cargo.toml file and using use to bring items from their crates into scope.
// |- 将它们列在你的包的 Cargo.toml 文件中，并使用 use 将它们的crates中的items带入作用域。

// Note that the standard std library is also a crate that’s external to our package.
// 请注意，标准 std 库也是我们package外部的create。
// Because the standard library is shipped with the Rust language,
// 因为标准库是随 Rust 语言一起提供的，
// we don’t need to change Cargo.toml to include std.
// 我们不需要更改 Cargo.toml 来包含 std。
// But we do need to refer to it with use to bring items from there into our package’s scope.
// 但我们确实需要使用 use 来引用它，以便将项目从那里带入我们的包范围。
// For example, with HashMap we would use this line:
// 例如，对于 HashMap，我们将使用这一行：

use std::collections::HashMap;

// This is an absolute path starting with std, the name of the standard library crate.
// 这是一个以标准库 crate 的名称 std 开头的绝对路径。

// Using Nested Paths to Clean Up Large use Lists
// 使用嵌套路径清理大量使用列表
// If we’re using multiple items defined in the same crate or same module,
// 如果我们使用同一个 crate 或同一个模块中定义的多个项目，
// listing each item on its own line can take up a lot of vertical space in our files.
// 将每个项目单独列在一行中会占用我们文件中的大量垂直空间。
// For example, these two use statements we had in the Guessing Game in Listing 2-4 bring items from std into scope:
// 例如，我们在清单 2-4 的猜谜游戏中使用的这两个 use 语句将 std 中的项目带入作用域：

// Filename: src/main.rs
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--

// Instead, we can use nested paths to bring the same items into scope in one line.
// 相反，我们可以使用嵌套路径在一行中将相同的项目引入范围。
// We do this by specifying the common part of the path, followed by two colons,
// 我们通过指定路径的公共部分来执行此操作，后跟两个冒号，
// |- and then curly brackets around a list of the parts of the paths that differ, as shown in Listing 7-18.
// |- 然后用大括号括起不同路径部分的列表，如清单 7-18 所示。

// Filename: src/main.rs
// --snip--
use std::{cmp::Ordering, io};
// --snip--
// Listing 7-18: Specifying a nested path to bring multiple items with the same prefix into scope
// 示例 7-18：指定嵌套路径以将具有相同前缀的多个项放入作用域

// In bigger programs, bringing many items into scope from the same crate or module using nested paths can reduce the number of separate use statements needed by a lot!
// 在更大的程序中，使用嵌套路径将同一个 crate 或模块中的许多项目引入作用域可以减少大量所需的单独 use 语句的数量！

// We can use a nested path at any level in a path, which is useful when combining two use statements that share a subpath.
// 我们可以在路径中的任何级别使用嵌套路径，这在组合两个共享子路径的 use 语句时很有用。
// For example, Listing 7-19 shows two use statements: one that brings std::io into scope and one that brings std::io::Write into scope.
// 例如，清单 7-19 显示了两个 use 语句：一个将 std::io 带入作用域，一个将 std::io::Write 带入作用域。

// Filename: src/lib.rs
use std::io;
use std::io::Write;
// Listing 7-19: Two use statements where one is a subpath of the other
// 示例 7-19：两个 use 语句，其中一个是另一个的子路径

// The common part of these two paths is std::io, and that’s the complete first path.
// 这两条路径的共同部分是std::io，也就是完整的第一条路径。
// To merge these two paths into one use statement, we can use self in the nested path, as shown in Listing 7-20.
// 要将这两个路径合并为一个 use 语句，我们可以在嵌套路径中使用 self，如清单 7-20 所示。

// Filename: src/lib.rs
use std::io::{self, Write};
// Listing 7-20: Combining the paths in Listing 7-19 into one use statement
// 示例 7-20：将示例 7-19 中的路径组合成一个 use 语句

// This line brings std::io and std::io::Write into scope.
// 这一行将 std::io 和 std::io::Write 引入作用域。

// The Glob Operator
// 全局操作符
// If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:
// 如果我们想将路径中定义的所有公共项（public items）都纳入作用域，我们可以指定该路径后跟 * glob 运算符：

use std::collections::*;

// This use statement brings all public items defined in std::collections into the current scope.
// 此 use 语句将 std::collections 中定义的所有公共项目（public items）带入当前作用域。
// Be careful when using the glob operator!
// 使用 glob 运算符时要小心！
// Glob can make it harder to tell what names are in scope and where a name used in your program was defined.
// Glob 让我们更难推导作用域内使用的名称和它们的定义位置。

// The glob operator is often used when testing to bring everything under test into the tests module;
// 测试时经常使用glob操作符，将所有被测试的都带入测试模块；
// we’ll talk about that in the “How to Write Tests” section in Chapter 11.
// 我们将在第 11 章的“如何编写测试”部分讨论这一点。
// The glob operator is also sometimes used as part of the prelude pattern:
// glob 运算符有时也用作前奏（prelude）模式的一部分：
// |- see the standard library documentation for more information on that pattern.
// |- 有关该模式的更多信息，请参阅标准库文档。

// Separating Modules into Different Files
// 将模块分离到不同的文件中
// So far, all the examples in this chapter defined multiple modules in one file.
// 到目前为止，本章中的所有示例都在一个文件中定义了多个模块。
// When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.
// 当模块变大时，您可能希望将它们的定义移动到单独的文件中，以使代码更易于浏览。

// For example, let’s start from the code in Listing 7-17 that had multiple restaurant modules.
// 例如，让我们从清单 7-17 中有多个餐厅模块的代码开始。
// We’ll extract modules into files instead of having all the modules defined in the crate root file.
// 我们将模块提取到文件中，而不是将所有模块定义在 crate 根文件中。
// In this case, the crate root file is src/lib.rs, but this procedure also works with binary crates whose crate root file is src/main.rs.
// 在这种情况下，crate 根文件是 src/lib.rs，但是这个过程也适用于 crate 根文件是 src/main.rs 的二进制 crate。

// First, we’ll extract the front_of_house module to its own file.
// 首先，我们将 front_of_house 模块提取到它自己的文件中。
// Remove the code inside the curly brackets for the front_of_house module, leaving only the mod front_of_house;
// 去掉front_of_house模块大括号内的代码，只留下mod front_of_house；
// declaration, so that src/lib.rs contains the code shown in Listing 7-21.
// 声明，以便 src/lib.rs 包含清单 7-21 中所示的代码。
// Note that this won’t compile until we create the src/front_of_house.rs file in Listing 7-22.
// 请注意，在我们创建示例 7-22 中的 src/front_of_house.rs 文件之前，这不会编译。

// Filename: src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
// Listing 7-21: Declaring the front_of_house module whose body will be in src/front_of_house.rs
// 示例 7-21：声明主体位于 src/front_of_house.rs 中的 front_of_house 模块

// Next, place the code that was in the curly brackets into a new file named src/front_of_house.rs, as shown in Listing 7-22.
// 接下来，将大括号中的代码放入名为 src/front_of_house.rs 的新文件中，如清单 7-22 所示。
// The compiler knows to look in this file because it came across the module declaration in the crate root with the name front_of_house.
// 编译器知道要查看这个文件，因为它在 crate root 中遇到了名称为 front_of_house 的模块声明。

// Filename: src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
// Listing 7-22: Definitions inside the front_of_house module in src/front_of_house.rs
// 示例 7-22：src/front_of_house.rs 中 front_of_house 模块的定义

// Note that you only need to load a file using a mod declaration once in your module tree.
// 请注意，您只需要在模块树中使用 mod 声明加载一次文件。
// Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the mod statement),
// 一旦编译器知道文件是项目的一部分（并且知道代码在模块树中的位置，因为你放置了 mod 语句），
// |- other files in your project should refer to the loaded file’s code using a path to where it was declared,
// |- 项目中的其他文件应该使用声明位置的路径来引用加载文件的代码，
// |- as covered in the “Paths for Referring to an Item in the Module Tree” section.
// |- 如“引用模块树中项目的路径”部分所述。
// In other words, mod is not an “include” operation that you may have seen in other programming languages.
// 换句话说，mod 不是您可能在其他编程语言中看到的“include”操作。

// Next, we’ll extract the hosting module to its own file.
// 接下来，我们将 hosting 模块提取到它自己的文件中。
// The process is a bit different because hosting is a child module of front_of_house, not of the root module.
// 这个过程有点不同，因为 hosting 是 front_of_house 的子模块，而不是根模块。
// We’ll place the file for hosting in a new directory that will be named for its ancestors in the module tree, in this case src/front_of_house/.
// 我们将 hosting 文件放置在一个新目录中，该目录将以其在模块树中的祖先命名，在本例中为 src/front_of_house/。

// To start moving hosting, we change src/front_of_house.rs to contain only the declaration of the hosting module:
// 要开始移动 hosting，我们将 src/front_of_house.rs 更改为仅包含 hosting 模块的声明：

// Filename: src/front_of_house.rs
pub mod hosting;

// Then we create a src/front_of_house directory and a file hosting.rs to contain the definitions made in the hosting module:
// 然后我们创建一个 src/front_of_house 目录和一个文件 hosting.rs 来包含在 hosting 模块中所做的定义：

// Filename: src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}

// If we instead put hosting.rs in the src directory, the compiler would expect the hosting.rs code to be in a hosting module declared in the crate root,
// 如果我们改为将 hosting.rs 放在 src 目录中，编译器会期望 hosting.rs 代码位于 crate root 中声明的托管模块中，
// and not declared as a child of the front_of_house module.
// 并且未声明为 front_of_house 模块的子项。
// The compiler’s rules for which files to check for which modules’ code means the directories and files more closely match the module tree.
// 编译器针对哪些文件检查哪些模块代码的规则意味着目录和文件更接近模块树。

// Alternate File Paths
// 备用文件路径
// So far we’ve covered the most idiomatic file paths the Rust compiler uses, but Rust also supports an older style of file path.
// 到目前为止，我们已经介绍了 Rust 编译器使用的最惯用的文件路径，但 Rust 还支持旧式的文件路径。
// For a module named front_of_house declared in the crate root, the compiler will look for the module’s code in:
// 对于在 crate root 中声明的名为 front_of_house 的模块，编译器将在以下位置查找该模块的代码：

// src/front_of_house.rs (what we covered)（我们涵盖的内容）
// src/front_of_house/mod.rs (older style, still supported path)（旧样式，仍然支持路径）

// For a module named hosting that is a submodule of front_of_house, the compiler will look for the module’s code in:
// 对于名为 hosting 的模块，它是 front_of_house 的子模块，编译器将在以下位置查找该模块的代码：

// src/front_of_house/hosting.rs (what we covered)（我们涵盖的内容）
// src/front_of_house/hosting/mod.rs (older style, still supported path)（旧样式，仍然支持路径）

// If you use both styles for the same module, you’ll get a compiler error.
// 如果你对同一个模块使用这两种样式，你会得到一个编译器错误。
// Using a mix of both styles for different modules in the same project is allowed, but might be confusing for people navigating your project.
// 允许在同一个项目中为不同的模块混合使用两种样式，但可能会让浏览您的项目的人感到困惑。

// The main downside to the style that uses files named mod.rs is that your project can end up with many files named mod.rs,
// 使用名为 mod.rs 的文件的样式的主要缺点是您的项目最终可能会包含许多名为 mod.rs 的文件，
// which can get confusing when you have them open in your editor at the same time.
// 当你在你的编辑器中同时打开它们时，这会让人感到困惑。

// We’ve moved each module’s code to a separate file, and the module tree remains the same.
// 我们将每个模块的代码移动到一个单独的文件中，模块树保持不变。
// The function calls in eat_at_restaurant will work without any modification, even though the definitions live in different files.
// eat_at_restaurant 中的函数调用无需任何修改即可工作，即使定义位于不同的文件中。
// This technique lets you move modules to new files as they grow in size.
// 这种技术可以让您将模块移动到新文件中，因为它们的大小会增加。

// Note that the pub use crate::front_of_house::hosting statement in src/lib.rs also hasn’t changed,
// 请注意，src/lib.rs 中的 pub use crate::front_of_house::hosting 语句也没有改变，
// |- nor does use have any impact on what files are compiled as part of the crate.
// |- use 对哪些文件被编译为 crate 的一部分也没有任何影响。
// The mod keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.
// mod 关键字声明模块，Rust 在与模块同名的文件中查找进入该模块的代码。

mod front_of_house;

pub use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}

// Summary
// 概括
// Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module.
// Rust 允许您将一个package拆分为多个crate，并将一个create拆分为多个module，这样您就可以从另一个module引用一个module中定义的items。
// You can do this by specifying absolute or relative paths.
// 你可以通过指定绝对或相对路径来做到这一点。
// These paths can be brought into scope with a use statement so you can use a shorter path for multiple uses of the item in that scope.
// 可以使用 use 语句将这些路径引入作用域，因此您可以使用较短的路径多次使用该作用域中的item。
// Module code is private by default, but you can make definitions public by adding the pub keyword.
// 模块代码（Module code）默认是私有的，但是你可以通过添加 pub 关键字使定义公开。

// In the next chapter, we’ll look at some collection data structures in the standard library that you can use in your neatly organized code.
// 在下一章中，我们将了解标准库中的一些集合数据结构，您可以在组织整齐的代码中使用它们。
