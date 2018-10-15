//! Creates the SVG from a canvas
use std::fmt::Write;
use std::fs;

use canvas::Canvas;
use graphic::Graphic;

pub fn build(canvas: &Canvas) -> String {
    let mut out = String::new();
    let (width, height) = canvas.dimensions();

    SVG!(out,
        svg (version="1.1"
            xmlns="http://www.w3.org/2000/svg" 
            xmlns:xhtml="http://www.w3.org/1999/xhtml"
            xmlns:xlink="http://www.w3.org/1999/xlink" 
            x="0px" y="0px"
            width={width}
            height={height}
            viewBox={format!("0 0 {} {}", width, height)}
            xml:space="preserve") [
                @ for graphic in canvas.graphics() {
                    construct(&mut out, graphic);
                };
                // Debug
                circle(cx={canvas.center_of_mass().x} cy={canvas.center_of_mass().y} r="10" stroke="white")
            ]
    );
    
    save_to_file(&out, "static/test.svg");

    out
}

fn save_to_file(contents: &str, path: &str) {
    if fs::write(path, contents).is_err() {
        println!("{:?}", "Error wrinting to file")
    };
}

fn construct(out: &mut String, graphic: &Graphic) {
    SVG!(out, 
        {graphic.element()}({graphic.attr_as_str()}) [
            @ if let Some(text) = graphic.text() { SVG!(out, text) };
            @ for child in graphic.children() {
                construct(out, child)
            };
        ]
    );
}