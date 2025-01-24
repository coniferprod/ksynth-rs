all_types = [
    'Volume,0,127,0',
    'BenderPitch,0,24,0',
    'BenderCutoff, 0, 31, 0',
    'EnvelopeTime,0,127,0',
    'EnvelopeLevel,-63,63,0',
    'EnvelopeRate,0,127,0',
    'HarmonicEnvelopeLevel, 0, 63, 0',
    'Bias, -63, 63, 0',
    'ControlTime,-63,63,0',
    'EnvelopeDepth,-63,63,0',
    'LFOSpeed,0,127,0',
    'LFODepth,0,63,0',
    'KeyScaling,-63,63,0',
    'EffectParameter,0,127,0',
    'Cutoff,0,127,0',
    'Resonance,0,31,0',
    'Level,0,31,0',
    'PitchEnvelopeLevel,-63,63,0',
    'PitchEnvelopeTime,0,127,0',
    'VelocityDepth,0,127,0',
    'VelocityControlLevel,0,127,0',
    'PortamentoLevel,0,127,0',
    'KeyOnDelay,0,127,0',
    'VelocitySensitivity,-63,63,0',
    'ControlDepth,-63,63,0',
    'Depth,0,100,0',
    'Pan,-63,63,0',
    'KeyScalingToGain,-63,63,0',
    'Coarse,-24,24,0',
    'Fine,-63,63,0',
    'MacroParameterDepth,-31,31,0'
]

for t in all_types:
    parts = t.split(',')
    name = parts[0].strip()
    min_value = parts[1].strip()
    max_value = parts[2].strip()
    default_value = parts[3].strip()
    print('/// {} ({}...{})'.format(name, min_value, max_value, default_value))
    print('#[derive(Debug, Clone, Copy, Eq, PartialEq)]')
    print('pub struct {}(i32);'.format(name))
    print('crate::ranged_impl!({}, {}, {}, {});'.format(name, min_value, max_value, default_value))
    print()
