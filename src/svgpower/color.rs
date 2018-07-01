#[allow(unused_macros)]
macro_rules! RGBA {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {{
        format!("rgba({},{},{},{})", 
            $r, 
            $g, 
            $b,
            $a,
        )
    }};
}

#[allow(unused_macros)]
macro_rules! ATTR {
    ($item:expr, $value:expr) => {{
        format!("{}=\"{}\"", $item, $value)
    }};
}