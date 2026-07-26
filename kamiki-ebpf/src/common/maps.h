#ifndef __MAPS_H__
#define __MAPS_H__

#include "stats.h"
#include <bpf/bpf_helpers.h>

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

#endif
