use crate::geometry::Point3;
use crate::geometry::Vec3;

#[derive(Default, Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}
