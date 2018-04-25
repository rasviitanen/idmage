use super::architect::Blueprint;
use super::svgpower;

pub fn build (bp: &Blueprint) -> String {
    use std::fmt::Write;
    let mut out = String::new();

    SVG!(&mut out,
        svg (version="1.1"
        xmlns="http://www.w3.org/2000/svg" 
        xmlns:xlink="http://www.w3.org/1999/xlink" 
        x="0px" y="0px"
        width={&bp.get_width()}
        height={&bp.get_height()}
        viewBox={format!("0 0 {} {}", &bp.get_scaled_width(), &bp.get_scaled_height())}
        xml:space="preserve") [
            // SVG STARTS HERE
            /*
            @ for tile in bp.get_tiles() {
                for instruction in tile.get_layout().get_paint_instructions() {
                    SVG!(&mut out,
                        {instruction.get_element()} ({instruction.get_attributes()}) [{instruction.get_content()}]
                    );
                };
            };
            */
            //@ svgpower::text::headed_paragraph(&mut out, 200, bp.get_profile().get_name(), "Test paragraph");
        ]
    );

    out
}
