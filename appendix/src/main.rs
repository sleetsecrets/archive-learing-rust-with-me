// Appendix A: Keywords
// 附录A:关键字
// The following list contains keywords that are reserved for current or future use by the Rust language.
// 下面的列表包含了Rust语言为当前或将来使用保留的关键字。
// As such, they cannot be used as identifiers (except as raw identifiers as we’ll discuss in the “Raw Identifiers” section).
// 因此，它们不能用作标识符(除了我们将在“raw标识符”部分讨论的原始标识符)。
// Identifiers are names of functions, variables, parameters, struct fields, modules, crates, constants, macros, static values, attributes, types, traits, or lifetimes.
// 标识符是函数、变量、参数、结构字段、模块、板条箱、常量、宏、静态值、属性、类型、特征或生命周期的名称。

// Keywords Currently in Use
//当前正在使用的关键字
// The following is a list of keywords currently in use, with their functionality described.
//下面是当前使用的关键字列表，并描述了它们的功能。

// * as - perform primitive casting, disambiguate the specific trait containing an item, or rename items in use statements
//      - 执行原语强制转换，消除包含项的特定trait的歧义，或重命名use语句中的项
// * async - return a Future instead of blocking the current thread
//         - 返回Future而不是阻塞当前线程
// * await - suspend execution until the result of a Future is ready
//         - 暂停执行，直到Future的结果准备好
// * break - exit a loop immediately
//         - 立即退出循环
// * const - define constant items or constant raw pointers
//         - 定义常量项或常量原始指针
// * continue - continue to the next loop iteration
//            - 继续到下一次循环迭代
// * crate - in a module path, refers to the crate root
//         - 在模块路径中，指的是crate根目录
// * dyn - dynamic dispatch to a trait object
//       - 动态调度trait对象
// * else - fallback for if and if let control flow constructs
//        - if和if let控制流结构的回退
// * enum - define an enumeration
//        - 定义一个枚举
// * extern - link an external function or variable
//          - 链接外部函数或变量
// * false - Boolean false literal
//         - 布尔值的假文字
// * fn - define a function or the function pointer type
//      - 定义一个函数或函数指针类型
// * for - loop over items from an iterator, implement a trait, or specify a higher-ranked lifetime
//       - 遍历迭代器中的项，实现trait，或指定更高级别的生命期
// * if - branch based on the result of a conditional expression
//      - 基于条件表达式结果的分支
// * impl - implement inherent or trait functionality
//        - 实现固有的或特征的功能
// * in - part of for loop syntax
//      - 循环语法的一部分
// * let - bind a variable
//       - 绑定变量
// * loop - loop unconditionally
//        - 无条件循环
// * match - match a value to patterns
//         - 将值与模式匹配
// * mod - define a module
//       - 定义一个模块
// * move - make a closure take ownership of all its captures
//        - 让闭包拥有它捕获的所有内容
// * mut - denote mutability in references, raw pointers, or pattern bindings
//       - 在引用、原始指针或模式绑定中表示可变性
// * pub - denote public visibility in struct fields, impl blocks, or modules
//       - 在struct字段、impl块或模块中表示公共可见性
// * ref - bind by reference
//       - 引用绑定
// * return - return from function
//          - 从函数返回
// * Self - a type alias for the type we are defining or implementing
//        - 定义或实现的类型的类型别名
// * self - method subject or current module
//        - 方法主题或当前模块
// * static - global variable or lifetime lasting the entire program execution
//          - 全局变量或持续整个程序执行的生命周期
// * struct - define a structure
//          - 定义结构
// * super - parent module of the current module
//         - 当前模块的父模块
// * trait - define a trait
//         - 定义一个特征
// * true - Boolean true literal
//        - 布尔值真字面量
// * type - define a type alias or associated type
//        - 定义类型别名或关联类型
// * union - define a union; is only a keyword when used in a union declaration
//         - 定义一个联合; 只有在联合声明中使用时才是关键字吗
// * unsafe - denote unsafe code, functions, traits, or implementations
//          - 标记不安全的代码、函数、特征或实现
// * use - bring symbols into scope
//       - 将符号引入范围
// * where - denote clauses that constrain a type
//         - 表示约束类型的子句
// * while - loop conditionally based on the result of an expression
//         - 根据表达式的结果有条件地循环

// Keywords Reserved for Future Use
// 保留以后使用的关键字
// The following keywords do not yet have any functionality but are reserved by Rust for potential future use.
// 以下关键字还没有任何功能，但是Rust保留了这些关键字以备将来使用。

// * abstract
// * become
// * box
// * do
// * final
// * macro
// * override
// * priv
// * try
// * typeof
// * unsized
// * virtual
// * yield

// Raw Identifiers
// 原始标识符
// Raw identifiers are the syntax that lets you use keywords where they wouldn’t normally be allowed. You use a raw identifier by prefixing a keyword with r#.
// 原始标识符是允许你在通常不允许的地方使用关键字的语法。通过在关键字前面加上r#来使用原始标识符。

// For example, match is a keyword. If you try to compile the following function that uses match as its name:
// 例如，match是关键字。如果你试图编译以下使用match作为其名称的函数:

// Filename: src/main.rs

fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

// you’ll get this error:
// 你会得到这样的错误:

// error: expected identifier, found keyword `match`
//  --> src/main.rs:4:4
//   |
// 4 | fn match(needle: &str, haystack: &str) -> bool {
//   |    ^^^^^ expected identifier, found keyword

// The error shows that you can’t use the keyword match as the function identifier.
// 错误显示不能使用关键字match作为函数标识符。
// To use match as a function name, you need to use the raw identifier syntax, like this:
// 使用match作为函数名，你需要使用原始标识符语法，像这样:

// Filename: src/main.rs

fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}

// This code will compile without any errors. Note the r# prefix on the function name in its definition as well as where the function is called in main.
// 此代码将编译而不会出现任何错误。注意函数定义中函数名上的r#前缀，以及main中函数被调用的位置。

// Raw identifiers allow you to use any word you choose as an identifier, even if that word happens to be a reserved keyword.
// 原始标识符允许您使用任何您选择的单词作为标识符，即使该单词恰好是保留关键字。
// This gives us more freedom to choose identifier names, as well as lets us integrate with programs written in a language where these words aren’t keywords.
// 这给了我们更多选择标识符名称的自由，以及让我们与用这些词不是关键字的语言编写的程序集成。
// In addition, raw identifiers allow you to use libraries written in a different Rust edition than your crate uses.
// 另外，raw标识符允许你使用不同Rust版本的库。
// For example, try isn’t a keyword in the 2015 edition but is in the 2018 edition.
// 例如，try在2015版中没有关键字，但在2018版中有。
// If you depend on a library that’s written using the 2015 edition and has a try function, you’ll need to use the raw identifier syntax, r#try in this case,
// 如果你依赖于一个使用2015版编写的库，并且有一个try函数，你将需要使用原始标识符语法，在这种情况下r#try，
//  to call that function from your 2018 edition code.
//  从2018版代码中调用该函数。
// See Appendix E for more information on editions.
// 关于版本的更多信息请参见附录E。

// Appendix B: Operators and Symbols
// 附录B:操作符和符号
// This appendix contains a glossary of Rust’s syntax,
// 这个附录包含Rust语法的词汇表，
//  including operators and other symbols that appear by themselves or in the context of paths, generics, trait bounds, macros, attributes, comments, tuples, and brackets.
//  包括操作符和其他单独出现或在路径、泛型、trait bounds、宏、属性、注释、元组和括号上下文中出现的符号。

