// The match Control Flow Construct
// match控制流结构
// Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
// Rust有一个非常强大的控制流结构，称为match，它允许你将一个值与一系列模式进行比较，然后根据匹配的模式执行代码。
// Patterns can be made up of literal values, variable names, wildcards, and many other things; Chapter 18 covers all the different kinds of patterns and what they do.
// 模式可以由文字值、变量名、通配符和许多其他东西组成;第18章涵盖了所有不同类型的模式及其作用。
// The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.
// 匹配的力量来自于模式的表达能力，以及编译器确认所有可能的情况都被处理的事实。

// Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it,
// 可以把match表达式想象成一台硬币分拣机:硬币沿着有大小不同的孔的轨道滑下，
// and each coin falls through the first hole it encounters that it fits into.
// 每枚硬币都会从它碰到的第一个适合它的洞里掉下去。
// In the same way, values go through each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution.
// 以同样的方式，值在匹配中的每个模式中，在值“符合”的第一个模式中，值落入相关的代码块中，在执行期间使用。

// Speaking of coins, let’s use them as an example using match! We can write a function that takes an unknown United States coin and,
// 说到硬币，让我们以它们为例使用match！我们可以写一个函数，取一枚未知的美国硬币，
// in a similar way as the counting machine, determines which coin it is and returns its value in cents, as shown here in Listing 6-3.
// 以类似于计数机的方式确定它是哪个硬币并以美分为单位返回其值，如清单6-3所示。

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Listing 6-3: An enum and a match expression that has the variants of the enum as its patterns
// 清单6-3:枚举和匹配表达式，其中枚举的变体作为其模式

// Let’s break down the match in the value_in_cents function.
// 让我们在value_in_cents函数中分解匹配。
// First, we list the match keyword followed by an expression, which in this case is the value coin.
// 首先，我们列出match关键字后面跟着一个表达式，在本例中是value coin。
// This seems very similar to an expression used with if, but there’s a big difference: with if, the expression needs to return a Boolean value, but here, it can return any type.
// 这看起来非常类似于if的表达式，但有一个很大的区别:if的表达式需要返回一个布尔值，但在这里，它可以返回任何类型。
// The type of coin in this example is the Coin enum that we defined on the first line.
// 本例中的coin类型是我们在第一行中定义的coin枚举。

// Next are the match arms.
// 接下来是match arms。
// An arm has two parts: a pattern and some code.
// arm有两部分:模式和一些代码。
// The first arm here has a pattern that is the value Coin::Penny and then the => operator that separates the pattern and the code to run.
// 这里的第一个arm有一个值为Coin::Penny的模式，然后是=>操作符，分隔模式和要运行的代码。
// The code in this case is just the value 1. Each arm is separated from the next with a comma.
// 在这种情况下，代码只是值1。每条arm之间用逗号隔开。

// When the match expression executes, it compares the resulting value against the pattern of each arm, in order.
// 当match表达式执行时，它将结果值与每个arm的模式进行顺序比较。
// If a pattern matches the value, the code associated with that pattern is executed.
// 如果模式与该值匹配，则执行与该模式相关的代码。
// If that pattern doesn’t match the value, execution continues to the next arm, much as in a coin-sorting machine.
// 如果该模式与值不匹配，则继续执行下一个arm，就像在硬币分拣机中一样。
// We can have as many arms as we need: in Listing 6-3, our match has four arms.
// 我们可以拥有任意数量的分支:在清单6-3中，我们的匹配有四个分支。

// The code associated with each arm is an expression, and the resulting value of the expression in the matching arm is the value that gets returned for the entire match expression.
// 与每个arm相关联的代码是一个表达式，匹配arm中的表达式的结果值是整个匹配表达式的返回值。

// We don’t typically use curly brackets if the match arm code is short, as it is in Listing 6-3 where each arm just returns a value.
// 如果匹配arm代码很短，我们通常不使用花括号，如清单6-3所示，其中每个arm只返回一个值。
// If you want to run multiple lines of code in a match arm, you must use curly brackets, and the comma following the arm is then optional.
// 如果你想在一个匹配arm中运行多行代码，你必须使用花括号，然后arm后面的逗号是可选的。
// For example, the following code prints “Lucky penny!” every time the method is called with a Coin::Penny, but still returns the last value of the block, 1:
// 例如，下面的代码输出“Lucky penny!”，但仍然返回块的最后一个值1:

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


