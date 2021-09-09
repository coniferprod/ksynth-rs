//! Data models for controllers and macros.
//!

use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;
use bit::BitIndex;

use crate::SystemExclusiveData;
use crate::k5000::{MacroParameterDepth, SignedLevel};

type Pan = SignedLevel;

/// Velocity switch settings.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum VelocitySwitch {
    Off,
    Loud,
    Soft,
    Unknown,
}

impl Default for VelocitySwitch {
    fn default() -> Self { VelocitySwitch::Off }
}

/// Velocity switch settings.
pub struct VelocitySwitchSettings {
    pub switch_type: VelocitySwitch,
    pub threshold: u8,
}

impl Default for VelocitySwitchSettings {
    fn default() -> Self {
        VelocitySwitchSettings {
            switch_type: Default::default(),
            threshold: 0,
        }
    }
}

impl VelocitySwitchSettings {
    fn threshold_from(value: usize) -> u8 {
        let table: [u8; 32] = [
            4, 8, 12, 16, 20, 24, 28, 32,
            36, 40, 44, 48, 52, 56, 60, 64,
            68, 72, 76, 80, 84, 88, 92, 96,
            100, 104, 108, 112, 116, 120, 124, 127
        ];
        table[value]
    }

    fn from_threshold(threshold: u8) -> usize {
        let table: [u8; 32] = [
            4, 8, 12, 16, 20, 24, 28, 32,
            36, 40, 44, 48, 52, 56, 60, 64,
            68, 72, 76, 80, 84, 88, 92, 96,
            100, 104, 108, 112, 116, 120, 124, 127
        ];

        table.to_vec().iter().position(|x| *x == threshold).unwrap_or_default()
    }
}

impl fmt::Display for VelocitySwitchSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vel. sw type: {:?}, threshold: {}",
            self.switch_type, self.threshold
        )
    }
}

impl SystemExclusiveData for VelocitySwitchSettings {
    fn from_bytes(data: Vec<u8>) -> Self {
        let vs = data[0].bit_range(5..7) & 0b11;  // bits 5-6
        let t = data[0].bit_range(0..5); // bits 0-4
        eprintln!("VelocitySwitchSettings: vs = 0b{:b} ({}), t = 0b{:b} ({})", vs, vs, t, t);
        VelocitySwitchSettings {
            switch_type: VelocitySwitch::try_from(vs).unwrap(),
            threshold: VelocitySwitchSettings::threshold_from(t as usize),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let t = VelocitySwitchSettings::from_threshold(self.threshold) as u8;
        let value = t | ((self.switch_type as u8) << 5);
        vec![value]
    }
}

/// Control source.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum ControlSource {
    Bender,
    ChannelPressure,
    Wheel,
    Expression,
    MidiVolume,
    PanPot,
    GeneralController1,
    GeneralController2,
    GeneralController3,
    GeneralController4,
    GeneralController5,
    GeneralController6,
    GeneralController7,
    GeneralController8,
}

impl Default for ControlSource {
    fn default() -> Self { ControlSource::Bender }
}

/// Control destination.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum ControlDestination {
    PitchOffset,
    CutoffOffset,
    Level,
    VibratoDepthOffset,
    GrowlDepthOffset,
    TremoloDepthOffset,
    LfoSpeedOffset,
    AttackTimeOffset,
    Decay1TimeOffset,
    ReleaseTimeOffset,
    VelocityOffset,
    ResonanceOffset,
    PanPotOffset,
    FormantFilterBiasOffset,
    FormantFilterEnvelopeLfoDepthOffset,
    FormantFilterEnvelopeLfoSpeedOffset,
    HarmonicLowOffset,
    HarmonicHighOffset,
    HarmonicEvenOffset,
    HarmonicOddOffset,
}

impl Default for ControlDestination {
    fn default() -> Self { ControlDestination::PitchOffset }
}

/// Macro controller.
pub struct MacroController {
    pub destination1: ControlDestination,
    pub depth1: MacroParameterDepth,
    pub destination2: ControlDestination,
    pub depth2: MacroParameterDepth,
}

impl Default for MacroController {
    fn default() -> Self {
        MacroController {
            destination1: Default::default(),
            depth1: MacroParameterDepth::from(0),
            destination2: Default::default(),
            depth2: MacroParameterDepth::from(0),
        }
    }
}

