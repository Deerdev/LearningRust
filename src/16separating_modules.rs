// ---------------------------------------------------------
// 文件名: src/lib.rs

/// 将 mod 的定义拆分到文件中，这里只定义名称
mod front_of_house;

// 会按照 src/front_of_house/hosting.rs 查找
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
// ---------------------------------------------------------

// ---------------------------------------------------------
// 文件名: src/front_of_house.rs
pub mod hosting;
// ---------------------------------------------------------

// ---------------------------------------------------------
// 文件名: src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
// ---------------------------------------------------------
