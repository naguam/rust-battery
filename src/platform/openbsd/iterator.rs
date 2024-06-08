use crate::platform::traits::BatteryIterator;
use std::sync::Arc;

use crate::Result;

use super::{SysCtlDevice, SysCtlManager};

#[derive(Debug)]
pub struct SysCtlIterator {
    #[allow(dead_code)]
    manager: Arc<SysCtlManager>,
}

impl Iterator for SysCtlIterator {
    type Item = Result<SysCtlDevice>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl BatteryIterator for SysCtlIterator {
    type Manager = SysCtlManager;
    type Device = SysCtlDevice;

    fn new(manager: Arc<Self::Manager>) -> Result<Self> {
        Ok(Self { manager })
    }
}
