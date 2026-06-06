specs = [
    ('Volume', 'Volume of patch', 0, 127, 0),
    ('BenderPitch', 'Bender pitch', 0, 24, 0),
    ('BenderCutoff', 'Bender cutoff', 0, 31, 0),
    ('EnvelopeTime', 'Envelope time', 0, 127, 0),
    ('EnvelopeLevel', 'Envelope level', -63, 63, 0),
    ('EnvelopeRate', 'Envelope rate', 0, 127, 0),
    ('ControlTime', 'Control time', -63, 63, 0),
    ('EnvelopeDepth', 'Envelope depth', -63, 63, 0),
    ('EffectParameter', 'Effect parameter', 0, 127, 0),
    ('Cutoff', 'Cutoff', 0, 127, 0),
    ('Resonance', 'Resonance', 0, 31, 0),
    ('Level', 'Level', 0, 31, 0),
    ('PitchEnvelopeLevel', 'Pitch envelope level', -63, 63, 0),
    ('PitchEnvelopeTime', 'Pitch envelope time', 0, 127, 0),
    ('VelocityDepth', 'Velocity depth', 0, 127, 0),
    ('VelocityControlLevel', 'Velocity control level', 0, 127, 0),
    ('PortamentoLevel', 'Portamento level', 0, 127, 0),
    ('KeyOnDelay', 'Key on delay', 0, 127, 0),
    ('VelocitySensitivity', 'Velocity sensitivity', -63, 63, 0),
    ('ControlDepth', 'ControlDepth', -63, 63, 0),
    ('Depth', 'Depth', 0, 100, 0),
    ('Pan', 'Pan', -63, 63, 0),
    ('KeyScalingToGain', 'KeyScalingToGain', -63, 63, 0),
    ('Coarse', 'Coarse', -24, 24, 0),
    ('Fine', 'Fine', -63, 63, 0),
    ('MacroParameterDepth', 'Macro parameter depth', -31, 31, 0),
    ('MIDINote', 'MIDI note', 0, 127, 60),
    ('PatchNumber', 'Patch number', 0, 127, 0),
    ('Transpose', 'Transpose', -24, 24, 0)
]

#/// LFO depth (0...63, default 0)
##[derive(Debug, Clone, Copy, Eq, PartialEq)]
#pub struct Depth(i32);
#ranged_impl!(Depth, 0, 63, 0);

for spec in specs:
    range_string = f'{spec[2]}...{spec[3]}'
    print('/// ' + spec[1] + f' ({range_string}, default {spec[4]}).')
    print(f'#[derive (Debug, Clone, Copy, Eq, PartialEq)]')
    print(f'pub struct {spec[0]}(i32);')
    print(f'ranged_impl!({spec[0]}, {spec[2]}, {spec[3]}, {spec[4]});')
    print()
    print('impl From<u8> for ' + spec[0] + ' {')
    print('    fn from(value: u8) -> Self {')
    print('        Self::new(value as i32)')
    print('    }')
    print('}')
    print()
    print(f'impl From<{spec[0]}> for u8' + ' {')
    print(f'    fn from(value: {spec[0]}) -> Self' + ' {')
    print(f'        value.value() as u8    // used as such in SysEx, redefine if necessary')
    print('    }')
    print('}')

    print()
