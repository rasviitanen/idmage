use std::collections::HashMap;

use profiler;

pub enum GraphicType {
    Background,
    Foreground,
    Solid,
    Text,
    Illustration,
    Gradient,
    Icon,
}

pub struct Graphic<'a> {
    element: String,
    attributes: HashMap<&'a str, &'a str>,
    graphic_type: GraphicType,
}

impl<'a> Graphic<'a> {
    pub fn new(element: &str, graphic_type: GraphicType) -> Graphic<'a> {
        Graphic{
            element: element.to_string(),
            attributes: HashMap::new(),
            graphic_type,
        }
    }

    pub fn set_attribute(&mut self, attribute: &'a str, value: &'a str) {
        self.attributes.insert(attribute, value);
    }

    pub fn get_type(&self) -> &GraphicType {&self.graphic_type}
    pub fn get_element(&self) -> &String {&self.element }
    pub fn get_attributes_as_string(&self) -> String {
        let mut attribute_string = "".to_string();
        for (attribute, value) in &self.attributes {
            attribute_string = format!("{} {}=\"{}\"", attribute_string, attribute, value);
        };
        attribute_string
    }
}