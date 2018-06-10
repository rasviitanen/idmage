use canvas::Canvas;
macro_rules! request {
    ($e:expr) => {{
        Some(Box::new($e))
    }};
}

pub trait Agent {
    fn update(&mut self, canvas: &mut Canvas);
    fn execute(&mut self, canvas: &mut Canvas);
}