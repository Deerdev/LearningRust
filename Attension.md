
# 借用和引用 (06)
- 不可变引用：`&String` 指针参数算借用，不拥有，不能修改
- 可变引用：`mut &String` 可以修改
    - 同一作用域，特定数据只能有一个可变引用
- 同一时刻，你只能拥有
    - 要么一个可变引用, 要么任意多个不可变引用
    - 当变量被不可变引用时，可变引用不可修改值，因为存在不可变的引用
- 引用必须总是有效的

# 一些函数标记
- `!` : 表明该函数是一个发散函数，不会返回任何值，包括 `()`
- `unimplemented!()` : 告诉编译器该函数尚未实现
    - 类似的标记还有 `todo!()`，当代码执行到这种未实现的地方时，程序会直接报错
```rust
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}
```

# 切片 与 字符串
- 切片需要注意字符的分界点：切片的索引必须落在字符之间的边界位置，也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节，如果切在中间会崩溃
- Rust 中的字符是 Unicode 类型，因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，字符串是 UTF-8 编码，也就是字符串中的字符所占的字节数是变化的(1 - 4)
- Rust 不允许通过下标索引字符串（可读字符是变长大小的）
- 遍历字符：
    - 以 Unicode 字符的方式遍历字符串：`for c in "中国人".chars()`
    - 以字节遍历 `for b in "中国人".bytes()`
- 字符串操作
    - 追加：`s.push('r');`
    - 插入 (Insert):
        - `s.insert(5, ',');`
        - `s.insert_str(6, " I like");`
    - 替换 (Replace):
        - `let new_string_replace = string_replace.replace("rust", "RUST");` 不修改当前字符串
        - `let new_string_replacen = string_replace.replacen("rust", "RUST", 1);`  第三个参数则表示替换的个数
        - `string_replace_range.replace_range(7..8, "R");` 修改原始字符串，第一个参数是要替换字符串的范围（Range），第二个参数是新的字符串
    - 删除 (Delete)
        - pop —— 删除并返回字符串的最后一个字符：`let p1 = string_pop.pop();`
        - remove —— 删除并返回字符串中指定索引位置的字符：`string_remove.remove(0);`
            - remove() 方法是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
        - truncate —— 删除字符串中从指定位置开始到结尾的全部字符：`string_truncate.truncate(3);`
            - 是按照字节来处理字符串的，如果参数所给的位置不是合法的字符边界，则会发生错误。
        - clear —— 清空字符串: `string_clear.clear();`
            - 删除字符串中的所有字符，相当于 `truncate()` 方法参数为 0 的时候。
    - 字符串连接 `format!("{} {}!", s1, s2);` 或者 `s = s1 + &s2`
- 字符串转义
    - `let raw_str = r"Escapes don't work here: \x3F \u{211D}";`
    - `let quotes = r#"And then I said: "There is no escape!""#;`
        - 如果字符串包含双引号，可以在开头和结尾加 #
    - `let longer_delimiter = r###"A string with "# in it. And even "##!"###;`
        - 如果还是有歧义，可以继续增加，没有限制

# 结构体
- 初始化实例时，每个字段都需要进行初始化；顺序不要求
- 结构体体互相赋值：**user1 中赋值的字段会发生所有权转移**，基础类型字段不会转移，比如 int；所以会导致 user1.int 可以，user1.String 报错
```rust
let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // 只会提取未赋值的字段，不符覆盖 email
    };
```
- 结构体的内存排列: 
    - 把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段。(每个字段分配独立的堆内存，互不影响)
