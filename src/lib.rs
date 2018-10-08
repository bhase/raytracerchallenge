
pub mod tuple {
    const POINT: f64 = 1.0;
    const VECTOR: f64 = 0.0;

    #[derive(PartialEq, Debug)]
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
