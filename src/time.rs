#![allow(clippy::missing_safety_doc)]

#[derive(Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
/// RFC3339 timestamp
pub struct Timestamp(String);

impl Timestamp {
    ///Constructs a new Timestamp if it conforms to [`Timestamp`]
    #[inline]
    pub fn new(raw: String) -> Result<Self, TimestampParseError> {
        TimestampRef::validate(raw.as_ref())?;
        Ok(Self(raw))
    }

    ///Constructs a new Timestamp without validation
    ///
    ///# Safety
    ///
    ///Consumers of this function must ensure that values conform to [`Timestamp`]. Failure to maintain this invariant may lead to undefined behavior.
    #[allow(unsafe_code)]
    #[inline]
    pub const unsafe fn new_unchecked(raw: String) -> Self { Self(raw) }

    #[inline]
    ///Constructs a new Timestamp from a static reference if it conforms to [`Timestamp`]
    ///
    ///# Panics
    ///
    ///This function will panic if the provided raw string is not valid.
    #[track_caller]
    pub fn from_static(raw: &'static str) -> Self {
        ::std::borrow::ToOwned::to_owned(TimestampRef::from_static(raw))
    }

    ///Converts this `Timestamp` into a [`Box<TimestampRef>`]
    ///
    ///This will drop any excess capacity.
    #[allow(unsafe_code)]
    #[inline]
    pub fn into_boxed_ref(self) -> Box<TimestampRef> {
        ///SAFETY: `TimestampRef` is `#[repr(transparent)]` around a single `str` field, so a `*mut str` can be safely reinterpreted as a `*mut TimestampRef`
        fn _ptr_safety_comment() {}

        let box_str = self.0.into_boxed_str();
        unsafe { Box::from_raw(Box::into_raw(box_str) as *mut TimestampRef) }
    }

    ///Unwraps the underlying [`String`] value
    #[inline]
    pub fn take(self) -> String { self.0 }
}

impl ::std::convert::From<&'_ TimestampRef> for Timestamp {
    #[inline]
    fn from(s: &TimestampRef) -> Self { ::std::borrow::ToOwned::to_owned(s) }
}

impl ::std::convert::From<Timestamp> for ::std::string::String {
    #[inline]
    fn from(s: Timestamp) -> Self { s.0 }
}

impl ::std::borrow::Borrow<TimestampRef> for Timestamp {
    #[inline]
    fn borrow(&self) -> &TimestampRef { ::std::ops::Deref::deref(self) }
}

impl ::std::convert::AsRef<TimestampRef> for Timestamp {
    #[inline]
    fn as_ref(&self) -> &TimestampRef { ::std::ops::Deref::deref(self) }
}

impl ::std::convert::AsRef<str> for Timestamp {
    #[inline]
    fn as_ref(&self) -> &str { self.as_str() }
}

impl ::std::convert::From<Timestamp> for Box<TimestampRef> {
    #[inline]
    fn from(r: Timestamp) -> Self { r.into_boxed_ref() }
}

impl ::std::convert::From<Box<TimestampRef>> for Timestamp {
    #[inline]
    fn from(r: Box<TimestampRef>) -> Self { r.into_owned() }
}

impl<'a> ::std::convert::From<::std::borrow::Cow<'a, TimestampRef>> for Timestamp {
    #[inline]
    fn from(r: ::std::borrow::Cow<'a, TimestampRef>) -> Self {
        match r {
            ::std::borrow::Cow::Borrowed(b) => ::std::borrow::ToOwned::to_owned(b),
            ::std::borrow::Cow::Owned(o) => o,
        }
    }
}

impl<'a> ::std::convert::From<Timestamp> for ::std::borrow::Cow<'a, TimestampRef> {
    #[inline]
    fn from(owned: Timestamp) -> Self { ::std::borrow::Cow::Owned(owned) }
}

impl ::std::convert::TryFrom<::std::string::String> for Timestamp {
    type Error = TimestampParseError;

    #[inline]
    fn try_from(s: ::std::string::String) -> Result<Self, Self::Error> {
        const fn ensure_try_from_string_error_converts_to_validator_error<
            T: ?Sized + From<<String as ::std::convert::TryFrom<::std::string::String>>::Error>,
        >() {
        }

        ensure_try_from_string_error_converts_to_validator_error::<Self::Error>();
        Self::new(s)
    }
}

impl ::std::convert::TryFrom<&'_ str> for Timestamp {
    type Error = TimestampParseError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let ref_ty = TimestampRef::from_str(s)?;
        Ok(::std::borrow::ToOwned::to_owned(ref_ty))
    }
}

impl ::std::str::FromStr for Timestamp {
    type Err = TimestampParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ref_ty = TimestampRef::from_str(s)?;
        Ok(::std::borrow::ToOwned::to_owned(ref_ty))
    }
}

