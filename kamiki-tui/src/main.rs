mod app;
mod ui;

use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use kamiki_core::{Kamiki, collector::CollectorConfig};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "kamiki-tui", about = "Kamiki - eBPF network inspector TUI")]
struct Cli {
    #[arg(short, long, default_value = "eth0")]
    interface: String,

    #[arg(short, long, default_value = "kamiki-ebpf/out/xdp_prober.bpf.o")]
    object: String,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    let config = CollectorConfig {
        interface: cli.interface.clone(),
        object_path: cli.object,
        ..Default::default()
    };

    let kamiki = Kamiki::start(config)?;
    let mut app = App::new(kamiki, cli.interface);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_event_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    app.shutdown();
    result
}

fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        app.tick();
        terminal.draw(|frame| ui::draw(frame, app))?;

        if event::poll(Duration::from_millis(50))?
            && let Event::Key(key) = event::read()?
        {
            match (key.code, key.modifiers) {
                (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                _ => app.handle_key(key),
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
