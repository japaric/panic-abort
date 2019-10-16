//! Set the panicking behavior to abort
//!
//! This crate contains an implementation of `panic_fmt` that simply calls [`intrinsics::abort`].
//!
//! [`intrinsics::abort`]: https://doc.rust-lang.org/core/intrinsics/fn.abort.html
//!
//! # Behavior
//!
//! As of Rust 1.38.0, `intrinsics::abort` lowers to a trap instruction on *most* architectures; on
//! some architectures it simply lowers to call to the `abort` function (unmangled name). The exact
//! behavior of `intrinsics::abort` is architecture and system dependent.
//!
//! On bare-metal (no OS) systems the trap instruction usually causes a *hardware* exception to be
//! raised in a *synchronous* fashion -- hardware exceptions have nothing to do with C++ exceptions
//! and are closer in semantics to POSIX signals (see `man 7 signals` on UNIX-y systems).
//!
//! On hosted applications (applications running under an OS), the trap instruction *usually*
//! terminates the whole process with an exit code that corresponds to SIGILL *unless* a signal
//! handler that handles this particular signal was registered (again, see `man 7 signals` on UNIX-y
//! systems).
//!
//! *HEADS UP* Because `intrinsics::abort` is an unstable API its semantics could change in any new
//! Rust release (minor or patch release).
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
