use canvas::Canvas;

pub fn build(canvas: &Canvas) -> String {
    use std::fmt::Write;
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
            @ for tile in canvas.tiles() {
                for graphic in tile.graphics() {
                    SVG!(out, 
                        {graphic.element()}({graphic.attr_as_str()})
                    );
                }
            };
        ]
    );
    out
}