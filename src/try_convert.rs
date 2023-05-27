//! Traits for converting from Ruby [`Value`]s to Rust types.

use std::path::PathBuf;

use rb_sys::{rb_get_path, rb_num2dbl};

#[cfg(ruby_use_flonum)]
use crate::value::Flonum;
use crate::{
    error::{protect, Error},
    exception,
    integer::Integer,
    r_array::RArray,
    r_hash::RHash,
    r_string::RString,
    value::{Fixnum, Value, QNIL},
};

/// Conversions from [`Value`] to Rust types.
pub trait TryConvert: Sized {
    /// Convert `val` into `Self`.
    fn try_convert(val: Value) -> Result<Self, Error>;
}

/// Conversions from [`Value`] to Rust types that do not contain [`Value`].
///
/// This trait is used as a bound on some implementations of [`TryConvert`]
/// (for example, for [`Vec`]) to prevent heap allocated datastructures
/// containing `Value`, as it is not safe to store a `Value` on the heap.
///
/// # Safety
///
/// This trait must not be implemented for types that contain `Value`.
pub unsafe trait TryConvertOwned: TryConvert {}

impl<T> TryConvert for Option<T>
where
    T: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        (!val.is_nil()).then(|| T::try_convert(val)).transpose()
    }
}

unsafe impl<T> TryConvertOwned for Option<T> where T: TryConvertOwned {}

impl TryConvert for bool {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Ok(val.to_bool())
    }
}
unsafe impl TryConvertOwned for bool {}

impl TryConvert for i8 {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_i8()
    }
}
unsafe impl TryConvertOwned for i8 {}

impl TryConvert for i16 {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_i16()
    }
}
unsafe impl TryConvertOwned for i16 {}

impl TryConvert for i32 {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_i32()
    }
}
unsafe impl TryConvertOwned for i32 {}

impl TryConvert for i64 {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_i64()
    }
}
unsafe impl TryConvertOwned for i64 {}

impl TryConvert for isize {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_isize()
    }
}
unsafe impl TryConvertOwned for isize {}

impl TryConvert for u8 {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_u8()
    }
}
unsafe impl TryConvertOwned for u8 {}

impl TryConvert for u16 {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_u16()
    }
}
unsafe impl TryConvertOwned for u16 {}

impl TryConvert for u32 {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_u32()
    }
}
unsafe impl TryConvertOwned for u32 {}

impl TryConvert for u64 {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_u64()
    }
}
unsafe impl TryConvertOwned for u64 {}

impl TryConvert for usize {
    fn try_convert(val: Value) -> Result<Self, Error> {
        Integer::try_convert(val)?.to_usize()
    }
}
unsafe impl TryConvertOwned for usize {}

impl TryConvert for f32 {
    fn try_convert(val: Value) -> Result<Self, Error> {
        f64::try_convert(val).map(|f| f as f32)
    }
}
unsafe impl TryConvertOwned for f32 {}

impl TryConvert for f64 {
    fn try_convert(val: Value) -> Result<Self, Error> {
        if let Some(fixnum) = Fixnum::from_value(val) {
            return Ok(fixnum.to_isize() as f64);
        }
        #[cfg(ruby_use_flonum)]
        if let Some(flonum) = Flonum::from_value(val) {
            return Ok(flonum.to_f64());
        }
        debug_assert_value!(val);
        let mut res = 0.0;
        protect(|| {
            res = unsafe { rb_num2dbl(val.as_rb_value()) };
            QNIL
        })?;
        Ok(res)
    }
}
unsafe impl TryConvertOwned for f64 {}

impl TryConvert for String {
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        RString::try_convert(val)?.to_string()
    }
}
unsafe impl TryConvertOwned for String {}

#[cfg(feature = "bytes-crate")]
impl TryConvert for bytes::Bytes {
    fn try_convert(val: Value) -> Result<bytes::Bytes, Error> {
        debug_assert_value!(val);
        Ok(RString::try_convert(val)?.to_bytes())
    }
}

#[cfg(feature = "bytes-crate")]
unsafe impl TryConvertOwned for bytes::Bytes {}

impl TryConvert for char {
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        RString::try_convert(val)?.to_char()
    }
}
unsafe impl TryConvertOwned for char {}

