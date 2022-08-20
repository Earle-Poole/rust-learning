extern crate nickel;
extern crate serde;

use nickel::{FaviconHandler, HttpRouter, Nickel};

mod file_handlers;
mod increment_count;
mod renderers;
mod say_hello;

pub mod structs;

fn main() {
    let addr = "127.42.0.69:6767";
    let favicon_handler = FaviconHandler::new("./assets/favicon.ico");

    let mut server = Nickel::new();

    server.get("/", renderers::home_render);

    server.utilize(favicon_handler);

    server.listen(addr).unwrap();
}
