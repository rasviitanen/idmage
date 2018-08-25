#[allow(dead_code)]
pub mod animate {
    pub fn fade_in_out(out: &mut String){
        use std::fmt::Write;
        SVG!(out,
            animate (
                attributeName="opacity"
                attributeType="XML"
                values="0.5; 1; 0.5"
                begin="0" dur="3"
                repeatCount="indefinite"
            )
        );
    }

    pub fn rotate(out: &mut String, x: f64, y: f64){
        use std::fmt::Write;
        SVG!(out,
            animateTransform (
                attributeType="xml"
                attributeName="transform"
                type="rotate"
                from={format!("0 {} {}", x, y)}
                to={format!("360 {} {}", x, y)}
                dur="30s"
                repeatCount="indefinite"
            )
        );
    }
}

#[allow(dead_code)]
pub mod text {
    extern crate textwrap;

    pub fn headed_paragraph(out: &mut String, width: usize, header: &str, text: &str){
        use std::fmt::Write;
        let chars_in_a_row: usize;
        chars_in_a_row = width/6;
        let mut first_line = true;

        SVG!(out,
            text (font-size="52" y="64" text-anchor="middle" fill="black")[
                @ for line in textwrap::fill(header, chars_in_a_row).split('\n') {
                    SVG!(out,
                            tspan (font-size="64" x="50%" dy="0")[{line}]
                        ); 
                };
                @ for line in textwrap::fill(text, chars_in_a_row).split('\n') {
                    if first_line {
                        // Add a greater space between the header and the first line
                        // In the paragraph
                        SVG!(out,
                            tspan (x="50%" dy="52")[{line}]
                        ); 
                        first_line = false;
                    } else {
                        SVG!(out,
                                tspan (x="50%" dy="12")[{line}]                 
                            ); 
                    };
                };
            ]
        );   
    }
    
    pub fn paragraph(out: &mut String, width: usize, text: &str){
        use std::fmt::Write;
        let chars_in_a_row: usize;
        chars_in_a_row = width/6;
        SVG!(out,
            text (font-size="12" y="50%" text-anchor="middle" fill="white")[
                @ for line in textwrap::fill(text, chars_in_a_row).split('\n') {
                    SVG!(out,
                            tspan (x="50%" dy="12")[{line}]                 
                        ); 
                };
            ]
        );   
    }
}
