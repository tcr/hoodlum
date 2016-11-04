module Second (
    input clk,
    output signal
);
    reg [(24)-1:0] index = 0;
    always @(posedge clk) begin
        if (index == 12000000 - 1) begin
            signal <= 1;
        end
        else begin
            index <= index + 1;
            signal <= 0;
        end
    end
endmodule

module Main (
    input clk,
    output LED1,
    output LED2,
    output LED3,
    output LED4,
    output LED5
);
    reg [(4)-1:0] rot = 0;
    reg [(24)-1:0] divider = 0;
    reg [(24)-1:0] index = 0;
    reg [(5)-1:0] _FSM = 0;
    reg ready;
    Second sec(.clk (clk), .signal (ready));
    always @(posedge clk) begin
        if (ready) begin
            if (divider == 1200000 - 1) begin
                divider <= 0;
                case (_FSM)
                    6, 0: begin
                        rot <= 1;
                        _FSM <= 5;
                    end
                    5: begin
                        rot <= 3;
                        _FSM <= 4;
                    end
                    4: begin
                        rot <= 6;
                        _FSM <= 3;
                    end
                    3: begin
                        rot <= 12;
                        _FSM <= 2;
                    end
                    2: begin
                        rot <= 8;
                        _FSM <= 1;
                    end
                    1: begin
                        rot <= 0;
                        _FSM <= 0;
                    end
                endcase
            end
            else begin
                divider <= divider + 1;
            end
        end
        else begin
            divider <= 0;
        end
    end
    assign LED1 = ready && rot[0];
    assign LED2 = ready && rot[1];
    assign LED3 = ready && rot[2];
    assign LED4 = ready && rot[3];
    assign LED5 = !(ready);
endmodule

