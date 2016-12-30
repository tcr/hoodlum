all: rise_ice.bin

rise_ice.bin:
	hoodlum math.hdl test.v
	yosys -q -p "synth_ice40 -top Main -blif rise_ice.blif" test.v
	arachne-pnr -p icestick.pcf rise_ice.blif -o rise_ice.txt
	icebox_explain rise_ice.txt > rise_ice.ex
	icepack rise_ice.txt rise_ice.bin
	icetime -mit -dhx1k rise_ice.txt

explain:
	icebox_vlog -p icestick_vlog.pcf rise_ice.txt > rise_ice.ex2

graph:
	yosys -p 'synth_ice40 -top Main -blif rise_ice.blif; show -colors 67 -format svg -prefix graph' test.v
	rsvg-convert -f pdf -o graph.pdf graph.svg

flash:
	iceprog rise_ice.bin

clean:
	rm -f rise_ice.blif rise_ice.txt rise_ice.ex rise_ice.bin
