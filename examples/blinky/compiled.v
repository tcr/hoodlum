module Main (
    input clk,
    output LED1,
    output LED2,
    output LED3,
    output LED4,
    output LED5
);
    reg [(23)-1:0] index = 0;
    always @(posedge clk) begin
        if ((index == (6000000 - 1))) begin
            LED5 <= ~(LED5);
            index <= 0;
        end
        else begin
            index <= (index + 1);
        end
    end
    always @(*) begin
        LED1 = 0;
    end
    always @(*) begin
        LED2 = 0;
    end
    always @(*) begin
        LED3 = 0;
    end
    always @(*) begin
        LED4 = 0;
    end
endmodule

