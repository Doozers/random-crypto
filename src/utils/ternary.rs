#[macro_export]
macro_rules! ternary {
    ($cond:expr, $true:expr, $false:expr) => {
        if $cond {
            $true
        } else {
            $false
        }
    };
}
