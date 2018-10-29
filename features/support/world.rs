use cucumber_rust;
use std;
use raytracer_challenge::tuple;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    pub t: tuple::Tuple,
    pub p1: tuple::Tuple,
    pub p2: tuple::Tuple,
    pub v: tuple::Tuple,
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld {
            t: tuple::Tuple { x:0.0, y:0.0, z:0.0, w:1.0 },
            p1: tuple::Tuple { x:0.0, y:0.0, z:0.0, w:1.0 },
            p2: tuple::Tuple { x:0.0, y:0.0, z:0.0, w:1.0 },
            v: tuple::Tuple { x:0.0, y:0.0, z:0.0, w:1.0 }
        }
    }
}

