#[macro_use]
use support;
use raytracer_challenge::tuple;

define_str!(GIVEN_TUPLE_STEP, r"a := tuple\((", FLOAT!(), "), (", FLOAT!(), "), (", FLOAT!(), "), (", FLOAT!(), r")\)");

define_str!(GIVEN_POINT_STEP, r"p := point\((", FLOAT!(), "), (", FLOAT!(), "), (", FLOAT!(), r")\)");

define_str!(THEN_TUPLE_ELEMENT_STEP, r"a\.([xyzw]) = (", FLOAT!(), ")");

const GIVEN_TUPLE: &'static str = GIVEN_TUPLE_STEP!();
const GIVEN_POINT: &'static str = GIVEN_POINT_STEP!();
const THEN_TUPLE_ELEMENT: &'static str = THEN_TUPLE_ELEMENT_STEP!();

// Any type that implements cucumber_rust::World + Default can be the world
steps!(support::MyWorld => {
    given regex GIVEN_TUPLE
        (f64, f64, f64, f64) |world, x, y, z, w, step| {
            world.tuple = tuple::Tuple { x, y, z, w };
    };
    then regex THEN_TUPLE_ELEMENT (char, f64) |world, part, value, step| {
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

    given regex GIVEN_POINT
        (f64, f64, f64) |world, x, y, z, step| {
    };

});

