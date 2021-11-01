use crate::ray::Ray;
use crate::vec3::Vec3;
use core::f64::consts;

use rand::prelude::*;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    while {
        p = 2.0 * Vec3::new(random(), random(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        p.dot(&p) >= 1.0
    } {}
    p
}


impl Camera {
    pub fn new(look_from: &Vec3, look_at: &Vec3, vup: &Vec3, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Self {
        let lens_radius = aperture / 2.0;
        // vfov is top to bottom in degrees
        let theta = vfov * consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (*look_from - *look_at).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        Self { 
            lower_left_corner: *look_from - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: *look_from,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray { A: self.origin + offset, B: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset }
    }
}
