use super::profiler;
use super::graphic::Graphic;

pub struct Request<'a> {
    pub width: f64,
    pub height: f64,
    pub content: Vec<Graphic<'a>>,
    pub color_type: profiler::ColorType,
}

impl<'a> Request<'a> {
    pub fn new(width: f64, height: f64, content: Vec<Graphic<'a>>, color_type: profiler::ColorType) -> Request<'a> {
        Request {
            width,
            height,
            content,
            color_type
        }
    }

    pub fn add_content(&mut self, graphic: Graphic<'a>) {
        self.content.push(graphic);
    }
}