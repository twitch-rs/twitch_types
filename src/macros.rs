/// A basic macro 1.0 implementation of [`aliri_braid`](https://crates.io/crates/aliri_braid)
#[macro_export]
#[doc(hidden)]
macro_rules! manual_braid {
    (
        $(#[$meta:meta])*

        $vis:vis struct $Owned:ident;
        $(#[$_unused_meta:meta])*
        $visref:vis struct $Borrowed:ident;

    ) => {
        manual_braid! {
            @common

            $(#[$meta])*

            $vis struct $Owned;
            $(#[$_unused_meta])*
            $visref struct $Borrowed;
        }

        impl ::std::fmt::Debug for $Owned {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                <$Borrowed as ::std::fmt::Debug>::fmt(::std::ops::Deref::deref(self), f)
            }
        }

        impl ::std::fmt::Debug for $Borrowed {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                <str as ::std::fmt::Debug>::fmt(&self.0, f)
            }
        }
    };

    (
        redact($redacted_type:literal);

        $(#[$meta:meta])*

        $vis:vis struct $Owned:ident;
        $(#[$_unused_meta:meta])*
        $visref:vis struct $Borrowed:ident;

    ) => {
        manual_braid! {
            @common

            $(#[$meta])*

            $vis struct $Owned;
            $(#[$_unused_meta])*
            $visref struct $Borrowed;
        }

        impl ::std::fmt::Debug for $Owned {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(concat!("[redacted ", $redacted_type, "]"))
            }
        }

        impl ::std::fmt::Debug for $Borrowed {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(concat!("[redacted ", $redacted_type, "]"))
            }
        }
    };

    (
        @common
        $(#[$meta:meta])*

        $vis:vis struct $Owned:ident;
        $(#[$_unused_meta:meta])*
        $visref:vis struct $Borrowed:ident;

    ) => {

        $(#[$meta])*
        #[derive(Clone, Hash, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $Owned(String);

        impl $Owned {
            #[doc = concat!("Constructs a new ", stringify!($Owned))]
            #[inline]
            pub const fn new(raw: String) -> Self {
                Self(raw)
            }
            #[inline]
            #[doc = concat!("Constructs a new", stringify!($Owned), "from a static reference")]
            #[track_caller]
            pub fn from_static(raw: &'static str) -> Self {
                ::std::borrow::ToOwned::to_owned($Borrowed::from_static(raw))
            }
            #[doc = concat!("Converts this `", stringify!($Owned), "` into a [`Box<", stringify!($Borrowed), ">`]\n\nThis will drop any excess capacity.")]
            #[allow(unsafe_code)]
            #[inline]
            pub fn into_boxed_ref(self) -> ::std::boxed::Box<$Borrowed> {
                #[doc = concat!("SAFETY: `", stringify!($Borrowed), "` is `#[repr(transparent)]` around a single `str` field, so a `*mut str` can be safely reinterpreted as a `*mut ", stringify!($Borrowed), "`")]
                fn _ptr_safety_comment() {}

                let box_str = self.0.into_boxed_str();
                unsafe {
                    ::std::boxed::Box::from_raw(::std::boxed::Box::into_raw(box_str) as *mut $Borrowed)
                }
            }
            #[doc = "Unwraps the underlying [`String`] value"]
            #[inline]
            pub fn take(self) -> String {
                self.0
            }
        }

        impl ::std::convert::From<&'_ $Borrowed> for $Owned {
            #[inline]
            fn from(s: &$Borrowed) -> Self {
                ::std::borrow::ToOwned::to_owned(s)
            }
        }

        impl ::std::convert::From<$Owned> for ::std::string::String {
            #[inline]
            fn from(s: $Owned) -> Self {
                s.0
            }
        }

        impl ::std::borrow::Borrow<$Borrowed> for $Owned {
            #[inline]
            fn borrow(&self) -> &$Borrowed {
                ::std::ops::Deref::deref(self)
            }
        }

        impl ::std::convert::AsRef<$Borrowed> for $Owned {
            #[inline]
            fn as_ref(&self) -> &$Borrowed {
                ::std::ops::Deref::deref(self)
            }
        }

        impl ::std::convert::AsRef<str> for $Owned {
            #[inline]
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl ::std::convert::From<$Owned> for ::std::boxed::Box<$Borrowed> {
            #[inline]
            fn from(r: $Owned) -> Self {
                r.into_boxed_ref()
            }
        }

        impl ::std::convert::From<::std::boxed::Box<$Borrowed>> for $Owned {
            #[inline]
            fn from(r: ::std::boxed::Box<$Borrowed>) -> Self {
                r.into_owned()
            }
        }

        impl<'a> ::std::convert::From<::std::borrow::Cow<'a, $Borrowed>> for $Owned {
            #[inline]
            fn from(r: ::std::borrow::Cow<'a, $Borrowed>) -> Self {
                match r {
                    ::std::borrow::Cow::Borrowed(b) => ::std::borrow::ToOwned::to_owned(b),
                    ::std::borrow::Cow::Owned(o) => o,
                }
            }
        }

        impl<'a> ::std::convert::From<$Owned> for ::std::borrow::Cow<'a, $Borrowed> {
            #[inline]
            fn from(owned: $Owned) -> Self {
                ::std::borrow::Cow::Owned(owned)
            }
        }

        impl ::std::convert::From<::std::string::String> for $Owned {
            #[inline]
            fn from(s: ::std::string::String) -> Self {
                Self::new(s)
            }
        }

        impl ::std::convert::From<&'_ str> for $Owned {
            #[inline]
            fn from(s: &str) -> Self {
                Self::new(::std::convert::From::from(s))
            }
        }

        impl ::std::convert::From<::std::boxed::Box<str>> for $Owned {
            #[inline]
            fn from(s: ::std::boxed::Box<str>) -> Self {
                Self::new(::std::convert::From::from(s))
            }
        }

        impl ::std::str::FromStr for $Owned {
            type Err = ::std::convert::Infallible;
            #[inline]
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                ::std::result::Result::Ok(::std::convert::From::from(s))
            }
        }

        impl ::std::borrow::Borrow<str> for $Owned {
            #[inline]
            fn borrow(&self) -> &str {
                self.as_str()
            }
        }

        impl ::std::ops::Deref for $Owned {
            type Target = $Borrowed;
            #[inline]
            fn deref(&self) -> &Self::Target {
                $Borrowed::from_str(::std::convert::AsRef::as_ref(&self.0))
            }
        }

        impl ::std::fmt::Display for $Owned {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                <$Borrowed as ::std::fmt::Display>::fmt(::std::ops::Deref::deref(self), f)
            }
        }

        impl ::std::cmp::Ord for $Owned {
            #[inline]
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                ::std::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        // TODO(2023-07-31): false-positive, remove when rust-lang/rust-clippy#11188 is on nightly
        impl ::std::cmp::PartialOrd for $Owned {
            #[inline]
            #[allow(unknown_lints)]
            #[allow(clippy::non_canonical_partial_ord_impl)]
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                ::std::option::Option::Some(::std::cmp::Ord::cmp(self, other))
            }
        }

        #[cfg(feature = "serde")]
        impl ::serde::Serialize for $Owned {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                <String as ::serde::Serialize>::serialize(&self.0, serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for $Owned {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let raw = <String as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
                Ok(Self::new(raw))
            }
        }

        $(#[$meta])*
        #[repr(transparent)]
        #[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $Borrowed(str);

        impl $Borrowed {
            #[allow(unsafe_code)]
            #[inline]
            #[doc = concat!("Transparently reinterprets the string slice as a strongly-typed ", stringify!($Borrowed))]
            pub const fn from_str(raw: &str) -> &Self {
                let ptr: *const str = raw;
                #[doc = concat!("SAFETY: `", stringify!($Borrowed), "` is `#[repr(transparent)]` around a single `str` field, so a `*const str` can be safely reinterpreted as a `*const ", stringify!($Borrowed), "`")]
                fn _ptr_safety_comment() {}

                unsafe { &*(ptr as *const Self) }
            }
            #[inline]
            #[doc = concat!("Transparently reinterprets the static string slice as a strongly-typed ", stringify!($Borrowed))]
            #[track_caller]
            pub const fn from_static(raw: &'static str) -> &'static Self {
                Self::from_str(raw)
            }
            #[allow(unsafe_code)]
            #[inline]
            #[doc = concat!("Converts a [`Box<", stringify!($Borrowed), ">`] into a [`", stringify!($Owned), "`] without copying or allocating")]
            pub fn into_owned(self: ::std::boxed::Box<$Borrowed>) -> $Owned {
                #[doc = concat!("SAFETY: `", stringify!($Borrowed), "` is `#[repr(transparent)]` around a single `str` field, so a `*mut str` can be safely reinterpreted as a `*mut ", stringify!($Borrowed), "`")]
                fn _ptr_safety_comment() {}

                let raw = ::std::boxed::Box::into_raw(self);
                let boxed = unsafe { ::std::boxed::Box::from_raw(raw as *mut str) };
                $Owned::new(::std::convert::From::from(boxed))
            }
            #[doc = r" Provides access to the underlying value as a string slice."]
            #[inline]
            pub const fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl ::std::borrow::ToOwned for $Borrowed {
            type Owned = $Owned;
            #[inline]
            fn to_owned(&self) -> Self::Owned {
                $Owned(self.0.into())
            }
        }

        impl ::std::cmp::PartialEq<$Borrowed> for $Owned {
            #[inline]
            fn eq(&self, other: &$Borrowed) -> bool {
                self.as_str() == other.as_str()
            }
        }

        impl ::std::cmp::PartialEq<&'_ $Owned> for $Owned {
            #[inline]
            fn eq(&self, other: &&$Owned) -> bool {
                self.as_str() == other.as_str()
            }
        }

        impl ::std::cmp::PartialEq<$Owned> for $Borrowed {
            #[inline]
            fn eq(&self, other: &$Owned) -> bool {
                self.as_str() == other.as_str()
            }
        }

        impl ::std::cmp::PartialEq<&'_ $Borrowed> for $Owned {
            #[inline]
            fn eq(&self, other: &&$Borrowed) -> bool {
                self.as_str() == other.as_str()
            }
        }

        impl ::std::cmp::PartialEq<$Owned> for &'_ $Borrowed {
            #[inline]
            fn eq(&self, other: &$Owned) -> bool {
                self.as_str() == other.as_str()
            }
        }

        impl<'a> ::std::convert::From<&'a str> for &'a $Borrowed {
            #[inline]
            fn from(s: &'a str) -> &'a $Borrowed {
                $Borrowed::from_str(s)
            }
        }

        impl ::std::borrow::Borrow<str> for $Borrowed {
            #[inline]
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl ::std::convert::AsRef<str> for $Borrowed {
            #[inline]
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl<'a> ::std::convert::From<&'a $Borrowed> for ::std::borrow::Cow<'a, $Borrowed> {
            #[inline]
            fn from(r: &'a $Borrowed) -> Self {
                ::std::borrow::Cow::Borrowed(r)
            }
        }

        impl<'a, 'b: 'a> ::std::convert::From<&'a ::std::borrow::Cow<'b, $Borrowed>>
            for &'a $Borrowed
        {
            #[inline]
            fn from(r: &'a ::std::borrow::Cow<'b, $Borrowed>) -> &'a $Borrowed {
                ::std::borrow::Borrow::borrow(r)
            }
        }

        impl ::std::convert::From<&'_ $Borrowed> for ::std::rc::Rc<$Borrowed> {
            #[allow(unsafe_code)]
            #[inline]
            fn from(r: &'_ $Borrowed) -> Self {
                #[doc = concat!("SAFETY: `", stringify!($Borrowed), "` is `#[repr(transparent)]` around a single `str` field, so a `*const str` can be safely reinterpreted as a `*const ", stringify!($Borrowed), "`")]
                fn _ptr_safety_comment() {}

                let rc = ::std::rc::Rc::<str>::from(r.as_str());
                unsafe { ::std::rc::Rc::from_raw(::std::rc::Rc::into_raw(rc) as *const $Borrowed) }
            }
        }

        impl ::std::convert::From<&'_ $Borrowed> for ::std::sync::Arc<$Borrowed> {
            #[allow(unsafe_code)]
            #[inline]
            fn from(r: &'_ $Borrowed) -> Self {
                #[doc = concat!("SAFETY: `", stringify!($Borrowed), "` is `#[repr(transparent)]` around a single `str` field, so a `*const str` can be safely reinterpreted as a `*const ", stringify!($Borrowed), "`")]
                fn _ptr_safety_comment() {}

                let arc = ::std::sync::Arc::<str>::from(r.as_str());
                unsafe {
                    ::std::sync::Arc::from_raw(::std::sync::Arc::into_raw(arc) as *const $Borrowed)
                }
            }
        }

        impl ::std::fmt::Display for $Borrowed {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                <str as ::std::fmt::Display>::fmt(&self.0, f)
            }
        }

        #[cfg(feature = "serde")]
        impl ::serde::Serialize for $Borrowed {
            fn serialize<S: ::serde::Serializer>(
                &self,
                serializer: S,
            ) -> ::std::result::Result<S::Ok, S::Error> {
                <str as ::serde::Serialize>::serialize(self.as_str(), serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de: 'a, 'a> ::serde::Deserialize<'de> for &'a $Borrowed {
            fn deserialize<D: ::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> ::std::result::Result<Self, D::Error> {
                let raw = <&str as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
                ::std::result::Result::Ok($Borrowed::from_str(raw))
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for ::std::boxed::Box<$Borrowed> {
            fn deserialize<D: ::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> ::std::result::Result<Self, D::Error> {
                let owned = <$Owned as ::serde::Deserialize<'de>>::deserialize(deserializer)?;
                ::std::result::Result::Ok(owned.into_boxed_ref())
            }
        }

        impl<'a> From<&'a Vec<&'a $Owned>>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a Vec<&'a $Owned>) -> Self { Self::Borrowed(::std::borrow::Cow::from(v)) }
        }

        impl<'a> From<&'a Vec<&'a str>>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a Vec<&'a str>) -> Self { Self::RefStr(::std::borrow::Cow::from(v)) }
        }

        impl<'a> From<&'a Vec<String>>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a Vec<String>) -> Self { Self::OwnedString(::std::borrow::Cow::from(v)) }
        }

        impl<'a> From<Vec<&'a $Borrowed>>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: Vec<&'a $Borrowed>) -> Self { Self::Ref(::std::borrow::Cow::from(v)) }
        }

        impl<'a> From<&'a [&'a $Borrowed]>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a [&'a $Borrowed]) -> Self { Self::Ref(::std::borrow::Cow::from(v)) }
        }

        impl From<Vec<String>>
            for $crate::collection::Collection<'_, $Owned>
        {
            fn from(v: Vec<String>) -> Self { Self::OwnedString(::std::borrow::Cow::from(v)) }
        }

        impl<'a> From<&'a [String]>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a [String]) -> Self { Self::OwnedString(::std::borrow::Cow::from(v)) }
        }

        impl<'a> From<Vec<&'a String>>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: Vec<&'a String>) -> Self { Self::BorrowedString(::std::borrow::Cow::from(v)) }
        }

        impl<'a> From<&'a [&'a String]>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a [&'a String]) -> Self { Self::BorrowedString(::std::borrow::Cow::from(v)) }
        }

        impl<'a> From<Vec<&'a str>>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: Vec<&'a str>) -> Self { Self::RefStr(::std::borrow::Cow::from(v)) }
        }

        impl<'a> From<&'a [&'a str]>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a [&'a str]) -> Self { Self::RefStr(::std::borrow::Cow::from(v)) }
        }

        impl<'a, const N: usize> From<&'a [&'a str; N]>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a [&'a str; N]) -> Self { Self::RefStr(::std::borrow::Cow::from(v.as_slice())) }
        }

        impl<'a, const N: usize> From<&'a [$Owned; N]>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a [$Owned; N]) -> Self { Self::Owned(::std::borrow::Cow::from(v.as_slice())) }
        }

        impl<'a, const N: usize> From<&'a [String; N]>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a [String; N]) -> Self { Self::OwnedString(::std::borrow::Cow::from(v.as_slice())) }
        }

        impl<'a, const N: usize> From<&'a [&'a $Owned; N]>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a [&'a $Owned; N]) -> Self { Self::Borrowed(::std::borrow::Cow::from(v.as_slice())) }
        }

        impl<'a, const N: usize> From<&'a [&'a $Borrowed; N]>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a [&'a $Borrowed; N]) -> Self { Self::Ref(::std::borrow::Cow::Borrowed(v.as_slice())) }
        }

        impl<'a> From<&'a $Owned>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a $Owned) -> Self { Self::Owned(::std::borrow::Cow::from(std::slice::from_ref(v))) }
        }

        impl<'a> From<&'a &'a $Borrowed>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a &'a $Borrowed) -> Self { Self::Ref(::std::borrow::Cow::from(std::slice::from_ref(v))) }
        }

        impl<'a> From<&'a &'a str>
            for $crate::collection::Collection<'a, $Owned>
        {
            fn from(v: &'a &'a str) -> Self { Self::RefStr(::std::borrow::Cow::from(std::slice::from_ref(v))) }
        }
    }
}

