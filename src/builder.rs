use std::fmt::Write;

use canvas::Canvas;
use graphic::Graphic;

pub fn build(canvas: &Canvas) -> String {
    let mut out = String::new();
    let (width, height) = canvas.dimensions();

    SVG!(out,
        svg (version="1.1"
            xmlns="http://www.w3.org/2000/svg" 
            xmlns:xlink="http://www.w3.org/1999/xlink" 
            x="0px" y="0px"
            width={width}
            height={height}
            viewBox={format!("0 0 {} {}", width, height)}
            xml:space="preserve") [
                defs [
                    linearGradient (id="MyGrad" x1="0" y1="0" x2="1" y2="1")[
                        stop (offset="20%" stop-color="red")
                        stop (offset="80%" stop-color="blue")
                    ]
                ]
                @ for graphic in canvas.graphics() {
                    construct(&mut out, graphic);
                };
            ]
    );
    out
}

fn construct(out: &mut String, graphic: &Graphic) {
    SVG!(out, 
        {graphic.element()}({graphic.attr_as_str()}) [
            @ for child in graphic.children() {
                construct(out, child)
            };
        ]
    );
}