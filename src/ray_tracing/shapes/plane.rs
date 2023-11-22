use super::{
    super::{ray::Ray, vector::Vector},
    color::Color,
    shapes::Shape,
};

pub struct Plane {
    pub origin: Vector,
    pub direction: Vector,
    pub color: Color,
}

impl Shape for Plane {
    fn color(&self) -> &Color {
        &self.color
    }
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let a = self.direction.dot(&ray.direction);
        let b = self.direction.dot(&(&self.origin - &ray.origin));

        let t = b / a;

        if t > 0. {
            Some(t.abs())
        } else {
            None
        }
    }
    fn normal(&self, _hit_point: &Vector) -> Vector {
        // (&self.center - hit_point).unit()
        self.direction.clone()
    }
}
