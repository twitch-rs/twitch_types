use std::{borrow::{Cow, Borrow}, sync::Arc};

/// Generic collection of an abstracted item.
///
/// This is used to abstract over the different types of collections that can be used,
/// such as `Vec<T>`, `&[T]`, `&[&T]`, `&[&str]`, etc.
///
/// In most cases, you can use the [`Collection::from`] method to create a collection.
/// If you get an error
///
/// # Examples
///
/// ```rust
/// use twitch_types::{Collection, UserId, UserIdRef};
///
/// // A vector of `UserId`s
/// let c0: Collection<UserId> = Collection::from(vec![UserId::from("1234"), UserId::from("5678")]);
/// // A vector of `&str`s
/// let c1: Collection<UserId> = Collection::from(vec!["1234", "5678"]);
/// // An array of `&str`s
/// let c2: Collection<UserId> = Collection::from(&["1234", "5678"]);
/// // A vector of `UserIdRef`s
/// let c3: Collection<UserId> = Collection::from(vec![
///     UserIdRef::from_static("1234"),
///     UserIdRef::from_static("5678"),
/// ]);
///
/// assert!([c1, c2, c3].iter().all(|c| *c == c0));
/// ```
pub enum Collection<'c, T: std::ops::Deref + 'static>
where [T]: ToOwned {
    /// A collection over owned items
    Owned(Arc<[T]>),
    /// A collection over borrowed items
    Borrowed(Cow<'c, [&'c T]>),
    /// A collection over deref items
    Ref(Cow<'c, [&'c T::Target]>),
    /// A collection over owned string items
    OwnedString(Arc<[String]>),
    /// A collection over borrowed string items
    BorrowedString(Cow<'c, [&'c String]>),
    /// A collection over &str items
    RefStr(Cow<'c, [&'c str]>),
}

