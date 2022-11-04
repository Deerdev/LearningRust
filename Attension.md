# 指针
- 原始指针`* mut T` `* const T`
```rust
fn main(){
    let mut x=10;
    let ptr_x = &mut x as *mut i32; 
    let y =Box::new(20); 
    let ptry = &*y as *const i32; 
    unsafe {
        *ptr_x += *ptr_y; 
    }
    assert eq!(x, 30);
}
```


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

# 元组
- 可以存放不同类型
- `(1,)` 只有一个元素需要逗号区分


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
- 内存对齐
    - rust 编译器会重排
    - `#repr(c)` 不重排，按 C 语言对齐
```rust
struct A {
    a: u8,
    b: u32,
    c: u16
}
fn main() {
    println!("{:?}", std::mem::size_of::<A>()); // 8
    let v = A {a:1, b:2, c:3};
}
/*
编译器重排字段，优化内存占用
struct A {
    b: u32, // 4
    c: u16, // 2
    a: u8,  // 1  再补齐 1，对齐 4 
    // 总 8 = 4 + 2 + 1 + 1
}

按 c 语言对齐：12 = 4 *3 按最长字段对齐
#repr(c)
struct A {
    a: u8,
    b: u32,
    c: u16
}
*/
```


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
- 特征对象动态派发

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

# 类型转换
- as
    - `let a = 3.1 as i8;`
    - 内存地址转换为指针 `let p1: *mut i32 = values.as_mut_ptr();`
- try_into
    - 强制转换，需要处理错误 `let b_: u8 = b.try_into().unwrap();`
- 点操作符
    - 隐式转换，会自动解引用
- 黑洞：
    - `mem::transmute<T, U>` 将类型 T 直接转成类型 U，唯一的要求就是，这两个类型占用同样大小的字节数
    - `mem::transmute_copy<T, U>` 更加危险和不安全。它从 T 类型中拷贝出 U 类型所需的字节数，然后转换成 U。 mem::transmute 尚有大小检查，能保证两个数据的内存大小一致，transmute_copy 不检查，只不过 U 的尺寸若是比 T 大，会是一个未定义行为。

# 错误处理
- 不可恢复：`panic!("crash and burn");`
    - 如果是 main 线程，则程序会终止，如果是其它子线程，该线程会终止，但是不会影响 main 线程
- 返回 `Result<T, E>`
    - 直接解 Result 是否有报错
```rust
// 错误处理
let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => panic!("Problem opening the file: {:?}", other_error),
    },
};
```
- 强制解，有 error 就会 panic：unwrap 和 expect
    - `File::open("hello.txt").unwrap();`
    - 携带自定义信息 `File::open("hello.txt").expect("Failed to open hello.txt");`

# 包
- module和文件拆分: https://course.rs/basic/crate-module/module.html
- use的使用和可见性: https://course.rs/basic/crate-module/use.html

# 格式化输出
- https://course.rs/basic/formatted-output.html
- `{}` `{:?}` `{:#?}`

# 生命周期
- 生命周期标注并不会改变任何引用的实际作用域，只是为了辅助编译器编译通过
```rust
&i32        // 一个引用
&'a i32     // 具有显式生命周期的引用
&'a mut i32 // 具有显式生命周期的可变引用
// 函数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {}
// 结构体: 结构体 ImportantExcerpt 所引用的字符串 str 必须比该结构体活得更久
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 方法中的生命周期
// impl 中必须使用结构体的完整名称，包括 <'a>，因为生命周期标注也是结构体类型的一部分
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```
- 生命周期消除法则（满足规则就不用标注生命周期）
    - 每一个引用参数都会获得独自的生命周期
        - 例如一个引用参数的函数就有一个生命周期标注: `fn foo<'a>(x: &'a i32)`，两个引用参数的有两个生命周期标注:f`n foo<'a, 'b>(x: &'a i32, y: &'b i32)`, 依此类推。
    - 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期，也就是所有返回值的生命周期都等于该输入生命周期
        - 例如函数 `fn foo(x: &i32) -> &i32`，x 参数的生命周期会被自动赋给返回值 `&i32`，因此该函数等同于 `fn foo<'a>(x: &'a i32) -> &'a i32`
    - 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期
        - 拥有 &self 形式的参数，说明该函数是一个方法，该规则让方法的使用便利度大幅提升。

- 静态生命周期 (能和程序活得一样久) `let s: &'static str = "我没啥优点，就是活得久，嘿嘿";`
- 泛型 + 生命周期 `<'a, T, U>`
```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
- 生命周期会影响变量的销毁：如果生命周期比较长，一个可变引用可能一直存在，导致无法对它进行不可变引用
- **无界生命周期**：不安全代码(unsafe)经常会凭空产生引用或生命周期，这些生命周期被称为是 无界(unbound) 的
    - unsafe 的代码产生的生命周期，该周期是凭空产生的，比'static还强大
```rust
fn f<'a, T>(x: *const T) -> &'a T {
    unsafe {
        &*x
    }
}
```
- **生命周期约束 HRTB**
    - `'a: 'b`: a 的生命周期>= b，`struct DoubleRef<'a,'b:'a, T> {}`
    - `T: 'a`: 表示类型 T 必须比 'a 活得要久 (最新版 rust 可以不写)
- 闭包的生命周期判断比函数困难，所以闭包干不了的，还是换成函数；（闭包的生命期消除规则 和 函数不一致）
```rust
// 函数签名可以明确生命周期，可以根据消除规则推导：如果函数参数中只有一个引用类型，那该引用的生命周期会被自动分配给所有的返回引用
fn fn_elision(x: &i32) -> &i32 { x }
// 闭包无法推断出，闭包内部的逻辑是不定的
let closure_slision = |x: &i32| -> &i32 { x };
```
- Reborrow
```rust
fn main() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    // reborrow! 此时对`r`的再借用不会导致跟上面的借用冲突
    let rr: &Point = &*r;

    // 再借用`rr`最后一次使用发生在这里，在它的生命周期中，我们并没有使用原来的借用`r`，因此不会报错
    //! 如果在使用 rr 前，使用了 r 会报错
    println!("{:?}", rr);

    // 再借用结束后，才去使用原来的借用`r`
    r.move_to(10, 10);
    println!("{:?}", r);
}
```

- 生命周期 impl 块消除
    - `'_` 生命周期表示 BufReader 有一个不使用的生命周期，可以忽略它，无需为它创建一个名称。
    - 既然用不到 'a，为何还要写出来？**生命周期参数也是类型的一部分**，因此 BufReader<'a> 是一个完整的类型，在实现它的时候，你不能把 'a 给丢了！
```rust
impl Reader for BufReader<'_> {
    // methods go here
}
````

# 闭包
- `|x: i32, y: i32| -> i32 { x+y }`
- `|x,y| x+y` 可以根据上下文做类型推断，不写类型，但是推断之后类型就确定了
- `|| x`
- `Fn` 闭包特征，同时也适合函数
```rust
// Fn(u32) -> u32 限定闭包或函数的类型
// T: Fn(u32) -> u32 意味着 query 的类型是 T，该类型必须实现了相应的闭包特征 Fn(u32) -> u32
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}
```
- 三种 Fn 特征：对应函数参数的三种传入方式：转移所有权、可变借用、不可变借用
    - FnOnce，该类型的闭包会拿走被捕获变量的所有权。Once 顾名思义，说明该闭包只能运行一次
```rust
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool,
{
}

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + copy,  // F 增加 Copy 特征，那么 func 参数可以在内部使用多次（参数传入，被 Copy 了）
{
}
```

- 如果你想强制闭包取得捕获变量的所有权，可以在参数列表前添加 move 关键字，这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程
```rust
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});
```

- FnMut，它以可变借用的方式捕获了环境中的值，因此可以修改该值：
```rust
// 需要 mut，下面才可以调用(写法比较凹)
let mut update_string =  |str| s.push_str(str);
update_string("hello");

// 或者 传递给函数处理，参数声明 FnMut
let update_string =  |str| s.push_str(str);
exec(update_string);

fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
    f("hello")
}
```
- Fn 特征，它以不可变借用的方式捕获环境中的值 让我们把上面的代码中 exec 的 F 泛型参数类型修改为 Fn(&'a str), 就会报错

- **一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们**
    - `move` 本身强调的就是后者，闭包如何捕获变量
```rust
let update_string =  move || println!("{}",s);
exec(update_string);

fn exec<F: FnOnce()>(f: F)  {
    f()
}
```
- 一个闭包并不仅仅实现某一种 Fn 特征，规则如下：
    - 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
    - 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
    - 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
- 从源码中还能看出一点：Fn 获取 &self，FnMut 获取 &mut self，而 FnOnce 获取 self。 在实际项目中，**建议先使用 Fn 特征，然后编译器会告诉你正误以及该如何选择**。

- 闭包作为返回值: 必须使用 impl, 因为`Fn` 只是一个 trait，不能确定大小；impl Trait 可以用来返回一个实现了指定特征的类型
```rust
fn factory(x:i32) -> impl Fn(i32) -> i32 {
}

fn factory(x:i32) -> impl Fn(i32) -> i32 {
    let num = 5;
    //! 报错
    // 就算签名一样的闭包，类型也是不同的，因此在这种情况下，就无法再使用 impl Trait 的方式去返回闭包
    if x > 1{
        move |x| x + num
    } else {
        move |x| x - num
    }
    // 解决
    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}
