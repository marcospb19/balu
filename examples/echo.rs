use balu::{Request, Response, Router, Server};

fn main() {
    let router = Router::new().get("/echo1", echo1).get("/echo2", echo2);

    Server::new(router).serve();
}

fn echo1(request: Request) -> String {
    request.body
}

fn echo2(request: Request) -> Response {
    Response::new().body(request.body)
}
