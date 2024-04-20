# Block single (Classic.KAA)

Kawai K5000 ASL Library, Classic

    f0  // SysEx initiator
    40  // Kawai manufacturer ID
    00  // MIDI channel 1
    21 00 0a 00 00  // Block, ADD Bank A 

    // Tone map (19 bytes)
    7f 7f 7f 7f 7f 7f 7f 7f
    7f 01 00 00 00 00 00 00
    00 00 00

    // Patch 001
    69  // single patch checksum

    // Common data
    00  // effect algorithm
    04 14 14 04 1e 04  // reverb
    29 0a 59 62 00 00  // effect1
    0b 00 00 05 09 00  // effect2
    15 00 0d 41 0c 00  // effect3
    0c 00 32 00 32 00  // effect4
    43 40 3f 3d 3f 42 46  // GEQ
    00  // drum mark
    55 70 52 69 74 65 20 20  // name="UpRite  "
    7e  // volume
    00  // poly = POLY1
    00  // no use
    04  // no. of sources
    0f  // src_mute1
    00  // AM
    02 04 5f 02 00 40  // effect control
    00  // portamento
    00  // portamento speed
    
    // Macro controllers (16 bytes total)
    12 00 01 00 0a 00 0c 00 
    5f 40 5f 40 5f 40 5f 40 

    02 07 01 04  // Switches
    
    // Source 1
    // Control (28 bytes)
    00 7f  // zone lo/hi
    10  // velo sw
    00  // effect path
    7f  // volume
    02 00  // bender pitch & cutoff
    00 40 00 40  // press
    03 40 00 40  // wheel
    02 5f 00 40  // expression
    00 00 40  // assignable 1
    00 00 40  // assignable 2
    00   // key on delay
    01 40  // pan type & value

    // DCO (12 bytes)
    04  // wave kit MSB
    00  // wave kit LSB
    40  // coarse
    3e  // fine
    00  // fixed key
    00  // KS pitch
    40 04 40 40 40 40  // Pitch env
    
    // DCF (20 bytes)
    00  // active/bypass
    00  // mode
    04  // velo curve
    00  // resonance
    00  // DCF level
    14  // cutoff
    57  // cutoff KS depth
    43  // cutoff velo depth
    60  // DCF env depth
    
    00 05 75 68 40 3c  // DCF env
    40 40  // DCF KS to env
    68 40 40  // DCF velo to env

    // DCA (15 bytes)
    04  // vel curve
    00 00 7f 6e 00 00  // DCA env
    40 40 40 40   // DCA KS to env
    18 40 40 40   // DCA vel sens
    
    // LFO (11 bytes)
    00  // waveform
    5b  // speed
    01  // delay onset
    0e 0a   // fade in time & to speed
    00 40   // pitch depth & KS
    00 40   // DCF depth & KS
    00 40   // DCA depth & KS


