module Main (
    input clk,
    output LED1,
    output LED2,
    output LED3,
    output LED4,
    output LED5
);
    reg [(23)-1:0] index = 0;
    always @(*) begin
        LED1 = 0;
        LED2 = 0;
        LED3 = 0;
        LED4 = 0;
    end
    always @(posedge clk) begin
        if ((index == (6000000 - 1))) begin
            LED5 <= ~(LED5);
            index <= 0;
        end
        else begin
            index <= (index + 1);
        end
    end
endmodule

