use support;
use raytracer_challenge::tuple;

// Any type that implements cucumber_rust::World + Default can be the world
steps!(support::MyWorld => {
    given regex r"a := tuple\(([+-]?\d+\.\d+), ([+-]?\d+\.\d+), ([+-]?\d+\.\d+), ([+-]?\d+\.\d+)\)"
        (f64, f64, f64, f64) |world, x, y, z, w, step| {
            world.basic_tuple = tuple::tuple { x, y, z, w };
    };
    then regex r"a\.([xyzw]) = ([+-]?\d+\.\d+)" (char, f64) |world, part, value, step| {
        let v = match part {
            'x' => world.basic_tuple.x,
            'y' => world.basic_tuple.y,
            'z' => world.basic_tuple.z,
            'w' => world.basic_tuple.w,
             _  => 0.0,
        };
        assert_eq!(value, v);
    };
    then regex r"a is (.*)a point" |world, not_a, step| {
        let expected_result = match not_a[1].as_str() {
            "" => true,
            _ => false,
        };
        assert_eq!(world.basic_tuple.is_point(), expected_result);
    };
    then regex r"a is (.*)a vector" |world, not_a, step| {
        let expected_result = match not_a[1].as_str() {
            "" => true,
            _ => false,
        };
        assert_eq!(world.basic_tuple.is_vector(), expected_result);
    };
});

