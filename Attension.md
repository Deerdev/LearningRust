
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




        
        
        
        
        
        
        