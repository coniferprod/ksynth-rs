# ksynth-rs

Patch manipulation helpers for Kawai digital synths:
K5, K1, K4, and K5000.

## Domain types and the `Ranged` trait

The crate contains several domain types that implement the `Ranged` trait, 
defined in the crate. Each of these types is a "newtype", followed by the trait implementation.

It becomes quite tedious to implement the `Ranged` trait for all the
required types, so the `ranged_impl!` macro is defined to handle the
grunt work. It generates an implementation of the `Ranged` trait for a given type,
along with the `Default` and `Display` traits. The default value is the
`DEFAULT` associated constant, while the displayed value is the actual
value wrapped by the type.

The `ranged_impl!` macro takes four parameters: type name, minimum value, maximum value,
and default value. The last three become the associated consts
`FIRST`, `LAST`, and `DEFAULT`, respectively.

To create a new domain type, make a newtype and use the `ranged_impl!` macro.
For example, the `MIDIChannel` type represents values from 1 to 16 inclusive,
with the default value 1:

```rust
/// MIDI channel (1...16)
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MIDIChannel(i32);
crate::ranged_impl!(MIDIChannel, 1, 16, 1);
```

Use the constructor of the domain type when making a value:

```rust
let channel = MIDIChannel::new(1);
```

If you specify an initial value that is outside the range `FIRST..=LAST`
the constructor will panic, so you should first check that the value fits
using the `contains` method:

```rust
let b = 32;
let channel: MIDIChannel;
if MIDIChannel::contains(b) {
    channel = MIDIChannel::new(b);
    println!("channel = {}", channel);
} else {
    eprintln!("Invalid MIDI channel: {}", b);
}
```

Remember to install `cargo-expand` if you want to see the result of the
macro expansion:

```shell
cargo install cargo-expand
```

Then use the `cargo expand` command to view the generated code.

Alternatively, you can also use Rust Analyzer in Visual Studio Code
to recursively expand the macro; see the _Expand macro recursively
at caret_ command.

For a longer explanation of the `Ranged` trait, see 
Flecks of Rust #12, [Subrange types in Rust](https://coniferproductions.com/rust/flecks/12/).

### Converting between SysEx bytes and domain types

Most synthesizer parameters appear as one byte in the System Exclusive data.
However, they often need a little adjustment to get them from the
7-bit MIDI byte to their respective range. For example, the allowed MIDI channel
value is 1...16, but it is stored as zero-based in the SysEx data, so it
actually appears as 0...15. Many other parameters are expressed similarly,
with a varying offset that needs to be added or subtracted.

The `Adjustment` trait is used to make these conversions nicer.
It has two methods: `incoming` and `outgoing`, which adjust the incoming SysEx bytes
as they are converted into domain types, and adjust the domain type values as they
are emitted into SysEx.
