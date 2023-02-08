// Functional Language Features: Iterators and Closures
// 函数式语言特性：iterators 和 closures
// Rust’s design has taken inspiration from many existing languages and techniques, and one significant influence is functional programming.
// Rust 的设计从许多现有的语言和技术中汲取了灵感，其中一个重要的影响是函数式编程。
// Programming in a functional style often includes using functions as values by passing them in arguments,
// 以函数式风格编程通常包括通过将函数作为参数传递给它们来使用函数作为值，
// |- returning them from other functions, assigning them to variables for later execution, and so forth.
// |- 从其他函数返回它们，将它们分配给变量供以后执行，等等。

// In this chapter,
// 在这一章当中，
// |- we won’t debate the issue of what functional programming is or isn’t but will instead discuss some features of Rust that are similar to features in many languages often referred to as functional.
// |- 我们不会讨论函数式编程是什么或不是什么的问题，而是讨论 Rust 的一些特性，这些特性类似于许多语言中通常被称为函数式的特性。

// More specifically, we’ll cover:
// 更具体地说，我们将介绍：
// - Closures, a function-like construct you can store in a variable
// - closures，一种可以存储在变量中的类似函数的结构
// - Iterators, a way of processing a series of elements
// - iterators，一种处理一系列元素的方法
// - How to use closures and iterators to improve the I/O project in Chapter 12
// - 如何使用 closures 和 iterators 改进第 12 章中的 I/O 项目
// - The performance of closures and iterators (Spoiler alert: they’re faster than you might think!)
// - closures 和 iterators 的性能（剧透警告：它们比你想象的要快！）

// We’ve already covered some other Rust features, such as pattern matching and enums, that are also influenced by the functional style.
// 我们已经介绍了其他一些 Rust 特性，例如模式匹配和枚举，它们也受函数式风格的影响。
// Because mastering closures and iterators is an important part of writing idiomatic, fast Rust code, we’ll devote this entire chapter to them.
// 因为掌握 closures 和 iterators 是编写惯用的、快速的 Rust 代码的重要部分，所以我们将用整章来介绍它们。

// `Closures`: Anonymous Functions that Capture Their Environment
// `Closures`：捕获其环境的匿名函数
// Rust’s `closures` are anonymous functions you can save in a variable or pass as arguments to other functions.
// Rust 的 `closures` 是匿名函数，您可以将其保存在变量中或作为参数传递给其他函数。
// You can create the `closure` in one place and then call the `closure` elsewhere to evaluate it in a different context.
// 您可以在一个地方创建 `closure`，然后在别处调用 `closure` 以在不同的上下文中评估它。
// Unlike functions, `closures` can capture values from the scope in which they’re defined.
// 与函数不同，`closures` 可以从定义它们的范围内捕获值。
// We’ll demonstrate how these `closure` features allow for code reuse and behavior customization.
// 我们将演示这些 `closure` 特性如何允许代码重用和行为定制。

// Capturing the Environment with `Closures`
// 使用 `Closures` 捕获环境
// We’ll first examine how we can use `closures` to capture values from the environment they’re defined in for later use.
// 我们将首先研究如何使用 `closures` 从它们定义的环境中捕获值以供以后使用。
// Here’s the scenario: Every so often, our t-shirt company gives away an exclusive, limited-edition shirt to someone on our mailing list as a promotion.
// 场景是这样的：每隔一段时间，我们的 T 恤公司就会向我们邮寄名单上的某个人赠送一件独家限量版 T 恤作为促销活动。
// People on the mailing list can optionally add their favorite color to their profile.
// 邮件列表中的人可以选择将他们喜欢的颜色添加到他们的个人资料中。
// If the person chosen for a free shirt has their favorite color set, they get that color shirt.
// 如果选择免费衬衫的人有他们最喜欢的颜色集，他们就会得到那件颜色的衬衫。
// If the person hasn’t specified a favorite color, they get whatever color the company currently has the most of.
// 如果此人没有指定最喜欢的颜色，他们将获得公司目前拥有最多的颜色。

// There are many ways to implement this.
// 有很多方法可以实现这个。
// For this example, we’re going to use an enum called `ShirtColor` that has the variants `Red` and `Blue` (limiting the number of colors available for simplicity).
// 对于这个例子，我们将使用一个名为 `ShirtColor` 的枚举，它有 `Red` 和 `Blue` 变体（为简单起见限制了可用颜色的数量）。
// We represent the company’s inventory with an `Inventory` struct that has a field named `shirts` that contains a Vec<ShirtColor> representing the shirt colors currently in stock.
// 我们用 `Inventory` 结构表示公司的库存，该结构有一个名为 `shirts` 的字段，其中包含表示当前库存衬衫颜色的 Vec<ShirtColor>。
// The method `giveaway` defined on `Inventory` gets the optional shirt color preference of the free shirt winner, and returns the shirt color the person will get.
// 在 `Inventory` 上定义的 `giveaway` 方法获取免费衬衫获得者的可选衬衫颜色偏好，并返回此人将获得的衬衫颜色。
// This setup is shown in Listing 13-1:
// 此设置如清单 13-1 所示：

// Filename: src/main.rs

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

// Listing 13-1: Shirt company giveaway situation
// 示例 13-1：衬衫公司的赠品情况

// The `store` defined in `main` has two blue shirts and one red shirt remaining to distribute for this limited-edition promotion.
// 在 `main` 中定义的 `store` 有两件蓝色衬衫和一件红色衬衫剩余用于分发此限量版促销活动。
// We call the `giveaway` method for a user with a preference for a red shirt and a user without any preference.
// 我们为偏好红色衬衫的用户和没有任何偏好的用户调用 `giveaway` 方法。

// Again, this code could be implemented in many ways, and here, to focus on `closures`,
// 同样，这段代码可以通过多种方式实现，这里，重点关注 `closures`，
// |- we’ve stuck to concepts you’ve already learned except for the body of the `giveaway` method that uses a `closure`.
// |- 除了使用 `closure` 的 `giveaway` 方法的主体外，我们坚持您已经学过的概念。
// In the `giveaway` method, we get the user preference as a parameter of type Option<ShirtColor> and call the `unwrap_or_else` method on user_preference.
// 在 `giveaway` 方法中，我们将用户偏好作为 Option<ShirtColor> 类型的参数获取，并在 user_preference 上调用 `unwrap_or_else` 方法。
// The `unwrap_or_else` method on Option<T> is defined by the standard library.
// Option<T> 上的 `unwrap_or_else` 方法由标准库定义。
// It takes one argument: a `closure` without any arguments that returns a value T (the same type stored in the `Some` variant of the Option<T>, in this case ShirtColor).
// 它接受一个参数：一个没有任何参数的 `closure`，它返回一个值 T（与存储在 Option<T> 的 `Some` 变体中的类型相同，在本例中为 ShirtColor）。
// If the Option<T> is the `Some` variant, `unwrap_or_else` returns the value from within the `Some`.
// 如果 Option<T> 是 `Some` 变体，`unwrap_or_else` 返回 `Some` 中的值。
// If the Option<T> is the `None` variant, `unwrap_or_else` calls the `closure` and returns the value returned by the `closure`.
// 如果 Option<T> 是 `None` 变体，`unwrap_or_else` 调用 `closure` 并返回 `closure` 返回的值。

// We specify the `closure` expression `|| self.most_stocked()` as the argument to `unwrap_or_else`.
// 我们指定 `closure` 表达式 `|| self.most_stocked()` 作为 `unwrap_or_else` 的参数。
// This is a `closure` that takes no parameters itself (if the `closure` had parameters, they would appear between the two vertical bars).
// 这是一个本身不带参数的 `closure`（如果 `closure` 有参数，它们将出现在两个竖线之间）。
// The body of the `closure` calls `self.most_stocked()`. We’re defining the `closure` here, and the implementation of `unwrap_or_else` will evaluate the `closure` later if the result is needed.
// `closure` 的主体调用 `self.most_stocked()`。 我们在这里定义了“closure”，如果需要结果，“unwrap_or_else”的实现将稍后评估“closure”。

// Running this code prints:

// $ cargo run
//    Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.27s
//      Running `target/debug/shirt-company`
// The user with preference Some(Red) gets Red
// The user with preference None gets Blue

