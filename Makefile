all:
	clang -target bpf -O2 -g -c xdp/xdp_prober.bpf.c -o xdp/xdp_prober.bpf.o

clean:
	rm -f xdp/xdp_prober.bpf.o
