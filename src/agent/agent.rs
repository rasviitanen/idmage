use canvas::Canvas;

pub trait Agent {
    fn update(&mut self, canvas: &Canvas);
    fn execute(&mut self, canvas: &mut Canvas);
}