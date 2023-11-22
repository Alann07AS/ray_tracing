use std::f64::consts::PI;

use crate::ray_tracing::vector::Vector;

use super::color::Color;

//TODO:
//-Ambiant light
//-Source light
//-Spot light

pub trait Light {
    fn color(&self) -> &Color;
    fn intensity(&self, point: &Vector, normal: &Vector) -> f64;
    fn light_apply(&self, base_color: &Color, point: &Vector, normal: &Vector) -> Color {
        let color_light = self.color();
        let intensity = self.intensity(point, normal);
        if intensity == 0. {
            return base_color.clone();
        };
        base_color.change_intensity_whit_color(Color {
            r: color_light.r * intensity,
            g: color_light.g * intensity,
            b: color_light.b * intensity,
        })
    }
}

pub struct AmbiantLight {
    pub color: Color,
    pub intensity: f64,
}

impl Light for AmbiantLight {
    fn color(&self) -> &Color {
        &self.color
    }
    fn intensity(&self, _point: &Vector, _normal: &Vector) -> f64 {
        self.intensity
    }
}

pub struct SourceLight {
    pub color: Color,
    pub intensity: f64,
    pub source: Vector,
}

impl Light for SourceLight {
    fn color(&self) -> &Color {
        &self.color
    }
    fn intensity(&self, point: &Vector, normal: &Vector) -> f64 {
        ((point - &self.source).unit().dot(normal) * -self.intensity).max(0.) 
    }
}

pub struct DirectionelLight {
    pub color: Color,
    pub intensity: f64,
    pub source: Vector,
    pub direction: Vector,
}

impl Light for DirectionelLight {
    fn color(&self) -> &Color {
        &self.color
    }
    fn intensity(&self, point: &Vector, normal: &Vector) -> f64 {
        let v = (point - &self.source).unit();
        (v.dot(normal).max(0.) * (&v).dot(&self.direction).max(0.)) * self.intensity
    }
}

pub struct SpotLight {
    pub color: Color,
    pub intensity: f64,
    pub source: Vector,
    pub direction: Vector,
    pub angle: f64,
}

impl Light for SpotLight {
    fn color(&self) -> &Color {
        &self.color
    }
    fn intensity(&self, point: &Vector, normal: &Vector) -> f64 {
        let v = (point - &self.source).unit();
        if self.direction.dot(&v) * PI > self.angle {
            (v.dot(normal) * -self.intensity).max(0.)
        } else {
            0.
        }
    }
}

impl SpotLight {
    pub fn new() -> Self {
        SpotLight {
            color: Color::WHITE,
            intensity: 100.,
            source: Vector {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            direction: Vector {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            angle: (180_f64 - 35_f64).to_radians(), //DEFAULT 35° / 2
        }
    }
    pub fn angle(&mut self, degres_angle: f64) {
        eprintln!("(degres_angle / 2.).to_radians() = {:?}", (degres_angle / 2.).to_radians());
        self.angle = (180_f64 - degres_angle ).to_radians()
    }
}

pub struct PlaneLight {
    pub color: Color,
    pub intensity: f64,
    pub source: Vector,
    pub direction: Vector,
}

impl Light for PlaneLight {
    fn color(&self) -> &Color {
        &self.color
    }
    fn intensity(&self, point: &Vector, normal: &Vector) -> f64 {
        let p_normal = &self.direction * -1.;

        // const A: f64 = -1.; //p_normal.dot(&ray);
        let b = p_normal.dot(&(&self.source - point));
        // let t = b / A;
        // if t > 0. {
        if b > 0. {
            p_normal.dot(normal).max(0.) * self.intensity
        } else {
            0.
        }
    }
}
