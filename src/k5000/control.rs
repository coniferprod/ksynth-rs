//! Data models for controllers and macros.
//!

use std::convert::TryFrom;
use std::fmt;

use num_enum::TryFromPrimitive;
use bit::BitIndex;
use strum_macros;

use crate::{
    SystemExclusiveData,
    ParseError
};
use crate::k5000::{
    MacroParameterDepth,
    Pan,
    ControlDepth
};

/// Velocity switch settings.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Default, strum_macros::Display)]
#[repr(u8)]
pub enum VelocitySwitch {
    #[default]
    Off,

    Loud,
    Soft,
    Unknown,
}

/// Velocity switch settings.
#[derive(Default, Debug)]
pub struct VelocitySwitchSettings {
    pub switch_type: VelocitySwitch,
    pub threshold: u8,
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
        write!(f, "Type={} Threshold={}",
            self.switch_type, self.threshold
        )
    }
}

impl SystemExclusiveData for VelocitySwitchSettings {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        let vs = data[0].bit_range(5..7) & 0b11;  // bits 5-6
        let t = data[0].bit_range(0..5); // bits 0-4
        eprintln!("VelocitySwitchSettings: vs = 0b{:b} ({}), t = 0b{:b} ({})", vs, vs, t, t);
        Ok(VelocitySwitchSettings {
            switch_type: VelocitySwitch::try_from(vs).unwrap(),
            threshold: VelocitySwitchSettings::threshold_from(t as usize),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let t = VelocitySwitchSettings::from_threshold(self.threshold) as u8;
        let value = t | ((self.switch_type as u8) << 5);
        vec![value]
    }

    fn data_size() -> usize { 1 }
}

/// Control source.
#[derive(
    Debug,
    Eq, PartialEq,
    Copy, Clone,
    TryFromPrimitive,
    Default,
    strum_macros::Display
)]
#[repr(u8)]
pub enum ControlSource {
    #[default]
    #[strum(to_string = "Bender")]
    Bender,

    #[strum(to_string = "Channel pressure")]
    ChannelPressure,

    #[strum(to_string = "Wheel")]
    Wheel,

    #[strum(to_string = "Expression")]
    Expression,

    #[strum(to_string = "MIDI volume")]
    MidiVolume,

    #[strum(to_string = "Pan pot")]
    PanPot,

    #[strum(to_string = "General controller 1")]
    GeneralController1,

    #[strum(to_string = "General controller 2")]
    GeneralController2,

    #[strum(to_string = "General controller 3")]
    GeneralController3,

    #[strum(to_string = "General controller 4")]
    GeneralController4,

    #[strum(to_string = "General controller 5")]
    GeneralController5,

    #[strum(to_string = "General controller 6")]
    GeneralController6,

    #[strum(to_string = "General controller 7")]
    GeneralController7,

    #[strum(to_string = "General controller 8")]
    GeneralController8,
}

/// Control destination.
#[derive(
    Debug, Eq, PartialEq,
    Copy, Clone,
    TryFromPrimitive,
    Default,
    strum_macros::Display
)]
#[repr(u8)]
pub enum ControlDestination {
    #[default]
    #[strum(to_string = "Pitch offset")]
    PitchOffset,

    #[strum(to_string = "Cutoff offset")]
    CutoffOffset,

    #[strum(to_string = "Level")]
    Level,

    #[strum(to_string = "Vibrato depth offset")]
    VibratoDepthOffset,

    #[strum(to_string = "Growl depth offset")]
    GrowlDepthOffset,

    #[strum(to_string = "Tremolo depth offset")]
    TremoloDepthOffset,

    #[strum(to_string = "LFO speed offset")]
    LfoSpeedOffset,

    #[strum(to_string = "Attack time offset")]
    AttackTimeOffset,

    #[strum(to_string = "Decay 1 time offset")]
    Decay1TimeOffset,

    #[strum(to_string = "Release time offset")]
    ReleaseTimeOffset,

    #[strum(to_string = "Velocity offset")]
    VelocityOffset,

    #[strum(to_string = "Resonance offset")]
    ResonanceOffset,

    #[strum(to_string = "Pan pot offset")]
    PanPotOffset,

    #[strum(to_string = "Formant filter bias offset")]
    FormantFilterBiasOffset,

    #[strum(to_string = "Formant filter envelope LFO depth offset")]
    FormantFilterEnvelopeLfoDepthOffset,

    #[strum(to_string = "Formant filter envelope LFO speed offset")]
    FormantFilterEnvelopeLfoSpeedOffset,

    #[strum(to_string = "Harmonic low offset")]
    HarmonicLowOffset,

    #[strum(to_string = "Harmonic high offset")]
    HarmonicHighOffset,

    #[strum(to_string = "Harmonic even offset")]
    HarmonicEvenOffset,

    #[strum(to_string = "Harmonic odd offset")]
    HarmonicOddOffset,
}

/// Macro controller.
#[derive(Debug)]
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
            depth1: MacroParameterDepth::new(0),
            destination2: Default::default(),
            depth2: MacroParameterDepth::new(0),
        }
    }
}

impl fmt::Display for MacroController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dest1={} Depth={}\nDest2={} Depth={}",
            self.destination1, self.depth1, self.destination2, self.depth2
        )
    }
}

impl SystemExclusiveData for MacroController {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        eprintln!("MacroController from bytes {:?}", data);

