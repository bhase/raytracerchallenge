
pub mod tuple {
    use std::ops::{Add, Sub};
    const POINT: f64 = 1.0;
    const VECTOR: f64 = 0.0;

    #[derive(PartialEq, PartialOrd, Debug)]
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
