use crate::platform::traits::BatteryDevice;
use crate::units::{ElectricPotential, Energy, Power, ThermodynamicTemperature};
//use crate::{Error, Result};
use crate::{State, Technology};

#[derive(Debug, Default)]
pub struct SysCtlDevice;

//impl SysMonDevice {
//}

// energy
// energy_full
// energy_full_design
// energy_rate
// voltage
// state
// technology
// temperature
// cycle_count
// vendor
// model
// serial_number
// time_to_full
// time_to_empty
// state_of_health
// state_of_charge

impl BatteryDevice for SysCtlDevice {
    fn energy(&self) -> Energy {
        microwatt_hour!(0)
    }

    fn energy_full(&self) -> Energy {
        microwatt_hour!(0)
    }

    fn energy_full_design(&self) -> Energy {
        microwatt_hour!(0)
    }

    fn energy_rate(&self) -> Power {
        microwatt!(0)
    }

    fn state(&self) -> State {
        State::Unknown
    }

    fn voltage(&self) -> ElectricPotential {
        microvolt!(0)
    }

    fn temperature(&self) -> Option<ThermodynamicTemperature> {
        None
    }

    fn vendor(&self) -> Option<&str> {
        None
    }

    fn model(&self) -> Option<&str> {
        None
    }

    fn serial_number(&self) -> Option<&str> {
        None
    }

    fn technology(&self) -> Technology {
        Technology::Unknown
    }

    fn cycle_count(&self) -> Option<u32> {
        None
    }
}
