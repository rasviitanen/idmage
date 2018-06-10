use canvas::Canvas;

pub trait Agent {
    fn update(&mut self, canvas: &mut Canvas);
    fn execute(&mut self, canvas: &mut Canvas);
}