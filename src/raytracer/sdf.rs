extern crate cgmath;

use cgmath::InnerSpace;

pub trait Sdf : Sync {
    fn sdf(&self, p: cgmath::Vector3<f32>) -> f32;
}

pub struct Sphere {
    pub p: cgmath::Vector3<f32>,
    pub r: f32,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Self {
        Self {
            p: cgmath::Vector3 { x, y, z },
            r,
        }
    }
}

impl Sdf for Sphere {
    fn sdf(&self, p: cgmath::Vector3<f32>) -> f32 {
        (self.p - p).magnitude() - self.r
    }
}

pub struct Plane {
    pub n: cgmath::Vector3<f32>,
    pub d: f32,
}

impl Plane {
    pub fn new(x: f32, y: f32, z: f32, d: f32) -> Self {
        Self {
            n: cgmath::Vector3 { x, y, z },
            d,
        }
    }
}

impl Sdf for Plane {
    fn sdf(&self, p: cgmath::Vector3<f32>) -> f32 {
        cgmath::dot(p, self.n) + self.d
    }
}

mod tests {
    use crate::raytracer::sdf::Sdf;

    fn test_sphere() -> super::Sphere {
        super::Sphere {
            p: cgmath::Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            r: 1.0,
        }
    }

    #[test]
    fn shpere_sdf_neg() {
        let sphere = test_sphere();

        let p = cgmath::Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(-1.0, sphere.sdf(p))
    }

    #[test]
    fn sphere_sdf_zero() {
        let sphere = test_sphere();

        let p = cgmath::Vector3 {
            x: 0.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(0.0, sphere.sdf(p));
    }

    #[test]
    fn sphere_sdf_pos() {
        let sphere = test_sphere();

        let p = cgmath::Vector3 {
            x: -1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(1.0, sphere.sdf(p));
    }

    fn test_plane() -> super::Plane {
        super::Plane {
            n: cgmath::Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            d: 0.0,
        }
    }

    #[test]
    fn plane_sdf_neg() {
        let plane = test_plane();

        let p = cgmath::Vector3 {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        };

        assert_eq!(-1.0, plane.sdf(p));
    }

    #[test]
    fn plane_sdf_zero() {
        let plane = test_plane();

        let p = cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        assert_eq!(0.0, plane.sdf(p));
    }

    #[test]
    fn plane_sdf_pos() {
        let plane = test_plane();

        let p = cgmath::Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        assert_eq!(1.0, plane.sdf(p));
    }
}
