# Ising Model in Rust

This repository contains an implementation of the **Ising model** written in Rust. The project allows simulating the behavior of a two-dimensional magnetic system and calculating typical statistical quantities such as **magnetization** and **specific heat (Cv)**. Additionally, it includes a **threaded version** to improve performance for large simulations.

---

## Overview

The Ising model is a classic model in statistical physics that describes interactions between spins on a lattice. It is fundamental for studying phase transitions and critical phenomena.

This Rust implementation offers:

- Simulation of the 2D Ising model.
- Calculation of statistical quantities including:
  - Average magnetization.
  - Specific heat (Cv).
- Parallelized version using threads to speed up simulation and sampling.

---

## Features

- **Rust**: safe and efficient language ideal for numerical computations.
- Monte Carlo simulation using the Metropolis algorithm.
- Computation of thermodynamic properties at various temperatures.
- Multithreading support to utilize multiple CPU cores.

