# Kamiki Roadmap

This document tracks the planned development of Kamiki.

Kamiki is an experimental eBPF-powered network observability and packet inspection tool for Linux. The roadmap is expected to evolve as capture strategies, kernel hooks, and process correlation mechanisms are explored.

## Phase 0: Research and Architecture

Establish the initial architecture and validate the core technical assumptions.

* [x] Choose the primary implementation language for userspace
* [x] Evaluate C vs Rust for the userspace collector
* [x] Evaluate libbpf, libbpf-rs, and Aya
* [x] Study Linux networking paths relevant to packet observation
* [x] Compare TC, XDP, socket, cgroup, tracepoint, and kprobe hooks
* [x] Define the kernel-to-userspace event format
* [x] Define initial repository structure
* [x] Document architecture decisions

### Exit Criteria

* A minimal architecture is selected
* The first eBPF attachment point is chosen
* Kernel-to-userspace communication strategy is defined

---

## Phase 1: Minimal eBPF Capture Pipeline

Build the smallest working path from the Linux kernel to userspace.

```text
Network Event
      │
      ▼
 eBPF Program
      │
      ▼
 Ring Buffer
      │
      ▼
 Userspace
      │
      ▼
 Terminal Output
```

* [ ] Build and load a minimal eBPF program
* [ ] Attach the program to a network-related hook
* [ ] Observe basic network events
* [ ] Define a shared event structure
* [ ] Send events through a BPF ring buffer
* [ ] Read events from userspace
* [ ] Print captured metadata to the terminal
* [ ] Handle program loading and attachment errors
* [ ] Handle clean shutdown and eBPF program detachment

### Exit Criteria

Running Kamiki produces live network events in the terminal.

---

## Phase 2: Packet Metadata

Extract useful metadata from observed network traffic.

### Ethernet

* [ ] Parse Ethernet headers
* [ ] Extract EtherType
* [ ] Handle unsupported link-layer protocols safely

### IPv4

* [ ] Parse IPv4 headers
* [ ] Extract source address
* [ ] Extract destination address
* [ ] Extract transport protocol
* [ ] Handle variable IPv4 header length

### IPv6

* [ ] Parse IPv6 headers
* [ ] Extract source address
* [ ] Extract destination address
* [ ] Identify next-header protocol
* [ ] Investigate extension header handling

### TCP

* [ ] Parse TCP headers
* [ ] Extract source port
* [ ] Extract destination port
* [ ] Extract TCP flags
* [ ] Track SYN, ACK, FIN, and RST events

### UDP

* [ ] Parse UDP headers
* [ ] Extract source port
* [ ] Extract destination port
* [ ] Extract datagram length

### Safety

* [ ] Validate packet boundaries before every header access
* [ ] Keep packet parsing verifier-safe
* [ ] Handle truncated packets
* [ ] Handle malformed headers

### Exit Criteria

Kamiki can display structured metadata for IPv4 TCP and UDP traffic.

---

## Phase 3: Flow Tracking

Move from individual events toward connection-level visibility.

Define flows using tuples such as:

```text
source IP
source port
destination IP
destination port
protocol
```

* [ ] Define a canonical flow key
* [ ] Normalize bidirectional flows
* [ ] Track first-seen timestamp
* [ ] Track last-seen timestamp
* [ ] Track packet count
* [ ] Track bytes sent
* [ ] Track bytes received
* [ ] Track flow direction
* [ ] Track TCP connection state
* [ ] Detect inactive flows
* [ ] Remove expired flow state
* [ ] Define memory limits for flow tracking

### Exit Criteria

Kamiki maintains a live table of active network flows.

Example:

```text
LOCAL                 REMOTE                PROTO    PACKETS    BYTES
192.168.1.4:49231     142.250.x.x:443       TCP      31         24 KB
192.168.1.4:53122     1.1.1.1:53            UDP      2          184 B
```

---

## Phase 4: Process Correlation

Associate network activity with the process responsible for it.

This phase is one of Kamiki's primary technical goals.