// One interesting aspect here is that we’ve passed a `closure` that calls `self.most_stocked()` on the current `Inventory` instance.
// 这里有一个有趣的方面是我们传递了一个在当前 `Inventory` 实例上调用 `self.most_stocked()` 的 `closure`。
// The standard library didn’t need to know anything about the `Inventory` or `ShirtColor` types we defined, or the logic we want to use in this scenario.
// 标准库不需要知道关于我们定义的 `Inventory` 或 `ShirtColor` 类型的任何信息，或者我们想要在这种情况下使用的逻辑。
// The `closure` captures an immutable reference to the `self` `Inventory` instance and passes it with the code we specify to the `unwrap_or_else` method.
// `closure` 捕获对 `self` `Inventory` 实例的不可变引用，并将其与我们指定的代码一起传递给 `unwrap_or_else` 方法。
// Functions, on the other hand, are not able to capture their environment in this way.
// 另一方面，函数无法以这种方式捕获其环境。

// `Closure` Type Inference and Annotation
// `Closure` 类型推断和注解
// There are more differences between functions and `closures`.
// 函数和 `closures` 之间有更多区别。
// `Closures` don’t usually require you to annotate the types of the parameters or the return value like fn functions do.
// `Closures` 通常不需要像 fn 函数那样注释参数或返回值的类型。
// Type annotations are required on functions because the types are part of an explicit interface exposed to your users.
// 函数需要类型注释，因为类型是暴露给用户的显式接口的一部分。
// Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns.
// 严格定义此接口对于确保每个人都同意函数使用和返回的值类型很重要。
// `Closures`, on the other hand, aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.
// 另一方面，`Closures` 不会像这样在暴露的接口中使用：它们存储在变量中并且在使用时没有命名它们并将它们暴露给我们库的用户。

// `Closures` are typically short and relevant only within a narrow context rather than in any arbitrary scenario.
// `Closures` 通常很短，并且只在狭窄的上下文中相关，而不是在任何任意场景中相关。
// Within these limited contexts, the compiler can infer the types of the parameters and the return type,
// 在这些有限的上下文中，编译器可以推断出参数的类型和返回类型，
// |- similar to how it’s able to infer the types of most variables (there are rare cases where the compiler needs `closure` type annotations too).
// |- 类似于它能够推断大多数变量类型的方式（在极少数情况下，编译器也需要 `closure` 类型注释）。

// As with variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary.
// 与变量一样，如果我们想增加明确性和清晰度，但代价是比绝对必要的更冗长，我们可以添加类型注释。
// Annotating the types for a `closure` would look like the definition shown in Listing 13-2.
// 注释 `closure` 的类型看起来像清单 13-2 中所示的定义。
// In this example, we’re defining a `closure` and storing it in a variable rather than defining the `closure` in the spot we pass it as an argument as we did in Listing 13-1.
// 在此示例中，我们定义了一个 `closure` 并将其存储在一个变量中，而不是像示例 13-1 中那样在我们将其作为参数传递的位置定义 `closure`。

// Filename: src/main.rs

let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};

// Listing 13-2: Adding optional type annotations of the parameter and return value types in the `closure`
// 示例 13-2：在 `closure` 中添加参数和返回值类型的可选类型注释

// With type annotations added, the syntax of `closures` looks more similar to the syntax of functions.
// 添加类型注解后，`closures` 的语法看起来更像函数的语法。
// Here we define a function that adds 1 to its parameter and a `closure` that has the same behavior, for comparison.
// 这里我们定义了一个将参数加 1 的函数和一个具有相同行为的 `closure`，用于比较。
// We’ve added some spaces to line up the relevant parts.
// 我们添加了一些空格来排列相关部分。
// This illustrates how `closure` syntax is similar to function syntax except for the use of pipes and the amount of syntax that is optional:
// 这说明了 `closure` 语法与函数语法的相似之处，除了管道的使用和可选语法的数量：

fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;

// The first line shows a function definition, and the second line shows a fully annotated `closure` definition.
// 第一行显示一个函数定义，第二行显示一个完整注释的 `closure` 定义。
// In the third line, we remove the type annotations from the `closure` definition.
// 在第三行，我们从 `closure` 定义中删除了类型注释。
// In the fourth line, we remove the brackets, which are optional because the `closure` body has only one expression.
// 在第四行中，我们删除了括号，这是可选的，因为 `closure` 主体只有一个表达式。
// These are all valid definitions that will produce the same behavior when they’re called.
// 这些都是有效的定义，它们在被调用时会产生相同的行为。
// The `add_one_v3` and `add_one_v4` lines require the `closures` to be evaluated to be able to compile because the types will be inferred from their usage.
// `add_one_v3` 和 `add_one_v4` 行需要评估 `closures` 才能编译，因为类型将从它们的用法中推断出来。
// This is similar to `let v = Vec::new();` needing either type annotations or values of some type to be inserted into the `Vec` for Rust to be able to infer the type.
// 这类似于 `let v = Vec::new();` 需要将类型注释或某种类型的值插入到 `Vec` 中，以便 Rust 能够推断类型。

// For `closure` definitions, the compiler will infer one concrete type for each of their parameters and for their return value.
// 对于 `closure` 定义，编译器将为它们的每个参数和返回值推断出一种具体类型。
// For instance, Listing 13-3 shows the definition of a short `closure` that just returns the value it receives as a parameter.
// 例如，清单 13-3 显示了一个简短的 `closure` 的定义，它只返回它作为参数接收的值。
// This `closure` isn’t very useful except for the purposes of this example.
// 这个 `closure` 不是很有用，除了这个例子的目的。
// Note that we haven’t added any type annotations to the definition.
// 请注意，我们没有在定义中添加任何类型注释。
// Because there are no type annotations, we can call the `closure` with any type, which we’ve done here with `String` the first time.
// 因为没有类型注释，我们可以用任何类型调用 `closure`，这是我们第一次使用 `String` 完成的。
// If we then try to call `example_closure` with an integer, we’ll get an error.
// 如果我们随后尝试用整数调用 `example_closure`，我们会得到一个错误。

// Filename: src/main.rs

let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);

// Listing 13-3: Attempting to call a `closure` whose types are inferred with two different types
// 示例 13-3：尝试调用其类型由两种不同类型推断的 `closure`

// The compiler gives us this error:
// 编译器给我们这个错误：

// $ cargo run
//    Compiling closure-example v0.1.0 (file:///projects/closure-example)
// error[E0308]: mismatched types
//  --> src/main.rs:5:29
//   |
// 5 |     let n = example_closure(5);
//   |                             ^- help: try using a conversion method: `.to_string()`
//   |                             |
//   |                             expected struct `String`, found integer
//
// For more information about this error, try `rustc --explain E0308`.
// error: could not compile `closure-example` due to previous error

// The first time we call `example_closure` with the `String` value, the compiler infers the type of x and the return type of the `closure` to be `String`.
// 我们第一次使用 `String` 值调用 `example_closure` 时，编译器推断 x 的类型和 `closure` 的返回类型为 `String`。
// Those types are then locked into the `closure` in `example_closure`, and we get a type error when we next try to use a different type with the same `closure`.
// 这些类型随后被锁定在 `example_closure` 中的 `closure` 中，当我们下次尝试使用具有相同 `closure` 的不同类型时，我们会收到类型错误。

// Capturing References or Moving `Ownership`
// 捕获引用或移动 `Ownership`
// `Closures` can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter:
// `Closures` 可以通过三种方式从它们的环境中捕获值，这直接映射到函数可以采用参数的三种方式：
// |- borrowing immutably, borrowing mutably, and taking `ownership`.
// |- 不可变地借用，可变地借用，并取得 `ownership`。
// The `closure` will decide which of these to use based on what the body of the function does with the captured values.
// `closure` 将根据函数体对捕获的值执行的操作来决定使用其中的哪些。

// In Listing 13-4, we define a `closure` that captures an immutable reference to the vector named list because it only needs an immutable reference to print the value:
// 在清单 13-4 中，我们定义了一个 `closure` 来捕获对名为 list 的向量的不可变引用，因为它只需要一个不可变引用来打印值：

// Filename: src/main.rs

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

// Listing 13-4: Defining and calling a `closure` that captures an immutable reference
// 示例 13-4：定义并调用捕获不可变引用的“closure”

// This example also illustrates that a variable can bind to a `closure` definition,
// 这个例子也说明了一个变量可以绑定到一个 `closure` 定义，
// |- and we can later call the `closure` by using the variable name and parentheses as if the variable name were a function name.
// |- 我们稍后可以通过使用变量名和括号来调用 `closure`，就好像变量名是函数名一样。

// Because we can have multiple immutable references to `list` at the same time,
// |- `list` is still accessible from the code before the `closure` definition,
// |- after the `closure` definition but before the `closure` is called,
// |- and after the `closure` is called. This code compiles, runs, and prints:
// 因为我们可以同时拥有对 list 的多个不可变引用，
// |- 所以仍然可以从 `closure` 定义之前的代码、`closure` 定义之后但调用 `closure` 之前以及之后的代码访问 `list` `closure` 被调用。
// |- 此代码编译、运行并打印：

