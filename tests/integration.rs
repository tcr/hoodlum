extern crate hoodlum;

use hoodlum::*;

/*
#[test]
fn rewrite_fsm1() {
    let code = r#"
fsm {
    while !tx_trigger {
        spi_tx <= 0;
        yield;
    }

    read_index <= 7;
    spi_tx <= tx_byte[7];
    tx_ready <= 0;
    yield;

    while read_index > 0 {
        spi_tx <= tx_byte[read_index - 1];
        read_index <= read_index - 1;
        yield;
    }

    tx_ready <= 1;

    loop {
        a <= 1;
        yield;
        a <= 2;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (FSM)
    0: begin
        if (!tx_trigger) begin
            spi_tx <= 0;
        end
        else begin
            read_index <= 7;
            spi_tx <= tx_byte[7];
            tx_ready <= 0;
            FSM <= 1;
        end
    end
    1, 2: begin
        if (FSM != 2 && read_index > 0) begin
            spi_tx <= tx_byte[read_index - 1];
            read_index <= read_index - 1;
        end
        else begin
            if (FSM == 1) begin
                tx_ready <= 1;
            end
            if (FSM == 2) begin
                a <= 2;
            end
            a <= 1;
            FSM <= 2;
        end
    end
endcase
"#);
}


#[test]
fn rewrite_fsm2() {
    let code = r#"
fsm {
    while !tx_trigger {
        spi_tx <= 0;
        yield;
    }

    read_index <= 7;
    spi_tx <= tx_byte[7];
    tx_ready <= 0;
    yield;

    while read_index > 0 {
        spi_tx <= tx_byte[read_index - 1];
        read_index <= read_index - 1;
        yield;
    }

    tx_ready <= 1;

    loop {
        a <= 1;
        yield;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (FSM)
    0: begin
        if (!tx_trigger) begin
            spi_tx <= 0;
        end
        else begin
            read_index <= 7;
            spi_tx <= tx_byte[7];
            tx_ready <= 0;
            FSM <= 1;
        end
    end
    1, 2: begin
        if (FSM != 2 && read_index > 0) begin
            spi_tx <= tx_byte[read_index - 1];
            read_index <= read_index - 1;
        end
        else begin
            if (FSM == 1) begin
                tx_ready <= 1;
            end
            a <= 1;
            FSM <= 2;
        end
    end
endcase
"#);
}
*/

