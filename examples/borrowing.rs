use balu::{Router, Server};

fn main() {
    // This isn't `&'static str` to showcase immutable borrowing in handlers
    let greet_message = String::from("Hello World!");

    let router = Router::new().get("/greet", |_request| greet_message.clone());

    Server::new(router).serve();
}