* [ ] Research socket-to-process correlation strategies
* [ ] Capture PID where available
* [ ] Capture TGID where relevant
* [ ] Capture UID
* [ ] Capture process command name
* [ ] Associate sockets with processes
* [ ] Maintain socket ownership state
* [ ] Correlate socket state with observed traffic
* [ ] Handle short-lived processes
* [ ] Handle process exit
* [ ] Handle socket reuse
* [ ] Investigate network namespaces
* [ ] Investigate cgroup metadata
* [ ] Investigate container-aware correlation

### Known Challenges

Process attribution may not be available at every network hook.

For example, packet-level hooks may execute in contexts where the originating userspace process is no longer directly available.

Kamiki may require correlation across multiple eBPF programs and shared BPF maps.

Possible architecture:

```text
Process / Socket Hooks
          │
          ▼
   Socket Ownership Map
          │
          │
          ▼
Packet / Network Hooks
          │
          ▼
   Correlated Event
          │
          ▼
      Userspace
```

### Exit Criteria

Kamiki can associate a meaningful subset of TCP and UDP traffic with originating processes.

Example:

```text
PROCESS      PID      LOCAL              REMOTE             PROTO
firefox      8421     :49231             :443               TCP
curl         9912     :53122             :443               TCP
ssh          2231     :49244             :22                TCP
```

---

## Phase 5: Userspace Event Engine

Build a stable userspace core between eBPF collection and the UI.

* [ ] Define internal event types
* [ ] Separate raw kernel events from application models
* [ ] Build flow state management
* [ ] Build process state management
* [ ] Build connection indexing
* [ ] Add event timestamps
* [ ] Add bounded event storage
* [ ] Handle event bursts
* [ ] Track dropped events
* [ ] Add structured logging
* [ ] Add graceful shutdown
* [ ] Separate capture logic from presentation logic

### Proposed Architecture

```text
eBPF Ring Buffer
       │
       ▼
 Event Decoder
       │
       ▼
 Event Processor
       │
       ├──────────────► Process State
       │
       ├──────────────► Flow State
       │
       └──────────────► Event History
                              │
                              ▼
                             UI
```

### Exit Criteria

The userspace core can run independently of the graphical interface.

---

## Phase 6: Filtering

Add live filtering for events and connections.

Initial filters:

* [ ] Filter by protocol
* [ ] Filter by source IP
* [ ] Filter by destination IP
* [ ] Filter by source port
* [ ] Filter by destination port
* [ ] Filter by PID
* [ ] Filter by process name
* [ ] Filter by UID

Example:

```text
protocol == tcp
```

```text
dst.port == 443
```

```text
process == "firefox"
```

```text
process == "curl" && dst.port == 443
```

Future investigation:

* [ ] Design a filter expression grammar
* [ ] Build a filter parser
* [ ] Build a filter AST
* [ ] Evaluate userspace filtering
* [ ] Evaluate kernel-side filtering
* [ ] Explore compiling filters into eBPF-safe logic

### Exit Criteria

Users can dynamically filter live network activity.

---

## Phase 7: Native UI

Build the graphical interface using SDL.

### Foundation

* [ ] Initialize SDL
* [ ] Create application window
* [ ] Build event loop
* [ ] Add font rendering
* [ ] Add keyboard input
* [ ] Add mouse input
* [ ] Add resizable layout

### Connection View

* [ ] Display active flows
* [ ] Display process names
* [ ] Display PIDs
* [ ] Display local endpoints
* [ ] Display remote endpoints
* [ ] Display protocol
* [ ] Display packet counts
* [ ] Display byte counts
* [ ] Add sorting
* [ ] Add scrolling
* [ ] Add row selection

### Event View

* [ ] Display live events
* [ ] Add timestamps
* [ ] Add protocol information
* [ ] Add process context
* [ ] Add pause and resume
* [ ] Add event selection

### Detail View

* [ ] Show selected flow metadata
* [ ] Show process information
* [ ] Show connection lifetime
* [ ] Show packet counters
* [ ] Show TCP state
* [ ] Show event timeline

