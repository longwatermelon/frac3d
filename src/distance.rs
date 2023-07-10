#![allow(dead_code)]
use glam::{Vec2, Vec3, Vec3Swizzles};

pub fn sphere(p: Vec3) -> f32 {
    p.length() - 1.
}

pub fn iq(p: Vec3) -> f32 {
    let ra: f32 = 0.5;
    let rb: f32 = 0.1;
    let h: f32 = 0.4;
    let d: Vec2 = Vec2::new(
        p.xz().length() - 2. * ra + rb,
        p.y.abs() - h
    );

    f32::min(f32::max(d.x, d.y), 0.) + d.max(Vec2::ZERO).length() - rb
}

pub fn takusakuw(p: Vec3) -> f32 {
    let sin: Vec3 = Vec3::new(
        f32::sin(p.x),
        f32::sin(p.y),
        f32::sin(p.z)
    );
    let cos: Vec3 = Vec3::new(
        f32::cos(p.x),
        f32::cos(p.y),
        f32::cos(p.z)
    );

    (sin.zxy() - cos.zzx()).length() - 0.5
}

