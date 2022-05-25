#[macro_export]
macro_rules! count {
    () => (0usize);
    ($a: expr) => (1usize);
    ($($a: expr, $b: expr),*) => (count!($($a),*) << 1usize);
    ($e: expr, $($a: expr, $b: expr),*) => (count!($($a),*) << 1usize | 1usize);
}

#[macro_export]
macro_rules! declare_static_array {
    ($visibility: vis $name: ident, $element_type: ty, [$($a: expr),*]) => {
        $visibility static $name: [$element_type; count!($($a),*)] = [$($a),*];
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count() {
        assert_eq!(0, count!());
        assert_eq!(1, count!(0));
        assert_eq!(2, count!(0, 1));
        assert_eq!(3, count!(0, 1, 2));
        assert_eq!(4, count!(0, 1, 2, 3));
    }

    #[test]
    fn test_declare_static_array() {
        declare_static_array!(
            TEST_ARRAY,
            (u8, &'static str),
            [(1, "x"), (2, "y")]
        );
        assert_eq!(2, TEST_ARRAY.len());
        assert_eq!((1, "x"), TEST_ARRAY[0]);
        assert_eq!((2, "y"), TEST_ARRAY[1]);
    }
}
