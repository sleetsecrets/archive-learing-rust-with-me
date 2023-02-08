fn main() {
    // All programmers strive to make their code easy to understand, but sometimes extra explanation is warranted.
    // 所有程序员都努力使代码易于理解，但有时需要额外的解释。
    // In these cases, programmers leave comments in their source code that the compiler will ignore but people reading the source code may find useful.
    // 在这种情况下，程序员在源代码中留下的注释会被编译器忽略，但阅读源代码的人可能会发现有用的注释。

    // Here’s a simple comment:
    // 下面是一条简单的注释:

    // hello, world

    // In Rust, the idiomatic comment style starts a comment with two slashes, and the comment continues until the end of the line.
    // 在Rust中，惯用的注释风格以两个斜杠开始，注释一直持续到行尾。
    // For comments that extend beyond a single line, you’ll need to include // on each line, like this:
    // 对于超出一行的注释，你需要在每一行都包含它，像这样:

    // So we’re doing something complicated here, long enough that we need
    // multiple lines of comments to do it! Whew! Hopefully, this comment will
    // explain what’s going on.

    // Comments can also be placed at the end of lines containing code:
    // 注释也可以放在包含代码的行末尾:
    let lucky_number = 7; // I’m feeling lucky today

    // But you’ll more often see them used in this format, with the comment on a separate line above the code it’s annotating:
    // 但你更经常看到它们以这种格式使用，注释在其注释代码的上方的单独一行中:

    // I’m feeling lucky today
    let lucky_number = 7;

    // Rust also has another kind of comment, documentation comments, which we’ll discuss in the “Publishing a Crate to Crates.io” section of Chapter 14.
    // Rust还有另一种注释，文档注释，我们将在13.2节中讨论。io“第14章的一部分。
}
