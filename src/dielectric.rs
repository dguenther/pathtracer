use crate::hit;
use crate::material;
use crate::ray;
use crate::vec3;

use rand::prelude::*;

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0*r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn reflect(v: &vec3::Vec3, n: &vec3::Vec3) -> vec3::Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

fn refract(v: &vec3::Vec3, n: &vec3::Vec3, ni_over_nt: f64) -> Option<vec3::Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
    if discriminant > 0.0 {
        return Some(ni_over_nt*(uv - *n * dt) - *n * discriminant.sqrt());
    }
    else {
        return None;
    }
}

impl material::Material for Dielectric {
    fn scatter(&self, r_in: &ray::Ray, rec: &hit::HitRecord, attenuation: &mut vec3::Vec3, scattered: &mut ray::Ray) -> bool {
        let outward_normal;
        let ni_over_nt;
        let cosine;
        let reflect_prob;
        let reflected = reflect(&r_in.direction(), &rec.normal);
        *attenuation = vec3::Vec3::new(1.0, 1.0, 1.0);

        if r_in.direction().dot(&rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction().dot(&rec.normal)
                        / r_in.direction().length();
        }
        else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction().dot(&rec.normal)
                        / r_in.direction().length();
        }

        let mut refracted = vec3::Vec3::new(1.0, 1.0, 0.0);
        if let Some(val) = refract(&r_in.direction(), &outward_normal, ni_over_nt) {
            reflect_prob = schlick(cosine, self.ref_idx);
            refracted = val;
        } else {
            reflect_prob = 1.0;
        }

        if random::<f64>() < reflect_prob {
            *scattered = ray::Ray { A: rec.p, B: reflected };
        } else {
            *scattered = ray::Ray { A: rec.p, B: refracted };
        }

        return true;
    }
}