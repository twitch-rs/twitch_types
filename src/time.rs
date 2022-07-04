/// RFC3339 timestamp
#[aliri_braid::braid(serde, validator, ord = "omit")]
pub struct Timestamp;

impl aliri_braid::Validator for Timestamp {
    type Error = TimestampParseError;

    fn validate(s: &str) -> Result<(), Self::Error> {
        #[cfg(feature = "time")]
        {
            let _ = time::OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)?;
            Ok(())
        }
        #[cfg(not(feature = "time"))]
        {
            // This validator is lacking some features for now
            if !s.chars().all(|c| {
                c.is_numeric()
                    || c == 'T'
                    || c == 'Z'
                    || c == '+'
                    || c == '.'
                    || c == '-'
                    || c == ':'
            }) {
                return Err(TimestampParseError::invalid());
            }
            // PSA: Don't do time stuff... it sucks
            if let Some(i) = s.find('T') {
                // if no `T`, then it's not a valid timestamp
                if i < 1 {
                    return Err(TimestampParseError::invalid());
                };
                let (full_date, full_time) = s.split_at(i);
                if full_date.len() != "1900-00-00".len() {
                    return Err(TimestampParseError::invalid_s(full_date));
                }
                if !full_date.chars().all(|c| c.is_numeric() || c == '-') {
                    return Err(TimestampParseError::invalid_s(full_date));
                }
                let partial_time = if let Some(stripped) = full_time.strip_suffix('Z') {
                    stripped
                } else {
                    return Err(TimestampParseError::Other("unsupported non-UTC timestamp, enable the `time` feature in `twitch_types` to enable parsing these"));
                };
                if 2 != partial_time
                    .chars()
                    .into_iter()
                    .filter(|&b| b == ':')
                    .count()
                {
                    return Err(TimestampParseError::invalid_s(partial_time));
                };
                if !partial_time.contains('.') && partial_time.len() != "T00:00:00".len() {
                    return Err(TimestampParseError::invalid_s(partial_time));
                } else if partial_time.contains('.') {
                    let mut i = partial_time.split('.');
                    // if len not correct or next is none
                    if !i
                        .next()
                        .map(|s| s.len() == "T00:00:00".len())
                        .unwrap_or_default()
                    {
                        return Err(TimestampParseError::invalid_s(partial_time));
                    }
                }
            } else {
                return Err(TimestampParseError::invalid());
            }
            Ok(())
        }
    }
}

