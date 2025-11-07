/*Rather than encapsulating the states and transitions completely so outside code has 
no knowledge of them, we’ll encode the states into different types. */

struct Post {
    content: String,
}

// Now, instead of using a content method that returns a empty string for draft post,
// we will make the draft post don't have the content method
struct DraftPost {
    content: String
}

impl Post {
    fn new() -> DraftPost {
        DraftPost { 
            content: String::new(),
        }
    }
    fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    } 
    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { 
            content: self.content 
        }
    }
}

// Extending the concept to new structures for PendingPost
struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    fn approve(self) -> Post {
        Post { 
            content: self.content 
        }
    }
}


fn main() {
    // With the changes, the main flow is sightly different too
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    
    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}