//! Make elements of an array or a vector movable with some simple magic.
//!
//! ## Vec -> array
//!
//! Move elements of a vector an array.
//! ```rust
//! use moving::move_vec_to_array;
//!
//! let v = vec![0, 1, 2, 3, 4];  // 5 items
//! let arr = move_vec_to_array::<i16, 5>(v).unwrap();
//!
//! assert_eq!(arr, [0, 1, 2, 3, 4]);
//! ```
//!
//! ## Movable vectors
//!
//! To make elements of an array or a vector movable, while the size is unknown, use `movable`:
//! ```rust
//! use moving::{ MovableVec, movable };
//!
//! let v = vec![0, 1, 2, 3, 4];  // 5 items
//! let arr = [0, 1, 2, 3, 4];
//!
//! let mvv: MovableVec<i32> = movable(v);
//! let mvv_arr: MovableVec<i32> = movable(arr);
//! ```
//!
//! Alternatively, you can use `ToMovable`:
//! ```no_run
//! use moving::ToMovable;
//!
//! some_vec.to_movable();
//! some_arr.to_movalbe();
//! ```
//!
//! ## Movable arrays
//!
//! To make elements of an array or a vector movable, while the size is known, use `nmovable` (notice the prefix "n"):
//! ```rust
//! use moving::{ MovableArray, nmovable};
//!
//! let v = vec![0, 1, 2, 3, 4];  // 5 items
//! let arr = [0, 1, 2, 3, 4];
//!
//! let mvv: MovableArray<i32, 5> = nmovable(v).unwrap();
//! let mvv_arr: MovableArray<i32, 5> = nmovable(arr).unwrap();
//! ```
//!
//! Alternatively, you can use `ToNMovable`:
//! ```no_run
//! use moving::ToNMovable;
//!
//! some_vec.to_nmovable()?;
//! some_arr.to_nmovable()?;
//! ```

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MovingArrayError {
    #[error("Expected length & capacity {expected}, got {got}")] LengthUnmatch {
        expected: usize,
        got: usize,
    },
}

/// Moves a vector to an array.
///
/// # Example
///
/// ```no_run
/// let v = vec![0, 1, 2, 3, 4];  // 5 items
/// let arr = move_vec_to_array::<i16, 5>(v).unwrap();
///
/// assert_eq!(arr, [0, 1, 2, 3, 4]);
/// ```
pub fn move_vec_to_array<T, const N: usize>(mut vec: Vec<T>) -> Result<[T; N], MovingArrayError> {
    if vec.len() != N || vec.capacity() != N {
        return Err(MovingArrayError::LengthUnmatch { expected: N, got: vec.len() });
    }

    let ptr = vec.as_mut_ptr();
    core::mem::forget(vec);

    Ok(unsafe { ptr.cast::<[T; N]>().read() })
}

/// An array with elements that can be moved out.
#[derive(Debug, Clone)]
pub struct MovableArray<T, const N: usize>([Option<T>; N]);

impl<T, const N: usize> MovableArray<T, N> {
    pub fn from_vec(vec: Vec<T>) -> Result<Self, MovingArrayError> {
        if vec.len() != N || vec.capacity() != N {
            return Err(MovingArrayError::LengthUnmatch { expected: N, got: vec.len() });
        }

        let mut iter = vec.into_iter();
        Ok(Self(core::array::from_fn::<Option<T>, N, _>(|_| iter.next())))
    }

    pub fn from_array(array: [T; N]) -> Self {
        let mut iter = array.into_iter();
        Self(core::array::from_fn::<Option<T>, N, _>(|_| iter.next()))
    }

    /// Take an element from the array.
    #[inline]
    pub const fn take(&mut self, index: usize) -> Option<T> {
        self.0[index].take()
    }

    /// Take elements (in a specific range) from an array, as an array.
    pub fn take_range_array<const S: usize>(
        &mut self,
        range: core::ops::Range<usize>
    ) -> Result<[Option<T>; S], MovingArrayError> {
        move_vec_to_array(self.take_range_vec(range))
    }

    /// Take elements (in a specific range) from an array, as a vector.
    pub fn take_range_vec(&mut self, range: core::ops::Range<usize>) -> Vec<Option<T>> {
        range.map(|i| self.0.get_mut(i).and_then(|v| v.take())).collect()
    }

    pub const fn len(&self) -> usize {
        N
    }

    pub fn into_inner(self) -> [Option<T>; N] {
        self.0
    }

    /// Map. If an element is taken away, `None` is present.
    pub fn map<R>(self, f: fn(Option<T>) -> Option<R>) -> MovableArray<R, N> {
        let mut iter = self.0.into_iter();
        MovableArray(core::array::from_fn::<Option<R>, N, _>(|_| f(iter.next().unwrap())))
    }
}

