use support;
use raytracer_challenge::tuple;

define_str!(TUPLE,  r"tuple\((", FLOAT!(), "), (", FLOAT!(), "), (", FLOAT!(), "), (", FLOAT!(), r")\)");
define_str!(POINT,  r"point\((", FLOAT!(), "), (", FLOAT!(), "), (", FLOAT!(), r")\)");
define_str!(VECTOR, r"vector\((", FLOAT!(), "), (", FLOAT!(), "), (", FLOAT!(), r")\)");

define_str!(GIVEN_TUPLE_STEP, r"a(\d?) := ", TUPLE!());
const GIVEN_TUPLE: &'static str = GIVEN_TUPLE_STEP!();

define_str!(GIVEN_POINT_STEP, r"p(\d?) := ", POINT!());
const GIVEN_POINT: &'static str = GIVEN_POINT_STEP!();

define_str!(THEN_TUPLE_ELEMENT_STEP, r"a\.([xyzw]) = (", FLOAT!(), ")");
const THEN_TUPLE_ELEMENT: &'static str = THEN_TUPLE_ELEMENT_STEP!();

define_str!(THEN_TUPLE_VECTORPOINT_STEP, r"([pv]) = ", TUPLE!());
const THEN_TUPLE_VECTORPOINT: &'static str = THEN_TUPLE_VECTORPOINT_STEP!();

define_str!(GIVEN_VECTOR_STEP, r"(?:zero|v)(\d?) := ", VECTOR!());
const GIVEN_VECTOR: &'static str = GIVEN_VECTOR_STEP!();

define_str!(ADD_RESULT_STEP, r"a1 \+ a2 = ", TUPLE!());
const ADD_RESULT: &'static str = ADD_RESULT_STEP!();

define_str!(SUB_POINT_FROM_POINT_STEP, r"p1 - p2 = ", VECTOR!());
const SUB_POINT_FROM_POINT: &'static str = SUB_POINT_FROM_POINT_STEP!();

define_str!(SUB_POINT_FROM_VECTOR_STEP, r"p - v = ", POINT!());
const SUB_POINT_FROM_VECTOR: &'static str = SUB_POINT_FROM_VECTOR_STEP!();

define_str!(SUB_VECTOR_FROM_VECTOR_STEP, r"(?:zero|v1) - v2 = ", VECTOR!());
const SUB_VECTOR_FROM_VECTOR: &'static str = SUB_VECTOR_FROM_VECTOR_STEP!();

define_str!(NEGATE_TUPLE_STEP, r"-a = ", TUPLE!());
const NEGATE_TUPLE: &'static str = NEGATE_TUPLE_STEP!();

define_str!(MULTIPLY_SCALAR_TUPLE_STEP, r"a \* (", FLOAT!(), ") = ", TUPLE!());
const MULTIPLY_SCALAR_TUPLE: &'static str = MULTIPLY_SCALAR_TUPLE_STEP!();

// Any type that implements cucumber_rust::World + Default can be the world
steps!(support::MyWorld => {
    given regex GIVEN_TUPLE
        (String, f64, f64, f64, f64) |world, m, x, y, z, w, _step| {
            match m.as_str() {
                "" => world.t = tuple::Tuple { x, y, z, w },
                "1" => world.p1 = tuple::Tuple { x, y, z, w },
                "2" => world.p2 = tuple::Tuple { x, y, z, w },
                _ => assert_eq!(false, true)
            };
    };
    then regex THEN_TUPLE_ELEMENT (char, f64) |world, part, value, _step| {
        let v = match part {
            'x' => world.t.x,
            'y' => world.t.y,
            'z' => world.t.z,
            'w' => world.t.w,
             _  => 0.0,
        };
        assert_eq!(value, v);
    };
    then regex r"a is (.*)a point" |world, not_a, _step| {
        let expected_result = match not_a[1].as_str() {
            "" => true,
            _ => false,
        };
        assert_eq!(world.t.is_point(), expected_result);
    };
    then regex r"a is (.*)a vector" |world, not_a, _step| {
        let expected_result = match not_a[1].as_str() {
            "" => true,
            _ => false,
        };
        assert_eq!(world.t.is_vector(), expected_result);
    };

    given regex GIVEN_POINT
        (String, f64, f64, f64) |world, m, x, y, z, _step| {
            match m.as_str() {
                "" | "1" => world.p1 = tuple::point(x, y, z),
                "2" => world.p2 = tuple::point(x, y, z),
                _ => assert_eq!(false, true)
            };
    };

    then regex THEN_TUPLE_VECTORPOINT
        (String, f64, f64, f64, f64) |world, pv, x, y, z, w, _step| {
            match pv.as_str() {
                "v" => assert_eq!(world.v1, tuple::Tuple { x, y, z, w }),
                _ => assert_eq!(world.p1, tuple::Tuple { x, y, z, w })
            };
    };

    given regex GIVEN_VECTOR
        (String, f64, f64, f64) |world, m, x, y, z, _step| {
            match m.as_str() {
                "" | "1" => world.v1 = tuple::vector(x, y, z),
                "2" => world.v2 = tuple::vector(x, y, z),
                _ => assert_eq!(false, true)
            };
    };

    then regex ADD_RESULT
        (f64, f64, f64, f64) |world, x, y, z, w, _step| {
        let result = &world.p1 + &world.p2;
        assert_eq!(result, tuple::Tuple { x, y, z, w });
        assert_eq!(world.p1 + world.p2, tuple::Tuple { x, y, z, w });
    };

    then regex SUB_POINT_FROM_POINT
        (f64, f64, f64) |world, x, y, z, _step| {
        let result = &world.p1 - &world.p2;
        assert_eq!(result, tuple::vector(x, y, z));
    };

    then regex SUB_POINT_FROM_VECTOR
        (f64, f64, f64) |world, x, y, z, _step| {
        // as reference
        let result = &world.p1 - &world.v1;
        assert_eq!(result, tuple::point(x, y, z));
        // and as copy
        assert_eq!(world.p1 - world.v1, tuple::point(x, y, z));
    };

    then regex SUB_VECTOR_FROM_VECTOR
        (f64, f64, f64) |world, x, y, z, _step| {
        let result = &world.v1 - &world.v2;
        assert_eq!(result, tuple::vector(x, y, z));
    };

    then regex NEGATE_TUPLE
        (f64, f64, f64, f64) |world, x, y, z, w, _step| {
        // as reference
        assert_eq!(-&world.t, tuple::Tuple { x, y, z, w });
        // and as copy
        assert_eq!(-world.t, tuple::Tuple { x, y, z, w });
        // and as copy
    };

    then regex MULTIPLY_SCALAR_TUPLE
        (f64, f64, f64, f64, f64) |world, s, x, y, z, w, _step| {
        assert_eq!(&world.t * s, tuple::Tuple { x, y, z, w });
        assert_eq!(world.t * s, tuple::Tuple { x, y, z, w });
     };
});

