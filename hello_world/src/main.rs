fn main() {
    // First, Rust style is to indent with four spaces, not a tab.
    // 首先，Rust 风格是用四个空格缩进，而不是制表符。
    // Second, println! calls a Rust macro. If it had called a function instead, it would be entered as println (without the !).
    // 其次，println! 调用 Rust 宏。 如果它调用了一个函数，它将作为 println 输入（不带 !）。
    // We’ll discuss Rust macros in more detail in Chapter 19.
    // 我们将在第 19 章更详细地讨论 Rust 宏。
    // For now, you just need to know that using a ! means that you’re calling a macro instead of a normal function, and that macros don’t always follow the same rules as functions.
    // 现在，你只需要知道使用 ! 意味着您正在调用宏而不是普通函数，并且宏并不总是遵循与函数相同的规则。
    println!("Hello, world!");
    // Third, you see the "Hello, world!" string. We pass this string as an argument to println!, and the string is printed to the screen.
    // 第三，你看到“Hello, world!” 细绳。 我们将此字符串作为参数传递给 println!，然后将字符串打印到屏幕上。
    // Fourth, we end the line with a semicolon (;), which indicates that this expression is over and the next one is ready to begin.
    // 第四，我们以分号 (;) 结束该行，表示此表达式已结束，下一个已准备好开始。
    // Most lines of Rust code end with a semicolon.
    // 大多数 Rust 代码行以分号结尾。
}
