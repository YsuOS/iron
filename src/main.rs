extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", hello_world, "root");
    router.get("/:query", hello_world, "query");

    Iron::new(router).http("localhost:3000").unwrap();

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world")))
    }
}


