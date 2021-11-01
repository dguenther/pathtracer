use crate::hit;
use crate::material;
use crate::ray;
use crate::vec3;

use rand::prelude::*;

#[derive(Clone)]
pub struct Metal {
    pub albedo: vec3::Vec3,
    pub fuzz: f64,
}

fn random_in_unit_sphere() -> vec3::Vec3 {
    let mut p: vec3::Vec3;
    while {
        p = 2.0*vec3::Vec3::new(random(), random(), random()) - vec3::Vec3::new(1.0, 1.0, 1.0);
        p.squared_length() >= 1.0
    } {}
    p
}

fn reflect(v: &vec3::Vec3, n: &vec3::Vec3) -> vec3::Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

impl material::Material for Metal {
    fn scatter(&self, r_in: &ray::Ray, rec: &hit::HitRecord, attenuation: &mut vec3::Vec3, scattered: &mut ray::Ray) -> bool {
        let reflected = reflect(&r_in.direction().unit_vector(), &rec.normal);
        *scattered = ray::Ray { A: rec.p, B: reflected + self.fuzz * random_in_unit_sphere() };
        *attenuation = self.albedo.clone();
        scattered.direction().dot(&rec.normal) > 0.0
    }
}