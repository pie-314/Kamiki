#ifndef __STATS_H__
#define __STATS_H__

#include "../../include/vmlinux.h"

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

#endif
