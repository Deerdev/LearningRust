///* HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。
// 它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中

//
// 必须首先 use 标准库中集合部分的 HashMap
// HashMap 是最不常用的，所以并没有被 prelude 自动引用
use std::collections::HashMap;
fn main() {
    // 创建
    let mut scores = HashMap::new();
    // 添加
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
// 哈希 map 将它们的数据储存在堆上，这个 HashMap 的键类型是 String 而值类型是 i32。
// 类似于 vector，哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。

//
//** collect: 另一个构建哈希 map 的方法是在一个元组的 vector 上使用迭代器（iterator）和 collect 方法，其中每个元组包含一个键值对
// 显式指定返回类型
fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // 这里 HashMap<_, _> 类型注解是必要的，表明是 HashMap
    // 因为可能 collect 为很多不同的数据结构，而除非显式指定否则 Rust 无从得知你需要的类型。
    // 是对于键和值的类型参数来说，可以使用下划线占位，而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型, 键（key）类型是 String，值（value）类型是 i32
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
}

//
///* 哈希 map 和所有权
// 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
    println!("{}", field_name); // err: borrow of moved value: `field_name`

    //
    // 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。
    // **但是这些引用指向的值必须至少在哈希 map 有效时也是有效的
    let field_name2 = String::from("Favorite color");
    let field_value2 = String::from("Blue");
    let mut map2 = HashMap::new();
    map2.insert(&field_name2, &field_value2);
    println!("{}", field_name2); // ok
}

//
///* 访问哈希 map 中的值
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // get 返回 Option<V>
    let score = scores.get(&team_name);
    if let Some(age) = score {
        println!("team age: {}", age);
    }

    // 遍历 hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

///* 更新哈希 map
//
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    // * insert 相同 key 覆盖
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // *只在键没有对应值时插入 entry
    // entry 函数的返回值是一个枚举，Entry，它代表了可能存在也可能不存在的值
    // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的「可变引用」，如果不存在则将参数作为新值插入并返回新值的可变引用。
    // 返回的是 可变引用，可以修改
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}

// 根据旧值更新一个值
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // 统计单词出现的次数
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // 返回可变引用
        *count += 1;
    }

    println!("{:?}", map); // {"world": 2, "hello": 1, "wonderful": 1}
}

//* 哈希函数
// HashMap 默认使用一种叫做 SipHash 的哈希函数，它可以抵御涉及哈希表（hash table）的拒绝服务（Denial of Service, DoS）攻击。
// 然而这并不是可用的最快的算法，不过为了更高的安全性值得付出一些性能的代价。
// 如果性能监测显示此哈希函数非常慢，以致于你无法接受，你可以指定一个不同的 hasher 来切换为其它函数。
// hasher 是一个实现了 BuildHasher trait 的类型。你并不需要从头开始实现你自己的 hasher；crates.io 有其他人分享的实现了许多常用哈希算法的 hasher 的库。
