#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate svgmacro;
extern crate rocket;

use std::io;

use rocket::response::NamedFile;
use rocket::response::content;

#[macro_use]
mod controller;
#[macro_use]
mod svgpower;

mod agent;
mod builder;
mod canvas;
mod graphic;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/canvas")]
fn canvas() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

// This is a comment
#[get("/generate")]
fn generate() -> content::Xml<String> {
    let mut canvas = canvas::Canvas::new(1920.0, 1080.0);
    let mut controller = controller::Controller::new(&mut canvas);
    let out = controller.build();
    content::Xml(out.to_string())
}

#[post("/create")]
fn create() -> content::Xml<String> {
    let mut canvas = canvas::Canvas::new(1920.0, 1080.0);
    let mut controller = controller::Controller::new(&mut canvas);
    let out = controller.build();
    content::Xml(out.to_string())
}

fn main() {
    rocket::ignite().mount("/", routes![index, canvas, create, generate]).launch();
}