// $ cargo run
//    Compiling closure-example v0.1.0 (file:///projects/closure-example)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.43s
//      Running `target/debug/closure-example`
// Before defining closure: [1, 2, 3]
// Before calling closure: [1, 2, 3]
// From closure: [1, 2, 3]
// After calling closure: [1, 2, 3]

// Next, in Listing 13-5, we change the `closure` body so that it adds an element to the list vector.
// 接下来，在示例 13-5 中，我们更改了 `closure` 主体，以便它向列表向量添加一个元素。
// The `closure` now captures a mutable reference:
// `closure` 现在捕获一个可变引用：

// Filename: src/main.rs

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

// Listing 13-5: Defining and calling a `closure` that captures a mutable reference
// 示例 13-5：定义和调用捕获可变引用的 closure

// This code compiles, runs, and prints:
// 此代码编译、运行并打印：

// $ cargo run
//    Compiling closure-example v0.1.0 (file:///projects/closure-example)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.43s
//      Running `target/debug/closure-example`
// Before defining closure: [1, 2, 3]
// After calling closure: [1, 2, 3, 7]

// Note that there’s no longer a `println!` between the definition and the call of the `borrows_mutably` `closure`:
// 请注意，在定义和调用 `borrows_mutably` `closure` 之间不再有 `println!`：
// |- when `borrows_mutably` is defined, it captures a mutable reference to `list`.
// |- 定义 `borrows_mutably` 时，它会捕获对 `list` 的可变引用。
// We don’t use the `closure` again after the `closure` is called, so the mutable borrow ends.
// 在调用 `closure` 之后，我们不再使用 `closure`，因此可变借用结束。
// Between the `closure` definition and the `closure` call, an immutable borrow to print isn’t allowed because no other borrows are allowed when there’s a mutable borrow.
// 在 `closure` 定义和 `closure` 调用之间，不允许对 print 进行不可变借用，因为当存在可变借用时不允许其他借用。
// Try adding a `println!` there to see what error message you get!
// 尝试在那里添加一个 `println!` 看看你得到了什么错误信息！

// If you want to force the `closure` to take ownership of the values it uses in the environment even though the body of the `closure` doesn’t strictly need ownership,
// 如果你想强制 `closure` 获得它在环境中使用的值的所有权，即使 `closure` 的主体并不严格需要所有权，
// |- you can use the `move` keyword before the parameter list.
// |- 你可以在参数列表前使用 `move` 关键字。

// This technique is mostly useful when passing a `closure` to a new thread to `move` the data so that it’s owned by the new thread.
// 当将 `closure` 传递给新线程以 `move` 数据使其归新线程所有时，此技术最有用。
// We’ll discuss threads and why you would want to use them in detail in Chapter 16 when we talk about concurrency,
// 我们将在第 16 章讨论并发时详细讨论线程以及为什么要使用它们，
// |- but for now, let’s briefly explore spawning a new thread using a `closure` that needs the `move` keyword.
// |- 但现在，让我们简要地探索使用需要 `move` 关键字的 `closure` 生成新线程。
// Listing 13-6 shows Listing 13-4 modified to print the vector in a new thread rather than in the main thread:
// 清单 13-6 显示修改后的清单 13-4 以在新线程而不是主线程中打印向量：

// Filename: src/main.rs

use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

// Listing 13-6: Using `move` to force the `closure` for the thread to take `ownership` of `list`
// 示例 13-6：使用 `move` 强制线程的 `closure` 获得 `list` 的 `ownership`

// We spawn a new thread, giving the thread a `closure` to run as an argument.
// 我们生成一个新线程，为该线程提供一个 `closure` 作为参数运行。
// The `closure` body prints out the `list`. In Listing 13-4, the `closure` only captured `list` using an immutable reference because that's the least amount of access to `list` needed to print it.
// `closure` 主体打印出 `list`。 在示例 13-4 中，closure 仅使用不可变引用捕获 list，因为这是打印它所需的对 list 的最少访问量。
// In this example, even though the `closure` body still only needs an immutable reference,
// 在此示例中，即使 `closure` 主体仍然只需要一个不可变引用，
// |- we need to specify that `list` should be moved into the `closure` by putting the `move` keyword at the beginning of the `closure` definition.
// |- 我们需要通过将 `move` 关键字放在 `closure` 定义的开头来指定 `list` 应该移动到 `closure` 中。
// The new thread might finish before the rest of the main thread finishes, or the main thread might finish first.
// 新线程可能在主线程的其余部分完成之前完成，或者主线程可能先完成。
// If the main thread maintained `ownership` of `list` but ended before the new thread did and dropped `list`, the immutable reference in the thread would be invalid.
// 如果主线程保持 `list` 的 `ownership` 但在新线程执行并删除 `list` 之前结束，则线程中的不可变引用将无效。
// Therefore, the compiler requires that `list` be moved into the `closure` given to the new thread so the reference will be valid.
// 因此，编译器要求将 `list` 移动到提供给新线程的 `closure` 中，以便引用有效。
// Try removing the `move` keyword or using `list` in the main thread after the `closure` is defined to see what compiler errors you get!
// 在定义了 `closure` 之后，尝试在主线程中删除 `move` 关键字或使用 `list` 以查看您得到了哪些编译器错误！

// Moving Captured Values Out of `Closures` and the `Fn` Traits
// 将捕获的值移出 `Closures` 和 `Fn` 特征
// Once a `closure` has captured a reference or captured `ownership` of a value from the environment where the `closure` is defined
// 一旦 `closure` 从定义 `closure` 的环境中捕获了引用或捕获了值的 `ownership`
// |- (thus affecting what, if anything, is moved into the `closure`),
// |-（因此影响什么，如果有的话，被移入 `closure`），
// |- the code in the body of the `closure` defines what happens to the references or values when the `closure` is evaluated later
// |- `closure` 主体中的代码定义了稍后评估 `closure` 时引用或值会发生什么
// |- (thus affecting what, if anything, is moved out of the `closure`).
// |- （因此影响什么，如果有的话，被移出 `closure`）。
// A `closure` body can do any of the following: move a captured value out of the `closure`, mutate the captured value, neither move nor mutate the value,
// `closure` 主体可以执行以下任何操作：将捕获的值移出 `closure`，改变捕获的值，既不移动也不改变值，
// |- or capture nothing from the environment to begin with.
// |- 或者一开始就从环境中捕获任何东西。

// The way a `closure` captures and handles values from the environment affects which traits the `closure` implements,
// `closure` 从环境中捕获和处理值的方式会影响 `closure` 实现的特征，
// |- and traits are how functions and structs can specify what kinds of `closures` they can use.
// |- 特征是函数和结构如何指定它们可以使用的“closures”类型。
// `Closures` will automatically implement one, two, or all three of these `Fn` traits, in an additive fashion, depending on how the `closure’s` body handles the values:
// `Closures` 将自动实现这些 `Fn` 特性中的一个、两个或所有三个，以一种附加的方式，具体取决于 `closures` 主体如何处理值：

// `FnOnce` applies to `closures` that can be called once.
// `FnOnce` 适用于只能调用一次的 `closures`。
// All `closures` implement at least this trait, because all `closures` can be called.
// 所有的 `closures` 都至少实现了这个特性，因为所有的 `closures` 都可以被调用。
// A `closure` that moves captured values out of its body will only implement `FnOnce` and none of the other `Fn` traits, because it can only be called once.
// 将捕获的值移出其主体的 `closure` 将仅实现 `FnOnce` 而不会实现任何其他 `Fn` 特征，因为它只能被调用一次。
// `FnMut` applies to `closures` that don’t move captured values out of their body, but that might mutate the captured values.
// `FnMut` 适用于不会将捕获的值移出其主体但可能会改变捕获的值的 `closures`。
// These `closures` can be called more than once.
// 这些 `closures` 可以被多次调用。
// `Fn` applies to `closures` that don’t move captured values out of their body and that don’t mutate captured values, as well as `closures` that capture nothing from their environment.
// `Fn` 适用于不会将捕获的值移出其主体且不会改变捕获的值的 `closures`，以及不会从其环境中捕获任何内容的 `closures`。
// These `closures` can be called more than once without mutating their environment, which is important in cases such as calling a `closure` multiple times concurrently.
// 这些 `closures` 可以在不改变环境的情况下被多次调用，这在诸如同时多次调用 `closures` 的情况下很重要。

// Let’s look at the definition of the `unwrap_or_else` method on Option<T> that we used in Listing 13-1:
// 让我们看看我们在示例 13-1 中使用的 Option<T> 上的 `unwrap_or_else` 方法的定义：

impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}

// Recall that T is the generic type representing the type of the value in the `Some` variant of an `Option`.
// 回想一下，T 是泛型类型，表示 `Option` 的 `Some` 变体中值的类型。
// That type T is also the return type of the `unwrap_or_else` function: code that calls `unwrap_or_else` on an Option<String>, for example, will get a `String`.
// 该类型 T 也是 `unwrap_or_else` 函数的返回类型：例如，在 Option<String> 上调用 `unwrap_or_else` 的代码将获得 `String`。

// Next, notice that the `unwrap_or_else` function has the additional generic type parameter F.
// 接下来，请注意 `unwrap_or_else` 函数具有额外的泛型类型参数 F。
// The F type is the type of the parameter named f, which is the `closure` we provide when calling `unwrap_or_else`.
// F 类型是名为 f 的参数的类型，也就是我们在调用 `unwrap_or_else` 时提供的 `closure`。

// The trait bound specified on the generic type F is `FnOnce() -> T`, which means F must be able to be called once, take no arguments, and return a T.
// 在泛型 F 上指定的 trait bound 是 `FnOnce() -> T`，这意味着 F 必须能够被调用一次，不接受任何参数，并返回一个 T。
// Using `FnOnce` in the trait bound expresses the constraint that `unwrap_or_else` is only going to call f at most one time.
// 在 trait bound 中使用 `FnOnce` 表达了 `unwrap_or_else` 最多只会调用 f 一次的约束。
// In the body of `unwrap_or_else`, we can see that if the Option is `Some`, f won’t be called. If the Option is `None`, f will be called once.
// 在 `unwrap_or_else` 的主体中，我们可以看到如果 Option 是 `Some`，则不会调用 f。 如果 Option 为“None”，则 f 将被调用一次。
// Because all `closures` implement `FnOnce`, `unwrap_or_else` accepts the most different kinds of `closures` and is as flexible as it can be.
// 因为所有 `closures` 都实现 `FnOnce`，所以 `unwrap_or_else` 接受最不同种类的 `closures` 并且尽可能灵活。

// Note: Functions can implement all three of the `Fn` traits too.
// 注意：函数也可以实现所有三个 `Fn` 特性。
// If what we want to do doesn’t require capturing a value from the environment,
// 如果我们想要做的事情不需要从环境中获取值，
// |- we can use the name of a function rather than a `closure` where we need something that implements one of the `Fn` traits.
// |- 我们可以使用函数的名称而不是 `closure`，因为我们需要实现 `Fn` 特征之一的东西。
// For example, on an Option<Vec<T>> value, we could call `unwrap_or_else(Vec::new)` to get a new, empty vector if the value is `None`.
// 例如，在 Option<Vec<T>> 值上，如果值为 `None`，我们可以调用 `unwrap_or_else(Vec::new)` 来获取一个新的空向量。

// Now let’s look at the standard library method `sort_by_key` defined on slices,
// 现在让我们看看在切片上定义的标准库方法 `sort_by_key`，
// |- to see how that differs from `unwrap_or_else` and why `sort_by_key` uses FnMut instead of FnOnce for the trait bound.
// |- 看看它与 `unwrap_or_else` 有何不同，以及为什么 `sort_by_key` 使用 FnMut 而不是 FnOnce 作为 trait bound。
// The `closure` gets one argument in the form of a reference to the current item in the slice being considered, and returns a value of type K that can be ordered.
// `closure` 以对正在考虑的切片中的当前项的引用的形式获取一个参数，并返回一个可以排序的 K 类型的值。
// This function is useful when you want to sort a slice by a particular attribute of each item. In Listing 13-7,
// 当您想按每个项目的特定属性对切片进行排序时，此函数很有用。 在清单 13-7 中，
// |- we have a list of `Rectangle` instances and we use `sort_by_key` to order them by their `width` attribute from low to high:
// |- 我们有一个 `Rectangle` 实例列表，我们使用 `sort_by_key` 按它们的 `width` 属性从低到高对它们进行排序：

// Filename: src/main.rs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

// Listing 13-7: Using `sort_by_key` to order rectangles by width

// This code prints:

// $ cargo run
//    Compiling rectangles v0.1.0 (file:///projects/rectangles)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.41s
//      Running `target/debug/rectangles`
// [
//     Rectangle {
//         width: 3,
//         height: 5,
//     },
//     Rectangle {
//         width: 7,
//         height: 12,
//     },
//     Rectangle {
//         width: 10,
//         height: 1,
//     },
// ]

// The reason `sort_by_key` is defined to take an `FnMut` `closure` is that it calls the `closure` multiple times:
// `sort_by_key` 被定义为采用 `FnMut` `closure` 的原因是它多次调用 `closure`：
// |- once for each item in the slice. The `closure` `|r| r.width` doesn’t capture, mutate, or move out anything from its environment, so it meets the trait bound requirements.
// |- 切片中的每个项目一次。 `closure` `|r| r.width` 不会从其环境中捕获、改变或移出任何东西，因此它满足特征绑定要求。

// In contrast, Listing 13-8 shows an example of a `closure` that implements just the `FnOnce` trait, because it moves a value out of the environment.
// 相反，清单 13-8 显示了一个仅实现 `FnOnce` 特性的 `closure` 示例，因为它将值移出环境。
// The compiler won’t let us use this `closure` with `sort_by_key`:
// 编译器不允许我们将此 `closure` 与 `sort_by_key` 一起使用：

// Filename: src/main.rs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}

// Listing 13-8: Attempting to use an `FnOnce` `closure` with `sort_by_key`
// 示例 13-8：尝试将 `FnOnce` `closure` 与 `sort_by_key` 结合使用

// This is a contrived, convoluted way (that doesn’t work) to try and count the number of times `sort_by_key` gets called when sorting `list`.
// 这是一种人为的、令人费解的方法（行不通），用于尝试计算在对 `list` 进行排序时调用 `sort_by_key` 的次数。
// This code attempts to do this counting by pushing `value—a` String from the `closure’s` environment—into the `sort_operations` vector.
// 这段代码试图通过将 `value—a` String 从 `closures` 环境中推入 `sort_operations` 向量来进行计数。
// The `closure` captures `value` then moves `value` out of the `closure` by transferring `ownership` of `value` to the `sort_operations` vector.
// `closure` 捕获 `value`，然后通过将 `value` 的 `ownership` 转移到 `sort_operations` 向量，将 `value` 移出 `closure`。
// This `closure` can be called once; trying to call it a second time wouldn’t work because `value` would no longer be in the environment to be pushed into `sort_operations` again!
// 这个 `closure` 可以被调用一次； 尝试第二次调用它是行不通的，因为 `value` 将不再位于环境中以再次被推入 `sort_operations`！
// Therefore, this `closure` only implements `FnOnce`. When we try to compile this code, we get this error that `value` can’t be moved out of the `closure` because the `closure` must implement `FnMut`:
// 因此，这个 `closure` 只实现了 `FnOnce`。 当我们尝试编译这段代码时，我们得到了这个错误，即 `value` 不能移出 `closure` 因为 `closure` 必须实现 `FnMut`：

// $ cargo run
//    Compiling rectangles v0.1.0 (file:///projects/rectangles)
// error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
//   --> src/main.rs:18:30
//    |
// 15 |       let value = String::from("by key called");
//    |           ----- captured outer variable
// 16 |
// 17 |       list.sort_by_key(|r| {
//    |  ______________________-
// 18 | |         sort_operations.push(value);
//    | |                              ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait
// 19 | |         r.width
// 20 | |     });
//    | |_____- captured by this `FnMut` closure
//
// For more information about this error, try `rustc --explain E0507`.
// error: could not compile `rectangles` due to previous error

// The error points to the line in the `closure` body that moves value out of the environment.
// 错误指向 `closure` 主体中将值移出环境的行。
// To fix this, we need to change the `closure` body so that it doesn’t move values out of the environment.
// 要解决此问题，我们需要更改 `closure` 主体，使其不会将值移出环境。
// To count the number of times `sort_by_key` is called, keeping a counter in the environment and incrementing its value in the `closure` body is a more straightforward way to calculate that.
// 要计算 `sort_by_key` 被调用的次数，在环境中保留一个计数器并在 `closure` 主体中增加它的值是一种更直接的计算方法。
// The `closure` in Listing 13-9 works with `sort_by_key` because it is only capturing a mutable reference to the `num_sort_operations` counter and can therefore be called more than once:
// 清单 13-9 中的 `closure` 与 `sort_by_key` 一起工作，因为它只捕获对 `num_sort_operations` 计数器的可变引用，因此可以多次调用：

// Filename: src/main.rs

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

