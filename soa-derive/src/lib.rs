//! This crate provides a custom derive (`#[derive(StructOfArray)]`) to
//! automatically generate code from a given struct `T` that allow to replace
//! `Vec<T>` with a struct of arrays. For example, the following code
//!
//! ```
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! #[derive(StructOfArray)]
//! pub struct Cheese {
//!     pub smell: f64,
//!     pub color: (f64, f64, f64),
//!     pub with_mushrooms: bool,
//!     pub name: String,
//! }
//! # }
//! ```
//!
//! will generate a `CheeseVec` struct that looks like this:
//!
//! ```
//! pub struct CheeseVec {
//!     pub smell: Vec<f64>,
//!     pub color: Vec<(f64, f64, f64)>,
//!     pub with_mushrooms: Vec<bool>,
//!     pub name: Vec<String>,
//! }
//! ```
//!
//! It will also generate the same functions that a `Vec<Chees>` would have, and
//! a few helper structs: `CheeseSlice`, `CheeseSliceMut`, `CheeseRef` and
//! `CheeseRefMut` corresponding respectivly to `&[Cheese]`, `&mut [Cheese]`,
//! `&Cheese` and `&mut Cheese`.
//!
//! # How to use it
//!
//! Add `#[derive(StructOfArray)]` to each struct you want to derive a struct of
//! array version. If you need the helper structs to derive additional traits
//! (such as `Debug` or `PartialEq`), you can add an attribute `#[soa_derive =
//! "Debug, PartialEq"]` to the struct declaration.
//!
//! ```
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! #[derive(Debug, PartialEq, StructOfArray)]
//! #[soa_derive(Debug, PartialEq)]
//! pub struct Cheese {
//!     pub smell: f64,
//!     pub color: (f64, f64, f64),
//!     pub with_mushrooms: bool,
//!     pub name: String,
//! }
//! # }
//! ```
//!
//! If you want to add attribute to a specific generated struct(such as
//! `#[cfg_attr(test, derive(PartialEq))]` on `CheeseVec`), you can add an
//! attribute `#[soa_attr(Vec, cfg_attr(test, derive(PartialEq)))]` to the
//! struct declaration.
//!
//! ```
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! #[derive(Debug, PartialEq, StructOfArray)]
//! #[soa_attr(Vec, cfg_attr(test, derive(PartialEq)))]
//! pub struct Cheese {
//!     pub smell: f64,
//!     pub color: (f64, f64, f64),
//!     pub with_mushrooms: bool,
//!     pub name: String,
//! }
//! # }
//! ```
//!
//! Mappings for first argument of ``soa_attr`` to the generated struct for ``Cheese``:
//! * `Vec` => `CheeseVec`
//! * `Slice` => `CheeseSlice`
//! * `SliceMut` => `CheeseSliceMut`
//! * `Ref` => `CheeseRef`
//! * `RefMut` => `CheeseRefMut`
//! * `Ptr` => `CheesePtr`
//! * `PtrMut` => `CheesePtrMut`
//!
//! # Usage and API
//!
//! All the generated code have some generated documentation with it, so you
//! should be able to use `cargo doc` on your crate and see the documentation
//! for all the generated structs and functions.
//!
//! Most of the time, you should be able to replace `Vec<Cheese>` by
//! `CheeseVec`, with exception of code using direct indexing in the vector and
//! a few other caveats listed below.
//!
//! ## Caveats and limitations
//!
//! `Vec<T>` functionalities rely a lot on references and automatic *deref*
//! feature, for getting function from `[T]` and indexing. But the SoA vector
//! (let's call it `CheeseVec`, generated from the `Cheese` struct) generated by
//! this crate can not implement `Deref<Target=CheeseSlice>`, because `Deref` is
//! required to return a reference, and `CheeseSlice` is not a reference. The
//! same applies to `Index` and `IndexMut` trait, that can not return
//! `CheeseRef/CheeseRefMut`.
//!
//! This means that the we can not index into a `CheeseVec`, and that a few
//! functions are duplicated, or require a call to `as_ref()/as_mut()` to change
//! the type used.
//!
//! # Iteration
//!
//! It is possible to iterate over the values in a `CheeseVec`
//!
//! ```no_run
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! # #[derive(Debug, PartialEq, StructOfArray)]
//! # pub struct Cheese {
//! #     pub smell: f64,
//! #     pub color: (f64, f64, f64),
//! #     pub with_mushrooms: bool,
//! #     pub name: String,
//! # }
//! # impl Cheese { fn new(name: &str) -> Cheese { unimplemented!() } }
//! # fn main() {
//! let mut vec = CheeseVec::new();
//! vec.push(Cheese::new("stilton"));
//! vec.push(Cheese::new("brie"));
//!
//! for cheese in vec.iter() {
//!     // when iterating over a CheeseVec, we load all members from memory
//!     // in a CheeseRef
//!     let typeof_cheese: CheeseRef = cheese;
//!     println!("this is {}, with a smell power of {}", cheese.name, cheese.smell);
//! }
//! # }
//! # }
//! ```
//!
//! One of the main advantage of the SoA layout is to be able to only load some
//! fields from memory when iterating over the vector. In order to do so, one
//! can manually pick the needed fields:
//!
//! ```no_run
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! # #[derive(Debug, PartialEq, StructOfArray)]
//! # pub struct Cheese {
//! #     pub smell: f64,
//! #     pub color: (f64, f64, f64),
//! #     pub with_mushrooms: bool,
//! #     pub name: String,
//! # }
//! # impl Cheese { fn new(name: &str) -> Cheese { unimplemented!() } }
//! # fn main() {
//! # let mut vec = CheeseVec::new();
//! # vec.push(Cheese::new("stilton"));
//! # vec.push(Cheese::new("brie"));
//! for name in &vec.name {
//!     // We get referenes to the names
//!     let typeof_name: &String = name;
//!     println!("got cheese {}", name);
//! }
//! # }
//! # }
//! ```
//!
//! In order to iterate over multiple fields at the same time, one can use the
//! [soa_zip!](macro.soa_zip.html) macro.
//!
//! ```no_run
//! # #[macro_use] extern crate soa_derive;
//! # mod cheese {
//! # #[derive(Debug, PartialEq, StructOfArray)]
//! # pub struct Cheese {
//! #     pub smell: f64,
//! #     pub color: (f64, f64, f64),
//! #     pub with_mushrooms: bool,
//! #     pub name: String,
//! # }
//! # impl Cheese { fn new(name: &str) -> Cheese { unimplemented!() } }
//! # fn main() {
//! # let mut vec = CheeseVec::new();
//! # vec.push(Cheese::new("stilton"));
//! # vec.push(Cheese::new("brie"));
//! for (name, smell, color) in soa_zip!(&mut vec, [name, mut smell, color]) {
//!     println!("this is {}, with color {:#?}", name, color);
//!     // smell is a mutable reference
//!     *smell += 1.0;
//! }
//! # }
//! # }
//! ```
//!
//! ## Nested Struct of Arrays
//!
//! In order to nest a struct of arrays inside another struct of arrays, one can use the `#[nested_soa]` attribute.
//!
//! For example, the following code
//!
//! ```
//! # mod cheese {
//! # use soa_derive::StructOfArray;
//! #[derive(StructOfArray)]
//! pub struct Point {
//!     x: f32,
//!     y: f32,
//! }
//! #[derive(StructOfArray)]
//! pub struct Particle {
//!     #[nested_soa]
//!     point: Point,
//!     mass: f32,
//! }
//! # }
//! ```
//!
//! will generate structs that looks like this:
//!
//! ```
//! pub struct PointVec {
//!     x: Vec<f32>,
//!     y: Vec<f32>,
//! }
//! pub struct ParticleVec {
//!     point: PointVec, // rather than Vec<Point>
//!     mass: Vec<f32>
//! }
//! ```
//!
//! All helper structs will be also nested, for example `PointSlice` will be nested in `ParticleSlice`.
//!
//! # Use in a generic context
//!
//! `StructOfArray` does not provide a set of common operations by default. Thus if you wanted to use a `StructOfArray`
//! type in a generic context, there is no way to guarantee to the type system that any methods are available.
//!
//! If the `generic_traits` feature is enabled, the attribute macro `#[generate_traits]` will generate
//! implementations of [`SoAVec`], [`SoASlice`], and [`SoASliceMut`] for the respective `Vec`, `Slice`
//! and `SliceMut` types.
//!
//! These rely on GATs, and so require Rust 1.65 or newer, and so this feature is disabled by default.
//! Even when enabled, trait implementations are only generated by opting in with the `#[generate_traits]`
//! attribute.
//!
//! ```ignore
//! # mod cheese {
//! # use soa_derive::{StructOfArray, prelude::*};
//! #[derive(StructOfArray)]
//! #[generate_traits]
//! pub struct Point {
//!     x: f32,
//!     y: f32,
//! }
//!
//! fn get_num_items<T: StructOfArray, V: SoAVec<T>>(values: &V) -> usize {
//!     values.len()
//! }
//! # }
//! ```

