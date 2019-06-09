extern crate iron;
#[macro_use]
extern crate router;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedQuery;

fn main() {
    let router = router!{
        id_1: get "/" => handler,
        id_2: get "/:query" => query_handler
    };


    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        match req.get_ref::<UrlEncodedQuery>() {
            Ok(ref query) => println!("{:?}", query),
            Err(ref e) => println!("{:?}", e)
        }

        Ok(Response::with((
                    status::Ok, 
                    "hello world"
                    )))
    }

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}


