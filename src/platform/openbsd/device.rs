use crate::platform::traits::BatteryDevice;
use crate::units::{ElectricPotential, Energy, Power, ThermodynamicTemperature};
//use crate::{Error, Result};
use crate::{State, Technology};

#[derive(Debug, Default)]
pub struct SysCtlDevice;

//impl SysMonDevice {
//}

// hw.sensors.acpibat0.volt0=11.10 VDC (voltage)
// hw.sensors.acpibat0.volt1=12.52 VDC (current voltage)
// hw.sensors.acpibat0.current0=1.42 A (rate)
// hw.sensors.acpibat0.amphour0=3.69 Ah (last full capacity)
// hw.sensors.acpibat0.amphour1=0.44 Ah (warning capacity)
// hw.sensors.acpibat0.amphour2=0.13 Ah (low capacity)
// hw.sensors.acpibat0.amphour3=1.79 Ah (remaining capacity), OK
// hw.sensors.acpibat0.amphour4=4.40 Ah (design capacity)
// hw.sensors.acpibat0.raw0=6 (battery charging), OK

// Need to understand what happens with watts

// energy                   amphour3
// energy_full              amphour0
// energy_full_design       amphour4
// energy_rate              current0
// voltage                  volt1
// state                    raw0
// technology               N/A
// temperature              N/A
// cycle_count              N/A
// vendor                   N/A
// model                    N/A
// serial_number            N/A
// time_to_full             Calculated
// time_to_empty            Calculated
// state_of_health          Calculated
// state_of_charge          Calculated

impl BatteryDevice for SysCtlDevice {
    fn energy(&self) -> Energy {
        let mut compile: i32 = 0;
        let mib: [i32; 2] = [1, 1];
        _ = super::sysctl::sysctl(&mib, 2, None, None).unwrap();
        _ = super::sysctl::as_bytes_ref(&mut compile, std::mem::size_of::<i32>());
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
