fn main() {
    println!("Hello, world!");
    // Repetition with Loops
    // 使用循环重复
    // It’s often useful to execute a block of code more than once.
    // 多次执行一段代码通常很有用
    // For this task, Rust provides several loops,
    // 对于这个任务，Rust提供了几个循环，
    // which will run through the code inside the loop body to the end and then start immediately back at the beginning.
    // 它将遍历循环体中的代码直到末尾，然后立即从头开始。
    // To experiment with loops, let’s make a new project called loops.
    // 为了试验循环，我们创建一个名为loops的新项目。

    // Rust has three kinds of loops: loop, while, and for. Let’s try each one.
    // Rust有三种循环:loop, while和for。让我们逐一尝试一下。

    loop {
        println!("again!");

        break; // demo it once
    }

    // When we run this program, we’ll see again! printed over and over continuously until we stop the program manually.
    // 当我们运行这个程序时，我们会再次看到!一遍又一遍地打印，直到我们手动停止程序。
    // Most terminals support the keyboard shortcut ctrl-c to interrupt a program that is stuck in a continual loop. Give it a try:
    // 大多数终端都支持快捷键ctrl-c来中断陷入连续循环的程序。试一下:

    // The symbol ^C represents where you pressed ctrl-c .
    // 符号^C表示你按下ctrl-c的位置
    // You may or may not see the word again! printed after the ^C, depending on where the code was in the loop when it received the interrupt signal.
    // 你可能会也可能不会再看到这个单词!在^C之后打印，这取决于代码接收到中断信号时在循环中的位置。

    // Fortunately, Rust also provides a way to break out of a loop using code.
    // 幸运的是，Rust也提供了一种使用代码跳出循环的方法。
    // You can place the break keyword within the loop to tell the program when to stop executing the loop.
    // 可以在循环中使用break关键字，告诉程序何时停止执行循环。
    // Recall that we did this in the guessing game in the “Quitting After a Correct Guess” section of Chapter 2 to exit the program when the user won the game by guessing the correct number.
    // 回想一下，在第2章的“猜对后退出”一节中，我们就这么做了，当用户猜对数字赢得游戏时退出程序。

    // We also used continue in the guessing game, which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.
    // 我们在猜字游戏中还使用了continue，它在循环中告诉程序跳过这次循环中余下的代码，进入下一次循环。

    // Returning Values from Loops
    // 从循环中返回值
    // One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job.
    // 循环的用途之一是重试可能失败的操作，例如检查线程是否完成了它的工作。
    // You might also need to pass the result of that operation out of the loop to the rest of your code.
    // 你可能还需要将操作的结果传递给循环之外的代码。
    // To do this, you can add the value you want returned after the break expression you use to stop the loop;
    // 要做到这一点，你可以在用于停止循环的break表达式之后添加想要返回的值;
    // that value will be returned out of the loop so you can use it, as shown here:
    // 这个值会返回到循环外部，所以你可以像下面这样使用它:

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Before the loop, we declare a variable named counter and initialize it to 0.
    // 在循环之前，我们声明了一个名为counter的变量，并将其初始化为0。
    // Then we declare a variable named result to hold the value returned from the loop.
    // 然后声明一个名为result的变量来保存循环返回的值。
    // On every iteration of the loop, we add 1 to the counter variable, and then check whether the counter is equal to 10.
    // 在循环的每次迭代中，我们将变量counter加1，然后检查计数器是否等于10。
    // When it is, we use the break keyword with the value counter * 2.
    // 当它是，我们使用break关键字和值counter * 2。
    // After the loop, we use a semicolon to end the statement that assigns the value to result.
    // 在循环之后，我们使用分号来结束将值赋给result的语句。
    // Finally, we print the value in result, which in this case is 20.
    // 最后，我们打印result的值，在本例中为20。

    // Loop Labels to Disambiguate Between Multiple Loops
    // 用于在多个循环之间消除歧义的循环标签
    // If you have loops within loops, break and continue apply to the innermost loop at that point.
    // 如果在循环中有循环，break和continue将作用于最内层的循环。
    // You can optionally specify a loop label on a loop that we can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop.
    // 你可以选择在循环中指定一个循环标签，然后我们可以在break或continue中指定这些关键字应用于标记的循环而不是最内层的循环。
    // Loop labels must begin with a single quote. Here’s an example with two nested loops:
    // 循环标签必须以单引号开头下面是两个嵌套循环的例子:

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // The outer loop has the label 'counting_up, and it will count up from 0 to 2.
    // 外层循环有一个标签'counting_up '，它将从0计数到2。
    // The inner loop without a label counts down from 10 to 9.
    // 没有标签的内循环从10倒数到9
    // The first break that doesn’t specify a label will exit the inner loop only.
    // 第一个没有指定标签的break语句将只退出内层循环
    // The break 'counting_up; statement will exit the outer loop. This code prints:
    // break 'counting_up;语句将退出外循环。这段代码会打印:

    // sleetsecrets@lolita loops % cargo run
    //     Compiling loops v0.1.0 (/Users/sleetsecrets/Desktop/learn-rust-with-me/loops)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.56s
    //     Running `target/debug/loops`
    // ----snip----
    // count = 0
    // remaining = 10
    // remaining = 9
    // count = 1
    // remaining = 10
    // remaining = 9
    // count = 2
    // remaining = 10
    // End count = 2
    // sleetsecrets@lolita loops %

    // Conditional Loops with while
    // 使用while进行条件循环
    // A program will often need to evaluate a condition within a loop.
    // 程序经常需要计算循环中的条件。
    // While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop.
    // 只要条件为真，循环就会运行。当条件不为真时，程序调用break，停止循环。
    // It’s possible to implement behavior like this using a combination of loop, if, else, and break;
    // 结合使用loop、if、else和break可以实现这种行为;
    // you could try that now in a program, if you’d like. However, this pattern is so common that Rust has a built-in language construct for it,
    // 如果你愿意，现在可以在程序中试试看。然而，这种模式如此常见，以至于Rust为它提供了一个内置的语言结构，
    //  called a while loop. In Listing 3-3, we use while to loop the program three times, counting down each time, and then, after the loop, print a message and exit.
    // 调用while循环。在代码清单3-3中，我们使用while循环3次，每次都倒数，然后在循环结束后打印一条消息并退出。
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    // Listing 3-3: Using a while loop to run code while a condition holds true
    // 代码清单3-3:在条件为真时，使用while循环运行代码

    // This construct eliminates a lot of nesting that would be necessary if you used loop, if, else, and break, and it’s clearer.
    // 这种结构消除了很多在使用loop、if、else和break时必须使用的嵌套，而且结构更清晰。
    // While a condition holds true, the code runs; otherwise, it exits the loop.
    // 只要条件为真，代码就会运行;否则，退出循环。

    // Looping Through a Collection with for
    // 使用for循环遍历集合
    // You can choose to use the while construct to loop over the elements of a collection, such as an array.
    // 你可以选择使用while结构来遍历集合(例如数组)中的元素。
    // For example, the loop in Listing 3-4 prints each element in the array a.
    // 例如，代码清单3-4中的循环会打印数组a中的每个元素。

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    // Listing 3-4: Looping through each element of a collection using a while loop
    // 代码清单3-4:使用while循环遍历集合中的每个元素

    // Here, the code counts up through the elements in the array. It starts at index 0, and then loops until it reaches the final index in the array (that is, when index < 5 is no longer true).
    // 这里，代码对数组中的元素进行了计数。它从索引0开始，循环直到到达数组的最后一个索引(即索引< 5不再为true)。
    // Running this code will print every element in the array:
    // 运行这段代码将打印出数组中的每个元素:
    // sleetsecrets@lolita loops % cargo run
    //     Compiling loops v0.1.0 (/Users/sleetsecrets/Desktop/learn-rust-with-me/loops)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.70s
    //     Running `target/debug/loops`
    // ----snip----
    // the value is: 10
    // the value is: 20
    // the value is: 30
    // the value is: 40
    // the value is: 50
    // sleetsecrets@lolita loops %

    // All five array values appear in the terminal, as expected.
    // 正如预期的那样，终端显示了所有的5个数组值
    // Even though index will reach a value of 5 at some point, the loop stops executing before trying to fetch a sixth value from the array.
    // 即使index的值在某个时候会达到5，循环在试图从数组中获取第六个值之前就停止了。

    // However, this approach is error prone; we could cause the program to panic if the index value or test condition are incorrect.
    // 然而，这种方法很容易出错;如果索引值或测试条件不正确，就可能导致程序出错。
    // For example,
    // 例如:
    // if you changed the definition of the a array to have four elements but forgot to update the condition to while index < 4, the code would panic. It’s also slow,
    // 如果你修改了数组的定义，使其包含4个元素，但忘记将条件更新为while index < 4，代码就会出错。它也很慢，
    // because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.
    // 因为编译器添加了运行时代码，在每次循环迭代时执行条件检查索引是否在数组的边界内。

    // As a more concise alternative, you can use a for loop and execute some code for each item in a collection.
    // 更简洁的做法是使用for循环，为集合中的每个元素执行一些代码。
    // A for loop looks like the code in Listing 3-5.
    // 一个for循环代码如代码清单3-5所示。

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    // Listing 3-5: Looping through each element of a collection using a for loop
    // 代码清单3-5:使用for循环遍历集合中的每个元素

    // When we run this code, we’ll see the same output as in Listing 3-4.
    // 运行这段代码，可以看到与代码清单3-4相同的输出。
    // More importantly, we’ve now increased the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.
    // 更重要的是，我们现在已经提高了代码的安全性，并消除了可能因超出数组末尾或走得不够远或遗漏某些项而导致bug的可能性。

    // Using the for loop, you wouldn’t need to remember to change any other code if you changed the number of values in the array, as you would with the method used in Listing 3-4.
    // 使用for循环，如果改变数组中值的个数，就不需要像代码清单3-4中的方法那样记着修改其他代码。

    // The safety and conciseness of for loops make them the most commonly used loop construct in Rust.
    // for循环的安全性和简洁性使其成为Rust中最常用的循环结构。
    // Even in situations in which you want to run some code a certain number of times, as in the countdown example that used a while loop in Listing 3-3, most Rustaceans would use a for loop.
    // 即使在你想运行某些代码一定次数的情况下，比如代码清单3-3中使用while循环的countdown例子，大多数rustacean人也会使用for循环。
    // The way to do that would be to use a Range, provided by the standard library, which generates all numbers in sequence starting from one number and ending before another number.
    // 要做到这一点，可以使用标准库提供的范围，它按顺序生成从一个数字开始，在另一个数字之前结束的所有数字。

    // Here’s what the countdown would look like using a for loop and another method we’ve not yet talked about, rev, to reverse the range:
    // 下面是使用for循环和另一个我们尚未讨论的方法rev来反转范围时的倒计时:

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    // This code is a bit nicer, isn’t it?

    // Summary
    // You made it! That was a sizable chapter: you learned about variables, scalar and compound data types, functions, comments, if expressions, and loops! To practice with the concepts discussed in this chapter, try building programs to do the following:

    // Convert temperatures between Fahrenheit and Celsius.
    // Generate the nth Fibonacci number.
    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
    // When you’re ready to move on, we’ll talk about a concept in Rust that doesn’t commonly exist in other programming languages: ownership.
    let factorial_result = factorial(5);
    println!("The value of factorial is: {factorial_result}");
}

fn factorial_impl(n: i32, total: i32) -> i32 {
    if n == 1 {
        return total;
    }
    factorial_impl(n - 1, n * total)
}

fn factorial(n: i32) -> i32 {
    factorial_impl(n, 1)
}