// The proc macro is implemented in soa_derive_internal, and re-exported by this
// crate. This is because a single crate can not define both a proc macro and a
// macro_rules macro.
pub use soa_derive_internal::StructOfArray;

// External dependency necessary for implementing the sorting methods.
// It is basically used by the macro-generated code.
#[doc(hidden)]
pub use permutation::permutation::*;

/// Any struct derived by StructOfArray will auto impl this trait You can use
/// `<Cheese as StructOfArray>::Type` instead of explicit named type
/// `CheeseVec`; This will helpful in generics programing that generate struct
/// can be expressed as `<T as StructOfArray>::Type`
pub trait StructOfArray {
    type Type;
}

/// Any struct derived by StructOfArray will auto impl this trait.
///
/// Useful for generic programming and implementation of attribute `nested_soa`.
///
/// `CheeseVec::iter(&'a self)` returns an iterator which has a type `<Cheese as SoAIter<'a>>::Iter`
///
/// `CheeseVec::iter_mut(&mut 'a self)` returns an iterator which has a type `<Cheese as SoAIter<'a>>::IterMut`
pub trait SoAIter<'a> {
    type Ref;
    type RefMut;
    type Iter: 'a + Iterator<Item=Self::Ref>;
    type IterMut: 'a + Iterator<Item=Self::RefMut>;
}

