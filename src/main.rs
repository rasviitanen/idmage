#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate svgmacro;
extern crate rocket;

use std::io;

use rocket::response::NamedFile;
use rocket::response::content;

#[macro_use]
mod svgpower;
mod canvas;
mod builder;

#[get("/")]
fn index() -> &'static str {
#[macro_use]
    "Hello, world!"
}

#[get("/canvas")]
fn canvas() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}
// This is a comment
#[post("/create")]
fn create() -> content::Xml<String> {
    let canvas = canvas::Canvas::new(1280.0, 720.0);
    let out = builder::build(canvas);
    println!("{:?}", out);
    content::Xml(out)
}

fn main() {
    rocket::ignite().mount("/", routes![index, canvas, create]).launch();
}