macro_rules! impl_extra {
    (validated, $owned:path, $ref:path, $error:path) => {
        impl<'a> TryFrom<&'a String> for &'a $ref {
            type Error = $error;
            fn try_from(string: &'a String) -> Result<Self, $error>  {
                <$ref>::from_str(string.as_str())
            }
        }

        impl_extra!(@all, $owned, $ref);
    };
    (no_arb, $owned:path, $ref:path) => {
        impl<'a> From<&'a String> for &'a $ref {
            fn from(string: &'a String) -> Self {
                <$ref>::from_str(string.as_str())
            }
        }

        impl_extra!(@all, $owned, $ref);
    };

    (ascii, $owned:path, $ref:path) => {
        impl<'a> From<&'a String> for &'a $ref {
            fn from(string: &'a String) -> Self {
                <$ref>::from_str(string.as_str())
            }
        }

        #[cfg(feature = "arbitrary")]
        impl<'a> arbitrary::Arbitrary<'a> for &'a $ref {
            fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                let string: &str = <&str as arbitrary::Arbitrary>::arbitrary(u)?;
                let string = if let Some(i) = string.find(|b: char| !b.is_ascii_alphanumeric()) {
                    let valid = &string[0..i];
                    valid
                } else {
                    string
                };
                if string.is_empty() {
                    Err(arbitrary::Error::IncorrectFormat)
                } else {
                    Ok(string.into())
                }
            }

            fn arbitrary_take_rest(u: arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                let string: &str = <&str as arbitrary::Arbitrary>::arbitrary_take_rest(u)?;
                if  string.as_bytes().iter().any(|b| !b.is_ascii_alphanumeric())|| string.is_empty() {
                    Err(arbitrary::Error::IncorrectFormat)
                } else {
                    Ok(string.into())
                }
            }

            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                <&str as arbitrary::Arbitrary>::size_hint(depth)
            }
        }

        #[cfg(feature = "arbitrary")]
        impl<'a> arbitrary::Arbitrary<'a> for $owned {
            fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                <&$ref as arbitrary::Arbitrary>::arbitrary(u).map(Into::into)
            }

            fn arbitrary_take_rest(u: arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                <&$ref as arbitrary::Arbitrary>::arbitrary_take_rest(u).map(Into::into)
            }

            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                <&$ref as arbitrary::Arbitrary>::size_hint(depth)
            }
        }

        impl_extra!(@all, $owned, $ref);
    };

    (numeric, $owned:path, $ref:path) => {
        impl<'a> From<&'a String> for &'a $ref {
            fn from(string: &'a String) -> Self {
                <$ref>::from_str(string.as_str())
            }
        }

        #[cfg(feature = "arbitrary")]
        impl<'a> arbitrary::Arbitrary<'a> for &'a $ref {
            fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                let string: &str = <&str as arbitrary::Arbitrary>::arbitrary(u)?;
                let string = if let Some(i) = string.find(|b: char| !b.is_ascii_digit()) {
                    let valid = &string[0..i];
                    valid
                } else {
                    string
                };
                if string.is_empty() {
                    Err(arbitrary::Error::IncorrectFormat)
                } else {
                    Ok(string.into())
                }
            }

            fn arbitrary_take_rest(u: arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                let string: &str = <&str as arbitrary::Arbitrary>::arbitrary_take_rest(u)?;
                if  string.as_bytes().iter().any(|b| !b.is_ascii_digit()) || string.is_empty() {
                    Err(arbitrary::Error::IncorrectFormat)
                } else {
                    Ok(string.into())
                }
            }

            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                <&str as arbitrary::Arbitrary>::size_hint(depth)
            }
        }

        #[cfg(feature = "arbitrary")]
        impl<'a> arbitrary::Arbitrary<'a> for $owned {
            fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                <u32 as arbitrary::Arbitrary>::arbitrary(u).map(|i| format!("{i}").into())
            }

            fn arbitrary_take_rest(u: arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                <u32 as arbitrary::Arbitrary>::arbitrary_take_rest(u).map(|i| format!("{i}").into())
            }

            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                <&$ref as arbitrary::Arbitrary>::size_hint(depth)
            }
        }

        impl_extra!(@all, $owned, $ref);
    };

    ($owned:path, $ref:path) => {
        impl<'a> From<&'a String> for &'a $ref {
            fn from(string: &'a String) -> Self {
                <$ref>::from_str(string.as_str())
            }
        }

        #[cfg(feature = "arbitrary")]
        impl<'a> arbitrary::Arbitrary<'a> for &'a $ref {
            fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                <&str as arbitrary::Arbitrary>::arbitrary(u).map(Into::into)
            }

            fn arbitrary_take_rest(u: arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                <&str as arbitrary::Arbitrary>::arbitrary_take_rest(u).map(Into::into)
            }

            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                <&str as arbitrary::Arbitrary>::size_hint(depth)
            }
        }

        #[cfg(feature = "arbitrary")]
        impl<'a> arbitrary::Arbitrary<'a> for $owned {
            fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                <&$ref as arbitrary::Arbitrary>::arbitrary(u).map(Into::into)
            }

            fn arbitrary_take_rest(u: arbitrary::Unstructured<'a>) -> Result<Self, arbitrary::Error> {
                <&$ref as arbitrary::Arbitrary>::arbitrary_take_rest(u).map(Into::into)
            }

            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                <&$ref as arbitrary::Arbitrary>::size_hint(depth)
            }
        }

        impl_extra!(@all, $owned, $ref);
    };
    (@all, $owned:path, $ref:path) => {
        impl $ref {
            /// Get a
            #[doc = concat!("[`Cow<'_, ", stringify!($ref), ">`](std::borrow::Cow::Borrowed)")]
            pub fn as_cow(&self) -> ::std::borrow::Cow<'_, $ref> {
                self.into()
            }
        }

        #[cfg(feature = "zerofrom")]
        impl<'zf> zerofrom::ZeroFrom<'zf, $ref> for &'zf $ref {
            #[inline]
            fn zero_from(other: &'zf $ref) -> Self {
                other
            }
        }

        #[cfg(feature = "zerofrom")]
        impl<'zf> zerofrom::ZeroFrom<'zf, $owned> for &'zf $ref {
            #[inline]
            fn zero_from(other: &'zf $owned) -> Self {
                other
            }
        }

        impl<'a> From<&'a $owned> for &'a $ref {
            fn from(owned: &'a $owned) -> Self {
                &*owned
            }
        }

        impl<'a> From<&'a $owned> for ::std::borrow::Cow<'a, $ref> {
            fn from(owned: &'a $owned) -> Self {
                ::std::borrow::Cow::Borrowed(&*owned)
            }
        }

        impl<'a> crate::IntoCow<'a, $ref> for &'a $ref {
            fn into_cow(self) -> ::std::borrow::Cow<'a, $ref> {
                ::std::borrow::Cow::Borrowed(self)
            }
        }

        impl<'a> crate::IntoCow<'a, $ref> for $owned {
            fn into_cow(self) -> ::std::borrow::Cow<'a, $ref> {
                ::std::borrow::Cow::Owned(self)
            }
        }

        impl<'a> crate::IntoCow<'a, $ref> for &'a $owned {
            fn into_cow(self) -> ::std::borrow::Cow<'a, $ref> {
                ::std::borrow::Cow::Borrowed(self.as_ref())
            }
        }

        const _: () = {
            #[cfg(feature = "arbitrary")]
            fn assert_arbitrary() {
                fn assert<'a, T: arbitrary::Arbitrary<'a>>() {}
                // XXX: Not asserting arbitrary is implemented for borrowed, this is because a validated type might need owned data.
                // assert::<&$ref>();
                assert::<$owned>();
            }

            #[cfg(feature = "zerofrom")]
            fn assert_zerofrom() {
                fn assert_borrowed<'zf, T: zerofrom::ZeroFrom<'zf, $ref>>() {}
                fn assert_owned<'zf, T: zerofrom::ZeroFrom<'zf, $owned>>() {}
                assert_borrowed::<&$ref>();
                assert_owned::<&$ref>();
            }
        };
    };
}
