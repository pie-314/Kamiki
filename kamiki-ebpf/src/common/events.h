#ifndef __EVENTS_H__
#define __EVENTS_H__

#include "../../include/vmlinux.h" // contains all kernel data structures from BTF
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

#endif
