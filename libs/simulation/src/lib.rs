pub use self::{statistics::*};

mod statistics;
pub mod world;

pub use lib_simulation_legacy as legacy;
use lib_genetic_algorithm as ga;
use nalgebra as na;
use rand::{Rng, RngCore};

pub struct SimulationStats {
    pub age: usize,
    pub generation_length: usize,
    pub generation: usize,
    pub min_fitness: Option<f32>,
    pub avg_fitness: Option<f32>,
    pub max_fitness: Option<f32>,
}

pub struct Simulation {
    config: legacy::Config,
    world: world::World,
    age: usize,
    generation: usize,
}

impl Simulation {
    pub fn random(config: legacy::Config, rng: &mut dyn RngCore) -> Self {
        let world = world::World::random(&config, rng);

        Self {
            config,
            world,
            age: 0,
            generation: 0,
        }
    }

    pub fn config(&self) -> &legacy::Config {
        &self.config
    }

    pub fn world(&self) -> &world::World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> SimulationStats {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();
        let gen_stat = self.try_evolving(rng);
        self.generate_statistics(&gen_stat)
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> SimulationStats {
        loop {
            let summary = self.step(rng);
            if summary.age == 0 {
                return summary;
            }
        }
    }
}

impl Simulation {
    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= self.config.food_size {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            animal.process_brain(&self.config, &self.world.foods);
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.process_movement();
        }
    }

    fn try_evolving(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.age += 1;

        if self.age > self.config.sim_generation_length {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> Statistics {
        self.age = 0;
        self.generation += 1;

        let mut individuals: Vec<_> = self
            .world
            .animals
            .iter()
            .map(legacy::AnimalIndividual::from_animal)
            .collect();

        if self.config.ga_reverse == 1 {
            let max_satiation = self
                .world
                .animals
                .iter()
                .map(|animal| animal.satiation)
                .max()
                .unwrap_or_default();

            for individual in &mut individuals {
                individual.fitness = (max_satiation as f32) - individual.fitness;
            }
        }

        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(self.config.ga_mut_chance, self.config.ga_mut_coeff),
        );

        let (individuals, statistics) = ga.evolve(rng, &individuals);

        self.world.animals = individuals
            .into_iter()
            .map(|i| i.into_animal(&self.config, rng))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.gen();
        }

        Statistics {
            generation: self.generation - 1,
            ga: statistics,
        }
    }

    pub fn generate_statistics(&self, stats: &Option<Statistics>) -> SimulationStats {
        SimulationStats {
            age: self.age,
            generation_length: self.config.sim_generation_length,
            generation: self.generation,
            min_fitness: stats.as_ref().map(|stats| stats.ga.min_fitness()),
            avg_fitness: stats.as_ref().map(|stats| stats.ga.avg_fitness()),
            max_fitness: stats.as_ref().map(|stats| stats.ga.max_fitness()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    #[ignore]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut sim = Simulation::random(Default::default(), &mut rng);

        let avg_fitness = (0..10)
            .map(|_| sim.train(&mut rng).avg_fitness.unwrap())
            .sum::<f32>()
            / 10.0;

        approx::assert_relative_eq!(31.944998, avg_fitness);
    }
}
