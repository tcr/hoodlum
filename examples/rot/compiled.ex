Reading file 'compiled.txt'..
Fabric size (without IO tiles): 12 x 16

.io_tile 13 8
IoCtrl IE_0
IoCtrl IE_1
buffer local_g1_0 fabout
buffer span12_horz_8 local_g1_0

.io_tile 6 17
IoCtrl IE_1

.io_tile 13 12
IOB_0 PINTYPE_0
IOB_0 PINTYPE_3
IOB_0 PINTYPE_4
IOB_1 PINTYPE_0
IOB_1 PINTYPE_3
IOB_1 PINTYPE_4
IoCtrl IE_0
IoCtrl IE_1
buffer local_g0_5 io_1/D_OUT_0
buffer local_g1_1 io_0/D_OUT_0
buffer span12_horz_13 local_g0_5
buffer span12_horz_17 local_g1_1

.io_tile 13 11
IOB_0 PINTYPE_0
IOB_0 PINTYPE_3
IOB_0 PINTYPE_4
IOB_1 PINTYPE_0
IOB_1 PINTYPE_3
IOB_1 PINTYPE_4
IoCtrl IE_0
IoCtrl IE_1
IoCtrl REN_0
IoCtrl REN_1
buffer local_g0_3 io_1/D_OUT_0
buffer local_g1_1 io_0/D_OUT_0
buffer span12_horz_9 local_g1_1
buffer span4_horz_43 local_g0_3

.io_tile 6 0
IoCtrl REN_0
IoCtrl REN_1

.io_tile 0 8
IOB_1 PINTYPE_0
IoCtrl IE_1
IoCtrl REN_0
buffer io_1/D_IN_0 span4_vert_b_2
buffer local_g1_2 fabout
buffer span4_vert_b_2 local_g1_2

.io_tile 13 10
IoCtrl IE_0
IoCtrl IE_1
IoCtrl REN_0
IoCtrl REN_1

.io_tile 7 17
IoCtrl IE_0
IoCtrl IE_1
buffer local_g0_1 fabout
buffer span4_vert_25 local_g0_1

.io_tile 13 9
IOB_1 PINTYPE_0
IOB_1 PINTYPE_3
IOB_1 PINTYPE_4
IoCtrl IE_0
IoCtrl IE_1
IoCtrl REN_1
buffer local_g0_3 fabout
buffer local_g0_3 io_1/D_OUT_0
buffer span4_horz_43 local_g0_3

.logic_tile 11 11
LC_5 0100000000000000 0100 DffEnable
buffer glb_netwk_6 lutff_global/clk
buffer glb_netwk_7 lutff_global/cen
buffer local_g0_5 lutff_5/in_0
buffer lutff_5/out sp4_r_v_b_43
buffer sp12_h_r_21 local_g0_5

.logic_tile 7 12
ColBufCtrl glb_netwk_1
ColBufCtrl glb_netwk_2
ColBufCtrl glb_netwk_6
LC_0 0000000000000001 0000
LC_1 0000111000000000 0000
LC_4 1110000000000000 0000
LC_7 0001000000000000 0100 DffEnable
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_0 lutff_7/in_1
buffer local_g0_1 lutff_0/in_1
buffer local_g0_3 lutff_0/in_3
buffer local_g0_6 lutff_1/in_1
buffer local_g0_6 lutff_4/in_0
buffer local_g1_0 lutff_1/in_0
buffer local_g1_0 lutff_4/in_1
buffer local_g1_2 lutff_1/in_2
buffer local_g1_5 lutff_0/in_0
buffer local_g1_6 lutff_7/in_0
buffer local_g2_4 lutff_0/in_2
buffer lutff_0/out local_g0_0
buffer lutff_0/out local_g1_0
buffer lutff_1/out sp4_h_r_2
buffer lutff_4/out sp4_v_b_40
buffer lutff_7/out sp12_v_b_14
buffer lutff_7/out sp4_h_r_30
buffer lutff_7/out sp4_r_v_b_15
buffer neigh_op_bot_1 local_g0_1
buffer neigh_op_top_3 local_g0_3
buffer sp12_h_r_14 local_g0_6
buffer sp12_h_r_14 local_g1_6
buffer sp4_h_r_44 local_g2_4
buffer sp4_v_b_2 local_g1_2
buffer sp4_v_b_5 local_g1_5
routing sp4_h_r_2 sp4_v_b_7
routing sp4_v_b_2 sp4_h_l_42