// Patterns that Bind to Values
// 绑定到值的模式
// Another useful feature of match arms is that they can bind to the parts of the values that match the pattern.
// 匹配臂（match arms）的另一个有用的特性是它们可以绑定到匹配模式的值的部分。
// This is how we can extract values out of enum variants.
// 这是我们如何从枚举变量中提取值。

// As an example, let’s change one of our enum variants to hold data inside it.
// 作为一个例子，让我们改变一个枚举变量来保存数据。
// From 1999 through 2008, the United States minted quarters with different designs for each of the 50 states on one side.
// 从1999年到2008年，美国在硬币的一面为50个州铸造了不同图案的硬币。
// No other coins got state designs, so only quarters have this extra value.
// 没有其他硬币有国家图案，所以只有25美分的硬币有这种额外的价值。
// We can add this information to our enum by changing the Quarter variant to include a UsState value stored inside it, which we’ve done here in Listing 6-4.
// 我们可以将此信息添加到枚举中，方法是更改Quarter变体以包含存储在其中的UsState值，如清单6-4所示。


#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Listing 6-4: A Coin enum in which the Quarter variant also holds a UsState value
// 清单6-4:一个Coin枚举，其中Quarter变体也包含UsState值

// Let’s imagine that a friend is trying to collect all 50 state quarters.
// 让我们想象一下，一个朋友正试图收集全部50个州的硬币。
// While we sort our loose change by coin type, we’ll also call out the name of the state associated with each quarter so if it’s one our friend doesn’t have, they can add it to their collection.
// 当我们把零钱按硬币的类型分类时，我们还会说出每个硬币对应的州名，这样如果我们的朋友没有这个硬币，他们就可以把它加入他们的收藏中。

// In the match expression for this code, we add a variable called state to the pattern that matches values of the variant Coin::Quarter.
// 在这段代码的匹配表达式中，我们将一个名为state的变量添加到匹配变体Coin::Quarter的值的模式中。
// When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state.
// 当Coin::Quarter匹配时，状态变量将绑定到该Quarter的状态值。
// Then we can use state in the code for that arm, like so:
// 然后我们可以在手臂（arm）的代码中使用state，像这样:

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be Coin::Quarter(UsState::Alaska).
// 如果我们要调用value_in_cents(Coin::Quarter(UsState::Alaska))，Coin将是Coin::Quarter(UsState::Alaska)。
// When we compare that value with each of the match arms, none of them match until we reach Coin::Quarter(state).
// 当我们将该值与每个匹配臂进行比较时，它们都不匹配，直到我们达到Coin::Quarter(状态)。
// At that point, the binding for state will be the value UsState::Alaska.
// 此时，state的绑定值为UsState::Alaska。
// We can then use that binding in the println! expression, thus getting the inner state value out of the Coin enum variant for Quarter.
// 我们可以在println!表达式，从而从Quarter的Coin枚举变量中获得内部状态值。

// Matching with Option<T>
// 匹配Option<T>
// In the previous section, we wanted to get the inner T value out of the Some case when using Option<T>;
// 在上一节中，当使用Option<T>时，我们想从Some情况中获得内部T值;
// we can also handle Option<T> using match as we did with the Coin enum! Instead of comparing coins,
// 我们也可以使用match来处理Option<T>，就像我们处理Coin enum一样！不是比较硬币，
// we’ll compare the variants of Option<T>, but the way that the match expression works remains the same.
// 我们将比较Option<T>的变体，但匹配表达式的工作方式保持不变。

// Let’s say we want to write a function that takes an Option<i32> and, if there’s a value inside, adds 1 to that value.
// 假设我们想要编写一个函数，该函数接受Option<i32>，如果里面有一个值，则将该值加1。
// If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.
// 如果里面没有值，函数应该返回None值并且不尝试执行任何操作。

// This function is very easy to write, thanks to match, and will look like Listing 6-5.
// 这个函数很容易写，多亏了match，看起来像清单6-5。

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

// Listing 6-5: A function that uses a match expression on an Option<i32>
// 清单6-5:在Option<i32>上使用匹配表达式的函数

// Let’s examine the first execution of plus_one in more detail.
// 让我们更详细地检查plus_one的第一次执行。
// When we call plus_one(five), the variable x in the body of plus_one will have the value Some(5).
// 当我们调用plus_one(5)时，plus_one函数体中的变量x的值将是Some(5)。
// We then compare that against each match arm.
// 然后将其与每个匹配臂进行比较。

