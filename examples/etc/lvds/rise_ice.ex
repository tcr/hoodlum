Reading file 'rise_ice.txt'..
Fabric size (without IO tiles): 12 x 16

.io_tile 0 14
IoCtrl IE_0
IoCtrl IE_1
IoCtrl LVDS
IoCtrl REN_0
IoCtrl REN_1
buffer io_0/D_IN_0 span12_horz_0
buffer io_0/D_IN_1 span12_horz_2
buffer local_g0_0 io_global/inclk
buffer span4_horz_16 local_g0_0

.io_tile 0 10
IoCtrl IE_0
IoCtrl IE_1
routing span4_vert_b_2 span4_horz_13

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
buffer local_g0_2 io_0/D_OUT_0
buffer local_g0_3 io_1/D_OUT_0
buffer span4_horz_3 local_g0_3
buffer span4_vert_b_10 local_g0_2

.io_tile 13 11
IoCtrl IE_0
IoCtrl IE_1
IoCtrl REN_0
IoCtrl REN_1

.io_tile 6 0
IoCtrl REN_0
IoCtrl REN_1

.io_tile 0 8
IOB_1 PINTYPE_0
IoCtrl IE_1
IoCtrl REN_0
buffer io_1/D_IN_0 span4_vert_b_10

.io_tile 13 14
IoCtrl IE_0
IoCtrl IE_1
routing span4_horz_13 span4_vert_b_2

.logic_tile 12 12
routing sp4_v_t_47 sp4_h_r_3

.logic_tile 11 14
routing sp12_h_l_22 sp12_h_r_1

.logic_tile 12 14
buffer sp12_h_r_2 sp4_h_r_13
routing sp12_h_l_23 sp12_v_t_23

.logic_tile 12 15
buffer sp12_v_b_23 sp4_v_b_23

.ramt_tile 3 10
routing sp4_h_l_37 sp4_v_t_37

.ramt_tile 3 14
routing sp4_v_b_0 sp4_h_l_40