impl fmt::Display for MacroController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dest1 = {:?}, Depth = {}\nDest2 = {:?}, Depth = {}",
            self.destination1, self.depth1, self.destination2, self.depth2
        )
    }
}

impl SystemExclusiveData for MacroController {
    fn from_bytes(data: Vec<u8>) -> Self {
        eprintln!("MacroController from bytes {:?}", data);

        MacroController {
            destination1: ControlDestination::try_from(data[0]).unwrap(),
            depth1: MacroParameterDepth::from(data[1]),
            destination2: ControlDestination::try_from(data[2]).unwrap(),
            depth2: MacroParameterDepth::from(data[3]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.destination1 as u8,
            self.depth1.as_byte(),
            self.destination2 as u8,
            self.depth2.as_byte()
        ]
    }
}

/// Assignable controller.
pub struct AssignableController {
    pub source: ControlSource,
    pub destination: ControlDestination,
    pub depth: u8,
}

impl Default for AssignableController {
    fn default() -> Self {
        AssignableController {
            source: Default::default(),
            destination: Default::default(),
            depth: 0,
        }
    }
}

impl SystemExclusiveData for AssignableController {
    fn from_bytes(data: Vec<u8>) -> Self {
        AssignableController {
            source: ControlSource::try_from(data[0]).unwrap(),
            destination: ControlDestination::try_from(data[1]).unwrap(),
            depth: data[2],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.source as u8, self.destination as u8, self.depth]
    }
}

/// Modulation settings.
pub struct ModulationSettings {
    pub pressure: MacroController,
    pub wheel: MacroController,
    pub expression: MacroController,
    pub assignable1: AssignableController,
    pub assignable2: AssignableController,
}

impl Default for ModulationSettings {
    fn default() -> Self {
        ModulationSettings {
            pressure: Default::default(),
            wheel: Default::default(),
            expression: Default::default(),
            assignable1: Default::default(),
            assignable2: Default::default(),
        }
    }
}

impl SystemExclusiveData for ModulationSettings {
    fn from_bytes(data: Vec<u8>) -> Self {
        ModulationSettings {
            pressure: MacroController::from_bytes(data[..4].to_vec()),
            wheel: MacroController::from_bytes(data[4..8].to_vec()),
            expression: MacroController::from_bytes(data[8..12].to_vec()),
            assignable1: AssignableController::from_bytes(data[12..15].to_vec()),  // NOTE: only three bytes
            assignable2: AssignableController::from_bytes(data[15..18].to_vec()),  // not four like macros
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        result.extend(self.pressure.to_bytes());
        result.extend(self.wheel.to_bytes());
        result.extend(self.expression.to_bytes());
        result.extend(self.assignable1.to_bytes());
        result.extend(self.assignable2.to_bytes());

        result
    }
}

/// Pan type.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum PanKind {
    Normal,
    Random,
    KeyScale,
    NegativeKeyScale,
}

impl Default for PanKind {
    fn default() -> Self { PanKind::Normal }
}

/// Pan settings.
pub struct PanSettings {
    pub pan_type: PanKind,
    pub pan_value: Pan,
}

impl Default for PanSettings {
    fn default() -> Self {
        PanSettings {
            pan_type: Default::default(),
            pan_value: Pan::from(0),
        }
    }
}

impl SystemExclusiveData for PanSettings {
    fn from_bytes(data: Vec<u8>) -> Self {
        PanSettings {
            pan_type: PanKind::try_from(data[0]).unwrap(),
            pan_value: Pan::from(data[1]),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.pan_type as u8, self.pan_value.as_byte()]
    }
}

/// Switch kind.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum Switch {
    Off,
    HarmMax,
    HarmBright,
    HarmDark,
    HarmSaw,
    SelectLoud,
    AddLoud,
    AddFifth,
    AddOdd,
    AddEven,
    He1,
    He2,
    HarmonicEnvelopeLoop,
    FfMax,
    FfComb,
    FfHiCut,
    FfComb2,
}

impl Default for Switch {
    fn default() -> Self { Switch::Off }
}