impl<T> TryConvert for Vec<T>
where
    T: TryConvertOwned,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        RArray::try_convert(val)?.to_vec()
    }
}
unsafe impl<T> TryConvertOwned for Vec<T> where T: TryConvertOwned {}

impl<T, const N: usize> TryConvert for [T; N]
where
    T: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        RArray::try_convert(val)?.to_array()
    }
}
unsafe impl<T, const N: usize> TryConvertOwned for [T; N] where T: TryConvert {}

impl<T0> TryConvert for (T0,)
where
    T0: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 1 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 1",
            ));
        }
        Ok((slice[0].try_convert()?,))
    }
}
unsafe impl<T0> TryConvertOwned for (T0,) where T0: TryConvert {}

impl<T0, T1> TryConvert for (T0, T1)
where
    T0: TryConvert,
    T1: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 2 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 2",
            ));
        }
        Ok((slice[0].try_convert()?, slice[1].try_convert()?))
    }
}
unsafe impl<T0, T1> TryConvertOwned for (T0, T1)
where
    T0: TryConvert,
    T1: TryConvert,
{
}

impl<T0, T1, T2> TryConvert for (T0, T1, T2)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 3 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 3",
            ));
        }
        Ok((
            slice[0].try_convert()?,
            slice[1].try_convert()?,
            slice[2].try_convert()?,
        ))
    }
}
unsafe impl<T0, T1, T2> TryConvertOwned for (T0, T1, T2)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
{
}

impl<T0, T1, T2, T3> TryConvert for (T0, T1, T2, T3)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 4 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 4",
            ));
        }
        Ok((
            slice[0].try_convert()?,
            slice[1].try_convert()?,
            slice[2].try_convert()?,
            slice[3].try_convert()?,
        ))
    }
}
unsafe impl<T0, T1, T2, T3> TryConvertOwned for (T0, T1, T2, T3)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
{
}

impl<T0, T1, T2, T3, T4> TryConvert for (T0, T1, T2, T3, T4)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 5 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 5",
            ));
        }
        Ok((
            slice[0].try_convert()?,
            slice[1].try_convert()?,
            slice[2].try_convert()?,
            slice[3].try_convert()?,
            slice[4].try_convert()?,
        ))
    }
}
unsafe impl<T0, T1, T2, T3, T4> TryConvertOwned for (T0, T1, T2, T3, T4)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
{
}

impl<T0, T1, T2, T3, T4, T5> TryConvert for (T0, T1, T2, T3, T4, T5)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 6 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 6",
            ));
        }
        Ok((
            slice[0].try_convert()?,
            slice[1].try_convert()?,
            slice[2].try_convert()?,
            slice[3].try_convert()?,
            slice[4].try_convert()?,
            slice[5].try_convert()?,
        ))
    }
}
unsafe impl<T0, T1, T2, T3, T4, T5> TryConvertOwned for (T0, T1, T2, T3, T4, T5)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
{
}

