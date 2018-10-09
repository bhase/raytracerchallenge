#![macro_use]

// Define new macro-string with name $name assembled from arguments $arg
macro_rules! define_str {
    ($name: ident, $($arg: expr), +)
        =>
        (macro_rules! $name {
                () => (concat!($($arg), +))
        });
}

define_str!(FLOAT, r"[+-]?\d+(?:\.\d+)?");
pub use self::world::MyWorld;

mod world;
