mod ray;
mod drawable;
mod sphere;
mod camera;

use cgmath::Vector3;
use cgmath::prelude::*;

use drawable::{
    Drawable,
    Drawables,
};

use sphere::Sphere;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn main() {
    let lower_left_corner = cgmath::Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = cgmath::Vector3::<f32>::unit_x() * 4.0;
    let vertical = cgmath::Vector3::<f32>::unit_y() * 2.0;
    let origin = cgmath::Vector3::new(0.0, 0.0, 0.0);

    let mut world = Drawables::new();

    let sphere1 = Sphere::new(
    Vector3::new(0.0, 0.0, -1.0), 
    0.5,
    );

    let sphere2 = Sphere::new(
        Vector3::new(0.0, -100.5, -1.0), 
        100.0,
    );

    world.push(&sphere1);
    world.push(&sphere2);

    // Output An Image
    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let iy = (HEIGHT as u32 - y) as f32;
        let r = ray::Ray::new(origin, lower_left_corner + (x as f32)*horizontal/WIDTH as f32 + iy*vertical/HEIGHT as f32);
        let c = color(&r, &world);

        let r = (255.0 * c[0]) as u8;
        let g = (255.0 * c[1]) as u8;
        let b = (255.0 * c[2]) as u8;
        *pixel = image::Rgb([r, g, b]);
    }
    imgbuf.save_with_format("output.png", image::ImageFormat::Png).unwrap();
}

// fn hit_sphere(centre: &Vector3<f32>, radius: f32, r: &Ray) -> f32 {
//     let oc = r.origin - centre;
//     let a = r.direction.dot(r.direction);
//     let b = 2.0 * oc.dot(r.direction);
//     let c = oc.dot(oc) - radius * radius;
//     let discriminant = b*b - 4.0*a*c;
//     if discriminant < 0 as f32 {
//         -1.0
//     } else {
//         (-b - discriminant.sqrt())/(2.0*a)
//     }
// }

fn color(r: &ray::Ray, world: &Drawables) -> cgmath::Vector3<f32> {
    if let Some(hit) = world.hit(r, 0.0, f32::MAX) {
        return 0.5 * Vector3::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0)
    }
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0-t)*cgmath::Vector3::new(1.0, 1.0, 1.0) + t*cgmath::Vector3::new(0.5, 0.7, 1.0)
}
