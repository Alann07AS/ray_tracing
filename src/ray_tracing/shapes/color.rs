#[derive(Debug, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    const DARK_COLOR: f64 = 0.01;
    pub fn dark_color(r: f64, g: f64, b: f64) -> Color {
        let r = r * Color::DARK_COLOR; //f64::MIN_POSITIVE;
        let g = g * Color::DARK_COLOR; //f64::MIN_POSITIVE;
        let b = b * Color::DARK_COLOR; //f64::MIN_POSITIVE;
        Color { r, g, b }
    }
    pub fn dark_from_color(color: Color) -> Color {
        let r = color.r * Color::DARK_COLOR; //f64::MIN_POSITIVE;
        let g = color.g * Color::DARK_COLOR; //f64::MIN_POSITIVE;
        let b = color.b * Color::DARK_COLOR; //f64::MIN_POSITIVE;
        Color { r, g, b }
    }
    pub fn sclaled_255(&self) -> Color {
        const M: f64 = 255.;
        Color {
            r: (self.r * M).trunc(),
            g: (self.g * M).trunc(),
            b: (self.b * M).trunc(),
        }
    }
    pub fn change_intensity_whit_color(&self, light_intensity: Color) -> Color {
        Color {
            r: (self.r * light_intensity.r),
            g: (self.g * light_intensity.g),
            b: (self.b * light_intensity.b),
        }
    }
    pub fn blend(&mut self, other: &Color) -> &mut Self {
        self.r += other.r;
        // self.r /= 2.;
        self.g += other.g;
        // self.g /= 2.;
        self.b += other.b;
        // self.b /= 2.;
        self
    }

    pub const WHITE: Color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
    };

    pub const BLACK: Color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
    };

    pub const RED: Color = Color {
        r: 1.,
        g: 0.,
        b: 0.,
    };

    pub const GREEN: Color = Color {
        r: 0.,
        g: 1.,
        b: 0.,
    };

    pub const BLUE: Color = Color {
        r: 0.,
        g: 0.,
        b: 1.,
    };
}