// Operators
// 运算符
// Table B-1 contains the operators in Rust, an example of how the operator would appear in context, a short explanation, and whether that operator is overloadable.
// 表B-1包含了Rust中的操作符，一个操作符如何在上下文中出现的例子，一个简短的解释，以及该操作符是否可重载。
// If an operator is overloadable, the relevant trait to use to overload that operator is listed.
// 如果一个操作符是可重载的，将列出用于重载该操作符的相关trait。

// Table B-1: Operators

// Operator     Example                 Explanation                         Overloadable?
// !	        ident!(...),            Macro expansion
//              ident!{...},            宏扩展
//              ident![...]
//
// !            !expr	                Bitwise or logical complement	    Not
//                                      按位或逻辑补码
//
// !=	        expr != expr	        Nonequality comparison	            PartialEq
//                                      异的比较
//
// %	        expr % expr	            Arithmetic remainder	            Rem
//                                      算术剩余（余数）
//
// %=	        var %= expr	            Arithmetic remainder and            RemAssign
//                                      assignment
//                                      算术余数和赋值

// &            &expr, &mut expr	    Borrow
//                                      借用
//
// &            &type, &mut type,       Borrowed pointer type
//              &'a type, &'a mut type  借用指针类型
//
// &	        expr & expr	            Bitwise AND	                        BitAnd
//                                      位与
//
// &=	        var &= expr	            Bitwise AND and assignment	        BitAndAssign
//                                      位与和赋值
//
// &&	        expr && expr	        Short-circuiting logical AND
//                                      逻辑与短路
//
// *	        expr * expr	            Arithmetic multiplication	        Mul
//                                      乘法运算
//
// *=	        var *= expr	            Arithmetic multiplication and       MulAssign
//                                      assignment
//                                      算术乘法和赋值
//
// *	        *expr	                Dereference	                        Deref
//                                      解引用
//
// *            *const type, *mut type	Raw pointer
//                                      原始指针
//
// +            trait + trait,          Compound type constraint
//              'a + trait              复合类型约束
//
// +            expr + expr	            Arithmetic addition
//                                      算术加法
//
// +=	        var += expr	            Arithmetic addition and assignment	AddAssign
//                                      算数加法和赋值
//
// ,            expr, expr	            Argument and element separator
//                                      参数和元素分隔符
//
// -            - expr	                Arithmetic negation	                Neg
//                                      算术否定
//
// -            expr - expr	            Arithmetic subtraction	            Sub
//                                      算术减法
//
// -=	        var -= expr	            Arithmetic subtraction and          SubAssign
//                                      assignment
//                                      算数减法和赋值
//
// ->	        fn(...) -> type,        Function and closure return type
//              |...| -> type           函数和闭包返回类型
//
//
// .            expr.ident	            Member access
//                                      成员访问
//
// ..	        .., expr.., ..expr,     Right-exclusive range literal	    PartialOrd
//              expr..expr              右专属范围文字
//
// ..=	        ..=expr, expr..=expr	Right-inclusive range literal	    PartialOrd
//                                      右包含范围文字
//
// ..	        ..expr	                Struct literal update syntax
//                                      结构字面值更新语法
//
// ..	        variant(x, ..),         “And the rest” pattern binding
//              struct_type { x, .. }   “其余”模式绑定
//
// ...	        expr...expr	            (Deprecated, use ..= instead)
//                                      In a pattern:
//                                      inclusive range pattern
//
// /	        expr / expr	            Arithmetic division	                Div
//                                      算数除法
//
// /=	        var /= expr	            Arithmetic division and assignment	DivAssign
//                                      算数除法和赋值
//
// :            pat: type, ident: type	Constraints
//                                      约束
//
// :            ident: expr	            Struct field initializer
//                                      Struct字段初始化器
//
// :	        'a: loop {...}	        Loop label
//                                      循环标签
//
// ;            expr;	                Statement and item terminator
//                                      语句和项结束符
//
// ;            [...; len]	            Part of fixed-size array syntax
//                                      固定大小数组语法的一部分
//
// <<	        expr << expr	        Left-shift	                        Shl
//                                      左移
//
// <<=	        var <<= expr	        Left-shift and assignment	        ShlAssign
//                                      左移和赋值
//
// <            expr < expr	            Less than comparison	            PartialOrd
//                                      小于比较
//
// <=	        expr <= expr	        Less than or equal to comparison	PartialOrd
//                                      小于或等于比较
//
// =            var = expr,             Assignment/equivalence
//              ident = type            分配/等价
//
// ==	        expr == expr	        Equality comparison	                PartialEq
//                                      平等的比较
//
// =>	        pat => expr	            Part of match arm syntax
//
//
// >            expr > expr	            Greater than comparison	            PartialOrd
//                                      大于比较
//
// >=	        expr >= expr	        Greater than or equal to comparison	PartialOrd
//                                      大于或等于比较
//
// >>	        expr >> expr	        Right-shift	                        Shr
//                                      右移
//
// >>=	        var >>= expr	        Right-shift and assignment	        ShrAssign
//                                      右移和赋值
//
// @            ident @ pat	            Pattern binding
//                                      模式绑定
//
// ^	        expr ^ expr	            Bitwise exclusive OR	            BitXor
//                                      位异或
//
// ^=	        var ^= expr	            Bitwise exclusive OR and assignment	BitXorAssign
//                                      位异或和赋值
//
// |            pat | pat	            Pattern alternatives
//                                      模式的选择
//
// |            expr | expr	            Bitwise OR	                        BitOr
//                                      按位或
//
// |=	        var |= expr	            Bitwise OR and assignment	        BitOrAssign
//                                      按位或和赋值
//
// ||	        expr || expr	        Short-circuiting logical OR
//                                      逻辑或短路
//
// ?	        expr?	                Error propagation
//                                      错误传播

// Non-operator Symbols
// 非操作符符号
// The following list contains all symbols that don’t function as operators; that is, they don’t behave like a function or method call.
// 以下列表包含所有不能作为操作符的符号;也就是说，它们的行为不像函数或方法调用。

// Table B-2 shows symbols that appear on their own and are valid in a variety of locations.
// 表B-2显示了单独出现并且在不同位置都有效的符号。

// Table B-2: Stand-Alone Syntax
// 表B-2:单机语法

// Symbol	                                Explanation
// 'ident	                                Named lifetime or loop label
//                                          命名的生命周期或循环标签
// ...u8, ...i32, ...f64, ...usize, etc.	Numeric literal of specific type
//                                          特定类型的数字字面值
// "..."	                                String literal
//                                          字符串文字
// r"...", r#"..."#, r##"..."##, etc.	    Raw string literal, escape characters not processed
//                                          原始字符串文字，转义字符未处理
// b"..."	                                Byte string literal; constructs an array of bytes instead of a string
//                                          字节字符串文字;构造字节数组而不是字符串
// br"...", br#"..."#, br##"..."##, etc.	Raw byte string literal, combination of raw and byte string literal
//                                          原始字节字符串字面量，原始和字节字符串字面量的组合
// '...'	                                Character literal
//                                          字符文字
// b'...'	                                ASCII byte literal
//                                          ASCII字节字面值
// |...| expr	                            Closure
//                                          闭包
// !	                                    Always empty bottom type for diverging functions
//                                          发散函数的底部类型始终为空
// _                                        “Ignored” pattern binding; also used to make integer literals readable
//                                          “被忽略”的模式绑定;也用于使整数字面量可读