mod private_soa_indexes {
    // From [`std::slice::SliceIndex`](https://doc.rust-lang.org/std/slice/trait.SliceIndex.html) code.
    // Limits the types that may implement the SoA index traits.
    // It's also helpful to have the exaustive list of all accepted types.

    use ::std::ops;

    pub trait Sealed {}

    impl Sealed for usize {}                        // [a]
    impl Sealed for ops::Range<usize> {}            // [a..b]
    impl Sealed for ops::RangeTo<usize> {}          // [..b]
    impl Sealed for ops::RangeFrom<usize> {}        // [a..]
    impl Sealed for ops::RangeFull {}               // [..]
    impl Sealed for ops::RangeInclusive<usize> {}   // [a..=b]
    impl Sealed for ops::RangeToInclusive<usize> {} // [..=b]
}

/// Helper trait used for indexing operations.
/// Inspired by [`std::slice::SliceIndex`](https://doc.rust-lang.org/std/slice/trait.SliceIndex.html).
pub trait SoAIndex<T>: private_soa_indexes::Sealed {
    /// The output for the non-mutable functions
    type RefOutput;

    /// Returns the reference output in this location if in bounds, `None`
    /// otherwise.
    fn get(self, soa: T) -> Option<Self::RefOutput>;
    /// Returns the reference output in this location without performing any
    /// bounds check.
    ///
    /// # Safety
    /// The index must be in bounds.
    unsafe fn get_unchecked(self, soa: T) -> Self::RefOutput;
    /// Returns the reference output in this location. Panics if it is not in
    /// bounds.
    fn index(self, soa: T) -> Self::RefOutput;
}

/// Helper trait used for indexing operations returning mutable references.
/// Inspired by [`std::slice::SliceIndex`](https://doc.rust-lang.org/std/slice/trait.SliceIndex.html).
pub trait SoAIndexMut<T>: private_soa_indexes::Sealed {
    /// The output for the mutable functions
    type MutOutput;

    /// Returns the mutable reference output in this location if in bounds,
    /// `None` otherwise.
    fn get_mut(self, soa: T) -> Option<Self::MutOutput>;
    /// Returns the mutable reference output in this location without performing
    /// any bounds check.
    ///
    /// # Safety
    /// The index must be in bounds.
    unsafe fn get_unchecked_mut(self, soa: T) -> Self::MutOutput;
    /// Returns the mutable reference output in this location. Panics if it is
    /// not in bounds.
    fn index_mut(self, soa: T) -> Self::MutOutput;
}

