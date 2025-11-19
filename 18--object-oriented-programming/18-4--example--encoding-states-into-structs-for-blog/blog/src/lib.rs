/*
This implementation doesn't quite follow the object-oriented state pattern.
The benefit is that the type system prevents invalid states,
catching bugs like showing content from an unpublished post at compile time
before they ever reach production.

Object-oriented patterns won't always be the best solution in Rust
due to certain features, like ownership, that object-oriented languages don't have.
 */

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        // Return an instance of `DraftPost`, not a `Post`
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // ---
    // `DraftPost` does not have a content method defined.
    // This ensures that draft posts don't have any content available for display.
    // ---

    pub fn request_review(self) -> InReviewPost {
        // Take ownership of `self`,
        // consuming the `DraftPost` instance and
        // transforming it into an `InReviewPost` instance.
        InReviewPost {
            content: self.content,
        }
    }
}

pub struct InReviewPost {
    content: String,
}

impl InReviewPost {
    // ---
    // `InReviewPost` does not have a content method defined.
    // This ensures that draft posts don't have any content available for display.
    // ---

    pub fn approve(self) -> Post {
        // Take ownership of `self`,
        // consuming the `InReviewPost` instance and
        // transforming it into a `Post` instance.
        Post {
            content: self.content,
        }
    }
}
