use canvas::Canvas;

pub trait CanvasAgent {
    fn update(&mut self, canvas: &Canvas);
    fn execute(&mut self, canvas: &mut Canvas);
}