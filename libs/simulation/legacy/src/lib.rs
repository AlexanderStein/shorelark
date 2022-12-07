pub use self::{animal::*, animal_individual::*, brain::*, config::*, eye::*, food::*};

mod animal;
mod animal_individual;
mod brain;
mod config;
mod eye;
mod food;

use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};
use std::f32::consts::*;
