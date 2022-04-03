struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main1() {
    // 定义
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // 访问
    user1.email = String::from("anotheremail@example.com");
}
// 一旦 struct 实例可变，那么实例的所有字段都是可变的，不支持单个字段可变

/// 作为返回值
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

/// 字段初始化简写
// 参数名与字段名都完全相同
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

/// 更新语法: 使用结构体更新语法从其他实例创建实例
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main2() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // -------
    // 更新语法
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // `..user1` 必须放在最后，以指定其余的字段应从 user1 的相应字段中获取其值

    // 请注意，结构更新语法就像带有 = 的赋值，因为它移动了数据，就像我们在“变量与数据交互的方式：移动”部分讲到的一样。在这个例子中，我们在创建 user3 后不能再使用 user1，因为 user1 的 username 字段中的 String 被移到 user3 中。如果我们给 user3 的 email 和 username 都赋予新的 String 值，从而只使用 user1 的 active 和 sign_in_count 值，那么 user1 在创建 user3 后仍然有效。active 和 sign_in_count 的类型是实现 Copy trait 的类型，所以我们在“变量与数据交互的方式：克隆” 部分讨论的行为同样适用。
}

/// Tuple Structs 元组结构体: 使用没有命名字段的元组结构体来创建不同的类型
// 元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main3() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // 注意 black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例。你定义的每一个结构体有其自己的类型，即使结构体中的字段有着相同的类型
}

/// 类单元结构体（unit-like structs）: 没有任何字段的类单元结构体(空结构体)
// 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
struct AlwaysEqual;

fn main4() {
    let subject = AlwaysEqual;
}

/// 结构体数据的所有权
// User 结构体的定义中，我们使用了自身拥有所有权的 String 类型而不是 &str 字符串 slice 类型。
// 这是一个有意而为之的选择，因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的。
// 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 生命周期（lifetimes）
// 生命周期确保结构体引用的数据有效性跟结构体本身保持一致
// 不指定生命周期会编译报错
struct User {
    active: bool,
    username: &str, // missing lifetime specifier
    email: &str,    // missing lifetime specifier
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}

/// 打印 struct

#[derive(Debug)] // 注解：Rectangle 派生(derive) Debug trait 的功能(类继承)
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // 未实现 `std::fmt::Display` trait
    // err: `Rectangle` doesn't implement `std::fmt::Display`
    // = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    println!("rect1 is {}", rect1);

    // error[E0277]: `Rectangle` doesn't implement `Debug`
    // 添加 #[derive(Debug)]
    println!("rect1 is {:?}", rect1);
    // rect1 is Rectangle { width: 30, height: 50 }

    println!("rect1 is {:#?}", rect1);
    // 打印风格更好看
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50,
    // }

    // dbg!
    // 执行表达式，并且返回所有权；(类似打印之后，将结果返回）
    // 注意：调用 dbg! 宏会打印到标准错误控制台流（stderr），与 println! 不同，后者会打印到标准输出控制台流（stdout）。
    dbg!(&rect1);
    // [src/main.rs:10] 30 * scale = 60
    // [src/main.rs:14] &rect1 = Rectangle {
    //     width: 60,
    //     height: 50,
    // }
}
