use datetime::{LocalDateTime, TimeZone, DatePiece};
use datetime::fmt::DateFormat;
use locale;

use fs::fields::time_t;


#[derive(Debug, Clone)]
pub struct TimeFormat {

    /// The year of the current time. This gets used to determine which date
    /// format to use.
    pub current_year: i64,

    /// Localisation rules for formatting timestamps.
    pub locale: locale::Time,

    /// Date format for printing out timestamps that are in the current year.
    pub date_and_time: DateFormat<'static>,

    /// Date format for printing out timestamps that *aren’t*.
    pub date_and_year: DateFormat<'static>,
}

impl TimeFormat {
    fn is_recent(&self, date: LocalDateTime) -> bool {
        date.year() == self.current_year
    }

    #[allow(trivial_numeric_casts)]
    pub fn format_local(&self, time: time_t) -> String {
        let date = LocalDateTime::at(time as i64);

        if self.is_recent(date) {
            self.date_and_time.format(&date, &self.locale)
        }
        else {
            self.date_and_year.format(&date, &self.locale)
        }
    }

    #[allow(trivial_numeric_casts)]
    pub fn format_zoned(&self, time: time_t, zone: &TimeZone) -> String {
        let date = zone.to_zoned(LocalDateTime::at(time as i64));

        if self.is_recent(date) {
            self.date_and_time.format(&date, &self.locale)
        }
        else {
            self.date_and_year.format(&date, &self.locale)
        }
    }

    pub fn deduce() -> TimeFormat {
        use unicode_width::UnicodeWidthStr;

        let locale = locale::Time::load_user_locale()
                       .unwrap_or_else(|_| locale::Time::english());

        let current_year = LocalDateTime::now().year();

        // Some locales use a three-character wide month name (Jan to Dec);
        // others vary between three and four (1月 to 12月). We assume that
        // December is the month with the maximum width, and use the width of
        // that to determine how to pad the other months.
        let december_width = UnicodeWidthStr::width(&*locale.short_month_name(11));
        let date_and_time = match december_width {
            4  => DateFormat::parse("{2>:D} {4>:M} {2>:h}:{02>:m}").unwrap(),
            _  => DateFormat::parse("{2>:D} {:M} {2>:h}:{02>:m}").unwrap(),
        };

        let date_and_year = match december_width {
            4 => DateFormat::parse("{2>:D} {4>:M} {5>:Y}").unwrap(),
            _ => DateFormat::parse("{2>:D} {:M} {5>:Y}").unwrap()
        };

        TimeFormat { current_year, locale, date_and_time, date_and_year }
    }
}
