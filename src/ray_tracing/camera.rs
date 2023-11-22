use super::vector::{Vector, VEC_NULL};

pub enum Resolution {
    QuatreTiers,
    SinzeNeuvieme,
}

pub struct Camera {
    pub position: Vector,
    orientation: Vector,
    fov: f64,
    focal: f64,
    aspect_ratio: f64,
    height: f64,
    width: f64,
    pub corner: Vector,
    pub vec_height: Vector,
    pub vec_width: Vector,
    resolution_x: f64,
    resolution_y: f64,
}


const DEFAULT_UP: Vector = Vector {x: 0., y: 0., z: -1.};
impl Camera {
    pub fn new(x: f64, y: f64) -> Self {
        let ac = 4. / 3.;
        let h = (90_f64.to_radians() / 2.).tan() * 2.;
        let mut camera = Self {
            position: VEC_NULL,
            orientation: DEFAULT_UP,
            fov: 90.,
            focal: 1.,
            aspect_ratio: ac,
            height: h,
            width: h * ac,
            corner: VEC_NULL,
            vec_height: VEC_NULL,
            vec_width: VEC_NULL,
            resolution_x: x,
            resolution_y: y,
        };
        camera.update();
        camera
    }
    pub fn update(&mut self) {
        // self.width = self.height * self.aspect_ratio;
        // self.vec_height = (self.orientation * Vector::new(0., 1., 0.)).unit();
        // self.vec_width = (self.orientation * self.vec_height).unit();
        // self.corner = self.vec_width * -(self.width / 2.) + self.vec_height * -(self.height / 2.)
        println!( "{} {} ", self.height, self.width);
        let u1 = &(&self.orientation * &Vector::new(0., 1., 0.)).unit()*(self.width/self.resolution_x);
        let u2 = &(&self.orientation * &u1).unit()*(self.height/self.resolution_y);
        let rini = &(&u1 * -(self.resolution_x/2.)) - &(&u2 * (self.resolution_y/2.));
        println!( "{:?} {:?} {:?} ", u1, u2, rini);
        self.corner = &(&self.position + &(&self.orientation * self.focal)) + &rini;
        self.vec_width = u1;
        self.vec_height = u2;

    }
    pub fn position(&mut self, position: Vector) {
        self.position = position;
        self.update()
    }
    pub fn orientation(&mut self, orientation: Vector) {
        self.orientation = orientation;
        self.update()
    }
    pub fn fov(&mut self, fov: f64) {
        self.fov = fov;
        self.update()
    }
    pub fn focal(&mut self, focal: f64) {
        self.focal = focal;
        self.update()
    }
}
