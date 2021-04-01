use std::env;
use std::fs::File;
use std::io::*;
use std::path::*;
const HOUR_TO_SEC: i32 = 3600;
const MIN_TO_SEC: i32 = 60;

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
    for (h,m) in utcvec{
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
        out_file.write_fmt(format_args!("pub type {ty_name} = UtcZst<{h},{m}>;",ty_name=ty_name,h=h,m=m));
    }
    //out_file.write(b"use chrono::offset::{TimeZone,Offset,FixedOffset};");

/*    
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
            "
#[derive(Clone,Copy,Eq,PartialEq,Hash,Debug,Default)]
pub struct {ty_name};
impl {ty_name}{{
    #![allow(unused)]
    const HOUR : i32 = {h};
    const MINUTE : i32 = {m};
    const OFFSET_SECS : i32 = {ofs};
}}
impl Offset for {ty_name}{{
    fn fix(&self) -> FixedOffset{{
        FixedOffset::east({ty_name}::OFFSET_SECS)
    }}
}}
impl TimeZone for {ty_name}{{
    type Offset = Self;
    fn from_offset(offset: &Self::Offset) -> Self {{*offset}}
    fn offset_from_local_date(&self, _local: &NaiveDate) -> LocalResult<Self::Offset>{{
        LocalResult::Single(*self)
    }}
    fn offset_from_local_datetime(
        &self,
        _local: &NaiveDateTime
    ) -> LocalResult<Self::Offset>{{
        LocalResult::Single(*self)
    }}
    fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Self::Offset{{
        *self
    }}
    fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Self::Offset{{
        *self
    }}
}}
",
            ty_name = ty_name,
            h = h,
            m = m,
            ofs = {
                if h < 0 {
                    h * HOUR_TO_SEC - m * MIN_TO_SEC
                } else {
                    h * HOUR_TO_SEC + m * MIN_TO_SEC
                }
            }
        ));
    }
    */
}
