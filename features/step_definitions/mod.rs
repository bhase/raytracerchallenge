use support;
// Any type that implements cucumber_rust::World + Default can be the world
steps!(support::MyWorld => {
    given "I am trying out Cucumber" |world, _step| {
        world.foo = "Some string".to_string();
        // Set up your context in given steps
    };
});

