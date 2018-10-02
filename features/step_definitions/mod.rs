use support;
use raytracer_challenge::tuple;

// Any type that implements cucumber_rust::World + Default can be the world
steps!(support::MyWorld => {
    given regex r"a := tuple\(([+-]?\d+\.\d+), ([+-]?\d+\.\d+), ([+-]?\d+\.\d+), ([+-]?\d+\.\d+)\)"
        (f64, f64, f64, f64) |world, x, y, z, w, step| {
            world.tuple = tuple::Tuple { x, y, z, w };
    };
    then regex r"a\.([xyzw]) = ([+-]?\d+\.\d+)" (char, f64) |world, part, value, step| {
        let v = match part {
            'x' => world.tuple.x,
            'y' => world.tuple.y,
            'z' => world.tuple.z,
            'w' => world.tuple.w,
             _  => 0.0,
        };
        assert_eq!(value, v);
    };
    then regex r"a is (.*)a point" |world, not_a, step| {
        let expected_result = match not_a[1].as_str() {
            "" => true,
            _ => false,
        };
        assert_eq!(world.tuple.is_point(), expected_result);
    };
    then regex r"a is (.*)a vector" |world, not_a, step| {
        let expected_result = match not_a[1].as_str() {
            "" => true,
            _ => false,
        };
        assert_eq!(world.tuple.is_vector(), expected_result);
    };
});

