use crate::ray;
use crate::vec3;
use crate::material;

pub struct HitRecord {
    pub t: f64,
    pub p: vec3::Vec3,
    pub normal: vec3::Vec3,
    pub material: Box<dyn material::Material>,
}

pub trait Hit: Send + Sync {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub v: Vec<Box<dyn Hit>>,
}

impl Hit for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut result = None;
        for hittable in self.v.iter() {
            if let Some(ret) = hittable.hit(r, t_min, closest_so_far) {
                closest_so_far = ret.t;
                result = Some(ret);
            }
        }
        return result;
    }
}