#[test]
fn rewrite_fsm3() {
    let code = r#"
fsm {
    while !tx_trigger {
        spi_tx <= 0;
        yield;
    }

    read_index <= 7;
    spi_tx <= tx_byte[7];
    tx_ready <= 0;
    yield;

    while read_index > 0 {
        spi_tx <= tx_byte[read_index - 1];
        read_index <= read_index - 1;
        yield;
    }

    tx_ready <= 1;

    while r > 0 {
        a <= 1;
        yield;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0: begin
        if (!(tx_trigger)) begin
            spi_tx <= 0;
            _FSM = 0;
        end
        else begin
            read_index <= 7;
            spi_tx <= tx_byte[7];
            tx_ready <= 0;
            _FSM = 1;
        end
    end
    1, 2: begin
        if (((_FSM == 1) && (read_index > 0))) begin
            spi_tx <= tx_byte[(read_index - 1)];
            read_index <= (read_index - 1);
            _FSM = 1;
        end
        else begin
            if ((_FSM == 1 || _FSM == 3)) begin
                tx_ready <= 1;
                _FSM = 1;
            end
            if (((_FSM == 1 || _FSM == 2) && (r > 0))) begin
                a <= 1;
                _FSM = 2;
            end
            else begin
                _FSM = 0;
            end
        end
    end
endcase
"#);
}


#[test]
fn rewrite_await1() {
    let code = r#"
fsm {
    spi_tx <= 0;

    while !tx_trigger {
        a <= 1;
        yield;
        b <= 1;
    }

    spi_tx <= 1;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1: begin
        if ((_FSM == 0)) begin
            spi_tx <= 0;
        end
        if ((_FSM == 1)) begin
            b <= 1;
        end
        if (((_FSM == 0 || _FSM == 1) && !(tx_trigger))) begin
            a <= 1;
            _FSM = 1;
        end
        else begin
            spi_tx <= 1;
            _FSM = 0;
        end
    end
endcase
"#);
}


#[test]
fn rewrite_await2() {
    let code = r#"
fsm {
    spi_tx <= 0;

    while !tx_trigger {
        a <= 1;
        yield;
    }

    spi_tx <= 1;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1: begin
        if ((_FSM == 0)) begin
            spi_tx <= 0;
        end
        if (((_FSM == 0 || _FSM == 1) && !(tx_trigger))) begin
            a <= 1;
            _FSM = 1;
        end
        else begin
            spi_tx <= 1;
            _FSM = 0;
        end
    end
endcase
"#);
}

#[test]
fn rewrite_await3() {
    let code = r#"
fsm {
    spi_tx <= 0;

    while !tx_trigger {
        yield;
    }

    spi_tx <= 1;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1: begin
        if ((_FSM == 0)) begin
            spi_tx <= 0;
        end
        if (((_FSM == 0 || _FSM == 1) && !(tx_trigger))) begin
            _FSM = 1;
        end
        else begin
            spi_tx <= 1;
            _FSM = 0;
        end
    end
endcase
"#);
}


#[test]
fn rewrite_await4() {
    let code = r#"
fsm {
    spi_tx <= 0;

    while !tx_trigger {
        yield;
    }

    spi_tx <= 1;

    loop {
        a <= 1;
        yield;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1, 2: begin
        if ((_FSM == 0)) begin
            spi_tx <= 0;
        end
        if (((_FSM == 0 || _FSM == 2) && !(tx_trigger))) begin
            _FSM = 2;
        end
        else begin
            if ((_FSM == 0 || _FSM == 2)) begin
                spi_tx <= 1;
                _FSM = 0;
            end
            if (((_FSM == 0 || _FSM == 1) && 1)) begin
                a <= 1;
                _FSM = 1;
            end
            else begin
                _FSM = 0;
            end
        end
    end
endcase
"#);
}


#[test]
fn rewrite_await5() {
    let code = r#"
fsm {
    LED1 <= 1;

    CS <= 0;
    tx_valid <= 1;
    tx_byte <= 0x22;
    await !spi_ready;
    await spi_ready;
    yield;
    tx_byte <= 0x16;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0: begin
        LED1 <= 1;
        CS <= 0;
        tx_valid <= 1;
        tx_byte <= 34;
        _FSM = 3;
    end
    3: begin
        if (!(spi_ready)) begin
            _FSM = 2;
        end
    end
    2: begin
        if (spi_ready) begin
            _FSM = 1;
        end
    end
    1: begin
        tx_byte <= 22;
        _FSM = 0;
    end
endcase
"#);
}


#[test]
fn rewrite_await6() {
    let code = r#"
fsm {
    LED1 <= 1;

    CS <= 0;
    tx_valid <= 1;
    tx_byte <= 0x22;
    while !spi_ready { yield; }
    while spi_ready { yield; }
    tx_byte <= 0x16;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1, 2: begin
        if ((_FSM == 0)) begin
            LED1 <= 1;
            CS <= 0;
            tx_valid <= 1;
            tx_byte <= 34;
        end
        if (((_FSM == 0 || _FSM == 2) && !(spi_ready))) begin
            _FSM = 2;
        end
        else begin
            if (((_FSM == 0 || _FSM == 1) && spi_ready)) begin
                _FSM = 1;
            end
            else begin
                tx_byte <= 22;
                _FSM = 0;
            end
        end
    end
endcase
"#);
}

#[test]
fn rewrite_await7() {
    let code = r#"
fsm {
    tx_valid <= 0;
    sleep_counter <= 0;
    while sleep_counter < 36 {
        sleep_counter <= sleep_counter + 1;
        yield;
    }
    tx_valid <= 1;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1: begin
        if ((_FSM == 0)) begin
            tx_valid <= 0;
            sleep_counter <= 0;
        end
        if (((_FSM == 0 || _FSM == 1) && (sleep_counter < 36))) begin
            sleep_counter <= (sleep_counter + 1);
            _FSM = 1;
        end
        else begin
            tx_valid <= 1;
            _FSM = 0;
        end
    end
endcase
"#);
}


#[test]
fn rewrite_await8() {
    let code = r#"
fsm {
    tx_valid <= 0;
    sleep_counter <= 0;
    while sleep_counter < 36 {
        sleep_counter <= sleep_counter + 1;
        yield;
    }

    tx_valid <= 0;
    sleep_counter <= 0;
    while sleep_counter < 36 {
        sleep_counter <= sleep_counter + 1;
        yield;
    }
    tx_valid <= 1;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1, 2: begin
        if ((_FSM == 0)) begin
            tx_valid <= 0;
            sleep_counter <= 0;
        end
        if (((_FSM == 0 || _FSM == 2) && (sleep_counter < 36))) begin
            sleep_counter <= (sleep_counter + 1);
            _FSM = 2;
        end
        else begin
            if ((_FSM == 0 || _FSM == 2)) begin
                tx_valid <= 0;
                sleep_counter <= 0;
                _FSM = 0;
            end
            if (((_FSM == 0 || _FSM == 1) && (sleep_counter < 36))) begin
                sleep_counter <= (sleep_counter + 1);
                _FSM = 1;
            end
            else begin
                tx_valid <= 1;
                _FSM = 0;
            end
        end
    end
endcase
"#);
}

#[test]
fn rewrite_yield1() {
    let code = r#"
fsm {
    tx <= 0;
    yield;
    tx <= 1;
    yield;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0: begin
        tx <= 0;
        _FSM = 2;
    end
    2: begin
        tx <= 1;
        _FSM = 1;
    end
    1: begin
        _FSM = 0;
    end
endcase
"#);
}


#[test]
fn rewrite_fsm_a() {
    let code = r#"
fsm {
    while !tx_trigger {
        a <= 0;
        yield;
    }

    a <= 1;
    yield;

    while read_index > 0 {
        a <= 1;
        yield;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0: begin
        if (!(tx_trigger)) begin
            a <= 0;
            _FSM = 0;
        end
        else begin
            a <= 1;
            _FSM = 1;
        end
    end
    1: begin
        if ((read_index > 0)) begin
            a <= 1;
            _FSM = 1;
        end
        else begin
            _FSM = 0;
        end
    end
endcase
"#);
}

#[test]
fn rewrite_fsm_if_1() {
    let code = r#"
fsm {
    if test > 0 {
        a <= 1;
        yield;
        a <= 2;
    }
    a <= 3;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1: begin
        if ((_FSM == 1)) begin
            a <= 2;
        end
        if (((_FSM == 0) && (test > 0))) begin
            a <= 1;
            _FSM = 1;
        end
        else begin
            a <= 3;
            _FSM = 0;
        end
    end
endcase
"#);
}

#[test]
fn rewrite_fsm_while_1() {
    let code = r#"
fsm {
    while a < 10 {
        a <= a + 1;
        yield;
    }

    while test > 0 {
        a <= 1;
        yield;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1: begin
        if (((_FSM == 0) && (a < 10))) begin
            a <= (a + 1);
            _FSM = 0;
        end
        else begin
            if (((_FSM == 0 || _FSM == 1) && (test > 0))) begin
                a <= 1;
                _FSM = 1;
            end
            else begin
                _FSM = 0;
            end
        end
    end
endcase
"#);
}


//#[test]
//fn rewrite_fsm_if_3() {
//    let code = r#"
//fsm {
//    while test > 0 {
//        a <= 1;
//        yield;
//    }
//
//    if a < 10 {
//        a <= a + 1;
//        yield;
//    }
//
//    while test > 0 {
//        a <= 1;
//        yield;
//    }
//}
//"#;
//
//    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));
//
//    let out = res.to_verilog(&VerilogState::new());
//
//    println!("OK:\n{}", out);
//
//    assert_eq!(out, r#"case (_FSM)
//    0, 1: begin
//        if (_FSM == 1) begin
//            a <= 2;
//        end
//        if (_FSM != 1 && test > 0) begin
//            a <= 1;
//            _FSM = 1;
//        end
//        else begin
//            a <= 3;
//            _FSM = 0;
//        end
//    end
//endcase
//"#);
//}

#[test]
fn rewrite_fsm_while_2() {
    let code = r#"
fsm {
    loop {
        a <= 1;
        yield;
        a <= 2;
        yield;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0: begin
        if (1) begin
            a <= 1;
            _FSM = 1;
        end
        else begin
            _FSM = 0;
        end
    end
    1: begin
        a <= 2;
        _FSM = 0;
    end
endcase
"#);
}

#[test]
fn rewrite_fsm_while_3() {
    let code = r#"
fsm {
    loop {
        a <= 1;
        yield;
        b <= 1;
        yield;
        c <= 1;
        while !result {
            d <= 1;
            yield;
        }
        e <= 1;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1, 3: begin
        if ((_FSM == 1)) begin
            c <= 1;
        end
        if (((_FSM == 1 || _FSM == 3) && !(result))) begin
            d <= 1;
            _FSM = 3;
        end
        else begin
            if ((_FSM == 1 || _FSM == 3)) begin
                e <= 1;
            end
            if (1) begin
                a <= 1;
                _FSM = 2;
            end
            else begin
                _FSM = 0;
            end
        end
    end
    2: begin
        b <= 1;
        _FSM = 1;
    end
endcase
"#);
}

#[test]
fn rewrite_fsm_while_4() {
    let code = r#"
fsm {
    loop {
        a <= 1;

        if dummy > 0 {
            a <= 2;
            yield;
            if status_vector & (1 << 7) {
                LED3 <= 1;
            }
        }

        tx_valid <= 0;
        yield;
    }

    CS <= 1;
    loop {
        yield;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1, 2: begin
        if (((_FSM == 0 || _FSM == 2) && 1)) begin
            if ((_FSM == 0)) begin
                a <= 1;
            end
            if ((_FSM == 2)) begin
                if ((status_vector & (1 << 7))) begin
                    LED3 <= 1;
                end
            end
            if (((_FSM == 0) && (dummy > 0))) begin
                a <= 2;
                _FSM = 2;
            end
            else begin
                tx_valid <= 0;
                _FSM = 0;
            end
        end
        else begin
            if ((_FSM == 0 || _FSM == 2)) begin
                CS <= 1;
                _FSM = 0;
            end
            if (((_FSM == 0 || _FSM == 1) && 1)) begin
                _FSM = 1;
            end
            else begin
                _FSM = 0;
            end
        end
    end
endcase
"#);
}

#[test]
fn rewrite_fsm_while_5() {
    let code = r#"
fsm {
    j <= 1;
    yield;
    loop {
        a <= 1;
        yield;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0: begin
        j <= 1;
        _FSM = 1;
    end
    1: begin
        if (1) begin
            a <= 1;
            _FSM = 1;
        end
        else begin
            _FSM = 0;
        end
    end
endcase
"#);
}


#[test]
fn rewrite_fsm_while_6() {
    let code = r#"
fsm {
    while 1 {
        a <= 1;

        if condition {
            dummy <= 0;
            yield;
            b <= 1;
        }
        c <= 1;
        yield;
    }
    while 2 {
        d2 <= 1;
        yield;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1, 2: begin
        if (((_FSM == 0 || _FSM == 2) && 1)) begin
            if ((_FSM == 0)) begin
                a <= 1;
            end
            if ((_FSM == 2)) begin
                b <= 1;
            end
            if (((_FSM == 0) && condition)) begin
                dummy <= 0;
                _FSM = 2;
            end
            else begin
                c <= 1;
                _FSM = 0;
            end
        end
        else begin
            if (((_FSM == 0 || _FSM == 1) && 2)) begin
                d2 <= 1;
                _FSM = 1;
            end
            else begin
                _FSM = 0;
            end
        end
    end
endcase
"#);
}


#[test]
fn rewrite_fsm_while_7() {
    let code = r#"
fsm {
    loop {
        if condition {
            dummy <= 0;
            yield;
            b <= 1;
        }
        c <= 1;
        yield;
    }
    while 2 {
        d2 <= 1;
        yield;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1, 2: begin
        if (((_FSM == 0 || _FSM == 2) && 1)) begin
            if ((_FSM == 2)) begin
                b <= 1;
            end
            if (((_FSM == 0) && condition)) begin
                dummy <= 0;
                _FSM = 2;
            end
            else begin
                c <= 1;
                _FSM = 0;
            end
        end
        else begin
            if (((_FSM == 0 || _FSM == 1) && 2)) begin
                d2 <= 1;
                _FSM = 1;
            end
            else begin
                _FSM = 0;
            end
        end
    end
endcase
"#);
}


#[test]
fn rewrite_fsm_rewrite() {
    let code = r#"
fsm {
    a <= 1;
    yield;
    a <= 2;
    yield;
    while !result {
        a <= 3;
        yield;
    }
    a <= 4;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0: begin
        a <= 1;
        _FSM = 2;
    end
    2: begin
        a <= 2;
        _FSM = 1;
    end
    1: begin
        if (!(result)) begin
            a <= 3;
            _FSM = 1;
        end
        else begin
            a <= 4;
            _FSM = 0;
        end
    end
endcase
"#);
}

#[test]
fn rewrite_fsm_2() {
    let code = r#"
fsm {
    a <= 1;
    yield;
    a <= 2;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0: begin
        a <= 1;
        _FSM = 1;
    end
    1: begin
        a <= 2;
        _FSM = 0;
    end
endcase
"#);
}

#[test]
fn eth_1() {
    let code = r#"
fsm {
    loop {
        tx_byte <= 1;
        await spi_ready;

        tx_byte <= 2;
        await spi_ready;

        tx_byte <= 3;
        await spi_ready;
    }
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0, 1: begin
        if (((_FSM != 1) || spi_ready)) begin
            if (1) begin
                tx_byte <= 1;
                _FSM = 3;
            end
            else begin
                _FSM = 0;
            end
        end
    end
    3: begin
        if (spi_ready) begin
            tx_byte <= 2;
            _FSM = 2;
        end
    end
    2: begin
        if (spi_ready) begin
            tx_byte <= 3;
            _FSM = 1;
        end
    end
endcase
"#);
}


#[test]
fn double_ids() {
    let code = r#"
fsm {
    yield; while !mini_delay_result { yield; }
    yield; while !spi_ready { yield; }

    while sleep_counter < 360 {
        yield;
    }
    yield;
}
"#;

    let res = parse_results(code, hoodlum::hdl_parser::parse_SeqStatement(code));

    let out = res.to_verilog(&VerilogState::new());

    println!("OK:\n{}", out);

    assert_eq!(out, r#"case (_FSM)
    0: begin
        _FSM = 4;
    end
    4: begin
        if (mini_delay_result) begin
            _FSM = 2;
        end
    end
    2, 3: begin
        if (((_FSM != 2) || spi_ready)) begin
            if (((_FSM == 2 || _FSM == 3) && (sleep_counter < 360))) begin
                _FSM = 3;
            end
            else begin
                _FSM = 1;
            end
        end
    end
    1: begin
        _FSM = 0;
    end
endcase
"#);
}
