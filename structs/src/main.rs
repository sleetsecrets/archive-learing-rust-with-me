// Defining and Instantiating Structs
// 定义和实例化结构体
// Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values.
// 结构体类似于5.1.1节中讨论的元组，它们都保存多个相关的值。
// Like tuples, the pieces of a struct can be different types.
// 像元组一样，结构体的各个部分可以是不同的类型。
// Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean.
// 与元组不同的是，在结构体中，你需要为每个数据块命名，以便清楚地说明值的含义。
// Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.
// 添加这些名称意味着结构体比元组更灵活:你不必依赖数据的顺序来指定或访问实例的值。

// To define a struct, we enter the keyword struct and name the entire struct.
// 要定义结构体，我们输入关键字struct并命名整个结构体。
// A struct’s name should describe the significance of the pieces of data being grouped together.
// 结构体的名称应该描述分组数据的重要性。
// Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields.
// 然后，在大括号中，我们定义数据片段的名称和类型，我们称之为字段。
// For example, Listing 5-1 shows a struct that stores information about a user account.
// 例如，代码清单5-1是一个存储用户账户信息的结构体。

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Listing 5-1: A User struct definition

// Using Tuple Structs without Named Fields to Create Different Types
// 使用没有命名字段的元组结构来创建不同的类型
// Rust also supports structs that look similar to tuples, called tuple structs.
// Rust还支持类似于元组的结构体，称为元组结构体。
// Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields;
// 元组结构体具有结构体名称提供的额外含义，但没有与其字段关联的名称;
// rather, they just have the types of the fields.
// 相反，它们只有字段的类型。
// Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples,
// 当你想给整个元组一个名字，并使元组不同于其他元组时，元组结构体很有用，
// and when naming each field as in a regular struct would be verbose or redundant.
// 什么时候像普通结构体那样命名每个字段会显得冗长或冗余。

// To define a tuple struct, start with the struct keyword and the struct name followed by the types in the tuple.
// 要定义元组结构体，从struct关键字开始，结构名后面跟着元组中的类型。
// For example, here we define and use two tuple structs named Color and Point:
// 例如，这里我们定义并使用两个元组结构体，名为Color和Point:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn use_tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Note that the black and origin values are different types, because they’re instances of different tuple structs.
// 注意，black和origin值是不同的类型，因为它们是不同元组结构体的实例。
// Each struct you define is its own type, even though the fields within the struct might have the same types.
// 你定义的每个结构体都是自己的类型，即使结构体中的字段可能具有相同的类型。
// For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values.
// 例如，接受Color类型形参的函数不能接受Point作为实参，即使这两种类型都由三个i32值组成。
// Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.
// 否则，元组结构体实例与元组类似，你可以将它们解构为单独的部分，并且可以使用。其次是用于访问单个值的索引。

// Unit-Like Structs Without Any Fields
// 没有任何字段的类单元结构体
// You can also define structs that don’t have any fields!
// 你也可以定义没有任何字段的结构体!
// These are called unit-like structs because they behave similarly to (), the unit type that we mentioned in “The Tuple Type” section.
// 这些被称为类单元结构体(unit-like struct)，因为它们的行为类似于()，即我们在“元组类型”一节中提到的单元类型。
// Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
// 当你需要在某种类型上实现一个trait，但又没有任何数据需要存储在该类型本身时，类单元结构体可以很有用。
// We’ll discuss traits in Chapter 10.
// 我们将在第10章讨论traits
// Here’s an example of declaring and instantiating a unit struct named AlwaysEqual:
// 下面是一个声明并实例化名为AlwaysEqual的unit结构体的例子:
struct AlwaysEqual;

fn use_unit_like_structs() {
    let subject = AlwaysEqual;
}

// To define AlwaysEqual, we use the struct keyword, the name we want, then a semicolon. No need for curly brackets or parentheses!
// 要定义AlwaysEqual，我们使用struct关键字，我们想要的名称，然后一个分号。不需要大括号或圆括号!
// Then we can get an instance of AlwaysEqual in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses.
// 然后，我们可以以类似的方式在subject变量中获取AlwaysEqual的实例:使用我们定义的名称，不使用任何大括号或圆括号。
// Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes.
// 想象一下，稍后我们将为这个类型实现行为，使AlwaysEqual的每个实例始终与任何其他类型的每个实例相等，以便测试时得到已知的结果。
// We wouldn’t need any data to implement that behavior!
// 我们不需要任何数据来实现这种行为!
// You’ll see in Chapter 10 how to define traits and implement them on any type, including unit-like structs.
// 第10章会介绍如何在任何类型上定义和实现trait，包括类单元的结构体。

// Ownership of Struct Data
// 结构体数据的所有权
// In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type.
// 在代码清单5-1的User结构体定义中，我们使用了拥有的String类型，而不是&str字符串切片类型。
// This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.
// 这是一个深思熟虑的选择，因为我们希望这个结构体的每个实例都拥有其所有数据，并且只要整个结构体有效，该数据就有效。

