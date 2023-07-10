mod distance;

use glam::Vec3;

const SCRSIZE: usize = 500;

fn trace(from: Vec3, dir: Vec3, de: impl Fn(Vec3) -> f32) -> f32 {
    let mut distance: f32 = 0.;
    let mut steps: i32 = 0;
    for _ in 0..100 {
        steps += 1;

        let p: Vec3 = from + dir * distance;
        let dist: f32 = de(p);
        distance += dist;

        if dist < 0.1 {
            break;
        }
    }

    1. - (steps as f32 / 100.)
}

fn main() {
    let mut frame: Vec<Vec3> = vec![Vec3::ZERO; SCRSIZE * SCRSIZE];

    for y in 0..SCRSIZE {
        for x in 0..SCRSIZE {
            let ha: f32 = x as f32 / SCRSIZE as f32 - 0.5;
            let va: f32 = y as f32 / SCRSIZE as f32 - 0.5;
            let px: f32 = f32::sin(ha);
            let py: f32 = f32::sin(va);

            let dir: Vec3 = Vec3::new(px, py, 1.).normalize();
            let color: f32 = trace(Vec3::new(0., 0., -3.), dir, distance::sphere);
            frame[y * SCRSIZE + x] = Vec3::new(color, color, color);
        }
    }

    let mut out: String = format!("P3\n{} {}\n255\n", SCRSIZE, SCRSIZE);
    for i in 0..(SCRSIZE * SCRSIZE) {
        let c: Vec3 = frame[i] * 255.;
        out.push_str(format!("{} {} {}\n", c.x as i32, c.y as i32, c.z as i32).as_str());
    }

    std::fs::write("out/out.ppm", out).unwrap();

    std::process::Command::new("feh").arg("out/out.ppm").output().unwrap();
}

