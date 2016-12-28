# hoodlum

A nice-looking hardware description language with FSM generation, simple reset
generation, and more. Hackable so you can add your own constructs.

```
cargo install hoodlum
```

Add this to `test.hdl`:

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

    always {
        LED1 = light;
    }
}
```

And run

```
hoodlum test.hdl output.v
```

Examples are being tested with the iCEstick evaluation board and the [IceStorm
open source compiler toolchain](http://www.clifford.at/icestorm/). See "examples/rot"
for an example that works with this board.

**NOTE:** I'm learning Verilog and VHDL as I go along. Feel free to suggest ideas
or best practices!

---

Goals:

1. Emit compatible Verilog and VHDL code.
1. Define a DSL that's elegant, borrowing from Rust syntax where applicable.
1. Create abstractions to simplify generation of state machines and complex logic.
1. Detect errors before they reach synthesis stage.
1. In the future, add simulation capabilities.

Non-goals:

1. Don't compile Rust into HDL. Rust's stdlib fits an entirely different computing
   model. The abstraction mismatch makes bad output.
1. Don't support all features of Verilog-2001 or VHDL, just a functional subset.

## Language

Entity definitions are in `entity` blocks. Logic definitions are in `impl` blocks.

Variables can be one of the following types:

* Registers of a certain bit width. These can have arithemtic operators: `|`
* Uints or Ints.

TODO: More documentation

## License

MIT or Apache-2.0.
