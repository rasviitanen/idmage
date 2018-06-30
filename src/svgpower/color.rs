#[allow(unused_macros)]
macro_rules! RGBA {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {{
        format!("rgba({},{},{},{})", 
            stringify!($r), 
            stringify!($g), 
            stringify!($b),
            stringify!($a),
        )
    }};
}
