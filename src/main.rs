mod colour;
mod common;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use std::io;

use colour::Colour;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};

fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = vec3::dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f32::sqrt(discriminant)) / a
    }
}

fn ray_colour(r: &Ray, world: &dyn Hittable) -> Colour {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, common::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0));
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

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0 as f32;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0 as f32;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin
        - horizontal / 2.0 as f32
        - vertical / 2.0 as f32
        - Vec3::new(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);

        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_colour = ray_colour(&r, &world);
            colour::write_colour(&mut io::stdout(), pixel_colour);
        }
    }

    eprint!("\nDone.\n");
}
