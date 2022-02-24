// 1.变量和可变性
fn variable() {
  // 不可修改
  let y = 6;
  // error
  y = 8;
  // 可二次修改
  let mut x = 5;
  println!("The value of x is: {}", x);
  //  ok
  x = 6;
  println!("The value of x is: {}", x);
}

// 2. 常量:
// 大写，下划线分隔，必须指明类型
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// 3. 隐藏（Shadowing）
fn shadowing() {
  let x = 5;
  // 变量可以重复定义，覆盖之前的（隐藏之前的变量）
  // 用法：改变值的类型，但是依然用这个名字
  let x = x + 1;
}
