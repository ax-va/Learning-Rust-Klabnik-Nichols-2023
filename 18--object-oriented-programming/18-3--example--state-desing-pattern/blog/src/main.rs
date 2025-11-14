use blog::Post;

fn main() {
    let mut post = Post::new(); // state "draft"

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review(); // state "in review"
    assert_eq!("", post.content());

    post.approve(); // state "published"
    assert_eq!("I ate a salad for lunch today", post.content());
}