### Filtering UI

* [ ] Add filter input
* [ ] Display filter errors
* [ ] Apply filters without restarting capture
* [ ] Preserve filter history

### Exit Criteria

Kamiki provides a usable live graphical view of process-aware network activity.

---

## Phase 8: Protocol Awareness

Add lightweight protocol-level visibility.

Initial targets:

* [ ] DNS
* [ ] HTTP metadata
* [ ] TLS metadata

### DNS

* [ ] Identify DNS traffic
* [ ] Extract query names
* [ ] Extract query types
* [ ] Correlate queries and responses

### HTTP

* [ ] Identify plaintext HTTP traffic
* [ ] Extract request method
* [ ] Extract host metadata where available

### TLS

* [ ] Identify TLS traffic
* [ ] Parse selected handshake metadata
* [ ] Investigate SNI visibility
* [ ] Investigate TLS version detection

### Exit Criteria

Kamiki can provide lightweight application-protocol context without attempting to become a full protocol dissector.

---

## Phase 9: Advanced TCP Observability

Explore deeper transport-level diagnostics.

* [ ] Track TCP connection lifetime
* [ ] Detect retransmissions
* [ ] Track connection failures
* [ ] Track resets
* [ ] Investigate RTT measurement
* [ ] Investigate handshake latency
* [ ] Investigate socket-level TCP metrics
* [ ] Surface abnormal connection behavior

Possible output:

```text
PROCESS    REMOTE       RTT       RETRANSMITS    STATE
firefox    :443         24 ms     2              ESTABLISHED
curl       :443         81 ms     0              ESTABLISHED
ssh        :22          12 ms     1              ESTABLISHED
```

---

## Phase 10: Persistence and Export

Allow captured information to be stored and analyzed later.

* [ ] Define a Kamiki capture format
* [ ] Export events
* [ ] Export flow summaries
* [ ] Add JSON export
* [ ] Add CSV export
* [ ] Investigate PCAP interoperability
* [ ] Load previous Kamiki sessions
* [ ] Add capture metadata

---

## Phase 11: Performance and Reliability

Measure and improve behavior under real workloads.

* [ ] Benchmark event throughput
* [ ] Benchmark CPU overhead
* [ ] Benchmark memory usage
* [ ] Track ring buffer pressure
* [ ] Track dropped events
* [ ] Test high connection counts
* [ ] Test high packet rates
* [ ] Add bounded queues
* [ ] Add backpressure strategies
* [ ] Fuzz userspace event decoding
* [ ] Test malformed packet handling
* [ ] Test verifier compatibility across supported kernels

---

## Phase 12: Packaging

Prepare Kamiki for broader use.

* [ ] Define supported Linux kernel versions
* [ ] Add reproducible build instructions
* [ ] Add dependency checks
* [ ] Add installation script
* [ ] Add release builds
* [ ] Add CI
* [ ] Add automated tests
* [ ] Add example captures
* [ ] Add troubleshooting documentation
* [ ] Document required Linux capabilities

---

## Future Ideas

These are exploratory and not guaranteed.

* [ ] Container-aware network inspection
* [ ] Kubernetes workload context
* [ ] Network namespace visualization
* [ ] Per-process bandwidth graphs
* [ ] Connection timeline visualization
* [ ] Remote capture agents
* [ ] Headless mode
* [ ] TUI mode
* [ ] Plugin system
* [ ] Custom protocol decoders
* [ ] eBPF-based anomaly detection
* [ ] EEL-powered filter expressions
* [ ] Compile user filters into eBPF programs

## Current Milestone

The immediate target is intentionally small:

```text
Load eBPF program
        ↓
Observe network event
        ↓
Send event through ring buffer
        ↓
Receive event in userspace
        ↓
Print structured metadata
```

Everything else builds on this pipeline.

## Contributing

Kamiki is currently experimental and under active development.

Architecture discussions, implementation ideas, bug reports, and contributions are welcome as the project evolves.

