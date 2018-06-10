use canvas::Canvas;

macro_rules! request {
    ($e:expr) => {{
        Request { code: Box::new($e) }
    }};
}

pub struct Request {
    pub code: Box<Fn(&mut Canvas)>,
}

impl Request {
    pub fn execute(&self, canvas: &mut Canvas) {
        (self.code)(canvas.borrow_mut());
    }
}