.logic_tile 7 11
LC_0 0000000000000001 0000
LC_1 0000000000000001 0000
LC_4 1000000000000000 0100 DffEnable
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_2 lutff_global/cen
buffer local_g0_3 lutff_0/in_3
buffer local_g0_4 lutff_4/in_0
buffer local_g1_1 lutff_0/in_0
buffer local_g1_3 lutff_0/in_2
buffer local_g1_4 lutff_0/in_1
buffer local_g1_5 lutff_1/in_1
buffer local_g1_6 lutff_1/in_0
buffer local_g1_7 lutff_1/in_3
buffer local_g2_1 lutff_1/in_2
buffer lutff_0/out sp4_v_b_16
buffer lutff_4/out local_g0_4
buffer lutff_4/out local_g1_4
buffer neigh_op_lft_3 local_g0_3
buffer neigh_op_lft_5 local_g1_5
buffer neigh_op_lft_6 local_g1_6
buffer neigh_op_lft_7 local_g1_7
buffer neigh_op_tnl_1 local_g2_1
buffer sp12_h_r_19 local_g1_3
buffer sp4_h_r_17 local_g1_1
buffer sp4_v_b_18 local_g0_2

.logic_tile 5 8
LC_4 0000111000000000 0100 DffEnable
LC_5 0000111000000000 0100 DffEnable
buffer glb_netwk_2 lutff_global/s_r
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_2 lutff_5/in_1
buffer local_g0_4 lutff_4/in_0
buffer local_g1_2 lutff_4/in_1
buffer local_g1_4 lutff_5/in_0
buffer local_g2_0 lutff_4/in_2
buffer local_g3_2 lutff_5/in_2
buffer lutff_4/out sp4_r_v_b_9
buffer neigh_op_bot_2 local_g0_2
buffer neigh_op_bot_2 local_g1_2
buffer neigh_op_rgt_0 local_g2_0
buffer neigh_op_rgt_2 local_g3_2
buffer sp4_h_r_12 local_g0_4
buffer sp4_h_r_12 local_g1_4

.logic_tile 6 7
LC_0 0110100110010110 1000 CarryEnable
LC_1 0110100110010110 1000 CarryEnable
LC_2 0110100110010110 1000 CarryEnable
LC_3 0110100110010110 1000 CarryEnable
LC_4 0110100110010110 1000 CarryEnable
LC_5 0110100110010110 1000 CarryEnable
LC_6 0110100110010110 1000 CarryEnable
LC_7 0110100110010110 1000 CarryEnable
buffer carry_in carry_in_mux
buffer carry_in_mux lutff_0/in_3
buffer local_g0_3 lutff_1/in_2
buffer local_g0_5 lutff_3/in_2
buffer local_g1_4 lutff_7/in_2
buffer local_g1_6 lutff_5/in_2
buffer local_g1_7 lutff_4/in_2
buffer local_g2_4 lutff_0/in_2
buffer local_g2_6 lutff_6/in_2
buffer local_g3_7 lutff_2/in_2
buffer lutff_0/cout lutff_1/in_3
buffer lutff_1/cout lutff_2/in_3
buffer lutff_2/cout lutff_3/in_3
buffer lutff_3/cout lutff_4/in_3
buffer lutff_4/cout lutff_5/in_3
buffer lutff_5/cout lutff_6/in_3
buffer lutff_6/cout lutff_7/in_3
buffer neigh_op_lft_3 local_g0_3
buffer neigh_op_lft_4 local_g1_4
buffer neigh_op_lft_5 local_g0_5
buffer neigh_op_lft_6 local_g1_6
buffer neigh_op_lft_7 local_g1_7
buffer neigh_op_rgt_4 local_g2_4
buffer neigh_op_rgt_6 local_g2_6
buffer neigh_op_rgt_7 local_g3_7

.logic_tile 5 5
ColBufCtrl glb_netwk_2
ColBufCtrl glb_netwk_6

