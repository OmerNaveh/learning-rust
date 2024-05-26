pub mod blog_site {
    pub struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    impl User {
        pub fn new(username: String, email: String) -> User {
            User {
                username,
                email,
                sign_in_count: 0,
                active: true,
            }
        }
        pub fn sign_in(&mut self) {
            self.sign_in_count += 1;
        }
    }

    pub struct Post {
        title: String,
        content: String,
        author: User,
    }
    impl Post {
        pub fn new(title: String, content: String, author: User) -> Post {
            Post {
                title,
                content,
                author,
            }
        }
        pub fn display(&self) {
            println!("Title: {}", self.title);
            println!("Content: {}", self.content);
            println!("Author: {}", self.author.username);
        }
    }

    pub struct Comment {
        content: String,
        author: User,
        post: Post,
    }

    impl Comment {
        pub fn new(content: String, author: User, post: Post) -> Comment {
            Comment {
                content,
                author,
                post,
            }
        }
        pub fn display(&self) {
            println!("Comment: {}", self.content);
            println!("Author: {}", self.author.username);
            println!("Post: {}", self.post.title);
        }
    }

    pub fn run() {
        let mut user1 = User::new("John".to_string(), "john@gmail.com".to_string());
        user1.sign_in();
        let mut user2 = User::new("Jane".to_string(), "jane@gmail.com".to_string());
        user2.sign_in();

        let post = Post::new("Post Title".to_string(), "Post Content".to_string(), user1);

        let comment = Comment::new("Comment Content".to_string(), user2, post);
        comment.display();
    }
}
