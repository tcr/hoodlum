# hoodlum (alpha)

**Hoodlum** is a nice-looking hardware description language that compiles to
Verilog. It wants to add stronger type guarantees and high-level concepts like
enums (and structs/typedefs), but also make FPGA design easier and more fun
to get involved with. To get started:

```
cargo install hoodlum
```

Add this to a file called `blinky.hdl`:

```rust
entity Main {
    in clk: bit,
    out LED1: bit,
}

impl Main {
    def mut index: uint{..6000000};
    on clk.posedge {
        if index == 6000000 - 1 {
            index <= 0;
            LED1 <= !LED1;
        } else {
            index <= index + 1;
        }
    }
}

```

And run:

```
hoodlum blinky.hdl -o output.v
```

The tutorial and examples target the $20 iCEstick evaluation board and the [IceStorm
open source compiler toolchain](http://www.clifford.at/icestorm/). See "examples/blinky"
for a simple blinking light.

**NOTE:** I'm learning Verilog and VHDL as I go along. Feel free to suggest ideas
and best practices in the issues section!

## Examples

* [examples/blinky](https://github.com/tcr/hoodlum/tree/master/examples/blinky) —
  Blink an LED on and off with a counter. Example for the iCEstick evaluation board.
* [examples/sequence](https://github.com/tcr/hoodlum/tree/master/examples/sequence) —
  Shows an LED display pattern compiled via a `sequence` generator.
* [examples/ntsc](https://github.com/tcr/hoodlum/tree/master/examples/ntsc) —
  Generates NTSC video and displays a static image.
* [examples/ethernet](https://github.com/tcr/hoodlum/tree/master/examples/ethernet) —
  Drives the `enc424j600` Ethernet PMOD board to send UDP packets.

## Language Design

Goals:

1. Define a hardware description language that's elegant, borrowing syntax from Rust
syntax and other modern (familiar) languages.
1. Emit compatible Verilog-2001 (and VHDL) code.
1. Create abstractions to simplify generation of state machines, sequential code,
and reset states.
1. Hackable so you can add your own constructs.
1. Statically detect errors before they reach synthesis stage.
1. In the future, add simulation capabilities.

Non-goals:

1. Don't compile Rust into HDL. Rust's stdlib fits an entirely different computing
   model. The abstraction mismatch makes code that's hard to debug.
1. Don't support compiling all features of Verilog-2001 or VHDL, just a functional subset.

## Tutorial

Entity definitions are in `entity` blocks. Logic definitions are in `impl` blocks.

Variables can be one of the following types:

* Registers of a certain bit width. These can have arithemtic operators: `|`
* Uints or Ints.

TODO: More documentation.

## License

MIT or Apache-2.0.
