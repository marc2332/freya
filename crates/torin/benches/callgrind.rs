pub mod support;

use crate::support::{
    BenchmarkConfig,
    BenchmarkInput,
};

const ITERATIONS: usize = 3;

#[hotpath::main(functions_limit = 0)]
fn main() {
    let measurements: [fn(&mut BenchmarkInput); 11] = [
        flat_1000,
        flat_10000,
        flat_100000,
        binary_depth_12,
        binary_depth_14,
        binary_depth_17,
        wide_depth_5,
        wide_depth_5_invalidated,
        wide_depth_7,
        wide_depth_7_invalidated,
        aligned_depth_8,
    ];
    let configs = BenchmarkConfig::all();
    assert_eq!(configs.len(), measurements.len());

    for (config, measure) in configs.into_iter().zip(measurements) {
        for _ in 0..ITERATIONS {
            let mut input = config.prepare();
            measure(std::hint::black_box(&mut input));
        }
    }
}

#[hotpath::measure]
fn flat_1000(input: &mut BenchmarkInput) {
    input.run();
}

#[hotpath::measure]
fn flat_10000(input: &mut BenchmarkInput) {
    input.run();
}

#[hotpath::measure]
fn flat_100000(input: &mut BenchmarkInput) {
    input.run();
}

#[hotpath::measure]
fn binary_depth_12(input: &mut BenchmarkInput) {
    input.run();
}

#[hotpath::measure]
fn binary_depth_14(input: &mut BenchmarkInput) {
    input.run();
}

#[hotpath::measure]
fn binary_depth_17(input: &mut BenchmarkInput) {
    input.run();
}

#[hotpath::measure]
fn wide_depth_5(input: &mut BenchmarkInput) {
    input.run();
}

#[hotpath::measure]
fn wide_depth_5_invalidated(input: &mut BenchmarkInput) {
    input.run();
}

#[hotpath::measure]
fn wide_depth_7(input: &mut BenchmarkInput) {
    input.run();
}

#[hotpath::measure]
fn wide_depth_7_invalidated(input: &mut BenchmarkInput) {
    input.run();
}

#[hotpath::measure]
fn aligned_depth_8(input: &mut BenchmarkInput) {
    input.run();
}
