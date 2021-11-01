use crate::hit;
use crate::material;
use crate::ray;
use crate::vec3;

pub struct Sphere {
    pub radius: f64,
    pub center: vec3::Vec3,
    pub material: Box<dyn material::Material>,
}

impl hit::Hit for Sphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hit::HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = r.point_at_parameter(temp);
                return Some(hit::HitRecord { t: temp, p: point, normal: (point - self.center) / self.radius, material: self.material.clone() });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = r.point_at_parameter(temp);
                return Some(hit::HitRecord { t: temp, p: point, normal: (point - self.center) / self.radius, material: self.material.clone() });
            }
        }
        None
    }
}