use crossterm::event::{ Event, KeyCode };
use ratatui::{
    DefaultTerminal,
    Frame,
    layout::{ Constraint, Layout },
    style::{ Color, Modifier },
    widgets::{ Block, Borders, List, ListState },
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut list_state = ListState::default().with_selected(Some(0));
    loop {
        terminal.draw(|frame| render(frame, &mut list_state))?;
        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => list_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => list_state.select_previous(),
                KeyCode::Char('q') | KeyCode::Esc => {
                    break Ok(());
                }
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, list_state: &mut ListState) {
    let layout = Layout::vertical([Constraint::Percentage(100)]).margin(1);
    let [top] = frame.area().layout(&layout);

    let items = ["Compress", "Decompress", "Quit"];

    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(Block::new().borders(Borders::ALL).title("SQUISH"));

    frame.render_stateful_widget(list, top, list_state);
}