// None => None,

// The Some(5) value doesn’t match the pattern None, so we continue to the next arm.
// Some(5)值与模式None不匹配，因此我们继续下一个臂。

// Some(i) => Some(i + 1),

// Does Some(5) match Some(i)? Why yes it does! We have the same variant.
// Some(5)匹配Some(i)?为什么会这样?我们有相同的变体。
// The i binds to the value contained in Some, so i takes the value 5.
// i绑定到Some中包含的值，因此i取值5。
// The code in the match arm is then executed, so we add 1 to the value of i and create a new Some value with our total 6 inside.
// 然后执行匹配臂中的代码，因此我们将i的值加上1，并创建一个新的Some值，其中包含我们的总数6。

// Now let’s consider the second call of plus_one in Listing 6-5, where x is None.
// 现在让我们考虑清单6-5中对plus_one的第二次调用，其中x为None。
// We enter the match and compare to the first arm.
// 我们输入匹配并与第一个臂进行比较。

// None => None,

// It matches! There’s no value to add to, so the program stops and returns the None value on the right side of =>.
// 匹配！没有要添加的值，因此程序停止并返回=>右侧的None值。
// Because the first arm matched, no other arms are compared.
// 由于第一个臂匹配，没有其他臂进行比较。

// Combining match and enums is useful in many situations.
// 结合match和enum在很多情况下是有用的。
// You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it.
// 你会在Rust代码中看到很多这种模式:匹配一个枚举，将一个变量绑定到其中的数据，然后基于它执行代码。
// It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite.
// 刚开始会有点棘手，但一旦你习惯了，你就会希望所有语言都有它了。它一直是用户的最爱。

// Matches Are Exhaustive
// 匹配是穷尽的
// There’s one other aspect of match we need to discuss: the arms’ patterns must cover all possibilities.
// 我们还需要讨论匹配的另一个方面:手臂的图案（the arms' patterns）必须涵盖所有的可能性。
// Consider this version of our plus_one function, which has a bug and won’t compile:
// 考虑这个版本的plus_one函数，它有一个bug，不能编译:

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}

// We didn’t handle the None case, so this code will cause a bug.
// 我们没有处理None情况，所以这段代码将导致一个错误。
// Luckily, it’s a bug Rust knows how to catch.
// 幸运的是，Rust知道如何捕捉这个bug。
// If we try to compile this code, we’ll get this error:
// 如果我们试图编译这段代码，我们会得到这样的错误:

// $ cargo run
//    Compiling enums v0.1.0 (file:///projects/enums)
// error[E0004]: non-exhaustive patterns: `None` not covered
//    --> src/main.rs:3:15
//     |
// 3   |         match x {
//     |               ^ pattern `None` not covered
//     |
// note: `Option<i32>` defined here
//     = note: the matched value is of type `Option<i32>`
// help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
//     |
// 4   ~             Some(i) => Some(i + 1),
// 5   ~             None => todo!(),
//     |
//
// For more information about this error, try `rustc --explain E0004`.
// error: could not compile `enums` due to previous error

// Rust knows that we didn’t cover every possible case and even knows which pattern we forgot!
// Rust知道我们没有涵盖所有可能的情况，甚至知道我们忘记了哪个模式!
// Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.
// Rust中的匹配是穷尽的:为了使代码有效，我们必须穷尽所有最后的可能性。
// Especially in the case of Option<T>, when Rust prevents us from forgetting to explicitly handle the None case,
// 特别是在Option<T>的情况下，当Rust阻止我们忘记显式地处理None情况时，
// it protects us from assuming that we have a value when we might have null, thus making the billion-dollar mistake discussed earlier impossible.
// 它保护我们不假设我们有一个值，当我们可能有null时，从而使前面讨论的数十亿美元的错误不可能发生。

