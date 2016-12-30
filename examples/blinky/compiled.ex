Reading file 'compiled.txt'..
Fabric size (without IO tiles): 12 x 16

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

.io_tile 6 0
IoCtrl REN_0
IoCtrl REN_1

.io_tile 13 6
IoCtrl IE_0
IoCtrl IE_1
routing span4_horz_37 span4_vert_t_14

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

.io_tile 13 9
IOB_1 PINTYPE_0
IOB_1 PINTYPE_3
IOB_1 PINTYPE_4
IoCtrl IE_0
IoCtrl IE_1
IoCtrl REN_1
buffer local_g0_5 io_1/D_OUT_0
buffer local_g1_6 fabout
buffer span4_horz_37 local_g0_5
buffer span4_vert_b_6 local_g1_6

.logic_tile 6 9
LC_1 1000000000000000 0000
LC_5 0110100110010110 0100 DffEnable
buffer glb_netwk_2 lutff_global/s_r
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_5 lutff_5/in_2
buffer local_g1_1 lutff_5/in_1
buffer lutff_1/out local_g1_1
buffer lutff_5/out local_g0_5
buffer lutff_5/out sp4_h_r_10
buffer lutff_5/out sp4_h_r_26

.logic_tile 7 12
ColBufCtrl glb_netwk_2
ColBufCtrl glb_netwk_6

.logic_tile 8 10
LC_1 0000000100000000 0000
LC_3 1000000000000000 0000
LC_4 0000000010000000 0000
LC_6 0000000000000001 0000
LC_7 0000000000000001 0000
buffer local_g0_1 lutff_7/in_2
buffer local_g0_3 lutff_3/in_0
buffer local_g0_4 lutff_3/in_3
buffer local_g0_5 lutff_3/in_2
buffer local_g0_6 lutff_7/in_1
buffer local_g0_7 lutff_6/in_1
buffer local_g1_1 lutff_6/in_0
buffer local_g1_2 lutff_6/in_3
buffer local_g1_3 lutff_6/in_2
buffer local_g1_4 lutff_1/in_2
buffer local_g1_5 lutff_7/in_3
buffer local_g1_6 lutff_1/in_0
buffer local_g2_1 lutff_4/in_1
buffer local_g2_2 lutff_4/in_0
buffer local_g2_7 lutff_4/in_3
buffer local_g3_3 lutff_1/in_1
buffer local_g3_5 lutff_4/in_2
buffer local_g3_6 lutff_7/in_0
buffer local_g3_7 lutff_3/in_1
buffer lutff_1/out sp4_r_v_b_3
buffer lutff_3/out local_g3_3
buffer lutff_4/out local_g1_4
buffer lutff_6/out local_g1_6
buffer lutff_7/out local_g0_7
buffer neigh_op_bnl_6 local_g3_6
buffer neigh_op_bnl_7 local_g3_7
buffer neigh_op_bot_1 local_g1_1
buffer neigh_op_lft_4 local_g0_4
buffer neigh_op_lft_5 local_g0_5
buffer neigh_op_tnl_2 local_g2_2
buffer neigh_op_tnl_5 local_g3_5
buffer neigh_op_top_2 local_g1_2
buffer neigh_op_top_3 local_g1_3
buffer sp4_h_r_13 local_g1_5
buffer sp4_h_r_17 local_g0_1
buffer sp4_h_r_41 local_g2_1
buffer sp4_v_b_14 local_g0_6
buffer sp4_v_b_3 local_g0_3
buffer sp4_v_b_39 local_g2_7

.logic_tile 7 11
LC_0 0110100110010110 1100 CarryEnable DffEnable
LC_1 0110100110010110 1100 CarryEnable DffEnable
LC_2 0110100110010110 1100 CarryEnable DffEnable
LC_3 0110100110010110 1100 CarryEnable DffEnable
LC_4 0110100110010110 1100 CarryEnable DffEnable
LC_5 0110100110010110 1100 CarryEnable DffEnable
LC_6 0110100110010110 0100 DffEnable
buffer carry_in carry_in_mux
buffer carry_in_mux lutff_0/in_3
buffer glb_netwk_2 lutff_global/s_r
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_0 lutff_0/in_2
buffer local_g0_1 lutff_1/in_2
buffer local_g0_2 lutff_2/in_2
buffer local_g0_3 lutff_3/in_2
buffer local_g0_4 lutff_4/in_2
buffer local_g0_5 lutff_5/in_2
buffer local_g0_6 lutff_6/in_2
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
buffer lutff_6/out local_g0_6

