mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        #[allow(dead_code)]
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("桃子"),
            }
        }
    }
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//透過 use 引入路徑，在此 crate 作用域中建立捷徑，之後要使用module就不需要打前面一大串
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    ////// BACK OF HOUSE //////
    // 點夏季早餐並選擇黑麥麵包
    let mut meal = back_of_house::Breakfast::summer("黑麥");
    // 我們想改成全麥麵包
    meal.toast = String::from("全麥");
    println!("我想要{}麵包，謝謝", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    // 接下來這行取消註解的話，我們就無法編譯通過
    // 我們無法擅自更改餐點搭配的季節水果
    // meal.seasonal_fruit = String::from("藍莓");
    println!("Order1 do {:?}.", order1);
    println!("Order2 do {:?}.", order2);

    ////// FRONT OF HOUSE //////
    hosting::add_to_waitlist();
}