/// Create an iterator over multiple fields in a Struct of array style vector.
///
/// This macro takes two main arguments: the array/slice container, and a list
/// of fields to use, inside square brackets. The iterator will give references
/// to the fields, which can be mutable references if the field name is prefixed
/// with `mut`.
///
/// ```
/// # #[macro_use] extern crate soa_derive;
/// # mod cheese {
/// #[derive(StructOfArray)]
/// struct Cheese {
///     size: f64,
///     mass: f64,
///     smell: f64,
///     name: String,
/// }
///
/// # fn main() {
/// let mut vec = CheeseVec::new();
/// // fill the vector
///
/// // Iterate over immutable references
/// for (mass, size, name) in soa_zip!(&vec, [mass, size, name]) {
///     println!("got {} kg and {} cm of {}", mass, size, name);
/// }
///
/// // Iterate over mutable references
/// for (mass, name) in soa_zip!(&mut vec, [mut mass, name]) {
///     println!("got {} kg of {}, eating 1 kg", mass, name);
///     *mass -= 1.0;
/// }
/// # }
/// # }
/// ```
///
/// The iterator can also work with external iterators. In this case, the
/// iterator will yields elements until any of the fields or one external
/// iterator returns None.
///
/// ```
/// # #[macro_use] extern crate soa_derive;
/// # mod cheese {
/// # #[derive(StructOfArray)]
/// # struct Cheese {
/// #     size: f64,
/// #     mass: f64,
/// #     smell: f64,
/// #     name: String,
/// # }
/// # #[derive(Debug)] struct Cellar;
/// # fn main() {
/// let mut vec = CheeseVec::new();
/// let mut cellars = Vec::<Cellar>::new();
///
/// for (name, mass, cellar) in soa_zip!(&vec, [name, mass], &cellars) {
///     println!("we have {} kg of {} in {:#?}", mass, name, cellar);
/// }
/// # }
/// # }
/// ```
#[macro_export]
macro_rules! soa_zip {
    ($self: expr, [$($fields: tt)*] $(, $external: expr)* $(,)*) => {{
        let this = $self;
        $crate::soa_zip_impl!(@munch this, {$($fields)*} -> [] $($external ,)*)
    }};
}

pub trait SoAPointers {
    type Ptr;
    type MutPtr;
}


#[cfg(feature = "generic_traits")]
mod generics {
    use super::*;

    /**
    The interface for the `Slice` immutable slice struct-of-arrays type.
    */
    pub trait SoASlice<T: StructOfArray> {
        /// The type that elements will be proxied with as
        type Ref<'t> where Self: 't;

        /// The type representing immutable slices of elements
        type Slice<'t>: SoASlice<T> + IntoSoAIter<'t, T, Ref<'t> = Self::Ref<'t>> where Self: 't;

        /// The type used for iteration over [`Self::Ref`]
        type Iter<'t>: Iterator<Item=Self::Ref<'t>> where Self: 't;

        /// The raw pointer type interface
        type Ptr;

        /// Returns the number of elements in the arrays
        fn len(&self) -> usize;

        /// Returns true if the arrays has a length of 0.
        fn is_empty(&self) -> bool;

        /// Create an immutable slice of the arrays
        fn as_slice<'c>(&'c self) -> Self::Slice<'c>;

