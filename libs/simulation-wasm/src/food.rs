use crate::*;

#[derive(Clone, Debug, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl From<&sim::legacy::Food> for Food {
    fn from(food: &sim::legacy::Food) -> Self {
        Self {
            x: food.position().x,
            y: food.position().y,
        }
    }
}