// Listing 13-9: Using an `FnMut` `closure` with `sort_by_key` is allowed
// 示例 13-9：允许将 `FnMut` `closure` 与 `sort_by_key` 结合使用

// The Fn traits are important when defining or using functions or types that make use of `closures`.
// 在定义或使用使用 `closures` 的函数或类型时，Fn 特征很重要。
// In the next section, we’ll discuss iterators.
// 在下一节中，我们将讨论 iterators。
// Many iterator methods take `closure` arguments, so keep these `closure` details in mind as we continue!
// 许多 iterators 方法采用 `closure` 参数，因此在我们继续时请记住这些 `closure` 细节！

// Processing a Series of Items with `Iterators`
// 使用 `Iterators` 处理一系列项目
// The `iterator` pattern allows you to perform some task on a sequence of items in turn.
// `iterator` 模式允许您依次对一系列项目执行某些任务。
// An `iterator` is responsible for the logic of iterating over each item and determining when the sequence has finished.
// `iterator` 负责迭代每个项目并确定序列何时完成的逻辑。
// When you use `iterators`, you don’t have to reimplement that logic yourself.
// 当您使用 `iterators` 时，您不必自己重新实现该逻辑。

// In Rust, `iterators` are lazy, meaning they have no effect until you call methods that consume the `iterator` to use it up.
// 在 Rust 中，`iterators` 是惰性的，这意味着在您调用使用 `iterator` 的方法将其用完之前它们没有任何效果。
// For example, the code in Listing 13-10 creates an `iterator` over the items in the vector `v1` by calling the `iter` method defined on Vec<T>.
// 例如，清单 13-10 中的代码通过调用在 Vec<T> 上定义的 `iter` 方法在向量 `v1` 中的项目上创建一个 `iterator`。
// This code by itself doesn’t do anything useful.
// 这段代码本身没有做任何有用的事情。

let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

// Listing 13-10: Creating an `iterator`
// 示例 13-10：创建一个 `iterator`

// The `iterator` is stored in the `v1_iter` variable. Once we’ve created an `iterator`, we can use it in a variety of ways.
// `iterator` 存储在 `v1_iter` 变量中。 一旦我们创建了一个“迭代器”，我们就可以以多种方式使用它。
// In Listing 3-5 in Chapter 3, we iterated over an array using a `for` loop to execute some code on each of its items.
// 在第 3 章的清单 3-5 中，我们使用 `for` 循环遍历一个数组，以在其中的每个项目上执行一些代码。
// Under the hood this implicitly created and then consumed an `iterator`, but we glossed over how exactly that works until now.
// 在幕后，这隐式地创建并使用了一个 `iterator`，但直到现在我们都忽略了它究竟是如何工作的。

// In the example in Listing 13-11, we separate the creation of the `iterator` from the use of the `iterator` in the `for` loop.
// 在清单 13-11 的示例中，我们将 `iterator` 的创建与 `for` 循环中的 `iterator` 的使用分开。
// When the `for` loop is called using the `iterator` in `v1_iter`, each element in the `iterator` is used in one iteration of the loop, which prints out each value.
// 当使用 `v1_iter` 中的 `iterator` 调用 `for` 循环时，`iterator` 中的每个元素都用于循环的一次迭代，打印出每个值。

let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}

// Listing 13-11: Using an `iterator` in a `for` loop
// 示例 13-11：在 `for` 循环中使用 `iterator`

// In languages that don’t have `iterators` provided by their standard libraries, you would likely write this same functionality by starting a variable at index 0,
// 在标准库不提供 `iterators` 的语言中，您可能会通过在索引 0 处启动变量来编写相同的功能，
// |- using that variable to index into the vector to get a value, and incrementing the variable value in a loop until it reached the total number of items in the vector.
// |- 使用该变量索引到向量中以获取值，并在循环中递增变量值，直到它达到向量中的项目总数。

// `Iterators` handle all that logic for you, cutting down on repetitive code you could potentially mess up.
// `Iterators` 为您处理所有这些逻辑，减少您可能搞砸的重复代码。
// `Iterators` give you more flexibility to use the same logic with many different kinds of sequences, not just data structures you can index into, like vectors.
// `Iterators` 使您可以更灵活地对许多不同类型的序列使用相同的逻辑，而不仅仅是您可以索引到的数据结构，例如向量。
// Let’s examine how `iterators` do that.
// 让我们看看 `iterators` 是如何做到这一点的。

// The `Iterator` Trait and the `next` Method
// `Iterator` Trait 和 `next` 方法
// All `iterators` implement a trait named `Iterator` that is defined in the standard library. The definition of the trait looks like this:
// 所有 `iterators` 都实现了在标准库中定义的名为 `Iterator` 的特征。 特征的定义如下所示：

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

// Notice this definition uses some new syntax: `type Item` and `Self::Item`, which are defining an associated type with this trait.
// 请注意，此定义使用了一些新语法：`type Item` 和 `Self::Item`，它们定义了具有此特征的关联类型。
// We’ll talk about associated types in depth in Chapter 19.
// 我们将在第 19 章深入讨论关联类型。
// For now, all you need to know is that this code says implementing the Iterator trait requires that you also define an `Item` type,
// 现在，您只需要知道这段代码表示实现 Iterator 特性需要您还定义一个 `Item` 类型，
// |- and this `Item` type is used in the return type of the `next` method.
// |- 此 `Item` 类型用于 `next` 方法的返回类型。
// In other words, the `Item` type will be the type returned from the iterator.
// 换句话说，`Item` 类型将是从迭代器返回的类型。

// The `Iterator` trait only requires implementors to define one method: the `next` method,
// `Iterator` 特性只需要实现者定义一个方法：`next` 方法，
// |- which returns one item of the `iterator` at a time wrapped in `Some` and, when iteration is over, returns `None`.
// |- 一次返回一个包含在 `Some` 中的 `iterator` 项，当迭代结束时，返回 `None`。

// We can call the `next` method on `iterators` directly; Listing 13-12 demonstrates what values are returned from repeated calls to next on the `iterator` created from the `vector`.
// 我们可以直接调用 `iterators` 的 `next` 方法； 清单 13-12 演示了在从 `vector` 创建的 `iterator` 上重复调用 next 返回的值。

// Filename: src/lib.rs
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
// Listing 13-12: Calling the `next` method on an `iterator`
// 示例 13-12：在 `iterator` 上调用 `next` 方法

// Note that we needed to make `v1_iter` mutable: calling the `next` method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence.
// 请注意，我们需要使 `v1_iter` 可变：在迭代器上调用 `next` 方法会更改迭代器用来跟踪它在序列中的位置的内部状态。
// In other words, this code consumes, or uses up, the iterator.
// 换句话说，这段代码消耗或用完了迭代器。
// Each call to `next` eats up an item from the iterator.
// 每次调用 `next` 都会吃掉迭代器中的一个项目。
// We didn’t need to make `v1_iter` mutable when we used a for loop because the loop took ownership of `v1_iter` and made it mutable behind the scenes.
// 当我们使用 for 循环时，我们不需要使 `v1_iter` 可变，因为该循环获取了 `v1_iter` 的所有权并使其在幕后可变。

// Also note that the values we get from the calls to `next` are immutable references to the values in the vector.
// 另请注意，我们从对 `next` 的调用中获得的值是对 vector 中值的不可变引用。
// The iter method produces an iterator over immutable references.
// iter 方法在不可变引用上生成迭代器。
// If we want to create an iterator that takes ownership of `v1` and returns owned values, we can call `into_iter` instead of `iter`.
// 如果我们想创建一个迭代器来获取 `v1` 的所有权并返回拥有的值，我们可以调用 `into_iter` 而不是 `iter`。
// Similarly, if we want to iterate over mutable references, we can call `iter_mut` instead of `iter`.
// 同样，如果我们想遍历可变引用，我们可以调用 `iter_mut` 而不是 `iter`。

// Methods that Consume the `Iterator`
// 使用 `Iterator` 的方法
// The `Iterator` trait has a number of different methods with default implementations provided by the standard library; you can find out about these methods by looking in the standard library API documentation for the `Iterator` trait.
// `Iterator` trait 有许多不同的方法，标准库提供了默认实现； 您可以通过查看“迭代器”特性的标准库 API 文档来了解这些方法。
// Some of these methods call the `next` method in their definition, which is why you’re required to implement the `next` method when implementing the Iterator trait.
// 其中一些方法在其定义中调用了 `next` 方法，这就是为什么在实现 Iterator trait 时需要实现 `next` 方法的原因。

