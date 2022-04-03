///* 泛型是具体类型或其它属性的抽象代替
// 泛型代码是一种模板，里面有一些占位符（T 泛型的类型参数）
// 编译器在编译时，将占位符替换为具体的类型 (单态化)

///* 函数中使用
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    // &item 直接解引用，item 类型为 T，不用解引用
    for &item in list {
        if item > largest {
            // error： binary operation `>` cannot be applied to type `T`
            // 需要限定 T 的类型
            largest = item;
        }
    }

    largest
}

fn main1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

///* struct 中使用
struct Point<T, U> {
    x: T,
    y: U,
}

fn main2() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

///* enum 中使用
enum Result<T, E> {
    Ok(T),
    Err(E),
}

///* 方法定义 中使用
struct Point<T> {
    x: T,
    y: T,
}

// 在类型 T 上实现方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 只针对具体类型实现方法（其他类型没实现该方法）
// Point<f32> 类型会有一个方法 distance_from_origin，而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main3() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

///* struct 中的类型参数可以和方法的泛型类型参数不同
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main4() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y); // p3.x = 5, p3.y = c
}

///* 泛型代码的性能
// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。
// 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
// 编译慢、体积会增加，但是运行速度无影响

fn main 5() {
    let integer = Some(5);
    let float = Some(5.0);
}

// 编译后，转变成指定类型
/*
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main6() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
*/