impl<T, const N: usize> core::ops::Index<usize> for MovableArray<T, N> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> core::ops::IndexMut<usize> for MovableArray<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub enum VecOrArray<T, const N: usize = 0> {
    Vec(Vec<T>),
    Array([T; N]),
}

pub trait IntoVecOrArray<T, const N: usize = 0> {
    fn vec_or_array(self) -> VecOrArray<T, N>;
}

impl<T, const N: usize> IntoVecOrArray<T, N> for Vec<T> {
    fn vec_or_array(self) -> VecOrArray<T, N> {
        VecOrArray::Vec(self)
    }
}

impl<T, const N: usize> IntoVecOrArray<T, N> for [T; N] {
    fn vec_or_array(self) -> VecOrArray<T, N> {
        VecOrArray::Array(self)
    }
}

/// Turn an an array or vec into a movable array with fixed size.
///
/// # Example
///
/// ```no_run
/// let v = vec![1, 2, 3, 4, 5];
/// let mva: MovableArray<i32, 5> = nmovable(v).unwrap();
/// ```
pub fn nmovable<T, const N: usize>(
    input: impl IntoVecOrArray<T, N>
) -> Result<MovableArray<T, N>, MovingArrayError> {
    match input.vec_or_array() {
        VecOrArray::Array(arr) => Ok(MovableArray::from_array(arr)),
        VecOrArray::Vec(vec) => MovableArray::from_vec(vec),
    }
}

pub trait ToNMovable<T, const N: usize> {
    fn to_nmovable(self) -> Result<MovableArray<T, N>, MovingArrayError>;
}

impl<T, const N: usize> ToNMovable<T, N> for Vec<T> {
    fn to_nmovable(self) -> Result<MovableArray<T, N>, MovingArrayError> {
        MovableArray::from_vec(self)
    }
}

impl<T, const N: usize> ToNMovable<T, N> for [T; N] {
    /// Convert this array into a [`MovableArray`]. **Size is known.**
    ///
    /// You can safely `.unwrap()` the `Result`.
    fn to_nmovable(self) -> Result<MovableArray<T, N>, MovingArrayError> {
        Ok(MovableArray::from_array(self))
    }
}

/// A `Vec` with elements that can be moved out.
pub struct MovableVec<T>(Vec<Option<T>>);

impl<T> MovableVec<T> {
    pub fn from_vec(v: Vec<T>) -> Self {
        Self(
            v
                .into_iter()
                .map(|item| Some(item))
                .collect()
        )
    }

    pub fn from_array<const N: usize>(arr: [T; N]) -> Self {
        let ln = arr.len();
        let mut iter = arr.into_iter();
        let mut vec = Vec::with_capacity(ln);
        while let Some(item) = iter.next() {
            vec.push(Some(item));
        }
        Self(vec)
    }

    /// Take an element from the array.
    #[inline]
    pub fn take(&mut self, index: usize) -> Option<T> {
        self.0[index].take()
    }

    /// Take elements (in a specific range) from an array, as an array.
    pub fn take_range_array<const S: usize>(
        &mut self,
        range: core::ops::Range<usize>
    ) -> Result<[Option<T>; S], MovingArrayError> {
        move_vec_to_array(self.take_range_vec(range))
    }

    /// Take elements (in a specific range) from an array, as a vector.
    pub fn take_range_vec(&mut self, range: core::ops::Range<usize>) -> Vec<Option<T>> {
        range.map(|i| self.0.get_mut(i).and_then(|v| v.take())).collect()
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub fn into_inner(self) -> Vec<Option<T>> {
        self.0
    }

    /// Map. If an element is taken away, `None` is present.
    pub fn map<R>(self, f: fn(Option<T>) -> Option<R>) -> MovableVec<R> {
        MovableVec(
            self.0
                .into_iter()
                .map(|item| f(item))
                .collect()
        )
    }
}

/// Turn an an array or vec into a movable vec with unknown size.
///
/// # Example
///
/// ```no_run
/// let v = vec![1, 2, 3, 4, 5];
/// let mvv: MovableVec<i32> = movable(v).unwrap();
/// ```
pub fn movable<T, const N: usize>(input: impl IntoVecOrArray<T, N>) -> MovableVec<T> {
    match input.vec_or_array() {
        VecOrArray::Array(a) => MovableVec::from_array(a),
        VecOrArray::Vec(v) => MovableVec::from_vec(v),
    }
}

pub trait ToMovable<T> {
    fn to_movable(self) -> MovableVec<T>;
}

impl<T> ToMovable<T> for Vec<T> {
    fn to_movable(self) -> MovableVec<T> {
        MovableVec::from_vec(self)
    }
}

impl<T, const N: usize> ToMovable<T> for [T; N] {
    /// Convert this array into a [`MovableVec`]. **Size is NOT known.**
    ///
    /// You can safely `.unwrap()` the `Result`.
    fn to_movable(self) -> MovableVec<T> {
        MovableVec::from_array(self)
    }
}
