use std::collections::HashMap;

use ggez::graphics::{Color, Rect, WHITE};

#[derive(Clone, Copy)]
pub enum DataType {
    Ball,
    Platform,
    None,
}

pub struct EntityData {
    next_id: u128,
    radiuses: HashMap<u128, f32>,
    default_radius: f32,
    data_types: HashMap<u128, DataType>,
    rectangles: HashMap<u128, Rect>,
    colors: HashMap<u128, Color>,
}

impl EntityData {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            radiuses: HashMap::new(),
            default_radius: 5.0,
            data_types: HashMap::new(),
            rectangles: HashMap::new(),
            colors: HashMap::new(),
        }
    }

    pub fn insert_ball(&mut self, radius: f32, color: Color) -> u128 {
        let id = self.next_id;
        self.next_id += 1;

        self.data_types.insert(id, DataType::Ball);
        self.radiuses.insert(id, radius);
        self.colors.insert(id, color);

        id
    }

    pub fn insert_platform(&mut self, x: f32, y: f32, width: f32, height: f32) -> u128 {
        let id = self.next_id;
        self.next_id += 1;

        self.data_types.insert(id, DataType::Platform);
        let rect = Rect::new(x, y, width, height);
        self.rectangles.insert(id, rect);

        id
    }

    /// Get the radius out of the data given the id. If there isn't an radius for that id then return the default radius
    pub fn get_radius(&self, id: u128) -> f32 {
        if let Some(radius) = self.radiuses.get(&id) {
            *radius
        } else {
            self.default_radius
        }
    }

    pub fn get_data_type(&self, id: u128) -> DataType {
        if let Some(data_type) = self.data_types.get(&id) {
            *data_type
        } else {
            DataType::None
        }
    }

    pub fn get_rect(&self, id: u128) -> Rect {
        if let Some(rect) = self.rectangles.get(&id) {
            *rect
        } else {
            Rect::new(0.0, 0.0, 15.0, 15.0)
        }
    }

    pub fn get_color(&self, id: u128) -> Color {
        if let Some(color) = self.colors.get(&id) {
            *color
        } else {
            WHITE
        }
    }
}
