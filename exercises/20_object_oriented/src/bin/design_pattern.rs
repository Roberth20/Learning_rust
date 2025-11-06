/*Here we will develop a blog posting workflow with the state pattern for OOP. Then, we
will make some changes with rust funcionalities. The functionalities are

1. A blog post starts as an empty draft.
2. When the draft is done, a review of the post is requested.
3. When the post is approved, it gets published.
4. Only published blog posts return content to print, so unapproved posts can’t 
   accidentally be published.
*/
// Let's start with the Post object
struct Post {
    // The state is an Option because the description in request_review method
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // Create a new empty post as a draft
    fn new() -> Self {
        Self { 
            // When starting a new post, the state is set as draft
            state: Some(Box::new(Draft {})),
            content: String::new()
        }
    }
    // Now, we need a method to add text to the content of the post
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    // And a method that return an empty content for the Post until is approved.
    fn content(&self) -> &str {
        // As the content returned depends of the state. We can delegate the return content
        // to the state. We use as_ref to take a reference to the state and not take ownership
        self.state.as_ref().unwrap().content(self)
    }
    // We add a method to request a review of the post and change it state
    fn request_review(&mut self) {
        // As we take ownership of the state to transform it. We use a Option to keep a 
        // value None when making the change. Rust doesn't allow to unpopulated a field struct
        if let Some(s) = self.state.take() {
            // Here, the request_review method consumes the actual method and change it 
            // for a new one
            self.state = Some(s.request_review())
        }
    }
    // Now, add an approve method that will perform similar to request_review
    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// Trait for the state of the post, allow to share the same behavior between different
// states
trait State {
    // Add the common behavior, request_review. This syntax takes ownership of the previous
    // state Box<Self>, invalidating it to transform it to the new one
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // Approve method for the states
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // Add the content method for the states
    // We use lifetimes because we are taking a reference to an object and returning a 
    // reference to a part of the same object
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// Struct for the draft state of the post
struct Draft {}

impl State for Draft {
    // For Draft, when requesting a review, the state must be changed
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    // As a draft can not be approved without request review, the state stay the same
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// Struct for the pending review state
struct PendingReview {}

impl State for PendingReview {
    // When requesting a review with the state PendingReview, we return the same because
    // it does not change it
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// Struct for the Published state
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn main() {
    // Create a new post
    let mut post = Post::new();

    post.add_text("I like to drink coffee at the morning");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I like to drink coffee at the morning", post.content());
}

/*All this code can be done with Enums too, but it will be repetitive with match expressions
to check states */