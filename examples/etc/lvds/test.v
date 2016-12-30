module Main (
    input clk,
    output LED1,
    output LED2,
    input PIO3_03,
);
  reg input_0;
  reg input_180;

  // Differential input, DDR data
  SB_IO #(
    .PIN_TYPE(6'b0000_00),
    .IO_STANDARD("SB_LVDS_INPUT")
  ) differential_input (
    .PACKAGE_PIN(PIO3_03),
    .LATCH_INPUT_VALUE ( ),
    .CLOCK_ENABLE ( ),
    .INPUT_CLK (clk),
    .OUTPUT_CLK ( ),
    .OUTPUT_ENABLE ( ),
    .D_OUT_0 ( ),
    .D_OUT_1 ( ),
    .D_IN_0 (input_0),
    .D_IN_1 (input_180)
  );

    always @(*) begin
        LED1 <= input_0;
        LED2 <= input_180;
    end
endmodule
