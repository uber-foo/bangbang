#[cfg(feature = "on-off")]
use bangbang::prelude::*;

#[cfg(feature = "on-off")]
#[test]
fn can_start_on() {
    let mut on_off = OnOff::new(true, None, None);
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);
}

#[cfg(feature = "on-off")]
#[test]
fn can_start_off() {
    let mut on_off = OnOff::new(false, None, None);
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);
}

#[cfg(feature = "on-off")]
#[test]
fn calls_handlers() {
    use std::sync::Arc;
    use std::sync::Mutex;

    let called_on_handler = Arc::new(Mutex::new(false));
    let called_on_inner_handler = Arc::clone(&called_on_handler);
    let mut handle_on = move || {
        *called_on_inner_handler.lock().unwrap() = true;
        Ok(())
    };

    let called_off_handler = Arc::new(Mutex::new(false));
    let called_off_inner_handler = Arc::clone(&called_off_handler);
    let mut handle_off = move || {
        *called_off_inner_handler.lock().unwrap() = true;
        Ok(())
    };

    {
        let _on_off = OnOff::new(false, Some(&mut handle_on), Some(&mut handle_off));
        let called_on_handler = called_on_handler.lock().unwrap();
        let called_off_handler = called_off_handler.lock().unwrap();
        assert_eq!(*called_on_handler, false);
        assert_eq!(*called_off_handler, false);
    }

    {
        let mut on_off = OnOff::new(true, Some(&mut handle_on), Some(&mut handle_off));
        assert!(on_off.bang().is_ok());
        let called_on_handler = called_on_handler.lock().unwrap();
        let mut called_off_handler = called_off_handler.lock().unwrap();
        assert_eq!(*called_on_handler, false);
        assert_eq!(*called_off_handler, true);
        *called_off_handler = false;
    }

    {
        let mut on_off = OnOff::new(false, Some(&mut handle_on), Some(&mut handle_off));
        assert!(on_off.bang().is_ok());
        assert!(on_off.bang().is_ok());
        let called_on_handler = called_on_handler.lock().unwrap();
        let called_off_handler = called_off_handler.lock().unwrap();
        assert_eq!(*called_on_handler, true);
        assert_eq!(*called_off_handler, true);
    }
}
