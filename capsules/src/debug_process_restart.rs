//! Debug capsule to cause a button press to restart all apps.
//!
//! This is useful for debugging that capsules and apps work when they are
//! restarted by the kernel.
//!
//! Usage
//! -----
//!
//! ```rust
//! let debug_process_restart = static_init!(
//!     capsules::debug_process_restart::DebugProcessRestart<'static, sam4l::gpio::GPIOPin>,
//!     capsules::debug_process_restart::DebugProcessRestart::new(&sam4l::gpio::PA[16])
//! );
//! sam4l::gpio::PA[16].set_client(debug_process_restart);
//! }
//! ```

use kernel::debug;
use kernel::hil;
use kernel::hil::gpio::{Client, InterruptMode};

pub struct DebugProcessRestart<'a, G: hil::gpio::Pin + 'a> {
    _pin: &'a G,
}

impl<'a, G: hil::gpio::Pin + hil::gpio::PinCtl> DebugProcessRestart<'a, G> {
    pub fn new(pin: &'a G) -> DebugProcessRestart<'a, G> {
        pin.make_input();
        pin.enable_interrupt(0, InterruptMode::RisingEdge);

        DebugProcessRestart { _pin: pin }
    }
}

impl<'a, G: hil::gpio::Pin + hil::gpio::PinCtl> Client for DebugProcessRestart<'a, G> {
    fn fired(&self, _pin_num: usize) {
        debug::hardfault_all_apps();
    }
}