```

# 迭代器
iter 方法类型：
- into_iter 会夺走所有权
- iter 是借用
- iter_mut 是可变借用

Iterator 和 IntoIterator 的区别:
- Iterator 就是迭代器特征，只有实现了它才能称为迭代器，才能调用 next
- IntoIterator 强调的是某一个类型如果实现了该特征，它可以通过 into_iter，iter 等方法变成一个迭代器。

迭代器 v.iter().map().filter()... 是惰性的，需要调用 collect 提取值（触发真实计算：需要一个消费者适配器来收尾，最终将迭代器转换成一个具体的值）
- collection 会根据定义的类型推断 提取成什么类型
- `let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();`
- zip 连接两个迭代器: 形成 `Iterator<Item=(ValueFromA, ValueFromB)>` 这样的新的迭代器，在此处就是形如 [(name1, age1), (name2, age2)] 的迭代器。
- 这里 collect 提取成 HashMap： `let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();`
- 闭包作为适配器参数filter 方法：`shoes.into_iter().filter(|s| s.size == shoe_size).collect()`

```rust
/*
其中 zip，map，filter 是迭代器适配器：
- zip 把两个迭代器合并成一个迭代器，新迭代器中，每个元素都是一个元组，由之前两个迭代器的元素组成。例如将形如 [1, 2, 3, 4, 5] 和 [2, 3, 4, 5] 的迭代器合并后，新的迭代器形如 [(1, 2),(2, 3),(3, 4),(4, 5)]
- map 是将迭代器中的值经过映射后，转换成新的值[2, 6, 12, 20]
- filter 对迭代器中的元素进行过滤，若闭包返回 true 则保留元素[6, 12]，反之剔除

sum 是消费者适配器，对迭代器中的所有元素求和，最终返回一个 u32 值 18
*/
let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
assert_eq!(18, sum);
```

- `.enumerate()` 获取迭代器索引：
```rust
let v = vec![1u64, 2, 3, 4, 5, 6];
for (i,v) in v.iter().enumerate() {
    println!("第{}个值是{}",i,v)
}

// 因为 enumerate 是迭代器适配器，因此我们可以对它返回的迭代器调用其它 Iterator 特征方法：
let v = vec![1u64, 2, 3, 4, 5, 6];
let val = v.iter()
    .enumerate()
    // 每两个元素剔除一个
    // [1, 3, 5]
    .filter(|&(idx, _)| idx % 2 == 0)
    .map(|(idx, val)| val)
    // 累加 1+3+5 = 9
    .fold(0u64, |sum, acm| sum + acm);

println!("{}", val);
```
- 迭代器性能比 for 循环好

# 类型

- 新类型(当成新类型处理)：`struct Meters(u32);`aaa
    - 用于为系统类型添加 Trait 实现
    - 依然可以通过 `vale.0` 获取到原始类型
- 别名(还会当成原来的类型处理)：`type Meters = u32`
  - 可以将复杂类型 type alias 之后，减少重复写的次数
- `!` 不返回类型：
  - rust 默认返回类型是 `()`
  - `panic!` 不会返回类型

- 定长类型 sized：这些类型的大小在编译时是已知的
- 不定长类型( unsized / DST-dynamically sized types)，与定长类型相反，它的大小只有到了程序运行时才能动态获知
  - 因为编译器无法在编译期获知类型大小，若你试图在代码中直接使用 DST 类型，将无法通过编译
  - `Rust 需要明确地知道一个特定类型的值占据了多少内存空间，同时该类型的所有值都必须使用相同大小的内存`。如果 Rust 允许我们使用这种动态类型，那么这两个 str 值就需要占用同样大小的内存，这显然是不现实的: s1 占用了 12 字节，s2 占用了 15 字节，总不至于为了满足同样的内存大小，用空白字符去填补字符串吧？

```rust
// error
let s1: str = "Hello there!";
let s2: str = "How's it going?";

// ok
let s3: &str = "on?"
```
**总结：只能间接使用的 DST**
- Rust 中常见的 DST 类型有: str、[T]、dyn Trait，它们都无法单独被使用，必须要通过引用或者 Box 来间接使用。

```rust
fn foobar_1(thing: &dyn MyThing) {}     // OK
fn foobar_2(thing: Box<dyn MyThing>) {} // OK
fn foobar_3(thing: MyThing) {}          // ERROR!
```

- 每一个特征都是一个可以通过名称来引用的动态大小类型。因此如果想把特征作为具体的类型来传递给函数，你必须将其转换成一个特征对象：诸如 `&dyn Trait` 或者 `Box<dyn Trait>` (还有 `Rc<dyn Trait>`)这些引用类型。

- 如何在泛型函数中使用动态数据类型？`?Sized` 特征: 表明类型 T 既有可能是固定大小的类型，也可能是动态大小的类型

```rust
// 参数类型也改为 &T，动态类型 需要使用指针参数
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```
- `Box<str>`

```rust
fn main() {
    // ERROR: 无法获取 str 的长度，无法使用这种语法进行 Box 进装
    let s1: Box<str> = Box::new("Hello there!" as str);
    // 主动转换成 str 的方式不可行，但是可以让编译器来帮我们完成，只要告诉它我们需要的类型即可。
    let s2: Box<str> = "Hello there!".into();
}
```
整数转为枚举：rust 不支持整数转为枚举（反之可以）

- 方法一：使用三方库 num-traits、num-derive、num_enum
- 方法二：TryFrom + 宏
- 方法三：邪恶之王 std::mem::transmute
```rust
// 使用#[repr(..)]来控制底层类型的大小，免得本来需要 i32，结果传入 i64，最终内存无法对齐，产生奇怪的结果
#[repr(i32)]
enum MyEnum {
    A = 1, B, C
}

let x = MyEnum::C;
let y = x as i32;
let z: MyEnum = unsafe { std::mem::transmute(y) };
```

# 智能指针
智能指针往往是基于结构体实现，它与我们自定义的结构体最大的区别在于它实现了 `Deref` 和 `Drop` 特征：
- `Deref` 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 *T
- `Drop` 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作

主要类型：
- `Box<T>`，可以将值分配到堆上
- `Rc<T>`，引用计数类型，允许多所有权存在
- `Ref<T>` 和 `RefMut<T>`，允许将借用规则检查从编译期移动到运行期进行

## 堆栈的性能
很多人可能会觉得栈的性能肯定比堆高，其实未必。 由于我们在后面的性能专题会专门讲解堆栈的性能问题，因此这里就大概给出结论：
- 小型数据，在栈上的分配性能和读取性能都要比堆上高
- 中型数据，栈上分配性能高，但是读取性能和堆上并无区别，因为无法利用寄存器或 CPU 高速缓存，最终还是要经过一次内存寻址
- 大型数据，只建议在堆上分配和使用

总之，栈的分配速度肯定比堆上快，但是读取速度往往取决于你的数据能不能放入寄存器或 CPU 高速缓存。 因此不要仅仅因为堆上性能不如栈这个印象，就总是优先选择栈，导致代码更复杂的实现。

## Box
Box 相比其它智能指针，功能较为单一，可以在以下场景中使用它：
- 特意的将数据分配在堆上
- 数据较大时，又不想在转移所有权时进行数据拷贝
- 类型的大小在编译期无法确定，但是我们又需要固定大小的类型时(DST -> Sized)
- 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型

```rust
// 分配在堆上
let a = Box::new(3);

// 大数据分配在堆上，copy 时，转义所有权（只 copy 栈指针，不会 copy 堆数据）
let arr = Box::new([0;1000]);
let arr1 = arr; // arr 转移所有权 失效

