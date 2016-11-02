#[macro_use] extern crate hoodlum;

use std::io::prelude::*;
use std::fs::File;
use hoodlum::*;

fn main() {
    let code = hdl! {

entity Second(clk: in, ready: out) {
    let index: uint{..1200000} = 0;

    on clk.posedge {
        if index == 12000000 - 1 {
            ready <= 1;
        } else {
            index <= index + 1;
            ready <= 0;
        }
    }
}

entity Half(clk: in, ready: out) {
    let index: uint{..1200000} = 0;

    on clk.posedge {
        if index == 10000000 - 1 {
            ready <= 1;
        } else {
            index <= index + 1;
            ready <= 0;
        }
    }
}

entity SpiMaster(
    rst: in,
    clk: in,
    tx_trigger: in,
    tx_ready: out,
    tx_byte: in[8],
    rx_byte: out[8],
    spi_clk: out,
    spi_tx: out,
    spi_rx: in,
) {
    let live_clk: reg[1] = 0;

    // Internal signals.
    let read_index: uint{0..8} = 0;
    let internal_clk: reg[1] = 0;

    let FSM: uint{..32} = 0;

    // Generate SPI signal from internal clock + SPI state.
    always {
    //    if live_clk {
    //        spi_clk = internal_clk;
    //    } else {
            spi_clk = live_clk && internal_clk;
    //    }
    }

    // Generate divided SPI clock.
    let div_idx: uint{..40} = 0;
    on clk.negedge {
        reset rst {
            if div_idx == 40 - 1 {
                internal_clk <= !internal_clk;
                div_idx <= 0;
            } else {
                div_idx <= div_idx + 1;
            }
        }
    }

    // Sample read values from positive clock edge.
    on internal_clk.posedge {
        rx_byte[read_index] <= spi_rx;
    //    reset rst {
    //        if state == SpiState.DATA {
    //            rx_byte[read_index] <= spi_rx;
    //        }
    //    }
    }

    let transmitting = 0;
    let transmit_save = 1;

    on clk.posedge {
        reset rst {
            // if tx_trigger is high, and we are not transmitting, start
            if transmit_save == transmitting {
                tx_ready <= 1;
                transmit_save <= !transmitting;
            } else if tx_trigger {
                tx_ready <= 0;
            }
        }
    }

    // SPI output state machine.
    on internal_clk.negedge {
        reset rst {
            fsm {
                // Wait for transition trigger.
                spi_tx <= 0;
                await tx_ready == 0;

                // Enable output clock.
                live_clk <= 1;

                // Start sequence.
                read_index <= 7;
                spi_tx <= tx_byte[7];
                yield;

                // Write bits.
                while read_index > 0 {
                    spi_tx <= tx_byte[read_index - 1];
                    read_index <= read_index - 1;
                    yield;
                }

                // Disable output clock.
                live_clk <= 0;
                transmitting <= !transmitting;

                // Loop forever.
                //loop {
                //    yield;
                //}
            }
        }
    }
}

entity Ethernet(
    rst: in,
    tx_clk: in,
    LED1: out,
    LED2: out,
    LED3: out,
    CS: out,
    spi_bit: out, // MOSI
    spi_rx: in, // MISO
    spi_clk: out,
) {
    let tx_valid = 0;
    let tx_byte: reg[8] = 0;
    let spi_ready;
    let spi_rx_value: reg[8];
    let spi = SpiMaster {
        rst: rst,
        clk: tx_clk,
        tx_trigger: tx_valid,
        tx_ready: spi_ready,
        tx_byte: tx_byte,
        rx_byte: spi_rx_value,
        spi_clk: spi_clk,
        spi_tx: spi_bit,
        spi_rx: spi_rx
    };


    let FSM: uint{..32} = 0;
    on tx_clk.negedge {
        reset rst {
            fsm {
                LED1 <= 1;

                CS <= 0;
                tx_valid <= 1;
                tx_byte <= 0x22; //EWCRU;
                await spi_ready;
                tx_byte <= 0x16; //EEUDASTL;
                await spi_ready;
                tx_byte <= 0x34;
                await spi_ready;
                tx_byte <= 0x12;
                await spi_ready;

                CS <= 1;
                tx_valid <= 0;
                // Skip two clock cycles
                yield;
                yield;

                CS <= 0;
                tx_valid <= 1;
                tx_byte <= 0x20; //ERCRU;
                await spi_ready;
                tx_byte <= 0x16; //EEUDASTL;
                await spi_ready;
                // noop
                await spi_ready;
                if spi_rx_value == 0x34 {
                    LED2 <= 1;
                }
                await spi_ready;
                if spi_rx_value == 0x12 {
                    LED3 <= 1;
                }

                CS <= 1;
                tx_valid <= 0;
                loop {
                    yield;
                }
            }
        }
    }
}

entity Main(
    clk: in,
    LED1: out,
    LED2: out,
    LED3: out,
    LED4: out,
    LED5: out,
    PMOD1: out,
    PMOD2: out,
    PMOD3: in,
    PMOD4: out,
    PMOD7: out,
    //PMOD8: out,
    //PMOD9: out,
    //PMOD10: out,
) {
    // PMOD1 = CS
    // PMOD2 = MOSI
    // PMOD3 = MISO
    // PMOD4 = SCLK
    let ready;
    let sec = Second { clk: clk, ready: ready };
    let half = Half { clk: clk, ready: PMOD7 };
    let ether = Ethernet {
        rst: ready,
        tx_clk: clk,
        LED1: LED1,
        LED2: LED2,
        LED3: LED3,
        CS: PMOD1,
        spi_bit: PMOD2,
        spi_rx: PMOD3,
        spi_clk: PMOD4,
    };

    always {
        LED5 = !ready;
    }
}

};

/*
    let code = hdl! {

entity Second(clk: in, ready: out) {
    let index: uint{..1200000} = 0;

    on clk.posedge {
        if index == 12000000 - 1 {
            ready <= 1;
        } else {
            index <= index + 1;
            ready <= 0;
        }
    }
}

entity Entry(clk: in, LED1: out, LED2: out, LED3: out, LED4: out, LED5: out) {
    let rot: reg[4] = 0b0001;
    let divider: uint{..1200000} = 0;
    let index: uint{..1200000} = 0;
    let FSM: uint{..3} = 0;

    let ready;
    let sec = Second(clk, ready);

    on clk.posedge {
        reset ready {
            if divider == 1200000 - 1 {
                divider <= 0;
                fsm {
                    rot <= 0b0001;
                    yield;
                    rot <= 0b0011;
                    yield;
                    rot <= 0b0110;
                    yield;
                    rot <= 0b1100;
                    yield;
                    rot <= 0b1000;
                    yield;
                    rot <= 0b0000;
                }
            } else {
                divider <= divider + 1;
            }
        }
    }

    always {
        LED1 = ready && rot[0];
        LED2 = ready && rot[1];
        LED3 = ready && rot[2];
        LED4 = ready && rot[3];
        LED5 = !ready;
    }
}

};
*/

    //println!("");
    //println!("AST: {:?}", code);

    let verilog = code.to_verilog(&Default::default());
    println!("Verilog:");
    codelist(&verilog);

    let mut f = File::create("out/test.v").unwrap();
    f.write_all(verilog.as_bytes()).unwrap();

    println!("");
    println!("File written as out/test.v");
}
