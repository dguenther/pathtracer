use crate::hit;
use crate::ray;
use crate::vec3;

pub trait Material: MaterialClone + Send + Sync {
    fn scatter(&self, r_in: &ray::Ray, rec: &hit::HitRecord, attenuation: &mut vec3::Vec3, scattered: &mut ray::Ray) -> bool;
}

pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}