// DST -> Sized
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 特征对象
let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];
```

```rust
/*
                    (heap)
(stack)    (heap)   ┌───┐
┌──────┐   ┌───┐ ┌─→│ 1 │
│ vec  │──→│B1 │─┘  └───┘
└──────┘   ├───┤    ┌───┐
           │B2 │───→│ 2 │
           └───┘    └───┘
*/
fn main() {
    let arr = vec![Box::new(1), Box::new(2)];
    // 使用 & 借用数组中的元素，否则会报所有权错误
    let (first, second) = (&arr[0], &arr[1]);
    // 表达式不能隐式的解引用，因此必须使用 ** 做两次解引用，第一次将 &Box<i32> 类型转成 Box<i32>，第二次将 Box<i32> 转成 i32
    let sum = **first + **second;
}
```
- Box::leak 它可以消费掉 Box 并且强制目标值从内存中泄漏
    - 简单的场景，你需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久，那么就可以使用 Box::leak，例如有一个存储配置的结构体实例，它是在运行期动态插入内容，那么就可以将其转为全局有效，虽然 Rc/Arc 也可以实现此功能，但是 Box::leak 是性能最高的。
```rust
fn gen_static_str() -> &'static str{
    let mut s = String::new();
    s.push_str("hello, world");
    // 转换成 'static 的变量
    Box::leak(s.into_boxed_str())
}
```

## Deref 解引用
- 一个类型为 T 的对象 foo，如果 `T: Deref<Target=U>`，那么，相关 foo 的引用 &foo 在应用的时候会自动转换为 &U。
- Rust 会在解引用时自动把智能指针和 &&&&v 做引用归一化操作，转换成 &v 形式，最终再对 &v 进行解引用
- 当 `T: Deref<Target=U>`，可以将 &T 转换成 &U，也就是我们之前看到的例子
- 当 `T: DerefMut<Target=U>`，可以将 &mut T 转换成 &mut U
- 当 `T: Deref<Target=U>`，可以将 &mut T 转换成 &U
- Rust 可以把可变引用隐式的转换成不可变引用，但反之则不行。

## Drop 释放资源
- 手动调用 drop `std::mem::drop; drop(xxx)` 获取xxx的所有权，导致后面的代码会报错；不允许直接调用`xxx.drop()`
- Copy 和 Drop 互斥: 无法为一个类型同时实现 Copy 和 Drop 特征。因为实现了 Copy 的特征会被编译器隐式的复制，因此非常难以预测析构函数执行的时间和频率。因此这些实现了 Copy 的类型无法拥有析构函数。

## Rc 与 Arc
- 脱离所有权，使用引用计数 解决一个值有多个所有者，比如线程间共享同一个数据
- Rc 和 Arc，前者适用于单线程，后者适用于多线程; 在不同的模块中：
    - `use std::sync::Arc`
    - `use std::rc::Rc`

```rust
// Arc 和 Rc 使用上一致
// Rc::new 创建一个Rc只能指针a，指向底层字符串数据
let a = Rc::new(String::from("hello, world"));
// Rc::strong_count 获取引用计数
println!("count after creating a = {}", Rc::strong_count(&a));
// Rc::clone 克隆了一份智能指针 Rc<String>，并将该智能指针的引用计数增加到 2。
// clone 仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据
let b = Rc::clone(&a);
```

- RC - 希望在堆上分配一个对象供程序的多个部分使用且无法确定哪个部分最后一个结束时，就可以使用 Rc 成为数据值的所有者
- Rc 简单总结
    - Rc/Arc 是不可变引用，你无法修改它指向的值，只能进行读取，如果要修改，需要配合后面章节的内部可变性 RefCell 或互斥锁 Mutex
    - 一旦最后一个拥有者消失，则资源会自动被回收，这个生命周期是在编译期就确定下来的
    - Rc 只能用于同一线程内部，想要用于线程之间的对象共享，你需要使用 Arc
        - 由于 `Rc<T>` 需要管理引用计数，但是该计数器并没有使用任何并发原语，因此无法实现原子化的计数操作，最终会导致计数错误。
    - `Rc<T>` 是一个智能指针，实现了 Deref 特征，因此你无需先解开 Rc 指针，再使用里面的 T，而是可以直接使用 T，例如上例中的 gadget1.owner.name

- Arc 是 Atomic Rc 的缩写，顾名思义：原子化的 `Rc<T> `智能指针。原子化是一种并发原语。

## Cell 和 RefCell
可以在拥有不可变引用的同时修改目标数据(因为 Rust 使用了 unsafe 来做到这一点)。

- Cell 和 RefCell 在功能上没有区别，区别在于 `Cell<T>` 只适用于 T 实现 Copy 的情况，实际开发中 Cell 使用不多
```rust
let c = Cell::new("asdf");
let one = c.get(); // 获取
c.set("qwer"); // 设置
let two = c.get();
println!("{},{}", one, two); // one two都被修改
```

- RefCell 用于你确信代码是正确的，而编译器却发生了误判时。
    - 当你确信编译器误报但不知道该如何解决时，或者你有一个引用类型，需要被四处使用和修改然后导致借用关系难以管理时，都可以优先考虑使用 RefCell


| Rust 规则 | 智能指针带来的额外规则 |
| -- | -- |
| 一个数据只有一个所有者 | Rc/Arc让一个数据可以拥有多个所有者 |
| 要么多个不可变借用，要么一个可变借用 | RefCell实现编译期可变、不可变引用共存 |
| 违背规则导致编译错误 | 违背规则导致运行时panic |

- Rc/Arc 和 RefCell 合在一起，解决了 Rust 中严苛的所有权和借用规则带来的某些场景下难使用的问题。
- 但是它们并不是银弹，例如 RefCell 实际上并没有解决可变引用和引用可以共存的问题，只是将报错从编译期推迟到运行时，从编译器错误变成了 panic 异常
- RefCell 总结：
    - 与 Cell 用于可 Copy 的值不同，RefCell 用于引用
    - RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
    - RefCell 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
    - 使用 RefCell 时，违背借用规则会导致运行期的 panic


```rust
// 编译不报错，运行时 panic
let s = RefCell::new(String::from("hello, world"));
// 不能绕过借用规则：一个生命周期内可变和不可变不能同时存在
let s1 = s.borrow();
let s2 = s.borrow_mut();
```

- 选择 Cell 还是 RefCell
    - Cell 只适用于 Copy 类型，用于提供值（str），而 RefCell 用于提供引用（String）
    - Cell 不会 panic，而 RefCell 会
    - 性能比较：
        - Cell性能最好
        - RefCell 其实是有一点运行期开销的，原因是它包含了一个字大小的“借用状态”指示器，该指示器在每次运行时借用时都会被修改，进而产生一点开销。
- 当非要使用内部可变性时，首选 Cell，只有你的类型没有实现 Copy 时，才去选择 RefCell。

## Rc + RefCell 使用
```rust
// 们使用 RefCell<String> 包裹一个字符串，同时通过 Rc 创建了它的三个所有者：s、s1和s2，并且通过其中一个所有者 s2 对字符串内容进行了修改。
let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

let s1 = s.clone();
let s2 = s.clone();
// 可变借用，修改值
// 由于 Rc 的所有者们共享同一个底层的数据，因此当一个所有者修改了数据时，会导致全部所有者持有的数据都发生了变化
s2.borrow_mut().push_str(", on yeah!");


// RC + RefCell 结合的数据结构与下面类似
struct Wrapper<T> {
    // Rc
    strong_count: usize,
    weak_count: usize,

    // Refcell
    borrow_count: isize,

    // 包裹的数据
    item: T,
}
```

## 通过 Cell::from_mut 解决借用冲突
在 Rust 1.37 版本中新增了两个非常实用的方法：

- `Cell::from_mut`，该方法将 `&mut T` 转为 `&Cell<T>`
- `Cell::as_slice_of_cells`，该方法将 `&Cell<[T]>` 转为 `&[Cell<T>]`

```rust
// 报错，nums 同时借用了不可变与可变引用
fn retain_even(nums: &mut Vec<i32>) {
    let mut i = 0;
    // nums 不可变
    for num in nums.iter().filter(|&num| is_even(*num)) {
        // nums可变
        nums[i] = *num;
        i += 1;
    }
    nums.truncate(i);
}

fn retain_even(nums: &mut Vec<i32>) {
    // 转成不可变slice
    let slice: &[Cell<i32>] = Cell::from_mut(&mut nums[..])
        .as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        // Cell 上的 set 方法获取的是不可变引用 `pub fn set(&self, val: T)`
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}
```
## 总结
- Cell 和 RefCell 都为我们带来了内部可变性这个重要特性，同时还将借用规则的检查从编译期推迟到运行期，但是这个检查并不能被绕过，该来早晚还是会来，RefCell 在运行期的报错会造成 panic。
    - 内部可见性：（对一个不可变的值进行可变借用）对外的接口参数都是不可变引用（`&self`），但是内部可以修改（`self.msg_cache.borrow_mut().push(msg)` msg_cache 是 RefCell）
- RefCell 适用于编译器误报或者一个引用被在多个代码中使用、修改以至于难于管理借用关系时，还有就是需要内部可变性时。
- 从性能上看，RefCell 由于是**非线程安全的**，因此无需保证原子性，性能虽然有一点损耗，但是依然非常好，而 Cell 则完全不存在任何额外的性能损耗。
- Rc 跟 RefCell 结合使用可以实现多个所有者共享同一份数据，非常好用，但是潜在的性能损耗也要考虑进去，建议对于热点代码使用时，做好 benchmark。

# 循环引用 x Weak
- Rc 多个指向一个
- RefCell 可修改
```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    // 这个类型很有意思，它的每个值都指向了另一个 List，此外，得益于 Rc 的使用还允许多个值指向一个 List
    // 同时，由于 RefCell 的使用，每个 List 所指向的 List 还能够被修改。
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {}
```

Weak 作用：
- 持有一个 Rc 对象的临时引用，并且不在乎引用的值是否依然存在; 返回是 `Option<Rc<T>>`
- 阻止 Rc 导致的循环引用，因为 Rc 的所有权机制，会导致多个 Rc 都无法计数归零
    - 对于父子引用关系，可以让父节点通过 Rc 来引用子节点，然后让子节点通过 Weak 来引用父节点。

Weak 通过 use std::rc::Weak 来引入，它具有以下特点:
- 可访问，但没有所有权，不增加引用计数，因此不会影响被引用值的释放回收
- 可由 `Rc<T>` 调用 downgrade 方法转换成 `Weak<T>`
- `Weak<T>` 可使用 upgrade 方法转换成 `Option<Rc<T>>`，如果资源已经被释放，则 Option 的值是 None
- 常用于解决循环引用的问题
```rust
// 创建Rc，持有一个值5
let five = Rc::new(5);

// 通过Rc，创建一个Weak指针
let weak_five = Rc::downgrade(&five);
// Weak引用的资源依然存在，取到值5
let strong_five: Option<Rc<_>> = weak_five.upgrade();
assert_eq!(*strong_five.unwrap(), 5);
```
解决循环引用
```rust
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

// 主人
struct Owner {
    name: String,
    gadgets: RefCell<Vec<Weak<Gadget>>>,
}

// 工具
struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