.logic_tile 7 6
LC_2 0000000000000001 0000
LC_3 0000111000000000 0100 DffEnable
LC_4 0000111000000000 0100 DffEnable
LC_5 0000111000000000 0100 DffEnable
LC_6 0000111000000000 0100 DffEnable
LC_7 0000111000000000 0100 DffEnable
buffer glb_netwk_2 lutff_global/s_r
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_0 lutff_3/in_1
buffer local_g0_0 lutff_5/in_1
buffer local_g0_0 lutff_7/in_1
buffer local_g0_2 lutff_6/in_2
buffer local_g0_3 lutff_3/in_2
buffer local_g0_4 lutff_2/in_2
buffer local_g0_5 lutff_7/in_2
buffer local_g0_6 lutff_4/in_2
buffer local_g0_7 lutff_2/in_3
buffer local_g1_0 lutff_4/in_1
buffer local_g1_0 lutff_6/in_1
buffer local_g1_1 lutff_4/in_0
buffer local_g1_1 lutff_6/in_0
buffer local_g1_2 lutff_3/in_0
buffer local_g1_2 lutff_5/in_0
buffer local_g1_2 lutff_7/in_0
buffer local_g1_4 lutff_5/in_2
buffer local_g1_5 lutff_2/in_0
buffer local_g2_3 lutff_2/in_1
buffer lutff_2/out sp4_v_b_20
buffer lutff_3/out local_g2_3
buffer lutff_4/out local_g0_4
buffer lutff_5/out local_g1_5
buffer lutff_6/out sp12_v_b_12
buffer lutff_7/out local_g0_7
buffer neigh_op_lft_2 local_g0_2
buffer neigh_op_lft_3 local_g0_3
buffer neigh_op_lft_4 local_g1_4
buffer neigh_op_lft_5 local_g0_5
buffer neigh_op_lft_6 local_g0_6
buffer neigh_op_top_2 local_g1_2
buffer sp4_h_r_16 local_g0_0
buffer sp4_h_r_16 local_g1_0
buffer sp4_v_b_17 local_g1_1

.logic_tile 9 11
LC_3 0001000000000000 0000
LC_5 0001000000000000 0000
LC_6 0100000000000000 0100 DffEnable
LC_7 0100000000000000 0100 DffEnable
buffer glb_netwk_6 lutff_global/clk
buffer glb_netwk_7 lutff_global/cen
buffer local_g0_0 lutff_6/in_0
buffer local_g0_3 lutff_3/in_0
buffer local_g0_3 lutff_5/in_0
buffer local_g0_6 lutff_5/in_1
buffer local_g1_4 lutff_7/in_0
buffer local_g1_7 lutff_3/in_1
buffer lutff_3/out sp4_h_r_6
buffer lutff_5/out sp12_h_r_2
buffer lutff_6/out local_g0_6
buffer lutff_7/out local_g1_7
buffer neigh_op_lft_0 local_g0_0
buffer neigh_op_lft_4 local_g1_4
buffer sp4_v_b_19 local_g0_3

.logic_tile 7 5
ColBufCtrl glb_netwk_2
ColBufCtrl glb_netwk_6

.logic_tile 6 5
ColBufCtrl glb_netwk_2
ColBufCtrl glb_netwk_6

.logic_tile 6 8
LC_0 0110100110010110 1000 CarryEnable
LC_1 0110100110010110 1000 CarryEnable
LC_2 0110100110010110 1000 CarryEnable
LC_3 0110100110010110 1000 CarryEnable
LC_4 0110100110010110 0000
LC_5 0000000000000001 0000
LC_6 0000111000000000 0100 DffEnable
LC_7 0000111000000000 0100 DffEnable
buffer carry_in carry_in_mux
buffer carry_in_mux lutff_0/in_3
buffer glb_netwk_2 lutff_global/s_r
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_1 lutff_7/in_2
buffer local_g0_2 lutff_6/in_0
buffer local_g0_4 lutff_6/in_2
buffer local_g0_6 lutff_5/in_1
buffer local_g0_7 lutff_1/in_2
buffer local_g0_7 lutff_5/in_0
buffer local_g1_1 lutff_0/in_2
buffer local_g1_2 lutff_7/in_0
buffer local_g1_5 lutff_2/in_2
buffer local_g1_6 lutff_5/in_2
buffer local_g2_2 lutff_7/in_1
buffer local_g2_6 lutff_4/in_2
buffer local_g3_2 lutff_6/in_1
buffer local_g3_3 lutff_5/in_3
buffer local_g3_6 lutff_3/in_2
buffer lutff_0/cout lutff_1/in_3
buffer lutff_1/cout lutff_2/in_3
buffer lutff_1/out local_g0_1
buffer lutff_2/cout lutff_3/in_3
buffer lutff_3/cout lutff_4/in_3
buffer lutff_4/out local_g0_4
buffer lutff_6/out local_g1_6
buffer lutff_6/out local_g2_6
buffer lutff_7/out local_g0_7
buffer neigh_op_bnl_2 local_g2_2
buffer neigh_op_bnl_2 local_g3_2
buffer neigh_op_bnl_3 local_g3_3
buffer neigh_op_bnr_2 local_g0_2
buffer neigh_op_bnr_2 local_g1_2
buffer neigh_op_bnr_6 local_g0_6
buffer neigh_op_lft_5 local_g1_5
buffer neigh_op_rgt_6 local_g3_6
buffer sp4_v_b_9 local_g1_1
routing sp4_v_b_8 sp4_h_r_8

