//! Set the panicking behavior to abort
//!
//! This crate contains an implementation of `panic_fmt` that simply calls [`intrinsics::abort`].
//!
//! [`intrinsics::abort`]: https://doc.rust-lang.org/core/intrinsics/fn.abort.html
//!
//! # Usage
//!
//! ``` ignore
//! #![no_std]
//!
//! extern crate panic_abort;
//!
//! fn main() {
//!     panic!("argument is ignored");
//! }
//! ```

#![allow(stable_features)]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(core_intrinsics)]
#![feature(panic_handler)]
#![no_std]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}
