use glam::Vec3;

pub fn sphere(p: Vec3) -> f32 {
    p.length() - 1.
}

