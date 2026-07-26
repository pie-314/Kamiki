// refrence : https://github.com/xdp-project/xdp-tutorial
#include "../../include/vmlinux.h" // contains all kernel data structures from BTF
// bpftool btf dump file /sys/kernel/btf/vmlinux format c > vmlinux.h

#include "../common/events.h"
#include "../common/maps.h"
#include "../common/socket.h"
#include "../common/stats.h"

#include "parser.h"
#include "ringbuf_events.h"

#include <bpf/bpf_endian.h>
#include <bpf/bpf_helpers.h>

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

  parse_l4_protocol(l4_hdr, data_end, proto, stats, &src_port, &dst_port);

  send_ringbuf_event(iph, src_port, dst_port, proto, len);

  return XDP_PASS;
}

char _license[] SEC("license") = "GPL"; // required for compilation by kernel
