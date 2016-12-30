all: compiled.bin

compiled.bin: ntsc.hdl
	hoodlum $^ -o compiled.v
	yosys -q -p "synth_ice40 -abc2 -top Main -blif compiled.blif" compiled.v
	arachne-pnr -p icestick.pcf compiled.blif -o compiled.txt
	icebox_explain compiled.txt > compiled.ex
	icepack compiled.txt compiled.bin
	icetime -mit -dhx1k compiled.txt

graph:
	yosys -o test_syn.v compiled.blif

flash:
	iceprog compiled.bin

clean:
	rm -f compiled.blif compiled.txt compiled.ex compiled.bin compiled.v
