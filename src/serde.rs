#[cfg(feature = "serde_ts_seconds")]
include!(concat!(env!("OUT_DIR"), "/serde_ts_seconds.rs"));
#[cfg(feature = "serde_ts_seconds_option")]
include!(concat!(env!("OUT_DIR"), "/serde_ts_seconds_option.rs"));
#[cfg(feature = "serde_ts_milliseconds")]
include!(concat!(env!("OUT_DIR"), "/serde_ts_milliseconds.rs"));
#[cfg(feature = "serde_ts_milliseconds_option")]
include!(concat!(env!("OUT_DIR"), "/serde_ts_milliseconds_option.rs"));
#[cfg(feature = "serde_ts_nanoseconds")]
include!(concat!(env!("OUT_DIR"), "/serde_ts_nanoseconds.rs"));
#[cfg(feature = "serde_ts_nanoseconds_option")]
include!(concat!(env!("OUT_DIR"), "/serde_ts_nanoseconds_option.rs"));

#[cfg(test)]
#[cfg(feature = "serde_ts_seconds")]
mod test_ts_seconds{
    use serde1::{Serialize, Deserialize};
    use serde_json::*;
    use chrono::*;
    use crate::*;
    use crate::known_timezones::UtcP9;
    #[derive(Serialize,Deserialize,PartialEq,Eq,Clone)]
    pub struct X{
        #[serde(with = "crate::serde::ts_seconds::P9")]
        pub  p9 : DateTime::<UtcP9>,
    }

    fn test(){
        let dt = UtcP9::new().ymd(2000,1,1).and_hms(12,0,0);
        let x = X{p9 : dt};
        let st = serde_json::to_string(x);
        assert!(st.is_ok());
        let x2 = serde_json::from_str(st.unwrap());
        assert_eq!(x,x2);
    }
}