        Ok(MacroController {
            destination1: ControlDestination::try_from(data[0]).unwrap(),
            depth1: MacroParameterDepth::from(data[1]),
            destination2: ControlDestination::try_from(data[2]).unwrap(),
            depth2: MacroParameterDepth::from(data[3]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![
            self.destination1 as u8,
            self.depth1.into(),
            self.destination2 as u8,
            self.depth2.into()
        ]
    }

    fn data_size() -> usize { 4 }
}

/// Assignable controller.
#[derive(Default, Debug)]
pub struct AssignableController {
    pub source: ControlSource,
    pub destination: ControlDestination,
    pub depth: ControlDepth,
}

impl SystemExclusiveData for AssignableController {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(AssignableController {
            source: ControlSource::try_from(data[0]).unwrap(),
            destination: ControlDestination::try_from(data[1]).unwrap(),
            depth: ControlDepth::from(data[2]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.source as u8, self.destination as u8, self.depth.into()]
    }

    fn data_size() -> usize { 3 }
}

/// Modulation settings.
#[derive(Default, Debug)]
pub struct ModulationSettings {
    pub pressure: MacroController,
    pub wheel: MacroController,
    pub expression: MacroController,
    pub assignable1: AssignableController,
    pub assignable2: AssignableController,
}

impl SystemExclusiveData for ModulationSettings {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(ModulationSettings {
            pressure: MacroController::from_bytes(&data[..4])?,
            wheel: MacroController::from_bytes(&data[4..8])?,
            expression: MacroController::from_bytes(&data[8..12])?,
            assignable1: AssignableController::from_bytes(&data[12..15])?,  // NOTE: only three bytes
            assignable2: AssignableController::from_bytes(&data[15..18])?,  // not four like macros
        })
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

    fn data_size() -> usize {
        3 * MacroController::data_size()
        + 2 * AssignableController::data_size()
    }
}

/// Pan type.
#[derive(
    Debug,
    Eq, PartialEq,
    Copy, Clone,
    TryFromPrimitive,
    Default,
    strum_macros::Display
)]
#[repr(u8)]
pub enum PanKind {
    #[default]
    Normal,

    Random,

    #[strum(to_string = "Key scale")]
    KeyScale,

    #[strum(to_string = "Negative key scale")]
    NegativeKeyScale,
}

/// Pan settings.
#[derive(Debug)]
pub struct PanSettings {
    pub pan_type: PanKind,
    pub pan_value: Pan,
}

impl Default for PanSettings {
    fn default() -> Self {
        PanSettings {
            pan_type: Default::default(),
            pan_value: Pan::new(0),
        }
    }
}

impl SystemExclusiveData for PanSettings {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(PanSettings {
            pan_type: PanKind::try_from(data[0]).unwrap(),
            pan_value: Pan::from(data[1]),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.pan_type as u8, self.pan_value.into()]
    }

    fn data_size() -> usize { 2 }
}

/// Switch kind.
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum Switch {
    #[default]
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

impl fmt::Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Switch::Off => String::from("Off"),
            Switch::HarmMax => String::from("Max harmonics"),
            Switch::HarmBright => String::from("Bright harmonics"),
            Switch::HarmDark => String::from("Dark harmonics"),
            Switch::HarmSaw => String::from("Saw harmonics"),
            Switch::SelectLoud => String::from("Select loud"),
            Switch::AddLoud => String::from("Add loud"),
            Switch::AddFifth => String::from("Add fifth"),
            Switch::AddOdd => String::from("Add odd"),
            Switch::AddEven => String::from("Add even"),
            Switch::He1 => String::from("Harmonic Env 1"),
            Switch::He2 => String::from("Harmonic Env 2"),
            Switch::HarmonicEnvelopeLoop => String::from("Harmonic envelope loop"),
            Switch::FfMax => String::from("Formant filter max"),
            Switch::FfComb => String::from("Formant filter comb"),
            Switch::FfHiCut => String::from("Formant filter high cut"),
            Switch::FfComb2 => String::from("Formant filter comb 2"),
        })
    }
}

/// Switch control settings.
#[derive(Debug, Default)]
pub struct SwitchControl {
    pub switch1: Switch,
    pub switch2: Switch,
    pub footswitch1: Switch,
    pub footswitch2: Switch,
}

impl SystemExclusiveData for SwitchControl {
    fn from_bytes(data: &[u8]) -> Result<Self, ParseError> {
        Ok(SwitchControl {
            switch1: Switch::try_from(data[0]).unwrap(),
            switch2: Switch::try_from(data[1]).unwrap(),
            footswitch1: Switch::try_from(data[2]).unwrap(),
            footswitch2: Switch::try_from(data[3]).unwrap(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.switch1 as u8, self.switch2 as u8, self.footswitch1 as u8, self.footswitch2 as u8]
    }

    fn data_size() -> usize { 4 }
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
#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, Default)]
#[repr(u8)]
pub enum AmplitudeModulation {
    #[default]
    Off,

    Source2,
    Source3,
    Source4,
    Source5,
    Source6,
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
        let mac = MacroController::from_bytes(&data);
        assert_eq!(mac.unwrap().destination1, ControlDestination::CutoffOffset);
    }

    #[test]
    fn test_velocity_switch_strum_display() {
        let vs = VelocitySwitch::Loud;
        assert_eq!(String::from("Loud"), format!("{}", vs));
    }

    #[test]
    fn test_control_destination_strum_display() {
        let cd = ControlDestination::VelocityOffset;
        assert_eq!(String::from("Velocity offset"), format!("{}", cd));
    }

    #[test]
    fn test_pan_kind_strum_display() {
        let p = PanKind::Normal;
        assert_eq!(String::from("Normal"), format!("{}", p));

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
