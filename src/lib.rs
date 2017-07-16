extern crate rand;
extern crate rayon;

use rayon::prelude::*;

fn within_circle() -> bool {
    let x = rand::random::<f64>() * 2f64 - 1f64;
    let y = rand::random::<f64>() * 2f64 - 1f64;
    x * x + y * y <= 1f64
}

pub fn monte_carlo_pi(num_points: usize) -> f64 {
    let num_within_circle = (0..num_points)
        .into_iter()
        .map(|_| within_circle())
        .filter(|x| *x)
        .count();
    4f64 * num_within_circle as f64 / num_points as f64
}

pub fn monte_carlo_pi_parallel(num_points: usize) -> f64 {
    let num_within_circle = (0..num_points)
        .into_par_iter()
        .map(|_| within_circle())
        .filter(|x| *x)
        .count();
    4f64 * num_within_circle as f64 / num_points as f64
}