.logic_tile 9 6
routing sp4_v_b_11 sp4_v_t_42
routing sp4_v_t_38 sp4_v_b_11
routing sp4_v_t_42 sp4_h_r_0

.logic_tile 8 12
ColBufCtrl glb_netwk_2
ColBufCtrl glb_netwk_6

.logic_tile 9 9
LC_0 1000000000000000 0100 DffEnable
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_0 lutff_0/in_0
buffer local_g0_2 lutff_global/cen
buffer lutff_0/out local_g0_0
buffer lutff_0/out sp4_h_r_0
buffer sp4_v_b_18 local_g0_2

.logic_tile 9 12
ColBufCtrl glb_netwk_6

.logic_tile 7 10
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
buffer glb_netwk_2 lutff_global/s_r
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
buffer lutff_0/out sp4_h_r_0
buffer lutff_1/cout lutff_2/in_3
buffer lutff_1/out local_g0_1
buffer lutff_1/out sp4_r_v_b_3
buffer lutff_2/cout lutff_3/in_3
buffer lutff_2/out local_g0_2
buffer lutff_2/out sp4_h_r_4
buffer lutff_3/cout lutff_4/in_3
buffer lutff_3/out local_g0_3
buffer lutff_4/cout lutff_5/in_3
buffer lutff_4/out local_g0_4
buffer lutff_5/cout lutff_6/in_3
buffer lutff_5/out local_g0_5
buffer lutff_6/cout lutff_7/in_3
buffer lutff_6/out local_g0_6
buffer lutff_6/out sp4_h_r_28
buffer lutff_7/out local_g0_7

.logic_tile 8 9
LC_1 0000000000000001 0000
LC_4 1000000000000000 0100 DffEnable
buffer glb_netwk_2 lutff_global/s_r
buffer glb_netwk_6 lutff_global/clk
buffer local_g0_4 lutff_1/in_1
buffer local_g0_4 lutff_4/in_0
buffer local_g1_2 lutff_1/in_0
buffer local_g1_3 lutff_1/in_3
buffer local_g1_4 lutff_1/in_2
buffer local_g2_2 lutff_global/cen
buffer lutff_4/out local_g0_4
buffer neigh_op_lft_2 local_g1_2
buffer neigh_op_lft_3 local_g1_3
buffer neigh_op_lft_4 local_g1_4
buffer sp4_h_r_34 local_g2_2
routing sp4_h_l_39 sp4_v_t_39

.logic_tile 8 11
LC_2 0000000000000001 0000
LC_3 0000000100000000 0000
buffer local_g0_3 lutff_2/in_1
buffer local_g0_4 lutff_2/in_0
buffer local_g0_6 lutff_2/in_2
buffer local_g1_0 lutff_3/in_2
buffer local_g1_1 lutff_3/in_1
buffer local_g2_3 lutff_3/in_0
buffer local_g2_7 lutff_2/in_3
buffer neigh_op_bnl_3 local_g2_3
buffer neigh_op_bnl_7 local_g2_7
buffer neigh_op_lft_0 local_g1_0
buffer neigh_op_lft_1 local_g1_1
buffer neigh_op_lft_3 local_g0_3
buffer neigh_op_lft_4 local_g0_4
buffer neigh_op_lft_6 local_g0_6

.logic_tile 6 12
ColBufCtrl glb_netwk_2
ColBufCtrl glb_netwk_6

.logic_tile 7 9
CarryInSet
LC_0 0000000000000000 1000 CarryEnable
LC_1 0000000000000000 1000 CarryEnable
LC_2 0110100110010110 1100 CarryEnable DffEnable
LC_3 0110100110010110 1100 CarryEnable DffEnable
LC_4 0110100110010110 1100 CarryEnable DffEnable
LC_5 0110100110010110 1100 CarryEnable DffEnable
LC_6 0110100110010110 1100 CarryEnable DffEnable
LC_7 0110100110010110 1100 CarryEnable DffEnable
buffer glb_netwk_2 lutff_global/s_r
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
buffer lutff_3/cout lutff_4/in_3
buffer lutff_3/out local_g0_3
buffer lutff_4/cout lutff_5/in_3
buffer lutff_4/out local_g0_4
buffer lutff_5/cout lutff_6/in_3
buffer lutff_5/out local_g2_5
buffer lutff_5/out sp4_r_v_b_27
buffer lutff_6/cout lutff_7/in_3
buffer lutff_6/out local_g0_6
buffer lutff_7/out local_g0_7
buffer neigh_op_lft_5 local_g0_5
buffer neigh_op_rgt_4 local_g3_4

