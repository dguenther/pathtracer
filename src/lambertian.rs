use crate::hit;
use crate::material;
use crate::ray;
use crate::vec3;

use rand::prelude::*;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: vec3::Vec3,
}

fn random_in_unit_sphere() -> vec3::Vec3 {
    let mut p: vec3::Vec3;
    while {
        p = 2.0*vec3::Vec3::new(random(), random(), random()) - vec3::Vec3::new(1.0, 1.0, 1.0);
        p.squared_length() >= 1.0
    } {}
    p
}

impl material::Material for Lambertian {
    fn scatter(&self, _r_in: &ray::Ray, rec: &hit::HitRecord, attenuation: &mut vec3::Vec3, scattered: &mut ray::Ray) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = ray::Ray { A: rec.p, B: target-rec.p };
        *attenuation = self.albedo.clone();
        true
    }
}