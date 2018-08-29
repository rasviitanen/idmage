//! This is the entry point, where an HTML request is parsed and routed.
#![feature(extern_prelude)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate svgmacro;
extern crate rocket;
extern crate rand;
extern crate rulinalg;

use std::io;

use rocket::response::NamedFile;
use rocket::response::content;

use agent::canvas::balancer::Balancer;
use agent::canvas::painter::Painter;

#[macro_use]
mod svgpower;
#[macro_use]
mod graphics;
mod controller;
mod agent;
mod builder;
mod canvas;
mod graphic;
mod profiles;
mod math;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
/// Individual profiles, for setting the style
#[get("/canvas")]
fn canvas() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/generate")]
fn generate() -> content::Xml<String> {
    let mut canvas = canvas::Canvas::new(1280.0, 420.0);
    let mut controller = controller::Controller::new(&mut canvas);
    controller.register_agent(Box::new(Painter::new()));
    controller.register_agent(Box::new(Balancer::new()));
    controller.tick();
    let out = controller.build();
    content::Xml(out.to_string())
}

#[post("/create")]
fn create() -> content::Xml<String> {
    let mut canvas = canvas::Canvas::new(1920.0, 1080.0);
    let mut controller = controller::Controller::new(&mut canvas);
    controller.register_agent(Box::new(Painter::new()));
    controller.register_agent(Box::new(Balancer::new()));
    controller.tick();
    let out = controller.build();
    content::Xml(out.to_string())
}

fn main() {
    rocket::ignite().mount("/", routes![index, canvas, create, generate]).launch();
}

