extern crate ndarray;
use image::RgbImage;
use ndarray::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    // not_main();

    // width is always larger than height
    // this is measured in decimeters. width = 10 would mean 1 meter
    let width = 150;
    let height = 120;

    let mut u: Array2<f64> = Array2::zeros((height, width));
    let mut v: Array2<f64> = Array2::zeros((height, width));
    let mut w: Array2<f64> = Array2::zeros((height, width));

    // v[[height / 2, width / 5]] = 100.0;
    v[[height / 2, 10]] = 100.0;

    let probe_x = height / 2;
    let probe_y = 20;

    // so distance between source and sensor is 150 - 10 - 10 = 130 decimeters = 13 meters
    // sound travels at 340 m/s so covering 13 meters should take 38.24 ms.
    // our dt is 1/44300, so it should take 1694 samples from start to when the first disturbance occurs

    let mut samples: Vec<f32> = vec![];
    // println!("In file: {:?}", inp_file);

    // let (header, data) = wav::read(&mut inp_file)?;

    let mut step_count = 0;

    for i in 0..15 {
        // 44300 / 60 = 738 so to simulate a frame for a 60 fps video, step 738 times
        // but remember we use a triple step, so just 246 steps is enough
        // for a 60 FPS movie that plays at 1/4 real speed, use 246 / 4 = 62
        for _j in 0..62 {
            let (a, b, c) = three_step(&mut u, &mut v, &mut w, probe_x, probe_y);
            step_count += 3;

            // if a > 0.001 {
            //     println!("Disturbance at step {}", step_count);
            //     return;
            // }
            samples.push(a as f32);
            samples.push(b as f32);
            samples.push(c as f32);
        }

        println!("Saving frame: {}", i);
        save_img(&v, i)
    }
    println!("Total steps: {}", step_count);

    // println!("Samples: {:?}", samples);
    save_as_wave(samples, "output.wav");

    // let mut out_file = File::create(Path::new("data/output.wav"));
    // wav::write(header, &data, &mut out_file);
}

fn not_main() -> Result<(), std::io::Error> {
    // let mut inp_file = File::open(Path::new("hello.wav"))?;
    // let (header, data) = wav::read(&mut inp_file)?;
    let h = wav::Header {
        audio_format: 3,
        channel_count: 1,
        sampling_rate: 44100,
        bytes_per_second: 176400,
        bytes_per_sample: 4,
        bits_per_sample: 32,
    };

    let samples_per_second = 44300;
    let seconds = 3;
    let pi = std::f32::consts::PI;
    let freq = 440.0;

    let mut fake_data: Vec<f32> = vec![];

    for t in 0..samples_per_second * seconds {
        let t2 = t as f32;
        let s = f32::sin(
            t2 * pi * 2.0 * (freq + t2 * 100.0 / samples_per_second as f32)
                / samples_per_second as f32,
        );
        fake_data.push(s);
    }

    let fd = wav::BitDepth::ThirtyTwoFloat(fake_data);

    let mut out_file = File::create(Path::new("output.wav"))?;
    wav::write(h, &fd, &mut out_file)?;

    Ok(())
}

fn save_as_wave(signal: Vec<f32>, filename: &str) -> Result<(), std::io::Error> {
    let h = wav::Header {
        audio_format: 3,
        channel_count: 1,
        sampling_rate: 44100,
        bytes_per_second: 176400,
        bytes_per_sample: 4,
        bits_per_sample: 32,
    };

    // let mut min = 0.0;
    let mut max = 0.0;
    for sample in signal.iter() {
        if *sample > max {
            max = *sample
        }
    }

    println!("Max: {}", max);

    let mut copy: Vec<f32> = vec![];
    for sample in signal {
        copy.push(sample / max);
    }

    let fd = wav::BitDepth::ThirtyTwoFloat(copy);

    let mut out_file = File::create(Path::new(filename))?;
    wav::write(h, &fd, &mut out_file)?;

    Ok(())
}

fn three_step(
    u: &mut Array2<f64>,
    v: &mut Array2<f64>,
    w: &mut Array2<f64>,
    probe_x: usize,
    probe_y: usize,
) -> (f64, f64, f64) {
    step(u, v, w);
    let a = w[[probe_x, probe_y]];
    step(v, w, u);
    let b = u[[probe_x, probe_y]];
    step(w, u, v);
    let c = v[[probe_x, probe_y]];

    return (a, b, c);
}

fn save_img(arr: &Array2<f64>, index: usize) {
    let colored = colorify(arr);
    let image = array_to_image(colored);
    image.save(format!("imgs/out_{:0>4}.png", index));
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

    // println!("In step: w: {} h: {}", width, height);
    // u is the array at t - 1
    // v is the array at t
    // w is the array at t + 1
    let c = 0.0049;
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
