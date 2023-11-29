mod post;

use crate::post::news::news;
use crate::post::tweet::tweet_post;
fn main() {
    tweet_post();
    news();
}
