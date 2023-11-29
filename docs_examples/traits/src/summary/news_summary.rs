pub use crate::Summary;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // default impl
}

pub fn notify(item: &impl Summary) {
    println!("頭條新聞！{}", item.summarize());
}
