use core::f64;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[macro_use] extern crate itertools;
use rand::prelude::*;
use rayon::prelude::*;

mod camera;
mod dielectric;
mod hit;
mod material;
mod metal;
mod lambertian;
mod ray;
mod sphere;
mod vec3;

fn color<T: hit::Hit>(r: &ray::Ray, world: &T, depth: usize) -> vec3::Vec3 {
    if let Some(rec) = world.hit(r, 0.001, f64::MAX) {
        let mut scattered = ray::Ray { A: vec3::Vec3::new(0.0, 0.0, 0.0), B: vec3::Vec3::new(0.0, 0.0, 0.0) };
        let mut attenuation = vec3::Vec3::new(0.0, 0.0, 0.0);
        if depth < 50 && rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation*color(&scattered, world, depth+1);
        } else {
            return vec3::Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = r.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0-t)*vec3::Vec3::new(1.0, 1.0, 1.0) + t*vec3::Vec3::new(0.5, 0.7, 1.0);
    }
}

fn random_scene() -> hit::HittableList {
    let mut list: Vec<Box<dyn hit::Hit>> = vec![];
    list.push(Box::new(sphere::Sphere { 
        center: vec3::Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(lambertian::Lambertian { albedo: vec3::Vec3::new(0.5, 0.5, 0.5) }),
    }));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = vec3::Vec3::new(a as f64+0.9*random::<f64>(),0.2,b as f64+0.9*random::<f64>());
            if (center-vec3::Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
                if choose_mat < 0.8 {   // diffuse
                    list.push(Box::new(sphere::Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(lambertian::Lambertian { albedo: vec3::Vec3::new(random::<f64>()*random::<f64>(),
                            random::<f64>()*random::<f64>(),
                            random::<f64>()*random::<f64>())})
                    }));
                }
                else if choose_mat < 0.95 { // metal
                    list.push(Box::new(sphere::Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(metal::Metal { albedo: vec3::Vec3::new(0.5*(1.0 + random::<f64>()),
                                            0.5*(1.0 + random::<f64>()),
                                            0.5*(1.0 + random::<f64>())),
                                            fuzz: 0.5*random::<f64>()}),
                    }));
                }
                else {  // glass
                    list.push(Box::new(sphere::Sphere {
                        center: center,
                        radius: 0.2,
                        material: Box::new(dielectric::Dielectric{ref_idx: 1.5})
                    }));
                }
            }
        }
    }

    list.push(Box::new(sphere::Sphere { center: vec3::Vec3::new(0.0, 1.0, 0.0), radius: 1.0, material: Box::new(dielectric::Dielectric{ref_idx: 1.5}) }));
    list.push(Box::new(sphere::Sphere { 
        center: vec3::Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(lambertian::Lambertian { albedo: vec3::Vec3::new(0.4, 0.2, 0.1) })
    }));
    list.push(Box::new(sphere::Sphere {
        center: vec3::Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(metal::Metal { albedo: vec3::Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 })
    }));

    return hit::HittableList { v: list }
}

fn main() {
    let nx = 1200;
    let ny = 800;
    let ns = 100;

    let path = Path::new("test.ppm");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    s.push_str("P3\n");
    s.push_str(&format!("{} {}\n", nx, ny));
    s.push_str("255\n");

    let look_from = vec3::Vec3::new(13.0, 2.0, 3.0);
    let look_at = vec3::Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    
    let cam = camera::Camera::new(
        &look_from,
        &look_at,
        &vec3::Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus
    );
    let world = random_scene();

    // Build a vec of coordinate tuples we need to process.
    // Have to collect the iter to avoid implementing Product in Rayon
    let coords: Vec<(isize, isize)> = iproduct!((0..=ny).rev(), 0..nx).collect();

    let output: Vec<String> = coords.par_iter().map(|(y, x)| {
        let mut col: vec3::Vec3 = (0..ns).map(|_| {
            let u = (*x as f64 + random::<f64>()) / (nx as f64);
            let v = (*y as f64 + random::<f64>()) / (ny as f64);
            let r = cam.get_ray(u, v);
            color(&r, &world, 0)
        }).sum();

        col /= ns as f64;
        // Adjust gamma of the output color
        col = vec3::Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

        let ir = (255.99 * col.x) as usize;
        let ig = (255.99 * col.y) as usize;
        let ib = (255.99 * col.z) as usize;
        format!("{} {} {}", ir, ig, ib)
    }).collect();

    s.push_str(&output.join("\n"));

    // Write the string to `file`, returns `io::Result<()>`
    match file.write_all(s.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}
