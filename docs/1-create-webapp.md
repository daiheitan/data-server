# Create a Rust-based app

The most popular Rust web framework are: [Iron](https://github.com/iron/iron) and
[Nickel](https://github.com/nickel-org/nickel.rs)

Nickel is more `Express` style based and maybe a better for people with a front-end
background like me. But as I am learning a new language, it would be better to learn
from the ground up so I could know better of the mechanism of a web framework.

So I chose Iron.

The next step is a view library. In my company people use `Jade`. I am not a fan of this
language, as it has a very strange grammar and I just could not remember it and has to
go to the docs everytime. Also, there's no good port of Jade in Rust now. So the choice
would be `handlebars` with [handlebars-iron](https://github.com/sunng87/handlebars-iron).

## The first taste of Iron

After a long evaluation and choosing, here's my final `Cargo.toml`

```manifest
[package]
name = "painter"
version = "0.0.1-alpha"
authors = ["Zimon Dai <daizhuoxian@gmail.com>"]

[dependencies]
iron = "*"
handlebars-iron = "*"
rustc-serialize = "*"
maplit = "*"
```

And here's `main.rs`

```rust
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
```

And the notes I've taken when I copy this sample code from
[here](https://github.com/sunng87/handlebars-iron/blob/master/examples/server.rs)

## #[macro_use]

This is something called `attributes` in Rust. I think it is
the alternative of `annotation` in Java 7 or ES7.

Attributes starts with `#`. Currently only the compiler could define
macros, not users. For a complete list of attributes, see the [ref](https://doc.rust-lang.org/reference.html#attributes).

`#[macro_use]` means that we want to import macros from the extern crate.

## Closures

[Closure](https://doc.rust-lang.org/book/closures.html) is something like anoymous functions in Javascript. The function
`renderer` could be written as:

```rust
let renderer = |_: &mut Request| {
    let mut res = Response::new();
    let data = data::make_data();
    res.set_mut(Template::new("memory", data)).set_mut(status::Ok);
    Ok(res)
}
```

## set_mut

`Iron::prelude` includes a _Trait_ `Set`. It provides
`set` and `set_mut` for all types in Iron. So we could use
`.set_mut` on `Response` to directly write the body of response.

# Run the app

- `cargo build`
- `cargo run`
- Browser, go to http://localhost:3000

Now it should be a simple webpage shouting
`This is Handlebars!`
