mod distance;

use glam::Vec3;
use std::io::Write;
use std::thread;
use std::sync::mpsc::{self, Sender};

const SCRSIZE: usize = 500;
const MAX_STEPS: usize = 50;

fn trace(from: Vec3, dir: Vec3, de: impl Fn(Vec3) -> f32) -> f32 {
    let mut distance: f32 = 0.;
    let mut steps: usize = 0;
    for _ in 0..MAX_STEPS {
        steps += 1;

        let p: Vec3 = from + dir * distance;
        let dist: f32 = de(p);
        distance += dist;

        if dist < 0.001 {
            break;
        }
    }

    1. - (steps as f32 / MAX_STEPS as f32)
}

fn render_area(ray_orig: Vec3, x0: usize, y0: usize, x1: usize, y1: usize, sender: Sender<usize>) -> Vec<Vec3> {
    let mut frame: Vec<Vec3> = vec![Vec3::ZERO; (x1 - x0) * (y1 - y0)];

    for y in y0..y1 {
        for x in x0..x1 {
            let ha: f32 = x as f32 / SCRSIZE as f32 - 0.4;
            let va: f32 = y as f32 / SCRSIZE as f32 - 0.2;
            let px: f32 = f32::sin(ha);
            let py: f32 = f32::sin(va);

            let x = x - x0;
            let y = y - y0;

            let dir: Vec3 = Vec3::new(px, py, 1.).normalize();
            let color: f32 = trace(ray_orig, dir, distance::takusakuw);
            frame[y * (x1 - x0) + x] = Vec3::new(color, color, color);
        }

        sender.send(x1 - x0).unwrap();
    }

    frame
}

fn main() {
    let orig: Vec3 = Vec3::new(2., -2., -4.5);

    let (send, recv) = mpsc::channel();

    let sendtl = send.clone();
    let sendtr = send.clone();
    let sendbl = send.clone();
    let sendbr = send.clone();
    let topleft = thread::spawn(move || render_area(orig, 0, 0, SCRSIZE / 2, SCRSIZE / 2, sendtl));
    let topright = thread::spawn(move || render_area(orig, SCRSIZE / 2, 0, SCRSIZE, SCRSIZE / 2, sendtr));
    let bottomleft = thread::spawn(move || render_area(orig, 0, SCRSIZE / 2, SCRSIZE / 2, SCRSIZE, sendbl));
    let bottomright = thread::spawn(move || render_area(orig, SCRSIZE / 2, SCRSIZE / 2, SCRSIZE, SCRSIZE, sendbr));
    println!("Waiting for threads to finish...");

    let mut total_pixels: usize = 0;
    loop {
        match recv.recv() {
            Ok(n) => total_pixels += n,
            Err(_) => break,
        }

        print!("\r{:.2}% done", total_pixels as f32 / (SCRSIZE * SCRSIZE) as f32 * 100.);
        std::io::stdout().flush().unwrap();

        if total_pixels == SCRSIZE * SCRSIZE {
            println!();
            break;
        }
    }

    let topleft: Vec<Vec3> = topleft.join().unwrap();
    let topright: Vec<Vec3> = topright.join().unwrap();
    let bottomleft: Vec<Vec3> = bottomleft.join().unwrap();
    let bottomright: Vec<Vec3> = bottomright.join().unwrap();

    println!("Writing thread data to main screen...");
    let mut frame: Vec<Vec3> = vec![Vec3::ZERO; SCRSIZE * SCRSIZE];
    for y in 0..(SCRSIZE / 2) {
        for x in 0..(SCRSIZE / 2) {
            let index: usize = y * SCRSIZE + x;
            let window_index: usize = y * SCRSIZE / 2 + x;
            frame[index] = topleft[window_index];
            frame[index + SCRSIZE / 2] = topright[window_index];
            frame[index + SCRSIZE * SCRSIZE / 2] = bottomleft[window_index];
            frame[index + SCRSIZE * SCRSIZE / 2 + SCRSIZE / 2] = bottomright[window_index];
        }
    }

    println!("Write to out/out.ppm...");
    let mut out: String = format!("P3\n{} {}\n255\n", SCRSIZE, SCRSIZE);
    for i in 0..(SCRSIZE * SCRSIZE) {
        let c: Vec3 = frame[i] * 255.;
        out.push_str(format!("{} {} {}\n", c.x as i32, c.y as i32, c.z as i32).as_str());
    }

    std::fs::write("out/out.ppm", out).unwrap();
    println!("Done");

    std::process::Command::new("feh").arg("out/out.ppm").output().unwrap();
}