fn main() {
    // 创建一个 Owner
    // 需要注意，该 Owner 也拥有多个 `gadgets`
    let gadget_owner : Rc<Owner> = Rc::new(
        Owner {
            name: "Gadget Man".to_string(),
            gadgets: RefCell::new(Vec::new()),
        }
    );

    // 创建工具，同时与主人进行关联：创建两个 gadget，他们分别持有 gadget_owner 的一个引用。
    let gadget1 = Rc::new(Gadget{id: 1, owner: gadget_owner.clone()});
    let gadget2 = Rc::new(Gadget{id: 2, owner: gadget_owner.clone()});

    // 为主人更新它所拥有的工具
    // 因为之前使用了 `Rc`，现在必须要使用 `Weak`，否则就会循环引用
    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1));
    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));

    // 遍历 gadget_owner 的 gadgets 字段
    for gadget_opt in gadget_owner.gadgets.borrow().iter() {

        // gadget_opt 是一个 Weak<Gadget> 。 因为 weak 指针不能保证他所引用的对象
        // 仍然存在。所以我们需要显式的调用 upgrade() 来通过其返回值(Option<_>)来判
        // 断其所指向的对象是否存在。
        // 当然，Option 为 None 的时候这个引用原对象就不存在了。
        let gadget = gadget_opt.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }

    // 在 main 函数的最后，gadget_owner，gadget1 和 gadget2 都被销毁。
    // 具体是，因为这几个结构体之间没有了强引用（`Rc<T>`），所以，当他们销毁的时候。
    // 首先 gadget2 和 gadget1 被销毁。
    // 然后因为 gadget_owner 的引用数量为 0，所以这个对象可以被销毁了。
    // 循环引用问题也就避免了
}
```

## 结构体自引用
> [结构体自引用](https://course.rs/advance/circle-self-ref/self-referential.html)

同时存储值又存储指针
```rust
struct SelfRef<'a> {
    value: String,
    // 该引用指向上面的value
    pointer_to_value: &'a str,
}
```

# 多线程
```rust
// 闭包执行
let handle = thread::spawn(|| {});
thread::sleep(Duration::from_millis(1));

// 等待子线程的结束
handle.join().unwrap();
```

- move: 转移变量给线程；Rust 无法确定新的线程会活多久（多个线程的结束顺序并不是固定的），所以也无法确定新线程所引用的 v 是否在使用过程中一直合法
```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});
```

- main线程结束后，子线程会被杀掉；如果父线程不是main，那么父线程结束后，子线程还是会继续运行，直到子线程的代码运行完成或者 main 线程的结束
- 无锁写入 吞吐并不是跟随线程数线性增长的：
    - 虽然是无锁，但是内部是 CAS 实现，大量线程的同时访问，会让 CAS 重试次数大幅增加
    - 线程过多时，CPU 缓存的命中率会显著下降，同时多个线程竞争一个 CPU Cache-line 的情况也会经常发生
    - 大量读写可能会让内存带宽也成为瓶颈
    - 读和写不一样，无锁数据结构的读往往可以很好地线性增长，但是写不行，因为写竞争太大

- 线程屏障(Barrier): 让线程卡在某一步，等待大家全部做完一些前置工作
```rust
// 全部执行完before wait之后，才会开始 after wait
let mut handles = Vec::with_capacity(6);
let barrier = Arc::new(Barrier::new(6));

for _ in 0..6 {
    let b = barrier.clone();
    handles.push(thread::spawn(move|| {
        println!("before wait");
        b.wait();
        println!("after wait");
    }));
}

for handle in handles {
    handle.join().unwrap();
}
```
- 线程局部变量：每个线程获得copy，独立使用，不会互相影响`thread_local!(static FOO: RefCell<u32> = RefCell::new(1));`
```rust
// 通过 with 获取值
FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 3;
    });
```

- 用条件控制线程的挂起和执行：
```rust
use std::thread;
use std::sync::{Arc, Mutex, Condvar};

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move|| {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}

