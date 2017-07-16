extern crate parallel_pi_mc;

fn main() {
    use parallel_pi_mc::*;

    println!("sequential: {}", monte_carlo_pi(1000000));
    println!("parallel: {}", monte_carlo_pi_parallel(1000000));
}
