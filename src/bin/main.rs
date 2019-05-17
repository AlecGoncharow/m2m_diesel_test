use iron::Iron;
use router::Router;

use m2m_diesel_test::{hello, get_server_port};

fn main() {
    let mut router = Router::new();
    router.get("/", hello, "index");

    let addr = "0.0.0.0";
    let port = get_server_port();
    match Iron::new(router)
        .http((addr, port)) {
        Ok(_) =>  println!("Server is running at: http://{}:{}", addr, port),
        Err(e) => println!("Error starting server: {}", e)
    }
}
