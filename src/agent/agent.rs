use canvas::Canvas;

pub trait Agent {
    fn update(&self, canvas: &mut Canvas);
}