#ifndef __RINGBUF_EVENTS_H__
#define __RINGBUF_EVENTS_H__

#include "../../include/vmlinux.h"
#include "../common/events.h"
#include "../common/maps.h"
#include <bpf/bpf_endian.h>
#include <bpf/bpf_helpers.h>

static __always_inline void send_ringbuf_event(struct iphdr *iph,
                                               __u16 src_port, __u16 dst_port,
                                               __u8 proto, __u32 len) {
  /* send event to userspace using Ring Buffer */
  /* reserve space in ring buffer */
  struct pkt_event *ev = bpf_ringbuf_reserve(&ringbuf, sizeof(*ev), 0);
  if (!ev)
    return; /* if ring buffer is full; skip event but keep packet */

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
}

#endif
