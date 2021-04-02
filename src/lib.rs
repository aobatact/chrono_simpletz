//! Simple Zero Sized Typed Utc timezones for [`chrono`].
//! This needs const generic (for rust >= 1.51 in stable).
//! ```
//! use chrono::*;
//! use chrono_simpletz::UtcZst;
//! use chrono_simpletz::known_timezones::*;
//! use std::mem::size_of_val;
//!
//! //construct by new() or Default::default()
//! let p9 = UtcP9::new();
//! //size of UtcP9 is zero
//! assert_eq!(size_of_val(&p9), 0);
//! assert_eq!(&p9.to_string(), "+09:00");
//! assert_eq!(UtcP9::IS_IN_VALID_RANGE, true);
//!
//! let time = p9.ymd(2000, 1, 1).and_hms(12, 00, 00);
//! let naive_time = NaiveDate::from_ymd(2000, 1, 1).and_hms(3, 0, 0);
//! assert_eq!(time.naive_utc(), naive_time);
//! //same size as naive datetime
//! assert_eq!(size_of_val(&time),size_of_val(&naive_time));
//!
//! let fixed = time.with_timezone(&p9.fix());
//! assert_eq!(time, fixed);
//! //same Display with FixedOffset
//! assert_eq!(time.to_string(), fixed.to_string());
//! // smaller size than FixedOffset size
//! assert!(size_of_val(&time) < size_of_val(&fixed) )
//! ```
//!
//! # features
//! ## std (default)
//! with std
//!
//! ## clock (default)
//! Adds today and now function for UtcZst.
//!
//! ## serde
//! ### serde_ts_(seconds|milliseconds|nanoseconds)(|_option)
//! Adds modules for de/serialize functions to use with de/serialize_with function.
//!
//! ### serde_ts_rfc3339(|_option)
//! Adds modules for de/serialize functions to use with de/serialize_with function.
//! You need this when you want to de/serialize like `DateTime<Utc>`, because `DateTime<UtcZtc<H,M>>` cannot impl De/Serialize.
//!
#![cfg_attr(not(std), no_std)]

use chrono::*;

const HOUR_TO_SEC: i32 = 3600;
const MIN_TO_SEC: i32 = 60;
pub mod known_timezones;

/// Represent Fixed Timezone with zero sized type and const generics.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Default, Ord, PartialOrd)]
pub struct UtcZst<const HOUR: i32, const MINUTE: u32>;
impl<const HOUR: i32, const MINUTE: u32> UtcZst<HOUR, MINUTE> {
    /// Gets the offset seconds. This is used to get [`FixedOffset`].
    pub const OFFSET_SECS: i32 =
        HOUR * HOUR_TO_SEC + if HOUR < 0 { -1 } else { 1 } * (MINUTE as i32) * MIN_TO_SEC;
    /// Checks whether the `HOUR` and `MINUTE` is in valid range`(-23 <= HOUR <= 23 & MINUTE < 60)`. This does not check whether the timezone is known.
    pub const IS_IN_VALID_RANGE: bool = (HOUR >= -23) & (HOUR <= 23) & (MINUTE < 60);
    /// Creates new `UtcZst`
    pub const fn new() -> Self {
        UtcZst
    }
    #[cfg(clock)]
    /// Returns a Date which corresponds to the current date. Only available with clock feature.
    pub fn today() -> Date<Self> {
        Utc::today().with_timezone(Self::new())
    }
    #[cfg(clock)]
    /// Returns a DateTime which corresponds to the current date. Only available with clock feature.
    pub fn now() -> DateTime<Self> {
        Utc::now().with_timezone(Self::new())
    }
}
impl<const HOUR: i32, const MINUTE: u32> Offset for UtcZst<HOUR, MINUTE> {
    fn fix(&self) -> FixedOffset {
        FixedOffset::east(Self::OFFSET_SECS)
    }
}
impl<const HOUR: i32, const MINUTE: u32> TimeZone for UtcZst<HOUR, MINUTE> {
    type Offset = Self;
    fn from_offset(offset: &Self::Offset) -> Self {
        *offset
    }
    fn offset_from_local_date(&self, _local: &NaiveDate) -> LocalResult<Self::Offset> {
        LocalResult::Single(*self)
    }
    fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> LocalResult<Self::Offset> {
        LocalResult::Single(*self)
    }
    fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Self::Offset {
        *self
    }
    fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Self::Offset {
        *self
    }
}
// I don't want to do like this (because it loses some information for debuging), but chrono/serde is using Debug of Offset for Serializing DateTime so ...
impl<const HOUR: i32, const MINUTE: u32> core::fmt::Debug for UtcZst<HOUR, MINUTE> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:+03}:{:02}", HOUR, MINUTE)
    }
}
impl<const HOUR: i32, const MINUTE: u32> core::fmt::Display for UtcZst<HOUR, MINUTE> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:+03}:{:02}", HOUR, MINUTE)
    }
}

#[cfg(feature = "serde1")]
pub mod serde;

#[cfg(test)]
#[cfg(std)]
mod tests {
    use crate::known_timezones::*;
    use crate::*;
    #[test]
    fn it_works() {
        let p9 = UtcP9::default();
        assert_eq!(&p9.to_string(), "+09:00");
        assert_eq!(std::mem::size_of_val(&p9), 0);
        assert_eq!(UtcP9::IS_IN_VALID_RANGE, true);
        let n = p9.ymd(2000, 1, 1).and_hms(12, 00, 00);
        assert_eq!(
            n.naive_utc(),
            NaiveDate::from_ymd(2000, 1, 1).and_hms(3, 0, 0)
        );
        let m9 = UtcM9::default();
        assert_eq!(&m9.to_string(), "-09:00");
        let n = m9.ymd(2000, 1, 1).and_hms(12, 00, 00);
        assert_eq!(
            n.naive_utc(),
            NaiveDate::from_ymd(2000, 1, 1).and_hms(21, 0, 0)
        );
        let p9 = UtcP9_30::default();
        let n = p9.ymd(2000, 1, 1).and_hms(12, 00, 00);
        assert_eq!(
            n.naive_utc(),
            NaiveDate::from_ymd(2000, 1, 1).and_hms(2, 30, 0)
        );
        let m9 = UtcM9_30::default();
        let n = m9.ymd(2000, 1, 1).and_hms(12, 00, 00);
        assert_eq!(
            n.naive_utc(),
            NaiveDate::from_ymd(2000, 1, 1).and_hms(21, 30, 0)
        );
    }
}