// Table B-3 shows symbols that appear in the context of a path through the module hierarchy to an item.
// 表B-3显示了出现在从模块层次结构到项目的路径上下文中的符号。

// Table B-3: Path-Related Syntax
// 表B-3:路径相关语法
// Symbol	                                Explanation
// ident::ident	                            Namespace path
//                                          名称空间路径
// ::path	                                Path relative to the crate root (i.e., an explicitly absolute path)
//                                          相对于板条箱根的路径(即显式的绝对路径)
// self::path	                            Path relative to the current module (i.e., an explicitly relative path).
//                                          相对于当前模块的路径(即显式相对路径)。
// super::path	                            Path relative to the parent of the current module
//                                          相对于当前模块的父模块的路径
// type::ident, <type as trait>::ident	    Associated constants, functions, and types
//                                          相关的常量、函数和类型
// <type>::...	                            Associated item for a type that cannot be directly named (e.g., <&T>::..., <[T]>::..., etc.)
//                                          不能直接命名的类型的关联项(例如，<&T>::…), < [T] >::…等)。
// trait::method(...)	                    Disambiguating a method call by naming the trait that defines it
//                                          通过命名定义方法调用的特征来消除方法调用的歧义
// type::method(...)	                    Disambiguating a method call by naming the type for which it’s defined
//                                          通过命名定义方法调用的类型来消除方法调用的歧义
// <type as trait>::method(...)	            Disambiguating a method call by naming the trait and type
//                                          通过命名特征和类型来消除方法调用的歧义

// Table B-4 shows symbols that appear in the context of using generic type parameters.
// 表B-4显示了在使用泛型类型参数的上下文中出现的符号。

// Table B-4: Generics
// 表B-4:泛型
// Symbol	                                Explanation
// path<...>	                            Specifies parameters to generic type in a type (e.g., Vec<u8>)
//                                          指定类型中泛型类型的参数(例如，Vec<u8>)
// path::<...>, method::<...>	            Specifies parameters to generic type, function, or method in an expression; often referred to as turbofish (e.g., "42".parse::<i32>())
//                                          为表达式中的泛型类型、函数或方法指定参数;通常被称为turbofish(例如，"42".parse::<i32>())
// fn ident<...> ...	                    Define generic function
//                                          定义泛型函数
// struct ident<...> ...	                Define generic structure
//                                          定义通用结构
// enum ident<...> ...	                    Define generic enumeration
//                                          定义泛型枚举
// impl<...> ...	                        Define generic implementation
//                                          定义泛型实现
// for<...> type	                        Higher-ranked lifetime bounds
//                                          更高等级的生命界
// type<ident=type>	                        A generic type where one or more associated types have specific assignments (e.g., Iterator<Item=T>)
//                                          一个泛型类型，其中一个或多个关联类型具有特定赋值(例如Iterator<Item=T>)

// Table B-5 shows symbols that appear in the context of constraining generic type parameters with trait bounds.
// 表B-5显示了在用trait边界约束泛型类型参数时出现的符号。

// Table B-5: Trait Bound Constraints
// 表B-5: Trait约束
// Symbol	                                Explanation
// T: U	                                    Generic parameter T constrained to types that implement U
//                                          泛型参数T被约束为实现U的类型
// T: 'a	                                Generic type T must outlive lifetime 'a (meaning the type cannot transitively contain any references with lifetimes shorter than 'a)
//                                          泛型类型T必须活得比生命周期'a更长(意味着该类型不能传递地包含任何生命周期短于'a的引用)
// T: 'static	                            Generic type T contains no borrowed references other than 'static ones
//                                          泛型类型T除了“静态”引用外不包含任何借来的引用
// 'b: 'a	                                Generic lifetime 'b must outlive lifetime 'a
//                                          一般的生命周期b必须比生命周期a活得长
// T: ?Sized	                            Allow generic type parameter to be a dynamically sized type
//                                          允许泛型类型参数为动态大小的类型
// 'a + trait, trait + trait	            Compound type constraint
//                                          复合类型约束

// Table B-6 shows symbols that appear in the context of calling or defining macros and specifying attributes on an item.
// 表B-6显示了在调用或定义宏和指定项属性时出现的符号。

// Table B-6: Macros and Attributes
// 表B-6:宏和属性
// Symbol	                                Explanation
// #[meta]	                                Outer attribute
//                                          外部属性
// #![meta]	                                Inner attribute
//                                          内在属性
// $ident	                                Macro substitution
//                                          宏替换
// $ident:kind	                            Macro capture
//                                          宏捕获
// $(…)…	                                Macro repetition
//                                          宏重复
// ident!(...), ident!{...}, ident![...]	Macro invocation
//                                          宏调用

// Table B-7 shows symbols that create comments.
// 表B-7显示了创建注释的符号。

// Table B-7: Comments
// 表B-7:注释
// Symbol	                                Explanation
// //	                                    Line comment
//                                          行注释
// //!	                                    Inner line doc comment
//                                          内部行文档注释
// ///	                                    Outer line doc comment
//                                          外部行文档注释
// /*...*/	                                Block comment
//                                          块注释
// /*!...*/	                                Inner block doc comment
//                                          内部块文档注释
// /**...*/	                                Outer block doc comment
//                                          外部块文档注释

// Table B-8 shows symbols that appear in the context of using tuples.
// 表B-8显示了在使用元组的上下文中出现的符号。

// Table B-8: Tuples
// 表B-8:元组
// Symbol	                                Explanation
// ()	                                    Empty tuple (aka unit), both literal and type
//                                          空元组(又名单元)，包括文字和类型
// (expr)	                                Parenthesized expression
//                                          括号表达式
// (expr,)	                                Single-element tuple expression
//                                          单元素元组表达式
// (type,)	                                Single-element tuple type
//                                          单元素元组类型
// (expr, ...)	                            Tuple expression
//                                          元组表达
// (type, ...)	                            Tuple type
//                                          Tuple类型
// expr(expr, ...)	                        Function call expression; also used to initialize tuple structs and tuple enum variants
//                                          函数调用表达式;也用于初始化元组结构和元组枚举变量
// expr.0, expr.1, etc.	                    Tuple indexing
//                                          元组索引

// Table B-9 shows the contexts in which curly braces are used.
// 表B-9显示了大括号的使用上下文。

// Table B-9: Curly Brackets
// 表B-9:括号
// Context	                                    Explanation
// [...]	                                    Array literal
//                                              数组文字量
// [expr; len]	                                Array literal containing len copies of expr
//                                              数组字面量，包含expr的len副本
// [type; len]	                                Array type containing len instances of type
//                                              数组类型，包含类型的len实例
// expr[expr]	                                Collection indexing. Overloadable (Index, IndexMut)
//                                              收集索引。可重载(Index, IndexMut)
// expr[..], expr[a..], expr[..b], expr[a..b]	Collection indexing pretending to be collection slicing, using Range, RangeFrom, RangeTo, or RangeFull as the “index”
//                                              集合索引假装是集合切片，使用Range、RangeFrom、RangeTo或RangeFull作为“索引”

// Appendix C: Derivable Traits
// 附录C:衍生Trait
// In various places in the book, we’ve discussed the derive attribute, which you can apply to a struct or enum definition.
// 在本书的很多地方，我们已经讨论了派生属性，你可以将其应用于结构体或枚举定义。
// The derive attribute generates code that will implement a trait with its own default implementation on the type you’ve annotated with the derive syntax.
// 派生属性生成代码，该代码将在使用派生语法注释的类型上使用其自己的默认实现实现trait。

