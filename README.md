# hoodlum

HDL generation library for hardware synthesis.

```rust
let code = hdl! {

module (clk: in, LED1: out) {
    reg light = 1;

    on clk.posedge {
        light <= !light;
    }

    always {
        LED1 = light;
    }
}

};

let verilog = code.to_verilog(&Default::default());
```

## License

MIT or Apache-2.0.
