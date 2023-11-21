pub use crate::back_of_house::breakfast as bs;
pub use crate::front_of_house::hosting;

pub mod back_of_house;
mod front_of_house;

pub fn eat_at_restaurant() {
    ////// BACK OF HOUSE //////
    // 點夏季早餐並選擇黑麥麵包
    let mut meal = bs::Breakfast::summer("黑麥");
    // 我們想改成全麥麵包
    meal.toast = String::from("全麥");
    println!("我想要{}麵包，謝謝", meal.toast);
    let order1 = bs::Appetizer::Soup;
    // 接下來這行取消註解的話，我們就無法編譯通過
    // 我們無法擅自更改餐點搭配的季節水果
    // meal.seasonal_fruit = String::from("藍莓");
    println!("Order1 do {:?}.", order1);
    ////// FRONT OF HOUSE //////
    hosting::add_to_waitlist();
}
