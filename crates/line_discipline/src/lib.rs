//! Init:
//! Firstly, driver registers itself, and gets its driver index.
//! Secondly, driver registers all devices it found, and gets their device indices.
//! Now, tty layer has all needed infomation.
//!
//! Read:
//! Driver sends data from a device to tty layer with driver index and device index.
//! Kernel gets data from a device by passing its name to tty layer.  
//!
//! Write:
//! Kernel writes data to a device by passing its name to tty layer.

#![no_std]

extern crate alloc;

mod buffer;
mod driver;
mod ldisc;
mod tty;

use alloc::vec;
use driver::get_driver_by_index;

pub use driver::{register_device, register_driver, TtyDriverOps};
use spinlock::SpinNoIrq;
pub use tty::{get_all_device_names, get_device_by_name};

/// called by driver when irq.
/// send data from hardware.
pub fn tty_receive_buf(driver_index: usize, device_index: usize, buf: &[u8]) {
    if let Some(driver) = get_driver_by_index(driver_index) {
        if let Some(tty) = driver.get_device_by_index(device_index) {
            let ldisc = tty.ldisc().clone();
            ldisc.receive_buf(tty.clone(), buf);
        }
    }
}

/// kernel reads tty.
pub fn tty_read(buf: &mut [u8], dev_name: &str) -> usize {
    if let Some(tty) = get_device_by_name(dev_name) {
        return tty.ldisc().read(buf);
    }
    0
}

/// kernel writes tty.
pub fn tty_write(buf: &[u8], dev_name: &str) -> usize {
    if let Some(tty) = get_device_by_name(dev_name) {
        return tty.ldisc().write(tty.clone(), buf);
    }
    0
}

/// init
pub fn init() {
    driver::ALL_DRIVERS.init_by(SpinNoIrq::new(vec![]));
    tty::ALL_DEVICES.init_by(SpinNoIrq::new(vec![]));
    tty::ALL_DEV_NAMES.init_by(SpinNoIrq::new(vec![]));
}