// In this appendix, we provide a reference of all the traits in the standard library that you can use with derive. Each section covers:
// 在这个附录中，我们提供了标准库中所有可以使用派生的trait的参考。每一节包括:

// * What operators and methods deriving this trait will enable
// * 派生该trait的操作符和方法将启用什么
// * What the implementation of the trait provided by derive does
// * 派生提供的trait的实现
// * What implementing the trait signifies about the type
// * 实现trait对类型意味着什么
// * The conditions in which you’re allowed or not allowed to implement the trait
// * 允许或不允许实现trait的条件
// * Examples of operations that require the trait
// * 需要trait的操作示例

// If you want different behavior from that provided by the derive attribute,
// 如果你想要不同于派生属性提供的行为，
//  consult the standard library documentation for each trait for details of how to manually implement them.
//  参考每个trait的标准库文档，了解如何手动实现它们。

// These traits listed here are the only ones defined by the standard library that can be implemented on your types using derive.
// 这里列出的这些特征是标准库定义的唯一可以使用派生在类型上实现的特征。
// Other traits defined in the standard library don’t have sensible default behavior, so it’s up to you to implement them in the way that makes sense for what you’re trying to accomplish.
// 在标准库中定义的其他特征没有合理的默认行为，所以由你来实现它们的方式对你试图完成的事情有意义。

// An example of a trait that can’t be derived is Display, which handles formatting for end users.
// 不能派生的trait的一个例子是Display，它为最终用户处理格式化。
// You should always consider the appropriate way to display a type to an end user.
// 您应该始终考虑向最终用户显示类型的适当方式。
// What parts of the type should an end user be allowed to see? What parts would they find relevant?
// 最终用户可以看到类型的哪些部分?他们觉得哪些部分是相关的?
// What format of the data would be most relevant to them?
// 什么格式的数据最适合他们?
// The Rust compiler doesn’t have this insight, so it can’t provide appropriate default behavior for you.
//  Rust编译器没有这种洞察力，所以它不能为你提供适当的默认行为。

// The list of derivable traits provided in this appendix is not comprehensive:
// 本附录中提供的衍生Trait列表并不全面:
//  libraries can implement derive for their own traits, making the list of traits you can use derive with truly open-ended.
//  库可以为自己的trait实现派生，使你可以使用派生的trait列表真正开放。
// Implementing derive involves using a procedural macro, which is covered in the “Macros” section of Chapter 19.
// 实现derive需要使用一个过程宏，这将在第19章的“宏”一节中介绍。

// Debug for Programmer Output
// 调试程序输出
// The Debug trait enables debug formatting in format strings, which you indicate by adding :? within {} placeholders.
//  Debug特性在格式字符串中启用调试格式，可以通过添加:?在{}占位符中。

// The Debug trait allows you to print instances of a type for debugging purposes,
// Debug特性允许你打印一个类型的实例用于调试，
//  so you and other programmers using your type can inspect an instance at a particular point in a program’s execution.
// 这样你和其他使用你的类型的程序员就可以在程序执行的特定点检查实例。

// The Debug trait is required, for example, in use of the assert_eq! macro.
//  Debug特征是必需的，例如，在使用assert_eq!宏。
// This macro prints the values of instances given as arguments if the equality assertion fails so programmers can see why the two instances weren’t equal.
// 如果相等断言失败，这个宏将打印实例的值作为参数，这样程序员就可以看到为什么两个实例不相等。

// PartialEq and Eq for Equality Comparisons
// 用于相等比较的PartialEq和Eq
// The PartialEq trait allows you to compare instances of a type to check for equality and enables use of the == and != operators.
// PartialEq trait允许你比较一个类型的实例来检查是否相等，并允许使用==和!=操作符。

// Deriving PartialEq implements the eq method.
// 导出PartialEq实现eq方法。
// When PartialEq is derived on structs, two instances are equal only if all fields are equal, and the instances are not equal if any fields are not equal.
// 当在struct上派生PartialEq时，只有当所有字段都相等时，两个实例才相等，如果任何字段不相等，则两个实例不相等。
// When derived on enums, each variant is equal to itself and not equal to the other variants.
// 当在枚举上派生时，每个变量等于它自己而不等于其他变量。

// The PartialEq trait is required, for example, with the use of the assert_eq! macro, which needs to be able to compare two instances of a type for equality.
// PartialEq trait是必需的，例如使用assert_eq!宏，它需要能够比较一个类型的两个实例是否相等。

// The Eq trait has no methods.
// Eq这个特质是没有方法的。
// Its purpose is to signal that for every value of the annotated type, the value is equal to itself.
// 它的目的是表明对于注释类型的每个值，该值都等于它本身。
// The Eq trait can only be applied to types that also implement PartialEq, although not all types that implement PartialEq can implement Eq.
// Eq特征只能应用于实现PartialEq的类型，尽管不是所有实现PartialEq的类型都可以实现Eq。
// One example of this is floating point number types: the implementation of floating point numbers states that two instances of the not-a-number (NaN) value are not equal to each other.
// 一个例子是浮点数类型:浮点数的实现表明not-a-number (NaN)值的两个实例彼此不相等。

// An example of when Eq is required is for keys in a HashMap<K, V> so the HashMap<K, V> can tell whether two keys are the same.
//一个需要Eq的例子是HashMap<K, V>中的键，因此HashMap<K, V>可以判断两个键是否相同。

// PartialOrd and Ord for Ordering Comparisons
// PartialOrd和Ord排序比较
// The PartialOrd trait allows you to compare instances of a type for sorting purposes.
// PartialOrd trait允许你比较一个类型的实例来排序。
// A type that implements PartialOrd can be used with the <, >, <=, and >= operators.
// 实现PartialOrd的类型可以使用<，>，<=和>=操作符。
// You can only apply the PartialOrd trait to types that also implement PartialEq.
// 你只能将PartialOrd特性应用到实现了PartialEq的类型上。

// Deriving PartialOrd implements the partial_cmp method, which returns an Option<Ordering> that will be None when the values given don’t produce an ordering.
// 派生PartialOrd实现partial_cmp方法，该方法返回一个Option<Ordering>，当给定的值不产生排序时，该选项将为None。
// An example of a value that doesn’t produce an ordering, even though most values of that type can be compared, is the not-a-number (NaN) floating point value.
// 一个不产生排序的值的例子，即使该类型的大多数值都可以进行比较，是非数字(NaN)浮点值。
// Calling partial_cmp with any floating point number and the NaN floating point value will return None.

// When derived on structs, PartialOrd compares two instances by comparing the value in each field in the order in which the fields appear in the struct definition.
// 当在struct上派生时，PartialOrd通过比较每个字段的值来比较两个实例，这些字段在结构定义中出现的顺序。
// When derived on enums, variants of the enum declared earlier in the enum definition are considered less than the variants listed later.
// 当在枚举上派生时，在枚举定义中前面声明的枚举的变量被认为比后面列出的变量小。

// The PartialOrd trait is required, for example, for the gen_range method from the rand crate that generates a random value in the range specified by a range expression.
// PartialOrd特征是必需的，例如，对于rand crate中的gen_range方法，它在range表达式指定的范围内生成一个随机值。

