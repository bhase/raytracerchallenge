
pub mod tuple {
    pub struct tuple {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub w: f64,
    }

    impl tuple {
        pub fn is_point(&self) -> bool {
            self.w == 1.0
        }
        pub fn is_vector(&self) -> bool {
            !self.is_point()
        }
    }
}
