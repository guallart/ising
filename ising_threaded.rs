use crate::constants::{J, K_B, L, N_ITERS};
use crate::random_generator::RandomGenerator;
use crate::types_def::{Grid, Index, InitMode, Neighbours, Spin};
use ndarray::Array2;
use std::fs::File;
use std::io::Write;

// Constante de normalizacion de las variables
const NORM_C: f64 = 1.0 / ((L * L) as f64);

// Variacion de energia en cada inversion
// en funcion de los vecinos y del espin
const ENERGY_DELTAS: [f64; 5] = [
    -8.0 * J * NORM_C,
    -4.0 * J * NORM_C,
    0.0,
    4.0 * J * NORM_C,
    8.0 * J * NORM_C,
];

// Variacion de la magneitzacion en cada inversion
const MAGNET_DELTA: f64 = 2.0 * NORM_C;

fn compute_change_probabilities(temperature: f64) -> [f64; 5] {
    // Probabilidades de inversion en funcion de los vecinos
    [
        (J * 8.0 / (K_B * temperature)).exp(),
        (J * 4.0 / (K_B * temperature)).exp(),
        1.0,
        (-J * 4.0 / (K_B * temperature)).exp(),
        (-J * 8.0 / (K_B * temperature)).exp(),
    ]
}

fn init_cell((i, j): Index, mode: InitMode, rng: &mut RandomGenerator) -> Spin {
    match mode {
        InitMode::Random => {
            if rng.bran() {
                1_i8
            } else {
                -1_i8
            }
        }
        InitMode::Chess => {
            if (i + j) % 2 == 0 {
                1_i8
            } else {
                -1_i8
            }
        }
        InitMode::AllUp => 1_i8,
        InitMode::AllDown => -1_i8,
    }
}

fn neighbours_indices((i, j): Index) -> [Index; 4] {
    [
        ((i + L - 1) % L, j),
        ((i + L + 1) % L, j),
        (i, (j + L + 1) % L),
        (i, (j + L - 1) % L),
    ]
}

pub struct IsingModelThreaded {
    grid: Grid,                     // Grid de espines
    neighs: Neighbours,             // Lista de vecinos para cada posicion
    rng: RandomGenerator,           // Generador de numeros aleatorios
    change_probabilities: [f64; 5], // Probabilidades de inversion de espin
    energy: f64,                    // Energias
    magnet: f64,                    // Magnetizacion
}

impl IsingModelThreaded {
    pub fn new(mode: InitMode, temperature: f64) -> Self {
        // Se iniclializan las variables
        let mut rng: RandomGenerator = RandomGenerator::new();
        let grid: Grid = Array2::from_shape_fn((L, L), |idx| init_cell(idx, mode, &mut rng));
        let neighs: Neighbours = Array2::from_shape_fn((L, L), |idx| neighbours_indices(idx));
        let change_probabilities: [f64; 5] = compute_change_probabilities(temperature);

        // Se inicializa el objeto IsingModel
        let mut ising = IsingModel {
            grid,
            neighs,
            rng,
            change_probabilities,
            energy: 0.0,
            magnet: 0.0,
        };

        // Se hace el calculo inicial de la energia y la magnetizacion
        ising.compute_energy();
        ising.compute_magnet();

        ising
    }

    fn compute_energy(&mut self) {
        // Se suma la contribucion a la energia de todos los spines
        let mut sum = 0.0;
        for (idx, cell) in self.grid.indexed_iter() {
            for neig in self.neighs[idx] {
                sum -= J * ((*cell) * self.grid[neig]) as f64;
            }
        }

        // Se descartan las interacciones que se han sumado dos veces
        // Se normaliza al número de celdas del sistema
        self.energy = sum * 0.5 * NORM_C;
    }

    fn compute_magnet(&mut self) {
        // Se suman los espines de toda la red
        // y se normaliza
        self.magnet = self.grid.mapv(|x| x as f64).sum() * NORM_C;
    }

    fn try_update_spin(&mut self) {
        // Se selecciona un espin al azar y se hace una prueba de invertirlo
        let idx: Index = self.rng.index();

        // Suma de los espines de los vecinos
        let sum_neighs = self.neighs[idx]
            .iter()
            .map(|&n_idx| self.grid[n_idx])
            .sum::<i8>();

        // Probabilidad de inversion del espin
        let prob_idx = ((self.grid[idx] * sum_neighs) / 2 + 2) as usize;

        // Metropolis
        if self.change_probabilities[prob_idx] > self.rng.fran() {
            // Se invierte el espin
            self.grid[idx] *= -1;

            // Se acumula el cambio de energia y magnetizacion de esta iteracion
            self.energy += ENERGY_DELTAS[prob_idx];
            self.magnet += MAGNET_DELTA * self.grid[idx] as f64;
        }
    }

    pub fn thermalize(&mut self, n_iters: usize) {
        // Se deja evolucionar el sistema sin registrar las variables
        for _ in 0..n_iters {
            for _ in 0..L * L {
                self.try_update_spin();
            }
        }
    }

    pub fn evolve(&mut self, fname: &String) {
        // Se deja evolucionar el sistema, registrando la energia y magnetizacion
        let mut file_out = File::create(fname).expect("Error opening the file");
        for _ in 0..N_ITERS {
            for _ in 0..L * L {
                self.try_update_spin();
            }
            writeln!(&mut file_out, "{} {}", self.energy, self.magnet).expect("Error writing");
        }
    }
}