```
- 只被调用一次的函数
```rust
static mut VAL: usize = 0;
static INIT: Once = Once::new();
let handle1 = thread::spawn(move || {
    INIT.call_once(|| {
        unsafe {
            VAL = 1;
        }
    });
});
```


## 通信 / 通道
- rust 有 channel
```rust
// mpsc是multiple producer, single consumer的缩写, 多发送至，单接收者
use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个数字1, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        tx.send(1).unwrap();

        // 下面代码将报错，因为编译器自动推导出通道传递的值是i32类型，那么Option<i32>类型将产生不匹配错误
        // tx.send(Some(1)).unwrap()
    });

    // 阻塞：在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
    // 不阻塞 try_recv, 没有数据返回 Err
    /*
    ···
    receive Err(Empty)
    receive Ok(1)
    receive Err(Disconnected) 发送端断开连接
    ···
    */
    println!("receive {:?}", rx.try_recv());
}
```
- 使用通道来传输数据，一样要遵循 Rust 的所有权规则：
    - 若值的类型实现了Copy特征，则直接复制一份该值，然后传输过去，例如之前的i32类型
    - 若值没有实现Copy，则它的所有权会被转移给接收端，在发送端继续使用该值将报错

- 循环发送和接收
```rust
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});
// 主线程使用for循环阻塞的从rx迭代器中接收消息，当子线程运行完成时，发送者tx会随之被drop，此时for循环将被终止，最终main线程成功结束。
for received in rx {
    println!("Got: {}", received);
}
```

- 使用多发送者: clone tx
    - 需要所有的发送者都被drop掉后，接收者rx才会收到错误，进而跳出for循环，最终结束主线程
    - 这里虽然用了clone但是并不会影响性能，因为它并不在热点代码路径中，仅仅会被执行一次
    - 由于两个子线程谁先创建完成是未知的，因此哪条消息先发送也是未知的，最终主线程的输出顺序也不确定
```rust
fn main() {
    let (tx, rx) = mpsc::channel();
    // 由于子线程会拿走发送者的所有权，因此我们必须对发送者进行克隆，然后让每个线程拿走它的一份拷贝
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx1.send(String::from("hi from cloned tx")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```
- 消息顺序: channel 中的消息是有序的；对于通道而言，消息的发送顺序和接收顺序是一致的，满足FIFO原则(先进先出)。
- 同步和异步通道（针对发送端）
    - mpsc::channel 默认是异步发送通道
    - mpsc::sync_channel 同步通道发送消息是阻塞的，只有在消息被接收后才解除阻塞 `let (tx, rx)= mpsc::sync_channel(0);`; 有接收消息彻底成功后，发送消息才算完成
- 消息缓存
    - 同步 channel `mpsc::sync_channel(N)` 可以指定缓存 N 个消息, 超过 N 之后，才会触发阻塞
    - 异步 channel 不需要指定，异步通道的缓冲上限取决于你的内存大小，不要撑爆就行。

- 关闭通道：所有发送者被drop或者所有接收者被drop后，通道会自动关闭；（这件事是在编译期实现的，完全没有运行期性能损耗！只能说 Rust 的Drop特征 YYDS!）
```rust
use std::sync::mpsc;
fn main() {

    use std::thread;

    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // main 函数无法结束
    //! 需要在这里drop send; 不然 send 要到 main 结束才会释放；但是下面的 for 又阻塞等待 send 的释放/关闭

    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}
```

- 通道支持传输多种类型的数据：使用枚举
```rust
enum Fruit {
    Apple(u8),
    Orange(String)
}
q
let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();
```
- 多发送者，多接受者：使用三方库
    - crossbeam-channel, 老牌强库，功能较全，性能较强，之前是独立的库，但是后面合并到了crossbeam主仓库中
    - flume, 官方给出的性能数据某些场景要比 crossbeam 更好些

## 线程同步：锁、Condvar 和信号量
- **消息传递**类似一个单所有权的系统：一个值同时只能有一个所有者，如果另一个线程需要该值的所有权，需要将所有权通过消息传递进行转移。
- 而**共享内存**类似于一个多所有权的系统：多个线程可以同时访问同一个值。

### Mutex
- 互斥锁Mutex(mutual exclusion 的缩写)。Mutex让多个线程并发的访问同一个值变成了排队访问：同一时间，只允许一个线程A访问该值，其它线程需要等待A访问完成后才能继续。
- 锁在多线程中使用`Arc`传递（而不是 RC，因为 RC 不支持多线程）
```rust
let counter = Arc::new(Mutex::new(0));
let counter = Arc::clone(&counter);
thread::spawn(move || {
    let mut num = counter.lock().unwrap();
    *num += 1;
});
```
- `Rc<T>/RefCell<T>`用于单线程内部可变性， `Arc<T>/Mutex<T>`用于多线程内部可变性。
    - 用不可变`Rc<T>/Arc<T>`包裹可变`RefCell<T>/Mutex<T>`，实现内部可变性

- 使用 try_lock 解决死锁, try_lock会尝试去获取一次锁，如果无法获取会返回一个错误，因此不会发生阻塞
    - 在 Rust 标准库中，使用`try_xxx`都会尝试进行一次操作，如果无法完成，就立即返回，不会发生阻塞。例如消息传递章节中的`try_recv`以及本章节中的`try_lock`

### RwLock
- 读写锁：可以同时读，但是不能同时`读写`
    - 读可以使用read、try_read，写write、try_write, 在实际项目中，try_xxx会安全的多
```rust
let lock = RwLock::new(5);
let r1 = lock.read().unwrap();
let mut w = lock.write().unwrap();
// 可以使用try_write和try_read来尝试进行一次写/读，若失败则返回错误.
```

### 锁性能对比
RwLock 使用的问题：
- 读和写不能同时发生，如果使用try_xxx解决，就必须做大量的错误处理和失败重试机制
- 当读多写少时，写操作可能会因为一直无法获得锁导致连续多次失败(writer starvation)
- RwLock 其实是操作系统提供的，实现原理要比Mutex复杂的多，因此单就锁的性能而言，比不上原生实现的Mutex

两个锁的区别:
- 追求高并发读取时，使用RwLock，因为Mutex一次只允许一个线程去读取
- 如果要保证写操作的成功性，使用Mutex
- 不知道哪个合适，统一使用Mutex

性能:
- 一个常见的、错误的使用RwLock的场景就是使用HashMap进行简单读写，因为HashMap的读和写都非常快，RwLock的复杂实现和相对低的性能反而会导致整体性能的降低，因此一般来说更适合使用Mutex。
- 如果你要使用RwLock要确保满足以下两个条件：并发读，且需要对读到的资源进行"长时间"的操作，HashMap也许满足了并发读的需求，但是往往并不能满足后者："长时间"的操作。

标准库在设计时总会存在取舍，因为往往性能并不是最好的，如果你追求性能，可以使用三方库提供的并发原语:
- parking_lot, 功能更完善、稳定，社区较为活跃，star 较多，更新较为活跃
-  spin, 在多数场景中性能比parking_lot高一点，最近没怎么更新
如果不是追求特别极致的性能，建议选择前者。

### Condvar 条件控制
```rust
use std::sync::{Arc,Mutex,Condvar};
use std::thread::{spawn,sleep};
use std::time::Duration;

fn main() {
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = spawn(move || {
        let mut m = { *cflag.lock().unwrap() };
        let mut counter = 0;

        while counter < 3 {
            while !m {
                // 等待条件被激活
                m = *ccond.wait(cflag.lock().unwrap()).unwrap();
            }

            {
                m = false;
                *cflag.lock().unwrap() = false;
            }

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        // 触发条件
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag);
}
/*
outside counter: 1
inner counter: 1
outside counter: 2
inner counter: 2
outside counter: 3
inner counter: 3
Mutex { data: true, poisoned: false, .. }
*/
```

### Semaphore
- 使用它可以让我们精准的控制当前正在运行的任务最大数量
```rust
// 本来 Rust 在标准库中有提供一个信号量实现, 但是由于各种原因这个库现在已经不再推荐使用了，因此我们推荐使用tokio中提供的Semaphore实现: tokio::sync::Semaphore。
use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = Vec::new();

    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            // do sth...
            // 释放
            drop(permit);
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
```

### Atomic 原子类型与内存顺序
- 原子指的是一系列不可被 CPU 上下文交换的机器指令，这些指令组合在一起就形成了原子操作。在多核 CPU 下，当某个 CPU 核心开始运行原子操作时，`会先暂停其它 CPU 内核对内存的操作，以保证原子操作不会被其它 CPU 内核所干扰`。
- 由于原子操作是通过指令提供的支持，因此它的`性能`相比锁和消息传递会好很多。相比较于锁而言，原子类型不需要开发者处理加锁和释放锁的问题，同时支持修改，读取等操作，还具备较高的并发性能，几乎所有的语言都支持原子类型。
- 可以看出原子类型是无锁类型，但是无锁不代表无需等待，因为原子类型内部使用了CAS循环，当大量的冲突发生时，该等待还是得等待！但是总归比锁要好。
    - CAS 全称是 Compare and swap, 它通过一条指令读取指定的内存地址，然后判断其中的值是否等于给定的前置值，如果相等，则将其修改为新的值

- 内存排序：[内存排序](https://course.rs/advance/concurrency-with-threads/sync2.html#%E5%86%85%E5%AD%98%E9%A1%BA%E5%BA%8F)
- 内存顺序的选择
    - 不知道怎么选择时，优先使用SeqCst，虽然会稍微减慢速度，但是慢一点也比出现错误好
    - 多线程只计数fetch_add而不使用该值触发其他逻辑分支的简单使用场景，可以使用Relaxed，参考 [Which std::sync::atomic::Ordering to use?](https://stackoverflow.com/questions/30407121/which-stdsyncatomicordering-to-use)

```rust
// 多线程中使用
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};

fn main() {
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move|| {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // 等待其它线程释放锁
    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}
```

- Atomic 能替代锁吗
    - 对于复杂的场景下，锁的使用简单粗暴，不容易有坑
    - std::sync::atomic包中仅提供了数值类型的原子操作：AtomicBool, AtomicIsize, AtomicUsize, AtomicI8, AtomicU16等，而锁可以应用于各种类型
    - 在有些情况下，必须使用锁来配合，例如上一章节中使用Mutex配合Condvar
- Atomic 的应用场景：高性能库 基础库
    - 无锁(lock free)数据结构
    - 全局变量，例如全局自增 ID, 在后续章节会介绍
    - 跨线程计数器，例如可以用于统计指标

## Send & Sync 特征
Send和Sync是 Rust 安全并发的重中之重，但是实际上它们只是标记特征(marker trait，该特征未定义任何行为，因此非常适合用于标记), 来看看它们的作用：
- 实现Send的类型可以在线程间安全的传递其所有权
- 实现Sync的类型可以在线程间安全的共享(通过引用)(多线程间共享一个值)

```rust
// Rc源码片段:
// !代表移除特征的相应实现，代码中Rc<T>的Send和Sync特征被特地移除了实现
impl<T: ?Sized> !marker::Send for Rc<T> {}
impl<T: ?Sized> !marker::Sync for Rc<T> {}

// Arc源码片段
unsafe impl<T: ?Sized + Sync + Send> Send for Arc<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for Arc<T> {}
```
- Rust 中绝大多数类型都实现了Send和Sync，除了以下几个(事实上不止这几个，只不过它们比较常见):
    - 裸指针两者都没实现，因为它本身就没有任何安全保证
    - UnsafeCell不是Sync，因此Cell和RefCell也不是
    - Rc两者都没实现(因为内部的引用计数器不是线程安全的)

- 实现Send的类型可以在线程间安全的传递其所有权, 实现Sync的类型可以在线程间安全的共享(通过引用)
- 绝大部分类型都实现了Send和Sync，常见的未实现的有：裸指针、Cell、RefCell、Rc 等
- 可以为自定义类型实现Send和Sync，但是需要unsafe代码块
- 可以为部分 Rust 中的类型实现Send、Sync，但是需要使用newtype，例如文中的裸指针例子
```rust
#[derive(Debug)]
struct MyBox(*const u8);
unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}
```

# 全局变量
```rust

// 静态变量需要在 unsafe 里使用（比如多线程）
// 只有在同一线程内或者不在乎数据的准确性时，才应该使用全局静态变量。
static mut REQUEST_RECV: usize = 0;
unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
   }

// 想要全局计数器、状态控制等功能，又想要线程安全的实现，原子类型是非常好的办法。
use std::sync::atomic::{AtomicUsize, Ordering};
static REQUEST_RECV: AtomicUsize  = AtomicUsize::new(0);
```

静态变量和常量的区别
- 静态变量不会被内联，在整个程序中，静态变量只有一个实例，所有的引用都会指向同一个地址
- 存储在静态变量中的值必须要实现 Sync trait

## 运行期初始化 lazy_static
- lazy_static是社区提供的非常强大的宏，用于懒初始化静态变量, 允许我们在运行期初始化静态变量！
    - 会有轻微的性能损失，类似于单例，内部使用了std::sync::Once
```rust
use lazy_static::lazy_static;
lazy_static! {
    // 匹配的是static ref，所以定义的静态变量都是不可变引用
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
}
```
- `Box::leak`: 将局部变量赋值给全局变量，提升声明周期
- [不稳定]在 Rust 标准库中提供 lazy::OnceCell 和 lazy::SyncOnceCell 两种 Cell，前者用于单线程，后者用于多线程，它们用来存储堆上的信息，并且具有最多只能赋值一次的特性：[标准库中的 OnceCell](https://course.rs/advance/global-variable.html#%E6%A0%87%E5%87%86%E5%BA%93%E4%B8%AD%E7%9A%84-oncecell)


# 错误处理
> https://course.rs/advance/errors.html
- 自定义错误类型
- 使用 From trait 转换不同类型
- 定义不同错误类型的返回
    - 使用特征对象 `Box<dyn Error>`
    - 自定义错误类型
    - 使用 thiserror
- 关于如何选用 thiserror 和 anyhow 只需要遵循一个原则即可：是否关注自定义错误消息，关注则使用 thiserror（常见业务代码），否则使用 anyhow（编写第三方库代码）。

# unsafe 简介
首先，unsafe 并不能绕过 Rust 的借用检查，也不能关闭任何 Rust 的安全检查规则，例如当你在 unsafe 中使用引用时，该有的检查一样都不会少。

因此 unsafe 能给大家提供的也仅仅是 5 种超能力，在使用这 5 种能力时，编译器才不会进行内存安全方面的检查，最典型的就是使用裸指针(引用和裸指针有很大的区别)。
- 解引用裸指针
- 调用一个 unsafe 或外部的函数
- 访问或修改一个可变的静态变量
- 实现一个 unsafe 特征
- 访问 union 中的字段

## 解引用裸指针
- 区别引用：裸指针(raw pointer，又称原生指针) 在功能上跟引用类似，同时它也需要显式地注明可变性。但是又和引用有所不同，裸指针长这样: *const T 和 *mut T，它们分别代表了不可变和可变。
- `* 操作符` 可以用于解引用，但是在裸指针 `*const T` 中，这里的 * 只是类型名称的一部分，并没有解引用的含义。

指针的概念：引用、智能指针和裸指针。与前两者不同，裸指针：
- 可以绕过 Rust 的借用规则，可以同时拥有一个数据的可变、不可变指针，甚至还能拥有多个可变的指针
- 并不能保证指向合法的内存
- 可以是 null
- 没有实现任何自动的回收 (drop)

```rust
/// 1.基于值引用 创建可变和不可变裸指针
let mut num = 5;

// 创建不需要使用 unsafe
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
// 创建裸指针是安全的行为，而解引用裸指针才是不安全的行为

/// 2.基于内存地址创建裸指针
let address = 0x012345usize;
let r = address as *const i32;

// 在指定的内存地址读取字符串
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
  unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

/// 3.使用 * 解引用
let a = 1;
let b: *const i32 = &a as *const i32;
let c: *const i32 = &a; // 可以不使用 as，但是最好使用，表示在使用裸指针
unsafe {
    println!("{}", *c);
}

/// 4.基于智能指针创建裸指针
let a: Box<i32> = Box::new(10);
// 需要先解引用a
let b: *const i32 = &*a;
// 使用 into_raw 来创建
let c: *const i32 = Box::into_raw(a);
```

- `unsafe fn` 定义的函数必须在 unsafe 中执行，在 unsafe 函数中需要使用 unsafe 代码块

```rust
// 使用裸指针：将数据分成两个可变的切片
// 类似于 C 的使用方式
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            // ptr.add(mid) 可以获取第二个切片的初始地址，由于切片中的元素是 i32 类型，每个元素都占用了 4 个字节的内存大小，因此我们不能简单的用 ptr + mid 来作为初始地址，而应该使用 ptr + 4 * mid，但是这种使用方式并不安全，因此 .add 方法是最佳选择
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```
## FFI
FFI（Foreign Function Interface）可以用来与其它语言进行交互，但是并不是所有语言都这么称呼，例如 Java 称之为 JNI（Java Native Interface）。
- unsafe 的另一个重要目的就是对 FFI 提供支持，它的全称是 Foreign Function Interface，顾名思义，通过 FFI , 我们的 Rust 代码可以跟其它语言的外部代码进行交互。
```rust
// C 语言的代码定义在了 extern 代码块中， 而 extern 必须使用 unsafe 才能进行进行调用，原因在于其它语言的代码并不会强制执行 Rust 的规则，因此 Rust 无法对这些代码进行检查，最终还是要靠开发者自己来保证代码的正确性和程序的安全性。
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```
## ABI
在 `exetrn "C"` 代码块中，我们列出了想要调用的外部函数的签名。其中 `"C"` 定义了外部函数所使用的应用二进制接口`ABI (Application Binary Interface)`：ABI 定义了如何在汇编层面来调用该函数。在所有 ABI 中，C 语言的是最常见的。

## 在其它语言中调用 Rust 函数
可以使用 extern 来创建一个接口，其它语言可以通过该接口来调用相关的 Rust 函数
```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```
- 上面的代码可以让 `call_from_c` 函数被 C 语言的代码调用，当然，前提是将其编译成一个**共享库**，然后链接到 C 语言中
- `#[no_mangle]`，它用于告诉 Rust 编译器：不要乱改函数的名称
    - Mangling 的定义是：当 Rust 因为编译需要去修改函数的名称，例如为了让名称包含更多的信息，这样其它的编译部分就能从该名称获取相应的信息，这种修改会导致函数名变得相当不可读。

## 实现 unsafe 特征
> Send 和 Sync 特征就是
- 为什么有 unsafe 的特征？
    - 是因为该特征至少有一个方法包含有编译器无法验证的内容
```rust
unsafe trait Foo {
    // 方法列表
}
// unsafe impl 告诉编译器：相应的正确性由我们自己来保证。
unsafe impl Foo for i32 {
    // 实现相应的方法
}
```
## 访问 union 中的字段
- union 主要用于跟 C 代码进行交互。
- 访问 union 的字段是不安全的，因为 Rust 无法保证当前存储在 union 实例中的数据类型。
```rust
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}
```

## 三方库
- rust-bindgen 用于在 Rust 中访问 C 代码
- cbindgen 用于在 C 中访问 Rust 代码
- cxx 用于 C++ Rust 双向调用，而且不用加 unsafe
- Miri 生成 rust 编译中间层 MIR


[Unsafe Rust: How and when (not) to use it - LogRocket Blog](https://blog.logrocket.com/unsafe-rust-how-and-when-not-to-use-it/)

# Macro 宏编程
- `println!` 后面跟着的是 ()，而 `vec!` 后面跟着的是 []，这是因为宏的参数可以使用 `()、[] 以及 {}`:
- 在 Rust 中宏分为两大类：声明式宏( declarative macros ) macro_rules! 和三种过程宏( procedural macros ):
    - `#[derive]`，在之前多次见到的派生宏，可以为目标结构体或枚举派生指定的代码，例如 Debug 特征
    - 类属性宏(Attribute-like macro)，用于为目标添加自定义的属性
    - 类函数宏(Function-like macro)，看上去就像是函数调用

相比函数的优点：
- 元编程：通过一种代码来生成另一种代码
- 支持可变参数：函数限制参数类型

## 声明式宏 macro_rules!
- 声明式宏允许我们写出类似 match 的代码。match 表达式是一个控制结构，其接收一个表达式，然后将表达式的结果与多个模式进行匹配，一旦匹配了某个模式，则该模式相关联的代码将被执行
- 宏里的值是一段 Rust 源代码(字面量)，模式用于跟这段源代码的结构相比较，一旦匹配，传入宏的那段源代码将被模式关联的代码所替换，最终实现宏展开。值得注意的是，所有的这些都是在编译期发生，并没有运行期的性能损耗。

- `#[macro_export]` 注释将宏进行了导出
- 使用 `macro_rules!` 进行了宏定义
- vec 的定义结构跟 match 表达式很像，但这里我们只有一个分支，其中包含一个模式 `( $( $x:expr ),* )`，跟模式相关联的代码就在 => 之后。一旦模式成功匹配，那这段相关联的代码就会替换传入的源代码。


```rust
-   // vec!
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```
## 模式解析
- 虽然宏和 match 都称之为模式，但是前者跟后者的模式规则是不同的
- `( $( $x:expr ),* )` 的含义。
    - 首先，我们使用圆括号 `()` 将整个宏模式包裹其中。紧随其后的是 `$()`，跟括号中模式相匹配的值(传入的 Rust 源代码)会被捕获，然后用于代码替换。在这里，模式 `$x:expr` 会匹配任何 Rust 表达式并给予该模式一个名称：`$x`。
    - `$()` 之后的逗号说明在 `$()` 所匹配的代码的后面会有一个可选的逗号分隔符，紧随逗号之后的 `*` 说明 `*` 之前的模式会被匹配零次或任意多次(类似正则表达式)。

- 当我们使用 `vec![1, 2, 3]` 来调用该宏时，`$x` 模式将被匹配三次，分别是 1、2、3。为了帮助大家巩固，我们再来一起过一下：
    - `$(`) 中包含的是模式 `$x:expr`，该模式中的 expr 表示会匹配任何 Rust 表达式，并给予该模式一个名称 `$x`
    - 因此 `$x` 模式可以跟整数 1 进行匹配，也可以跟字符串 "hello" 进行匹配: vec!["hello", "world"]
    - `$()` 之后的逗号，意味着1 和 2 之间可以使用逗号进行分割，也意味着 3 既可以没有逗号，也可以有逗号：vec![1, 2, 3,]
    - `*` 说明之前的模式可以出现零次也可以任意次，这里出现了三次

```rust
{
    {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }
};
/*
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
*/
```
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)

## 用过程宏为属性标记生成代码
过程宏 ( procedural macros )，从形式上来看，过程宏跟函数较为相像，但过程宏是使用源代码作为输入参数，基于代码进行一系列操作后，再输出一段全新的代码。
- 注意，过程宏中的 derive 宏输出的代码并不会替换之前的代码，这一点与声明宏有很大的不同！

当创建过程宏时，它的定义**必须要放入一个独立的包中**，且包的类型也是特殊的，这么做的原因相当复杂，大家只要知道这种限制在未来可能会有所改变即可。
- 事实上，根据[这个说法](https://www.reddit.com/r/rust/comments/t1oa1e/what_are_the_complex_technical_reasons_why/)，过程宏放入独立包的原因在于它必须先被编译后才能使用，如果过程宏和使用它的代码在一个包，就必须先单独对过程宏的代码进行编译，然后再对我们的代码进行编译，但悲剧的是 Rust 的编译单元是包，因此你无法做到这一点。
- 用于定义过程宏的函数 some_name 使用 TokenStream 作为输入参数，并且返回的也是同一个类型。TokenStream 是在 proc_macro 包中定义的，顾名思义，它代表了一个 Token 序列。
    - sync: https://docs.rs/syn/1.0.98/syn/struct.DeriveInput.html
    - quote: https://docs.rs/quote/latest/quote/


```rust
use proc_macro;

#[proc_macro_derive(HelloMacro)]
pub fn some_name(input: TokenStream) -> TokenStream {}
```
```rust
/// 1. 自定义 derive 过程宏 `#[derive(HelloMacro)]`
pub trait HelloMacro {
    fn hello_macro();
}
// 创建单独的 lib 包，名称必须以 `_derive` 结尾 `cargo new hello_macro_derive --lib`
// syn 和 quote 库
// - syn 将字符串形式的 Rust 代码解析为一个 AST 树的数据结构，该数据结构可以在随后的 impl_hello_macro 函数中进行操作
// - 操作的结果又会被 quote 包转换回 Rust 代码
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast = syn::parse(input).unwrap();

    // 构建特征实现代码
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
```
其中 `stringify!` 是 Rust 提供的内置宏，可以将一个表达式(例如 1 + 2)在编译期转换成一个字符串字面值("1 + 2")，该字面量会直接打包进编译出的二进制文件中，具有 'static 生命周期。而 `format!` 宏会对表达式进行求值，最终结果是一个 String 类型。在这里使用 stringify! 有两个好处:
- #name 可能是一个表达式，我们需要它的字面值形式
- 可以减少一次 String 带来的内存分配

## 类属性宏(Attribute-like macros)
类属性过程宏跟 derive 宏类似，但是前者允许我们定义自己的属性。除此之外，derive 只能用于结构体和枚举，而类属性宏可以用于其它类型项

与 derive 宏不同，类属性宏的定义函数有两个参数：
- 第一个参数时用于说明属性包含的内容：Get, "/" 部分
- 第二个是属性所标注的类型项，在这里是 fn index() {...}，注意，函数体也被包含其中

```rust
#[route(GET, "/")]
fn index() {}


#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
```

## 类函数宏(Function-like macros)
类函数宏可以让我们定义像函数那样调用的宏，从这个角度来看，它跟声明宏 macro_rules 较为类似。

区别在于，macro_rules 的定义形式与 match 匹配非常相像，而类函数宏的定义形式则类似于之前讲过的两种过程宏:
- 为何我们不使用声明宏 macro_rules 来定义呢？原因是这里需要对 SQL 语句进行解析并检查其正确性，这个复杂的过程是 macro_rules 难以对付的，而过程宏相比起来就会灵活的多。
```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}

// 使用形式则类似于函数调用:
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

[补充学习资料](#补充学习资料)

1.  [dtolnay/proc-macro-workshop](https://github.com/dtolnay/proc-macro-workshop)，学习如何编写过程宏
2.  [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)，学习如何编写声明宏 `macro_rules!`
3.  [syn](https://crates.io/crates/syn) 和 [quote](https://crates.io/crates/quote) ，用于编写过程宏的包，它们的文档有很多值得学习的东西
4.  [Structuring, testing and debugging procedural macro crates](https://www.reddit.com/r/rust/comments/rjumsg/any_good_resources_for_learning_rust_macros/)，从测试、debug、结构化的角度来编写过程宏
5.  [blog.turbo.fish](https://blog.turbo.fish)，里面的过程宏系列文章值得一读
6.  [Rust 宏小册中文版](https://zjp-cn.github.io/tlborm/)，非常详细的解释了宏各种知识


# async
选型：
- 有大量 IO 任务需要并发运行时，选 async 模型
- 有部分 IO 任务需要并发运行时，选多线程，如果想要降低线程创建和销毁的开销，可以使用线程池
- 有大量 CPU 密集任务需要并行运行时，例如并行计算，选多线程模型，且让线程数等于或者稍大于 CPU 核心数
- 无所谓时，统一选多线程

若大家使用 tokio，那 CPU 密集的任务尤其需要用线程的方式去处理，例如使用 `spawn_blocking` 创建一个阻塞的线程取完成相应 CPU 密集任务。
- 使用 spawn_blocking 后，会创建一个单独的 OS 线程，该线程并不会被 tokio 所调度( 被 OS 所调度 )，因此它所执行的 CPU 密集任务也不会导致 tokio 调度的那些异步任务被饿死

## async模式下的问题
- Rust 不允许你在特征中声明 async 函数(可以通过三方
- 在同步( synchronous )代码中使用的一些语言特性在 async 中可能将无法再使用

## 使用 .await
与`block_on`不同，`.await`并不会阻塞当前的线程，而是异步的等待Future A的完成，在等待的过程中，该线程还可以继续执行其它的Future B，最终实现了并发处理的效果
```rust
use futures::executor::block_on;

async fn hello_world() {
    // async中调用async 方法
    hello_cat().await;
    println!("hello, world!");
}

async fn hello_cat() {
    println!("hello, kitty!");
}
fn main() {
    let future = hello_world();
    // block_on 同步阻塞
    block_on(future);
}
```
- `join!`可以并发的处理和等待多个`Future`，若`f1 Future`被阻塞，那`f2 Future`可以拿过线程的所有权继续执行。若`f2`也变成阻塞状态，那`f1`又可以再次拿回线程所有权，继续执行。
- 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
```rust
futures::join!(f1, f2);
```


## Pin
- Pin 是一个结构体
    - 可以被 Pin 住的值实现的特征是 `!Unpin`
- Unpin 是一个 Trait
    - 如果实现了 Unpin 特征，就不能被 Pin 了？其实，还是可以 Pin 的，毕竟它只是一个结构体，你可以随意使用，但是**不再有任何效果而已**，该值一样可以被移动！(无效果)
    - 一个类型如果不能被移动，它必须实现 !Unpin 特征
- Unpin 与之前章节学过的 Send/Sync 进行下对比
    - 都是标记特征(marker trait)，该特征未定义任何行为，非常适用于标记
    - 都可以通过`!`语法去除实现
    - 绝大多数情况都是自动实现, 无需我们的操心

Pin可以加个内存固定在栈上和堆上
```rust
// 固定到栈上
#![allow(unused)]
fn main() {
use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}


impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}
}
```

```rust
// 固定到堆上
use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

        boxed
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        unsafe { &*(self.b) }
    }
}

pub fn main() {
    let test1 = Test::new("test1");
    let test2 = Test::new("test2");

    println!("a: {}, b: {}",test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}",test2.as_ref().a(), test2.as_ref().b());
}

```
## 将固定住的 Future 变为 Unpin
- async 函数返回的 Future 默认就是 !Unpin 的，但是，在实际应用中，一些函数会要求它们处理的 Future 是 Unpin 的
- 若你使用的 Future 是 !Unpin 的，必须要使用以下的方法先将 Future 进行固定:
    - `Box::pin`， 创建一个 `Pin<Box<T>>`
    - `pin_utils::pin_mut!`， 创建一个 `Pin<&mut T>`
    - 固定后获得的 `Pin<Box<T>>` 和 `Pin<&mut T>` 既可以用于 Future ，又会自动实现 Unpin。
```rust
fn main() {
    use pin_utils::pin_mut; // `pin_utils` 可以在crates.io中找到

    // 函数的参数是一个`Future`，但是要求该`Future`实现`Unpin`
    fn execute_unpin_future(x: impl Future<Output = ()> + Unpin) { /* ... */ }

    let fut = async { /* ... */ };
    // 下面代码报错: 默认情况下，`fut` 实现的是`!Unpin`，并没有实现`Unpin`
    // execute_unpin_future(fut);

    // 使用`Box`进行固定
    let fut = async { /* ... */ };
    let fut = Box::pin(fut);
    execute_unpin_future(fut); // OK

    // 使用`pin_mut!`进行固定
    let fut = async { /* ... */ };
    pin_mut!(fut);
    execute_unpin_future(fut); // OK
}
```


总结:
- 若 `T: Unpin` ( Rust 类型的默认实现)，那么 `Pin<'a, T>` 跟 `&'a mut T` 完全相同，也就是 Pin 将没有任何效果, 该移动还是照常移动
- 绝大多数标准库类型都实现了 Unpin ，事实上，对于 Rust 中你能遇到的绝大多数类型，该结论依然成立 ，其中一个例外就是：async/await 生成的 Future 没有实现 Unpin
- 你可以通过以下方法为自己的类型添加 !Unpin 约束：
- 使用文中提到的 `std::marker::PhantomPinned`
- 使用nightly 版本下的 feature flag
- 可以将值固定到栈上，也可以固定到堆上
- 将 !Unpin 值固定到栈上需要使用 unsafe
- 将 !Unpin 值固定到堆上无需 unsafe ，可以通过 `Box::pin` 来简单的实现
- 当固定类型T: !Unpin时，你需要保证数据从被固定到被 drop 这段时期内，其内存不会变得非法或者被重用
- Pin 主要用来🪄固定指针指向的值🪄不被移动的，例如 Pin<&mut T>，Pin<&T>， Pin<Box> 都保证 T 不会移动，在 Rust 中，能够被移动内存代表着拥有其所有权或可变引用，而 Pin 可以处理这个问题。所以，Pin 应用场景一般用于结构中出现了自引用时使用，Pin 的诞生就是因为 async/.await 内部编译器产生代码出现了自引用的问题。
    - `Box::pin` , `Pin::as_mut`, `Pin::new_unchecke` 和 `Pin::get_unchecked_mut` 等关联函数        


## async 生命周期
- 当 x 依然有效时， 该 Future 就必须继续等待( .await ), 也就是说x 必须比 Future活得更久。
```rust
async fn foo(x: &u8) -> u8 { *x }

// 上面的函数跟下面的函数是等价的:
fn foo_expanded<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move { *x }
}

// Future会在未来执行，所以x 必须活的够久
fn bad() -> impl Future<Output = u8> {
    let x = 5;
    borrow_x(&x) // ERROR: `x` does not live long enough
}
fn good() -> impl Future<Output = u8> {
    // 移动到 async 语句块内， 我们将它的生命周期扩展到 'static
    async {
        let x = 5;
        borrow_x(&x).await
    }
}
```
- async move: async 允许我们使用 move 关键字来将环境中变量的所有权转移到语句块内，就像闭包那样
    - 好处是你不再发愁该如何解决借用生命周期的问题
    - 坏处就是无法跟其它代码实现对变量的共享

```rust
// 多个不同的 `async` 语句块可以访问同一个本地变量，只要它们在该变量的作用域内执行
async fn blocks() {
    let my_string = "foo".to_string();

    let future_one = async {
        // ...
        println!("{my_string}");
    };
    let future_two = async {
        // ...
        println!("{my_string}");
    };

    // 运行两个 Future 直到完成
    let ((), ()) = futures::join!(future_one, future_two);
}

// 由于`async move`会捕获环境中的变量，因此只有一个`async move`语句块可以访问该变量，
// 但是它也有非常明显的好处： 变量可以转移到返回的 Future 中，不再受借用生命周期的限制
fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        // ...
        println!("{my_string}");
    }
}
```
- 当.await 遇见多线程执行器:
    - 当使用多线程 Future 执行器( executor )时， Future 可能会在线程间被移动，因此 async 语句块中的变量必须要能在线程间传递。 
        - 至于 Future 会在线程间移动的原因是：它内部的任何.await都可能导致它被切换到一个新线程上去执行。
    - 由于需要在**多线程**环境使用，意味着 Rc、 RefCell 、没有实现 Send 的所有权类型、没有实现 Sync 的引用类型，它们都是不安全的，因此无法被使用
    - 在 .await 时使用普通的锁也不安全，例如 Mutex; 我们需要使用 futures 包下的锁 `futures::lock` 来替代 Mutex 完成任务。

## Stream 流处理
- Stream 特征类似于 Future 特征，但是前者在完成前可以生成多个值，这种行为跟标准库中的 Iterator 特征倒是颇为相似。
- 关于 Stream 的一个常见例子是消息通道（futures 包中的）的消费者 `Receiver`。每次有消息从 Send 端发送后，它都可以接收到一个 Some(val) 值， 一旦 Send 端关闭(drop)，且消息通道中没有消息后，它会接收到一个 None 值。

```rust
trait Stream {
    // Stream生成的值的类型
    type Item;

    // 尝试去解析Stream中的下一个值,
    // 若无数据，返回`Poll::Pending`, 若有数据，返回 `Poll::Ready(Some(x))`, `Stream`完成则返回 `Poll::Ready(None)`
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Option<Self::Item>>;
}

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    // `StreamExt::next` 类似于 `Iterator::next`, 但是前者返回的不是值，而是一个 `Future<Output = Option<T>>`，
    // 因此还需要使用`.await`来获取具体的值
    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}
```
- 迭代和并发: 跟迭代器类似，我们也可以迭代一个 Stream
    - 例如使用map，filter，fold方法，以及它们的遇到错误提前返回的版本： try_map，try_filter，try_fold。
    - for 循环无法在这里使用，但是命令式风格的循环`while let`是可以用的，同时还可以使用next 和 try_next 方法:

```rust
async fn sum_with_next(mut stream: Pin<&mut dyn Stream<Item = i32>>) -> i32 {
    use futures::stream::StreamExt; // 引入 next
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum
}

async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, io::Error> {
    use futures::stream::TryStreamExt; // 引入 try_next
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    Ok(sum)
}
```
- 上面代码是一次处理一个值的模式; 如果你选择一次处理一个值的模式，可能会造成无法并发，这就失去了异步编程的意义
- 因此，如果可以的话我们还是要选择从一个 Stream 并发处理多个值的方式，通过 `for_each_concurrent` 或 `try_for_each_concurrent` 方法来实现:
```rust
async fn jump_around(
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt; // 引入 `try_for_each_concurrent`
    const MAX_CONCURRENT_JUMPERS: usize = 100;

    stream.try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
        jump_n_times(num).await?;
        report_n_jumps(num).await?;
        Ok(())
    }).await?;

    Ok(())
}
```
## 使用join!和select!同时运行多个 Future
- join!宏， 它允许我们同时等待多个不同 Future 的完成，且可以并发地运行这些 Future
    - 如果希望同时运行一个数组里的多个异步任务，可以使用 `futures::future::join_all` 方法 (传参是数组)
```rust
use futures::join;

async fn enjoy_book_and_music() -> (Book, Music) {
    let book_fut = enjoy_book();
    let music_fut = enjoy_music();
    // join!会返回一个元组，里面的值是对应的Future执行结束后输出的值。
    join!(book_fut, music_fut)
}
```

- `try_join!`: 当某一个 Future 报错后就立即停止所有 Future 的执行; 特别是当 Future 返回 Result 时
```rust
use futures::try_join;

async fn get_book() -> Result<Book, String> { /* ... */ Ok(Book) }
async fn get_music() -> Result<Music, String> { /* ... */ Ok(Music) }

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book();
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}

//! 传给 try_join! 的所有 Future 都必须拥有相同的错误类型。如果错误类型不同，可以考虑使用来自 futures::future::TryFutureExt 模块的 map_err和err_info方法将错误进行转换:
use futures::{
    future::TryFutureExt,
    try_join,
};

async fn get_book() -> Result<Book, ()> { /* ... */ Ok(Book) }
async fn get_music() -> Result<Music, String> { /* ... */ Ok(Music) }

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book().map_err(|()| "Unable to get book".to_string());
    let music_fut = get_music();
    try_join!(book_fut, music_fut)
}
```

- `futures::select!`: 如果你想同时等待多个 Future ，且任何一个 Future 结束后，都可以立即被处理，可以考虑使用; (只会执行先到的任务，未完成的任务就不会执行了，类似 go 的 select)
    - join! 只有等所有 Future 结束后，才能集中处理结果，
```rust
use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};

async fn task_one() { /* ... */ }
async fn task_two() { /* ... */ }

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();

    pin_mut!(t1, t2);

    // 无论两者哪个先完成，都会调用对应的 println! 打印相应的输出，
    // 然后函数结束且不会等待另一个任务的完成。
    select! {
        () = t1 => println!("任务1率先完成"),
        () = t2 => println!("任务2率先完成"),
    }
}
```
- select!还支持 default 和 complete 分支:
    - complete 分支当所有的 Future 和 Stream 完成后才会被执行，它往往配合loop使用，loop用于循环完成所有的 Future
    - default分支，若没有任何 Future 或 Stream 处于 Ready 状态， 则该分支会被立即执行
```rust
pub fn main() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break,
            default => panic!(), // 该分支永远不会运行，因为`Future`会先运行，然后是`complete`
        };
    }
    assert_eq!(total, 10);
}
```
### 跟 Unpin 和 FusedFuture 进行交互
.fuse()方法可以让 Future 实现 `FusedFuture` 特征， 而 pin_mut! 宏会为 Future 实现 `Unpin`特征，这两个特征恰恰是使用 select 所必须的:
- Unpin，由于 select 不会通过拿走所有权的方式使用Future，而是通过可变引用的方式去使用，这样当 select 结束后，该 Future 若没有被完成，它的所有权还可以继续被其它代码使用。
- FusedFuture的原因跟上面类似，当 Future 一旦完成后，那 select 就不能再对其进行轮询使用。Fuse意味着熔断，相当于 Future 一旦完成，再次调用poll会直接返回Poll::Pending。

> 只有实现了FusedFuture，select 才能配合 loop 一起使用。假如没有实现，就算一个 Future 已经完成了，它依然会被 select 不停的轮询执行。


Stream 稍有不同，它们使用的特征是 `FusedStream`。 通过`.fuse()`(也可以手动实现)实现了该特征的 Stream，对其调用`.next()` 或 `.try_next()`方法可以获取实现了FusedFuture特征的Future:
```rust
use futures::{
    stream::{Stream, StreamExt, FusedStream},
    select,
};

async fn add_two_streams(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    total
}
```

准确的说来自`变量名/路径`的future要求其实现Unpin+FusedFuture，对于来自表达式的future可以放宽Unpin的限制。
```rust
// Ok! Unpin+FusedFuture
let t1 = task_one().fuse();
let t2 = task_two().fuse();
pin_mut!(t1, t2);
select! {
    () = t1 => println!("任务1率先完成"),
    () = t2 => println!("任务2率先完成"),
}

// Ok! only FusedFuture
select! {
    () = task_one().fuse() => println!("任务1率先完成"),
    () = task_two().fuse() => println!("任务2率先完成"),
}
```
## 在 select 循环中并发
一个很实用但又鲜为人知的函数是 `Fuse::terminated()` ，可以使用它构建一个空的 Future ，可以先创建一个空的，后面再赋值

考虑以下场景：当你要在select循环中运行一个任务，但是该任务却是在select循环内部创建时，上面的函数就非常好用了。
```rust
use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    stream::{FusedStream, Stream, StreamExt},
    pin_mut,
    select,
};

async fn get_new_num() -> u8 { /* ... */ 5 }

async fn run_on_new_num(_: u8) { /* ... */ }

async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(run_on_new_num_fut, get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // 定时器已结束，若`get_new_num_fut`没有在运行，就创建一个新的
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                // 收到新的数字 -- 创建一个新的`run_on_new_num_fut`并丢弃掉旧的
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            // 运行 `run_on_new_num_fut`
            () = run_on_new_num_fut => {},
            // 若所有任务都完成，直接 `panic`， 原因是 `interval_timer` 应该连续不断的产生值，而不是结束
            //后，执行到 `complete` 分支
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}
```

当某个 Future 有多个拷贝都需要同时运行时，可以使用 `FuturesUnordered` 类型。下面的例子跟上个例子大体相似，但是它会将 run_on_new_num_fut 的每一个拷贝都运行到完成，而不是像之前那样一旦创建新的就终止旧的。
```rust
use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    stream::{FusedStream, FuturesUnordered, Stream, StreamExt},
    pin_mut,
    select,
};

async fn get_new_num() -> u8 { /* ... */ 5 }

async fn run_on_new_num(_: u8) -> u8 { /* ... */ 5 }


// 使用从 `get_new_num` 获取的最新数字 来运行 `run_on_new_num`
//
// 每当计时器结束后，`get_new_num` 就会运行一次，它会立即取消当前正在运行的`run_on_new_num` ,
// 并且使用新返回的值来替换
async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let mut run_on_new_num_futs = FuturesUnordered::new();
    run_on_new_num_futs.push(run_on_new_num(starting_num));
    let get_new_num_fut = Fuse::terminated();
    pin_mut!(get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                 // 定时器已结束，若`get_new_num_fut`没有在运行，就创建一个新的
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                 // 收到新的数字 -- 创建一个新的`run_on_new_num_fut` (并没有像之前的例子那样丢弃掉旧值)
                run_on_new_num_futs.push(run_on_new_num(new_num));
            },
            // 运行 `run_on_new_num_futs`, 并检查是否有已经完成的
            res = run_on_new_num_futs.select_next_some() => {
                println!("run_on_new_num_fut returned {:?}", res);
            },
            // 若所有任务都完成，直接 `panic`， 原因是 `interval_timer` 应该连续不断的产生值，而不是结束
            //后，执行到 `complete` 分支
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}
```
















