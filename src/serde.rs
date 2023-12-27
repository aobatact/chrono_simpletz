/*!
Serialization/Deserialization same as [`chrono::serde`]
```
# #![cfg(feature = "serde_rfc3339")]
use chrono_simpletz::known_timezones::UtcP9;
use ::serde::*;
use chrono::*;

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct X {
    #[serde(with = "chrono_simpletz::serde::rfc3339")]
    pub p9: DateTime<UtcP9>,
}

let dt = UtcP9::new().with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
let x = X { p9: dt };
let st = serde_json::to_string(&x);
assert!(st.is_ok());
let x2 = serde_json::from_str(&st.unwrap());
assert_eq!(x, x2.unwrap());
```
*/
use super::*;

macro_rules! known_timezone_serde_rfc3339 {
    ($mod_name:ident,$known_ty:ty,$known_ty_ident:ident) => {
        #[deprecated(since = "0.3.0", note = "use `crate::serde::rfc3339` instead")]
        pub mod $mod_name {
            use crate::known_timezones::$known_ty_ident;
            use ::serde::*;
            use chrono::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S>(dt: &DateTime<$known_ty>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                dt.with_timezone(&Utc).serialize(serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D>(d: D) -> Result<DateTime<$known_ty>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                DateTime::<Utc>::deserialize(d).map(|x| x.with_timezone(&<$known_ty>::new()))
            }
        }
    };
}

macro_rules! known_timezone_serde_rfc3339_option {
    ($mod_name:ident,$known_ty:ty,$known_ty_ident:ident) => {
        #[deprecated(since = "0.3.0", note = "use `crate::serde::rfc3339_option` instead")]
        pub mod $mod_name {
            use crate::known_timezones::$known_ty_ident;
            use ::serde::*;
            use chrono::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S>(
                dt: &Option<DateTime<$known_ty>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                match dt {
                    Some(dt) => dt.with_timezone(&Utc).serialize(serializer),
                    None => serializer.serialize_none(),
                }
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D>(d: D) -> Result<Option<DateTime<$known_ty>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                Option::<DateTime<Utc>>::deserialize(d)
                    .map(|x| x.map(|x| x.with_timezone(&<$known_ty>::new())))
            }
        }
    };
}

macro_rules! known_timezone_serde_with {
    ($mod_name:ident,$known_ty:ty,$known_ty_ident:ident,$ser:path,$de:path) => {
        #[deprecated(since = "0.3.0", note = "use super instead")]
        pub mod $mod_name {
            use crate::known_timezones::$known_ty_ident;
            use chrono::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S>(dt: &DateTime<$known_ty>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ($ser)(&dt.with_timezone(&chrono::Utc), serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D>(d: D) -> Result<DateTime<$known_ty>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                ($de)(d).map(|x| x.with_timezone(&<$known_ty>::new()))
            }
        }
    };
}

macro_rules! known_timezone_serde_with_opt {
    ($mod_name:ident,$known_ty:ty,$known_ty_ident:ident,$ser:path,$de:path) => {
        #[deprecated(since = "0.3.0", note = "use super instead")]
        pub mod $mod_name {
            use crate::known_timezones::$known_ty_ident;
            use chrono::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S>(
                dt: &Option<DateTime<$known_ty>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ($ser)(&dt.map(|x| x.with_timezone(&chrono::Utc)), serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D>(d: D) -> Result<Option<DateTime<$known_ty>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                ($de)(d).map(|x| x.map(|y| y.with_timezone(&<$known_ty>::new())))
            }
        }
    };
}

macro_rules! known_timezone_serde {
    (
        [$(($mod_name:ident,$known_ty:ty,$known_ty_ident:ident)),*$(,)?]
    ) => {
        #[cfg(feature="serde_rfc3339")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde_rfc3339")))]
        /// Ser/de to/from rfc3339
        pub mod rfc3339 {
            use ::serde::*;
            use chrono::*;
            use super::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S, const HOUR: i32, const MINUTE: u32>(dt: &DateTime<TimeZoneZst<HOUR, MINUTE>>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                dt.with_timezone(&Utc).serialize(serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D, const HOUR: i32, const MINUTE: u32>(d: D) -> Result<DateTime<TimeZoneZst<HOUR, MINUTE>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                DateTime::<Utc>::deserialize(d).map(|x| x.with_timezone(&<TimeZoneZst<HOUR, MINUTE>>::new()))
            }
            $(known_timezone_serde_rfc3339!($mod_name,$known_ty,$known_ty_ident);)*
        }
        #[cfg(feature="serde_rfc3339_option")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde_rfc3339_option")))]
        /// Ser/de to/from optional rfc3339
        pub mod rfc3339_option {
            use ::serde::*;
            use super::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S, const HOUR: i32, const MINUTE: u32>(
                dt: &Option<DateTime<TimeZoneZst<HOUR, MINUTE>>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                match dt {
                    Some(dt) => dt.with_timezone(&Utc).serialize(serializer),
                    None => serializer.serialize_none(),
                }
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D, const HOUR: i32, const MINUTE: u32>(d: D) -> Result<Option<DateTime<TimeZoneZst<HOUR, MINUTE>>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                Option::<DateTime<Utc>>::deserialize(d)
                    .map(|x| x.map(|x| x.with_timezone(&<TimeZoneZst<HOUR, MINUTE>>::new())))
            }
            $(known_timezone_serde_rfc3339_option!($mod_name,$known_ty,$known_ty_ident);)*
        }
        #[cfg(feature="serde_ts_seconds")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde_ts_seconds")))]
        /// Ser/de to/from timestamps in seconds
        pub mod ts_seconds {
            use super::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S, const HOUR: i32, const MINUTE: u32>(
                dt: &DateTime<TimeZoneZst<HOUR, MINUTE>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                chrono::serde::ts_seconds::serialize(&dt.with_timezone(&chrono::Utc), serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D, const HOUR: i32, const MINUTE: u32>(
                d: D,
            ) -> Result<DateTime<TimeZoneZst<HOUR, MINUTE>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                chrono::serde::ts_seconds::deserialize(d).map(|x| x.with_timezone(&TimeZoneZst::<HOUR, MINUTE>::new()))
            }

            $(
                known_timezone_serde_with!($mod_name,$known_ty,$known_ty_ident,
                    chrono::serde::ts_seconds::serialize,
                    chrono::serde::ts_seconds::deserialize);
            )*
        }
        #[cfg(feature="serde_ts_seconds_option")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde_ts_seconds_option")))]
        /// Ser/de to/from optional timestamps in seconds
        pub mod ts_seconds_option {
            use super::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S, const HOUR: i32, const MINUTE: u32>(
                dt: &Option<DateTime<TimeZoneZst<HOUR, MINUTE>>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,

            {
                chrono::serde::ts_seconds_option::serialize(&dt.map(|x| x.with_timezone(&chrono::Utc)), serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D, const HOUR: i32, const MINUTE: u32>(
                d: D,
            ) -> Result<Option<DateTime<TimeZoneZst<HOUR, MINUTE>>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                chrono::serde::ts_seconds_option::deserialize(d).map(|x| x.map(|y| y.with_timezone(&<TimeZoneZst<HOUR, MINUTE>>::new())))
            }
            $(
                known_timezone_serde_with_opt!($mod_name,$known_ty,$known_ty_ident,
                    chrono::serde::ts_seconds_option::serialize,
                    chrono::serde::ts_seconds_option::deserialize);
            )*
        }
        #[cfg(feature="serde_ts_nanoseconds")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde_ts_nanoseconds")))]
        /// Ser/de to/from timestamps in nanoseconds
        pub mod ts_nanoseconds {
            use super::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S, const HOUR: i32, const MINUTE: u32>(
                dt: &DateTime<TimeZoneZst<HOUR, MINUTE>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                chrono::serde::ts_nanoseconds::serialize(&dt.with_timezone(&chrono::Utc), serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D, const HOUR: i32, const MINUTE: u32>(
                d: D,
            ) -> Result<DateTime<TimeZoneZst<HOUR, MINUTE>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                chrono::serde::ts_nanoseconds::deserialize(d).map(|x| x.with_timezone(&TimeZoneZst::<HOUR, MINUTE>::new()))
            }
            $(
                known_timezone_serde_with!($mod_name,$known_ty,$known_ty_ident,
                    chrono::serde::ts_nanoseconds::serialize,
                    chrono::serde::ts_nanoseconds::deserialize);
            )*
        }
        #[cfg(feature="serde_ts_nanoseconds_option")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde_ts_nanoseconds_option")))]
        /// Ser/de to/from optional timestamps in nanoseconds
        pub mod ts_nanoseconds_option {
            use super::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S, const HOUR: i32, const MINUTE: u32>(
                dt: &Option<DateTime<TimeZoneZst<HOUR, MINUTE>>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                chrono::serde::ts_nanoseconds_option::serialize(&dt.map(|x| x.with_timezone(&chrono::Utc)), serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D, const HOUR: i32, const MINUTE: u32>(
                d: D,
            ) -> Result<Option<DateTime<TimeZoneZst<HOUR, MINUTE>>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                chrono::serde::ts_nanoseconds_option::deserialize(d).map(|x| x.map(|y| y.with_timezone(&<TimeZoneZst<HOUR, MINUTE>>::new())))
            }
            $(
                known_timezone_serde_with_opt!($mod_name,$known_ty,$known_ty_ident,
                    chrono::serde::ts_nanoseconds_option::serialize,
                    chrono::serde::ts_nanoseconds_option::deserialize);
            )*
        }
        #[cfg(feature="serde_ts_milliseconds")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde_ts_milliseconds")))]
        /// Ser/de to/from timestamps in milliseconds
        pub mod ts_milliseconds {
            use super::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S, const HOUR: i32, const MINUTE: u32>(
                dt: &DateTime<TimeZoneZst<HOUR, MINUTE>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                chrono::serde::ts_milliseconds::serialize(&dt.with_timezone(&chrono::Utc), serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D, const HOUR: i32, const MINUTE: u32>(
                d: D,
            ) -> Result<DateTime<TimeZoneZst<HOUR, MINUTE>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                chrono::serde::ts_milliseconds::deserialize(d).map(|x| x.with_timezone(&TimeZoneZst::<HOUR, MINUTE>::new()))
            }
            $(
                known_timezone_serde_with!($mod_name,$known_ty,$known_ty_ident,
                    chrono::serde::ts_milliseconds::serialize,
                    chrono::serde::ts_milliseconds::deserialize);
            )*
        }
        #[cfg(feature="serde_ts_milliseconds_option")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde_ts_milliseconds_option")))]
        /// Ser/de to/from optional timestamps in millisecond
        pub mod ts_milliseconds_option {
            use super::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S, const HOUR: i32, const MINUTE: u32>(
                dt: &Option<DateTime<TimeZoneZst<HOUR, MINUTE>>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                chrono::serde::ts_milliseconds_option::serialize(&dt.map(|x| x.with_timezone(&chrono::Utc)), serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D, const HOUR: i32, const MINUTE: u32>(
                d: D,
            ) -> Result<Option<DateTime<TimeZoneZst<HOUR, MINUTE>>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                chrono::serde::ts_milliseconds_option::deserialize(d).map(|x| x.map(|y| y.with_timezone(&<TimeZoneZst<HOUR, MINUTE>>::new())))
            }
            $(
                known_timezone_serde_with_opt!($mod_name,$known_ty,$known_ty_ident,
                    chrono::serde::ts_milliseconds_option::serialize,
                    chrono::serde::ts_milliseconds_option::deserialize);
            )*
        }
        #[cfg(feature="serde_ts_microseconds")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde_ts_microseconds")))]
        /// Ser/de to/from timestamps in microseconds
        pub mod ts_microseconds {
            use super::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S, const HOUR: i32, const MINUTE: u32>(
                dt: &DateTime<TimeZoneZst<HOUR, MINUTE>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                chrono::serde::ts_microseconds::serialize(&dt.with_timezone(&chrono::Utc), serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D, const HOUR: i32, const MINUTE: u32>(
                d: D,
            ) -> Result<DateTime<TimeZoneZst<HOUR, MINUTE>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                chrono::serde::ts_microseconds::deserialize(d).map(|x| x.with_timezone(&TimeZoneZst::<HOUR, MINUTE>::new()))
            }
        }
        #[cfg(feature="serde_ts_microseconds_option")]
        #[cfg_attr(doc_cfg, doc(cfg(feature = "serde_ts_microseconds_option")))]
        /// Ser/de to/from optional timestamps in microseconds
        pub mod ts_microseconds_option {
            use super::*;
            /// Funciton for serialize. Use this for serialize_with.
            pub fn serialize<S, const HOUR: i32, const MINUTE: u32>(
                dt: &Option<DateTime<TimeZoneZst<HOUR, MINUTE>>>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                chrono::serde::ts_microseconds_option::serialize(&dt.map(|x| x.with_timezone(&chrono::Utc)), serializer)
            }
            /// Funciton for deserialize. Use this for deserialize_with.
            pub fn deserialize<'de, D, const HOUR: i32, const MINUTE: u32>(
                d: D,
            ) -> Result<Option<DateTime<TimeZoneZst<HOUR, MINUTE>>>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                chrono::serde::ts_microseconds_option::deserialize(d).map(|x| x.map(|y| y.with_timezone(&<TimeZoneZst<HOUR, MINUTE>>::new())))
            }
        }
    };
}

known_timezone_serde!([
    (p14, UtcP14, UtcP14),
    (p13, UtcP13, UtcP13),
    (p12_45, UtcP12_45, UtcP12_45),
    (p12, UtcP12, UtcP12),
    (p11, UtcP11, UtcP11),
    (p10_30, UtcP10_30, UtcP10_30),
    (p10, UtcP10, UtcP10),
    (p9_30, UtcP9_30, UtcP9_30),
    (p9, UtcP9, UtcP9),
    (p8_45, UtcP8_45, UtcP8_45),
    (p8, UtcP8, UtcP8),
    (p7, UtcP7, UtcP7),
    (p6_30, UtcP6_30, UtcP6_30),
    (p6, UtcP6, UtcP6),
    (p5_45, UtcP5_45, UtcP5_45),
    (p5_30, UtcP5_30, UtcP5_30),
    (p5, UtcP5, UtcP5),
    (p4_30, UtcP4_30, UtcP4_30),
    (p4, UtcP4, UtcP4),
    (p3_30, UtcP3_30, UtcP3_30),
    (p3, UtcP3, UtcP3),
    (p2, UtcP2, UtcP2),
    (p1, UtcP1, UtcP1),
    (p0, UtcP0, UtcP0),
    (m1, UtcM1, UtcM1),
    (m2, UtcM2, UtcM2),
    (m3, UtcM3, UtcM3),
    (m3_30, UtcM3_30, UtcM3_30),
    (m4, UtcM4, UtcM4),
    (m5, UtcM5, UtcM5),
    (m6, UtcM6, UtcM6),
    (m7, UtcM7, UtcM7),
    (m8, UtcM8, UtcM8),
    (m9, UtcM9, UtcM9),
    (m9_30, UtcM9_30, UtcM9_30),
    (m10, UtcM10, UtcM10),
    (m11, UtcM11, UtcM11),
    (m12, UtcM12, UtcM12)
]);

#[cfg(test)]
#[cfg(feature = "serde_ts_seconds")]
mod test_ts_seconds {
    use crate::known_timezones::UtcP9;
    use ::serde::*;
    use chrono::*;

    #[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
    pub struct X {
        #[serde(with = "crate::serde::ts_seconds")]
        pub p9: DateTime<UtcP9>,
    }

    #[test]
    fn test() {
        let dt = UtcP9::new().with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
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
        #[serde(with = "crate::serde::rfc3339")]
        pub p9: DateTime<UtcP9>,
    }

    #[test]
    fn test() {
        let dt = UtcP9::new().with_ymd_and_hms(2000, 1, 1, 12, 0, 0).unwrap();
        let x = X { p9: dt };
        let st = serde_json::to_string(&x);
        assert!(st.is_ok());
        let x2 = serde_json::from_str(&st.unwrap());
        assert_eq!(x, x2.unwrap());
    }
}
