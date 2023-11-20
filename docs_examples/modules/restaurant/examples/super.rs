fn deliver_order() {}

fn main() {
    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::deliver_order(); //super可以前往back_of_house的上層模組，此範例是 crate 的 deliver_order()
        }

        fn cook_order() {}
    }
}
