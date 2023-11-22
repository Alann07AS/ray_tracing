use super::{
    super::{ray::Ray, vector::Vector},
    shapes::Shape, color::Color,
};

pub struct Sphere {
    pub center: Vector,
    pub rayon: f64,
    pub color: Color,
}

impl Shape for Sphere {
    fn color(&self) -> &Color {
        &self.color
    }
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let rs0 = &ray.origin - &self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = (rs0).dot(&ray.direction);
        let c = rs0.dot(&rs0) - self.rayon.powf(2.);
        let discriminant = b.powf(2.) - a * c;
        if discriminant > 0. {
            let rd = discriminant.sqrt();
            let t1 = (-b + rd) * a;
            let t2 = (-b - rd) * a;
            let t = t1.min(t2);
            // eprintln!("t1, t2 = {:?}, {:?}", t1, t2);
            if t < 0. {
                None
            } else {
                Some(t)
            }
        } else {
            None
        }
    }
    fn normal(&self, hit_point: &Vector) -> Vector {
        (hit_point - &self.center).unit()
    }
}
