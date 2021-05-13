use crate::core::config::TILE_PIXEL_SIZE;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn is_straight_neighbour_of(&self, position_to_compare: &Point) -> bool {
        let absolute_delta_x = (self.x - position_to_compare.x).abs();
        let absolute_delta_y = (self.y - position_to_compare.y).abs();

        absolute_delta_x <= 1 && absolute_delta_y <= 1 && absolute_delta_x != absolute_delta_y
    }

    pub fn direction_towards(&self, position_to_compare: &Point) -> Direction {
        let delta_x = self.x - position_to_compare.x;
        let delta_y = self.y - position_to_compare.y;
        let absolute_delta_x = (self.x - position_to_compare.x).abs();
        let absolute_delta_y = (self.y - position_to_compare.y).abs();

        if delta_x > 0 && absolute_delta_x > absolute_delta_y {
            return Direction::Left;
        }

        if delta_y < 0 {
            return Direction::Bottom;
        }

        if delta_x < 0 {
            return Direction::Right;
        }

        Direction::Top
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        return Point::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        return Point::new(self.x - rhs.x, self.y - rhs.y);
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        return Point::new(self.x * rhs.x, self.y * rhs.y);
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, rhs: Point) -> Point {
        return Point::new(self.x / rhs.x, self.y / rhs.y);
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub start: Point,
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    pub fn new(start: Point, width: i32, height: i32) -> Rectangle {
        Rectangle {
            start,
            width,
            height,
        }
    }

    pub fn intersects(&self, r: Rectangle) -> bool {
        self.start.x < r.start.x + r.width
            && self.start.x + self.width > r.start.x
            && self.start.y < r.start.y + r.height
            && self.start.y + self.height > r.start.y
    }
}

impl From<Point> for Rectangle {
    fn from(point: Point) -> Self {
        Rectangle::new(point, 0, 0)
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Bottom,
    Left,
    Right,
    Top,
}

impl Direction {
    pub fn get_lowercase(&self) -> String {
        match self {
            Direction::Bottom => String::from("bottom"),
            Direction::Left => String::from("left"),
            Direction::Right => String::from("right"),
            Direction::Top => String::from("top"),
        }
    }
}

pub fn map_pos_to_pixel_pos(map_position: Point) -> Point {
    Point {
        x: map_position.x * TILE_PIXEL_SIZE,
        y: map_position.y * TILE_PIXEL_SIZE,
    }
}

pub fn pixel_pos_to_map_pos(pixel_position: Point) -> Point {
    pixel_position
        / Point {
            x: TILE_PIXEL_SIZE,
            y: TILE_PIXEL_SIZE,
        }
}
