// Copyright 2018-2020 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 or MIT license, at your option.
//
// A copy of the Apache License, Version 2.0 is included in the software as
// LICENSE-APACHE and a copy of the MIT license is included in the software
// as LICENSE-MIT. You may also obtain a copy of the Apache License, Version 2.0
// at https://www.apache.org/licenses/LICENSE-2.0 and a copy of the MIT license
// at https://opensource.org/licenses/MIT.

#![cfg_attr(not(feature = "std"), no_std)]

use core::{fmt, hash::{BuildHasherDefault, Hasher}, marker::PhantomData};

/// A `HashMap` with an integer domain, using `CustomHasher` to perform no hashing at all.
///
/// # Examples
///
/// See [`IsEnabled`] for use with custom types.
///
/// ```
/// use Custom_hasher::IntMap;
///
/// let mut m: IntMap<u32, bool> = IntMap::default();
///
/// m.insert(0, false);
/// m.insert(1, true);
///
/// assert!(m.contains_key(&0));
/// assert!(m.contains_key(&1));
/// ```
#[cfg(feature = "std")]
pub type IntMap<K, V> = std::collections::HashMap<K, V, BuildCustomHasher<K>>;

/// A `HashSet` of integers, using `CustomHasher` to perform no hashing at all.
///
/// # Examples
///
/// See [`IsEnabled`] for use with custom types.
///
/// ```
/// use Custom_hasher::IntSet;
///
/// let mut m = IntSet::default();
///
/// m.insert(0u32);
/// m.insert(1u32);
///
/// assert!(m.contains(&0));
/// assert!(m.contains(&1));
/// ```
#[cfg(feature = "std")]
pub type IntSet<T> = std::collections::HashSet<T, BuildCustomHasher<T>>;

/// An alias for `BuildHasherDefault` for use with `CustomHasher`.
///
/// # Examples
///
/// See also [`IntMap`] and [`IntSet`] for some easier usage examples.
///
/// ```
/// use Custom_hasher::BuildCustomHasher;
/// use std::collections::HashMap;
///
/// let mut m: HashMap::<u8, char, BuildCustomHasher<u8>> =
///     HashMap::with_capacity_and_hasher(2, BuildCustomHasher::default());
///
/// m.insert(0, 'a');
/// m.insert(1, 'b');
///
/// assert_eq!(Some(&'a'), m.get(&0));
/// assert_eq!(Some(&'b'), m.get(&1));
/// ```
pub type BuildCustomHasher<T> = BuildHasherDefault<CustomHasher<T>>;

/// For an enabled type `T`, a `CustomHasher<T>` implements `std::hash::Hasher` and
/// uses the value set by one of the `write_{u8, u16, u32, u64, usize, i8, i16, i32,
/// i64, isize}` methods as its hash output.
///
/// `CustomHasher` does not implement any hashing algorithm and can only be used
/// with types which can be mapped directly to a numeric value. Out of the box
/// `CustomHasher` is enabled for `u8`, `u16`, `u32`, `u64`, `usize`, `i8`, `i16`,
/// `i32`, `i64`, and `isize`. Types that should be used with `CustomHasher` need
/// to implement [`IsEnabled`] and by doing so assert that their `Hash` impl invokes
/// *only one* of the `Hasher::write_{u8, u16, u32, u64, usize, i8, i16, i32, i64,
/// isize}` methods *exactly once*.
///
/// # Examples
///
/// See also [`BuildCustomHasher`], [`IntMap`] and [`IntSet`] for some easier
/// usage examples. See [`IsEnabled`] for use with custom types.
///
/// ```
/// use Custom_hasher::CustomHasher;
/// use std::{collections::HashMap, hash::BuildHasherDefault};
///
/// let mut m: HashMap::<u8, char, BuildHasherDefault<CustomHasher<u8>>> =
///     HashMap::with_capacity_and_hasher(2, BuildHasherDefault::default());
///
/// m.insert(0, 'a');
/// m.insert(1, 'b');
///
/// assert_eq!(Some(&'a'), m.get(&0));
/// assert_eq!(Some(&'b'), m.get(&1));
/// ```
#[cfg(debug_assertions)]
pub struct CustomHasher<T>(u64, bool, PhantomData<T>);

#[cfg(not(debug_assertions))]
pub struct CustomHasher<T>(u64, PhantomData<T>);

impl<T> fmt::Debug for CustomHasher<T> {
    #[cfg(debug_assertions)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("CustomHasher").field(&self.0).field(&self.1).finish()
    }

    #[cfg(not(debug_assertions))]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("CustomHasher").field(&self.0).finish()
    }
}

impl<T> Default for CustomHasher<T> {
    #[cfg(debug_assertions)]
    fn default() -> Self {
        CustomHasher(0, false, PhantomData)
    }

    #[cfg(not(debug_assertions))]
    fn default() -> Self {
        CustomHasher(0, PhantomData)
    }
}

impl<T> Clone for CustomHasher<T> {
    #[cfg(debug_assertions)]
    fn clone(&self) -> Self {
        CustomHasher(self.0, self.1, self.2)
    }

