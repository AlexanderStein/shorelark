use crate::*;

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<legacy::Animal>,
    pub(crate) foods: Vec<legacy::Food>,
}

impl World {
    pub fn animals(&self) -> &[legacy::Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[legacy::Food] {
        &self.foods
    }
}

impl World {
    pub(crate) fn random(config: &legacy::Config, rng: &mut dyn RngCore) -> Self {
        let animals = (0..config.world_animals)
            .map(|_| legacy::Animal::random(config, rng))
            .collect();

        let foods = (0..config.world_foods).map(|_| legacy::Food::random(rng)).collect();

        Self { animals, foods }
    }
}
