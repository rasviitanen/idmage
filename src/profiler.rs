pub enum ContentType {
    LOGO,
}

pub struct Icon {
    src: String,
    content_type: ContentType,
}

impl Icon {
    pub fn new(src: &str, content_type: ContentType) -> Icon {
        Icon {
            src: src.to_string(),
            content_type,
        }
    }
}

pub enum ColorType {
    PRIMARY,
    SECONDARY,
    COMPLEMENTARY,
}

pub struct Color {
    rgba: (i32, i32, i32),
}

impl Color {
    pub fn rgba(&self) -> (i32, i32, i32) {
        return self.rgba   
    }
}

pub struct Profile {
    name: String,
    icons: Vec<Icon>,
    text: String,
    colors: Vec<Color>,
}

impl Profile {
    pub fn new() -> Profile {
        Profile {
            name: "Example Profile".to_string(),
            text: "text".to_string(),
            icons: vec![Icon::new("circle", ContentType::LOGO)],
            colors: vec![Color{rgba: (0, 0, 0)}, Color{rgba: (255, 255, 255)}, Color{rgba: (255, 0, 0)}],
        }
    }

    pub fn get_name(&self) -> &String {&self.name}
    pub fn get_text(&self) -> &String {&self.text}
    pub fn get_icons(&self) -> &Vec<Icon> {&self.icons}
    pub fn get_colors(&self) -> &Vec<Color> {&self.colors}

    pub fn get_color(&self, color_type: ColorType) -> &self::Color {
        match color_type {
            ColorType::PRIMARY => {
                println!("THIS IS PRIMARY");
                &self.colors[0]
            },
            ColorType::SECONDARY => {
                println!("THIS IS SECONDARY");
                &self.colors[1]
            },
            ColorType::COMPLEMENTARY => {
                println!("THIS IS COMPLEMENTARY");
                &self.colors[2]
            },
        }
    }
}
