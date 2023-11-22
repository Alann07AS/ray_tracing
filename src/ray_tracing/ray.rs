use super::vector::{Vector, VEC_NULL};
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
    x: f64,
    vec_x: Vector,
    y: f64,
    vec_y: Vector,
    width: Vector,
    height: Vector,
    source_point: Vector,
    corner: Vector,
}

impl Ray {
    pub fn new(source_point: Vector, corner: Vector, width: Vector, height: Vector) -> Ray {
        Ray {
            x: 0.,
            vec_x: VEC_NULL,
            y: 0.,
            vec_y: VEC_NULL,
            origin: VEC_NULL,
            direction: VEC_NULL,
            width,
            height,
            source_point,
            corner
        }
    }
    fn update(&mut self) {
        let r0 = &(&self.vec_x + &self.vec_y) + &self.corner;
        self.direction = (&r0 - &self.source_point).unit();
        self.origin = r0;
    }
    pub fn at(&self, dist: f64) -> Vector {
        &self.origin + &(&self.direction * dist)
    }
    pub fn set_x(&mut self, new: f64) {
        self.x = new;
        self.vec_x = &self.width * new;
        self.update()
    }
    pub fn set_y(&mut self, new: f64) {
        self.y = new;
        self.vec_y = &self.height * new;
        self.update()
    }
}
