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
    type_name = spec[0]
    range_string = f'{spec[2]}...{spec[3]}'
    print('/// ' + spec[1] + f' ({range_string}, default {spec[4]}).')
    print(f'#[derive (Debug, Clone, Copy, Eq, PartialEq)]')
    print(f'pub struct {type_name}(i32);')
    print(f'ranged_impl!({type_name}, {spec[2]}, {spec[3]}, {spec[4]});')
    print()
    print(f'impl Encoding for {type_name} ' + '{ }  // use the default implementations')
    print()
