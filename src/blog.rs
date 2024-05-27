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

    pub struct Post<'a> {
        title: String,
        content: String,
        author: &'a User,
    }
    impl<'a> Post<'a> {
        pub fn new(title: String, content: String, author: &'a User) -> Post {
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

    pub struct Comment<'a,'b> {
        content: String,
        author: &'a User,
        post: &'b Post<'a>,
    }

    impl <'a, 'b> Comment<'a, 'b> {
        pub fn new(content: String, author: &'a User, post: &'b Post<'a>) ->  Comment<'a,'b> {
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

        let post = Post::new("Post Title".to_string(), "Post Content".to_string(), &user1);

        let comment = Comment::new("Comment Content".to_string(), &user2, &post);
        comment.display();
        post.display();
        
        println!("{} has signed in {} times", user1.username, user1.sign_in_count);
        println!("{} has signed in {} times", user2.username, user2.sign_in_count);
    }
}