// The Ord trait allows you to know that for any two values of the annotated type, a valid ordering will exist.
// Ord特征允许你知道对于注释类型的任何两个值，一个有效的排序将存在。
// The Ord trait implements the cmp method, which returns an Ordering rather than an Option<Ordering> because a valid ordering will always be possible.
// Ord trait实现了cmp方法，该方法返回的是order而不是Option< ordered >，因为有效的排序总是可能的。
// You can only apply the Ord trait to types that also implement PartialOrd and Eq (and Eq requires PartialEq).
// 你只能将Ord特性应用到实现PartialOrd和Eq的类型(Eq需要PartialEq)。
// When derived on structs and enums, cmp behaves the same way as the derived implementation for partial_cmp does with PartialOrd.
// 当在结构和枚举上派生时，cmp的行为与partial_cmp在PartialOrd上的派生实现相同。

// An example of when Ord is required is when storing values in a BTreeSet<T>, a data structure that stores data based on the sort order of the values.
// 一个需要Ord的例子是在BTreeSet<T>中存储值时，BTreeSet<T>是一种基于值的排序顺序存储数据的数据结构。

// Clone and Copy for Duplicating Values
// Clone和Copy复制值
// The Clone trait allows you to explicitly create a deep copy of a value, and the duplication process might involve running arbitrary code and copying heap data.
// Clone特性允许您显式地创建一个值的深度副本，复制过程可能涉及运行任意代码和复制堆数据。
// See the “Ways Variables and Data Interact: Clone” section in Chapter 4 for more information on Clone.
// 关于克隆的更多信息，请参见第四章的“变量和数据交互方式:克隆”一节。

// Deriving Clone implements the clone method, which when implemented for the whole type, calls clone on each of the parts of the type.
// 派生Clone实现Clone方法，当为整个类型实现时，对类型的每个部分调用Clone。
// This means all the fields or values in the type must also implement Clone to derive Clone.
// 这意味着类型中的所有字段或值也必须实现Clone以派生Clone。

// An example of when Clone is required is when calling the to_vec method on a slice.
// 一个需要Clone的例子是在slice上调用to_vec方法。
// The slice doesn’t own the type instances it contains, but the vector returned from to_vec will need to own its instances, so to_vec calls clone on each item.
// slice不拥有它所包含的类型实例，但是从to_vec返回的vector需要拥有它的实例，所以to_vec在每个项目上调用clone。
// Thus, the type stored in the slice must implement Clone.
// 因此，slice中存储的类型必须实现Clone。

// The Copy trait allows you to duplicate a value by only copying bits stored on the stack; no arbitrary code is necessary.
// Copy trait允许你复制一个值，只复制存储在栈上的位;不需要任何代码。
// See the “Stack-Only Data: Copy” section in Chapter 4 for more information on Copy.
// 有关Copy的更多信息，请参见第四章的“Stack-Only Data: Copy”部分。

// The Copy trait doesn’t define any methods to prevent programmers from overloading those methods and violating the assumption that no arbitrary code is being run.
// Copy trait没有定义任何方法来防止程序员重载这些方法，从而违反没有任意代码正在运行的假设。
// That way, all programmers can assume that copying a value will be very fast.
// 这样，所有程序员都可以认为复制一个值会非常快。

// You can derive Copy on any type whose parts all implement Copy.
// 你可以在任何部分都实现Copy的类型上派生Copy。
// A type that implements Copy must also implement Clone, because a type that implements Copy has a trivial implementation of Clone that performs the same task as Copy.

// The Copy trait is rarely required; types that implement Copy have optimizations available, meaning you don’t have to call clone, which makes the code more concise.
// Copy trait很少需要;实现Copy的类型具有可用的优化，这意味着您不必调用clone，这使得代码更加简洁。

// Everything possible with Copy you can also accomplish with Clone, but the code might be slower or have to use clone in places.

// Hash for Mapping a Value to a Value of Fixed Size
// 将值映射到固定大小的值的散列
// The Hash trait allows you to take an instance of a type of arbitrary size and map that instance to a value of fixed size using a hash function.
// Hash trait允许你使用一个Hash函数将任意大小的实例映射到一个固定大小的值。
// Deriving Hash implements the hash method.
// 派生哈希实现哈希方法。
// The derived implementation of the hash method combines the result of calling hash on each of the parts of the type, meaning all fields or values must also implement Hash to derive Hash.
// 哈希方法的派生实现结合对类型的每个部分调用哈希的结果，这意味着所有字段或值也必须实现哈希来派生哈希。

// An example of when Hash is required is in storing keys in a HashMap<K, V> to store data efficiently.
// 一个需要哈希的例子是在HashMap<K, V>中存储键以有效地存储数据。

// Default for Default Values
// 默认值
// The Default trait allows you to create a default value for a type.
// Default trait允许你为类型创建一个默认值。
// Deriving Default implements the default function.
// 派生Default实现Default函数
// The derived implementation of the default function calls the default function on each part of the type, meaning all fields or values in the type must also implement Default to derive Default.
// default函数的派生实现在类型的每个部分调用default函数，这意味着类型中的所有字段或值也必须实现default来派生default。

// The Default::default function is commonly used in combination with the struct update syntax discussed in the “Creating Instances From Other Instances With Struct Update Syntax” section in Chapter 5.
// Default:: Default函数通常与第5章“使用结构更新语法从其他实例创建实例”一节中讨论的结构更新语法结合使用。
// You can customize a few fields of a struct and then set and use a default value for the rest of the fields by using ..Default::default().
// 你可以自定义结构中的一些字段，然后使用.. default::default()为其余字段设置和使用默认值。

// The Default trait is required when you use the method unwrap_or_default on Option<T> instances, for example.
// 例如，当你在Option<T>实例上使用unwrap_or_default方法时，Default特征是必需的。
// If the Option<T> is None, the method unwrap_or_default will return the result of Default::default for the type T stored in the Option<T>.
// 如果Option<T>为None，方法unwrap_or_default将返回存储在Option<T>中的类型T的Default::default的结果。

// Appendix D - Useful Development Tools
// 附录D - 有用的开发工具
// In this appendix, we talk about some useful development tools that the Rust project provides.
// 在本附录中，我们将讨论Rust项目提供的一些有用的开发工具。
// We’ll look at automatic formatting, quick ways to apply warning fixes, a linter, and integrating with IDEs.
// 我们将了解自动格式化、快速应用警告修复、linter以及与ide集成。

// Automatic Formatting with rustfmt
// 使用rustfmt自动格式化
// The rustfmt tool reformats your code according to the community code style.
// rustfmt工具根据社区代码样式重新格式化代码。
// Many collaborative projects use rustfmt to prevent arguments about which style to use when writing Rust: everyone formats their code using the tool.
// 许多合作项目使用rustfmt来防止在编写Rust时使用哪种风格的争论:每个人都使用该工具格式化他们的代码。

// To install rustfmt, enter the following:
// 安装rustfmt，输入如下:

$ rustup component add rustfmt

// This command gives you rustfmt and cargo-fmt, similar to how Rust gives you both rustc and cargo.
// 这个命令会给你rustfmt和cargo-fmt，类似于Rust同时给你rustc和cargo。
// To format any Cargo project, enter the following:
// 要格式化任何Cargo项目，输入以下内容:

$ cargo fmt

// Running this command reformats all the Rust code in the current crate.
// 执行此命令将重新格式化当前crate中的所有Rust代码。
// This should only change the code style, not the code semantics.
// 这应该只改变代码风格，而不是代码语义。
// For more information on rustfmt, see its documentation.
// 关于rustfmt的更多信息，请参阅它的文档。

