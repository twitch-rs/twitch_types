#![allow(warnings, missing_docs)]
use std::borrow::Cow;

/// Generic collection of an abstracted item.
pub enum Collection<'a, T: std::ops::Deref>
where [T]: ToOwned {
    /// A collection over owned items
    Owned(Cow<'a, [T]>),
    /// A collection over borrowed items
    Borrowed(Cow<'a, [&'a T]>),
    /// A collection over deref items
    Ref(Cow<'a, [&'a T::Target]>),
    /// A collection over owned string items
    OwnedString(Cow<'a, [String]>),
    /// A collection over borrowed string items
    BorrowedString(Cow<'a, [&'a String]>),
    /// A collection over &str items
    RefStr(Cow<'a, [&'a str]>),
}

impl<'a, T: std::ops::Deref + std::fmt::Debug> std::fmt::Debug for Collection<'a, T>
where
    [T]: ToOwned,
    <[T] as ToOwned>::Owned: std::fmt::Debug,
    T: std::fmt::Debug,
    T::Target: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Owned(owned) => f.debug_tuple("Owned").field(owned).finish(),
            Self::Borrowed(borrowed) => f.debug_tuple("Borrowed").field(borrowed).finish(),
            Self::Ref(ref_) => f.debug_tuple("Ref").field(ref_).finish(),
            Self::OwnedString(owned) => f.debug_tuple("OwnedString").field(owned).finish(),
            Self::BorrowedString(borrowed) => {
                f.debug_tuple("BorrowedString").field(borrowed).finish()
            }
            Self::RefStr(ref_) => f.debug_tuple("RefStr").field(ref_).finish(),
        }
    }
}

impl<'a, T: std::ops::Deref> Collection<'a, T> where [T]: ToOwned {}

impl<'a, T: std::ops::Deref> Collection<'a, T>
where
    [T]: ToOwned,
    for<'t> &'t T::Target: From<&'t str>,
{
    fn iter<'s>(&'s self) -> CollectionIter<'s, T>
where {
        match self {
            Collection::Owned(v) => CollectionIter {
                inner: CollectionIterInner::Owned(v.as_ref()),
            },
            Collection::Borrowed(v) => CollectionIter {
                inner: CollectionIterInner::Borrowed(v.as_ref()),
            },
            Collection::Ref(v) => CollectionIter {
                inner: CollectionIterInner::Ref(v.as_ref()),
            },
            Collection::OwnedString(v) => {
                let mut iter = Box::new(v.iter().map(|v| <_>::from(v.as_str())));
                CollectionIter {
                    inner: CollectionIterInner::String(iter),
                }
            }
            Collection::BorrowedString(v) => {
                let mut iter = Box::new(v.iter().map(|&v| <_>::from(v.as_str())));
                CollectionIter {
                    inner: CollectionIterInner::String(iter),
                }
            }
            Collection::RefStr(v) => {
                let mut iter = Box::new(v.iter().map(move |&v| <_>::from(v)));
                CollectionIter {
                    inner: CollectionIterInner::String(iter),
                }
            }
        }
    }

    fn into_owned(self) -> Vec<T>
    where
        [T]: ToOwned<Owned = Vec<T>>,
        T: 'static,
        for<'d> T: From<&'d <T as std::ops::Deref>::Target>, {
        match self {
            Collection::Owned(v) => v.into_owned(),
            _ => self.iter().map(|v| T::from(v)).collect(),
        }
    }

    fn index(&self, range: impl std::ops::RangeBounds<usize>) -> Option<Collection<'_, T>> {
        let range = (range.start_bound().cloned(), range.end_bound().cloned());
        let new = match self {
            Collection::Owned(v) => Collection::Owned(Cow::Borrowed(v.get(range)?)),
            Collection::Borrowed(v) => Collection::Borrowed(Cow::Borrowed(v.get(range)?)),
            Collection::Ref(v) => Collection::Ref(Cow::Borrowed(v.get(range)?)),
            Collection::OwnedString(v) => Collection::OwnedString(Cow::Borrowed(v.get(range)?)),
            Collection::BorrowedString(v) => {
                Collection::BorrowedString(Cow::Borrowed(v.get(range)?))
            }
            Collection::RefStr(v) => Collection::RefStr(Cow::Borrowed(v.get(range)?)),
        };
        Some(new)
    }

    /// Returns chunks of i
    pub fn chunks<'s: 'a>(
        &'s self,
        chunk_size: usize,
    ) -> impl Iterator<Item = Collection<'s, T>> + 's {
        let len = self.iter().len();
        let mut start = 0;
        std::iter::from_fn(move || {
            if start >= len {
                return None;
            }
            let end = start + chunk_size;
            let end = if end > len { len } else { end };
            let range = start..end;
            start = end;
            self.index(range)
        })
    }
}

