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
