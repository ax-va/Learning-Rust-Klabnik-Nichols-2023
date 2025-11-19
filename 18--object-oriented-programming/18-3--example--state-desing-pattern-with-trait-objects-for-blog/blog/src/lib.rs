pub struct Post {
    // `Box<dyn State>`:
    // - `dyn State` means a *trait object*:
    // a value whose concrete type is unknown at compile time
    // but which implements the `State` trait.
    // - `Box<dyn State>` means allocating that trait object on the heap and storing a pointer to it.
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
        // 1. `self.state.as_ref()` gives `Option<&Box<dyn State>>`
        // 2. `unwrap()` gives `&Box<dyn State>`.
        // 3. Deref coercion turns `&Box<dyn State>` into `&dyn State`.
        // 4. Then it calls the state-specific `content` method.
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

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

// This trait defines the behavior shared by different post states
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn reject(self: Box<Self>) -> Box<dyn State>;

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

    fn reject(self: Box<Self>) -> Box<dyn State> {
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

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
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

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(InReview {})
    }

    // Override the default implementation.
    // The lifetime of the returned reference is related to the lifetime of the `post` argument.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}