/// Errors that can occur when parsing a timestamp.
#[derive(Debug, thiserror::Error, displaydoc::Display)]
#[ignore_extra_doc_attributes]
#[non_exhaustive]
pub enum TimestampParseError {
    /// Could not parse the timestamp using `time`
    #[cfg(feature = "time")]
    #[cfg_attr(nightly, doc(cfg(feature = "time")))]
    TimeError(#[from] time::error::Parse),
    /// Could not format the timestamp using `time`
    #[cfg(feature = "time")]
    #[cfg_attr(nightly, doc(cfg(feature = "time")))]
    TimeFormatError(#[from] time::error::Format),
    /// {0}
    Other(&'static str),
    /// timestamp has an invalid format. {s:?} - {location}
    InvalidFormat {
        /// location of error
        location: &'static std::panic::Location<'static>,
        /// Thing that failed
        s: Option<String>,
    },
}

impl TimestampParseError {
    #[cfg(not(feature = "time"))]
    #[track_caller]
    fn invalid() -> Self {
        Self::InvalidFormat {
            location: std::panic::Location::caller(),
            s: None,
        }
    }

    #[cfg(not(feature = "time"))]
    #[track_caller]
    fn invalid_s(s: &str) -> Self {
        Self::InvalidFormat {
            location: std::panic::Location::caller(),
            s: Some(s.to_string()),
        }
    }
}

impl Timestamp {
    /// Set the partial-time component of the timestamp.
    ///
    /// # Panics
    ///
    /// Internally, without the `time` feature, this uses `unsafe` to deal with the raw string bytes. To ensure safety, the method will panic on invalid input and source.
    fn set_time(&mut self, hours: u8, minutes: u8, seconds: u8) {
        #[cfg(feature = "time")]
        {
            let _ = std::mem::replace(
                self,
                self.to_fixed_offset()
                    .replace_time(
                        time::Time::from_hms(hours, minutes, seconds)
                            .expect("could not create time"),
                    )
                    .try_into()
                    .expect("could not make timestamp"),
            );
        }
        #[cfg(not(feature = "time"))]
        {
            const ERROR_MSG: &str = "malformed timestamp";
            assert!(hours < 24);
            assert!(minutes < 60);
            assert!(seconds < 60);

            #[inline]
            fn replace_len2(s: &mut str, replace: &str) {
                assert!(replace.as_bytes().len() == 2);
                assert!(s.as_bytes().len() == 2);

                let replace = replace.as_bytes();
                // Safety:
                // There are two things to make sure the replacement is valid.
                // 1. The length of the two slices are equal to two.
                // 2. `replace` slice does not contain any invalid characters.
                //    As a property of being a `&str` of len 2, start and end of the str slice are valid boundaries, start is index 0, end is index 1 == `replace.len()` => 2 iff 1.)
                let b = unsafe { s.as_bytes_mut() };
                b[0] = replace[0];
                b[1] = replace[1];
            }
            let t = self.0.find('T').expect(ERROR_MSG);
            let partial_time: &mut str = &mut self.0[t + 1..];
            // find the hours, minutes and seconds
            let mut matches = partial_time.match_indices(':');
            let (h, m, s) = (
                0,
                matches.next().expect(ERROR_MSG).0 + 1,
                matches.next().expect(ERROR_MSG).0 + 1,
            );
            assert!(matches.next().is_none());
            // RFC3339 requires partial-time components to be 2DIGIT
            partial_time
                .get_mut(h..h + 2)
                .map(|s| replace_len2(s, &format!("{:02}", hours)))
                .expect(ERROR_MSG);
            partial_time
                .get_mut(m..m + 2)
                .map(|s| replace_len2(s, &format!("{:02}", minutes)))
                .expect(ERROR_MSG);
            partial_time
                .get_mut(s..s + 2)
                .map(|s| replace_len2(s, &format!("{:02}", seconds)))
                .expect(ERROR_MSG);
        }
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl Timestamp {
    /// Create a timestamp corresponding to current time
    pub fn now() -> Timestamp {
        time::OffsetDateTime::now_utc()
            .try_into()
            .expect("could not make timestamp")
    }

    /// Create a timestamp corresponding to the start of the current day. Timezone will always be UTC.
    pub fn today() -> Timestamp {
        time::OffsetDateTime::now_utc()
            .replace_time(time::Time::MIDNIGHT)
            .try_into()
            .expect("could not make timestamp")
    }
}

impl TimestampRef {
    /// Normalize the timestamp into UTC time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_types::Timestamp;
    ///
    /// let time = Timestamp::new("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.normalize()?.as_ref(), &time);
    /// let time2 = Timestamp::new("2021-07-01T13:37:00-01:00").unwrap();
    /// assert_ne!(time2.normalize()?.as_ref(), &time2);
    /// # Ok::<(), std::boxed::Box<dyn std::error::Error + 'static>>(())
    /// ```
    #[allow(unreachable_code)]
    pub fn normalize(&'_ self) -> Result<std::borrow::Cow<'_, TimestampRef>, TimestampParseError> {
        let s = self.as_str();
        if s.ends_with('Z') {
            Ok(self.into())
        } else {
            #[cfg(feature = "time")]
            {
                let utc = self.to_utc();
                return Ok(std::borrow::Cow::Owned(utc.try_into()?));
            }
            panic!("non `Z` timestamps are not possible to use without the `time` feature enabled for `twitch_types`")
        }
    }

    /// Compare another time and return `self < other`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_types::Timestamp;
    ///
    /// let time2021 = Timestamp::new("2021-07-01T13:37:00Z").unwrap();
    /// let time2020 = Timestamp::new("2020-07-01T13:37:00Z").unwrap();
    /// assert!(time2020.is_before(&time2021));
    /// ```
    pub fn is_before<T>(&self, other: &T) -> bool
    where Self: PartialOrd<T> {
        self < other
    }

    /// Make a timestamp with the time component set to 00:00:00.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_types::Timestamp;
    ///
    /// let time = Timestamp::new("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.to_day().as_str(), "2021-07-01T00:00:00Z")
    /// ```  
    pub fn to_day(&self) -> Timestamp {
        let mut c = self.to_owned();
        c.set_time(0, 0, 0);
        c
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl TimestampRef {
    /// Construct into a [`OffsetDateTime`](time::OffsetDateTime) time with a guaranteed UTC offset.
    ///
    /// # Panics
    ///
    /// This method assumes the timestamp is a valid rfc3339 timestamp, and panics if not.
    pub fn to_utc(&self) -> time::OffsetDateTime {
        self.to_fixed_offset().to_offset(time::UtcOffset::UTC)
    }

    /// Construct into a [`OffsetDateTime`](time::OffsetDateTime) time.
    ///
    /// # Panics
    ///
    /// This method assumes the timestamp is a valid rfc3339 timestamp, and panics if not.
    pub fn to_fixed_offset(&self) -> time::OffsetDateTime {
        time::OffsetDateTime::parse(&self.0, &time::format_description::well_known::Rfc3339)
            .expect("this should never fail")
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Defer to TimestampRef impl
        let this: &TimestampRef = self.as_ref();
        let other: &TimestampRef = other.as_ref();
        this.partial_cmp(other)
    }
}

impl PartialOrd<Timestamp> for TimestampRef {
    fn partial_cmp(&self, other: &Timestamp) -> Option<std::cmp::Ordering> {
        // Defer to TimestampRef impl
        let other: &TimestampRef = other.as_ref();
        self.partial_cmp(other)
    }
}

impl PartialOrd for TimestampRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // to check ordering, we normalize offset, then do a lexicographic comparison if possible,
        // We can do this because the timestamp should always be RFC3339 with time-offset = 'Z' with normalize.
        // However, we need to make sure punctuation and length is correct. Without the `time` feature, it's impossible to get a non-UTC timestamp, so normalize will do nothing.
        let this = self
            .normalize()
            .expect("normalization failed, this is a bug");
        let other = other
            .normalize()
            .expect("normalization of other failed, this is a bug");
        // If a punctuation exists in only one, we can't order.
        #[allow(clippy::if_same_then_else)]
        if this.as_ref().as_str().contains('.') ^ other.as_ref().as_str().contains('.') {
            #[cfg(feature = "tracing")]
            tracing::warn!("comparing two `Timestamps` with differing punctuation");
            return None;
        } else if this.0.len() != other.0.len() {
            #[cfg(feature = "tracing")]
            tracing::warn!("comparing two `Timestamps` with differing length");
            return None;
        }
        this.as_str().partial_cmp(other.as_str())
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl PartialEq<time::OffsetDateTime> for Timestamp {
    fn eq(&self, other: &time::OffsetDateTime) -> bool {
        // Defer to TimestampRef impl
        let this: &TimestampRef = self.as_ref();
        this.eq(other)
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl PartialOrd<time::OffsetDateTime> for Timestamp {
    fn partial_cmp(&self, other: &time::OffsetDateTime) -> Option<std::cmp::Ordering> {
        // Defer to TimestampRef impl
        let this: &TimestampRef = self.as_ref();
        this.partial_cmp(other)
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl PartialEq<time::OffsetDateTime> for TimestampRef {
    fn eq(&self, other: &time::OffsetDateTime) -> bool { &self.to_utc() == other }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl PartialOrd<time::OffsetDateTime> for TimestampRef {
    fn partial_cmp(&self, other: &time::OffsetDateTime) -> Option<std::cmp::Ordering> {
        self.to_utc().partial_cmp(other)
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly, doc(cfg(feature = "time")))]
impl std::convert::TryFrom<time::OffsetDateTime> for Timestamp {
    type Error = time::error::Format;

    fn try_from(value: time::OffsetDateTime) -> Result<Self, Self::Error> {
        Ok(Timestamp(
            value.format(&time::format_description::well_known::Rfc3339)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn time_test() {
        let mut time1 = Timestamp::new("2021-11-11T10:00:00Z").unwrap();
        time1.set_time(10, 0, 32);
        let time2 = Timestamp::new("2021-11-10T10:00:00Z").unwrap();
        assert!(time2.is_before(&time1));
        dbg!(time1.normalize().unwrap());
        #[cfg(feature = "time")]
        let time = Timestamp::new("2021-11-11T13:37:00-01:00").unwrap();
        #[cfg(feature = "time")]
        dbg!(time.normalize().unwrap());
    }
}
