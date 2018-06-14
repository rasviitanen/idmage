use canvas::Canvas;

macro_rules! request {
    ($e:expr) => {{
        Request { code: Box::new($e) }
    }};
}

pub struct Request {
    pub code: Box<Fn(&mut Canvas) + Send + Sync>,
}

impl Request {
    pub fn execute(&self, canvas: &mut Canvas) {
        self.code(&canvas);
    }
}