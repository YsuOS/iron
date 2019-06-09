extern crate iron;
#[macro_use]
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let router = router!(root: get "/" => handler, query: get "/:query" => query_handler);

    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world")))
    }

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}