impl<'c, T: std::ops::Deref> Collection<'c, T>
where [T]: ToOwned
{
    /// Returns an iterator over the collection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_types::{Collection, UserId, UserIdRef};
    ///
    /// let collection: Collection<UserId> = Collection::from(&["1234", "5678"]);
    /// let mut iter = collection.iter();
    /// assert_eq!(iter.next(), Some(UserIdRef::from_static("1234")));
    /// assert_eq!(iter.next(), Some(UserIdRef::from_static("5678")));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&'_ self) -> CollectionIter<'c, T>
    where
        for<'s> &'s T::Target: From<&'s str>,
        for<'a> T: Borrow<T::Target>, {
        match self {
            Collection::Owned(v) => CollectionIter {
                inner: CollectionIterInner::Owned(v.clone()),
            },
            Collection::Borrowed(Cow::Owned(v)) => CollectionIter {
                inner: CollectionIterInner::Borrowed(todo!()),
            },
            Collection::Borrowed(Cow::Borrowed(v)) => CollectionIter {
                inner: CollectionIterInner::Borrowed(v),
            },
            Collection::Ref(Cow::Owned(v)) => CollectionIter {
                inner: CollectionIterInner::Ref(todo!()),
            },
            Collection::Ref(Cow::Borrowed(v)) => CollectionIter {
                inner: CollectionIterInner::Ref(&v),
            },
            Collection::OwnedString(v) => {
                // let iter = Box::new(v.iter().map(|v| <_>::from(v.as_str())));
                // CollectionIter {
                //     inner: CollectionIterInner::String(iter),
                // }
                todo!()
            }
            Collection::BorrowedString(Cow::Owned(v)) => {
                // let iter = Box::new(v.iter().map(|&v| <_>::from(v.as_str())));
                // CollectionIter {
                //     inner: CollectionIterInner::String(iter),
                // }
                todo!()
            }
            Collection::BorrowedString(Cow::Borrowed(v)) => {
                let iter = Box::new(v.iter().map(|&v| <_>::from(v.as_str())));
                CollectionIter {
                    inner: CollectionIterInner::String(iter),
                }
            }
            Collection::RefStr(Cow::Owned(v)) => {
                // let iter = Box::new(v.iter().map(move |&v| <_>::from(v)));
                // CollectionIter {
                //     inner: CollectionIterInner::String(iter),
                // }
                todo!()
            }
            Collection::RefStr(Cow::Borrowed(v)) => {
                let iter = Box::new(v.iter().map(move |&v| <_>::from(v)));
                CollectionIter {
                    inner: CollectionIterInner::String(iter),
                }
            }
        }
    }

    /// Returns the number of items in the collection.
    pub fn len(&self) -> usize {
        match self {
            Collection::Owned(v) => v.len(),
            Collection::Borrowed(v) => v.len(),
            Collection::Ref(v) => v.len(),
            Collection::OwnedString(v) => v.len(),
            Collection::BorrowedString(v) => v.len(),
            Collection::RefStr(v) => v.len(),
        }
    }

    /// Returns `true` if the collection contains no items.
    pub fn is_empty(&self) -> bool {
        match self {
            Collection::Owned(v) => v.is_empty(),
            Collection::Borrowed(v) => v.is_empty(),
            Collection::Ref(v) => v.is_empty(),
            Collection::OwnedString(v) => v.is_empty(),
            Collection::BorrowedString(v) => v.is_empty(),
            Collection::RefStr(v) => v.is_empty(),
        }
    }

    /// Converts the collection into a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use twitch_types::{Collection, UserId};
    ///
    /// let collection = Collection::from(vec!["1", "2", "3"]);
    /// let vector: Vec<UserId> = collection.into_vec();
    /// ```
    pub fn into_vec(self) -> Vec<T>
    where
        [T]: ToOwned<Owned = Vec<T>>,
        T: 'static + Clone,
        for<'d> T: From<&'d <T as std::ops::Deref>::Target>,
        for<'d> T: From<&'d str>, {
        match self {
            Collection::Owned(v) => v.to_vec(),
            Collection::Borrowed(v) => v.iter().map(|v| (*v).clone()).collect(),
            Collection::Ref(v) => v.iter().map(|&v| <T>::from(v)).collect(),
            Collection::OwnedString(v) => v.iter().map(|v| <T>::from(v.as_str())).collect(),
            Collection::BorrowedString(v) => v.iter().map(|v| <T>::from(v.as_str())).collect(),
            Collection::RefStr(v) => v.iter().map(|&v| <T>::from(v)).collect(),
        }
    }

    /// Make a ref vec
    pub fn into_ref_vec(self) -> Arc<[&'c T::Target]>
    where
        [T]: ToOwned<Owned = Vec<T>>,
        T: 'static + Clone,
        for<'d> &'d T::Target: From<&'d str>, {
        match self {
            //Collection::Owned(v) => {
            //    let v: Arc<[T]> = v;
            //    v.clone().iter().map(|v| v.deref()).collect()
            //}
            Collection::Borrowed(v) => v.iter().map(|v| v.deref()).collect(),
            Collection::Ref(v) => v.into(),
            //Collection::OwnedString(v) => v.iter().map(|v| v.as_str().into()).collect(),
            Collection::BorrowedString(v) => v.iter().map(|v| <_>::from(v.as_str())).collect(),
            Collection::RefStr(v) => v.iter().map(|&v| v.into()).collect(),
            _ => todo!(),
        }
    }

    /// Returns chunks of items, similar to [`slice::chunks`]
    pub fn chunks(&'c self, chunk_size: usize) -> CollectionChunks<'c, T>
    where
        for<'s> &'s T::Target: From<&'s str>,
        T: Clone, {
        CollectionChunks {
            inner: self,
            chunk_size,
            index: 0,
        }
    }
}

pub struct CollectionChunks<'c, T: std::ops::Deref + 'static>
where [T]: ToOwned {
    inner: &'c Collection<'c, T>,
    chunk_size: usize,
    index: usize,
}

impl<'c, T: std::ops::Deref + Clone + 'static> Iterator for CollectionChunks<'c, T>
where [T]: ToOwned
{
    type Item = Collection<'c, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }
        let end = if self.chunk_size + self.index > self.inner.len() {
            self.inner.len()
        } else {
            self.chunk_size + self.index
        };
        let new = match self.inner {
            Collection::Owned(v) => {
                Collection::<'c, _>::Owned(v.get(self.index..end)?.clone().into())
            }
            Collection::Borrowed(v) => Collection::Borrowed(v.get(self.index..end)?.clone().into()),
            Collection::Ref(v) => Collection::Ref(v.get(self.index..end)?.clone().into()),
            Collection::OwnedString(v) => {
                Collection::OwnedString(v.get(self.index..end)?.clone().into())
            }
            Collection::BorrowedString(v) => {
                Collection::BorrowedString(v.get(self.index..end)?.clone().into())
            }
            Collection::RefStr(v) => Collection::RefStr(v.get(self.index..end)?.clone().into()),
        };
        self.index += self.chunk_size;
        Some(new)
    }
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

