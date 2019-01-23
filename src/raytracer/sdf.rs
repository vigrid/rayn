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

pub struct Cube {
    pub p: cgmath::Vector3<f32>,
    pub b: cgmath::Vector3<f32>,
}

impl Cube {
    pub fn new(x: f32, y: f32, z: f32, dx : f32, dy: f32, dz: f32) -> Self {
        Self {
            p: cgmath::Vector3 { x, y, z, },
            b: cgmath::Vector3 { x: dx, y: dy, z: dz },
        }
    }
}

impl Sdf for Cube {
    fn sdf(&self, p: cgmath::Vector3<f32>) -> f32 {
        let d = cgmath::Vector3 {
            x: ((p.x - self.p.x).abs() - self.b.x).max(0.0),
            y: ((p.y - self.p.y).abs() - self.b.y).max(0.0),
            z: ((p.z - self.p.z).abs() - self.b.z).max(0.0),
        };

        d.magnitude()
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

    fn test_cube() -> super::Cube {
        super::Cube {
            p: cgmath::Vector3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            b: cgmath::Vector3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            }
        }
    }

    #[test]
    fn cube_sdf_zero() {
        let cube = test_cube();

        let p = cgmath::Vector3 {
            x: 1.5,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(0.0, cube.sdf(p));
    }

    #[test]
    fn cube_sdf_pos() {
        let cube = test_cube();

        let p = cgmath::Vector3 {
            x: 2.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(0.5, cube.sdf(p));
    }
}
