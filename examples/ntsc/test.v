module Second (
    input clk,
    output value
);
    reg [(26)-1:0] index = 0;
    always @(posedge clk) begin
        if ((index == (25000000 - 1))) begin
            index <= 0;
            value <= ~(value);
        end
        else begin
            index <= (index + 1);
        end
    end
endmodule

module Ntsc (
    input clk,
    input [(3)-1:0] pixel_data,
    output h_sync_out,
    output v_sync_out,
    output [(10)-1:0] pixel_y,
    output [(10)-1:0] pixel_x,
    output pixel_is_visible,
    output [(3)-1:0] ntsc_out
);
    localparam BASE_PIXEL_X = 184;
    localparam RESOLUTION_HORIZONTAL = 560;
    localparam BASE_PIXEL_Y = 89;
    localparam RESOLUTION_VERTICAL = 400;
    reg [(3)-1:0] luminance;
    reg [(10)-1:0] line_count_reg = 0;
    reg [(10)-1:0] line_count_reg_next;
    reg [(2)-1:0] line_type_reg = 0;
    reg [(2)-1:0] line_type_reg_next;
    reg [(12)-1:0] horizontal_count_reg = 0;
    reg [(12)-1:0] horizontal_count_reg_next;
    localparam LINE_TYPE_EQ = 0;
    localparam LINE_TYPE_VERTICAL_BLANK = 1;
    localparam LINE_TYPE_SCANLINE = 2;
    localparam WIDTH_FRONT_PORCH = 75;
    localparam WIDTH_SYNC_TIP = 235;
    localparam WIDTH_BACK_PORCH = 235;
    localparam WIDTH_VIDEO = 2630;
    localparam WIDTH_WHOLE_LINE = 3175;
    localparam WIDTH_HALF_LINE = 1588;
    localparam WIDTH_EQ_PULSE = 117;
    localparam WIDTH_V_SYNC_PULSE = 1353;
    localparam SIGNAL_LEVEL_SYNC = 0;
    localparam SIGNAL_LEVEL_BLANK = 1;
    localparam SIGNAL_LEVEL_BLACK = 2;
    localparam SIGNAL_LEVEL_DARK_GREY = 3;
    localparam SIGNAL_LEVEL_GREY = 4;
    localparam SIGNAL_LEVEL_LIGHT_GREY = 5;
    localparam SIGNAL_LEVEL_WHITE = 6;
    localparam SIGNAL_LEVEL_BRIGHT_WHITE = 7;
    localparam HALF_LINE_EVEN_FIELD = 18;
    localparam HALF_LINE_ODD_FIELD = 527;
    reg [(1)-1:0] at_half_line_width;
    always @(*) at_half_line_width = (horizontal_count_reg >= WIDTH_HALF_LINE);
    reg [(1)-1:0] at_full_line_width;
    always @(*) at_full_line_width = (horizontal_count_reg >= WIDTH_WHOLE_LINE);
    reg [(1)-1:0] is_a_half_line;
    always @(*) is_a_half_line = ((line_count_reg == HALF_LINE_EVEN_FIELD) | (line_count_reg == HALF_LINE_ODD_FIELD));
    reg [(1)-1:0] is_a_whole_line;
    always @(*) is_a_whole_line = ~(is_a_half_line);
    reg [(1)-1:0] h_sync;
    always @(*) h_sync = ((is_a_half_line & at_half_line_width) | (is_a_whole_line & at_full_line_width));
    reg [(1)-1:0] v_sync;
    always @(*) v_sync = (h_sync & (line_count_reg >= 526));
    reg [(1)-1:0] h_sync_out;
    always @(*) h_sync_out = h_sync;
    reg [(1)-1:0] v_sync_out;
    always @(*) v_sync_out = v_sync;
    reg [(1)-1:0] pixel_is_visible;
    always @(*) pixel_is_visible = ((((horizontal_count_reg[(12)-1:2] >= BASE_PIXEL_X) & (horizontal_count_reg[(12)-1:2] < (BASE_PIXEL_X + RESOLUTION_HORIZONTAL))) & (line_count_reg >= BASE_PIXEL_Y)) & (line_count_reg < (BASE_PIXEL_Y + RESOLUTION_VERTICAL)));
    always @(*) begin
        pixel_x = (pixel_is_visible ? (horizontal_count_reg[(12)-1:2] - BASE_PIXEL_X) : 0);
        pixel_y = (pixel_is_visible ? (line_count_reg - BASE_PIXEL_Y) : 0);
    end
    always @(posedge clk) begin
        horizontal_count_reg <= horizontal_count_reg_next;
        line_count_reg <= line_count_reg_next;
        line_type_reg <= line_type_reg_next;
    end
    always @(*) begin
        if (((line_count_reg <= 5) || ((line_count_reg >= 12) && (line_count_reg <= 18)))) begin
            line_type_reg_next = LINE_TYPE_EQ;
        end
        else if (((line_count_reg >= 6) && (line_count_reg <= 11))) begin
            line_type_reg_next = LINE_TYPE_VERTICAL_BLANK;
        end
        else begin
            line_type_reg_next = LINE_TYPE_SCANLINE;
        end
    end
    always @(*) begin
        if (h_sync) begin
            horizontal_count_reg_next = 0;
        end
        else begin
            horizontal_count_reg_next = (horizontal_count_reg + 1);
        end
    end
    always @(*) begin
        if ((line_type_reg == LINE_TYPE_EQ)) begin
            if (((horizontal_count_reg < WIDTH_EQ_PULSE) || ((horizontal_count_reg > WIDTH_HALF_LINE) && (horizontal_count_reg < (WIDTH_HALF_LINE + WIDTH_EQ_PULSE))))) begin
                ntsc_out = SIGNAL_LEVEL_SYNC;
            end
            else begin
                ntsc_out = SIGNAL_LEVEL_BLANK;
            end
        end
        else if ((line_type_reg == LINE_TYPE_VERTICAL_BLANK)) begin
            if (((horizontal_count_reg < WIDTH_V_SYNC_PULSE) || ((horizontal_count_reg > WIDTH_HALF_LINE) && (horizontal_count_reg < (WIDTH_HALF_LINE + WIDTH_V_SYNC_PULSE))))) begin
                ntsc_out = SIGNAL_LEVEL_SYNC;
            end
            else begin
                ntsc_out = SIGNAL_LEVEL_BLANK;
            end
        end
        else if ((line_type_reg == LINE_TYPE_SCANLINE)) begin
            if (((horizontal_count_reg > WIDTH_FRONT_PORCH) && (horizontal_count_reg < (WIDTH_FRONT_PORCH + WIDTH_SYNC_TIP)))) begin
                ntsc_out = SIGNAL_LEVEL_SYNC;
            end
            else if ((horizontal_count_reg > (WIDTH_WHOLE_LINE - WIDTH_VIDEO))) begin
                ntsc_out = luminance;
            end
            else begin
                ntsc_out = SIGNAL_LEVEL_BLANK;
            end
        end
    end
    always @(*) begin
        if ((((v_sync == 1) && (h_sync == 1)) && (line_count_reg == 526))) begin
            line_count_reg_next = 0;
        end
        else if ((((v_sync == 1) && (h_sync == 1)) && (line_count_reg == 527))) begin
            line_count_reg_next = 0;
        end
        else if (((v_sync == 0) && (h_sync == 1))) begin
            line_count_reg_next = (line_count_reg + 2);
        end
        else begin
            line_count_reg_next = line_count_reg;
        end
    end
    always @(*) begin
        if (pixel_is_visible) begin
            case (pixel_data)
                0: begin
                    luminance = SIGNAL_LEVEL_BLANK;
                end
                1: begin
                    luminance = SIGNAL_LEVEL_DARK_GREY;
                end
                2: begin
                    luminance = SIGNAL_LEVEL_GREY;
                end
                3: begin
                    luminance = SIGNAL_LEVEL_LIGHT_GREY;
                end
                4: begin
                    luminance = SIGNAL_LEVEL_WHITE;
                end
                5: begin
                    luminance = SIGNAL_LEVEL_BRIGHT_WHITE;
                end
                default: begin
                    luminance = SIGNAL_LEVEL_BLANK;
                end
            endcase
        end
        else begin
            luminance = SIGNAL_LEVEL_BLANK;
        end
    end
