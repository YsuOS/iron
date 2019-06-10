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
        let ref query = req.get_ref::<UrlEncodedQuery>().unwrap();
        let value = query.get("key").unwrap();
        let value1 = value[0].parse::<u32>().unwrap();
        let value2 = value[1].parse::<u32>().unwrap();
        println!("{:?}", query);
//        println!("{}", value1);
//        println!("{}", value2);

        Ok(Response::with((
                    status::Ok, 
                    (value1 + value2).to_string()
                    )))
    }

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}