.logic_tile 8 12
ColBufCtrl glb_netwk_6
ColBufCtrl glb_netwk_7
LC_5 0100000000000000 0100 DffEnable
LC_7 0100000000000000 0100 DffEnable
buffer glb_netwk_6 lutff_global/clk
buffer glb_netwk_7 lutff_global/cen
buffer local_g1_2 lutff_7/in_0
buffer lutff_5/out sp12_v_b_10
buffer lutff_7/out sp12_h_r_6
buffer neigh_op_bot_2 local_g1_2

.logic_tile 7 13
LC_3 0000000000000001 0000
buffer local_g0_2 lutff_3/in_1
buffer local_g0_7 lutff_3/in_2
buffer local_g1_4 lutff_3/in_0
buffer local_g1_5 lutff_3/in_3
buffer neigh_op_lft_2 local_g0_2
buffer neigh_op_lft_4 local_g1_4
buffer neigh_op_lft_5 local_g1_5
buffer neigh_op_lft_7 local_g0_7

.logic_tile 9 9
LC_3 1000000000000000 0000
buffer local_g0_7 lutff_3/in_0
buffer lutff_3/out sp4_h_r_6
buffer sp4_h_r_15 local_g0_7

.logic_tile 6 13
ColBufCtrl glb_netwk_1
ColBufCtrl glb_netwk_6
LC_0 0110100110010110 1100 CarryEnable DffEnable
LC_1 0110100110010110 1100 CarryEnable DffEnable
LC_2 0110100110010110 1100 CarryEnable DffEnable
LC_3 0110100110010110 1100 CarryEnable DffEnable
LC_4 0110100110010110 1100 CarryEnable DffEnable
LC_5 0110100110010110 1100 CarryEnable DffEnable
LC_6 0110100110010110 1100 CarryEnable DffEnable
LC_7 0110100110010110 0100 DffEnable
buffer carry_in carry_in_mux
buffer carry_in_mux lutff_0/in_3
buffer glb_netwk_1 lutff_global/cen
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_0 lutff_0/in_2
buffer local_g0_1 lutff_1/in_2
buffer local_g0_2 lutff_2/in_2
buffer local_g0_3 lutff_3/in_2
buffer local_g0_4 lutff_4/in_2
buffer local_g0_5 lutff_5/in_2
buffer local_g0_6 lutff_6/in_2
buffer local_g0_7 lutff_7/in_2
buffer lutff_0/cout lutff_1/in_3
buffer lutff_0/out local_g0_0
buffer lutff_1/cout lutff_2/in_3
buffer lutff_1/out local_g0_1
buffer lutff_2/cout lutff_3/in_3
buffer lutff_2/out local_g0_2
buffer lutff_3/cout lutff_4/in_3
buffer lutff_3/out local_g0_3
buffer lutff_3/out sp12_v_b_22
buffer lutff_4/cout lutff_5/in_3
buffer lutff_4/out local_g0_4
buffer lutff_5/cout lutff_6/in_3
buffer lutff_5/out local_g0_5
buffer lutff_6/cout lutff_7/in_3
buffer lutff_6/out local_g0_6
buffer lutff_7/out local_g0_7

.logic_tile 7 8
LC_2 0000111000000000 0100 DffEnable
LC_6 0000111000000000 0100 DffEnable
buffer glb_netwk_2 lutff_global/s_r
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_0 lutff_2/in_0
buffer local_g0_0 lutff_6/in_0
buffer local_g0_2 lutff_2/in_2
buffer local_g0_5 lutff_2/in_1
buffer local_g0_5 lutff_6/in_1
buffer local_g1_3 lutff_6/in_2
buffer lutff_2/out sp4_r_v_b_21
buffer lutff_2/out sp4_v_b_4
buffer lutff_6/out sp4_v_b_12
buffer neigh_op_lft_3 local_g1_3
buffer neigh_op_top_2 local_g0_2
buffer sp4_h_r_21 local_g0_5
buffer sp4_r_v_b_24 local_g0_0
routing sp4_v_t_44 sp4_v_b_5