// Fix Your Code with rustfix
// 使用rustfix修复代码
// The rustfix tool is included with Rust installations and can automatically fix compiler warnings that have a clear way to correct the problem that’s likely what you want.
// Rust安装中包含了rustfix工具，可以自动修复编译器警告，这些警告有一个明确的方法来纠正问题，这可能是你想要的。
// It’s likely you’ve seen compiler warnings before. For example, consider this code:
// 可能你以前见过编译器警告。例如，考虑以下代码:

// Filename: src/main.rs

fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}

// Here, we’re calling the do_something function 100 times, but we never use the variable i in the body of the for loop. Rust warns us about that:
//在这里，我们调用了100次do_something函数，但我们从未在for循环体中使用变量i。Rust提醒我们:

$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 --> src/main.rs:4:9
  |
4 |     for i in 0..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s

// The warning suggests that we use _i as a name instead: the underscore indicates that we intend for this variable to be unused.
// 该警告建议我们使用_i作为名称:下划线表示我们打算不使用这个变量。
// We can automatically apply that suggestion using the rustfix tool by running the command cargo fix:
// 我们可以通过运行命令cargo fix自动应用这个建议:

$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s

// When we look at src/main.rs again, we’ll see that cargo fix has changed the code:
//当我们查看src/main。再次，我们会看到货物修复已经改变了代码:

// Filename: src/main.rs

fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}

// The for loop variable is now named _i, and the warning no longer appears.
// for循环变量现在被命名为_i，警告不再出现。

// You can also use the cargo fix command to transition your code between different Rust editions. Editions are covered in Appendix E.
// 你也可以使用cargo fix命令在不同的Rust版本之间转换代码。版本详见附录E。

// More Lints with Clippy
// 更多带有Clippy的Lints
// The Clippy tool is a collection of lints to analyze your code so you can catch common mistakes and improve your Rust code.
// Clippy工具是一个分析你的代码的lint集合，这样你就可以抓住常见的错误并改进你的Rust代码。

// To install Clippy, enter the following:
// 安装Clippy，输入以下命令:

$ rustup component add clippy

// To run Clippy’s lints on any Cargo project, enter the following:
// 要在任何Cargo项目上运行Clippy的lints，输入以下命令:

$ cargo clippy

// For example, say you write a program that uses an approximation of a mathematical constant, such as pi, as this program does:
// 例如，假设你写了一个程序，使用一个数学常数的近似值，比如圆周率，就像这个程序一样:

// Filename: src/main.rs

fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}

// Running cargo clippy on this project results in this error:
// 在这个项目上运行cargo clippy会导致这个错误:

// error: approximate value of `f{32, 64}::consts::PI` found
//  --> src/main.rs:2:13
//   |
// 2 |     let x = 3.1415;
//   |             ^^^^^^
//   |
//   = note: `#[deny(clippy::approx_constant)]` on by default
//   = help: consider using the constant directly
//   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant

// This error lets you know that Rust already has a more precise PI constant defined, and that your program would be more correct if you used the constant instead.
// 这个错误让你知道Rust已经定义了一个更精确的PI常数，如果你使用这个常数，你的程序会更正确。
// You would then change your code to use the PI constant. The following code doesn’t result in any errors or warnings from Clippy:
// 你会改变你的代码使用PI常数。下面的代码不会导致来自Clippy的任何错误或警告:

// Filename: src/main.rs

fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}

// For more information on Clippy, see its documentation.
// 关于Clippy的更多信息，请参阅它的文档。

// IDE Integration Using rust-analyzer
// IDE集成使用rust-analyzer
// To help IDE integration, the Rust community recommends using rust-analyzer.
// 为了帮助集成IDE, Rust社区建议使用rust-analyzer。
// This tool is a set of compiler-centric utilities that speaks the Language Server Protocol, which is a specification for IDEs and programming languages to communicate with each other.
// 该工具是一组以编译器为中心的实用程序，它讲语言服务器协议，这是ide和编程语言相互通信的规范。
// Different clients can use rust-analyzer, such as the Rust analyzer plug-in for Visual Studio Code.
// 不同的客户端可以使用rust-analyzer，例如Visual Studio Code的Rust分析器插件。

// Visit the rust-analyzer project’s home page for installation instructions, then install the language server support in your particular IDE.
// 访问rust-analyzer项目的主页获取安装说明，然后在您的特定IDE中安装语言服务器支持。
// Your IDE will gain abilities such as autocompletion, jump to definition, and inline errors.
// 您的IDE将获得自动补全、跳转到定义和内联错误等能力。

// Appendix E - Editions
// 附录E -版本
// In Chapter 1, you saw that cargo new adds a bit of metadata to your Cargo.toml file about an edition.
// 在第1章中，你看到cargo new为你的cargo添加了一些元数据。关于Toml文件的一个版本。
// This appendix talks about what that means!
// 这个附录讨论的是什么意思!

// The Rust language and compiler have a six-week release cycle, meaning users get a constant stream of new features.
// Rust语言和编译器的发布周期为6周，这意味着用户可以不断获得新功能。
// Other programming languages release larger changes less often; Rust releases smaller updates more frequently.
// 其他编程语言较少发布较大的更改;Rust更频繁地发布较小的更新。
// After a while, all of these tiny changes add up.
// 过了一段时间，所有这些微小的变化积少成多。
// But from release to release, it can be difficult to look back and say, “Wow, between Rust 1.10 and Rust 1.31, Rust has changed a lot!”
// 但是从一个版本到另一个版本，我们很难回头说:“哇，从Rust 1.10到Rust 1.31, Rust改变了很多!”

// Every two or three years, the Rust team produces a new Rust edition.
// 每隔两三年，Rust团队就会推出一个新的Rust版本。
// Each edition brings together the features that have landed into a clear package with fully updated documentation and tooling.
// 每个版本都将已经登陆的功能整合到一个清晰的包中，并带有完全更新的文档和工具。
// New editions ship as part of the usual six-week release process.
// 新版本作为通常的六周发布过程的一部分。

// Editions serve different purposes for different people:
// 不同的人使用不同的版本:

// * For active Rust users, a new edition brings together incremental changes into an easy-to-understand package.
// * 对于活跃的Rust用户，新版本将增量变化整合到一个易于理解的包中。
// * For non-users, a new edition signals that some major advancements have landed, which might make Rust worth another look.
// * 对于非用户来说，新版本标志着一些主要的改进已经登陆，这可能会让Rust值得再看一看。
// * For those developing Rust, a new edition provides a rallying point for the project as a whole.
// * 对于那些开发Rust的人来说，新版本为整个项目提供了一个集结点。

// At the time of this writing, three Rust editions are available: Rust 2015, Rust 2018, and Rust 2021.
// 在撰写本文时，Rust有三个版本:Rust 2015, Rust 2018和Rust 2021。
// This book is written using Rust 2021 edition idioms.
// 本书使用Rust 2021版成语编写。

// The edition key in Cargo.toml indicates which edition the compiler should use for your code.
// Cargo中的edition键。Toml指示编译器应该为您的代码使用哪个版本。
// If the key doesn’t exist, Rust uses 2015 as the edition value for backward compatibility reasons.
// 如果键不存在，Rust将使用2015作为版本值，以实现向后兼容。

