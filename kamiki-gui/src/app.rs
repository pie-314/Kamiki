use egui::{CentralPanel, Color32, Panel, RichText, ScrollArea};
use kamiki_core::{
    Kamiki,
    collector::CollectorConfig,
    event::NetworkEvent,
    filter::Filter,
    flow::{FlowEntry, FlowKey, update_flow},
};
use std::collections::VecDeque;

const MAX_EVENT_LOG: usize = 1000;

pub struct KamikiApp {
    kamiki: Option<Kamiki>,
    filter: Filter,
    event_log: VecDeque<NetworkEvent>,
    flow_snapshot: Vec<(FlowKey, FlowEntry)>,
    selected_flow: Option<usize>,
    status: String,
}

impl KamikiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let kamiki = match Kamiki::start(CollectorConfig::default()) {
            Ok(k) => Some(k),
            Err(e) => {
                log::error!("failed to start kamiki: {e}");
                None
            }
        };

        Self {
            kamiki,
            filter: Filter::default(),
            event_log: VecDeque::with_capacity(MAX_EVENT_LOG),
            flow_snapshot: Vec::new(),
            selected_flow: None,
            status: "Ready - waiting for events".into(),
        }
    }

    fn tick(&mut self) {
        let Some(kamiki) = &self.kamiki else { return };

        let mut count = 0;
        while let Ok(event) = kamiki.events.try_recv() {
            if self.filter.matches(&event) {
                update_flow(&kamiki.flows, &event);
                if self.event_log.len() >= MAX_EVENT_LOG {
                    self.event_log.pop_front();
                }
                self.event_log.push_back(event);
            }
            count += 1;
            if count >= 512 {
                break;
            }
        }

        self.flow_snapshot = kamiki
            .flows
            .iter()
            .map(|e| (e.key().clone(), e.value().clone()))
            .collect();
        self.flow_snapshot
            .sort_by_key(|b| std::cmp::Reverse(b.1.bytes));

        self.status = format!(
            "Flows: {}   Events captured: {}",
            self.flow_snapshot.len(),
            self.event_log.len()
        );
    }
}

impl eframe::App for KamikiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.tick();
        ui.ctx().request_repaint(); // continuous redraw for live data

        Panel::top("header").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("Kamiki").color(Color32::from_rgb(80, 200, 255)));
                ui.label(" - eBPF Network Observability");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("● LIVE").color(Color32::GREEN));
                });
            });
        });

        Panel::bottom("status").show(ui, |ui| {
            ui.label(RichText::new(&self.status).color(Color32::GRAY));
        });

        Panel::right("event_log")
            .resizable(true)
            .default_size(380.0)
            .show(ui, |ui| {
                ui.heading("Events");
                ui.separator();
                ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                    for event in self.event_log.iter().rev().take(200) {
                        ui.label(format!(
                            "{} {}:{} → {}:{}  {} B",
                            event.protocol,
                            event.src_ip,
                            event.src_port,
                            event.dst_ip,
                            event.dst_port,
                            event.pkt_len,
                        ));
                    }
                });
            });

        CentralPanel::default().show(ui, |ui| {
            ui.heading("Flows");
            ui.separator();

            if self.flow_snapshot.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        RichText::new(
                            "No flows captured yet.\nMake sure Kamiki is running as root on Linux.",
                        )
                        .color(Color32::GRAY),
                    );
                });
                return;
            }

            egui::Grid::new("flow_table")
                .num_columns(6)
                .striped(true)
                .show(ui, |ui| {
                    for col in &["Process", "Src", "Dst", "Proto", "Pkts", "Bytes"] {
                        ui.label(RichText::new(*col).strong());
                    }
                    ui.end_row();

                    for (i, (key, entry)) in self.flow_snapshot.iter().enumerate() {
                        let selected = self.selected_flow == Some(i);
                        if selected {
                            ui.visuals_mut().override_text_color =
                                Some(Color32::from_rgb(200, 230, 255));
                        }

                        ui.label("—"); // Phase 4: process name
                        ui.label(format!("{}:{}", key.src_ip, key.src_port));
                        ui.label(format!("{}:{}", key.dst_ip, key.dst_port));
                        ui.label(format!("{}", key.protocol));
                        ui.label(format!("{}", entry.packets));
                        if ui.label(human_bytes(entry.bytes)).clicked() {
                            self.selected_flow = Some(i);
                        }
                        ui.end_row();

                        if selected {
                            ui.visuals_mut().override_text_color = None;
                        }
                    }
                });
        });
    }
}

fn human_bytes(bytes: u64) -> String {
    match bytes {
        b if b < 1024 => format!("{b} B"),
        b if b < 1024 * 1024 => format!("{:.1} KB", b as f64 / 1024.0),
        b => format!("{:.1} MB", b as f64 / (1024.0 * 1024.0)),
    }
}
