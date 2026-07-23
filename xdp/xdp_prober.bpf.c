// refrence : https://github.com/xdp-project/xdp-tutorial
//
#include "vmlinux.h" // contains all kernel data structures from BTF
    // bpftool btf dump file /sys/kernel/btf/vmlinux format c > vmlinux.h

#include <bpf/bpf_endian.h>
#include <bpf/bpf_helpers.h>

// event structure this is passed to user space via ring buffer
// for real-time packet inspection
// ring buffers are more efficient thatn perf events for kernel to userspace
struct pkt_event {
  __u32 srcIp;
  __u32 dstIp;
  __u16 srcPort;
  __u16 dstPort;
  __u8 protocol;
  __u32 pkt_len;
};

// cpu has multiple cores each core handles packet simultaneously,
// this is used to agregate of all the counts
struct pkt_stats {
  __u64 total_pkts;
  __u64 total_bytes;
  __u64 tcp_pkts;
  __u64 udp_pkts;
  __u64 icmp_pkts;
  // add more packet types
};

// for real-time packet probing
// https://github.com/xdp-project/xdp-tutorial/tree/main/basic03-map-counter
struct {
  __uint(type, BPF_MAP_TYPE_RINGBUF);
  __uint(max_entries, 256 * 1024); // 256kb
} ringbuf SEC(".maps");

// Per-CPU array map
struct {
  // PERCPU_ARRAY is for all CPU cores
  // if it was single core cpu BPF_MAP_TYPE_ARRAY should work
  // avoids expensive synchronization when multiple CPUs update counters
  // concurrently
  __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);

  __uint(max_entries,
         1);          // each CPU in every cpu cycle has only one entry (key=0)
  __type(key, __u32); // always key = 0
  __type(value, struct pkt_stats); // packet counters
} stats_map SEC(".maps");

/*
 * Packet Flow
 *
 * Incoming packet
 *        │
 *        ▼
 *   XDP/eBPF program
 *        │
 *        ├── Update per-CPU statistics
 *        └── Send packet metadata to ring buffer
 *                     │
 *                     ▼
 *              User-space application
 */

char _license[] SEC("license") = "GPL"; // required for compilation by kernel
