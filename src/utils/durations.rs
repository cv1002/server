#[macro_export]
macro_rules! seconds_of {
    (
        $(years: $years:expr,)?
        $(weeks: $weeks:expr,)?
        $(days: $days:expr,)?
        $(hours: $hours:expr,)?
        $(minutes: $minutes:expr,)?
        $(seconds: $seconds:expr,)?
    ) => {
        0
        $(+ $seconds)?
        $(+ $minutes * 60)?
        $(+ $hours * 3600)?
        $(+ $days * 3600 * 24)?
        $(+ $weeks * 3600 * 24 * 7)?
        $(+ $years * 3600 * 24 * 365)?
    };
    ($($ident:ident: $expr:expr), +) => {
        seconds_of!($($ident: $expr,) +)
    };
}

#[macro_export]
macro_rules! duration_of {
    ($($ident:ident: $expr:expr),*) => {
        std::time::Duration::from_secs(
            seconds_of!($($ident: $expr,) *)
        )
    };
}

macro_rules! durations {
    ($($seconds: ident, $duration: ident => $expr: expr;) *) => {
        $(
            #[allow(unused)]
            pub const fn $seconds() -> u64 {
                $expr
            }
            #[allow(unused)]
            pub const fn $duration() -> std::time::Duration {
                std::time::Duration::from_secs($expr)
            }
        ) *
    };
}

durations! {
    one_minute_secs, one_minute_duration
        => seconds_of!(minutes: 1)
    ;
    one_hour_secs, one_hour_duration
        => seconds_of!(hours: 1)
    ;
    six_hours_secs, six_hours_duration
        => seconds_of!(hours: 6)
    ;
    half_day_secs, half_day_duration
        => seconds_of!(hours: 12)
    ;
    one_day_secs, one_day_duration
        => seconds_of!(days: 1)
    ;
    three_days_secs, three_days_duration
        => seconds_of!(days: 3)
    ;
    a_week_secs, a_week_duration
        => seconds_of!(weeks: 1)
    ;
    a_year_secs, a_year_duration
        => seconds_of!(years: 1)
    ;
}
