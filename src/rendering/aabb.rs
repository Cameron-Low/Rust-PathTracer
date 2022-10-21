use std::cmp::Ordering;

use super::math::{Ray, Vec3, vec3};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let mut t0 = (self.min[a] - r.origin[a]) * r.inv_dir[a];
            let mut t1 = (self.max[a] - r.origin[a]) * r.inv_dir[a];
            if r.inv_dir[a] < 0.0 {
                (t0, t1) = (t1, t0);
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = vec3!(box0.min[0].min(box1.min[0]),
                      box0.min[1].min(box1.min[1]),
                      box0.min[2].min(box1.min[2]));
    let big = vec3!(box0.max[0].max(box1.max[0]),
                    box0.max[1].max(box1.max[1]),
                    box0.max[2].max(box1.max[2]));
    Aabb { min: small, max: big }
}

pub fn box_compare(box0: &Aabb, box1: &Aabb, axis: usize) -> Ordering {
    box0.min[axis].total_cmp(&box1.min[axis])
}