impl ::std::borrow::Borrow<str> for Timestamp {
    #[inline]
    fn borrow(&self) -> &str { self.as_str() }
}

impl ::std::ops::Deref for Timestamp {
    type Target = TimestampRef;

    #[allow(unsafe_code)]
    #[inline]
    fn deref(&self) -> &Self::Target {
        ///SAFETY: The value was satisfies the type's invariant and conforms to the required implicit contracts of the validator.
        fn _unchecked_safety_comment() {}

        unsafe { TimestampRef::from_str_unchecked(::std::convert::AsRef::as_ref(&self.0)) }
    }
}

impl ::std::fmt::Debug for Timestamp {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        <TimestampRef as ::std::fmt::Debug>::fmt(::std::ops::Deref::deref(self), f)
    }
}

impl ::std::fmt::Display for Timestamp {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        <TimestampRef as ::std::fmt::Display>::fmt(::std::ops::Deref::deref(self), f)
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for Timestamp {
    fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <String as ::serde::Serialize>::serialize(&self.0, serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for Timestamp {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = <String as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
        Self::new(raw).map_err(<D::Error as ::serde::de::Error>::custom)
    }
}
#[repr(transparent)]
#[derive(Hash, PartialEq, Eq)]
/// RFC3339 timestamp
pub struct TimestampRef(str);

impl TimestampRef {
    #[allow(unsafe_code, clippy::should_implement_trait)]
    #[inline]
    ///Transparently reinterprets the string slice as a strongly-typed TimestampRef if it conforms to [`Timestamp`]
    pub fn from_str(raw: &str) -> Result<&Self, TimestampParseError> {
        Self::validate(raw)?;
        ///SAFETY: The value was just checked and found to already conform to the required implicit contracts of the validator.
        fn _unchecked_safety_comment() {}

        Ok(unsafe { Self::from_str_unchecked(raw) })
    }

    #[allow(unsafe_code)]
    #[inline]
    ///Transparently reinterprets the string slice as a strongly-typed TimestampRef without validating
    pub const unsafe fn from_str_unchecked(raw: &str) -> &Self {
        ///SAFETY: `TimestampRef` is `#[repr(transparent)]` around a single `str` field, so a `*const str` can be safely reinterpreted as a `*const TimestampRef`
        fn _ptr_safety_comment() {}

        &*(raw as *const str as *const Self)
    }

    #[inline]
    ///Transparently reinterprets the static string slice as a strongly-typed TimestampRef if it conforms to [`Timestamp`]
    ///
    ///# Panics
    ///
    ///This function will panic if the provided raw string is not valid.
    #[track_caller]
    pub fn from_static(raw: &'static str) -> &'static Self {
        Self::from_str(raw).expect(concat!("invalid ", stringify!(TimestampRef)))
    }

    #[allow(unsafe_code)]
    #[inline]
    ///Converts a [`Box<TimestampRef>`] into a [`Timestamp`] without copying or allocating
    pub fn into_owned(self: Box<TimestampRef>) -> Timestamp {
        ///SAFETY: `TimestampRef` is `#[repr(transparent)]` around a single `str` field, so a `*mut str` can be safely reinterpreted as a `*mut TimestampRef`
        fn _ptr_safety_comment() {}

        let raw = Box::into_raw(self);
        let boxed = unsafe { Box::from_raw(raw as *mut str) };
        let s = ::std::convert::From::from(boxed);
        ///SAFETY: The value was just checked and found to already conform to the required implicit contracts of the validator.
        fn _unchecked_safety_comment() {}

        unsafe { Timestamp::new_unchecked(s) }
    }

    /// Provides access to the underlying value as a string slice.
    #[inline]
    pub const fn as_str(&self) -> &str { &self.0 }
}

impl ::std::borrow::ToOwned for TimestampRef {
    type Owned = Timestamp;

    #[inline]
    fn to_owned(&self) -> Self::Owned { Timestamp(self.0.into()) }
}

impl ::std::cmp::PartialEq<TimestampRef> for Timestamp {
    #[inline]
    fn eq(&self, other: &TimestampRef) -> bool { self.as_str() == other.as_str() }
}

impl ::std::cmp::PartialEq<Timestamp> for TimestampRef {
    #[inline]
    fn eq(&self, other: &Timestamp) -> bool { self.as_str() == other.as_str() }
}

impl ::std::cmp::PartialEq<&'_ TimestampRef> for Timestamp {
    #[inline]
    fn eq(&self, other: &&TimestampRef) -> bool { self.as_str() == other.as_str() }
}

impl ::std::cmp::PartialEq<Timestamp> for &'_ TimestampRef {
    #[inline]
    fn eq(&self, other: &Timestamp) -> bool { self.as_str() == other.as_str() }
}

impl<'a> ::std::convert::TryFrom<&'a str> for &'a TimestampRef {
    type Error = TimestampParseError;

