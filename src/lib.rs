
pub mod tuple {
    use std::ops::{Add, Sub, Neg, Mul, Div};
    const POINT: f64 = 1.0;
    const VECTOR: f64 = 0.0;

    #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
    pub struct Tuple {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }

    impl Tuple {
        pub fn is_point(&self) -> bool {
            self.w == POINT
        }
        pub fn is_vector(&self) -> bool {
            !self.is_point()
        }

        pub fn magnitude(&self) -> f64 {
            let sum = self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2);
            sum.sqrt()
        }
    }

    impl<'a, 'b> Add<&'b Tuple> for &'a Tuple {
        type Output = Tuple;

        fn add(self, other: &'b Tuple) -> Tuple {
            Tuple {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
                w: self.w + other.w,
            }
        }
    }
    impl Add<Tuple> for Tuple {
        type Output = Tuple;

        fn add(self, other: Tuple) -> Tuple {
            Tuple {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
                w: self.w + other.w,
            }
        }
    }

    impl<'a, 'b> Sub<&'b Tuple> for &'a Tuple {
        type Output = Tuple;

        fn sub(self, other: &'b Tuple) -> Tuple {
            Tuple {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
                w: self.w - other.w,
            }
        }
    }
    impl Sub<Tuple> for Tuple {
        type Output = Tuple;

        fn sub(self, other: Tuple) -> Tuple {
            Tuple {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
                w: self.w - other.w,
            }
        }
    }

    impl<'a> Neg for &'a Tuple {
        type Output = Tuple;

        fn neg(self) -> Tuple {
            Tuple {
                x: -self.x,
                y: -self.y,
                z: -self.z,
                w: -self.w,
            }
        }
    }
    impl Neg for Tuple {
        type Output = Tuple;

        fn neg(self) -> Tuple {
            Tuple {
                x: -self.x,
                y: -self.y,
                z: -self.z,
                w: -self.w,
            }
        }
    }

    impl<'a> Mul<f64> for &'a Tuple {
        type Output = Tuple;

        fn mul(self, other: f64) -> Tuple {
            Tuple {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
                w: self.w * other,
            }
        }
    }
    impl Mul<f64> for Tuple {
        type Output = Tuple;

        fn mul(self, other: f64) -> Tuple {
            Tuple {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
                w: self.w * other,
            }
        }
    }

    impl<'a> Div<f64> for &'a Tuple {
        type Output = Tuple;

        fn div(self, other: f64) -> Tuple {
            Tuple {
                x: self.x / other,
                y: self.y / other,
                z: self.z / other,
                w: self.w / other,
            }
        }
    }
    impl Div<f64> for Tuple {
        type Output = Tuple;

        fn div(self, other: f64) -> Tuple {
            Tuple {
                x: self.x / other,
                y: self.y / other,
                z: self.z / other,
                w: self.w / other,
            }
        }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: POINT
        }
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: VECTOR
        }
    }
}
