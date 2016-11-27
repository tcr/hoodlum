all: rise_ice.bin

rise_ice.bin:
	yosys -q -p "synth_ice40 -top Main -blif rise_ice.blif" test.v
	arachne-pnr --max-passes 2000 -p icestick.pcf rise_ice.blif -o rise_ice.txt
	icebox_explain rise_ice.txt > rise_ice.ex
	icepack rise_ice.txt rise_ice.bin
	icetime -mit -dhx8k rise_ice.txt

graph:
	yosys -o test_syn.v rise_ice.blif

flash:
	iceprog rise_ice.bin

clean:
	rm -f rise_ice.blif rise_ice.txt rise_ice.ex rise_ice.bin