        /// Create a slice of this vector matching the given `range`. This
        /// is analogous to `Index<Range<usize>>`.
        fn slice<'c, 'a: 'c>(&'c self, index: impl core::ops::RangeBounds<usize>) -> Self::Slice<'c> where Self: 'a;

        /// Analogous to [`slice::get()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get)
        fn get<'c>(&'c self, index: usize) -> Option<Self::Ref<'c>>;

        /// Analogous to [`std::ops::Index::index()`] for `usize`
        fn index<'c>(&'c self, index: usize) -> Self::Ref<'c>;

        /// Create an immutable iterator
        fn iter<'c>(&'c self) -> Self::Iter<'c>;

        /// Analogous to [`slice::first()`](https://doc.rust-lang.org/std/primitive.slice.html#method.first)
        fn first<'c>(&'c self) -> Option<Self::Ref<'c>> {
            self.get(0)
        }

        /// Analogous to [`slice::last()`](https://doc.rust-lang.org/std/primitive.slice.html#method.last)
        fn last<'c>(&'c self) -> Option<Self::Ref<'c>> {
            self.get(self.len().saturating_sub(1))
        }

        /// Obtain a `const` pointer type for this data
        fn as_ptr(&self) -> Self::Ptr;
    }

    /**
    The interface for the `SliceMut` mutable slice struct-of-arrays type. A generalization of [`SoASlice`]
    whose methods can modify elements of the arrays
    */
    pub trait SoASliceMut<T: StructOfArray> {
        /// The type that elements will be proxied with as
        type Ref<'t> where Self: 't;

        /// The type representing immutable slices of elements
        type Slice<'t>: SoASlice<T> + IntoSoAIter<'t, T, Ref<'t> = Self::Ref<'t>> where Self: 't;

        /// The type used for iteration over [`Self::Ref`]
        type Iter<'t>: Iterator<Item=Self::Ref<'t>> where Self: 't;

        /// The const pointer type interface
        type Ptr;

        /// The type that elements will be proxied with as when mutable
        type RefMut<'t> where Self: 't;

        /// The type representing mutable slices of elements
        type SliceMut<'t>: SoASliceMut<T> where Self: 't;

        /// The type used for iteration over [`Self::RefMut`]
        type IterMut<'t>: Iterator<Item=Self::RefMut<'t>> where Self: 't;

        /// The mut pointer type interface
        type PtrMut;

        /// Returns the number of elements in the arrays
        fn len(&self) -> usize;

        /// Returns true if the arrays has a length of 0.
        fn is_empty(&self) -> bool;

        /// Create an immutable slice of the arrays
        fn as_slice<'c>(&'c self) -> Self::Slice<'c>;

        /// Create a slice of this vector matching the given `range`. This
        /// is analogous to `Index<Range<usize>>`.
        fn slice<'c, 'a: 'c>(&'c self, index: impl core::ops::RangeBounds<usize>) -> Self::Slice<'c> where Self: 'a;

        /// Analogous to [`slice::get()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get)
        fn get<'c>(&'c self, index: usize) -> Option<Self::Ref<'c>>;

        /// Analogous to [`std::ops::Index::index()`] for `usize`
        fn index<'c>(&'c self, index: usize) -> Self::Ref<'c>;

        /// Create an immutable iterator
        fn iter<'c>(&'c self) -> Self::Iter<'c>;

        /// Analogous to [`slice::first()`](https://doc.rust-lang.org/std/primitive.slice.html#method.first)
        fn first<'c>(&'c self) -> Option<Self::Ref<'c>> {
            self.get(0)
        }

        /// Analogous to [`slice::last()`](https://doc.rust-lang.org/std/primitive.slice.html#method.last)
        fn last<'c>(&'c self) -> Option<Self::Ref<'c>> {
            self.get(self.len().saturating_sub(1))
        }

        /// Obtain a `const` pointer type for this data
        fn as_ptr(&self) -> Self::Ptr;

        /// Analogous to [`Vec::as_mut_slice()`]
        fn as_mut_slice<'c: 'b, 'b>(&'c mut self) -> Self::SliceMut<'c> where Self: 'b;

        /// Create a mutable slice of this vector matching the given
        /// `range`. This is analogous to `IndexMut<Range<usize>>`.
        fn slice_mut<'c>(&'c mut self, index: impl core::ops::RangeBounds<usize>) -> Self::SliceMut<'c>;

        /// Analogous to [`slice::get_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_mut)
        fn get_mut<'c>(&'c mut self, index: usize) -> Option<Self::RefMut<'c>>;

        /// Analogous to [`std::ops::IndexMut::index_mut()`] for `usize`
        fn index_mut<'c>(&'c mut self, index: usize) -> Self::RefMut<'c>;

        /// Creates a mutable iterator
        fn iter_mut<'c>(&'c mut self) -> Self::IterMut<'c>;

        /** Re-order the arrays using the provided indices. This is provided so that generic sorting methods
         can be implemented because closure-passing trait methods encounter difficulties with lifetimes.
        */
        fn apply_index(&mut self, indices: &[usize]);

        /// `[slice::sort_by()`](<https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by>).
        fn sort_by<F>(&mut self, mut f: F) where F: FnMut(Self::Ref<'_>, Self::Ref<'_>) -> std::cmp::Ordering {
            let mut permutation: Vec<usize> = (0..self.len()).collect();
            permutation.sort_by(|j, k| f(self.index(*j), self.index(*k)));

            self.apply_index(&permutation);
        }

        /// `[slice::sort_by()`](<https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key>).
        fn sort_by_key<F, K>(&mut self, mut f: F) where
            F: FnMut(Self::Ref<'_>) -> K,
            K: Ord,
        {
            let mut permutation: Vec<usize> = (0..self.len()).collect();
            permutation.sort_by_key(|j| f(self.index(*j)));

            self.apply_index(&permutation);
        }

        /// Analogous to [`slice::first_mut()`](<https://doc.rust-lang.org/std/primitive.slice.html#method.first_mut>).
        fn first_mut<'c>(&'c mut self) -> Option<Self::RefMut<'c>> {
            self.get_mut(0)
        }

        /// Analogous to [`slice::last_mut()`](<https://doc.rust-lang.org/std/primitive.slice.html#method.last_mut>).
        fn last_mut<'c>(&'c mut self) -> Option<Self::RefMut<'c>> {
            self.get_mut(self.len().saturating_sub(1))
        }

        /// Obtain a `mut` pointer type for this data
        fn as_mut_ptr(&mut self) -> Self::PtrMut;
    }

    /**
    The interface for the `Vec`-like struct-of-arrays type. A generalization of [`SoASliceMut`] whose methods can
    also re-size the underlying arrays.

    **NOTE**: This interface is incomplete and additional methods may be added as needed.
    */
    pub trait SoAVec<T: StructOfArray> {
        /// The type that elements will be proxied with as
        type Ref<'t> where Self: 't;

        /// The type representing immutable slices of elements
        type Slice<'t>: SoASlice<T> + IntoSoAIter<'t, T> where Self: 't;

        /// The type used for iteration over [`Self::Ref`]
        type Iter<'t>: Iterator<Item=Self::Ref<'t>> where Self: 't;

        /// The const pointer type interface
        type Ptr;

        /// The type that elements will be proxied with as when mutable
        type RefMut<'t> where Self: 't;

        /// The type representing mutable slices of elements
        type SliceMut<'t>: SoASliceMut<T> where Self: 't;

        /// The type used for iteration over [`Self::RefMut`]
        type IterMut<'t>: Iterator<Item=Self::RefMut<'t>> where Self: 't;

        /// The mut pointer type interface
        type PtrMut;

        /// Returns the number of elements in the arrays
        fn len(&self) -> usize;

        /// Returns true if the arrays has a length of 0.
        fn is_empty(&self) -> bool;

        /// Create an immutable slice of the arrays
        fn as_slice<'c, 'a: 'c>(&'c self) -> Self::Slice<'c> where Self: 'a;

        /// Create a slice of this vector matching the given `range`. This
        /// is analogous to `Index<Range<usize>>`.
        fn slice<'c, 'a: 'c>(&'c self, index: impl core::ops::RangeBounds<usize>) -> Self::Slice<'c> where Self: 'a;

        /// Analogous to [`slice::get()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get)
        fn get<'c>(&'c self, index: usize) -> Option<Self::Ref<'c>>;

        /// Analogous to [`std::ops::Index::index()`] for `usize`
        fn index<'c>(&'c self, index: usize) -> Self::Ref<'c>;

        /// Create an immutable iterator
        fn iter<'c>(&'c self) -> Self::Iter<'c>;

        /// Analogous to [`slice::first()`](https://doc.rust-lang.org/std/primitive.slice.html#method.first)
        fn first<'c>(&'c self) -> Option<Self::Ref<'c>> {
            self.get(0)
        }

        /// Analogous to [`slice::last()`](https://doc.rust-lang.org/std/primitive.slice.html#method.last)
        fn last<'c>(&'c self) -> Option<Self::Ref<'c>> {
            self.get(self.len().saturating_sub(1))
        }

        /// Obtain a `const` pointer type for this data
        fn as_ptr(&self) -> Self::Ptr;

        /// Analogous to [`Vec::as_mut_slice()`]
        fn as_mut_slice<'c, 'a: 'c>(&'c mut self) -> Self::SliceMut<'c> where Self: 'a;

        /// Create a mutable slice of this vector matching the given
        /// `range`. This is analogous to `IndexMut<Range<usize>>`.
        fn slice_mut<'c>(&'c mut self, index: impl core::ops::RangeBounds<usize>) -> Self::SliceMut<'c>;

        /// Analogous to [`slice::get_mut()`](https://doc.rust-lang.org/std/primitive.slice.html#method.get_mut)
        fn get_mut<'c>(&'c mut self, index: usize) -> Option<Self::RefMut<'c>>;

        /// Analogous to [`std::ops::IndexMut::index_mut()`] for `usize`
        fn index_mut<'c>(&'c mut self, index: usize) -> Self::RefMut<'c>;

        /// Creates a mutable iterator
        fn iter_mut<'c>(&'c mut self) -> Self::IterMut<'c>;

        /** Re-order the arrays using the provided indices. This is provided so that generic sorting methods
         can be implemented because closure-passing trait methods encounter difficulties with lifetimes.
        */
        fn apply_index(&mut self, indices: &[usize]);

        /// `[slice::sort_by()`](<https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by>).
        fn sort_by<F>(&mut self, mut f: F) where F: FnMut(Self::Ref<'_>, Self::Ref<'_>) -> std::cmp::Ordering {
            let mut permutation: Vec<usize> = (0..self.len()).collect();
            permutation.sort_by(|j, k| f(self.index(*j), self.index(*k)));

            self.apply_index(&permutation);
        }

        /// `[slice::sort_by()`](<https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by_key>).
        fn sort_by_key<F, K>(&mut self, mut f: F) where
            F: FnMut(Self::Ref<'_>) -> K,
            K: Ord,
        {
            let mut permutation: Vec<usize> = (0..self.len()).collect();
            permutation.sort_by_key(|j| f(self.index(*j)));

            self.apply_index(&permutation);
        }

        /// Analogous to [`slice::first_mut()`](<https://doc.rust-lang.org/std/primitive.slice.html#method.first_mut>)
        fn first_mut<'c>(&'c mut self) -> Option<Self::RefMut<'c>> {
            self.get_mut(0)
        }

        /// Analogous to [`slice::last_mut()`](<https://doc.rust-lang.org/std/primitive.slice.html#method.last_mut>)
        fn last_mut<'c>(&'c mut self) -> Option<Self::RefMut<'c>> {
            self.get_mut(self.len().saturating_sub(1))
        }

        /// Obtain a `mut` pointer type for this data
        fn as_mut_ptr(&mut self) -> Self::PtrMut;

        /// Create a new, empty struct of arrays
        fn new() -> Self;

        /// Create a new, empty struct of arrays with the specified capacity
        fn with_capacity(capacity: usize) -> Self;

        /// Analogous to [`Vec::capacity`]
        fn capacity(&self) -> usize;

        /// Analogous to [`Vec::reserve`]
        fn reserve(&mut self, additional: usize);

        /// Analogous to [`Vec::reserve_exact`]
        fn reserve_exact(&mut self, additional: usize);

        /// Analogous to [`Vec::shrink_to_fit`]
        fn shrink_to_fit(&mut self);

        /// Analogous to [`Vec::truncate`]
        fn truncate(&mut self, len: usize);

        /// Add a singular value of `T` to the arrays. Analogous to [`Vec::push`]
        fn push(&mut self, value: T);

        /// Analogous to [`Vec::swap_remove`]
        fn swap_remove(&mut self, index: usize) -> T;

        /// Analogous to [`Vec::insert`]
        fn insert(&mut self, index: usize, element: T);

        /// Similar to [`std::mem::replace()`](https://doc.rust-lang.org/std/mem/fn.replace.html).
        fn replace(&mut self, index: usize, element: T) -> T;

        /// Analogous to [`Vec::remove`]
        fn remove(&mut self, index: usize) -> T;

        /// Analogous to [`Vec::pop`]
        fn pop(&mut self) -> Option<T>;

        /// Analogous to [`Vec::append`]
        fn append(&mut self, other: &mut Self);

        /// Analogous to [`Vec::clear`]
        fn clear(&mut self);

        /// Analogous to [`Vec::split_off`]
        fn split_off(&mut self, at: usize) -> Self;
    }

    /// A trait to implement `Clone`-dependent behavior to convert a non-owning SoA type into an
    /// owning [`SoAVec`].
    pub trait ToSoAVec<T: StructOfArray> {
        type SoAVecType: SoAVec<T>;

        /// Similar to [`slice::to_vec()`](https://doc.rust-lang.org/std/primitive.slice.html#method.to_vec)
        fn to_vec(&self) -> Self::SoAVecType;
    }

    /// A trait to implement `Clone`-dependent behavior to extend an [`SoAVec`] with data copied
    /// from its associated `Slice` type.
    pub trait SoAAppendVec<T: StructOfArray>: SoAVec<T> {

        /// Analogous to [`Vec::extend_from_slice`]
        fn extend_from_slice(&mut self, other: Self::Slice<'_>);
    }

    /// A trait to express the [`IntoIterator`] guarantee of [`SoASlice`] types in the type system.
    pub trait IntoSoAIter<'a, T: StructOfArray>: SoASlice<T> + IntoIterator<Item=Self::Ref<'a>> + 'a {}
}

