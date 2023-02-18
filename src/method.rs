//! Types and Traits for exposing Rust functions as Ruby methods.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::missing_safety_doc)]

use std::{ffi::c_void, marker::PhantomData, os::raw::c_int, panic::AssertUnwindSafe, slice};

use crate::{
    block::{
        do_yield_iter, do_yield_splat_iter, do_yield_values_iter, Proc, Yield, YieldSplat,
        YieldValues,
    },
    error::{raise, Error},
    into_value::{ArgList, IntoValue},
    r_array::RArray,
    try_convert::TryConvert,
    value::Value,
};

mod private {
    use super::*;

    pub unsafe trait Method {
        fn arity() -> i8;
        #[allow(clippy::wrong_self_convention)]
        fn as_ptr(self) -> *mut c_void;
    }

    unsafe impl Method for unsafe extern "C" fn(Value, RArray) -> Value {
        fn arity() -> i8 {
            -2
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method for unsafe extern "C" fn(c_int, *const Value, Value) -> Value {
        fn arity() -> i8 {
            -1
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method for unsafe extern "C" fn(Value) -> Value {
        fn arity() -> i8 {
            0
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method for unsafe extern "C" fn(Value, Value) -> Value {
        fn arity() -> i8 {
            1
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method for unsafe extern "C" fn(Value, Value, Value) -> Value {
        fn arity() -> i8 {
            2
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method for unsafe extern "C" fn(Value, Value, Value, Value) -> Value {
        fn arity() -> i8 {
            3
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method for unsafe extern "C" fn(Value, Value, Value, Value, Value) -> Value {
        fn arity() -> i8 {
            4
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method for unsafe extern "C" fn(Value, Value, Value, Value, Value, Value) -> Value {
        fn arity() -> i8 {
            5
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method
        for unsafe extern "C" fn(Value, Value, Value, Value, Value, Value, Value) -> Value
    {
        fn arity() -> i8 {
            6
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method
        for unsafe extern "C" fn(Value, Value, Value, Value, Value, Value, Value, Value) -> Value
    {
        fn arity() -> i8 {
            7
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method
        for unsafe extern "C" fn(
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
        ) -> Value
    {
        fn arity() -> i8 {
            8
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method
        for unsafe extern "C" fn(
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
        ) -> Value
    {
        fn arity() -> i8 {
            9
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method
        for unsafe extern "C" fn(
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
        ) -> Value
    {
        fn arity() -> i8 {
            10
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method
        for unsafe extern "C" fn(
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
        ) -> Value
    {
        fn arity() -> i8 {
            11
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method
        for unsafe extern "C" fn(
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
        ) -> Value
    {
        fn arity() -> i8 {
            12
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method
        for unsafe extern "C" fn(
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
        ) -> Value
    {
        fn arity() -> i8 {
            13
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method
        for unsafe extern "C" fn(
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
        ) -> Value
    {
        fn arity() -> i8 {
            14
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method
        for unsafe extern "C" fn(
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
        ) -> Value
    {
        fn arity() -> i8 {
            15
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    unsafe impl Method
        for unsafe extern "C" fn(
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
            Value,
        ) -> Value
    {
        fn arity() -> i8 {
            16
        }

        fn as_ptr(self) -> *mut c_void {
            self as *mut c_void
        }
    }

    pub trait ReturnValue {
        fn into_return_value(self) -> Result<Value, Error>;
    }

    impl<T> ReturnValue for Result<T, Error>
    where
        T: IntoValue,
    {
        fn into_return_value(self) -> Result<Value, Error> {
            self.map(|val| unsafe { val.into_value_unchecked() })
        }
    }

    impl<T> ReturnValue for T
    where
        T: IntoValue,
    {
        fn into_return_value(self) -> Result<Value, Error> {
            Ok(self).into_return_value()
        }
    }

    impl<I, T> ReturnValue for Yield<I>
    where
        I: Iterator<Item = T>,
        T: IntoValue,
    {
        fn into_return_value(self) -> Result<Value, Error> {
            match self {
                Yield::Iter(iter) => {
                    unsafe { do_yield_iter(iter) };
                    Ok(Value::default())
                }
                Yield::Enumerator(e) => Ok(e.into()),
            }
        }
    }

    impl<I, T> ReturnValue for Result<Yield<I>, Error>
    where
        I: Iterator<Item = T>,
        T: IntoValue,
    {
        fn into_return_value(self) -> Result<Value, Error> {
            self?.into_return_value()
        }
    }

    impl<I, T> ReturnValue for YieldValues<I>
    where
        I: Iterator<Item = T>,
        T: ArgList,
    {
        fn into_return_value(self) -> Result<Value, Error> {
            match self {
                YieldValues::Iter(iter) => {
                    unsafe { do_yield_values_iter(iter) };
                    Ok(Value::default())
                }
                YieldValues::Enumerator(e) => Ok(e.into()),
            }
        }
    }

    impl<I, T> ReturnValue for Result<YieldValues<I>, Error>
    where
        I: Iterator<Item = T>,
        T: ArgList,
    {
        fn into_return_value(self) -> Result<Value, Error> {
            self?.into_return_value()
        }
    }

    impl<I> ReturnValue for YieldSplat<I>
    where
        I: Iterator<Item = RArray>,
    {
        fn into_return_value(self) -> Result<Value, Error> {
            match self {
                YieldSplat::Iter(iter) => {
                    unsafe { do_yield_splat_iter(iter) };
                    Ok(Value::default())
                }
                YieldSplat::Enumerator(e) => Ok(e.into()),
            }
        }
    }

    impl<I> ReturnValue for Result<YieldSplat<I>, Error>
    where
        I: Iterator<Item = RArray>,
    {
        fn into_return_value(self) -> Result<Value, Error> {
            self?.into_return_value()
        }
    }

    pub trait InitReturn {
        fn into_init_return(self) -> Result<(), Error>;
    }

    impl InitReturn for () {
        fn into_init_return(self) -> Result<(), Error> {
            Ok(())
        }
    }

    impl InitReturn for Result<(), Error> {
        fn into_init_return(self) -> Result<(), Error> {
            self
        }
    }

    pub trait BlockReturn {
        fn into_block_return(self) -> Result<Value, Error>;
    }

    impl<T> BlockReturn for Result<T, Error>
    where
        T: IntoValue,
    {
        fn into_block_return(self) -> Result<Value, Error> {
            self.map(|val| unsafe { val.into_value_unchecked() })
        }
    }

    impl<T> BlockReturn for T
    where
        T: IntoValue,
    {
        fn into_block_return(self) -> Result<Value, Error> {
            Ok(self).into_block_return()
        }
    }
}

/// Trait implemented for function pointers that can be registed as Ruby
/// methods.
///
/// While it is possible to directly write functions that will automatically
/// implement this trait it is not recommended, as those functions will not
/// have the type conversions or error handling usually provided by this
/// library. See the [`method`](crate::method!) and
/// [`function`](crate::function!) macros for converting functions to an
/// implementor of this trait.
///
/// This trait is implemented for the following function signatures:
///
/// | Arity | Signature                                                    |
/// |-------|--------------------------------------------------------------|
/// |    -2 | `unsafe extern "C" fn(Value, RArray) -> Value;`              |
/// |    -1 | `unsafe extern "C" fn(c_int, *const Value, Value) -> Value;` |
/// |     0 | `unsafe extern "C" fn(Value) -> Value;`                      |
/// |     1 | `unsafe extern "C" fn(Value, Value) -> Value;`               |
/// |     2 | `unsafe extern "C" fn(Value, Value, Value) -> Value;`        |
/// |   ... | ...                                                          |
/// |    16 | ...                                                          |
///
/// note: for arity 0..=16 the number of arguments is 1 greater than the arity,
/// due to the initial `self` argument.
pub trait Method: private::Method {}

impl<T> Method for T where T: private::Method {}

/// Trait marking types that can be returned to Ruby.
///
/// Implemented for the following types:
///
/// * `T`
/// * [`Yield<I>`]
/// * [`YieldValues<I>`]
/// * [`YieldSplat<I>`]
/// * `Result<T, magnus::Error>`
/// * `Result<Yield<I>, magnus::Error>`
/// * `Result<YieldValues<I>, magnus::Error>`
/// * `Result<YieldSplat<I>, magnus::Error>`
///
/// where `I` implements `Iterator<Item = T>` and `T` implements [`IntoValue`].
///
/// When is `Err(magnus::Error)` returned to Ruby it will be conveted to and
/// raised as a Ruby exception.
///
/// [`Yield`], [`YieldValues`], and [`YieldSplat`] allow returning a Rust
/// [`Iterator`] to be bridged to Ruby method that calls a block with the
/// elements of that [`Iterator`].
///
/// Note: functions without a specified return value will return `()`. `()`
/// implements [`IntoValue`] (converting to `nil`).
pub trait ReturnValue: private::ReturnValue {}

impl<T> ReturnValue for T where T: private::ReturnValue {}

/// Trait marking types that can be returned to Ruby from a library
/// [`init`](magnus_macros::init) function.
///
/// Implemented for the following types:
///
/// * `()`
/// * `Result<(), magnus::Error>`
///
/// When is `Err(magnus::Error)` returned to Ruby it will be conveted to and
/// raised as a Ruby exception.
///
/// Note: functions without a specified return value will return `()`. `()`
/// implements [`IntoValue`] (converting to `nil`).
pub trait InitReturn: private::InitReturn {}

impl<T> InitReturn for T where T: private::InitReturn {}

/// Trait marking types that can be returned to Ruby from a block.
///
/// Implemented for the following types:
///
/// * `T`
/// * `Result<T, magnus::Error>`
///
/// where `T` implements [`IntoValue`].
///
/// When is `Err(magnus::Error)` returned to Ruby it will be conveted to and
/// raised as a Ruby exception.
///
/// Note: functions without a specified return value will return `()`. `()`
/// implements [`IntoValue`] (converting to `nil`).
pub trait BlockReturn: private::BlockReturn {}

impl<T> BlockReturn for T where T: private::BlockReturn {}

/// Helper type for wrapping a function with type conversions and error
/// handling, as an 'init' function.
///
/// See the [`init`](magnus_macros::init) macro.
#[doc(hidden)]
pub struct Init<Func, Res> {
    func: Func,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, Res> Init<Func, Res>
where
    Func: Fn() -> Res,
    Res: InitReturn,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            res: Default::default(),
        }
    }

    #[inline]
    pub unsafe fn call_handle_error(self) {
        let res =
            match std::panic::catch_unwind(AssertUnwindSafe(|| (self.func)().into_init_return())) {
                Ok(v) => v,
                Err(e) => Err(Error::from_panic(e)),
            };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function with type conversions and error
/// handling, as an 'block' function.
///
/// See the [`Value::block_call`] function.
#[doc(hidden)]
pub struct Block<Func, Res> {
    func: Func,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, Res> Block<Func, Res>
where
    Func: FnMut(&[Value], Option<Proc>) -> Res,
    Res: BlockReturn,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            res: Default::default(),
        }
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        argc: c_int,
        argv: *const Value,
        blockarg: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(argc, argv, blockarg)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        mut self,
        argc: c_int,
        argv: *const Value,
        blockarg: Value,
    ) -> Result<Value, Error> {
        let args = slice::from_raw_parts(argv, argc as usize);
        (self.func)(args, Proc::from_value(blockarg)).into_block_return()
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and a Ruby
/// array of arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct MethodRbAry<Func, RbSelf, Args, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    args: PhantomData<Args>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, Args, Res> MethodRbAry<Func, RbSelf, Args, Res>
where
    Func: Fn(RbSelf, Args) -> Res,
    RbSelf: TryConvert,
    Args: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            args: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(self, rb_self: Value, args: RArray) -> Result<Value, Error> {
        (self.func)(rb_self.try_convert()?, args.try_convert()?).into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self, rb_self: Value, args: RArray) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, args)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and a
/// slice of arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct MethodCAry<Func, RbSelf, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, Res> MethodCAry<Func, RbSelf, Res>
where
    Func: Fn(RbSelf, &[Value]) -> Res,
    RbSelf: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        argc: c_int,
        argv: *const Value,
        rb_self: Value,
    ) -> Result<Value, Error> {
        let args = slice::from_raw_parts(argv, argc as usize);
        (self.func)(rb_self.try_convert()?, args).into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        argc: c_int,
        argv: *const Value,
        rb_self: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(argc, argv, rb_self)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and no
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method0<Func, RbSelf, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, Res> Method0<Func, RbSelf, Res>
where
    Func: Fn(RbSelf) -> Res,
    RbSelf: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),

            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(self, rb_self: Value) -> Result<Value, Error> {
        (self.func)(rb_self.try_convert()?).into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self, rb_self: Value) -> Value {
        let res =
            match std::panic::catch_unwind(AssertUnwindSafe(|| self.call_convert_value(rb_self))) {
                Ok(v) => v,
                Err(e) => Err(Error::from_panic(e)),
            };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 1
/// argument, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method1<Func, RbSelf, A, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, Res> Method1<Func, RbSelf, A, Res>
where
    Func: Fn(RbSelf, A) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(self, rb_self: Value, a: Value) -> Result<Value, Error> {
        (self.func)(rb_self.try_convert()?, a.try_convert()?).into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self, rb_self: Value, a: Value) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 2
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method2<Func, RbSelf, A, B, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, Res> Method2<Func, RbSelf, A, B, Res>
where
    Func: Fn(RbSelf, A, B) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(self, rb_self: Value, a: Value, b: Value) -> Result<Value, Error> {
        (self.func)(rb_self.try_convert()?, a.try_convert()?, b.try_convert()?).into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self, rb_self: Value, a: Value, b: Value) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 3
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method3<Func, RbSelf, A, B, C, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, Res> Method3<Func, RbSelf, A, B, C, Res>
where
    Func: Fn(RbSelf, A, B, C) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self, rb_self: Value, a: Value, b: Value, c: Value) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 4
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method4<Func, RbSelf, A, B, C, D, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, Res> Method4<Func, RbSelf, A, B, C, D, Res>
where
    Func: Fn(RbSelf, A, B, C, D) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 5
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method5<Func, RbSelf, A, B, C, D, E, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, Res> Method5<Func, RbSelf, A, B, C, D, E, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 6
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method6<Func, RbSelf, A, B, C, D, E, F, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, F, Res> Method6<Func, RbSelf, A, B, C, D, E, F, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E, F) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e, f)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 7
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method7<Func, RbSelf, A, B, C, D, E, F, G, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, F, G, Res> Method7<Func, RbSelf, A, B, C, D, E, F, G, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E, F, G) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e, f, g)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 8
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method8<Func, RbSelf, A, B, C, D, E, F, G, H, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, F, G, H, Res> Method8<Func, RbSelf, A, B, C, D, E, F, G, H, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E, F, G, H) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e, f, g, h)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 9
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method9<Func, RbSelf, A, B, C, D, E, F, G, H, I, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, F, G, H, I, Res>
    Method9<Func, RbSelf, A, B, C, D, E, F, G, H, I, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E, F, G, H, I) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e, f, g, h, i)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 10
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method10<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, Res>
    Method10<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E, F, G, H, I, J) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e, f, g, h, i, j)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 11
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method11<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, Res>
    Method11<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E, F, G, H, I, J, K) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e, f, g, h, i, j, k)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 12
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method12<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    l: PhantomData<L>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, Res>
    Method12<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E, F, G, H, I, J, K, L) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    L: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            l: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
            l.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e, f, g, h, i, j, k, l)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 13
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method13<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    l: PhantomData<L>,
    m: PhantomData<M>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, Res>
    Method13<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    L: TryConvert,
    M: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            l: Default::default(),
            m: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
            l.try_convert()?,
            m.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e, f, g, h, i, j, k, l, m)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 14
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method14<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    l: PhantomData<L>,
    m: PhantomData<M>,
    n: PhantomData<N>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N, Res>
    Method14<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    L: TryConvert,
    M: TryConvert,
    N: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            l: Default::default(),
            m: Default::default(),
            n: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
            l.try_convert()?,
            m.try_convert()?,
            n.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e, f, g, h, i, j, k, l, m, n)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 15
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method15<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    l: PhantomData<L>,
    m: PhantomData<M>,
    n: PhantomData<N>,
    o: PhantomData<O>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, Res>
    Method15<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    L: TryConvert,
    M: TryConvert,
    N: TryConvert,
    O: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            l: Default::default(),
            m: Default::default(),
            n: Default::default(),
            o: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
        o: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
            l.try_convert()?,
            m.try_convert()?,
            n.try_convert()?,
            o.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
        o: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method taking self and 16
/// arguments, with type conversions and error handling.
///
/// See the [`method`](crate::method!) macro.
#[doc(hidden)]
pub struct Method16<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Res> {
    func: Func,
    rb_self: PhantomData<RbSelf>,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    l: PhantomData<L>,
    m: PhantomData<M>,
    n: PhantomData<N>,
    o: PhantomData<O>,
    p: PhantomData<P>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Res>
    Method16<Func, RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Res>
where
    Func: Fn(RbSelf, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) -> Res,
    RbSelf: TryConvert,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    L: TryConvert,
    M: TryConvert,
    N: TryConvert,
    O: TryConvert,
    P: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            rb_self: Default::default(),
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            l: Default::default(),
            m: Default::default(),
            n: Default::default(),
            o: Default::default(),
            p: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
        o: Value,
        p: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            rb_self.try_convert()?,
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
            l.try_convert()?,
            m.try_convert()?,
            n.try_convert()?,
            o.try_convert()?,
            p.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        rb_self: Value,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
        o: Value,
        p: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(rb_self, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Wrap a Rust function item with Ruby type conversion and error handling.
///
/// This macro wraps the given function and returns a function pointer
/// implementing the [`Method`] trait, suitable for passing to functions that
/// define Ruby methods such as
/// [`define_method`](crate::module::Module::define_method).
///
/// Ruby code implicitly always has a `self` parameter available. In the
/// extention API this is passed explicitly. As a result there is always an
/// extra `self` argument before the arguments explitly passed in Ruby, and the
/// number of Rust argument will be one more than the Ruby arity.
///
/// The values `-2` and `-1` for `arity` have special meaning. Both indicate
/// functions with any number of arguments, with `-2` the arguments are passed
/// as a [`RArray`], with `-1` they are passed as a slice of [`Value`]s.
/// Arity of `-1` can be used with [`scan_args`](crate::scan_args::scan_args)
/// and [`get_kwargs`](crate::scan_args::get_kwargs) for more complex method
/// signatures.
///
/// | Arity | Signature                                                 |
/// |-------|-----------------------------------------------------------|
/// |    -2 | `fn(rb_self: T, arguments: RArray) -> Result<R, Error>`   |
/// |    -1 | `fn(rb_self: T, arguments: &[Value]) -> Result<R, Error>` |
/// |     0 | `fn(rb_self: T) -> Result<R, Error>`                      |
/// |     1 | `fn(rb_self: T, arg1: U) -> Result<R, Error>`             |
/// |     2 | `fn(rb_self: T, arg1: U, arg2: V) -> Result<R, Error>`    |
/// |   ... | ...                                                       |
/// |    16 | ...                                                       |
///
/// Where `T`, `U`, `V` and so on are any types that implement `TryConvert`,
/// and `R` implements [`IntoValue`]. It is also possible to return just `R`
/// rather than a `Result` for functions that will never error, and omit the
/// return value (i.e. return `()`) for a function that returns `nil` to Ruby.
/// See [`ReturnValue`] for more details on what can be returned.
///
/// See the [`function`](crate::function!) macro for cases where there is no
/// need to handle the `self` argument.
///
/// # Examples
///
/// ```
/// use magnus::{class, define_class, method, prelude::*, Error};
///
/// fn rb_is_blank(rb_self: String) -> bool {
///     rb_self.contains(|c: char| !c.is_whitespace())
/// }
///
/// #[magnus::init]
/// fn init() -> Result<(), Error> {
///     let class = define_class("String", class::object())?;
///     class.define_method("blank?", method!(rb_is_blank, 0))?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! method {
    ($name:expr, -2) => {{
        unsafe extern "C" fn anon(rb_self: $crate::Value, args: $crate::RArray) -> $crate::Value {
            $crate::method::MethodRbAry::new($name).call_handle_error(rb_self, args)
        }
        anon as unsafe extern "C" fn($crate::Value, $crate::RArray) -> $crate::Value
    }};
    ($name:expr, -1) => {{
        unsafe extern "C" fn anon(
            argc: std::os::raw::c_int,
            argv: *const $crate::Value,
            rb_self: $crate::Value,
        ) -> $crate::Value {
            $crate::method::MethodCAry::new($name).call_handle_error(argc, argv, rb_self)
        }
        anon as unsafe extern "C" fn(
            std::os::raw::c_int,
            *const $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 0) => {{
        unsafe extern "C" fn anon(rb_self: $crate::Value) -> $crate::Value {
            $crate::method::Method0::new($name).call_handle_error(rb_self)
        }
        anon as unsafe extern "C" fn($crate::Value) -> $crate::Value
    }};
    ($name:expr, 1) => {{
        unsafe extern "C" fn anon(rb_self: $crate::Value, a: $crate::Value) -> $crate::Value {
            $crate::method::Method1::new($name).call_handle_error(rb_self, a)
        }
        anon as unsafe extern "C" fn($crate::Value, $crate::Value) -> $crate::Value
    }};
    ($name:expr, 2) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method2::new($name).call_handle_error(rb_self, a, b)
        }
        anon as unsafe extern "C" fn($crate::Value, $crate::Value, $crate::Value) -> $crate::Value
    }};
    ($name:expr, 3) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method3::new($name).call_handle_error(rb_self, a, b, c)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 4) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method4::new($name).call_handle_error(rb_self, a, b, c, d)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 5) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method5::new($name).call_handle_error(rb_self, a, b, c, d, e)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 6) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method6::new($name).call_handle_error(rb_self, a, b, c, d, e, f)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 7) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method7::new($name).call_handle_error(rb_self, a, b, c, d, e, f, g)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 8) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method8::new($name).call_handle_error(rb_self, a, b, c, d, e, f, g, h)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 9) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method9::new($name)
                .call_handle_error(rb_self, a, b, c, d, e, f, g, h, i)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 10) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method10::new($name)
                .call_handle_error(rb_self, a, b, c, d, e, f, g, h, i, j)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 11) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method11::new($name)
                .call_handle_error(rb_self, a, b, c, d, e, f, g, h, i, j, k)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 12) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
            l: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method12::new($name)
                .call_handle_error(rb_self, a, b, c, d, e, f, g, h, i, j, k, l)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 13) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
            l: $crate::Value,
            m: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method13::new($name)
                .call_handle_error(rb_self, a, b, c, d, e, f, g, h, i, j, k, l, m)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 14) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
            l: $crate::Value,
            m: $crate::Value,
            n: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method14::new($name)
                .call_handle_error(rb_self, a, b, c, d, e, f, g, h, i, j, k, l, m, n)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 15) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
            l: $crate::Value,
            m: $crate::Value,
            n: $crate::Value,
            o: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method15::new($name)
                .call_handle_error(rb_self, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 16) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
            l: $crate::Value,
            m: $crate::Value,
            n: $crate::Value,
            o: $crate::Value,
            p: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Method16::new($name)
                .call_handle_error(rb_self, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, $arity:expr) => {
        compile_error!("arity must be an integer literal between -2..=16")
    };
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking a Ruby array of arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct FunctionRbAry<Func, Args, Res> {
    func: Func,
    args: PhantomData<Args>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, Args, Res> FunctionRbAry<Func, Args, Res>
where
    Func: Fn(Args) -> Res,
    Args: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            args: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(self, args: RArray) -> Result<Value, Error> {
        (self.func)(args.try_convert()?).into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self, args: RArray) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| self.call_convert_value(args)))
        {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking a slice of arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct FunctionCAry<Func, Res> {
    func: Func,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, Res> FunctionCAry<Func, Res>
where
    Func: Fn(&[Value]) -> Res,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(self, argc: c_int, argv: *const Value) -> Result<Value, Error> {
        let args = slice::from_raw_parts(argv, argc as usize);
        (self.func)(args).into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self, argc: c_int, argv: *const Value) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(argc, argv)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking no arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function0<Func, Res> {
    func: Func,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, Res> Function0<Func, Res>
where
    Func: Fn() -> Res,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,

            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(self) -> Result<Value, Error> {
        (self.func)().into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| self.call_convert_value())) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 1 argument, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function1<Func, A, Res> {
    func: Func,
    a: PhantomData<A>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, Res> Function1<Func, A, Res>
where
    Func: Fn(A) -> Res,
    A: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(self, a: Value) -> Result<Value, Error> {
        (self.func)(a.try_convert()?).into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self, a: Value) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| self.call_convert_value(a))) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 2 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function2<Func, A, B, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, Res> Function2<Func, A, B, Res>
where
    Func: Fn(A, B) -> Res,
    A: TryConvert,
    B: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(self, a: Value, b: Value) -> Result<Value, Error> {
        (self.func)(a.try_convert()?, b.try_convert()?).into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self, a: Value, b: Value) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| self.call_convert_value(a, b)))
        {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 3 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function3<Func, A, B, C, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, Res> Function3<Func, A, B, C, Res>
where
    Func: Fn(A, B, C) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(self, a: Value, b: Value, c: Value) -> Result<Value, Error> {
        (self.func)(a.try_convert()?, b.try_convert()?, c.try_convert()?).into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self, a: Value, b: Value, c: Value) -> Value {
        let res =
            match std::panic::catch_unwind(AssertUnwindSafe(|| self.call_convert_value(a, b, c))) {
                Ok(v) => v,
                Err(e) => Err(Error::from_panic(e)),
            };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 4 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function4<Func, A, B, C, D, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, Res> Function4<Func, A, B, C, D, Res>
where
    Func: Fn(A, B, C, D) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(self, a: Value, b: Value, c: Value, d: Value) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 5 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function5<Func, A, B, C, D, E, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, Res> Function5<Func, A, B, C, D, E, Res>
where
    Func: Fn(A, B, C, D, E) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 6 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function6<Func, A, B, C, D, E, F, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, F, Res> Function6<Func, A, B, C, D, E, F, Res>
where
    Func: Fn(A, B, C, D, E, F) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e, f)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 7 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function7<Func, A, B, C, D, E, F, G, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, F, G, Res> Function7<Func, A, B, C, D, E, F, G, Res>
where
    Func: Fn(A, B, C, D, E, F, G) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e, f, g)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 8 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function8<Func, A, B, C, D, E, F, G, H, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, F, G, H, Res> Function8<Func, A, B, C, D, E, F, G, H, Res>
where
    Func: Fn(A, B, C, D, E, F, G, H) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e, f, g, h)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 9 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function9<Func, A, B, C, D, E, F, G, H, I, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, F, G, H, I, Res> Function9<Func, A, B, C, D, E, F, G, H, I, Res>
where
    Func: Fn(A, B, C, D, E, F, G, H, I) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e, f, g, h, i)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 10 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function10<Func, A, B, C, D, E, F, G, H, I, J, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, F, G, H, I, J, Res> Function10<Func, A, B, C, D, E, F, G, H, I, J, Res>
where
    Func: Fn(A, B, C, D, E, F, G, H, I, J) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e, f, g, h, i, j)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 11 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function11<Func, A, B, C, D, E, F, G, H, I, J, K, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, F, G, H, I, J, K, Res>
    Function11<Func, A, B, C, D, E, F, G, H, I, J, K, Res>
where
    Func: Fn(A, B, C, D, E, F, G, H, I, J, K) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e, f, g, h, i, j, k)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 12 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function12<Func, A, B, C, D, E, F, G, H, I, J, K, L, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    l: PhantomData<L>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, F, G, H, I, J, K, L, Res>
    Function12<Func, A, B, C, D, E, F, G, H, I, J, K, L, Res>
where
    Func: Fn(A, B, C, D, E, F, G, H, I, J, K, L) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    L: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            l: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
            l.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e, f, g, h, i, j, k, l)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 13 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function13<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    l: PhantomData<L>,
    m: PhantomData<M>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, Res>
    Function13<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, Res>
