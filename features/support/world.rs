use cucumber_rust;
use std;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    pub foo: String
}

impl cucumber_rust::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld {
            foo: "a default string".to_string()
        }
    }
}

