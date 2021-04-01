use std::env;
use std::fs::File;
use std::io::*;
use std::path::*;

fn main() {
    let utcvec = vec![
        (14, 0),
        (13, 0),
        (12, 45),
        (12, 0),
        (11, 0),
        (10, 30),
        (10, 0),
        (9, 30),
        (9, 0),
        (8, 45),
        (8, 0),
        (7, 0),
        (6, 30),
        (6, 0),
        (5, 45),
        (5, 30),
        (5, 0),
        (4, 30),
        (4, 0),
        (3, 30),
        (3, 0),
        (2, 0),
        (1, 0),
        (0, 0),
        (-1, 0),
        (-2, 0),
        (-3, 0),
        (-3, 30),
        (-4, 0),
        (-5, 0),
        (-6, 0),
        (-7, 0),
        (-8, 0),
        (-9, 0),
        (-9, 30),
        (-10, 0),
        (-11, 0),
        (-12, 0),
    ];
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("known_timezones.rs");
    let mut out_file = File::create(out_path).unwrap();
    for (h, m) in utcvec {
        let ty_name = if h < 0 {
            if m == 0 {
                format!("UtcM{}", -h)
            } else {
                format!("UtcM{}_{}", -h, m)
            }
        } else {
            if m == 0 {
                format!("UtcP{}", h)
            } else {
                format!("UtcP{}_{}", h, m)
            }
        };
        let _ = out_file.write_fmt(format_args!(
            "pub type {ty_name} = UtcZst<{h},{m}>;",
            ty_name = ty_name,
            h = h,
            m = m
        ));
    }
}
