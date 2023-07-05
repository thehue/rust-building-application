fn main() {
    let addr = "127.0.0.1:8080".to_string();
    let server = Server::new(addr);
    server.run()
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self { addr }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

/*
GET /user?id=10 HTTP/1.1
HEADERS
BODY
 */

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    CONNECT,
    HEAD,
    OPTIONS,
    TRACE,
}
