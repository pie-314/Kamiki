// refrence : https://github.com/xdp-project/xdp-tutorial
#include "include/vmlinux.h" // contains all kernel data structures from BTF
// bpftool btf dump file /sys/kernel/btf/vmlinux format c > vmlinux.h

#include <bpf/bpf_endian.h>
#include <bpf/bpf_helpers.h>

// event structure this is passed to user space via ring buffer
// for real-time packet inspection
// ring buffers are more efficient thatn perf events for kernel to userspace
struct pkt_event {
  __u32 src_ip;
  __u32 dst_ip;
  __u16 src_port;
  __u16 dst_port;
  __u8 protocol; // 6-> tcp; 17-> udp; 1 -> icmp;
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

/*
  Packet Layout

    ctx->data                                                   ctx->data_end
        |                                                             |
        v                                                             v
    +---------------+--------------------+------------------+---------+
    | ethhdr (14B)  | iphdr (iph->ihl*4) | tcphdr / udphdr  | payload |
    +---------------+--------------------+------------------+---------+
    ^               ^                    ^
    eth             iph                  l4_hdr
 */

// [ 00 11 22 ... 14 bytes L2 ... 20 bytes L3 ... 20 bytes L4 ... 20 bytes
// Payload ]

SEC("xdp")
int xdp_prober(struct xdp_md *ctx) {
  // packet memory boundary
  // __u32 len = (void *)(long)ctx->data - (void *)(long)ctx->data_end;
  void *data_start = (void *)(long)ctx->data;
  void *data_end = (void *)(long)ctx->data_end;

  // fetch cpu-local stats bucket
  // this is PERCPU_ARRAY
  // it automatically returns the CPU-local stats bucket for
  // whichever core processed this packet
  __u32 key = 0;
  struct pkt_stats *stats = bpf_map_lookup_elem(&stats_map, &key);

  // verifier rejects if NULL
  if (!stats)
    return XDP_PASS;

  // layer 2 : etthdr (14B)
  // parsing header
  struct ethhdr *eth = data_start;
  if ((void *)(eth + 1) > data_end)
    return XDP_PASS;

  __u32 len = data_end - data_start;
  stats->total_pkts++;
  stats->total_bytes += len;

  // ignore non ipv4 traffic
  if (eth->h_proto != bpf_htons(0x0800))
    return XDP_PASS;

  // layer 3 : iphdr (iph->ihl*4)
  // the IP header
  // pointer skip 14B ethernet header
  // point to start of IP header
  struct iphdr *iph = (void *)(eth + 1);

  // bound check 20B
  if ((void *)(iph + 1) > data_end)
    return XDP_PASS;

  // reads ethernet header field which is 4 bytes (32 bits)
  __u32 ip_hdr_len = iph->ihl * 4;
  if (ip_hdr_len < sizeof(*iph)) // corruted ip header check
    return XDP_PASS;

  // layer 4 : protocol header

  // pointer pass ip header to reach transport/protocol layer l4
  void *l4_hdr = (void *)iph + ip_hdr_len;

  if (l4_hdr > data_end) // boundary check : should not exceed the end
    return XDP_PASS;

  __u16 src_port = 0;
  __u16 dst_port = 0;
  __u8 proto = iph->protocol;

  // parse protocol header
  switch (proto) {
  case IPPROTO_TCP: {
    struct tcphdr *tcph = l4_hdr;
    // it should not exceed data_end
    if ((void *)(tcph + 1) <= data_end) {
      src_port = bpf_ntohs(tcph->source);
      dst_port = bpf_ntohs(tcph->dest);
    }
    stats->tcp_pkts++;
    break;
  }
  case IPPROTO_UDP: {
    struct udphdr *udph = l4_hdr;
    // it should not exceed data_end
    if ((void *)(udph + 1) <= data_end) {
      src_port = bpf_ntohs(udph->source);
      dst_port = bpf_ntohs(udph->dest);
    }
    stats->udp_pkts++;
    break;
  }
  case IPPROTO_ICMP:
    stats->icmp_pkts++;
    break;
  }

  /* send event to userspace using Ring Buffer */
  /* reserve space in ring buffer */
  struct pkt_event *ev = bpf_ringbuf_reserve(&ringbuf, sizeof(*ev), 0);
  if (!ev)
    return XDP_PASS; /* if ring buffer is full; skip event but keep packet */

  /*
   bpf_ntohs()
   bpf_ntohl()
   bpf_htons()

   packet headers store multi-byte values in network byte order (big-endian).
   convert them to host byte order before using them in userspace.
   */

  ev->src_ip = bpf_ntohl(iph->saddr); /* source IP to host byte order */
  ev->dst_ip = bpf_ntohl(iph->daddr); /* dest IP to host byte order */
  ev->src_port = src_port;
  ev->dst_port = dst_port;
  ev->protocol = proto;
  ev->pkt_len = len;

  /* commit ring buffer for userspace */
  bpf_ringbuf_submit(ev, 0);

  return XDP_PASS;
}

char _license[] SEC("license") = "GPL"; // required for compilation by kernel