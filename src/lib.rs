#[macro_export]
macro_rules! const_format {
    (a0 = $a0:expr; $($tail:tt),+) => {concat!(const_format!($a0; $($tail),*))};
    ($a0:expr; a0) => {$a0};
    ($a0:expr; a0, $($tail:tt),+) => {concat!($a0, const_format!($a0; $($tail),*))};
    ($a0:expr; $e:expr) => {$e};
    ($a0:expr; $e:expr, $($tail:tt),+) => {concat!($e, const_format!($a0; $($tail),*))};

    (a0 = $a0:expr, a1 = $a1:expr; $($tail:tt),+) => {concat!(const_format!($a0, $a1; $($tail),*))};
    ($a0:expr, $a1:expr; a0) => {$a0};
    ($a0:expr, $a1:expr; a0, $($tail:tt),+) => {concat!($a0, const_format!($a0, $a1; $($tail),*))};
    ($a0:expr, $a1:expr; a1) => {$a1};
    ($a0:expr, $a1:expr; a1, $($tail:tt),+) => {concat!($a1, const_format!($a0, $a1; $($tail),*))};
    ($a0:expr, $a1:expr; $e:expr) => {$e};
    ($a0:expr, $a1:expr; $e:expr, $($tail:tt),+) => {concat!($e, const_format!($a0, $a1; $($tail),*))};

    (a0 = $a0:expr, a1 = $a1:expr, a2 = $a2:expr; $($tail:tt),+) => {concat!(const_format!($a0, $a1, $a2; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr; a0) => {$a0};
    ($a0:expr, $a1:expr, $a2:expr; a0, $($tail:tt),+) => {concat!($a0, const_format!($a0, $a1, $a2; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr; a1) => {$a1};
    ($a0:expr, $a1:expr, $a2:expr; a1, $($tail:tt),+) => {concat!($a1, const_format!($a0, $a1, $a2; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr; a2) => {$a2};
    ($a0:expr, $a1:expr, $a2:expr; a2, $($tail:tt),+) => {concat!($a2, const_format!($a0, $a1, $a2; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr; $e:expr) => {$e};
    ($a0:expr, $a1:expr, $a2:expr; $e:expr, $($tail:tt),+) => {concat!($e, const_format!($a0, $a1, $a2; $($tail),*))};

    (a0 = $a0:expr, a1 = $a1:expr, a2 = $a2:expr, a3 = $a3:expr; $($tail:tt),+) => {concat!(const_format!($a0, $a1, $a2, $a3; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr; a0) => {$a0};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr; a0, $($tail:tt),+) => {concat!($a0, const_format!($a0, $a1, $a2, $a3; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr; a1) => {$a1};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr; a1, $($tail:tt),+) => {concat!($a1, const_format!($a0, $a1, $a2, $a3; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr; a2) => {$a2};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr; a2, $($tail:tt),+) => {concat!($a2, const_format!($a0, $a1, $a2, $a3; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr; a3) => {$a3};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr; a3, $($tail:tt),+) => {concat!($a3, const_format!($a0, $a1, $a2, $a3; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr; $e:expr) => {$e};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr; $e:expr, $($tail:tt),+) => {concat!($e, const_format!($a0, $a1, $a2, $a3; $($tail),*))};

    (a0 = $a0:expr, a1 = $a1:expr, a2 = $a2:expr, a3 = $a3:expr, a4 = $a4:expr; $($tail:tt),+) => {concat!(const_format!($a0, $a1, $a2, $a3, $a4; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; a0) => {$a0};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; a0, $($tail:tt),+) => {concat!($a0, const_format!($a0, $a1, $a2, $a3, $a4; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; a1) => {$a1};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; a1, $($tail:tt),+) => {concat!($a1, const_format!($a0, $a1, $a2, $a3, $a4; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; a2) => {$a2};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; a2, $($tail:tt),+) => {concat!($a2, const_format!($a0, $a1, $a2, $a3, $a4; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; a3) => {$a3};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; a3, $($tail:tt),+) => {concat!($a3, const_format!($a0, $a1, $a2, $a3, $a4; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; a4) => {$a4};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; a4, $($tail:tt),+) => {concat!($a4, const_format!($a0, $a1, $a2, $a3, $a4; $($tail),*))};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; $e:expr) => {$e};
    ($a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr; $e:expr, $($tail:tt),+) => {concat!($e, const_format!($a0, $a1, $a2, $a3, $a4; $($tail),*))};
}