// It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10.
// 结构体也可以存储对其他对象拥有的数据的引用，但这样做需要使用生命周期(lifetime)，这是Rust的一个特性，将在第10章讨论。
// Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
// 生命周期确保结构体引用的数据在该结构体存在的时间内有效。
// Let’s say you try to store a reference in a struct without specifying lifetimes, like the following; this won’t work:
// 假设你试图将引用存储在结构体中，但没有指定生命周期，如下所示;这是行不通的:

struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn use_slice() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

// The compiler will complain that it needs lifetime specifiers:
// 编译器会提示它需要生命周期说明符:

// $ cargo run
//    Compiling structs v0.1.0 (file:///projects/structs)
// error[E0106]: missing lifetime specifier
//  --> src/main.rs:3:15
//   |
// 3 |     username: &str,
//   |               ^ expected named lifetime parameter
//   |
// help: consider introducing a named lifetime parameter
//   |
// 1 ~ struct User<'a> {
// 2 |     active: bool,
// 3 ~     username: &'a str,
//   |

// error[E0106]: missing lifetime specifier
//  --> src/main.rs:4:12
//   |
// 4 |     email: &str,
//   |            ^ expected named lifetime parameter
//   |
// help: consider introducing a named lifetime parameter
//   |
// 1 ~ struct User<'a> {
// 2 |     active: bool,
// 3 |     username: &str,
// 4 ~     email: &'a str,
//   |

// For more information about this error, try `rustc --explain E0106`.
// error: could not compile `structs` due to 2 previous errors

// In Chapter 10, we’ll discuss how to fix these errors so you can store references in structs,
// 在第10章中，我们将讨论如何修复这些错误，以便你可以在结构体中存储引用，
// but for now, we’ll fix errors like these using owned types like String instead of references like &str.
// 但现在，我们将使用String等自有类型而不是&str等引用来修复此类错误。

