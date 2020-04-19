/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: Appache License 2.0
 **********************************************************************************************************************/
 #![doc(html_root_url = "https://docs.rs/ruspiro-error/0.1")]
 #![cfg_attr(not(any(test, doctest)), no_std)]
 
//! # Basic Error trait
//! This is more or less the same as found in Rust std library:
//! [Error](https://doc.rust-lang.org/std/error/trait.Error.html) but made available in ``no_std`` environment where an 
//! allocator is in place, which is the case for the RusPiRo family.

extern crate alloc;
use alloc::{boxed::Box, string::String};
use core::fmt::{Debug, Display};

/// The type that shall be used as ``Error`` type when returning a [``Result``]. This allows to use
/// one return type while the function might use different error types when using the ``?`` operator
/// and mapping all error types to a generic one would be quite cumbersome
pub type BoxError = Box<dyn Error + Send>; // + Send + Sync needed ?

/// The generic Error trait. All actual errors implementing this trait also need to implement ``Debug``
/// and ``Display`` to provide human readable text of the error.
pub trait Error: Debug + Display + Send {
    /// the underlaying source of this error, if any. This allows to "stack" errors while keeping
    /// track to it's root cause
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl<'a, E: Error + 'a + Send> From<E> for Box<dyn Error + 'a + Send> {
    /// Conviniently convert an [``Error``] into a boxed ``dyn Error``. This allows simple usage of
    /// ``.into()`` calls when returning an ``Error`` type.
    fn from(orig: E) -> Box<dyn Error + 'a + Send> {
        Box::new(orig)
    }
}

/// The most generic Error type. This should only be used as a fall back in case "you are to lazy" to implement your
/// specific error type in the crate that is returning the [Err].
pub struct GenericError {
    message: Option<String>,
}

impl GenericError {
    pub fn default() -> Self {
        GenericError { message: None }
    }

    pub fn with_message(message: &str) -> Self {
        GenericError {
            message: Some(message.into()),
        }
    }
}

impl Error for GenericError {}
impl Display for GenericError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.message {
            Some(msg) => write!(fmt, "Error: {}", msg),
            None => write!(fmt, "Generic Error"),
        }
    }
}

impl Debug for GenericError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // re-use the display implementation in case of debug formatting
        <GenericError as Display>::fmt(self, fmt)
    }
}