.logic_tile 9 12
ColBufCtrl glb_netwk_6
ColBufCtrl glb_netwk_7
routing sp4_h_l_43 sp4_v_b_6

.logic_tile 6 6
CarryInSet
LC_0 0000000000000000 1000 CarryEnable
LC_1 0000000000000000 1000 CarryEnable
LC_2 0110100110010110 1000 CarryEnable
LC_3 0110100110010110 1000 CarryEnable
LC_4 0110100110010110 1000 CarryEnable
LC_5 0110100110010110 1000 CarryEnable
LC_6 0110100110010110 1000 CarryEnable
LC_7 0110100110010110 1000 CarryEnable
buffer local_g0_1 lutff_7/in_2
buffer local_g0_5 lutff_1/in_2
buffer local_g1_4 lutff_0/in_1
buffer local_g2_3 lutff_3/in_2
buffer local_g2_4 lutff_6/in_2
buffer local_g2_6 lutff_2/in_2
buffer local_g2_7 lutff_5/in_2
buffer local_g3_5 lutff_4/in_2
buffer lutff_1/cout lutff_2/in_3
buffer lutff_2/cout lutff_3/in_3
buffer lutff_3/cout lutff_4/in_3
buffer lutff_4/cout lutff_5/in_3
buffer lutff_5/cout lutff_6/in_3
buffer lutff_6/cout lutff_7/in_3
buffer neigh_op_lft_1 local_g0_1
buffer neigh_op_rgt_3 local_g2_3
buffer neigh_op_rgt_4 local_g2_4
buffer neigh_op_rgt_5 local_g3_5
buffer neigh_op_rgt_6 local_g2_6
buffer neigh_op_rgt_7 local_g2_7
buffer sp4_r_v_b_28 local_g1_4
buffer sp4_r_v_b_29 local_g0_5
routing sp4_v_t_37 sp4_h_r_5

.logic_tile 5 6
LC_1 0000111000000000 0100 DffEnable
buffer glb_netwk_2 lutff_global/s_r
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_2 lutff_1/in_1
buffer local_g0_3 lutff_1/in_0
buffer local_g2_7 lutff_1/in_2
buffer neigh_op_rgt_7 local_g2_7
buffer neigh_op_top_2 local_g0_2
buffer sp4_h_r_19 local_g0_3

.logic_tile 7 7
LC_0 0000000000000010 0000
LC_1 0000000100000000 0000
LC_2 0000000000000001 0000
LC_4 0000111000000000 0100 DffEnable
LC_6 0000111000000000 0100 DffEnable
LC_7 0000111000000000 0100 DffEnable
buffer glb_netwk_2 lutff_global/s_r
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_0 lutff_4/in_2
buffer local_g0_1 lutff_2/in_1
buffer local_g0_2 lutff_4/in_0
buffer local_g0_2 lutff_6/in_0
buffer local_g0_3 lutff_0/in_3
buffer local_g0_4 lutff_7/in_1
buffer local_g0_6 lutff_6/in_2
buffer local_g0_7 lutff_1/in_2
buffer local_g1_0 lutff_4/in_1
buffer local_g1_0 lutff_6/in_1
buffer local_g1_1 lutff_2/in_2
buffer local_g1_2 lutff_7/in_2
buffer local_g1_4 lutff_1/in_0
buffer local_g2_0 lutff_2/in_0
buffer local_g2_2 lutff_1/in_1
buffer local_g2_5 lutff_2/in_3
buffer local_g3_0 lutff_0/in_1
buffer local_g3_1 lutff_0/in_0
buffer local_g3_2 lutff_7/in_0
buffer local_g3_3 lutff_0/in_2
buffer lutff_0/out local_g2_0
buffer lutff_1/out local_g1_1
buffer lutff_2/out local_g0_2
buffer lutff_2/out local_g3_2
buffer lutff_2/out sp12_h_r_12
buffer lutff_2/out sp4_h_r_36
buffer lutff_2/out sp4_r_v_b_21
buffer lutff_2/out sp4_r_v_b_37
buffer lutff_2/out sp4_v_b_4
buffer lutff_4/out local_g1_4
buffer lutff_7/out local_g0_7
buffer neigh_op_lft_0 local_g0_0
buffer neigh_op_lft_2 local_g1_2
buffer neigh_op_lft_6 local_g0_6
buffer neigh_op_tnl_5 local_g2_5
buffer sp12_h_r_16 local_g1_0
buffer sp12_h_r_16 sp4_h_r_20
buffer sp12_v_b_11 local_g3_3
buffer sp12_v_b_16 local_g3_0
buffer sp4_h_r_20 local_g0_4
buffer sp4_h_r_34 local_g2_2
buffer sp4_r_v_b_32 local_g0_3
buffer sp4_v_b_25 local_g3_1
buffer sp4_v_b_9 local_g0_1