![](https://pic3.zhimg.com/80/v2-8cc4ed8cd06d60f974d06ca2199b8df5_1440w.png)
- 结构体打印
    - `{}` 的方式打印结构体，那就自己实现 Display 特征
    - `{:?}`，实现 Debug 特征 `#[derive(Debug)]`
        - `println!("rect1 is {:?}", rect1);`
    - 当结构体较大时，我们可能希望能够有更好的输出表现，此时可以使用 `{:#?}` 来替代 `{:?}`
    - `dbg!` 输出到标准错误输出 stderr，而 `println!` 输出到标准输出 stdout
        - `dbg!` 它最终还会把表达式值的所有权返回！可以重新赋值 `width = dbg!(30 * scale)` 

# 数组 array
- array为数组，Vector为动态数组；关系类似 &str 和 String
    - array存储在栈上；Vector存储在堆上
- 数组：长度固定；元素类型相同；依次线性排序
- [T;n]是一个数组类型；[T]是数组切片类型
    - [u8;3]和[u8;4]是不同的类型，数组的长度也是类型的一部分
    - 使用最多的是数组切片[T], 并通过引用的方式`&[T]`
   
# 循环
- `for item in collection`转移所有权
    - `for item in IntoIterator::into_iter(collection)` 
- `for item in &collection`不可变借用
    - `for item in collection.iter()`
- `for item in &mut collection`可变借用
    - `for item in collection.iter_mut()`
- 性能比较
    - `for i in 0..collection.len()`
    - `for item in collection`
    - 性能：第一种使用方式中 collection[index] 的索引访问，会因为边界检查(Bounds Checking)导致运行时的性能损耗 —— Rust 会检查并确认 index 是否落在集合内，但是第二种直接迭代的方式就不会触发这种检查，因为编译器会在编译时就完成分析并证明这种访问是合法的
    - 安全：第一种方式里对 collection 的索引访问是非连续的，存在一定可能性在两次访问之间，collection 发生了变化，导致脏数据产生。而第二种直接迭代的方式是连续访问，因此不存在这种风险（这里是因为所有权吗？是的话可能要强调一下）
- 减少使用索引的循环
 
# 函数/方法

- 方法
    
    ![](https://pica.zhimg.com/80/v2-0d848e960f3279999eab4b1317f6538e_1440w.png)

# Trait
- 特征对象; [./src/33oop/2trait_object_特征对象.rs](./src/33oop/2trait_object_特征对象.rs)
    - 通过特征对象实现类似 「多态」 的特性，以 Trait 为对象封装类型 `Box<dyn Trait>`，所有遵循该 Trait 的对象都可以用 Box 包起来传递进去
- 特征对下动态派发 

  - 泛型是在编译期完成处理的：编译器会为每一个泛型参数对应的具体类型生成一份代码，这种方式是静态分发(static dispatch)，因为是在编译期完成的，对于运行期性能完全没有任何影响。
  - 与静态分发相对应的是动态分发(dynamic dispatch)，在这种情况下，直到运行时，才能确定需要调用什么方法。之前代码中的关键字 `dyn` 正是在强调这一“动态”的特点。
  - 使用特征对象时，Rust 必须使用动态分发。编译器无法知晓所有可能用于特征对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。为此，Rust 在运行时使用特征对象中的指针来知晓需要调用哪个方法。动态分发也阻止编译器有选择的内联方法代码，这会相应的禁用一些优化。
  - 静态分发 `Box<T>` 和动态分发 `Box<dyn Trait>` 的区别:
    - 特征对象**大小不固定**
    - 几乎总是使用特征对象的**引用方式**，如 `&dyn Draw`、`Box<dyn Draw>`
      - 虽然特征对象没有固定大小，但它的引用类型的大小是固定的，它由两个指针组成（ptr 和 vptr），因此占用两个指针大小
      - 一个指针 ptr 指向实现了特征 Draw 的具体类型的实例，也就是当作特征 Draw 来用的类型的实例，比如类型 Button 的实例、类型 SelectBox 的实例
      - 另一个指针 vptr 指向一个虚表 vtable，vtable 中保存了类型 Button 或类型 SelectBox 的实例对于可以调用的实现于特征 Draw 的方法。当调用方法时，直接从 vtable 中找到方法并调用。之所以要使用一个 vtable 来保存各实例的方法，是因为实现了特征 Draw 的类型有多种，这些类型拥有的方法各不相同，当将这些类型的实例都当作特征 Draw 来使用时(此时，它们全都看作是特征 Draw 类型的实例)，有必要区分这些实例各自有哪些方法可调用
  ![](https://pic1.zhimg.com/80/v2-b771fe4cfc6ebd63d9aff42840eb8e67_1440w.jpg)
  - 简而言之，当类型 Button 实现了特征 Draw 时，类型 Button 的实例对象 btn 可以当作特征 Draw 的特征对象类型来使用，btn 中保存了作为特征对象的数据指针（指向类型 Button 的实例数据）和行为指针（指向 vtable）。
  - 一定要注意，此时的 btn 是 Draw 的特征对象的实例，而不再是具体类型 Button 的实例，而且 btn 的 vtable 只包含了实现自特征 Draw 的那些方法（比如 draw），因此 btn 只能调用实现于特征 Draw 的 draw 方法，而不能调用类型 Button 本身实现的方法和类型 Button 实现于其他特征的方法。也就是说，**btn 是哪个特征对象的实例，它的 vtable 中就包含了该特征的方法。**

- 在 Rust 中，有两个self，一个指代当前的实例对象，一个指代特征或者方法类型的别名：`fn draw(&self) -> Self`
- **不是所有特征都能拥有特征对象**，只有对象安全的特征才行。
  - 当一个特征的所有方法都有如下属性时
    - 方法的返回类型不能是 Self, 比如不能有 Clone 方法
    - 方法没有任何泛型参数
  - 主要是类型被抹除了，无法判断
    - 对象安全对于特征对象是必须的，因为一旦有了特征对象，就不再需要知道实现该特征的具体类型是什么了。如果特征方法返回了具体的 Self 类型，但是特征对象忘记了其真正的类型，那这个 Self 就非常尴尬，因为没人知道它是谁了。但是对于泛型类型参数来说，当使用特征时其会放入具体的类型参数：此具体类型变成了实现该特征的类型的一部分。而当使用特征对象时其具体类型被抹去了，故而无从得知放入泛型参数类型到底是什么。
- 高级 Trait
  - 使用 newtype的方式可以打破孤儿规则，给内部类型添加trait 实现
  - supertrait 可以「继承」trait


# Vector
- API:
  - 读取 `get(index)`, 返回 Option 类型 `vec.get(0)`; 或者直接使用下标（会有越界问题）
  - 遍历 `for i in &v` `for i in &mut v`

# HashMap
- 所有权：
  - 若类型实现 Copy 特征，该类型会被复制进 HashMap，因此无所谓所有权
  - 若没实现 Copy 特征，所有权将被转移给 HashMap 中
  - 如果你使用引用类型放入 HashMap 中，请确保该引用的生命周期至少跟 HashMap 活得一样久
- API:
  - 插入 insert `scores.insert(String::from("Blue"), 10);`
  - 获取 get，返回 Option 类型 `let team_name = String::from("Blue"); let score: Option<&i32> = scores.get(&team_name);`
  - 循环 `for (key, value) in &scores`
  - 不存在时插入 or_insert `let count = map.entry(word).or_insert(0);`
    - or_insert 返回了 `&mut v` 引用，因此可以通过该可变引用直接修改 map 中对应的值
    - 使用 count 引用时，需要先进行解引用 *count，否则会出现类型不匹配


