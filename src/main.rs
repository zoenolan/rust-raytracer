mod camera;
mod colour;
mod common;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use std::io;

use camera::Camera;
use colour::Colour;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Point3;

fn ray_colour(r: &Ray, world: &dyn Hittable, depth: i32) -> Colour {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Colour::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let direction = rec.normal + vec3::random_in_unit_sphere();
        return 0.5 * ray_colour(&Ray::new(rec.p, direction), world, depth - 1);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image size
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + common::random_float()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + common::random_float()) / (IMAGE_HEIGHT - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_colour += ray_colour(&r, &world, MAX_DEPTH);
            }

            colour::write_colour(&mut io::stdout(), pixel_colour, SAMPLES_PER_PIXEL);
        }
    }

    eprint!("\nDone.\n");
}
