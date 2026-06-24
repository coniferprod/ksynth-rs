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

### Leaning on the `From` and `Into` traits

Most synthesizer parameters appear as one byte in the System Exclusive data.
However, they often need a little adjustment to get them from the
7-bit MIDI byte to their respective range. For example, the allowed MIDI channel
value is 1...16, but it is stored as zero-based in the SysEx data, so it
actually appears as 0...15. Many other parameters are expressed similarly,
with a varying offset that needs to be added or subtracted.

The `std::convert::From` trait is used to make these conversions nicer.
A `From<u8>` trait implementation on a newtype implementing the `Ranged` trait
adjusts an incoming System Exclusive data byte of type `u8` to the domain value. For example,
for the `MIDIChannel` type shown above the `From<u8>` implementation is:

```rust
impl From<u8> for MIDIChannel {
    fn from(item: u8) -> Self {
        Self((item as i32) + 1)  // bring into 1...16
    }
}
```

A `MIDIChannel` value is constructed from a System Exclusive data byte like this:

```rust
let b = 9u8;
let channel = MIDIChannel::from(b);
```

Similarly, the `Into` trait is used to convert the wrapped value of a newtype
implementing the `Ranged` trait into a System Exclusive data byte. For example,
for the `MIDIChannel` type this involves subtracting one from the value:

```rust
impl Into<u8> for MIDIChannel {
    fn into(self) -> u8 {
        (self.value() as u8) - 1
    }
}
```

Unfortunately it is necessary to add `From` and `Into` implementations also for
those newtypes that don't require an incoming or outgoing adjustment.
The `ranged_impl!` macro could
create default implementations for these traits, but they can't be overridden.
You could always create another macro that creates the boilerplate for these
implementations. The output of that macro should look something like this:

```rust
impl From<u8> for KeyOnDelay {
    fn from(value: u8) -> Self {
        Self::new(value as i32)
    }
}

impl Into<u8> for KeyOnDelay {
    fn into(self) -> u8 {
        self.value() as u8
    }
}
```
