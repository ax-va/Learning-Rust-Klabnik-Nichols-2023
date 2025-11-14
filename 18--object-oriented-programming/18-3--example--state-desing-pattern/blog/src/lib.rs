pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // deref coercion after calling `unwrap`
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // Call the `take` method to take the `Some` value out of the state field and leave a `None` in its place.
        // This leads to moving the `state` value out of `Post`.
        if let Some(s) = self.state.take() {
            // Call an internal `request_review` method on the current state
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// This trait defines the behavior shared by different post states
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    // default implementation
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(InReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct InReview {}

impl State for InReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Override the default implementation.
    // The lifetime of the returned reference is related to the lifetime of the `post` argument.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}