mod constants;
mod ising;
mod random_generator;
mod types_def;

use crate::constants::{L, N_TEMP, T_FIN, T_INI};
use crate::ising::IsingModel;
use crate::types_def::InitMode;
use std::path::Path;

fn linspace(x_ini: f64, x_end: f64, steps: usize) -> Vec<f64> {
    let step = (x_end - x_ini) / (steps - 1) as f64;
    (0..steps).map(|i| x_ini + (i as f64) * step).collect()
}

fn main() {
    let temperatures = linspace(T_FIN, T_INI, N_TEMP);

    for temp in temperatures {
        println!("L={}  T={}", L, temp);
        let fname = String::from(format!("results/L{}_T{:.3}.txt", L, temp));
        let mut ising = IsingModel::new(InitMode::Random, temp);
        ising.evolve(&fname);
    }
}
