pub use self::{animal::*, food::*, world::*};

mod animal;
mod food;
mod world;

use lib_simulation as sim;
use rand::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Statistics {
    age: usize,
    generation_length: usize,
    generation: usize,
    min: Option<f32>,
    avg: Option<f32>,
    max: Option<f32>,
}
#[wasm_bindgen]
impl Statistics {
    fn from_simulation_stats(stats: &sim::SimulationStats) -> Self {
        Self {
            age: stats.age,
            generation_length: stats.generation_length,
            generation: stats.generation,
            min: stats.min_fitness,
            avg: stats.avg_fitness,
            max: stats.max_fitness,
        }
    }
    pub fn age(&self) -> usize {
        self.age
    }
    pub fn generation_length(&self) -> usize {
        self.generation_length
    }
    pub fn generation(&self) -> usize {
        self.generation
    }

    pub fn min(&self) -> Option<f32> {
        self.min
    }

    pub fn avg(&self) -> Option<f32> {
        self.avg
    }

    pub fn max(&self) -> Option<f32> {
        self.max
    }
}
#[wasm_bindgen]
pub struct Simulation {
    rng: ThreadRng,
    sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue) -> Self {
        let config: sim::Config = serde_wasm_bindgen::from_value(config).unwrap();

        let mut rng = thread_rng();
        let sim = sim::Simulation::random(config, &mut rng);

        Self { rng, sim }
    }

    pub fn default_config() -> JsValue {
        serde_wasm_bindgen::to_value(&sim::Config::default()).unwrap()
    }

    pub fn config(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self.sim.config()).unwrap()
    }

    pub fn world(&self) -> JsValue {
        let world = World::from(self.sim.world());
        serde_wasm_bindgen::to_value(&world).unwrap()
    }

    pub fn step(&mut self) -> Statistics {
        let stats = self.sim.step(&mut self.rng);
        Statistics::from_simulation_stats(&stats)
    }

    pub fn train(&mut self) -> Statistics {
        let stats = self.sim.train(&mut self.rng);
        Statistics::from_simulation_stats(&stats)
    }
}