    #[inline]
    fn try_from(s: &'a str) -> Result<&'a TimestampRef, Self::Error> { TimestampRef::from_str(s) }
}

impl ::std::borrow::Borrow<str> for TimestampRef {
    #[inline]
    fn borrow(&self) -> &str { &self.0 }
}

impl ::std::convert::AsRef<str> for TimestampRef {
    #[inline]
    fn as_ref(&self) -> &str { &self.0 }
}

impl<'a> ::std::convert::From<&'a TimestampRef> for ::std::borrow::Cow<'a, TimestampRef> {
    #[inline]
    fn from(r: &'a TimestampRef) -> Self { ::std::borrow::Cow::Borrowed(r) }
}

impl<'a, 'b: 'a> ::std::convert::From<&'a ::std::borrow::Cow<'b, TimestampRef>>
    for &'a TimestampRef
{
    #[inline]
    fn from(r: &'a ::std::borrow::Cow<'b, TimestampRef>) -> &'a TimestampRef {
        ::std::borrow::Borrow::borrow(r)
    }
}

impl ::std::convert::From<&'_ TimestampRef> for ::std::rc::Rc<TimestampRef> {
    #[allow(unsafe_code)]
    #[inline]
    fn from(r: &'_ TimestampRef) -> Self {
        ///SAFETY: `TimestampRef` is `#[repr(transparent)]` around a single `str` field, so a `*const str` can be safely reinterpreted as a `*const TimestampRef`
        fn _ptr_safety_comment() {}

        let rc = ::std::rc::Rc::<str>::from(r.as_str());
        unsafe { ::std::rc::Rc::from_raw(::std::rc::Rc::into_raw(rc) as *const TimestampRef) }
    }
}

impl ::std::convert::From<&'_ TimestampRef> for ::std::sync::Arc<TimestampRef> {
    #[allow(unsafe_code)]
    #[inline]
    fn from(r: &'_ TimestampRef) -> Self {
        ///SAFETY: `TimestampRef` is `#[repr(transparent)]` around a single `str` field, so a `*const str` can be safely reinterpreted as a `*const TimestampRef`
        fn _ptr_safety_comment() {}

        let arc = ::std::sync::Arc::<str>::from(r.as_str());
        unsafe {
            ::std::sync::Arc::from_raw(::std::sync::Arc::into_raw(arc) as *const TimestampRef)
        }
    }
}

impl ::std::fmt::Debug for TimestampRef {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        <str as ::std::fmt::Debug>::fmt(&self.0, f)
    }
}