// Methods that call `next` are called consuming adaptors, because calling them uses up the iterator.
// 调用 `next` 的方法被称为消耗适配器，因为调用它们会耗尽迭代器。
// One example is the `sum` method, which takes ownership of the iterator and iterates through the items by repeatedly calling `next`, thus consuming the iterator.
// 一个例子是 `sum` 方法，它获取迭代器的所有权并通过重复调用 `next` 来迭代项目，从而消耗迭代器。
// As it iterates through, it adds each item to a running total and returns the total when iteration is complete.
// 在迭代过程中，它将每个项目添加到一个运行总计中，并在迭代完成时返回总计。
// Listing 13-13 has a test illustrating a use of the `sum` method:
// 清单 13-13 有一个测试说明了 `sum` 方法的使用：

// Filename: src/lib.rs

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// Listing 13-13: Calling the `sum` method to get the total of all items in the iterator
// 示例 13-13：调用 `sum` 方法获取迭代器中所有项的总和

// We aren’t allowed to use `v1_iter` after the call to `sum` because `sum` takes ownership of the iterator we call it on.
// 我们不允许在调用 `sum` 之后使用 `v1_iter`，因为 `sum` 拥有我们调用它的迭代器的所有权。

// Methods that Produce Other Iterators
// 产生其他迭代器的方法
// Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator.
// Iterator 适配器是定义在 Iterator trait 上的方法，它不消耗迭代器。
// Instead, they produce different iterators by changing some aspect of the original iterator.
// 相反，它们通过改变原始迭代器的某些方面来产生不同的迭代器。

// Listing 13-14 shows an example of calling the iterator adaptor method `map`, which takes a `closure` to call on each item as the items are iterated through.
// 清单 13-14 显示了一个调用迭代器适配器方法 `map` 的示例，它采用一个 `closure` 在迭代项目时调用每个项目。
// The `map` method returns a new iterator that produces the modified items.
// `map` 方法返回一个生成修改项的新迭代器。
// The `closure` here creates a new iterator in which each item from the `vector` will be incremented by 1:
// 这里的 `closure` 创建了一个新的迭代器，其中 `vector` 中的每一项都将递增 1：

// Filename: src/main.rs

let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);

// Listing 13-14: Calling the iterator adaptor `map` to create a new iterator
// 示例 13-14：调用迭代器适配器 `map` 来创建一个新的迭代器

// However, this code produces a warning:
// 但是，此代码会产生警告：

// $ cargo run
//    Compiling iterators v0.1.0 (file:///projects/iterators)
// warning: unused `Map` that must be used
//  --> src/main.rs:4:5
//   |
// 4 |     v1.iter().map(|x| x + 1);
//   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
//   |
//   = note: `#[warn(unused_must_use)]` on by default
//   = note: iterators are lazy and do nothing unless consumed
//
// warning: `iterators` (bin "iterators") generated 1 warning
//     Finished dev [unoptimized + debuginfo] target(s) in 0.47s
//      Running `target/debug/iterators`

// The code in Listing 13-14 doesn’t do anything; the `closure` we’ve specified never gets called.
// 示例 13-14 中的代码什么都不做； 我们指定的 closure 永远不会被调用。
// The warning reminds us why: iterator adaptors are lazy, and we need to consume the iterator here.
// 警告提醒我们原因：迭代器适配器是惰性的，我们需要在这里消费迭代器。

// To fix this warning and consume the iterator, we’ll use the `collect` method, which we used in Chapter 12 with `env::args` in Listing 12-1.
// 要修复此警告并使用迭代器，我们将使用 `collect` 方法，我们在第 12 章中使用了清单 12-1 中的 `env::args`。
// This method consumes the iterator and collects the resulting values into a collection data type.
// 此方法使用迭代器并将结果值收集到集合数据类型中。

// In Listing 13-15, we `collect` the results of iterating over the iterator that’s returned from the call to `map` into a vector.
// 在示例 13-15 中，我们将从调用 `map` 返回的 `iterator` 迭代到一个向量中，`collect` 结果。
// This vector will end up containing each item from the original vector incremented by 1.
// 该向量最终将包含原始向量中每一项增加 1。

// Filename: src/main.rs

let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);

// Listing 13-15: Calling the `map` method to create a new `iterator` and then calling the collect method to consume the new `iterator` and create a vector
// 示例 13-15：调用 `map` 方法创建一个新的 `iterator`，然后调用 collect 方法使用新的 `iterator` 并创建一个 vector

// Because `map` takes a `closure`, we can specify any operation we want to perform on each item.
// 因为 `map` 有一个 `closure`，我们可以指定我们想要对每个项目执行的任何操作。
// This is a great example of how `closures` let you customize some behavior while reusing the iteration behavior that the `Iterator` trait provides.
// 这是一个很好的例子，说明 `closures` 如何让您自定义一些行为，同时重用 `Iterator` 特性提供的迭代行为。

// You can chain multiple calls to `iterator` adaptors to perform complex actions in a readable way.
// 您可以将多个调用链接到 `iterator` 适配器，以可读的方式执行复杂的操作。
// But because all `iterators` are lazy, you have to call one of the consuming adaptor methods to get results from calls to `iterator` adaptors.
// 但因为所有 `iterators` 都是惰性的，所以您必须调用其中一种消耗适配器方法才能从对 `iterator` 适配器的调用中获取结果。

// Using `Closures` that Capture Their Environment
// 使用捕获环境的 `Closures`
// Many `iterator` adapters take `closures` as arguments, and commonly the `closures` we’ll specify as arguments to `iterator` adapters will be `closures` that capture their environment.
// 许多 `iterator` 适配器将 `closures` 作为参数，通常我们将指定为 `iterator` 适配器参数的 `closures` 将是捕获其环境的 `closures`。

// For this example, we’ll use the `filter` method that takes a `closure`. The `closure` gets an item from the `iterator` and returns a `bool`.
// 对于这个例子，我们将使用带有 `closure` 的 `filter` 方法。 `closure` 从 `iterator` 中获取一个项目并返回一个 `bool`。
// If the `closure` returns true, the value will be included in the iteration produced by `filter`. If the `closure` returns false, the value won’t be included.
// 如果 `closure` 返回 true，该值将包含在 `filter` 产生的迭代中。 如果 `closure` 返回 false，则不会包含该值。

// In Listing 13-16, we use `filter` with a `closure` that captures the `shoe_size` variable from its environment to iterate over a collection of `Shoe` struct instances.
// 在清单 13-16 中，我们将 `filter` 与一个 `closure` 一起使用，该闭包从其环境中捕获 `shoe_size` 变量以迭代一组 `Shoe` 结构实例。
// It will return only `shoes` that are the specified size.
// 它将只返回指定尺寸的 `shoes`。

// Filename: src/lib.rs
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

// Listing 13-16: Using the `filter` method with a `closure` that captures `shoe_size`
// 示例 13-16：将 `filter` 方法与捕获 `shoe_size` 的 `closure` 结合使用

// The `shoes_in_size` function takes `ownership` of a vector of shoes and a shoe size as parameters.
// `shoes_in_size` 函数以鞋子向量的 `ownership` 和鞋码作为参数。
// It returns a vector containing only shoes of the specified size.
// 它返回一个只包含指定尺寸鞋子的向量。

// In the body of `shoes_in_size`, we call `into_iter` to create an iterator that takes `ownership` of the vector.
// 在 `shoes_in_size` 的主体中，我们调用 `into_iter` 来创建一个迭代器来获取向量的 `ownership`。
// Then we call `filter` to adapt that iterator into a new iterator that only contains elements for which the `closure` returns true.
// 然后我们调用 `filter` 将该迭代器调整为一个新的迭代器，该迭代器只包含 `closure` 返回 true 的元素。

// The `closure` captures the `shoe_size` parameter from the environment and compares the value with each shoe’s size, keeping only shoes of the size specified.
// `closure` 从环境中捕获 `shoe_size` 参数并将该值与每只鞋的尺码进行比较，只保留指定尺码的鞋子。
// Finally, calling `collect` gathers the values returned by the adapted iterator into a vector that’s returned by the function.
// 最后，调用 `collect` 将适配迭代器返回的值收集到函数返回的向量中。

// The test shows that when we call `shoes_in_size`, we get back only shoes that have the same size as the value we specified.
// 测试表明，当我们调用 `shoes_in_size` 时，我们只会返回与我们指定的值具有相同尺码的鞋子。

// Improving Our I/O Project
// 改进我们的 I/O 项目
// With this new knowledge about iterators, we can improve the I/O project in Chapter 12 by using iterators to make places in the code clearer and more concise.
// 有了这些关于迭代器的新知识，我们可以通过使用迭代器来改进第 12 章中的 I/O 项目，使代码中的地方更清晰、更简洁。
// Let’s look at how iterators can improve our implementation of the Config::build function and the `search` function.
// 让我们看看迭代器如何改进我们对 Config::build 函数和 `search` 函数的实现。