// Catch-all Patterns and the _ Placeholder
// 捕获所有模式和_占位符
// Using enums, we can also take special actions for a few particular values, but for all other values take one default action.
// 使用枚举，我们还可以对一些特定值采取特殊操作，但对所有其他值采取一个默认操作。
// Imagine we’re implementing a game where, if you roll a 3 on a dice roll, your player doesn’t move, but instead gets a new fancy hat.
// 假设我们正在执行一个游戏，如果你在掷骰子时掷出3，你的玩家不会移动，而是得到一顶新帽子。
// If you roll a 7, your player loses a fancy hat.
// 如果你摇到7，你的玩家会失去一顶漂亮的帽子。
// For all other values, your player moves that number of spaces on the game board.
// 对于所有其他值，你的玩家移动游戏棋盘上的空间数量。
// Here’s a match that implements that logic,
// 这是一个实现该逻辑的匹配，
// with the result of the dice roll hardcoded rather than a random value,
// 与骰子滚动的结果硬编码而不是随机值，
// and all other logic represented by functions without bodies because actually implementing them is out of scope for this example:
// 和所有其他由没有实现（body）的函数表示的逻辑，因为实际实现它们超出了这个例子的范围:

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

// For the first two arms, the patterns are the literal values 3 and 7.
// 对于前两个arm，模式是文字值3和7。
// For the last arm that covers every other possible value, the pattern is the variable we’ve chosen to name other.
// 对于覆盖所有其他可能值的最后一个arm，模式是我们选择命名为other的变量。
// The code that runs for the other arm uses the variable by passing it to the move_player function.
// 为另一个手臂运行的代码通过将其传递给move_player函数来使用该变量。

// This code compiles, even though we haven’t listed all the possible values a u8 can have, because the last pattern will match all values not specifically listed.
// 这段代码会被编译，尽管我们没有列出u8可能拥有的所有值，因为最后一个模式将匹配所有没有特别列出的值。
// This catch-all pattern meets the requirement that match must be exhaustive.
// 此catch-all模式满足匹配必须是穷尽的要求。
// Note that we have to put the catch-all arm last because the patterns are evaluated in order.
// 注意，我们必须把catch-all臂放在最后，因为模式是按顺序计算的。
// If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add arms after a catch-all!
// 如果我们把catch-all手臂放在更早的时候，其他的手臂将永远不会运行，所以Rust会警告我们如果我们在catch-all之后添加手臂!

// Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern:
// Rust也有一个模式，当我们想要一个catch-all但不想使用catch-all模式中的值时可以使用:
// _ is a special pattern that matches any value and does not bind to that value.
// _是一个特殊的模式，它匹配任何值并且不绑定到该值。
// This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.
// 这告诉Rust我们不会使用这个值，所以Rust不会警告我们有一个未使用的变量。

// Let’s change the rules of the game: now, if you roll anything other than a 3 or a 7, you must roll again.
// 让我们改变游戏规则:现在，如果你摇到的不是3或7，你必须再摇一次。
// We no longer need to use the catch-all value, so we can change our code to use _ instead of the variable named other:
// 我们不再需要使用catch-all值，所以我们可以将代码改为使用_来代替名为other的变量:

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

// This example also meets the exhaustiveness requirement because we’re explicitly ignoring all other values in the last arm; we haven’t forgotten anything.
// 这个例子也满足穷尽性要求，因为我们显式地忽略了最后一个臂中的所有其他值;我们什么都没忘。

// Finally, we’ll change the rules of the game one more time, so that nothing else happens on your turn if you roll anything other than a 3 or a 7.
// 最后，我们再一次改变游戏规则，如果你掷到3或7以外的点，在你的回合不会发生其他事情。
// We can express that by using the unit value (the empty tuple type we mentioned in “The Tuple Type” section) as the code that goes with the _ arm:
// 我们可以通过使用unit value（单位值）(我们在“元组类型”部分提到的空元组类型)作为 _ arm 的代码来表示:

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}

// Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t match a pattern in an earlier arm, and we don’t want to run any code in this case.
// 在这里，我们显式地告诉Rust，我们不会使用任何与早期arm中的模式不匹配的其他值，并且我们不想在这种情况下运行任何代码。

// There’s more about patterns and matching that we’ll cover in Chapter 18.
// 我们将在第18章讨论更多关于模式和匹配的内容。
// For now, we’re going to move on to the if let syntax, which can be useful in situations where the match expression is a bit wordy.
// 现在，我们将转向if let语法，它在匹配表达式有点啰嗦的情况下很有用。

