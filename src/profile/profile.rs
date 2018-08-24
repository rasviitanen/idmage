use graphic::Graphic;

pub trait Profile {
    fn font_family(&self) -> &Vec<String>;
    fn text_colors(&self) -> &Vec<String>;
    fn primary_colors(&self) -> &Vec<String>;
    fn background_colors(&self) -> &Vec<String>;
    fn main_background(&self, x: f64, y:f64, width:f64, height: f64) -> Graphic;
    fn logo(&self, x: f64, y: f64, size: f64) -> Graphic;
}