impl<T: std::ops::Deref + Clone> From<Vec<T>> for Collection<'_, T>
where
    [T]: ToOwned,
    T: 'static,
{
    fn from(v: Vec<T>) -> Self { Self::Owned(v.into()) }
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
    fn from(v: &'c [T]) -> Self { Self::Owned(v.into()) }
}

impl<'c, T: std::ops::Deref + Clone> From<&'c [&'c T]> for Collection<'c, T>
where
    [T]: ToOwned,
    T: 'static,
{
    fn from(v: &'c [&'c T]) -> Self { Self::Borrowed(Cow::from(v)) }
}

impl<'se, 'c, T: std::ops::Deref> IntoIterator for &'se Collection<'c, T>
where
    [T]: ToOwned,
    for<'s> &'s T::Target: From<&'s str>,
    for<'a> T: Borrow<T::Target>,
    'se: 'c,
{
    type IntoIter = CollectionIter<'c, T>;
    type Item = &'c T::Target;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

/// Iterator over the elements of a [`Collection`].
pub struct CollectionIter<'c, T: std::ops::Deref>
where [T]: ToOwned {
    inner: CollectionIterInner<'c, T>,
}

enum CollectionIterInner<'c, T: std::ops::Deref>
where [T]: ToOwned {
    Ref(&'c [&'c T::Target]),
    Borrowed(&'c [&'c T]),
    Owned(Arc<[T]>),
    String(Box<dyn std::iter::ExactSizeIterator<Item = &'c T::Target> +Send + Sync + 'c>),
}

impl<'c, T: std::ops::Deref> Iterator for CollectionIter<'c, T>
where
    [T]: ToOwned,
    for<'a> T: Borrow<T::Target>,
{
    type Item = &'c T::Target;

    fn next(&mut self) -> Option<Self::Item> {
        fn take_first<'a, TT>(slice: &mut &'a mut [TT]) -> Option<&'a TT> {
            let (first, rem) = std::mem::take(slice).split_first_mut()?;
            *slice = rem;
            Some(first)
        }

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
                if let Some(mut owned) = Arc::get_mut(owned) {
                    let v = take_first(&mut owned);
                    // v.map(|v| v.deref())
                    todo!()
                } else {
                    panic!()
                }
            }
            CollectionIterInner::String(b) => b.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) { (self.len(), Some(self.len())) }

    fn count(self) -> usize
    where Self: Sized {
        self.len()
    }

    // fn last(self) -> Option<Self::Item>
    // where Self: Sized {
    //     match self.inner {
    //         CollectionIterInner::Ref(v) => v.last().copied(),
    //         CollectionIterInner::Borrowed(v) => v.last().map(|v| v.deref()),
    //         CollectionIterInner::Owned(v) => v.last().map(|v| v.into()),
    //         CollectionIterInner::String(b) => b.last(),
    //     }
    // }
}

impl<T: std::ops::Deref> CollectionIter<'_, T>
where [T]: ToOwned
{
    fn len(&self) -> usize {
        match &self.inner {
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
        Ok(Collection::Owned(collection.into()))
    }
}

#[cfg(feature = "serde")]
impl<T: std::ops::Deref> serde::Serialize for Collection<'_, T>
where
    [T]: ToOwned,
    for<'s> &'s T::Target: serde::Serialize,
    for<'s> &'s T::Target: From<&'s str>,
    for<'a> T: Borrow<T::Target>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.collect_seq(self.iter())
    }
}

//#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
impl<'c, T: std::ops::Deref> Clone for Collection<'c, T>
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

impl<T: std::ops::Deref> PartialEq for Collection<'_, T>
where
    [T]: ToOwned,
    T: PartialEq,
    T::Target: PartialEq,
    T: std::cmp::PartialEq<T::Target>,
    for<'a> T: Borrow<T::Target>,
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

impl<T: std::ops::Deref> Eq for Collection<'_, T>
where
    [T]: ToOwned,
    T: PartialEq,
    T::Target: PartialEq,
    T: std::cmp::PartialEq<T::Target>,
    for<'a> T: Borrow<T::Target>,
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