impl<T: std::ops::Deref + Clone> From<Vec<T>> for Collection<'_, T>
where
    [T]: ToOwned,
    T: 'static,
{
    fn from(v: Vec<T>) -> Self { Self::Owned(Cow::from(v)) }
}

impl<'c, T: std::ops::Deref + Clone> From<Vec<&'c T>> for Collection<'c, T>
where
    [T]: ToOwned,
    T: 'static,
{
    fn from(v: Vec<&'c T>) -> Self { Self::Borrowed(Cow::from(v)) }
}

//
impl<'c, T: std::ops::Deref + Clone> From<&'c [T]> for Collection<'c, T>
where
    [T]: ToOwned,
    T: 'static,
{
    fn from(v: &'c [T]) -> Self { Self::Owned(Cow::Borrowed(v)) }
}

impl<'c, T: std::ops::Deref + Clone> From<&'c [&'c T]> for Collection<'c, T>
where
    [T]: ToOwned,
    T: 'static,
{
    fn from(v: &'c [&'c T]) -> Self { Self::Borrowed(Cow::from(v)) }
}

impl<'a, T: std::ops::Deref> IntoIterator for &'a Collection<'_, T>
where
    [T]: ToOwned,
    for<'t> &'t T::Target: From<&'t str>,
{
    type IntoIter = CollectionIter<'a, T>;
    type Item = &'a T::Target;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

pub struct CollectionIter<'c, T: std::ops::Deref>
where [T]: ToOwned {
    inner: CollectionIterInner<'c, T>,
}

enum CollectionIterInner<'c, T: std::ops::Deref>
where [T]: ToOwned {
    Ref(&'c [&'c T::Target]),
    Borrowed(&'c [&'c T]),
    Owned(&'c [T]),
    String(Box<dyn std::iter::ExactSizeIterator<Item = &'c T::Target> + 'c>),
}

impl<'a, T: std::ops::Deref> Iterator for CollectionIter<'a, T>
where [T]: ToOwned
{
    type Item = &'a T::Target;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            CollectionIterInner::Ref(ref_) => {
                let v = ref_.first()?;
                *ref_ = &ref_[1..];
                Some(v)
            }
            CollectionIterInner::Borrowed(borrowed) => {
                let v = borrowed.first()?;
                *borrowed = &borrowed[1..];
                Some(v)
            }
            CollectionIterInner::Owned(owned) => {
                let v = owned.first()?;
                *owned = &owned[1..];
                Some(v)
            }
            CollectionIterInner::String(b) => b.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (self.len(), Some(self.len())) }

    fn count(self) -> usize
    where Self: Sized {
        self.len()
    }

    fn last(self) -> Option<Self::Item>
    where Self: Sized {
        match self.inner {
            CollectionIterInner::Ref(v) => v.last().copied(),
            CollectionIterInner::Borrowed(v) => v.last().map(|v| v.deref()),
            CollectionIterInner::Owned(v) => v.last().map(|v| v.deref()),
            CollectionIterInner::String(b) => b.last(),
        }
    }
}

impl<T: std::ops::Deref> CollectionIter<'_, T>
where [T]: ToOwned
{
    fn len(&self) -> usize {
        match self.inner {
            CollectionIterInner::Ref(v) => v.len(),
            CollectionIterInner::Borrowed(v) => v.len(),
            CollectionIterInner::Owned(v) => v.len(),
            CollectionIterInner::String(ref b) => b.len(),
        }
    }
}

#[cfg(feature = "serde")]
impl<'a, 'de: 'a, T: std::ops::Deref + serde::Deserialize<'de> + Clone> serde::Deserialize<'de>
    for Collection<'a, T>
where
    [T]: ToOwned,
    &'a T::Target: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let collection: Vec<T> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Collection::Owned(Cow::from(collection)))
    }
}

#[cfg(feature = "serde")]
impl<T: std::ops::Deref> serde::Serialize for Collection<'_, T>
where
    [T]: ToOwned,
    for<'a> &'a T::Target: serde::Serialize,
    for<'s> &'s T::Target: From<&'s str>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.collect_seq(self.iter())
    }
}

//#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
impl<T: Clone + std::ops::Deref> Clone for Collection<'_, T>
where [T]: ToOwned
{
    fn clone(&self) -> Self {
        match self {
            Collection::Owned(v) => Collection::Owned(v.clone()),
            Collection::Borrowed(v) => Collection::Borrowed(v.clone()),
            Collection::Ref(v) => Collection::Ref(v.clone()),
            Collection::OwnedString(v) => Collection::OwnedString(v.clone()),
            Collection::BorrowedString(v) => Collection::BorrowedString(v.clone()),
            Collection::RefStr(v) => Collection::RefStr(v.clone()),
        }
    }
}

