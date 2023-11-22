use super::{super::{vector::Vector, ray::Ray}, color::Color};
pub trait Shape {
	fn color(&self) -> &Color;
	fn intersect(&self, ray: &Ray) -> Option<f64>;
	fn normal(&self, hit_point: &Vector) -> Vector;
}