impl ::std::fmt::Display for TimestampRef {
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        <str as ::std::fmt::Display>::fmt(&self.0, f)
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for TimestampRef {
    fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <str as ::serde::Serialize>::serialize(self.as_str(), serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de: 'a, 'a> ::serde::Deserialize<'de> for &'a TimestampRef {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = <&str as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
        TimestampRef::from_str(raw).map_err(<D::Error as ::serde::de::Error>::custom)
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for Box<TimestampRef> {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let owned = <Timestamp as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
        Ok(owned.into_boxed_ref())
    }
}

impl_extra!(validated, Timestamp, TimestampRef, TimestampParseError);

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Timestamp {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let year = u.int_in_range(0..=9999)?;
        let month = u.int_in_range(1..=12)?;
        const M_D: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let day = u.int_in_range(1..=M_D[month as usize - 1])?;
        let hour = u.int_in_range(0..=23)?;
        let minute = u.int_in_range(0..=59)?;
        let second = u.int_in_range(0..=59)?;
        let millis = if bool::arbitrary(u)? {
            let millis = u.int_in_range(0..=999)?;
            format!(".{millis:03}")
        } else {
            "".to_owned()
        };
        format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}{millis}Z")
            .parse()
            .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

impl TimestampRef {
    fn validate(s: &str) -> Result<(), TimestampParseError> {
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

impl From<std::convert::Infallible> for TimestampParseError {
    fn from(value: std::convert::Infallible) -> Self { match value {} }
}

/// Errors that can occur when parsing a timestamp.
#[derive(Debug)]
#[non_exhaustive]
pub enum TimestampParseError {
    /// Could not parse the timestamp using `time`
    #[cfg(feature = "time")]
    TimeError(time::error::Parse),
    /// Could not format the timestamp using `time`
    #[cfg(feature = "time")]
    TimeFormatError(time::error::Format),
    /// Other error
    Other(&'static str),
    /// Timestamp has an invalid format.
    InvalidFormat {
        /// location of error
        location: &'static std::panic::Location<'static>,
        /// Thing that failed
        s: Option<String>,
    },
}

impl core::fmt::Display for TimestampParseError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        #[allow(unused_variables)]
        match self {
            #[cfg(feature = "time")]
            TimestampParseError::TimeError(parse_error) => {
                write!(formatter, "Could not parse the timestamp using `time`")
            }
            #[cfg(feature = "time")]
            TimestampParseError::TimeFormatError(_) => {
                write!(formatter, "Could not format the timestamp using `time`")
            }
            TimestampParseError::Other(other) => {
                write!(formatter, "{other}")
            }
            TimestampParseError::InvalidFormat { location, s } => {
                write!(
                    formatter,
                    "timestamp has an invalid format. {s:?} - {location}"
                )
            }
        }
    }
}

impl std::error::Error for TimestampParseError {
    fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            #[cfg(feature = "time")]
            TimestampParseError::TimeError { 0: source, .. } => {
                std::option::Option::Some(source as _)
            }
            #[cfg(feature = "time")]
            TimestampParseError::TimeFormatError { 0: source, .. } => {
                std::option::Option::Some(source as _)
            }
            TimestampParseError::Other { .. } => std::option::Option::None,
            TimestampParseError::InvalidFormat { .. } => std::option::Option::None,
        }
    }
}
#[cfg(feature = "time")]
impl std::convert::From<time::error::Parse> for TimestampParseError {
    fn from(source: time::error::Parse) -> Self { TimestampParseError::TimeError(source) }
}
#[cfg(feature = "time")]
impl std::convert::From<time::error::Format> for TimestampParseError {
    fn from(source: time::error::Format) -> Self { TimestampParseError::TimeFormatError(source) }
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
    /// let time = Timestamp::try_from("2021-07-01T13:37:00Z")?;
    /// assert_eq!(time.normalize()?.as_ref(), &time);
    /// let time2 = Timestamp::try_from("2021-07-01T13:37:00-01:00")?;
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
    /// let time2021 = Timestamp::try_from("2021-07-01T13:37:00Z").unwrap();
    /// let time2020 = Timestamp::try_from("2020-07-01T13:37:00Z").unwrap();
    /// assert!(time2020.is_before(&time2021));
    /// ```
    pub fn is_before<T: ?Sized>(&self, other: &T) -> bool
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
    /// let time = Timestamp::try_from("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.to_day().as_str(), "2021-07-01T00:00:00Z");
    /// ```
    pub fn to_day(&self) -> Timestamp {
        let mut c = self.to_owned();
        c.set_time(0, 0, 0);
        c
    }

    /// Get the year
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_types::Timestamp;
    ///
    /// let time = Timestamp::try_from("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.year(), "2021");
    /// ```
    pub fn year(&self) -> &str { &self.0[0..4] }

    /// Get the month
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_types::Timestamp;
    ///
    /// let time = Timestamp::try_from("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.month(), "07");
    /// ```
    pub fn month(&self) -> &str { &self.0[5..7] }

    /// Get the day
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_types::Timestamp;
    ///
    /// let time = Timestamp::try_from("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.day(), "01");
    /// ```
    pub fn day(&self) -> &str { &self.0[8..10] }

    /// Get the hour
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_types::Timestamp;
    ///
    /// let time = Timestamp::try_from("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.hour(), "13");
    /// ```
    pub fn hour(&self) -> &str { &self.0[11..13] }

    /// Get the minute
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_types::Timestamp;
    ///
    /// let time = Timestamp::try_from("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.minute(), "37");
    /// ```
    pub fn minute(&self) -> &str { &self.0[14..16] }

    /// Get the second
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_types::Timestamp;
    ///
    /// let time = Timestamp::try_from("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.second(), "00");
    /// ```
    pub fn second(&self) -> &str { &self.0[17..19] }

    /// Get the millis
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_types::Timestamp;
    ///
    /// let time = Timestamp::try_from("2021-07-01T13:37:00.123Z").unwrap();
    /// assert_eq!(time.millis(), Some("123"));
    /// let time = Timestamp::try_from("2021-07-01T13:37:00Z").unwrap();
    /// assert_eq!(time.millis(), None);
    /// ```
    pub fn millis(&self) -> Option<&str> {
        if self.0[19..].contains('.') {
            let sub = &self.0[20..];
            Some(&sub[..sub.find(|c: char| !c.is_ascii_digit()).unwrap_or(sub.len())])
        } else {
            None
        }
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
        let mut time1 = Timestamp::try_from("2021-11-11T10:00:00Z").unwrap();
        time1.set_time(10, 0, 32);
        let time2 = Timestamp::try_from("2021-11-10T10:00:00Z").unwrap();
        assert!(time2.is_before(&time1));
        dbg!(time1.normalize().unwrap());
        #[cfg(feature = "time")]
        let time = Timestamp::try_from("2021-11-11T13:37:00-01:00").unwrap();
        #[cfg(feature = "time")]
        dbg!(time.normalize().unwrap());
    }
}
