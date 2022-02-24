//  1.Rust 是 静态类型（statically typed）语言，也就是说在编译时就必须知道所有变量的类型
// 一般可以自行推断，无法推断时需要显式标注
fn dataType() {
  // 当多种类型均有可能时，比如使用 parse 将 String 转换为数字时，必须增加类型注解
  let guess: u32 = "42".parse().expect("Not a number!");
}

// 2.标量类型scalar

// 整型
// 长度	有符号	无符号
// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128	u128
// arch	isize	usize
// isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。
// Rust 数字类型默认是 i32。isize 或 usize 主要作为某些集合的索引。

// 数字字面值	例子
// Decimal (十进制)	98_222
// Hex (十六进制)	0xff
// Octal (八进制)	0o77
// Binary (二进制)	0b1111_0000
// Byte (单字节字符)(仅限于u8)	b'A'

// 整数溢出：debug 下会 panic，release 会循环赋值

// 浮点型
// Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。
// 默认类型是 f64，因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高。所有的浮点型都是有符号的。
// 浮点数采用 IEEE-754 标准表示。f32 是单精度浮点数，f64 是双精度浮点数。
fn float() {
  let x = 2.0; // f64
  let y: f32 = 3.0; // f32
}

// 数值操作
// 加减乘除 取余：+ - * / %

// 布尔类型 true false

// 字符类型
// Rust的 char 类型是语言中最原生的字母类型
// 注意，我们用单引号声明 char 字面量，而与之相反的是，使用双引号声明字符串字面量
// Rust 的 char 类型的大小为四个字节(four bytes)，并代表了一个 Unicode 标量值（Unicode Scalar Value），
//    这意味着它可以比 ASCII 表示更多内容。在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。
//    Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值。不过，“字符” 并不是一个 Unicode 中的概念，所以人直觉上的 “字符” 可能与 Rust 中的 char 并不符合
fn char() {
  let c = 'z';
  let z = 'ℤ';
  let heart_eyed_cat = '😻';
}

// 3. 复合类型（Compound types）
// Rust 有两个原生的复合类型：元组（tuple）和数组（array）

// 元组类型
// 元组长度固定：一旦声明，其长度不会增大或缩小
fn tuple() {
  // 元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  // 为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值
  let (x, y, z) = tup;

  // 使用点号（.）后跟值的索引来直接访问它们
  let five_hundred = x.0;
  let six_point_four = x.1;
  let one = x.2;
}

// 数组类型
// 数组中的每个元素的类型必须相同
// Rust中的数组长度是固定的
// 当你想要在栈（stack）而不是在堆（heap）上为数据分配空间，或者是想要确保总是有固定数量的元素时，数组非常有用
// （可变是 Vector）: vector 类型是标准库提供的一个 允许 增长和缩小长度的类似数组的集合类型
fn array() {
  let a = [1, 2, 3, 4, 5];
  // 数组的类型：在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  // 初始值加分号再加元素个数
  // 包含 5 个元素，这些元素的值最初都将被设置为 3
  let a = [3; 5];
  // 访问
  let first = a[0];
  let second = a[1];
  // error:数组越界会 panic
  // 明确越界的 编译期会报错；运行时确定的，编译期不报错
  let err = a[9]
}

