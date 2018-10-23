//! This crate provides an abstraction of a [bang-bang controller](https://en.wikipedia.org/wiki/Bang%E2%80%93bang_control)
//! along with a simple reference implementation. Suitable for regular applications using the standard
//! library and embedded applications using [`#![no_std]`](https://doc.rust-lang.org/reference/attributes.html?highlight=no_std#crate-only-attributes).
//!
//! Bang-bang controllers are relatively simple machines, able to flip-flop solely between two mutually
//! exclusive states. A common example of a bang-bang controller is a thermostat—the furnace can be turned
//! either on or off with no other states of operation between those two extremes.
//!
//! # Example
//! ```
//! use bangbang::prelude::*;
//!
//! // Simple struct to hold our current state
//! struct FlipFlop {
//!     current_state: BangBangState,
//! }
//!
//! // Simplest implementation of a bang-bang controller
//! impl BangBang for FlipFlop {
//!     // Return the current active state
//!     fn state(&self) -> BangBangState {
//!         self.current_state
//!     }
//!     
//!     // Change the current active state
//!     fn set(&mut self, new_state: BangBangState) -> Result<(), BangBangError> {
//!         // Normally there would be logic here to ensure a state transition is possible
//!
//!         // For the example, we'll just simply change the state as requested
//!         self.current_state = new_state;
//!
//!         // No failure cases in this example
//!         Ok(())
//!     }
//! }
//!
//! fn run_example() -> Result<(), BangBangError> {
//!     // Create a new bang-bang controller with initial state set to `A`
//!     let mut flip_flop = FlipFlop { current_state: BangBangState::A };
//!     assert_eq!(flip_flop.state(), BangBangState::A);
//!
//!     // Trigger the bang-bang controller, flipping the state to `B`
//!     flip_flop.bang()?;
//!     assert_eq!(flip_flop.state(), BangBangState::B);
//!
//!     // Trigger the bang-bang controller, flipping the state back to `A`
//!     flip_flop.bang()?;
//!     assert_eq!(flip_flop.state(), BangBangState::A);
//!
//!     Ok(())
//! }
//!
//! run_example();
//! ```
//!
//! # Features
//!
//! | Feature | Default | Description |
//! | --- | --- | --- |
//! | log | enabled | enables the [`log`] crate dependency and logging calls |
//! | on-off | enabled | enables the [`OnOff`] reference implementation |
#![no_std]
#![deny(warnings)]
#![deny(bad_style)]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(unused)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(box_pointers)]
#![deny(macro_use_extern_crate)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_lifetimes)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(variant_size_differences)]
#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]

#[cfg(feature = "log")]
use log::{debug, trace};

#[cfg(feature = "on-off")]
mod on_off;
#[cfg(feature = "on-off")]
pub use self::on_off::OnOff;

/// A convenience module appropriate for glob imports (`use bangbang::prelude::*;`)
pub mod prelude {
    #[cfg(feature = "on-off")]
    #[doc(no_inline)]
    pub use super::on_off::OnOff;
    #[doc(no_inline)]
    pub use super::{BangBang, BangBangError, BangBangState};
}

/// bang-bang controller errors
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BangBangError {
    /// an unexpected error occured during state change
    StateChangeFailedUnexpectedly {
        /// failed to change from state
        from: BangBangState,
        /// failed to change to state
        to: BangBangState,
        /// error code provided by the underlying implementation
        code: u8,
    },
    /// state change has been temporarily constrained by the implementation
    StateChangeTemporarilyConstrained {
        /// failed to change from state
        from: BangBangState,
        /// failed to change to state
        to: BangBangState,
        /// error code provided by the underlying implementation
        code: u8,
    },
    /// custom state change handler failed
    StateChangeHandlerFailed {
        /// failed to change from state
        from: BangBangState,
        /// failed to change to state
        to: BangBangState,
        /// error code provided by the underlying implementation
        code: u8,
    },
    /// implementation handler unexpected error
    ImplementationHandlerUnexpectedError {},
}

/// bang-bang controller states
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BangBangState {
    /// one of the two states the controller can be in
    A,
    /// one of the two states the controller can be in
    B,
}

impl From<bool> for BangBangState {
    fn from(value: bool) -> Self {
        if value {
            BangBangState::B
        } else {
            BangBangState::A
        }
    }
}

/// abstraction of a bang-bang controller
pub trait BangBang {
    /// returns the current state of the controller
    fn state(&self) -> BangBangState;

    /// updates the current state of the controller
    fn set(&mut self, new_state: BangBangState) -> Result<(), BangBangError>;

    /// toggles the controller between states
    fn bang(&mut self) -> Result<(), BangBangError> {
        let current_state = self.state();

        let new_state = match current_state {
            BangBangState::A => BangBangState::B,
            BangBangState::B => BangBangState::A,
        };

        #[cfg(feature = "log")]
        trace!(
            "attempting state change from {:?} to {:?}",
            current_state,
            new_state,
        );

        self.set(new_state)?;

        #[cfg(feature = "log")]
        debug!(
            "state changed from {:?} to {:?}",
            current_state,
            self.state()
        );

        Ok(())
    }
}
