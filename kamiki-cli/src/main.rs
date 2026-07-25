use anyhow::Result;
use clap::Parser;
use kamiki_core::{Kamiki, collector::CollectorConfig, filter::Filter, flow::update_flow};
use log::info;

#[derive(Parser)]
#[command(
    name = "kamiki",
    about = "eBPF-powered network observability — process-aware packet inspection",
    version
)]
struct Cli {
    #[arg(short, long, default_value = "eth0")]
    interface: String,

    #[arg(short, long, default_value = "kamiki-ebpf/out/xdp_prober.bpf.o")]
    object: String,

    #[arg(short, long)]
    protocol: Option<String>,

    #[arg(long)]
    dst_port: Option<u16>,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    let config = CollectorConfig {
        interface: cli.interface,
        object_path: cli.object,
        ..Default::default()
    };

    let kamiki = Kamiki::start(config)?;

    info!("Kamiki started - Ctrl+C to stop");
    println!("{:<20} {:<20} {:<8} BYTES", "SRC", "DST", "PROTO");
    println!("{}", "-".repeat(60));

    let filter = Filter::default(); // Phase 6: populate from CLI args

    for event in &kamiki.events {
        if !filter.matches(&event) {
            continue;
        }

        update_flow(&kamiki.flows, &event);

        let src = format!("{}:{}", event.src_ip, event.src_port);
        let dst = format!("{}:{}", event.dst_ip, event.dst_port);
        println!(
            "{:<20} {:<20} {:<8} {}",
            src, dst, event.protocol, event.pkt_len
        );
    }

    kamiki.stop();
    Ok(())
}