fn main() {
    // Concise Control Flow with if let
    // The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.
    // if let语法允许你将if和let组合成一种不太详细的方式来处理匹配一个模式的值，同时忽略其余的值。
    // Consider the program in Listing 6-6 that matches on an Option<u8> value in the config_max variable but only wants to execute code if the value is the Some variant.
    // 考虑清单6-6中的程序，它匹配config_max变量中的Option<u8>值，但只希望在该值为Some变量时执行代码。

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Listing 6-6: A match that only cares about executing code when the value is Some
    // 清单6-6:当值为Some时，匹配只关心执行代码

    // If the value is Some, we print out the value in the Some variant by binding the value to the variable max in the pattern.
    // 如果值是Some，我们通过将值绑定到模式中的变量max来打印出Some变量中的值。
    // We don’t want to do anything with the None value. To satisfy the match expression, we have to add _ => () after processing just one variant, which is annoying boilerplate code to add.
    // 我们不想对None值做任何事情。为了满足匹配表达式，我们必须在处理了一个变量后添加_ =>()，这是一个讨厌的样板代码。

    // Instead, we could write this in a shorter way using if let.
    // 相反，我们可以用if let把它写得更简短。
    // The following code behaves the same as the match in Listing 6-6:
    // 下面的代码行为与清单6-6中的匹配相同:

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // The syntax if let takes a pattern and an expression separated by an equal sign.
    // if let的语法是一个模式和一个用等号分隔的表达式。
    // It works the same way as a match, where the expression is given to the match and the pattern is its first arm.
    // 它的工作方式与匹配相同，其中表达式被赋予match，而模式是它的第一个arm。
    // In this case, the pattern is Some(max), and the max binds to the value inside the Some.
    // 在这种情况下，模式是Some(max)， max绑定到Some内部的值。
    // We can then use max in the body of the if let block in the same way as we used max in the corresponding match arm.
    // 我们可以在if let块的主体中使用max，就像我们在对应的match arm中使用max一样。
    // The code in the if let block isn’t run if the value doesn’t match the pattern.
    // 如果值与模式不匹配，if let块中的代码将不运行。

    // Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that match enforces.
    // 使用if let意味着更少的输入，更少的缩进和更少的样板代码。但是，您失去了match强制执行的详尽检查。
    // Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
    // match和if let之间的选择取决于你在特定情况下所做的事情，以及获得简洁和失去详尽检查的适当权衡。

    // In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
    // 换句话说，你可以把if let看作是一个匹配的语法糖，当值匹配一个模式时运行代码，然后忽略所有其他值。

    // We can include an else with an if let.
    // 我们可以在if let中包含else。
    // The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else.
    // 与else匹配的代码块与匹配表达式中等价于if let和else的_ case对应的代码块相同。
    // Recall the Coin enum definition in Listing 6-4, where the Quarter variant also held a UsState value.
    // 回顾清单6-4中的Coin枚举定义，其中Quarter变体也包含UsState值。
    // If we wanted to count all non-quarter coins we see while also announcing the state of the quarters, we could do that with a match expression like this:
    // 如果我们想要计数我们看到的所有不是25美分硬币，同时报告25美分硬币所属的州，我们可以用匹配表达式（match expression）这样做:

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Or we could use an if let and else expression like this:
    // 或者我们可以像这样使用if let和else表达式:

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    // If you have a situation in which your program has logic that is too verbose to express using a match, remember that if let is in your Rust toolbox as well.
    // 如果你的程序逻辑过于冗长，无法使用匹配来表达，请记住，if let也在Rust工具箱中。

    // Summary
    // 总结
    // We’ve now covered how to use enums to create custom types that can be one of a set of enumerated values.
    // 我们已经介绍了如何使用枚举创建自定义类型，这些类型可以是一组枚举值之一。
    // We’ve shown how the standard library’s Option<T> type helps you use the type system to prevent errors.
    // 我们已经展示了标准库的Option<T>类型如何帮助您使用类型系统来防止错误。
    // When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.
    // 当枚举值中包含数据时，您可以使用match或if let来提取和使用这些值，这取决于您需要处理多少情况。

    // Your Rust programs can now express concepts in your domain using structs and enums.
    // 你的Rust程序现在可以在你的领域中使用结构和枚举来表达概念。
    // Creating custom types to use in your API ensures type safety: the compiler will make certain your functions get only values of the type each function expects.
    // 创建在API中使用的自定义类型确保类型安全:编译器将确保你的函数只获得每个函数所期望的类型的值。

    // In order to provide a well-organized API to your users that is straightforward to use and only exposes exactly what your users will need, let’s now turn to Rust’s modules.
    // 为了给你的用户提供一个组织良好的API，简单易用，只暴露你的用户需要的东西，现在让我们转向Rust的模块。

    println!("Hello, world!");
}
