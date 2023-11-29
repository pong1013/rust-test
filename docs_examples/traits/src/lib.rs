pub mod summary;

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(閱讀更多...)")
    }
}