.logic_tile 7 15
routing sp4_v_b_5 sp4_v_t_36

.logic_tile 12 9
routing sp4_h_l_39 sp4_v_t_39

.logic_tile 5 12
LC_1 0001000000000000 0000
LC_2 0000000000000001 0000
LC_4 0000000010000000 0000
LC_6 1000000000000000 0000
buffer local_g0_6 lutff_1/in_1
buffer local_g1_4 lutff_1/in_0
buffer local_g1_6 lutff_6/in_3
buffer local_g2_0 lutff_4/in_0
buffer local_g2_2 lutff_4/in_2
buffer local_g2_3 lutff_2/in_1
buffer local_g2_4 lutff_2/in_0
buffer local_g2_5 lutff_4/in_1
buffer local_g2_6 lutff_6/in_2
buffer local_g2_7 lutff_4/in_3
buffer local_g3_0 lutff_2/in_3
buffer local_g3_1 lutff_2/in_2
buffer local_g3_6 lutff_6/in_1
buffer local_g3_7 lutff_6/in_0
buffer lutff_1/out sp12_h_r_10
buffer lutff_2/out sp4_h_r_20
buffer lutff_4/out local_g1_4
buffer lutff_6/out local_g0_6
buffer neigh_op_rgt_0 local_g2_0
buffer neigh_op_rgt_2 local_g2_2
buffer neigh_op_rgt_3 local_g2_3
buffer neigh_op_rgt_4 local_g2_4
buffer neigh_op_rgt_5 local_g2_5
buffer neigh_op_rgt_6 local_g3_6
buffer neigh_op_rgt_7 local_g3_7
buffer neigh_op_tnr_0 local_g3_0
buffer neigh_op_tnr_1 local_g3_1
buffer neigh_op_tnr_6 local_g2_6
buffer sp12_h_r_22 local_g1_6
buffer sp4_h_r_31 local_g2_7

.logic_tile 7 10
LC_5 0110100110010110 0100 DffEnable
buffer glb_netwk_1 lutff_global/cen
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_5 lutff_5/in_2
buffer local_g1_3 lutff_5/in_1
buffer lutff_5/out local_g0_5
buffer lutff_5/out sp4_v_b_26
buffer neigh_op_bot_3 local_g1_3

.logic_tile 8 6
routing sp4_v_t_37 sp4_h_l_43

.logic_tile 8 9
routing sp4_v_t_39 sp4_h_r_2

.logic_tile 8 11
LC_0 0000100000000000 0000
LC_1 0000000011100000 0000
LC_2 1000000000001000 0000
LC_3 0000100000000000 0000
LC_4 0101001100000000 0000
LC_5 0100000000000000 0100 DffEnable
LC_6 1110000000000000 0100 DffEnable
LC_7 0100000000000000 0100 DffEnable
buffer glb_netwk_6 lutff_global/clk
buffer glb_netwk_7 lutff_global/cen
buffer local_g0_5 lutff_3/in_2
buffer local_g0_5 lutff_6/in_1
buffer local_g0_6 lutff_4/in_2
buffer local_g0_7 lutff_1/in_0
buffer local_g0_7 lutff_2/in_1
buffer local_g1_0 lutff_4/in_1
buffer local_g1_2 lutff_5/in_0
buffer local_g1_3 lutff_4/in_0
buffer local_g1_4 lutff_7/in_0
buffer local_g1_5 lutff_0/in_0
buffer local_g1_5 lutff_1/in_1
buffer local_g1_5 lutff_2/in_2
buffer local_g1_6 lutff_2/in_3
buffer local_g1_6 lutff_6/in_3
buffer local_g1_7 lutff_0/in_2
buffer local_g1_7 lutff_3/in_1
buffer local_g1_7 lutff_6/in_0
buffer local_g2_5 lutff_0/in_1
buffer local_g2_5 lutff_1/in_2
buffer local_g2_5 lutff_3/in_0
buffer local_g3_5 lutff_2/in_0
buffer local_g3_5 lutff_6/in_2
buffer local_g3_7 lutff_1/in_3
buffer lutff_0/out local_g1_0
buffer lutff_1/out sp4_r_v_b_3
buffer lutff_2/out local_g1_2
buffer lutff_3/out local_g1_3
buffer lutff_3/out sp12_h_r_14
buffer lutff_4/out local_g1_4
buffer lutff_5/out local_g0_5
buffer lutff_5/out local_g1_5
buffer lutff_6/out local_g0_6
buffer lutff_6/out local_g1_6
buffer lutff_7/out local_g0_7
buffer lutff_7/out local_g1_7
buffer neigh_op_tnl_7 local_g3_7
buffer sp12_v_b_13 local_g2_5
buffer sp12_v_b_13 local_g3_5

