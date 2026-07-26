CLANG      ?= clang
LLC        ?= llc
BPFTOOL    ?= bpftool

SRC_DIR    := src
OUT_DIR    := out
INCLUDE    := -I .

BPFTARGET  := bpf
CFLAGS     := -target $(BPFTARGET) -O2 -g -Wall
# BTF generation is needed for CO-RE (Compile Once, Run Everywhere)
CFLAGS     += -D__TARGET_ARCH_x86

BPF_SRCS   := $(wildcard $(SRC_DIR)/*/*.bpf.c)
BPF_OBJS   := $(patsubst %.bpf.c,$(OUT_DIR)/%.bpf.o,$(notdir $(BPF_SRCS)))

.PHONY: all clean skel vmlinux

all: $(BPF_OBJS)

$(OUT_DIR):
	mkdir -p $(OUT_DIR)

$(OUT_DIR)/%.bpf.o: $(SRC_DIR)/*/%.bpf.c | $(OUT_DIR)
	$(CLANG) $(CFLAGS) $(INCLUDE) -c $< -o $@

# Generate BPF skeleton header (used by libbpf-rs build.rs for type generation)
# Run this after building the .bpf.o files
skel: $(BPF_OBJS)
	@for obj in $^; do \
		name=$$(basename $$obj .bpf.o); \
		$(BPFTOOL) gen skeleton $$obj > $(OUT_DIR)/$$name.skel.h; \
		echo "Generated skeleton: $(OUT_DIR)/$$name.skel.h"; \
	done

# Dump vmlinux BTF to a C header (needed if not using your distro's vmlinux.h)
# Only needed once per target kernel version
vmlinux:
	$(BPFTOOL) btf dump file /sys/kernel/btf/vmlinux format c > include/vmlinux.h
	@echo "vmlinux.h regenerated"

clean:
	rm -rf $(OUT_DIR)
