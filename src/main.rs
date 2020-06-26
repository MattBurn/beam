mod ray;
mod drawable;
mod sphere;
mod camera;
mod material;

use rand::Rng;
use cgmath::Vector3;
use cgmath::prelude::*;

use camera::Camera;
use drawable::{
    Drawable,
    Drawables,
};

use sphere::Sphere;

use material::{
    Diffuse,
    Metal,
};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const MAX_DEPTH: u32 = 50;

fn main() {
    let camera = Camera::new(
        cgmath::Vector3::new(0.0, 0.0, 0.0),
        cgmath::Vector3::<f32>::unit_x() * 4.0,
        cgmath::Vector3::<f32>::unit_y() * 2.0,
        cgmath::Vector3::new(-2.0, -1.0, -1.0),
    );

    let sphere1 = Sphere::new(
        Vector3::new(0.0, 0.0, -1.0), 
        0.5,
        Diffuse::new(Vector3::new(0.8, 0.3, 0.3)),
    );

    let sphere2 = Sphere::new(
        Vector3::new(0.0, -100.5, -1.0), 
        100.0,
        Diffuse::new(Vector3::new(0.8, 0.8, 0.0)),
    );

    let sphere3 = Sphere::new(
        Vector3::new(1.0, 0.0, -1.0), 
        0.5,
        Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0),
    );

    let sphere4 = Sphere::new(
        Vector3::new(-1.0, 0.0, -1.0), 
        0.5,
        Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.3),
    );

    let world = Drawables::new_from(vec![
        &sphere1,
        &sphere2,
        &sphere3,
        &sphere4,
    ]);

    let nsamples: u32 = 300;
    
    // Output An Image
    let mut imgbuf = image::ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

        let mut rng = rand::thread_rng();
        let col = (0..nsamples).map(|_| {
            let u = (x as f32 + rng.gen::<f32>()) / WIDTH as f32;
            let v = ((HEIGHT as u32 - y) as f32 + rng.gen::<f32>()) / HEIGHT as f32;
            let r = camera.get_ray(u, v);
            color(&r, &world, 0)
        }).sum::<Vector3<f32>>() / nsamples as f32;

        let r = (255.0 * col[0].sqrt()) as u8;
        let g = (255.0 * col[1].sqrt()) as u8;
        let b = (255.0 * col[2].sqrt()) as u8;
        *pixel = image::Rgb([r, g, b]);
    }
    imgbuf.save_with_format("output.png", image::ImageFormat::Png).unwrap();
}

fn color(r: &ray::Ray, world: &Drawables, depth: u32) -> cgmath::Vector3<f32> {
    if let Some(hit) = world.hit(r, 0.001, f32::MAX) {
        if let Some(scatter) = hit.material.scatter(r, &hit) {
            if depth < MAX_DEPTH {
                return scatter.attenuation.mul_element_wise(color(&scatter.ray, world, depth+1))
            } else {
                return Vector3::new(0.0, 0.0, 0.0)
            }
        } else {
            return Vector3::new(0.0, 0.0, 0.0)
        }
        // return 0.5 * Vector3::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0)
    }
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0-t)*cgmath::Vector3::new(1.0, 1.0, 1.0) + t*cgmath::Vector3::new(0.5, 0.7, 1.0)
}
