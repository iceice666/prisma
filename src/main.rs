mod cli;
mod ray;

use clap::Parser;
use cli::{Cli, Size};
use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use nalgebra::{Point3, Vector3};
use ray::Ray;

fn hit_sphere(center: Point3<f64>, radius: f64, ray: &Ray) -> bool {
    let a = ray.dir.magnitude_squared();
    let b = (-2.0 * ray.dir).dot(&(center - ray.orig));
    let c = (center - ray.orig).magnitude_squared() - radius * radius;
    return b * b - 4.0 * a * c > 0.0;
}

fn ray_color(ray: &Ray) -> Vector3<f64> {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vector3::new(1.0, 0.0, 0.0);
    }

    let dir = ray.dir.normalize();
    let a = 0.5 * (dir.y + 1.0);
    (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
}

fn main() {
    let cli = Cli::parse();
    let Size { width, height } = cli.size;

    let mut image = RgbImage::new(width, height);
    let progress_bar = ProgressBar::new(height as u64);

    let camera_pos = Point3::new(0.0, 0.0, 0.0);
    let camera_focal_len = 1.0;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f64 / height as f64);

    let pixel_delta_x = Vector3::new(viewport_width, 0.0, 0.0) / width as f64;
    let pixel_delta_y = Vector3::new(0.0, -viewport_height, 0.0) / height as f64;
    let pixel_pos_orig = camera_pos + Vector3::new(0.0, 0.0, -camera_focal_len)
        - width as f64 / 2.0 * pixel_delta_x
        - height as f64 / 2.0 * pixel_delta_y;

    for y in 0..height {
        for x in 0..width {
            let pixel_pos = pixel_pos_orig + x as f64 * pixel_delta_x + y as f64 * pixel_delta_y;
            let ray = Ray::new(camera_pos, pixel_pos - camera_pos);
            let color = ray_color(&ray);

            let r = (255.999 * color.x) as u8;
            let g = (255.999 * color.y) as u8;
            let b = (255.999 * color.z) as u8;

            image.put_pixel(x, y, Rgb([r, g, b]));
        }
        progress_bar.inc(1);
    }

    image.save(cli.output).unwrap();
    progress_bar.finish();
}
