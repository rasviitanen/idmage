use canvas::Canvas;

pub fn update(canvas: Canvas) {
    let (width, height) = canvas.dimensions();
    if canvas.aspect_ratio >= 21/9 {
        // ULTRAWIDE
    } else if canvas.aspect_ratio >= 16/9 {
        // WIDESCREEN
    } else if canvas.aspect_ratio >= 4/3 {
        // STANDARD
    }
}