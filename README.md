# hoodlum

**Hoodlum** is a nice-looking hardware description language that compiles to
Verilog. It wants to add stronger type guarantees and high-level concepts like
enums (and structs/typedefs), but also make FPGA design easier and more fun
to get involved with. To get started:

```
cargo install hoodlum
```

Add this to a file called `blink.hdl`:

```rust
entity Main {
    in clk: bit,
    out LED1: bit
}

impl Main {
    def mut light: bit;

    on clk.posedge {
        light <= !light;
    }

    LED1 = light;
}
```

And run:

```
hoodlum test.hdl -o output.v
```

The tutorial and examples target the $20 iCEstick evaluation board and the [IceStorm
open source compiler toolchain](http://www.clifford.at/icestorm/). See "examples/blinky"
for a simple blinking light.

**NOTE:** I'm learning Verilog and VHDL as I go along. Feel free to suggest ideas
and best practices in the issues section!

## Examples

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
