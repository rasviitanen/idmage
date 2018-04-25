#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate svgmacro;
extern crate rocket;

use std::io;

use rocket::response::NamedFile;
use rocket::response::content;
use graphic::Graphic;
use graphic::GraphicType;
use instructions::LayoutInstruction;

mod svgpower;
mod architect;
mod painter;
mod profiler;
mod layout;
mod request;
mod instructions;
mod distributions;
mod graphic;
mod designer;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/canvas")]
fn canvas() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[post("/create")]
fn create() -> content::Xml<String> {
    let profile = profiler::Profile::new();

    let request =
        request::Request::new(
            1280.0, 720.0,
            vec![
                Graphic::new("circle",
                             GraphicType::Text),
            ],
            profiler::ColorType::PRIMARY
        )
    ;

    let blueprint = architect::Blueprint::new(profile, request);

    let out = painter::build(&blueprint);

    println!("{}", out);
    content::Xml(out)
}

fn main() {
    rocket::ignite().mount("/", routes![index, canvas, create]).launch();
}

