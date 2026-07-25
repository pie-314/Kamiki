use crate::app::App;
use kamiki_core::event::Protocol;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Min(6),
            Constraint::Length(1),
        ])
        .split(area);

    draw_header(frame, chunks[0]);
    draw_info(frame, app, chunks[1]);
    draw_stats(frame, app, chunks[2]);
    draw_packets(frame, app, chunks[3]);
    draw_footer(frame, chunks[4]);
}

fn draw_header(frame: &mut Frame, area: ratatui::layout::Rect) {
    let block = Block::default().borders(Borders::ALL);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let text = vec![
        Line::from(Span::styled(
            "KAMIKI v0.1",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "eBPF XDP Packet Inspector (LIVE)",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    frame.render_widget(Paragraph::new(text).alignment(Alignment::Center), inner);
}

fn draw_info(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let status_color = if app.is_running {
        Color::Green
    } else {
        Color::Red
    };
    let status_text = if app.is_running { "RUNNING" } else { "STOPPED" };

    let lines = vec![
        Line::from(vec![
            Span::raw(format!(" Interface : {:<20}", app.interface)),
            Span::raw("   "),
            Span::raw("Status : "),
            Span::styled(
                status_text,
                Style::default()
                    .fg(status_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw(format!(" Mode      : {:<20}", "XDP")),
            Span::raw("   "),
            Span::raw("Events : "),
            Span::styled("Ring Buffer", Style::default().fg(Color::Yellow)),
        ]),
    ];

    frame.render_widget(Paragraph::new(lines), area);
}

fn draw_stats(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let s = &app.stats;

    let lines = vec![
        Line::from(format!(" Total Packets : {}", fmt_num(s.total_pkts))),
        Line::from(format!(" Total Bytes   : {}", fmt_bytes(s.total_bytes))),
        Line::from(format!(" TCP           : {}", fmt_num(s.tcp_pkts))),
        Line::from(format!(" UDP           : {}", fmt_num(s.udp_pkts))),
        Line::from(format!(" ICMP          : {}", fmt_num(s.icmp_pkts))),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " Statistics ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    frame.render_widget(Paragraph::new(lines).block(block), area);
}

fn draw_packets(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let header = Row::new(vec![
        Cell::from(Span::styled(
            "Proto",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Source",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Destination",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Size",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ])
    .height(1);

    let rows: Vec<Row> = app
        .packet_log
        .iter()
        .enumerate()
        .map(|(i, ev)| {
            let proto_color = match ev.protocol {
                Protocol::Tcp => Color::Green,
                Protocol::Udp => Color::Yellow,
                Protocol::Icmp => Color::Magenta,
                Protocol::Unknown(_) => Color::DarkGray,
            };

            let src = match ev.protocol {
                Protocol::Icmp => ev.src_ip.to_string(),
                _ => format!("{}:{}", ev.src_ip, ev.src_port),
            };
            let dst = match ev.protocol {
                Protocol::Icmp => ev.dst_ip.to_string(),
                _ => format!("{}:{}", ev.dst_ip, ev.dst_port),
            };

            let style = if i == app.selected_row {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(Span::styled(
                    format!("{}", ev.protocol),
                    Style::default()
                        .fg(proto_color)
                        .add_modifier(Modifier::BOLD),
                )),
                Cell::from(src),
                Cell::from(dst),
                Cell::from(format!("{} B", ev.pkt_len)),
            ])
            .style(style)
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            " Live Packets ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Center);

    let table = Table::new(
        rows,
        [
            Constraint::Length(7),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
            Constraint::Min(10),
        ],
    )
    .header(header)
    .row_highlight_style(Style::default().bg(Color::DarkGray))
    .block(block);

    let mut state = TableState::default().with_selected(Some(app.selected_row));
    frame.render_stateful_widget(table, area, &mut state);
}

fn draw_footer(frame: &mut Frame, area: ratatui::layout::Rect) {
    let text = Line::from(vec![
        Span::styled(" q ", Style::default().bg(Color::DarkGray).fg(Color::White)),
        Span::raw(" Quit  "),
        Span::styled(
            " ↑↓ ",
            Style::default().bg(Color::DarkGray).fg(Color::White),
        ),
        Span::raw(" Scroll  "),
        Span::styled(
            " j/k ",
            Style::default().bg(Color::DarkGray).fg(Color::White),
        ),
        Span::raw(" Scroll (vim)"),
    ]);

    frame.render_widget(Paragraph::new(text), area);
}

fn fmt_num(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out.chars().rev().collect()
}

fn fmt_bytes(bytes: u64) -> String {
    match bytes {
        b if b < 1_024 => format!("{} B", b),
        b if b < 1_024 * 1_024 => format!("{:.1} KB", b as f64 / 1_024.0),
        b if b < 1_024 * 1_024 * 1_024 => format!("{:.1} MB", b as f64 / (1_024.0 * 1_024.0)),
        b => format!("{:.2} GB", b as f64 / (1_024.0 * 1_024.0 * 1_024.0)),
    }
}