// Removing a clone Using an Iterator
// 使用 迭代器 删除 clone
// In Listing 12-6, we added code that took a slice of String values and created an instance of the Config struct by indexing into the slice and cloning the values,
// 在清单 12-6 中，我们添加了获取 String 值切片的代码，并通过索引切片并克隆值来创建 Config 结构的实例，
// |- allowing the Config struct to own those values. In Listing 13-17, we’ve reproduced the implementation of the Config::build function as it was in Listing 12-23:
// |- 允许 Config 结构拥有这些值。 在示例 13-17 中，我们重现了示例 12-23 中的 Config::build 函数的实现：

// Filename: src/lib.rs

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// Listing 13-17: Reproduction of the Config::build function from Listing 12-23
// 示例 13-17：示例 12-23 中 Config::build 函数的再现

// At the time, we said not to worry about the inefficient `clone` calls because we would remove them in the future. Well, that time is now!
// 当时，我们说不用担心低效的 `clone` 调用，因为我们将来会删除它们。 好吧，现在是时候了！

// We needed `clone` here because we have a slice with String elements in the parameter `args`, but the `build` function doesn’t own `args`.
// 我们在这里需要 `clone`，因为我们在参数 `args` 中有一个包含 String 元素的切片，但是 `build` 函数不拥有 `args`。
// To return ownership of a Config instance, we had to `clone` the values from the `query` and `filename` fields of Config so the Config instance can own its values.
// 要返回 Config 实例的所有权，我们必须从 Config 的 `query` 和 `filename` 字段中“克隆”值，以便 Config 实例可以拥有它的值。

// With our new knowledge about iterators, we can change the `build` function to take ownership of an iterator as its argument instead of borrowing a slice.
// 有了关于迭代器的新知识，我们可以更改 `build` 函数以将迭代器的所有权作为其参数，而不是借用切片。
// We’ll use the iterator functionality instead of the code that checks the length of the slice and indexes into specific locations.
// 我们将使用迭代器功能而不是检查切片长度和索引到特定位置的代码。
// This will clarify what the Config::build function is doing because the iterator will access the values.
// 这将阐明 Config::build 函数正在做什么，因为迭代器将访问这些值。

// Once Config::build takes ownership of the iterator and stops using indexing operations that borrow,
// 一旦 Config::build 获得迭代器的所有权并停止使用借用的索引操作，
// |- we can move the String values from the iterator into Config rather than calling `clone` and making a new allocation.
// |- 我们可以将迭代器中的字符串值移动到 Config 中，而不是调用 `clone` 并进行新的分配。

// Using the Returned Iterator Directly
// 直接使用返回的迭代器
// Open your I/O project’s src/main.rs file, which should look like this:
// 打开 I/O 项目的 src/main.rs 文件，它应该如下所示：

// Filename: src/main.rs

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
}

// We’ll first change the start of the main function that we had in Listing 12-24 to the code in Listing 13-18, which this time uses an iterator.
// 我们首先将示例 12-24 中的 main 函数的开头更改为示例 13-18 中的代码，这次使用迭代器。
// This won’t compile until we update Config::build as well.
// 在我们也更新 Config::build 之前，这不会编译。

// Filename: src/main.rs

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip--
}

// Listing 13-18: Passing the return value of env::args to Config::build
// 示例 13-18：将 env::args 的返回值传递给 Config::build

// The env::args function returns an iterator! Rather than collecting the iterator values into a vector and then passing a slice to Config::build, `
// env::args 函数返回一个迭代器！ 与其将迭代器值收集到一个向量中，然后将一个切片传递给 Config::build，不如`
// |- now we’re passing ownership of the iterator returned from env::args to Config::build directly.
// |- 现在我们将从 env::args 返回的迭代器的所有权直接传递给 Config::build。

// Next, we need to update the definition of Config::build.
// 接下来，我们需要更新 Config::build 的定义。
// In your I/O project’s src/lib.rs file, let’s change the signature of Config::build to look like Listing 13-19.
// 在您的 I/O 项目的 src/lib.rs 文件中，我们将 Config::build 的签名更改为如清单 13-19 所示。
// This still won’t compile because we need to update the function body.
// 这仍然不会编译，因为我们需要更新函数体。

// Filename: src/lib.rs

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // --snip--

// Listing 13-19: Updating the signature of Config::build to expect an iterator
// 示例 13-19：更新 Config::build 的签名以期待一个迭代器

// The standard library documentation for the env::args function shows that the type of the iterator it returns is std::env::Args,
// env::args 函数的标准库文档显示它返回的迭代器的类型是 std::env::Args，
// |- and that type implements the Iterator trait and returns String values.
// |- 并且该类型实现了 Iterator 特性并返回 String 值。

// We’ve updated the signature of the Config::build function so the parameter args has a generic type with the trait bounds `impl Iterator<Item = String>` instead of &[String].
// 我们更新了 Config::build 函数的签名，因此参数 args 具有泛型类型，其特征范围为 `impl Iterator<Item = String>` 而不是 &[String]。
// This usage of the impl Trait syntax we discussed in the “Traits as Parameters” section of Chapter 10 means that args can be any type that implements the Iterator type and returns String items.
// 我们在第 10 章的“作为参数的特征”一节中讨论的 impl Trait 语法的这种用法意味着 args 可以是实现 Iterator 类型并返回 String 项的任何类型。

// Because we’re taking ownership of args and we’ll be mutating args by iterating over it, we can add the mut keyword into the specification of the args parameter to make it mutable.
// 因为我们正在获取 args 的所有权，并且我们将通过迭代它来改变 args，我们可以将 mut 关键字添加到 args 参数的规范中以使其可变。

// Using Iterator Trait Methods Instead of Indexing
// 使用迭代器特征方法而不是索引
// Next, we’ll fix the body of Config::build.
// 接下来，我们将修复 Config::build 的主体。
// Because args implements the Iterator trait, we know we can call the next method on it! Listing 13-20 updates the code from Listing 12-23 to use the next method:
// 因为 args 实现了 Iterator 特性，我们知道我们可以调用它的 next 方法！ 清单 13-20 更新了清单 12-23 中的代码以使用 next 方法：

// Filename: src/lib.rs

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// Listing 13-20: Changing the body of Config::build to use iterator methods
// 示例 13-20：更改 Config::build 的主体以使用迭代器方法

// Remember that the first value in the return value of env::args is the name of the program.
// 请记住，env::args 的返回值中的第一个值是程序的名称。
// We want to ignore that and get to the next value, so first we call next and do nothing with the return value.
// 我们想忽略它并获取下一个值，所以首先我们调用 next 并且不对返回值执行任何操作。
// Second, we call next to get the value we want to put in the query field of Config.
// 其次，我们调用 next 来获取我们想要放入 Config 的查询字段中的值。
// If next returns a Some, we use a match to extract the value. If it returns None, it means not enough arguments were given and we return early with an Err value.
// 如果 next 返回一个 Some，我们使用匹配来提取值。 如果它返回 None，则意味着没有提供足够的参数，我们会提前返回一个 Err 值。
// We do the same thing for the filename value.
// 我们对文件名值做同样的事情。

// Making Code Clearer with Iterator Adaptors
// 使用迭代器适配器使代码更清晰
// We can also take advantage of iterators in the search function in our I/O project, which is reproduced here in Listing 13-21 as it was in Listing 12-19:
// 我们还可以在 I/O 项目的搜索函数中利用迭代器，它在清单 13-21 中重现为清单 12-19：

// Filename: src/lib.rs

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// Listing 13-21: The implementation of the search function from Listing 12-19
// 示例 13-21：示例 12-19 中搜索功能的实现

// We can write this code in a more concise way using iterator adaptor methods.
// 我们可以使用迭代器适配器方法以更简洁的方式编写此代码。
// Doing so also lets us avoid having a mutable intermediate `results` vector.
// 这样做还可以让我们避免使用可变的中间 `results` 向量。
// The functional programming style prefers to minimize the amount of mutable state to make code clearer.
// 函数式编程风格倾向于最小化可变状态的数量以使代码更清晰。
// Removing the mutable state might enable a future enhancement to make searching happen in parallel, because we wouldn’t have to manage concurrent access to the `results` vector.
// 删除可变状态可能会启用未来的增强功能，使搜索并行进行，因为我们不必管理对 `results` 向量的并发访问。
// Listing 13-22 shows this change:
// 清单 13-22 显示了这个变化：

