use super::Point;

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}

#[derive(Clone)]
pub enum SpriteType {
    Image,
    Text,
    Rect,
}

#[derive(Clone)]
pub struct Sprite {
    pub position: Point,
    pub texture_path: String,
    pub visible: bool,
    pub sprite_type: SpriteType,
    pub text: String,
    pub width: u32,
    pub height: u32,
    pub font_size: u16,
    pub color: Color,
    pub rotation: f64,
}

impl Sprite {
    pub fn create_image(
        texture_path: &str,
        position: Point,
        width: u32,
        height: u32,
        rotation: f64,
    ) -> Sprite {
        Sprite {
            position,
            sprite_type: SpriteType::Image,
            texture_path: String::from(texture_path),
            visible: true,
            text: String::from(""),
            width,
            height,
            font_size: 0,
            color: Color::new(0, 0, 0, 0),
            rotation,
        }
    }

    pub fn create_text(text: &str, position: Point, font_size: u16) -> Sprite {
        Sprite {
            position,
            sprite_type: SpriteType::Text,
            texture_path: String::from(""),
            visible: true,
            text: String::from(text),
            width: 0,
            height: 0,
            font_size,
            color: Color::new(0, 0, 0, 0),
            rotation: 0.0,
        }
    }

    pub fn empty() -> Sprite {
        Sprite {
            position: Point { x: 0, y: 0 },
            texture_path: String::from(""),
            visible: false,
            sprite_type: SpriteType::Image,
            text: String::from(""),
            width: 0,
            height: 0,
            font_size: 0,
            color: Color::new(0, 0, 0, 0),
            rotation: 0.0,
        }
    }

    pub fn create_rect(color: Color, position: Point, width: u32, height: u32) -> Sprite {
        Sprite {
            position,
            sprite_type: SpriteType::Rect,
            texture_path: String::from(""),
            visible: true,
            text: String::from(""),
            width,
            height,
            font_size: 0,
            color,
            rotation: 0.0,
        }
    }
}

pub trait GameRenderer {
    fn draw(&mut self, sprites: &Vec<Sprite>) -> ();
}