endmodule

module Demo (
  input clk,
  input [14:0] raddr0,
  output reg [1:0] rdata0,
);
  reg [1:0] memory [0:32767];
  initial $readmemh("zelda.hex", memory);
  always @(posedge clk) rdata0 <= memory[raddr0];
endmodule

module Main (
    input clk,
    output LED1,
    output LED2,
    output LED3,
    output LED4,
    output LED5,
    input PMOD1,
    output PMOD2,
    output PMOD3,
    output PMOD4
);
    reg [(1)-1:0] oob;
    reg [(15)-1:0] pos;
    always @(*) pos = ((pixel_y << 7) + ((pixel_x >> 1) - 128));
    reg [(2)-1:0] rdata0;
    reg [(3)-1:0] pixel;
    always @(*) case (rdata0)
      0: pixel = 1;
      1: pixel = 2;
      2: pixel = 3;
      3: pixel = 4;
      endcase
    Demo demo(.clk (PMOD1),
        .raddr0 (pos),
        .rdata0 (rdata0));
    reg [(10)-1:0] pixel_x;
    reg [(10)-1:0] pixel_y;
    Ntsc ntsc(.clk (PMOD1),
        .pixel_data (pixel),
        .h_sync_out (),
        .v_sync_out (),
        .pixel_y (pixel_y),
        .pixel_x (pixel_x),
        .pixel_is_visible (),
        .ntsc_out ({PMOD4, PMOD3, PMOD2}));
    Second s(.clk (PMOD1),
        .value (LED5));
endmodule