/// Switch control settings.
#[derive(Debug)]
pub struct SwitchControl {
    pub switch1: Switch,
    pub switch2: Switch,
    pub footswitch1: Switch,
    pub footswitch2: Switch,
}

impl Default for SwitchControl {
    fn default() -> Self {
        SwitchControl {
            switch1: Default::default(),
            switch2: Default::default(),
            footswitch1: Default::default(),
            footswitch2: Default::default(),
        }
    }
}

impl SystemExclusiveData for SwitchControl {
    fn from_bytes(data: Vec<u8>) -> Self {
        SwitchControl {
            switch1: Switch::try_from(data[0]).unwrap(),
            switch2: Switch::try_from(data[1]).unwrap(),
            footswitch1: Switch::try_from(data[2]).unwrap(),
            footswitch2: Switch::try_from(data[3]).unwrap(),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.switch1 as u8, self.switch2 as u8, self.footswitch1 as u8, self.footswitch2 as u8]
    }
}

/// Polyphony type.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum Polyphony {
    Poly,
    Solo1,
    Solo2,
}

impl fmt::Display for Polyphony {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Polyphony::Poly => "POLY",
            Polyphony::Solo1 => "SOLO1",
            Polyphony::Solo2 => "SOLO2",
        })
    }
}

/// Amplitude modulation kind.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum AmplitudeModulation {
    Off,
    Source2,
    Source3,
    Source4,
    Source5,
    Source6,
}

impl Default for AmplitudeModulation {
    fn default() -> Self { AmplitudeModulation::Off }
}

impl fmt::Display for AmplitudeModulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            AmplitudeModulation::Off => "OFF",
            AmplitudeModulation::Source2 => "1->2",
            AmplitudeModulation::Source3 => "2->3",
            AmplitudeModulation::Source4 => "3->4",
            AmplitudeModulation::Source5 => "4->5",
            AmplitudeModulation::Source6 => "5->6",
        })
    }
}

/// Velocity curve.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum VelocityCurve {
    Curve1,
    Curve2,
    Curve3,
    Curve4,
    Curve5,
    Curve6,
    Curve7,
    Curve8,
    Curve9,
    Curve10,
    Curve11,
    Curve12,
}

impl fmt::Display for VelocityCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            VelocityCurve::Curve1 => String::from("Curve 1"),
            VelocityCurve::Curve2 => String::from("Curve 2"),
            VelocityCurve::Curve3 => String::from("Curve 3"),
            VelocityCurve::Curve4 => String::from("Curve 4"),
            VelocityCurve::Curve5 => String::from("Curve 5"),
            VelocityCurve::Curve6 => String::from("Curve 6"),
            VelocityCurve::Curve7 => String::from("Curve 7"),
            VelocityCurve::Curve8 => String::from("Curve 8"),
            VelocityCurve::Curve9 => String::from("Curve 9"),
            VelocityCurve::Curve10 => String::from("Curve 10"),
            VelocityCurve::Curve11 => String::from("Curve 11"),
            VelocityCurve::Curve12 => String::from("Curve 12"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn test_macro_controller_from_bytes() {
        let data = vec![0x01, 0x4f, 0x03, 0x40];
        let mac = MacroController::from_bytes(data);
        assert_eq!(mac.destination1, ControlDestination::CutoffOffset);
    }

    /*
    #[test]
    fn test_modulation_settings_from_bytes() {
        let data = vec![
            0x01, 0x4f, 0x03, 0x40,  // press: destination 1, depth, destination 2, depth
            0x03, 0x59, 0x01, 0x40,  // wheel: destination 1, depth, destination 2, depth
            0x02, 0x5f, 0x00, 0x40,  // express: destination 1, depth, destination 2, depth
            0x02, 0x0d, 0x40,  // assignable: control source 1, destination, depth
            0x02, 0x09, 0x40,  // assignable: control source 2, destination, depth
        ];
        let modulation_settings = ModulationSettings::from_bytes(data);
        assert_eq!(modulation_settings.pressure,
            MacroController {
                destination1: ControlDestination::CutoffOffset,
                depth1: MacroParameterDepth::from(0x4f),
                destination2: ControlDestination::VibratoDepthOffset,
                depth2: MacroParameterDepth::from(0x40),
             }
        );
    }
    */
}
