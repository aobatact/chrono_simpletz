# chrono-simpletz
[![Doc](https://docs.rs/chrono-simpletz/badge.svg)](https://docs.rs/chrono-simpletz)
[![Crate](https://img.shields.io/crates/v/chrono-simpletz.svg)](https://crates.io/crates/chrono-simpletz)


Simple Zero Sized Typed Utc timezones for [chrono](https://docs.rs/chrono/).
This needs const generic (for rust >= 1.51 in stable).
```
use chrono::*;
use chrono_simpletz::UtcZst;
use chrono_simpletz::known_timezones::*;
use std::mem::size_of_val;
//construct by new() or Default::default()
let p9 = UtcP9::new();
//size of UtcP9 is zero
assert_eq!(size_of_val(&p9), 0);
assert_eq!(&p9.to_string(), "+09:00");
assert_eq!(UtcP9::IS_IN_VALID_RANGE, true);
let time = p9.ymd(2000, 1, 1).and_hms(12, 00, 00);
let naive_time = NaiveDate::from_ymd(2000, 1, 1).and_hms(3, 0, 0);
assert_eq!(time.naive_utc(), naive_time);
//same size as naive datetime
assert_eq!(size_of_val(&time),size_of_val(&naive_time));
let fixed = time.with_timezone(&p9.fix());
assert_eq!(time, fixed);
//same Display with FixedOffset
assert_eq!(time.to_string(), fixed.to_string());
// smaller size than fixed offset size
assert!(size_of_val(&time) < size_of_val(&fixed) )
```

# features
## clock
Adds today and now function for UtcZst. 

## serde
### serde_ts_(seconds|milliseconds|nanoseconds)(|_option)
Adds modules for de/serialize functions to use with de/serialize_with function.

