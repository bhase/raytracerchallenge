extern crate raytracer_challenge;

#[macro_use]
extern crate cucumber_rust;

#[path = "../features/support/mod.rs"]
mod support;

#[path = "../features/step_definitions/mod.rs"]
mod step_definitions;

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {
    
}

cucumber! {
    features: "./features", // Path to our feature files
    world: support::MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        step_definitions::steps // the `steps!` macro creates a `steps` function in a module
    ],
    setup: setup, // Optional; called once before everything
    before: &[
        a_before_fn // Optional; called before each scenario
    ], 
    after: &[
        an_after_fn // Optional; called after each scenario
    ] 
}
