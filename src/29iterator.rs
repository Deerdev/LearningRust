///* 迭代器模式允许你对一个序列的项进行某些处理。
/// 迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。
/// 在 Rust 中，迭代器是 惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // 使用 for 循环时无需使 v1_iter 可变，因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

///* Iterator trait 和 next 方法
// 迭代器都实现了一个叫做 Iterator 的trait, 定义于标准库

// 定义类似：type Item 和 Self::Item，定义了 trait 的 关联类型（associated type）
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 此处省略了方法的默认实现
}
// * 这段代码表明实现 Iterator trait 要求同时定义一个 Item 类型，这个 Item 类型被用作 next 方法的返回值类型。换句话说，Item 类型将是迭代器返回元素的类型。
// * iterator trait 仅要求实现一个方法 next
// next 一次返回迭代器中的一个项，封装在 Some 中，当迭代器结束时，它返回 None。

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        // 需要 mut，因为 next 会修改它内部（在 for 循环中，已经取得所有权，并把 item 变成可变的了）
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None); // 结束
    }
}
// * 另外需要注意到从 next 调用中得到的值是 vector 的不可变引用。
// iter 方法是在不可变引用上创建迭代器。
// into_iter: 创建迭代器会获得所有权(move 了)
// iter_mut: 创建迭代器的可变引用

///* 消费迭代器的方法
// Iterator trait 有一系列不同的由标准库提供默认实现的方法, 哪些调用 next 方法的方法被称为 消费适配器（consuming adaptors）
// 因为调用他们会消耗迭代器
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();
        // sum 会消耗完迭代器
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }
}

///* 产生其他迭代器的方法
// Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors）
// 他们允许我们将当前迭代器变为不同类型的迭代器
// 可以链式调用多个迭代器适配器。不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // map
    // 没有消耗，map 不会执行
    // v1.iter().map(|x| x + 1);

    // 使用 collect 消耗，执行 iter
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

///* 使用闭包获取环境
// 通过使用 filter 迭代器适配器和捕获环境的闭包的常规用例
// filter 方法获取一个使用迭代器的每一个项并返回布尔值的闭包。
// 如果闭包返回 true，其值将会包含在 filter 提供的新迭代器中。如果闭包返回 false，其值不会包含在结果迭代器中。
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // 捕获了 shoe_size
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

///* 实现 Iterator trait 来创建自定义迭代器
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    // 1-5
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    // 打印 1-5
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// * 使用自定义迭代器中其他 Iterator trait 方法
// 通过定义 next 方法实现 Iterator trait，现在就可以使用任何标准库定义的拥有默认实现的 Iterator trait 方法了(sum map...)，因为他们都使用了 next 方法的功能。

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn using_other_iterator_trait_methods() {
        // 将这些值与另一个 Counter 实例在省略了第一个值之后产生的值配对，将每一对值相乘，只保留那些可以被三整除的结果，然后将所有保留的结果相加
        // [1*2, 2*3, 3*4, 4*5, 5*None]
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}


///* 循环和迭代器的性能
// 迭代器，作为一个高级的抽象，被编译成了与手写的底层代码大体一致性能代码。
// 迭代器是 Rust 的 零成本抽象（zero-cost abstractions）之一，它意味着抽象并不会引入运行时开销，它与本贾尼·斯特劳斯特卢普（C++ 的设计和实现者）在 “Foundations of C++”（2012） 中所定义的 零开销（zero-overhead）如出一辙：

/*
In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.

Bjarne Stroustrup "Foundations of C++"
从整体来说，C++ 的实现遵循了零开销原则：你不需要的，无需为他们买单。更有甚者的是：你需要的时候，也不可能找到其他更好的代码了。
*/

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
// 创建了一个迭代器，使用了两个适配器，接着消费了其值。Rust 代码将会被编译为什么样的汇编代码呢？好吧，在编写本书的这个时候，它被编译成与手写的相同的汇编代码。
// *遍历 coefficients 的值完全用不到循环：Rust 知道这里会迭代 12 次，所以它“展开”（unroll）了循环。展开是一种移除循环控制代码的开销并替换为每个迭代中的重复代码的优化。

// 所有的系数都被储存在了寄存器中，这意味着访问他们非常快。这里也没有运行时数组访问边界检查。所有这些 Rust 能够提供的优化使得结果代码极为高效。

// 现在知道这些了，请放心大胆的使用迭代器和闭包吧！他们使得代码看起来更高级，但并不为此引入运行时性能损失。



