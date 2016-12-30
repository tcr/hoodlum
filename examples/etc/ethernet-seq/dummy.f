#fn load
#fn store
#fn pop
#fn ifz
#fn ifnz

#fn tx
#fn rx
#fn sleep
#fn cs
#fn assert_eq

#include "consts.f"

#mem NEXT_PACKET_H
#mem NEXT_PACKET_L
#mem MAC_0
#mem MAC_1
#mem MAC_2
#mem MAC_3
#mem MAC_4
#mem MAC_5
#mem RECEIVED_L
#mem RECEIVED_H

#define EWCRU 22

.start:
cs
EWCRU tx
EEUDASTL tx
0x34 tx
0x12 tx

cs
ERCRU tx
EEUDASTL tx
rx 0x34 .start assert_eq
rx 0x12 .start assert_eq

cs
ESSETETHRST tx
360 sleep

// NOTE: don't raise CS high
// magic number
EWCRU tx
EECON2L tx
0x00 tx
0xCB tx

// RX_BUFFER_START 0x5340
cs
EWCRU tx
EERXSTL tx
0x40 tx
0x53 tx

// MAX_FRAMELEN 0x0242
cs
EWCRU tx
EMAMXFLL tx
0x40 tx
0x53 tx

// (magic number)
cs
EWCRU tx
EERXTAILL tx
0xFE tx
0x5F tx

// Read MAC address
cs
ERCRU tx
EMAADR1L tx
rx MAC_0 store
rx MAC_1 store
cs
ERCRU tx
EMAADR2L tx
rx MAC_2 store
rx MAC_3 store
cs
ERCRU tx
EMAADR3L tx
rx MAC_4 store
rx MAC_5 store

cs
ESENABLERX tx

//Memory configuration
//The ENC424j600 has 0x6000 (24kB) bytes of memory
//We have to make good use of it.
// 0x0000
//  [Scratchpad]
// 0x0400
//  [TCP packets (578+42)*TCP_SOCKETS
// 0x1b84 (assuming 10 sockets)
//  [unused area]
// 0x5340 (RX_BUFFER_START (0x6000-RX_BUFFER_SIZE))
//  [RX Buffer]
// 0x6000 (End of standard SRAM)

0x40 NEXT_PACKET_L store
0x53 NEXT_PACKET_H store

.loop:
cs
ESENABLERX tx
cs
(ERCR | EESTATL) tx
rx .loop ifnz

// Configure ERXDATA for reading.
cs
EWCRU tx
EERXRDPTL tx
NEXT_PACKET_L load tx
NEXT_PACKET_H load tx

// Start reading!!!
ERRXDATA tx

// Read next packet pointer.
rx NEXT_PACKET_L store
rx NEXT_PACKET_H store

// Read received byte count.
rx RECEIVED_L store
rx RECEIVED_H store

rx // STATUS_VECTOR store
(1 << 7) and
.next ifz
3 led_on

.next:
rx pop // dummy
.loop goto