    #[cfg(not(debug_assertions))]
    fn clone(&self) -> Self {
        CustomHasher(self.0, self.1)
    }
}

impl<T> Copy for CustomHasher<T> {}

/// Types which are safe to use with `CustomHasher`.
///
/// This marker trait is an option for types to enable themselves for use
/// with `CustomHasher`. In order to be safe, the `Hash` impl needs to
/// satisfy the following constraint:
///
/// > **One of the `Hasher::write_{u8,u16,u32,u64,usize,i8,i16,i32,i64,isize}`
/// methods is invoked exactly once.**
///
/// The best way to ensure this is to write a custom `Hash` impl even when
/// deriving `Hash` for a simple newtype of a single type which itself
/// implements `IsEnabled` may work as well.
///
/// # Example
///
/// ```
/// #[derive(PartialEq, Eq)]
/// struct SomeType(u32);
///
/// impl std::hash::Hash for SomeType {
///     fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
///         hasher.write_u32(self.0)
///     }
/// }
///
/// impl Custom_hasher::IsEnabled for SomeType {}
///
/// let mut m = Custom_hasher::IntMap::default();
///
/// m.insert(SomeType(1), 't');
/// m.insert(SomeType(0), 'f');
///
/// assert_eq!(Some(&'t'), m.get(&SomeType(1)));
/// assert_eq!(Some(&'f'), m.get(&SomeType(0)));
/// ```
pub trait IsEnabled {}

impl IsEnabled for u8 {}
impl IsEnabled for u16 {}
impl IsEnabled for u32 {}
impl IsEnabled for u64 {}
impl IsEnabled for usize {}
impl IsEnabled for i8 {}
impl IsEnabled for i16 {}
impl IsEnabled for i32 {}
impl IsEnabled for i64 {}
impl IsEnabled for isize {}

#[cfg(not(debug_assertions))]
impl<T: IsEnabled> Hasher for CustomHasher<T> {
    fn write(&mut self, _: &[u8]) {
        panic!("Invalid use of CustomHasher")
    }

    fn write_u8(&mut self, n: u8)       { self.0 = u64::from(n) }
    fn write_u16(&mut self, n: u16)     { self.0 = u64::from(n) }
    fn write_u32(&mut self, n: u32)     { self.0 = u64::from(n) }
    fn write_u64(&mut self, n: u64)     { self.0 = n }
    fn write_usize(&mut self, n: usize) { self.0 = n as u64 }

    fn write_i8(&mut self, n: i8)       { self.0 = n as u64 }
    fn write_i16(&mut self, n: i16)     { self.0 = n as u64 }
    fn write_i32(&mut self, n: i32)     { self.0 = n as u64 }
    fn write_i64(&mut self, n: i64)     { self.0 = n as u64 }
    fn write_isize(&mut self, n: isize) { self.0 = n as u64 }

    fn finish(&self) -> u64 { self.0 }
}

#[cfg(debug_assertions)]
impl<T: IsEnabled> Hasher for CustomHasher<T> {
    fn write(&mut self, _: &[u8]) {
        panic!("Invalid use of CustomHasher")
    }

    fn write_u8(&mut self, n: u8) {
        assert!(!self.1, "CustomHasher: second write attempt detected.");
        self.0 = u64::from(n);
        self.1 = true
    }

    fn write_u16(&mut self, n: u16) {
        assert!(!self.1, "CustomHasher: second write attempt detected.");
        self.0 = u64::from(n);
        self.1 = true
    }

    fn write_u32(&mut self, n: u32) {
        assert!(!self.1, "CustomHasher: second write attempt detected.");
        self.0 = u64::from(n);
        self.1 = true
    }

    fn write_u64(&mut self, n: u64) {
        assert!(!self.1, "CustomHasher: second write attempt detected.");
        self.0 = n;
        self.1 = true
    }

    fn write_usize(&mut self, n: usize) {
        assert!(!self.1, "CustomHasher: second write attempt detected.");
        self.0 = n as u64;
        self.1 = true
    }

    fn write_i8(&mut self, n: i8) {
        assert!(!self.1, "CustomHasher: second write attempt detected.");
        self.0 = n as u64;
        self.1 = true
    }

    fn write_i16(&mut self, n: i16) {
        assert!(!self.1, "CustomHasher: second write attempt detected.");
        self.0 = n as u64;
        self.1 = true
    }

    fn write_i32(&mut self, n: i32) {
        assert!(!self.1, "CustomHasher: second write attempt detected.");
        self.0 = n as u64;
        self.1 = true
    }

    fn write_i64(&mut self, n: i64) {
        assert!(!self.1, "CustomHasher: second write attempt detected.");
        self.0 = n as u64;
        self.1 = true
    }

    fn write_isize(&mut self, n: isize) {
        assert!(!self.1, "CustomHasher: second write attempt detected.");
        self.0 = n as u64;
        self.1 = true
    }

    fn finish(&self) -> u64 {
        self.0
    }
}