.logic_tile 6 11
CarryInSet
LC_0 0000000000000000 1000 CarryEnable
LC_1 0000000000000000 1000 CarryEnable
LC_2 0110100110010110 1100 CarryEnable DffEnable
LC_3 0110100110010110 1100 CarryEnable DffEnable
LC_4 0110100110010110 1100 CarryEnable DffEnable
LC_5 0110100110010110 1100 CarryEnable DffEnable
LC_6 0110100110010110 1100 CarryEnable DffEnable
LC_7 0110100110010110 1100 CarryEnable DffEnable
buffer glb_netwk_1 lutff_global/cen
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_2 lutff_2/in_2
buffer local_g0_3 lutff_3/in_2
buffer local_g0_4 lutff_4/in_2
buffer local_g0_5 lutff_0/in_1
buffer local_g0_6 lutff_6/in_2
buffer local_g0_7 lutff_7/in_2
buffer local_g2_5 lutff_5/in_2
buffer local_g3_4 lutff_1/in_2
buffer lutff_1/cout lutff_2/in_3
buffer lutff_2/cout lutff_3/in_3
buffer lutff_2/out local_g0_2
buffer lutff_2/out sp4_h_r_4
buffer lutff_3/cout lutff_4/in_3
buffer lutff_3/out local_g0_3
buffer lutff_4/cout lutff_5/in_3
buffer lutff_4/out local_g0_4
buffer lutff_4/out sp12_h_r_16
buffer lutff_5/cout lutff_6/in_3
buffer lutff_5/out local_g2_5
buffer lutff_6/cout lutff_7/in_3
buffer lutff_6/out local_g0_6
buffer lutff_7/out local_g0_7
buffer neigh_op_bnr_5 local_g0_5
buffer neigh_op_rgt_4 local_g3_4

.logic_tile 6 12
ColBufCtrl glb_netwk_1
ColBufCtrl glb_netwk_6
LC_0 0110100110010110 1100 CarryEnable DffEnable
LC_1 0110100110010110 1100 CarryEnable DffEnable
LC_2 0110100110010110 1100 CarryEnable DffEnable
LC_3 0110100110010110 1100 CarryEnable DffEnable
LC_4 0110100110010110 1100 CarryEnable DffEnable
LC_5 0110100110010110 1100 CarryEnable DffEnable
LC_6 0110100110010110 1100 CarryEnable DffEnable
LC_7 0110100110010110 1100 CarryEnable DffEnable
buffer carry_in carry_in_mux
buffer carry_in_mux lutff_0/in_3
buffer glb_netwk_1 lutff_global/cen
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_0 lutff_0/in_2
buffer local_g0_1 lutff_1/in_2
buffer local_g0_2 lutff_2/in_2
buffer local_g0_3 lutff_3/in_2
buffer local_g0_4 lutff_4/in_2
buffer local_g0_5 lutff_5/in_2
buffer local_g0_6 lutff_6/in_2
buffer local_g0_7 lutff_7/in_2
buffer lutff_0/cout lutff_1/in_3
buffer lutff_0/out local_g0_0
buffer lutff_1/cout lutff_2/in_3
buffer lutff_1/out local_g0_1
buffer lutff_2/cout lutff_3/in_3
buffer lutff_2/out local_g0_2
buffer lutff_3/cout lutff_4/in_3
buffer lutff_3/out local_g0_3
buffer lutff_4/cout lutff_5/in_3
buffer lutff_4/out local_g0_4
buffer lutff_5/cout lutff_6/in_3
buffer lutff_5/out local_g0_5
buffer lutff_6/cout lutff_7/in_3
buffer lutff_6/out local_g0_6
buffer lutff_7/out local_g0_7
routing sp12_v_t_22 sp12_h_l_22