#[cfg(not(feature = "generic_traits"))]
mod generics {
    use super::*;

    /**
    The interface for the `Slice` immutable slice struct-of-arrays type.
    */
    pub trait SoASlice<T: StructOfArray> {}

    /**
    The interface for the `SliceMut` mutable slice struct-of-arrays type. A generalization of [`SoASlice`]
    whose methods can modify elements of the arrays
    */
    pub trait SoASliceMut<T: StructOfArray> {}

    /**
    The interface for the `Vec`-like struct-of-arrays type. A generalization of [`SoAMutSlice`] whose methods can
    also re-size the underlying arrays.

    **NOTE**: This interface is incomplete and additional methods may be added as needed.
    */
    pub trait SoAVec<T: StructOfArray> {}

    /// A trait to implement `Clone`-dependent behavior to convert a non-owning SoA type into an
    /// owning [`SoAVec`].
    pub trait ToSoAVec<T: StructOfArray> {}

    /// A trait to implement `Clone`-dependent behavior to extend an [`SoAVec`] with data copied
    /// from its associated `Slice` type.
    pub trait SoAAppendVec<T: StructOfArray>: SoAVec<T> {}

    /// A trait to express the [`IntoIterator`] guarantee of [`SoASlice`] types in the type system.
    pub trait IntoSoAIter<'a, T: StructOfArray> {}
}