// Each project can opt in to an edition other than the default 2015 edition.
// 每个项目可以选择2015年默认版本以外的版本。
// Editions can contain incompatible changes, such as including a new keyword that conflicts with identifiers in code.
// 版本可以包含不兼容的更改，例如包含与代码中的标识符冲突的新关键字。
// However, unless you opt in to those changes, your code will continue to compile even as you upgrade the Rust compiler version you use.
// 但是，除非你选择了这些变化，否则即使你升级了Rust编译器版本，你的代码也会继续编译。

// All Rust compiler versions support any edition that existed prior to that compiler’s release, and they can link crates of any supported editions together.
// 所有Rust编译器版本都支持该编译器发布之前的任何版本，并且可以将任何支持版本的板条箱链接在一起。
// Edition changes only affect the way the compiler initially parses code.
// 版本更改只影响编译器最初解析代码的方式。
// Therefore, if you’re using Rust 2015 and one of your dependencies uses Rust 2018, your project will compile and be able to use that dependency.
// 因此，如果你正在使用Rust 2015，而你的一个依赖项使用Rust 2018，你的项目将编译并能够使用该依赖项。
// The opposite situation, where your project uses Rust 2018 and a dependency uses Rust 2015, works as well.
// 相反的情况，你的项目使用Rust 2018，而一个依赖项使用Rust 2015，也同样有效。

// To be clear: most features will be available on all editions.
// 需要明确的是:大多数功能将在所有版本中可用。
// Developers using any Rust edition will continue to see improvements as new stable releases are made.
// 使用任何Rust版本的开发人员将继续看到新的稳定版本的改进。
// However, in some cases, mainly when new keywords are added, some new features might only be available in later editions.
// 然而，在某些情况下，主要是当添加新的关键字时，一些新功能可能只在后续版本中可用。
// You will need to switch editions if you want to take advantage of such features.
// 如果你想利用这些特性，你需要切换版本。

// For more details, the Edition Guide is a complete book about editions that enumerates the differences between editions and explains how to automatically upgrade your code to a new edition via cargo fix.
// 关于更多细节，版本指南是一本关于版本的完整书籍，列举了版本之间的差异，并解释了如何通过cargo fix自动将您的代码升级到新版本。

// Appendix G - How Rust is Made and “Nightly Rust”
// 附录G - Rust 的制作过程 和 “Nightly Rust”
// This appendix is about how Rust is made and how that affects you as a Rust developer.
// 这个附录是关于Rust是如何制作的，以及它对Rust开发人员的影响。

// Stability Without Stagnation
// 稳定无停滞
// As a language, Rust cares a lot about the stability of your code.
// 作为一门语言，Rust非常关心代码的稳定性。
// We want Rust to be a rock-solid foundation you can build on, and if things were constantly changing, that would be impossible.
// 我们想让Rust成为你可以构建的坚如磐石的基础，如果事情不断变化，那就不可能了。
// At the same time, if we can’t experiment with new features, we may not find out important flaws until after their release, when we can no longer change things.
// 与此同时，如果我们不能尝试新功能，我们可能要等到新功能发布后才会发现重要的缺陷，那时我们已经不能再修改了。

// Our solution to this problem is what we call “stability without stagnation”, and our guiding principle is this: you should never have to fear upgrading to a new version of stable Rust.
// 我们对这个问题的解决方案是我们所谓的“稳定而不停滞”，我们的指导原则是:您永远不必担心升级到新版本的稳定Rust。
// Each upgrade should be painless, but should also bring you new features, fewer bugs, and faster compile times.
// 每次升级都应该是无痛的，但也应该为您带来新的功能，更少的错误和更快的编译时间。

// Choo, Choo! Release Channels and Riding the Trains
// 啾啾，啾啾!释放通道和乘坐火车
// Rust development operates on a train schedule. That is, all development is done on the master branch of the Rust repository.
// Rust开发按列车时刻表运行。也就是说，所有的开发都在Rust存储库的主分支上完成。
// Releases follow a software release train model, which has been used by Cisco IOS and other software projects.
// 发布遵循一个软件发布序列模型，该模型已被Cisco IOS和其他软件项目所采用。
// There are three release channels for Rust:
// Rust有三个发布通道:

// * Nightly
// * Beta
// * Stable

// Most Rust developers primarily use the stable channel, but those who want to try out experimental new features may use nightly or beta.
// 大多数Rust开发人员主要使用稳定版，但那些想尝试实验性新功能的人可能会使用夜间版或beta版。

// Here’s an example of how the development and release process works: let’s assume that the Rust team is working on the release of Rust 1.5.
// 这里有一个开发和发布过程的例子:让我们假设Rust团队正在发布Rust 1.5。
// That release happened in December of 2015, but it will provide us with realistic version numbers.
// 该版本发布于2015年12月，但它将为我们提供真实的版本号。
// A new feature is added to Rust: a new commit lands on the master branch. Each night, a new nightly version of Rust is produced.
// 一个新特性被添加到Rust: 一个新的提交落在主分支上。每天晚上，Rust都会生成一个新的夜间版本。
// Every day is a release day, and these releases are created by our release infrastructure automatically.
// 每天都是一个发布日，这些发布是由我们的发布基础设施自动创建的。
// So as time passes, our releases look like this, once a night:
// 所以随着时间的推移，我们的发布就像这样，每晚一次:

nightly: * - - * - - *

// Every six weeks, it’s time to prepare a new release! The beta branch of the Rust repository branches off from the master branch used by nightly. Now, there are two releases:
// 每六个星期，就该准备一个新版本了!Rust存储库的beta分支从夜间使用的主分支中分离出来。现在，有两个版本:

nightly: * - - * - - *
                     |
beta:                *

// Most Rust users do not use beta releases actively, but test against beta in their CI system to help Rust discover possible regressions.
// 大多数Rust用户并不积极使用beta版本，而是在CI系统中测试beta版本，以帮助Rust发现可能的回归。
// In the meantime, there’s still a nightly release every night:
// 与此同时，每天晚上仍有一个夜间发布:

nightly: * - - * - - * - - * - - *
                     |
beta:                *

// Let’s say a regression is found. Good thing we had some time to test the beta release before the regression snuck into a stable release!
// 假设找到了一个回归。好在我们有一些时间在回归进入稳定版本之前测试beta版本!
// The fix is applied to master, so that nightly is fixed, and then the fix is backported to the beta branch, and a new release of beta is produced:
// 修复被应用到master，因此nightly被修复，然后修复被反向移植到beta分支，并产生一个新的beta版本:

nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *

// Six weeks after the first beta was created, it’s time for a stable release! The stable branch is produced from the beta branch:
// 在第一个测试版创建六周后，是时候发布稳定版了!稳定分支由beta分支产生:

nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *

// Hooray! Rust 1.5 is done! However, we’ve forgotten one thing: because the six weeks have gone by, we also need a new beta of the next version of Rust, 1.6.
// 万岁!Rust 1.5完成了!然而，我们忘记了一件事:因为6周已经过去了，我们还需要Rust的下一个版本1.6的新测试版。
// So after stable branches off of beta, the next version of beta branches off of nightly again:
// 所以在稳定的beta分支之后，beta的下一个版本再次分支了nightly:

nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *

// This is called the “train model” because every six weeks, a release “leaves the station”, but still has to take a journey through the beta channel before it arrives as a stable release.
// 这被称为“火车模型”，因为每隔六周，就会有一个版本“离开车站”，但在作为稳定版本到达之前，仍然需要经过一段beta通道。

