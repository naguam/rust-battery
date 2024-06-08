use crate::platform::traits::BatteryManager;
use crate::Result;

use super::{SysCtlDevice, SysCtlIterator};

#[derive(Debug)]
pub struct SysCtlManager;

impl BatteryManager for SysCtlManager {
    type Iterator = SysCtlIterator;

    fn new() -> Result<Self> {
        Ok(Self {})
    }

    fn refresh(&self, _device: &mut SysCtlDevice) -> Result<()> {
        Ok(())
    }
}
