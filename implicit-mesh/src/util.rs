#[macro_export]
macro_rules! assert_similiar {
    ($left:expr, $right:expr) => {
        let difference = $right - $left;

        // TODO: I think we should have an epsilon value somewhere
        if difference.abs() > 0.001 {
            let message = format!(
                "assert_similiar failed:\nleft: {}\nright: {}",
                $left, $right
            );
            panic!(message);
        }
    };
}

#[macro_export]
macro_rules! debug_assert_similiar {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_similiar!($($arg)*); })
}
