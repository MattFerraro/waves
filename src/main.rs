extern crate ndarray;
use ndarray::prelude::*;
// use ndarray::{Array, ShapeBuilder};
use image::RgbImage;

fn main() {
    let width = 100;
    let height = 50;

    let mut u: Array2<f64> = Array2::zeros((height, width));
    let mut v: Array2<f64> = Array2::zeros((height, width));
    let mut w: Array2<f64> = Array2::zeros((height, width));

    v[[25, 5]] = 10.0;

    for i in 0..90 {
        step(&mut u, &mut v, &mut w);
        step(&mut v, &mut w, &mut u);
        step(&mut w, &mut u, &mut v);
        save_img(&v, i)
    }
}

fn save_img(arr: &Array2<f64>, index: usize) {
    let colored = colorify(arr);
    let image = array_to_image(colored);
    image.save(format!("out_{}.png", index));
}

fn colorify(arr: &Array2<f64>) -> Array3<u8> {
    let (height, width) = arr.dim();
    // println!("in colorify: w: {} h: {}", width, height);

    let mut output: Array3<u8> = Array3::zeros((height, width, 3));
    for ((x, y), v) in arr.indexed_iter() {
        output[[x, y, 0]] = (*v * 255.0) as u8;
        output[[x, y, 1]] = (*v * 255.0) as u8;
        output[[x, y, 2]] = (*v * 255.0) as u8;
    }
    output
}

fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

fn step(u: &mut Array2<f64>, v: &mut Array2<f64>, w: &mut Array2<f64>) {
    let (height, width) = u.dim();

    println!("In step: w: {} h: {}", width, height);
    // u is the array at t - 1
    // v is the array at t
    // w is the array at t + 1
    let c = 0.1;
    // for ((i, j), val) in u.indexed_iter() {
    for i in 1..width - 1 {
        for j in 1..height - 1 {
            // i goes from 0..width so is the x axis
            // j goes from 0..height so is the y axis

            // w[[x, y]] = w[[x, y]] + 1.0;
            w[[j, i]] = 2.0 * v[[j, i]] - u[[j, i]]
                + c * (v[[j, i + 1]] + v[[j, i - 1]] + v[[j + 1, i]] + v[[j - 1, i]]
                    - 4.0 * v[[j, i]]);
        }
    }
}