impl<T0, T1, T2, T3, T4, T5, T6> TryConvert for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 7 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 7",
            ));
        }
        Ok((
            slice[0].try_convert()?,
            slice[1].try_convert()?,
            slice[2].try_convert()?,
            slice[3].try_convert()?,
            slice[4].try_convert()?,
            slice[5].try_convert()?,
            slice[6].try_convert()?,
        ))
    }
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6> TryConvertOwned for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
{
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> TryConvert for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
    T7: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 8 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 8",
            ));
        }
        Ok((
            slice[0].try_convert()?,
            slice[1].try_convert()?,
            slice[2].try_convert()?,
            slice[3].try_convert()?,
            slice[4].try_convert()?,
            slice[5].try_convert()?,
            slice[6].try_convert()?,
            slice[7].try_convert()?,
        ))
    }
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7> TryConvertOwned for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
    T7: TryConvert,
{
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TryConvert for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
    T7: TryConvert,
    T8: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 9 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 9",
            ));
        }
        Ok((
            slice[0].try_convert()?,
            slice[1].try_convert()?,
            slice[2].try_convert()?,
            slice[3].try_convert()?,
            slice[4].try_convert()?,
            slice[5].try_convert()?,
            slice[6].try_convert()?,
            slice[7].try_convert()?,
            slice[8].try_convert()?,
        ))
    }
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TryConvertOwned
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
    T7: TryConvert,
    T8: TryConvert,
{
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TryConvert for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
    T7: TryConvert,
    T8: TryConvert,
    T9: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 10 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 10",
            ));
        }
        Ok((
            slice[0].try_convert()?,
            slice[1].try_convert()?,
            slice[2].try_convert()?,
            slice[3].try_convert()?,
            slice[4].try_convert()?,
            slice[5].try_convert()?,
            slice[6].try_convert()?,
            slice[7].try_convert()?,
            slice[8].try_convert()?,
            slice[9].try_convert()?,
        ))
    }
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TryConvertOwned
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
    T7: TryConvert,
    T8: TryConvert,
    T9: TryConvert,
{
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TryConvert
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
    T7: TryConvert,
    T8: TryConvert,
    T9: TryConvert,
    T10: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 11 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 11",
            ));
        }
        Ok((
            slice[0].try_convert()?,
            slice[1].try_convert()?,
            slice[2].try_convert()?,
            slice[3].try_convert()?,
            slice[4].try_convert()?,
            slice[5].try_convert()?,
            slice[6].try_convert()?,
            slice[7].try_convert()?,
            slice[8].try_convert()?,
            slice[9].try_convert()?,
            slice[10].try_convert()?,
        ))
    }
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TryConvertOwned
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
    T7: TryConvert,
    T8: TryConvert,
    T9: TryConvert,
    T10: TryConvert,
{
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TryConvert
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
    T7: TryConvert,
    T8: TryConvert,
    T9: TryConvert,
    T10: TryConvert,
    T11: TryConvert,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        let array = RArray::try_convert(val)?;
        let slice = unsafe { array.as_slice() };
        if slice.len() != 12 {
            return Err(Error::new(
                exception::type_error(),
                "expected Array of length 12",
            ));
        }
        Ok((
            slice[0].try_convert()?,
            slice[1].try_convert()?,
            slice[2].try_convert()?,
            slice[3].try_convert()?,
            slice[4].try_convert()?,
            slice[5].try_convert()?,
            slice[6].try_convert()?,
            slice[7].try_convert()?,
            slice[8].try_convert()?,
            slice[9].try_convert()?,
            slice[10].try_convert()?,
            slice[11].try_convert()?,
        ))
    }
}
unsafe impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TryConvertOwned
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: TryConvert,
    T1: TryConvert,
    T2: TryConvert,
    T3: TryConvert,
    T4: TryConvert,
    T5: TryConvert,
    T6: TryConvert,
    T7: TryConvert,
    T8: TryConvert,
    T9: TryConvert,
    T10: TryConvert,
    T11: TryConvert,
{
}

impl<K, V> TryConvert for std::collections::HashMap<K, V>
where
    K: TryConvertOwned + Eq + std::hash::Hash,
    V: TryConvertOwned,
{
    fn try_convert(val: Value) -> Result<Self, Error> {
        debug_assert_value!(val);
        RHash::try_convert(val)?.to_hash_map()
    }
}
unsafe impl<K, V> TryConvertOwned for std::collections::HashMap<K, V>
where
    K: TryConvertOwned + Eq + std::hash::Hash,
    V: TryConvertOwned,
{
}

#[cfg(unix)]
impl TryConvert for PathBuf {
    fn try_convert(val: Value) -> Result<Self, Error> {
        use std::os::unix::ffi::OsStringExt;

        let bytes = unsafe {
            let r_string =
                protect(|| RString::from_rb_value_unchecked(rb_get_path(val.as_rb_value())))?;
            r_string.as_slice().to_owned()
        };
        Ok(std::ffi::OsString::from_vec(bytes).into())
    }
}

#[cfg(not(unix))]
impl TryConvert for PathBuf {
    fn try_convert(val: Value) -> Result<Self, Error> {
        protect(|| unsafe { RString::from_rb_value_unchecked(rb_get_path(val.as_rb_value())) })?
            .to_string()
            .map(Into::into)
    }
}

unsafe impl TryConvertOwned for PathBuf {}
