use super::prelude::*;
use core::fmt;

/// handler method to be called on a state change
type StateChangeHander = dyn FnMut() -> Result<(), BangBangError> + Sync + Send;

/// simple on/off bang-bang controller
///
/// # Simple Example
/// ```
/// use bangbang::prelude::*;
///
/// fn example() -> Result<(), BangBangError> {
///     // we can create this controller to start in the "on" state
///     let mut controller = OnOff::new(true, None, None);
///     assert_eq!(controller.is_on(), true);
///     assert_eq!(controller.is_off(), false);
///
///     // or, it can start in the "off" state
///     let mut controller = OnOff::new(false, None, None);
///     assert_eq!(controller.is_on(), false);
///     assert_eq!(controller.is_off(), true);
///
///     // transition state (in this case, from "off" to "on")
///     controller.bang()?;
///     assert_eq!(controller.is_on(), true);
///     assert_eq!(controller.is_off(), false);
///
///     Ok(())
/// }
/// ```
///
/// # Example with Custom State Change Handlers
/// ```
/// use bangbang::prelude::*;
///
/// fn example() -> Result<(), BangBangError> {
///     // handler that always fails, `code` is a failure code that we can choose arbitrarily
///     let mut handle_on = || Err(BangBangError::StateChangeFailedUnexpectedly {
///         from: BangBangState::A,
///         to: BangBangState::B,
///         code: 1,
///     });
///
///     // handler that always succeeds   
///     let mut handle_off = || Ok(());
///
///     // this controller defaults to the on state
///     let mut controller = OnOff::new(true, Some(&mut handle_on), Some(&mut handle_off));
///     assert_eq!(controller.is_on(), true);
///     assert_eq!(controller.is_off(), false);
///
///     // transition to off state, will succeed given our handler
///     assert!(controller.bang().is_ok());
///
///     // because transition was successful, state has been updated
///     assert_eq!(controller.is_on(), true);
///     assert_eq!(controller.is_off(), false);
///
///     // transition to the on state, will fail given our handler
///     assert!(controller.bang().is_err());
///
///     // because transition failed, state remains the same
///     assert_eq!(controller.is_on(), true);
///     assert_eq!(controller.is_off(), false);
///
///     Ok(())
/// }
/// ```
#[derive(Default)]
pub struct OnOff<'a> {
    on: bool,
    handle_on: Option<&'a mut StateChangeHander>,
    handle_off: Option<&'a mut StateChangeHander>,
}

impl fmt::Debug for OnOff<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OnOff {{ on: {} }}", self.on)
    }
}

impl BangBang for OnOff<'_> {
    fn state(&self) -> BangBangState {
        self.on.into()
    }

    fn set(&mut self, new_state: BangBangState) -> Result<(), BangBangError> {
        let result = match new_state {
            BangBangState::A => {
                if let Some(handler) = &mut self.handle_off {
                    handler()
                } else {
                    Ok(())
                }
            }
            BangBangState::B => {
                if let Some(handler) = &mut self.handle_on {
                    handler()
                } else {
                    Ok(())
                }
            }
        };
        if result.is_ok() {
            self.on = new_state != BangBangState::A;
        }
        result
    }
}

impl<'a> OnOff<'a> {
    /// creates a new on/off controller with optional notification handlers for each state transition
    /// ```
    /// use bangbang::OnOff;
    ///
    /// // state transition handlers that never block state transition
    /// let mut handle_on = || Ok(());
    /// let mut handle_off = || Ok(());
    ///
    /// // create a controller that starts in the off state
    /// let on_off = OnOff::new(false, Some(&mut handle_on), Some(&mut handle_off));
    /// assert!(on_off.is_off());
    ///
    /// // create a controller that starts in the on state
    /// let on_off = OnOff::new(true, Some(&mut handle_on), Some(&mut handle_off));
    /// assert!(on_off.is_on());
    /// ```
    pub fn new(
        on: bool,
        handle_on: Option<&'a mut StateChangeHander>,
        handle_off: Option<&'a mut StateChangeHander>,
    ) -> Self {
        Self {
            on,
            handle_on,
            handle_off,
        }
    }

    /// convienence method for checking if the controller is in the `on` state
    /// ```
    /// use bangbang::prelude::*;
    ///
    /// let on_off = OnOff::new(true, None, None);
    ///
    /// // these two calls are equavalent
    /// assert!(on_off.is_on());
    /// assert_eq!(on_off.state(), BangBangState::B);
    /// ```
    pub fn is_on(&self) -> bool {
        self.on
    }

    /// convienence method for checking if the controller is in the `off` state
    /// ```
    /// use bangbang::prelude::*;
    ///
    /// let on_off = OnOff::new(false, None, None);
    ///
    /// // these two calls are equavalent
    /// assert!(on_off.is_off());
    /// assert_eq!(on_off.state(), BangBangState::A);
    /// ```
    pub fn is_off(&self) -> bool {
        !self.on
    }
}