impl<'t, T: std::ops::Deref> PartialEq for Collection<'t, T>
where
    [T]: ToOwned,
    T: PartialEq,
    T::Target: PartialEq,
    T: std::cmp::PartialEq<T::Target>,
    for<'s> &'s T::Target: From<&'s str>,
    T::Target: std::fmt::Debug,
    T: std::fmt::Debug,
{
    #[rustfmt::skip]
    fn eq(&self, other: &Self) -> bool {
        use self::Collection::*;
        match (self, other) {
            (Owned(v), Owned(v2)) => v == v2,
            (Borrowed(v), Borrowed(v2)) => v == v2,
            (Ref(v), Ref(v2)) => v == v2,
            (OwnedString(v), OwnedString(v2)) => v == v2,
            (BorrowedString(v), BorrowedString(v2)) => v == v2,
            (RefStr(v), RefStr(v2)) => v == v2,

            // (Owned(owned), Borrowed(borrowed)) | (Borrowed(borrowed), Owned(owned)) => borrowed == owned,
            // (Ref(ref_), Owned(owned)) | (Owned(owned), Ref(ref_)) => owned == ref_,
            (Borrowed(borrowed), Ref(ref_)) | (Ref(ref_), Borrowed(borrowed)) => borrowed == ref_,

            // etc for strings
            _ => {
                let _: Vec<_> = dbg!(self.iter().collect());
                let _: Vec<_> = dbg!(other.iter().collect());
                dbg!(self.iter().eq(other.iter()))},
        }
    }
}

impl<'t, T: std::ops::Deref> Eq for Collection<'t, T>
where
    [T]: ToOwned,
    T: PartialEq,
    T::Target: PartialEq,
    T: std::cmp::PartialEq<T::Target>,
    for<'s> &'s T::Target: From<&'s str>,
    T::Target: std::fmt::Debug,
    T: std::fmt::Debug,
{
}

impl<T: std::ops::Deref> Default for Collection<'_, T>
where
    [T]: ToOwned,
    <[T] as ToOwned>::Owned: Default,
{
    fn default() -> Self { Self::Ref(Cow::default()) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collection() {
        let owned: Vec<_> = vec![crate::UserId::from("1234"), crate::UserId::from("5678")];
        let borrow = (&crate::UserId::from("1234"), &crate::UserId::from("5678"));
        let borrowed: Vec<_> = vec![borrow.0, borrow.1];
        let ref_: Vec<_> = vec![
            crate::UserIdRef::from_str("1234"),
            crate::UserIdRef::from_str("5678"),
        ];
        let collection_owned = Collection::from(owned.clone());
        let collection_borrowed = Collection::from(borrowed.clone());
        let collection_ref = Collection::from(ref_);
        assert_eq!(collection_owned, collection_borrowed);
        assert_eq!(collection_owned, collection_ref);
        assert_eq!(collection_borrowed, collection_ref);
        assert_eq!(collection_owned.iter().collect::<Vec<_>>(), owned);
        assert_eq!(collection_borrowed.iter().collect::<Vec<_>>(), owned);
        assert_eq!(collection_ref.iter().collect::<Vec<_>>(), owned);
        assert_eq!(collection_ref.iter().last(), Some("5678".into()));
    }

    #[test]
    fn test_collection_stringed() {
        let owned_s: Vec<_> = vec![String::from("1234"), String::from("5678")];
        let borrow_s = (&String::from("1234"), &String::from("5678"));
        let borrowed_s: Vec<_> = vec![borrow_s.0, borrow_s.1];
        let ref_s: Vec<_> = vec!["1234", "5678"];
        let ref_: Vec<_> = vec![
            crate::UserIdRef::from_str("1234"),
            crate::UserIdRef::from_str("5678"),
        ];
        let collection_owned_s: Collection<crate::UserId> = Collection::from(owned_s.clone());
        let collection_borrowed_s: Collection<crate::UserId> = Collection::from(borrowed_s.clone());
        let collection_ref_s: Collection<crate::UserId> = Collection::from(ref_s);
        let collection_ref: Collection<crate::UserId> = Collection::from(ref_);
        assert_eq!(collection_owned_s, collection_borrowed_s);
        assert_eq!(collection_owned_s, collection_ref_s);
        assert_eq!(collection_borrowed_s, collection_ref_s);
        assert_eq!(collection_ref, collection_ref_s);
        assert_eq!(collection_ref_s.iter().last(), Some("5678".into()));
    }
}
