//! Serialization/Deserialization same as [`chrono::serde`]
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
#[cfg(feature = "serde_rfc3339")]
include!(concat!(env!("OUT_DIR"), "/serde_rfc3339.rs"));


#[cfg(test)]
#[cfg(feature = "serde_ts_seconds")]
mod test_ts_seconds {
    use crate::known_timezones::UtcP9;
    use ::serde::*;
    use chrono::*;

    #[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
    pub struct X {
        #[serde(with = "crate::serde::ts_seconds::p9")]
        pub p9: DateTime<UtcP9>,
    }

    #[test]
    fn test() {
        let dt = UtcP9::new().ymd(2000, 1, 1).and_hms(12, 0, 0);
        let x = X { p9: dt };
        let st = serde_json::to_string(&x);
        assert!(st.is_ok());
        let x2 = serde_json::from_str(&st.unwrap());
        assert_eq!(x, x2.unwrap());
    }
}

#[cfg(test)]
#[cfg(feature = "serde_rfc3339")]
mod test_rfc3339 {
    use crate::known_timezones::UtcP9;
    use ::serde::*;
    use chrono::*;

    #[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
    pub struct X {
        #[serde(with = "crate::serde::rfc3339::p9")]
        pub p9: DateTime<UtcP9>,
    }

    #[test]
    fn test() {
        let dt = UtcP9::new().ymd(2000, 1, 1).and_hms(12, 0, 0);
        let x = X { p9: dt };
        let st = serde_json::to_string(&x);
        assert!(st.is_ok());
        let x2 = serde_json::from_str(&st.unwrap());
        assert_eq!(x, x2.unwrap());
    }
}
