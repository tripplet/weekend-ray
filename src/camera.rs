use std::ops::Range;

use indicatif::{ParallelProgressIterator, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

use crate::color::Color;
use crate::{color, v3d_zero};

use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3d::Vec3d;

#[derive(serde::Deserialize)]
pub struct Camera {
    pub cfg: CameraConfig,

    /// Rendered image width in pixel count
    pub image_width: u16,

    // Rendered image height in pixel count
    pub image_height: u16,

    pub viewport_width: f32,
    pub viewport_height: f32,

    /// Number of random samples for each pixel
    pub samples_per_pixel: u16,

    /// Maximum number of ray bounces into scene
    pub max_depth: u16,

    pixel_delta_u: Vec3d,
    pixel_delta_v: Vec3d,

    /// Location of the (0, 0) pixel
    pixel00_loc: Vec3d,

    /// Defocus disk horizontal radius
    defocus_disk_u: Vec3d,

    /// Defocus disk vertical radius
    defocus_disk_v: Vec3d,
}

#[derive(Clone, serde::Deserialize)]
pub struct CameraConfig {
    /// Point camera is looking from
    pub look_from: Vec3d,

    /// Point camera is looking at
    pub look_at: Vec3d,

    /// Camera-relative "up" direction
    pub vup: Vec3d,

    /// Vertical field-of-view in degrees
    pub vfov: f32,

    /// Ratio of image width over height
    pub aspect_ratio: f32,

    /// Variation angle of rays through each pixel
    pub defocus_angle: f32,

    /// Distance from camera `look_from` point to plane of perfect focus
    pub focus_dist: f32,
}

impl Camera {
    pub fn new(cfg: &CameraConfig, image_width: u16, samples_per_pixel: u16, depth: u16) -> Self {
        // Determine the viewport
        let theta = cfg.vfov.to_radians();
        let viewport_height = (theta / 2.0).tan() * 2.0 * cfg.focus_dist;
        let viewport_width = viewport_height * cfg.aspect_ratio;
        let image_height = (image_width as f32 / cfg.aspect_ratio) as u16;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (cfg.look_from - cfg.look_at).unit_vector();
        let u = cfg.vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        // Calculate the horizontal and vertical delta vectors to the next pixel.
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left = cfg.look_from - (cfg.focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = cfg.focus_dist * (cfg.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            cfg: cfg.clone(),
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            samples_per_pixel,
            max_depth: depth,
            defocus_disk_u,
            defocus_disk_v,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, objects: &Vec<Sphere>) -> Vec<Color> {
        let mut img = vec![v3d_zero!(); self.image_height as usize * self.image_width as usize];

        let sty = ProgressStyle::with_template("[{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-");

        // - Slice the img array into lines for parallel rendering
        // - Create a big, expensive to initialize and slower, but unpredictable RNG.
        //   This is cached and done only once per thread / rayon job.
        img.par_chunks_mut(self.image_width as usize)
            .progress_count(self.image_height as u64)
            .with_style(sty)
            .enumerate()
            .for_each_init(
                SmallRng::from_entropy,
                |rng, (h, line)| self.render_line(h as u16, line, objects, rng),
            );

        img
    }

    fn render_line(&self, h: u16, line: &mut [Vec3d], objects: &Vec<Sphere>, mut thread_rng: &mut impl rand::Rng) {
        let scale = 1.0 / self.samples_per_pixel as f32;

        for w in 0..self.image_width {
            let mut color = v3d_zero!();

            // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
            // the camera defocus disk.
            for _ in 0..self.samples_per_pixel {
                let pixel_center = self.pixel00_loc + (w as f32 * self.pixel_delta_u) + (h as f32 * self.pixel_delta_v);
                let pixel_sample = pixel_center + self.pixel_sample_square(&mut thread_rng);

                let camera_origin = if self.cfg.defocus_angle <= 0.0 {
                    self.cfg.look_from
                } else {
                    self.defocus_disk_sample(&mut thread_rng)
                };

                let r = Ray {
                    origin: camera_origin,
                    direction: pixel_sample - camera_origin,
                };

                color += Camera::ray_color(&r, self.max_depth, &mut thread_rng, objects);
            }

            color.x = Camera::linear_to_gamma(color.x * scale);
            color.y = Camera::linear_to_gamma(color.y * scale);
            color.z = Camera::linear_to_gamma(color.z * scale);

            line[w as usize] = color;
        }
    }

    #[inline(always)]
    fn ray_color(ray: &Ray, depth: u16, rng: &mut impl rand::Rng, world: &Vec<Sphere>) -> Color {
        use crate::hittable::Hittable;

        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return color!(0.0, 0.0, 0.0);
        }

        let mut rng_func = |range: Range<f32>| rng.gen_range(range);

        if let Some(hit) = world.hit(ray, 0.0001, f32::INFINITY) {
            if let Some(scatter) = hit.material.scatter(&mut rng_func, ray, &hit) {
                return scatter.attenuation * Camera::ray_color(&scatter.ray, depth - 1, rng, world);
            }

            return color!(0.0, 0.0, 0.0);
        }

        // blue sky background
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        color!(1.0, 1.0, 1.0) * (1.0 - t) + color!(0.5, 0.7, 1.0) * t
    }

    /// Returns a random point in the camera defocus disk.
    #[inline(always)]
    fn defocus_disk_sample(&self, mut rng: &mut impl rand::Rng) -> Vec3d {
        let p = Vec3d::random_in_unit_circle(&mut rng);
        self.cfg.look_from + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    /// Returns a random point in the square
    #[inline(always)]
    fn pixel_sample_square(&self, rng: &mut impl rand::Rng) -> Vec3d {
        (rng.gen_range(0.0..0.5) * self.pixel_delta_u) + (rng.gen_range(0.0..0.5) * self.pixel_delta_v)
    }

    #[inline(always)]
    fn linear_to_gamma(linear_value: f32) -> f32 {
        linear_value.sqrt()
    }
}
