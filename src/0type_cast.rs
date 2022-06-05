///* 类型转换
///
/// * as转换
fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < (b as i32) {
        println!("Ten is less than one hundred.");
    }
}

// 内存地址转换为指针
fn main() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // 将p1内存地址转换为一个整数
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()，i32类型占用4个字节，因此将内存地址 + 4
    let p2 = second_address as *mut i32; // 访问该地址指向的下一个整数p2
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);
}

//# 转换不具有传递性 就算 e as U1 as U2 是合法的，也不能说明 e as U2 是合法的（e 不能直接转换成 U2）

/// * TryInto 转换
use std::convert::TryInto;

fn main() {
    let a: u8 = 10;
    let b: u16 = 1500;
    // try_into 会尝试进行一次转换，并返回一个 Result，此时就可以对其进行相应的错误处理
    let b_: u8 = b.try_into().unwrap();

    // 正常的错误处理
    let bb_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

///* 通用类型转换
// 强制类型转换
// 首先，在匹配特征时，不会做任何强制转换(除了方法)。一个类型 T 可以强制转换为 U，不代表 impl T 可以强制转换为 impl U，例如下面的代码就无法通过编译检查：
trait Trait {}

fn foo<X: Trait>(t: X) {}

impl<'a> Trait for &'a i32 {}

fn main() {
    //* error: &i32 实现了特征 Trait， &mut i32 可以转换为 &i32，但是 &mut i32 依然无法作为 Trait 来使用。 */
    let t: &mut i32 = &mut 0;
    foo(t);
}
// 点操作符
// * Rust 自动解析 方法调用方
/*
假设有一个方法 foo，它有一个接收器(接收器就是 self、&self、&mut self 参数)。如果调用 value.foo()，编译器在调用 foo 之前，需要决定到底使用哪个 Self 类型来调用。现在假设 value 拥有类型 T。

再进一步，我们使用「完全限定语法」(https://course.rs/basic/trait/advance-trait.html#%E5%AE%8C%E5%85%A8%E9%99%90%E5%AE%9A%E8%AF%AD%E6%B3%95)来进行准确的函数调用:

- 首先，编译器检查它是否可以直接调用 T::foo(value)，称之为值方法调用
- 如果上一步调用无法完成(例如方法类型错误或者特征没有针对 Self 进行实现，上文提到过特征不能进行强制转换)，那么编译器会尝试增加自动引用，例如会尝试以下调用： <&T>::foo(value) 和 <&mut T>::foo(value)，称之为引用方法调用
- 若上面两个方法依然不工作，编译器会试着解引用 T ，然后再进行尝试。这里使用了 Deref 特征 —— 若 T: Deref<Target = U> (T 可以被解引用为 U)，那么编译器会使用 U 类型进行尝试，称之为解引用方法调用
- 若 T 不能被解引用，且 T 是一个定长类型(在编译器类型长度是已知的)，那么编译器也会尝试将 T 从定长类型转为不定长类型，例如将 [i32; 2] 转为 [i32]
- 若还是不行，那...没有那了，最后编译器大喊一声：汝欺我甚，不干了！
*/

fn main() {
    let array: Rc<Box<[T; 3]>> = [1, 2, 3].into();
    let first_entry = array[0];
}
// array 数组的底层数据隐藏在了重重封锁之后，那么编译器如何使用 array[0] 这种数组原生访问语法通过重重封锁，准确的访问到数组中的第一个元素？
/*
- 首先， array[0] 只是Index特征的语法糖：编译器会将 array[0] 转换为 array.index(0) 调用，当然在调用之前，编译器会先检查 array 是否实现了 Index 特征。
- 接着，编译器检查 Rc<Box<[T; 3]>> 是否有否实现 Index 特征，结果是否，不仅如此，&Rc<Box<[T; 3]>> 与 &mut Rc<Box<[T; 3]>> 也没有实现。
- 上面的都不能工作，编译器开始对 Rc<Box<[T; 3]>> 进行解引用，把它转变成 Box<[T; 3]>
- 此时继续对 Box<[T; 3]> 进行上面的操作 ：Box<[T; 3]>， &Box<[T; 3]>，和 &mut Box<[T; 3]> 都没有实现 Index 特征，所以编译器开始对 Box<[T; 3]> 进行解引用，然后我们得到了 [T; 3]
- [T; 3] 以及它的各种引用都没有实现 Index 索引(是不是很反直觉:D，在直觉中，数组都可以通过索引访问，实际上只有数组切片才可以!)，它也不能再进行解引用，因此编译器只能祭出最后的大杀器：将定长转为不定长，因此 [T; 3] 被转换成 [T]，也就是数组切片，它实现了 Index 特征，因此最终我们可以通过 index 方法访问到对应的元素。
*/

// * clone()的返回类型？自动引用生效
// 方法签名: &T ： fn clone(&T) -> T
fn do_stuff<T: Clone>(value: &T) {
    // cloned: T
    let cloned = value.clone();
}

fn do_stuff2<T>(value: &T) {
    // cloned: &T
    // * 所有的引用类型都可以被复制，因为其实就是复制一份地址
    let cloned = value.clone();
}
/*
首先，从直觉上来说，该方法会报错，因为 T 没有实现 Clone 特征，但是真实情况是什么呢？

我们先来推导一番。 首先通过值方法调用就不再可行，因为 T 没有实现 Clone 特征，也就无法调用 T 的 clone 方法。接着编译器尝试引用方法调用，此时 T 变成 &T，在这种情况下， clone 方法的签名如下： fn clone(&&T) -> &T，接着我们现在对 value 进行了引用。 编译器发现 &T 实现了 Clone 类型(所有的引用类型都可以被复制，因为其实就是复制一份地址)，因此可以推出 cloned 也是 &T 类型。

最终，我们复制出一份引用指针，这很合理，因为值类型 T 没有实现 Clone，只能去复制一个指针了。
*/

// * 自动引用生效 - 复杂案例

fn main() {
    #[derive(Clone)]
    struct Container<T>(Arc<T>);

    fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
        // foo_cloned: Container<i32>
        let foo_cloned = foo.clone();
        // bar_cloned: &Container<T>
        let bar_cloned = bar.clone();
    }
}
// 关键在 Container 的泛型参数，一个是 i32 的具体类型，一个是泛型类型，其中 i32 实现了 Clone，但是 T 并没有。
/*
首先要复习一下复杂类型派生 Clone 的规则：一个复杂类型能否派生 Clone，需要它内部的所有子类型都能进行 Clone。因此 Container<T>(Arc<T>) 是否实现 Clone 的关键在于 T 类型是否实现了 Clone 特征。

上面代码中，Container<i32> 实现了 Clone 特征，因此编译器可以直接进行值方法调用，此时相当于直接调用 foo.clone，其中 clone 的函数签名是 fn clone(&T) -> T，由此可以看出 foo_cloned 的类型是 Container<i32>。
*/

