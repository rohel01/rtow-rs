use crate::data::Point3;
use crate::data::Vec3;

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    // TODO Determiner si &self est benefique
    pub fn at(self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}
