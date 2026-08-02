use crossterm::event::{Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier},
    widgets::{Block, Borders, List, ListItem, ListState},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let menu = Menu {
        items: vec![
            MenuItem {
                label: "Compress".into(),
                screen: Screen::Compress,
            },
            MenuItem {
                label: "Decompress".into(),
                screen: Screen::Decompress,
            },
            MenuItem {
                label: "Quit".into(),
                screen: Screen::Quit,
            },
        ],
        screen: Screen::Home,
    };

    let mut app = App {
        current_screen: Screen::Home,
        list_state: ListState::default().with_selected(Some(0)),
        menu: &menu,
    };

    loop {
        terminal.draw(|frame| render(frame, &mut app))?;
        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => app.list_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => app.list_state.select_previous(),
                KeyCode::Char('q') | KeyCode::Esc => {
                    break Ok(());
                }
                KeyCode::Enter => {
                    if let Some(index) = app.list_state.selected() {
                        app.current_screen = menu.items[index].screen;
                    }
                }
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, app: &mut App) {
    let items = app
        .menu
        .items
        .iter()
        .map(|item| ListItem::new(item.label.as_str()));

    let layout = Layout::vertical([Constraint::Percentage(100)]).margin(1);
    let [top] = frame.area().layout(&layout);

    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(Block::new().borders(Borders::ALL).title("SQUISH"));

    frame.render_stateful_widget(list, top, &mut app.list_state);
}

struct MenuItem {
    label: String,
    screen: Screen,
}

struct Menu {
    items: Vec<MenuItem>,
    screen: Screen,
}

struct App<'a> {
    current_screen: Screen,
    list_state: ListState,
    menu: &'a Menu,
}

#[derive(Clone, Copy)]
enum Screen {
    Home,
    Compress,
    Decompress,
    Quit,
}
