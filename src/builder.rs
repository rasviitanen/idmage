use canvas::Canvas;

pub fn build(canvas: Canvas) -> String {
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
            circle(cx={width/2.0} cy={height/2.0} r="10" fill={RGBA!(12, 12, 100, 0.5)})
        ]
    );
    println!("{:#?}", canvas.dimensions());
    out
}