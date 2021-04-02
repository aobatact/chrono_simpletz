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

    {
        let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("known_timezones.rs");
        let mut out_file = File::create(out_path).unwrap();
        for (h, m) in &utcvec {
            let ty_name = if *h < 0 {
                if *m == 0 {
                    format!("UtcM{}", -h)
                } else {
                    format!("UtcM{}_{}", -h, m)
                }
            } else {
                if *m == 0 {
                    format!("UtcP{}", h)
                } else {
                    format!("UtcP{}_{}", h, m)
                }
            };
            let _ = out_file.write_fmt(format_args!(
                "
/// Alias for Utc{h:+03}:{m:02}
pub type {ty_name} = UtcZst<{h},{m}>;",
                ty_name = ty_name,
                h = h,
                m = m
            ));
        }
    }
    let serde_mods = [
        "ts_seconds",
        "ts_seconds_option",
        "ts_milliseconds",
        "ts_milliseconds_option",
        "ts_nanoseconds",
        "ts_nanoseconds_option",
    ];
    for s_m in &serde_mods {
        if env::var(format!("CARGO_FEATURE_SERDE_{}", s_m.to_uppercase())).is_ok() {
            let out_path =
                Path::new(&env::var("OUT_DIR").unwrap()).join(format!("serde_{}.rs", s_m));
            let mut out_file = File::create(out_path).unwrap();

            let _ = out_file.write_fmt(format_args!(
                "/// Ser/de to/from timestamps with {s_m}.\\
                /// Only available for serde_{s_m} feature flag.
            pub mod {s_m} {{",
                s_m = s_m
            ));
            for (h, m) in &utcvec {
                let type_name = if *h < 0 {
                    if *m == 0 {
                        format!("M{}", -h)
                    } else {
                        format!("M{}_{}", -h, m)
                    }
                } else {
                    if *m == 0 {
                        format!("P{}", h)
                    } else {
                        format!("P{}_{}", h, m)
                    }
                };
                let (ty, dt_conv, is_op) = if s_m.ends_with("option") {
                    (
                        format!("Option<DateTime<Utc{type_name}>>", type_name = type_name),
                        "&dt.map(|x|x.with_timezone(&chrono::Utc))",
                        true,
                    )
                } else {
                    (
                        format!("DateTime<Utc{type_name}>", type_name = type_name),
                        "&dt.with_timezone(&chrono::Utc)",
                        false,
                    )
                };
                let _ = out_file.write_fmt(format_args!(
                    "
/// De/serialize {s_m} for Utc{h:+03}:{m:02}. 
pub mod {mod_name}{{
    use crate::known_timezones::Utc{type_name};
    use chrono::*;
    /// Funciton for serialize. Use this for serialize_with. 
    pub fn serialize<S>(dt: &{ty}, serializer : S) -> Result<S::Ok, S::Error> 
        where S : ::serde::Serializer {{ chrono::serde::{s_m}::serialize({dt_conv}, serializer) }}
    /// Funciton for deserialize. Use this for deserialize_with. 
    pub fn deserialize<'de, D>(d: D) -> Result<{ty}, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {{
        chrono::serde::{s_m}::deserialize(d).map(|x| {demap})
    }}
}}",
                    mod_name = type_name.to_lowercase(),
                    type_name = type_name,
                    s_m = s_m,
                    ty = ty,
                    dt_conv = dt_conv,
                    demap = if is_op {
                        format!("x.map(|y|y.with_timezone( &Utc{}::new()))", type_name)
                    } else {
                        format!("x.with_timezone( &Utc{}::new())", type_name)
                    },
                    h = h,
                    m = m
                ));
            }
            let _ = out_file.write(b"}\n");
        }
    }
    if env::var("CARGO_FEATURE_SERDE_RFC3339").is_ok() {
        let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("serde_rfc3339.rs");
        let mut out_file = File::create(out_path).unwrap();

        let _ = out_file.write(
            b"/// De/serialize as De/Serialize for DateTime<Utc>.\\
            /// We need this because DateTime<UtcZtc<H,M>> cannot impl De/Serialize.\\
            /// Only available for serde_rfc3339 feature flag.
            pub mod rfc3339 {",
        );
        for (h, m) in &utcvec {
            let type_name = if *h < 0 {
                if *m == 0 {
                    format!("UtcM{}", -h)
                } else {
                    format!("UtcM{}_{}", -h, m)
                }
            } else {
                if *m == 0 {
                    format!("UtcP{}", h)
                } else {
                    format!("UtcP{}_{}", h, m)
                }
            };
            let _ = out_file.write_fmt(format_args!(
                "
/// De/serialize as De/Serialize for DateTime<UtcZtc<H,M>>.
pub mod {mod_name}{{
    use crate::known_timezones::{ty};
    use chrono::*;
    use ::serde::*;
    /// Funciton for serialize. Use this for serialize_with. 
    pub fn serialize<S>(dt: &DateTime<{ty}>, serializer : S) -> Result<S::Ok, S::Error> 
        where S : ::serde::Serializer {{ dt.with_timezone(&Utc).serialize(serializer) }}
    /// Funciton for deserialize. Use this for deserialize_with. 
    pub fn deserialize<'de, D>(d: D) -> Result<DateTime<{ty}>, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {{
        DateTime::<Utc>::deserialize(d).map(|x|x.with_timezone(&{ty}::new()))
    }}
}}
",
                mod_name = type_name[3..].to_lowercase(),
                ty = type_name,
            ));
        }

        let _ = out_file.write(b"}\n");
    }
}
