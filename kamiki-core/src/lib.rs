pub mod collector;
pub mod error;
pub mod event;
pub mod filter;
pub mod flow;
pub mod process;

pub use error::Error;

pub struct Kamiki {
    collector_handle: collector::CollectorHandle,
    pub flows: std::sync::Arc<flow::FlowTable>,
    pub events: crossbeam_channel::Receiver<event::NetworkEvent>,
}

impl Kamiki {
    pub fn start(config: collector::CollectorConfig) -> Result<Self, Error> {
        let flows = std::sync::Arc::new(flow::FlowTable::new());
        let collector = collector::Collector::new(config);
        let events = collector.events();
        let collector_handle = collector.spawn()?;

        Ok(Self {
            collector_handle,
            flows,
            events,
        })
    }

    pub fn stop(self) {
        self.collector_handle.stop();
    }
}