// Filename: src/lib.rs

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// Listing 13-22: Using iterator adaptor methods in the implementation of the `search` function
// 示例 13-22：在 `search` 函数的实现中使用迭代器适配器方法

// Recall that the purpose of the `search` function is to return all lines in contents that contain the query.
// 回想一下，`search` 函数的目的是返回内容中包含查询的所有行。
// Similar to the filter example in Listing 13-16, this code uses the filter adaptor to keep only the lines that line.contains(query) returns true for.
// 类似于清单 13-16 中的过滤器示例，此代码使用过滤器适配器仅保留 line.contains(query) 为其返回 true 的行。
// We then collect the matching lines into another vector with collect.
// 然后我们使用 collect 将匹配的行收集到另一个向量中。
// Much simpler! Feel free to make the same change to use iterator methods in the `search_case_insensitive` function as well.
// 简单多了！ 随意进行相同的更改以在 `search_case_insensitive` 函数中使用迭代器方法。

// Choosing Between Loops or Iterators
// 在循环或迭代器之间进行选择
// The next logical question is which style you should choose in your own code and why: the original implementation in Listing 13-21 or the version using iterators in Listing 13-22.
// 下一个合乎逻辑的问题是您应该在自己的代码中选择哪种样式以及为什么：示例 13-21 中的原始实现或示例 13-22 中使用迭代器的版本。
// Most Rust programmers prefer to use the iterator style.
// 大多数 Rust 程序员更喜欢使用迭代器样式。
// It’s a bit tougher to get the hang of at first, but once you get a feel for the various iterator adaptors and what they do, iterators can be easier to understand.
// 一开始有点难掌握，但是一旦你了解了各种迭代器适配器及其作用，迭代器就会更容易理解。
// Instead of fiddling with the various bits of looping and building new vectors, the code focuses on the high-level objective of the loop.
// 代码没有摆弄循环和构建新向量的各种位，而是专注于循环的高级目标。
// This abstracts away some of the commonplace code so it’s easier to see the concepts that are unique to this code, such as the filtering condition each element in the iterator must pass.
// 这里抽象掉了一些普通的代码，所以更容易看出这段代码特有的概念，比如迭代器中每个元素必须通过的过滤条件。

// But are the two implementations truly equivalent? The intuitive assumption might be that the more low-level loop will be faster. Let’s talk about performance.
// 但是这两个实现真的是等价的吗？ 直观的假设可能是更底层的循环越快。 让我们谈谈性能。


// Comparing Performance: Loops vs. Iterators
// 比较性能：循环与迭代器
// To determine whether to use loops or iterators, you need to know which implementation is faster: the version of the search function with an explicit for loop or the version with iterators.
// 要确定是使用循环还是迭代器，您需要知道哪种实现更快：具有显式 for 循环的搜索函数版本或具有迭代器的版本。

// We ran a benchmark by loading the entire contents of The Adventures of Sherlock Holmes by Sir Arthur Conan Doyle into a String and looking for the word the in the contents.
// 我们通过将 Arthur Conan Doyle 爵士的 The Adventures of Sherlock Holmes 的全部内容加载到一个字符串中并在内容中查找单词 the 来运行基准测试。
// Here are the results of the benchmark on the version of search using the for loop and the version using iterators:
// 以下是使用 for 循环的搜索版本和使用迭代器的版本的基准测试结果：

// test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
// test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)

// The iterator version was slightly faster!
// 迭代器版本稍微快一点！
// We won’t explain the benchmark code here, because the point is not to prove that the two versions are equivalent but to get a general sense of how these two implementations compare performance-wise.
// 我们不会在这里解释基准测试代码，因为重点不是要证明这两个版本是等价的，而是要大致了解这两个实现在性能方面的比较。

// For a more comprehensive benchmark, you should check using various texts of various sizes as the contents, different words and words of different lengths as the query, and all kinds of other variations.
// 为了更全面的基准，你应该检查使用各种大小的文本作为内容，不同的单词和不同长度的单词作为查询，以及各种其他变体。
// The point is this: iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself.
// 重点是：迭代器虽然是高级抽象，但会被编译成大致相同的代码，就好像您自己编写了较低级别的代码一样。
// Iterators are one of Rust’s zero-cost abstractions, by which we mean using the abstraction imposes no additional runtime overhead.
// 迭代器是 Rust 的零成本抽象之一，我们的意思是使用抽象不会强加额外的运行时开销。
// This is analogous to how Bjarne Stroustrup, the original designer and implementor of C++, defines zero-overhead in “Foundations of C++” (2012):
// 这类似于 C++ 的原始设计者和实现者 Bjarne Stroustrup 在“Foundations of C++”（2012 年）中定义零开销的方式：

// In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.

// As another example, the following code is taken from an audio decoder.
// 作为另一个例子，以下代码取自音频解码器。
// The decoding algorithm uses the linear prediction mathematical operation to estimate future values based on a linear function of the previous samples.
// 解码算法使用线性预测数学运算，根据先前样本的线性函数来估计未来值。
// This code uses an iterator chain to do some math on three variables in scope: a buffer slice of data, an array of 12 coefficients, and an amount by which to shift data in qlp_shift.
// 此代码使用迭代器链对范围内的三个变量进行一些数学运算：数据的缓冲区切片、12 个系数的数组和 qlp_shift 中的数据移位量。
// We’ve declared the variables within this example but not given them any values;
// 我们已经在这个例子中声明了变量，但没有给它们任何值；
// although this code doesn’t have much meaning outside of its context, it’s still a concise, real-world example of how Rust translates high-level ideas to low-level code.
// 尽管这段代码在其上下文之外没有太多意义，但它仍然是 Rust 如何将高级思想转换为低级代码的简洁、真实的示例。

let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}

// To calculate the value of prediction, this code iterates through each of the 12 values in coefficients and uses the zip method to pair the coefficient values with the previous 12 values in buffer.
// 为了计算预测值，此代码遍历系数中的 12 个值中的每一个，并使用 zip 方法将系数值与缓冲区中的前 12 个值配对。
// Then, for each pair, we multiply the values together, sum all the results, and shift the bits in the sum qlp_shift bits to the right.
// 然后，对于每一对，我们将值相乘，对所有结果求和，并将求和 qlp_shift 位中的位右移。

// Calculations in applications like audio decoders often prioritize performance most highly.
// 音频解码器等应用程序中的计算通常最优先考虑性能。
// Here, we’re creating an iterator, using two adaptors, and then consuming the value.
// 在这里，我们创建一个迭代器，使用两个适配器，然后使用值。
// What assembly code would this Rust code compile to? Well, as of this writing, it compiles down to the same assembly you’d write by hand.
// 这个 Rust 代码会编译成什么样的汇编代码？ 好吧，在撰写本文时，它会编译成您手写的相同的assembly。
// There’s no loop at all corresponding to the iteration over the values in coefficients: Rust knows that there are 12 iterations, so it “unrolls” the loop.
// 根本没有对应于系数值迭代的循环：Rust 知道有 12 次迭代，所以它“展开”循环。
// Unrolling is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop.
// 展开是一种优化，它消除了循环控制代码的开销，而是为循环的每次迭代生成重复代码。

// All of the coefficients get stored in registers, which means accessing the values is very fast.
// 所有系数都存储在寄存器中，这意味着访问这些值非常快。
// There are no bounds checks on the array access at runtime.
// 在运行时对数组访问没有边界检查。
// All these optimizations that Rust is able to apply make the resulting code extremely efficient.
// Rust 能够应用的所有这些优化使得生成的代码非常高效。
// Now that you know this, you can use iterators and closures without fear!
// 既然你知道了这一点，你就可以毫无畏惧地使用迭代器和闭包了！
// They make code seem like it’s higher level but don’t impose a runtime performance penalty for doing so.
// 它们使代码看起来更高级，但不会因此而造成运行时性能损失。

// Summary
// 概括
// Closures and iterators are Rust features inspired by functional programming language ideas.
// 闭包和迭代器是受函数式编程语言思想启发的 Rust 特性。
// They contribute to Rust’s capability to clearly express high-level ideas at low-level performance.
// 它们有助于 Rust 以低级性能清楚地表达高级思想的能力。
// The implementations of closures and iterators are such that runtime performance is not affected.
// 闭包和迭代器的实现不会影响运行时性能。
// This is part of Rust’s goal to strive to provide zero-cost abstractions.
// 这是 Rust 努力提供零成本抽象的目标的一部分。

// Now that we’ve improved the expressiveness of our I/O project, let’s look at some more features of cargo that will help us share the project with the world.
// 现在我们已经改进了 I/O 项目的表现力，让我们看看 cargo 的更多功能，这些功能将帮助我们与世界分享该项目。
