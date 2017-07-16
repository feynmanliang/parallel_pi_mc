#![feature(test)]

extern crate test;

extern crate parallel_pi_mc;

#[cfg(test)]
mod tests {
    use parallel_pi_mc::*;
    use test::Bencher;

    #[bench]
    fn bench_sequential_mc(b: &mut Bencher) {
        b.iter(|| monte_carlo_pi(100_000))
    }

    #[bench]
    fn bench_parallel_mc(b: &mut Bencher) {
        b.iter(|| monte_carlo_pi_parallel(100_000))
    }
}
