
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
        - 由于 Rc<T> 需要管理引用计数，但是该计数器并没有使用任何并发原语，因此无法实现原子化的计数操作，最终会导致计数错误。
    - Rc<T> 是一个智能指针，实现了 Deref 特征，因此你无需先解开 Rc 指针，再使用里面的 T，而是可以直接使用 T，例如上例中的 gadget1.owner.name

- Arc 是 Atomic Rc 的缩写，顾名思义：原子化的 Rc<T> 智能指针。原子化是一种并发原语。

## Cell 和 RefCell
可以在拥有不可变引用的同时修改目标数据(因为 Rust 使用了 unsafe 来做到这一点)。

- Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 只适用于 T 实现 Copy 的情况，实际开发中 Cell 使用不多
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
    // 等待 子线程通知
    started = cvar.wait(started).unwrap();
}

println!("started changed");
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