// * 然而，bar_cloned 的类型却是 &Container<T>，这个不合理啊，明明我们为 Container<T> 派生了 Clone 特征，因此它也应该是 Container<T> 类型才对。万事皆有因，我们先来看下 derive 宏最终生成的代码大概是啥样的：
fn main() {
    impl<T> Clone for Container<T>
    where
        T: Clone,
    {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
}
// 从上面代码可以看出，派生 Clone 能实现的根本是 T 实现了Clone特征(https://doc.rust-lang.org/std/clone/trait.Clone.html#derivable)：where T: Clone， 因此 Container<T> 就没有实现 Clone 特征。
// * 编译器接着会去尝试引用方法调用，此时 &Container<T> 引用实现了 Clone，最终可以得出 bar_cloned 的类型是 &Container<T>。

// 也可以为 Container<T> 手动实现 Clone 特征：
// 此时，编译器首次尝试值方法调用即可通过，因此 bar_cloned 的类型变成 Container<T>。
impl<T> Clone for Container<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

/// * 变形记(Transmutes)
// * - mem::transmute<T, U> 将类型 T 直接转成类型 U，唯一的要求就是，这两个类型占用同样大小的字节数！我的天，这也算限制？这简直就是无底线的转换好吧？看看会导致什么问题：
/*
1. 首先也是最重要的，转换后创建一个任意类型的实例会造成无法想象的混乱，而且根本无法预测。不要把 3 转换成 bool 类型，就算你根本不会去使用该 bool 类型，也不要去这样转换
2. 变形后会有一个重载的返回类型，即使你没有指定返回类型，为了满足类型推导的需求，依然会产生千奇百怪的类型
3. 将 & 变形为 &mut 是未定义的行为
    - 种转换永远都是未定义的
    - 不，你不能这么做
    - 不要多想，你没有那种幸运
4. 变形为一个未指定生命周期的引用会导致无界生命周期
5. 在复合类型之间互相变换时，你需要保证它们的排列布局是一模一样的！一旦不一样，那么字段就会得到不可预期的值，这也是未定义的行为，至于你会不会因此愤怒， WHO CARES ，你都用了变形了，老兄！

对于第 5 条，你该如何知道内存的排列布局是一样的呢？对于 repr(C) 类型和 repr(transparent) 类型来说，它们的布局是有着精确定义的。但是对于你自己的"普通却自信"的 Rust 类型 repr(Rust) 来说，它可不是有着精确定义的。甚至同一个泛型类型的不同实例都可以有不同的内存布局。 Vec<i32> 和 Vec<u32> 它们的字段可能有着相同的顺序，也可能没有。对于数据排列布局来说，什么能保证，什么不能保证目前还在 Rust 开发组的工作任务中呢。
*/
// * - mem::transmute_copy<T, U> 才是真正的深渊，它比之前的还要更加危险和不安全。它从 T 类型中拷贝出 U 类型所需的字节数，然后转换成 U。 mem::transmute 尚有大小检查，能保证两个数据的内存大小一致，现在这哥们干脆连这个也丢了，只不过 U 的尺寸若是比 T 大，会是一个未定义行为。

// 案例
// 将裸指针变成函数指针：
fn main() {
    fn foo() -> i32 {
        0
    }

    let pointer = foo as *const ();
    let function = unsafe {
        // 将裸指针转换为函数指针
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    assert_eq!(function(), 0);
}

// 延长生命周期，或者缩短一个静态生命周期寿命：
fn main() {
    struct R<'a>(&'a i32);

    // 将 'b 生命周期延长至 'static 生命周期
    unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
        std::mem::transmute::<R<'b>, R<'static>>(r)
    }

    // 将 'static 生命周期缩短至 'c 生命周期
    unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
        std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
    }
}
