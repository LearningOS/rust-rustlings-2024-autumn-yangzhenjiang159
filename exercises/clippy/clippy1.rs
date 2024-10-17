// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.
// Clippy 的 approx_constant lint 是为了避免使用不精确的数学常量。在这种情况下，使用 f32::consts::PI 可以确保 π 的精确值（相对于 f32 类型的精度）。
// #[allow(clippy::approx_constant)]   关闭 Clippy


use std::f32;

fn main() {
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