// Rust releases every six weeks, like clockwork. If you know the date of one Rust release, you can know the date of the next one: it’s six weeks later.
// Rust每六周发布一次，就像时钟一样。如果你知道一个Rust发布的日期，你就可以知道下一个发布的日期:6周后。
// A nice aspect of having releases scheduled every six weeks is that the next train is coming soon.
// 每六周发布一次的一个好处是下一班车马上就要来了。
// If a feature happens to miss a particular release, there’s no need to worry: another one is happening in a short time!
// 如果某个特性错过了某个特定的版本，不必担心:另一个特性很快就会出现!
// This helps reduce pressure to sneak possibly unpolished features in close to the release deadline.
// 这有助于减少在接近发布截止日期时偷偷添加未优化功能的压力。

// Thanks to this process, you can always check out the next build of Rust and verify for yourself that it’s easy to upgrade to:
// 多亏了这个过程，你可以随时查看Rust的下一个版本，并亲自验证它很容易升级到:
//  if a beta release doesn’t work as expected, you can report it to the team and get it fixed before the next stable release happens!
// 如果一个测试版没有像预期的那样工作，你可以向团队报告，并在下一个稳定版本出现之前修复它!
// Breakage in a beta release is relatively rare, but rustc is still a piece of software, and bugs do exist.
// 在beta版本中被破坏的情况相对较少，但rustc仍然是一个软件，bug确实存在。

// Unstable Features
// 不稳定的特性
// There’s one more catch with this release model: unstable features.
// 这个发布模型还有一个问题:不稳定的特性。
// Rust uses a technique called “feature flags” to determine what features are enabled in a given release.
// Rust使用一种称为“特性标志”的技术来确定在给定的版本中启用了哪些特性。
// If a new feature is under active development, it lands on master, and therefore, in nightly, but behind a feature flag.
// 如果一个新特性正在开发中，它会登陆master，因此，在nightly，但是在一个feature标志后面。
// If you, as a user, wish to try out the work-in-progress feature, you can,
// 如果你作为一个用户，想要尝试制品功能（work-in-progress feature），你可以，
//  but you must be using a nightly release of Rust and annotate your source code with the appropriate flag to opt in.
//  但你必须使用Rust的夜间版本，并注释你的源代码与适当的标志选择加入。

// If you’re using a beta or stable release of Rust, you can’t use any feature flags.
// 如果你正在使用Rust的beta版或稳定版，你不能使用任何feature标志。
// This is the key that allows us to get practical use with new features before we declare them stable forever.
// 这是允许我们在宣布新特性永远稳定之前获得实际使用的关键。
// Those who wish to opt into the bleeding edge can do so, and those who want a rock-solid experience can stick with stable and know that their code won’t break.
// 那些想要进入前沿的人可以这样做，那些想要坚如磐石的体验的人可以坚持稳定，并且知道他们的代码不会崩溃。
// Stability without stagnation.
// 稳定无停滞。

// This book only contains information about stable features, as in-progress features are still changing,
// 本书只包含关于稳定特性的信息，因为正在进行的特性仍在变化，
//  and surely they’ll be different between when this book was written and when they get enabled in stable builds.
//  当然，在本书撰写时和在稳定版本中启用时，它们会有所不同。
// You can find documentation for nightly-only features online.
// 你可以在网上找到只有夜间功能的文档。

// Rustup and the Role of Rust Nightly
// Rust和Rust Nightly的作用
// Rustup makes it easy to change between different release channels of Rust, on a global or per-project basis.
// Rust可以方便地在Rust的不同发布渠道之间进行更改，无论是在全局还是每个项目的基础上。
// By default, you’ll have stable Rust installed.
// 默认情况下，你会安装稳定的Rust。
// To install nightly, for example:
// 夜间安装，例如:

$ rustup toolchain install nightly

// You can see all of the toolchains (releases of Rust and associated components) you have installed with rustup as well. Here’s an example on one of your authors’ Windows computer:
// 你可以看到你安装的所有工具链(Rust版本和相关组件)。下面是你的一位作者的Windows电脑上的一个例子:

> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc

// As you can see, the stable toolchain is the default.
// 如你所见，稳定的工具链是默认的。
// Most Rust users use stable most of the time.
// 大多数Rust用户大部分时间使用stable。
// You might want to use stable most of the time, but use nightly on a specific project, because you care about a cutting-edge feature.
// 你可能想在大多数时间使用stable，但在特定项目中使用nightly，因为你关心的是最前沿的特性。
// To do so, you can use rustup override in that project’s directory to set the nightly toolchain as the one rustup should use when you’re in that directory:
// 要做到这一点，你可以在该项目的目录下使用rustup覆盖来设置夜间工具链，当你在该目录下时，rustup应该使用:

$ cd ~/projects/needs-nightly
$ rustup override set nightly

// Now, every time you call rustc or cargo inside of ~/projects/needs-nightly, rustup will make sure that you are using nightly Rust, rather than your default of stable Rust.
// 现在，每次你在~/projects/needs-nightly中调用rustc或cargo时，rustup都会确保你使用的是夜间Rust，而不是默认的稳定Rust。
// This comes in handy when you have a lot of Rust projects!
// 当你有很多Rust项目时，这就派上用场了!

// The RFC Process and Teams
// RFC流程和团队
// So how do you learn about these new features? Rust’s development model follows a Request For Comments (RFC) process.
// 那么你如何了解这些新特性呢?Rust的开发模型遵循一个请求评论(Request For Comments, RFC)过程。
// If you’d like an improvement in Rust, you can write up a proposal, called an RFC.
// 如果你想改进Rust，你可以写一个提案，叫做RFC。

// Anyone can write RFCs to improve Rust, and the proposals are reviewed and discussed by the Rust team, which is comprised of many topic subteams.
// 任何人都可以编写rfc来改进Rust，提案由Rust团队进行审查和讨论，Rust团队由许多主题子团队组成。
// There’s a full list of the teams on Rust’s website, which includes teams for each area of the project: language design, compiler implementation, infrastructure, documentation, and more.
// Rust的网站上有一个完整的团队列表，包括项目各个领域的团队:语言设计、编译器实现、基础设施、文档等等。
// The appropriate team reads the proposal and the comments, writes some comments of their own, and eventually, there’s consensus to accept or reject the feature.
// 合适的团队阅读提案和评论，写一些他们自己的评论，最终达成接受或拒绝该功能的共识。

// If the feature is accepted, an issue is opened on the Rust repository, and someone can implement it.
// 如果该特性被接受，Rust存储库上就会打开一个问题，有人可以实现它。
// The person who implements it very well may not be the person who proposed the feature in the first place!
// 很好地实现该功能的人可能不是最初提出该功能的人!
// When the implementation is ready, it lands on the master branch behind a feature gate, as we discussed in the “Unstable Features” section.
// 当实现准备就绪时，它会落在一个特性门后面的主分支上，正如我们在“不稳定特性”一节中讨论的那样。

// After some time, once Rust developers who use nightly releases have been able to try out the new feature, team members will discuss the feature, how it’s worked out on nightly,
// 一段时间后，一旦使用夜间发布的Rust开发人员能够尝试新功能，团队成员将讨论该功能，它是如何在夜间运行的，
//  and decide if it should make it into stable Rust or not.
//  并决定它是否应该成为稳定的Rust。
// If the decision is to move forward, the feature gate is removed, and the feature is now considered stable!
// 如果决定继续前进，那么特性门将被移除，现在该特性被认为是稳定的!
// It rides the trains into a new stable release of Rust.
// 它骑着火车进入Rust的新稳定版本。

fn main() {
    println!("Hello, world!");
}
