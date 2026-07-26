#ifndef __XDP_PARSER_H__
#define __XDP_PARSER_H__

#include "../../include/vmlinux.h"
#include "../common/stats.h"
#include <bpf/bpf_endian.h>

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

static __always_inline void
parse_l4_protocol(void *l4_hdr, void *data_end, __u8 proto,
                  struct pkt_stats *stats, __u16 *src_port, __u16 *dst_port) {
  // parse protocol header
  switch (proto) {
  case IPPROTO_TCP: {
    struct tcphdr *tcph = l4_hdr;
    // it should not exceed data_end
    if ((void *)(tcph + 1) <= data_end) {
      *src_port = bpf_ntohs(tcph->source);
      *dst_port = bpf_ntohs(tcph->dest);
    }
    stats->tcp_pkts++;
    break;
  }
  case IPPROTO_UDP: {
    struct udphdr *udph = l4_hdr;
    // it should not exceed data_end
    if ((void *)(udph + 1) <= data_end) {
      *src_port = bpf_ntohs(udph->source);
      *dst_port = bpf_ntohs(udph->dest);
    }
    stats->udp_pkts++;
    break;
  }
  case IPPROTO_ICMP:
    stats->icmp_pkts++;
    break;
  }
}

#endif
