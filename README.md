# Sentry

> An eBPF-powered network observability and packet inspection tool for Linux.

Sentry is an experimental network inspection tool inspired by Wireshark, built around eBPF.

Instead of only capturing packets and decoding protocols, Sentry aims to connect network activity with the system context behind it: processes, PIDs, sockets, connections, and kernel-level events.

The goal is to answer not only:

> What packet was sent?

but also:

> Which process sent it, where did it go, and what happened along the way?

## Why Sentry?

Traditional packet analyzers provide deep visibility into network traffic. However, understanding which process or system activity caused a particular network event often requires combining multiple tools.

Sentry explores a different architecture by using eBPF hooks inside the Linux kernel and streaming events to a userspace application.

## Architecture

```text
                 Linux Kernel
                      │
                      ▼
           ┌─────────────────────┐
           │    eBPF Programs    │
           │                     │
           │  XDP / TC / Hooks   │
           └──────────┬──────────┘
                      │
                Ring Buffer
                      │
                      ▼
           ┌─────────────────────┐
           │ Userspace Collector │
           │                     │
           │ Filtering           │
           │ Aggregation         │
           │ Flow Tracking       │
           └──────────┬──────────┘
                      │
                      ▼
           ┌─────────────────────┐
           │      SDL UI         │
           │                     │
           │ Connections         │
           │ Packets             │
           │ Processes           │
           │ Events              │
           └─────────────────────┘
```

## Goals

Sentry aims to provide:

* Live network event inspection
* TCP and UDP flow tracking
* Process-aware network visibility
* PID and process correlation
* Source and destination inspection
* Protocol metadata
* Live filtering
* Connection timelines
* Kernel-level observability through eBPF
* A lightweight native UI

## Example

A traditional packet view might show:

```text
192.168.1.4:49231 → 142.250.x.x:443
TCP | 24 KB | 31 packets
```

Sentry aims to associate that traffic with system context:

```text
firefox
PID 8412

192.168.1.4:49231
        ↓
142.250.x.x:443

TCP | TLS | 24 KB | 31 packets
```

## Components

### Kernel Space

eBPF programs observe network activity through Linux kernel hooks.

Potential attachment points include:

* TC
* XDP
* Tracepoints
* Kprobes
* Socket hooks
* Cgroup hooks

The exact capture strategy is still evolving as the project develops.

### Userspace

The userspace component is responsible for:

* Receiving events from eBPF programs
* Tracking active flows
* Correlating traffic with processes
* Aggregating network metadata
* Applying filters
* Maintaining application state

Communication between kernel space and userspace is planned through BPF ring buffers.

### Interface

Sentry is planned as a native graphical application using SDL.

```text
┌──────────────────────────────────────────────────────┐
│ Sentry                              Capture: ● LIVE  │
├────────────┬─────────┬────────────┬───────┬──────────┤
│ Process    │ PID     │ Remote     │ Proto │ Traffic  │
├────────────┼─────────┼────────────┼───────┼──────────┤
│ firefox    │ 8421    │ :443       │ TCP   │ 1.2 MB   │
│ discord    │ 9122    │ :443       │ TCP   │ 842 KB   │
│ ssh        │ 2231    │ :22        │ TCP   │ 24 KB    │
│ curl       │ 9912    │ :443       │ TCP   │ 8 KB     │
└────────────┴─────────┴────────────┴───────┴──────────┘
```

## Tech Stack

The exact implementation stack is still being explored.

```text
eBPF programs     C
Userspace         Rust or C
UI                SDL
```

Potential eBPF userspace libraries include:

* libbpf
* libbpf-rs
* Aya

## Project Status

🚧 Sentry is currently in the early development and research phase.

The architecture, implementation language, and capture strategy are still being explored.

See [`ROADMAP.md`](./ROADMAP.md) for planned milestones and future development.

## Non-Goals

At least initially, Sentry is not intended to:

* Replace Wireshark
* Implement every protocol dissector
* Provide full payload inspection for every protocol
* Support every operating system

The initial focus is Linux network observability through eBPF.

## Inspiration

Sentry is inspired by:

* Wireshark
* tcpdump
* libpcap
* eBPF
* bpftrace
* Cilium
* Pixie

## License

TBD