where
    Func: Fn(A, B, C, D, E, F, G, H, I, J, K, L, M) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    L: TryConvert,
    M: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            l: Default::default(),
            m: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
            l.try_convert()?,
            m.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e, f, g, h, i, j, k, l, m)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 14 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function14<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, N, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    l: PhantomData<L>,
    m: PhantomData<M>,
    n: PhantomData<N>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, N, Res>
    Function14<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, N, Res>
where
    Func: Fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    L: TryConvert,
    M: TryConvert,
    N: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            l: Default::default(),
            m: Default::default(),
            n: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
            l.try_convert()?,
            m.try_convert()?,
            n.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e, f, g, h, i, j, k, l, m, n)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 15 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function15<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    l: PhantomData<L>,
    m: PhantomData<M>,
    n: PhantomData<N>,
    o: PhantomData<O>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, Res>
    Function15<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, Res>
where
    Func: Fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    L: TryConvert,
    M: TryConvert,
    N: TryConvert,
    O: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            l: Default::default(),
            m: Default::default(),
            n: Default::default(),
            o: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
        o: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
            l.try_convert()?,
            m.try_convert()?,
            n.try_convert()?,
            o.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
        o: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Helper type for wrapping a function as a Ruby method ignoring self and
/// taking 16 arguments, with type conversions and error handling.
///
/// See the [`function`](crate::function!) macro.
#[doc(hidden)]
pub struct Function16<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Res> {
    func: Func,
    a: PhantomData<A>,
    b: PhantomData<B>,
    c: PhantomData<C>,
    d: PhantomData<D>,
    e: PhantomData<E>,
    f: PhantomData<F>,
    g: PhantomData<G>,
    h: PhantomData<H>,
    i: PhantomData<I>,
    j: PhantomData<J>,
    k: PhantomData<K>,
    l: PhantomData<L>,
    m: PhantomData<M>,
    n: PhantomData<N>,
    o: PhantomData<O>,
    p: PhantomData<P>,
    res: PhantomData<Res>,
}