fn main() {
    println!("Hello, world!");

    // Using Structs to Structure Related Data
    // 使用结构体来构建相关数据
    // A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.
    // 结构体(struct)是一种自定义数据类型，它允许你打包在一起，并命名多个相关值，组成有意义的组。
    // If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.
    // 如果你熟悉面向对象语言，结构体就像对象的数据属性。
    // In this chapter, we’ll compare and contrast tuples with structs to build on what you already know and demonstrate when structs are a better way to group data.
    // 在本章中，我们将在你已经知道的基础上，比较和对比元组和结构体，并演示何时结构体是一种更好的数据分组方式。

    // We’ll demonstrate how to define and instantiate structs.
    // 我们将演示如何定义和实例化结构体。
    // We’ll discuss how to define associated functions, especially the kind of associated functions called methods, to specify behavior associated with a struct type.
    // 我们将讨论如何定义关联函数，特别是称为方法的关联函数的类型，以指定与结构类型关联的行为。
    // Structs and enums (discussed in Chapter 6) are the building blocks for creating new types in your program’s domain to take full advantage of Rust’s compile time type checking.
    // 结构体和枚举(在第6章讨论)是在程序的域中创建新类型的构建块，以充分利用Rust的编译时类型检查。

    // To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields.
    // 为了在定义后使用结构体，我们通过为每个字段指定具体的值来创建该结构体的实例。
    // We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs,
    // 我们通过声明结构体的名称，然后添加包含键:值对的大括号来创建实例，
    // where the keys are the names of the fields and the values are the data we want to store in those fields.
    // 键是字段的名称，值是我们想要存储在这些字段中的数据。
    // We don’t have to specify the fields in the same order in which we declared them in the struct.
    // 我们不必按照在结构体中声明字段的顺序指定它们。
    // In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type.
    // 换句话说，结构体定义就像类型的通用模板，实例用特定的数据填充该模板，以创建该类型的值。
    // For example, we can declare a particular user as shown in Listing 5-2.
    // 例如，我们可以像代码清单5-2那样声明一个特定的用户。

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // Listing 5-2: Creating an instance of the User struct
    // 代码清单5-2:创建一个User结构体的实例

    // To get a specific value from a struct, we use dot notation.
    // 要从结构体中获取特定的值，我们使用句点表示法。
    // For example, to access this user’s email address, we use user1.email.
    // 例如，要访问用户的电子邮件地址，我们使用user1.email。
    // If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.
    // 如果实例是可变的，我们可以使用句点表示法并将值赋值给特定的字段。
    // Listing 5-3 shows how to change the value in the email field of a mutable User instance.
    // 清单5-3展示了如何修改可变用户实例的email字段的值

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // Listing 5-3: Changing the value in the email field of a User instance
    // 清单5-3:修改用户实例的email字段的值

    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
    // 注意，整个实例必须是可变的;Rust不允许我们只将某些字段标记为可变字段。
    // As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.
    // 和任何表达式一样，我们可以构造一个结构体的新实例，作为函数体中的最后一个表达式，以隐式返回该新实例。

    // Listing 5-4 shows a build_user function that returns a User instance with the given email and username.
    // 清单5-4展示了一个build_user函数，它返回一个用户实例，其中包含给定的电子邮件和用户名。
    // The active field gets the value of true, and the sign_in_count gets a value of 1.
    // active字段的值为true, sign_in_count的值为1。

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }
    // Listing 5-4: A build_user function that takes an email and username and returns a User instance
    // 清单5-4:一个build_user函数，它接受电子邮件和用户名，并返回一个User实例

    // It makes sense to name the function parameters with the same name as the struct fields,
    // 将函数参数命名为与结构体字段同名是有意义的，
    // but having to repeat the email and username field names and variables is a bit tedious.
    // 但是重复电子邮件地址和用户名字段名称和变量有点繁琐。
    // If the struct had more fields, repeating each name would get even more annoying.
    // 如果结构体有更多字段，重复每个名称将变得更加烦人。
    // Luckily, there’s a convenient shorthand!
    // 幸运的是，有一种方便的简写方式!

    // Using the Field Init Shorthand
    // 使用字段初始化速记
    // Because the parameter names and the struct field names are exactly the same in Listing 5-4,
    // 因为参数名称和结构体字段名称与清单5-4中完全相同，
    // we can use the field init shorthand syntax to rewrite build_user so that it behaves exactly the same but doesn’t have the repetition of email and username,
    // 可以使用字段初始化速记语法重写build_user，使其行为完全相同，但不会有重复的email和username，
    // as shown in Listing 5-5.
    // 如清单5-5所示。

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    // Listing 5-5: A build_user function that uses field init shorthand because the email and username parameters have the same name as struct fields
    // 清单5-5:一个build_user函数，它使用field init简写，因为email和username参数与struct fields同名

    // Here, we’re creating a new instance of the User struct,
    // 这里，我们创建了一个User结构体的新实例，
    // which has a field named email. We want to set the email field’s value to the value in the email parameter of the build_user function.
    // 它有一个名为email的字段。我们想将电子邮件字段的值设置为build_user函数的email参数中的值。
    // Because the email field and the email parameter have the same name, we only need to write email rather than email: email.
    // 因为email字段和email参数同名，我们只需要写email而不是email: email。

    // Creating Instances From Other Instances With Struct Update Syntax
    // 使用Struct Update语法从其他实例创建实例
    // It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some.
    // 创建一个结构体的新实例，包含另一个实例的大部分值，但对其中一些进行修改，这通常很有用。
    // You can do this using struct update syntax.
    // 你可以使用struct update语法来实现

    // First, in Listing 5-6 we show how to create a new User instance in user2 regularly, without the update syntax.
    // 首先，在代码清单5-6中，我们展示了如何在user2中常规地创建一个新用户实例，而不使用update语法。
    // We set a new value for email but otherwise use the same values from user1 that we created in Listing 5-2.
    // 我们给email设置了一个新值，但使用与代码清单5-2中user1的值相同的值。

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Listing 5-6: Creating a new User instance using one of the values from user1
    // 代码清单5-6:使用user1中的一个值创建一个新用户实例

    // Using struct update syntax, we can achieve the same effect with less code, as shown in Listing 5-7.
    // 使用struct update语法，我们可以用更少的代码达到同样的效果，如清单5-7所示。
    // The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    // 语法…指定未显式设置的其余字段与给定实例中的字段具有相同的值。

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // Listing 5-7: Using struct update syntax to set a new email value for a User instance but use the rest of the values from user1
    // 清单5-7:使用struct update语法为用户实例设置一个新的email值，但使用user1中的其余值

    // The code in Listing 5-7 also creates an instance in user2 that has a different value for email but has the same values for the username, active, and sign_in_count fields from user1.
    // 清单5-7中的代码还在user2中创建了一个实例，它的email属性值不同，但user1的username、active和sign_in_count属性值相同。
    // The ..user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1,
    // ..user1必须排在最后，以指定任何剩余的字段都应该从user1的相应字段中获取值，
    // but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.
    // 但是我们可以选择以任何顺序为任意数量的字段指定值，而无需考虑结构体定义中字段的顺序。

    // Note that the struct update syntax uses = like an assignment; this is because it moves the data, just as we saw in the “Ways Variables and Data Interact: Move” section.
    // 注意，struct update语法使用=作为赋值语句;这是因为它会移动数据，就像我们在5.1.3节中看到的那样。
    // In this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2.
    // 在这个例子中，创建user2后就不能再使用user1了，因为user1的username字段中的字符串被移到了user2中。
    // If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2.
    // 如果我们给user2的email和username都赋了新的字符串值，因此只使用了user1的active和sign_in_count值，那么user1在创建user2后仍然有效。
    // The types of active and sign_in_count are types that implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.
    // active和sign_in_count的类型是实现Copy trait的类型，因此我们在“Stack-Only Data: Copy”一节中讨论的行为将适用。
}
