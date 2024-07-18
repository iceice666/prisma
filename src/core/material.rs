use crate::core::{Ray, RayIntersection};
use palette::LinSrgb;
use rand::rngs::ThreadRng;

pub trait Material {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        ray: &Ray,
        intersection: &RayIntersection,
    ) -> Option<(Ray, LinSrgb<f64>)>;
}