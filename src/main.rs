extern crate iron;
extern crate handlebars_iron as hbs;
extern crate rustc_serialize;
#[macro_use]
extern crate maplit;

use std::error::Error;

use iron::prelude::*;
use iron::{status};
use hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};

mod data {
    use rustc_serialize::json::{ToJson, Json};
    use std::collections::BTreeMap;
    
    pub fn make_data () -> BTreeMap<String, Json> {
        let mut data = BTreeMap::new();
        data
    }
}

fn renderer(_: &mut Request) -> IronResult<Response> {
    let mut res = Response::new();
    let data = data::make_data();
    res.set_mut(Template::new("memory", data)).set_mut(status::Ok);
    Ok(res)
}

fn main() {
    let mut chain = Chain::new(renderer);
    let mut hbse = HandlebarsEngine::new2();
    
    // add a memory based source
    let mem_templates = btreemap! {
        "memory".to_owned() => "<h1>This is HandleBars!</h1>".to_owned()
    };
    hbse.add(Box::new(MemorySource(mem_templates)));
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }
    chain.link_after(hbse);
    Iron::new(chain).http("localhost:3000").unwrap();
    println!("Server started on 3000");
}