.logic_tile 11 12
ColBufCtrl glb_netwk_6
ColBufCtrl glb_netwk_7
LC_1 0001000000000000 0000
LC_3 0001000000000000 0000
buffer local_g0_6 lutff_3/in_1
buffer local_g1_5 lutff_1/in_1
buffer local_g2_7 lutff_1/in_0
buffer local_g2_7 lutff_3/in_0
buffer lutff_1/out sp12_h_r_10
buffer lutff_3/out sp12_h_r_14
buffer sp12_h_r_13 local_g1_5
buffer sp4_r_v_b_15 local_g2_7
buffer sp4_r_v_b_30 local_g0_6

.logic_tile 7 9
LC_2 0110100110010110 0000
LC_3 1000000000000000 0000
LC_4 1101000000000000 0000
LC_6 1000000000000000 0100 DffEnable
buffer glb_netwk_2 lutff_global/s_r
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_2 lutff_global/cen
buffer local_g0_3 lutff_2/in_1
buffer local_g0_6 lutff_6/in_0
buffer local_g2_0 lutff_2/in_2
buffer local_g2_0 lutff_4/in_0
buffer local_g2_5 lutff_4/in_1
buffer lutff_3/out local_g0_3
buffer lutff_4/out sp4_v_b_8
buffer lutff_6/out local_g0_6
buffer lutff_6/out sp12_v_b_12
buffer lutff_6/out sp4_v_b_44
buffer sp12_v_b_21 local_g2_5
buffer sp4_h_r_2 local_g0_2
buffer sp4_r_v_b_8 local_g2_0
routing sp4_v_b_8 sp4_h_r_2

.logic_tile 5 7
LC_0 1000000000000000 0000
LC_2 0000100000000000 0000
LC_3 0000111000000000 0100 DffEnable
LC_4 0000111000000000 0100 DffEnable
LC_5 0000111000000000 0100 DffEnable
LC_6 0000111000000000 0100 DffEnable
LC_7 0000111000000000 0100 DffEnable
buffer glb_netwk_2 lutff_global/s_r
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_0 lutff_2/in_2
buffer local_g0_1 lutff_2/in_1
buffer local_g0_2 lutff_3/in_1
buffer local_g0_2 lutff_5/in_1
buffer local_g0_2 lutff_7/in_1
buffer local_g0_4 lutff_4/in_0
buffer local_g0_4 lutff_6/in_0
buffer local_g1_0 lutff_3/in_0
buffer local_g1_0 lutff_5/in_0
buffer local_g1_0 lutff_7/in_0
buffer local_g1_2 lutff_4/in_1
buffer local_g1_2 lutff_6/in_1
buffer local_g1_4 lutff_0/in_3
buffer local_g1_5 lutff_0/in_2
buffer local_g1_6 lutff_0/in_1
buffer local_g1_7 lutff_2/in_0
buffer local_g2_1 lutff_3/in_2
buffer local_g2_3 lutff_5/in_2
buffer local_g2_4 lutff_0/in_0
buffer local_g3_4 lutff_7/in_2
buffer local_g3_5 lutff_6/in_2
buffer local_g3_7 lutff_4/in_2
buffer lutff_0/out local_g0_0
buffer lutff_2/out local_g0_2
buffer lutff_2/out local_g1_2
buffer lutff_2/out sp12_h_r_12
buffer lutff_2/out sp4_r_v_b_21
buffer lutff_2/out sp4_r_v_b_37
buffer lutff_4/out local_g2_4
buffer lutff_5/out sp4_h_r_10
buffer lutff_6/out local_g1_6
buffer lutff_7/out local_g1_7
buffer neigh_op_bot_1 local_g0_1
buffer neigh_op_rgt_1 local_g2_1
buffer neigh_op_rgt_3 local_g2_3
buffer neigh_op_rgt_4 local_g3_4
buffer neigh_op_rgt_5 local_g3_5
buffer neigh_op_rgt_7 local_g3_7
buffer neigh_op_top_4 local_g1_4
buffer neigh_op_top_5 local_g1_5
buffer sp12_h_r_8 local_g1_0
buffer sp4_h_r_12 local_g0_4

.logic_tile 8 8
LC_4 0000000100000000 0000
buffer local_g2_0 lutff_4/in_0
buffer local_g2_6 lutff_4/in_2
buffer local_g3_2 lutff_4/in_1
buffer lutff_4/out sp12_h_r_0
buffer neigh_op_bnl_2 local_g3_2
buffer sp4_h_r_32 local_g2_0
buffer sp4_r_v_b_38 local_g2_6
routing sp4_v_b_8 sp4_h_l_36

