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
  Generates NTSC video and displays a static image. **Requires an external circuit.**
* [examples/ethernet](https://github.com/tcr/hoodlum/tree/master/examples/ethernet) —
  Drives the `enc424j600` Ethernet PMOD board to send UDP packets. **Requires an
  enc424j600 PMOD board.**

### Running on an [iCEstick Evaluation Kit](http://www.latticesemi.com/icestick)

Each example has a Makefile allowing you to run:

```
make && make flash
```

If you are getting errors on macOS about the device not being found, try running
this first to unload the native FTDI driver:

```
sudo kextunload -b com.apple.driver.AppleUSBFTDI
```

### Atom Highlighting Plugin

You can highlight `.hdl` scripts in Atom. Run the following to install the Atom Plugin:

```
cd language-hoodlum
apm link --local .
```

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

A Hoodlum script is composed of entity definitions in `entity` blocks, and
logic definitions are in `impl` blocks. The topmost entity is the `Main` block.

```
entity Main {
    in clk: bit,
    out LED1: bit,
}
```

These top level entries reference pin definitions in your `.pcf` file; see
"examples/blinky/icestick.pcf" for one for the iCEstick evaluation board.

Each of these entity members declares:

* A **direction**: `in` indicates a value will be *read by* the entity, and `out`
  indicates a value will be *written by* the entity.
* A **name**. This can be referenced by name in the `impl` block.
* A **type**. Here, we declare both values to be one `bit` wide, i.e. a logic value
  of either `0` or `1` (or `x`, in more complicated logic).

Inside of a corresponding `impl` block, we can reference these values:

```
impl Main {
    def mut toggle: bit;

    def toggle_next: bit = !toggle;

    on clk.negedge {
        toggle <= toggle_next;
    }

    LED1 = toggle;
}
```

Let's break this down. First, we define a variable "toggle" which is one
bit wide. We declare this as `def mut`, meaning the variable is mutable inside
of a `on` block.

Next, we define a variable "toggle_next" which is *not* mutable. This means we
have a stateless definition that requires no state (latches or flip-flops). We
can declare its value inline with the definition or as a standalone declaration.
Its value is always equal to the inverse of the "toggle" variable.

The `on` block has a sensitivity to either the positive or negative falling edge
of a signal value. Inside of an `on` block we can make assignments to mutable
variables using the nonblocking `<=` or blocking `:=` operators. Here, every
time we see a negative clock edge, we reset the "toggle" latch to be the value
of "toggle_next", i.e. the inverse of itself.

(Nonblocking definitions are deferred until the end of the `on` block; this simplifies
stateful logic to all be set on the same clock impulse.)

Last, we declare a value for our output variable. `LED1` is set to always be
equal to the value of the mutable "toggle" value.

At 12Mhz, you won't be able to see the LED toggling at this speed! See
"examples/blinky/blinky.hdl" for how to use a counter to make it visible.

### Types

Each definition has a type, declared by the format `def [mut] name: <type>`.

* `bit` declares a value that can hold `0`, `1`, or `x`.
* `bit[n]` declares a port that is *n* bits wide. You can reference individual
  bits using a slice operator (e.g. `myvariable[1]`) and set it at once with
  a numeric literal (e.g. `myvariable = 0b101101`, etc.)
* `uint{..n}` and `int{..n}` declares integers (unsigned and signed) whose max
  value is `n` (to the nearest power of two). This is shorthand for calculating
  the maximum bit length for a given integer.

Inside an entity, you can also define sub-entities:

```
entity Toggle {
    in value: bit,
    out opposite: bit,
}

impl Toggle { ... }

entity Main { ... }

impl Main {
    def clk_prime: bit;
    def toggle = Toggle {
        value: clk,
        opposite: clk_prime,
    }
}
```

Main declares a sub-entity Toggle with an input value given as "clk" and a
output value as "clk_prime". Note that you have to declare output variables
alongside the sub-entity to use them.

## License

MIT or Apache-2.0.