#[allow(missing_docs)]
impl<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Res>
    Function16<Func, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Res>
where
    Func: Fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) -> Res,
    A: TryConvert,
    B: TryConvert,
    C: TryConvert,
    D: TryConvert,
    E: TryConvert,
    F: TryConvert,
    G: TryConvert,
    H: TryConvert,
    I: TryConvert,
    J: TryConvert,
    K: TryConvert,
    L: TryConvert,
    M: TryConvert,
    N: TryConvert,
    O: TryConvert,
    P: TryConvert,
    Res: ReturnValue,
{
    #[inline]
    pub fn new(func: Func) -> Self {
        Self {
            func,
            a: Default::default(),
            b: Default::default(),
            c: Default::default(),
            d: Default::default(),
            e: Default::default(),
            f: Default::default(),
            g: Default::default(),
            h: Default::default(),
            i: Default::default(),
            j: Default::default(),
            k: Default::default(),
            l: Default::default(),
            m: Default::default(),
            n: Default::default(),
            o: Default::default(),
            p: Default::default(),
            res: Default::default(),
        }
    }

    #[inline]
    unsafe fn call_convert_value(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
        o: Value,
        p: Value,
    ) -> Result<Value, Error> {
        (self.func)(
            a.try_convert()?,
            b.try_convert()?,
            c.try_convert()?,
            d.try_convert()?,
            e.try_convert()?,
            f.try_convert()?,
            g.try_convert()?,
            h.try_convert()?,
            i.try_convert()?,
            j.try_convert()?,
            k.try_convert()?,
            l.try_convert()?,
            m.try_convert()?,
            n.try_convert()?,
            o.try_convert()?,
            p.try_convert()?,
        )
        .into_return_value()
    }

    #[inline]
    pub unsafe fn call_handle_error(
        self,
        a: Value,
        b: Value,
        c: Value,
        d: Value,
        e: Value,
        f: Value,
        g: Value,
        h: Value,
        i: Value,
        j: Value,
        k: Value,
        l: Value,
        m: Value,
        n: Value,
        o: Value,
        p: Value,
    ) -> Value {
        let res = match std::panic::catch_unwind(AssertUnwindSafe(|| {
            self.call_convert_value(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p)
        })) {
            Ok(v) => v,
            Err(e) => Err(Error::from_panic(e)),
        };
        match res {
            Ok(v) => v,
            Err(e) => raise(e),
        }
    }
}