pub use generics::*;

/// A collection of supporting traits for [`StructOfArray`] bundled in one place for ease-of-access
pub mod prelude {
    pub use super::{SoAVec, SoAIter, SoASlice, SoASliceMut, SoAPointers, StructOfArray, ToSoAVec, IntoSoAIter, SoAAppendVec};
}


#[macro_export]
#[doc(hidden)]
macro_rules! soa_zip_impl {
    // @flatten creates a tuple-flattening closure for .map() call
    // Finish recursion
    (@flatten $p:pat => $tup:expr ) => {
        |$p| $tup
    };
    // Eat an element ($_iter) and add it to the current closure. Then recurse
    (@flatten $p:pat => ( $($tup:tt)* ) , $_iter:expr $( , $tail:expr )* ) => {
        $crate::soa_zip_impl!(@flatten ($p, a) => ( $($tup)*, a ) $( , $tail )*)
    };

    // The main code is emmited here: we create an iterator, zip it and then
    // map the zipped iterator to flatten it
    (@last , $first: expr, $($tail: expr,)*) => {
        ::std::iter::IntoIterator::into_iter($first)
            $(
                .zip($tail)
            )*
            .map(
                $crate::soa_zip_impl!(@flatten a => (a) $( , $tail )*)
            )
    };

    // Eat the last `mut $field` and then emit code
    (@munch $self: expr, {mut $field: ident} -> [$($output: tt)*] $($ext: expr ,)*) => {
        $crate::soa_zip_impl!(@last $($output)*, $self.$field.iter_mut(), $($ext, )*)
    };
    // Eat the last `$field` and then emit code
    (@munch $self: expr, {$field: ident} -> [$($output: tt)*] $($ext: expr ,)*) => {
        $crate::soa_zip_impl!(@last $($output)*, $self.$field.iter(), $($ext, )*)
    };

    // Eat the next `mut $field` and then recurse
    (@munch $self: expr, {mut $field: ident, $($tail: tt)*} -> [$($output: tt)*] $($ext: expr ,)*) => {
        $crate::soa_zip_impl!(@munch $self, {$($tail)*} -> [$($output)*, $self.$field.iter_mut()] $($ext, )*)
    };
    // Eat the next `$field` and then recurse
    (@munch $self: expr, {$field: ident, $($tail: tt)*} -> [$($output: tt)*] $($ext: expr ,)*) => {
        $crate::soa_zip_impl!(@munch $self, {$($tail)*} -> [$($output)*, $self.$field.iter()] $($ext, )*)
    };
}