/// Wrap a Rust function item with Ruby type conversion and error handling,
/// ignoring Ruby's `self` argument.
///
/// This macro wraps the given function and returns a function pointer
/// implementing the [`Method`] trait, suitable for passing to functions that
/// define Ruby methods such as
/// [`define_method`](crate::module::Module::define_method).
///
/// Ruby code implicitly always has a `self` parameter available. In the
/// extention API this is passed explicitly. The wrapper this macro generates
/// ignores that argument, and does not pass it to the wrapped function.
///
/// The values `-2` and `-1` for `arity` have special meaning. Both indicate
/// functions with any number of arguments, with `-2` the arguments are passed
/// as a [`RArray`], with `-1` they are passed as a slice of [`Value`]s.
/// Arity of `-1` can be used with [`scan_args`](crate::scan_args::scan_args)
/// and [`get_kwargs`](crate::scan_args::get_kwargs) for more complex method
/// signatures.
///
/// | Arity | Signature                                     |
/// |-------|-----------------------------------------------|
/// |    -2 | `fn(arguments: RArray) -> Result<R, Error>`   |
/// |    -1 | `fn(arguments: &[Value]) -> Result<R, Error>` |
/// |     0 | `fn()-> Result<R, Error>`                     |
/// |     1 | `fn(arg1: T) -> Result<R, Error>`             |
/// |     2 | `fn(arg1: T, arg2: U) -> Result<R, Error>`    |
/// |   ... | ...                                           |
/// |    16 | ...                                           |
///
/// Where `T`, `U`, and so on are any types that implement `TryConvert`,
/// and `R` implements [`IntoValue`]. It is also possible to return just `R`
/// rather than a `Result` for functions that will never error, and omit the
/// return value (i.e. return `()`) for a function that returns `nil` to Ruby.
/// See [`ReturnValue`] for more details on what can be returned.
///
/// See the [`method`](crate::method!) macro for cases where the `self` argument is required.
///
/// # Examples
///
/// ```
/// fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
///     ((b.0 - a.0).powi(2) + (b.0 - a.0).powi(2)).sqrt()
/// }
///
/// #[magnus::init]
/// fn init() {
///     magnus::define_global_function("distance", magnus::function!(distance, 2));
/// }
/// ```
#[macro_export]
macro_rules! function {
    ($name:expr, -2) => {{
        unsafe extern "C" fn anon(rb_self: $crate::Value, args: $crate::RArray) -> $crate::Value {
            $crate::method::FunctionRbAry::new($name).call_handle_error(args)
        }
        anon as unsafe extern "C" fn($crate::Value, $crate::RArray) -> $crate::Value
    }};
    ($name:expr, -1) => {{
        unsafe extern "C" fn anon(
            argc: std::os::raw::c_int,
            argv: *const $crate::Value,
            rb_self: $crate::Value,
        ) -> $crate::Value {
            $crate::method::FunctionCAry::new($name).call_handle_error(argc, argv)
        }
        anon as unsafe extern "C" fn(
            std::os::raw::c_int,
            *const $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 0) => {{
        unsafe extern "C" fn anon(rb_self: $crate::Value) -> $crate::Value {
            $crate::method::Function0::new($name).call_handle_error()
        }
        anon as unsafe extern "C" fn($crate::Value) -> $crate::Value
    }};
    ($name:expr, 1) => {{
        unsafe extern "C" fn anon(rb_self: $crate::Value, a: $crate::Value) -> $crate::Value {
            $crate::method::Function1::new($name).call_handle_error(a)
        }
        anon as unsafe extern "C" fn($crate::Value, $crate::Value) -> $crate::Value
    }};
    ($name:expr, 2) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function2::new($name).call_handle_error(a, b)
        }
        anon as unsafe extern "C" fn($crate::Value, $crate::Value, $crate::Value) -> $crate::Value
    }};
    ($name:expr, 3) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function3::new($name).call_handle_error(a, b, c)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 4) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function4::new($name).call_handle_error(a, b, c, d)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 5) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function5::new($name).call_handle_error(a, b, c, d, e)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 6) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function6::new($name).call_handle_error(a, b, c, d, e, f)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 7) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function7::new($name).call_handle_error(a, b, c, d, e, f, g)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 8) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function8::new($name).call_handle_error(a, b, c, d, e, f, g, h)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 9) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function9::new($name).call_handle_error(a, b, c, d, e, f, g, h, i)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 10) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function10::new($name).call_handle_error(a, b, c, d, e, f, g, h, i, j)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 11) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function11::new($name)
                .call_handle_error(a, b, c, d, e, f, g, h, i, j, k)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 12) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
            l: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function12::new($name)
                .call_handle_error(a, b, c, d, e, f, g, h, i, j, k, l)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 13) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
            l: $crate::Value,
            m: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function13::new($name)
                .call_handle_error(a, b, c, d, e, f, g, h, i, j, k, l, m)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 14) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
            l: $crate::Value,
            m: $crate::Value,
            n: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function14::new($name)
                .call_handle_error(a, b, c, d, e, f, g, h, i, j, k, l, m, n)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 15) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
            l: $crate::Value,
            m: $crate::Value,
            n: $crate::Value,
            o: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function15::new($name)
                .call_handle_error(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, 16) => {{
        unsafe extern "C" fn anon(
            rb_self: $crate::Value,
            a: $crate::Value,
            b: $crate::Value,
            c: $crate::Value,
            d: $crate::Value,
            e: $crate::Value,
            f: $crate::Value,
            g: $crate::Value,
            h: $crate::Value,
            i: $crate::Value,
            j: $crate::Value,
            k: $crate::Value,
            l: $crate::Value,
            m: $crate::Value,
            n: $crate::Value,
            o: $crate::Value,
            p: $crate::Value,
        ) -> $crate::Value {
            $crate::method::Function16::new($name)
                .call_handle_error(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p)
        }
        anon as unsafe extern "C" fn(
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
            $crate::Value,
        ) -> $crate::Value
    }};
    ($name:expr, $arity:expr) => {
        compile_error!("arity must be an integer literal between -2..=16")
    };
}
