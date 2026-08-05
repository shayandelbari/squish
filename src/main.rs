mod app;

use crossterm::event::{ Event, KeyCode };
use ratatui::{
    DefaultTerminal,
    Frame,
    layout::{ Constraint, Layout },
    style::{ Color, Modifier },
    widgets::{ Block, Borders, List, ListState },
};

use crate::{ Action::{ Open, Quit }, app::{ App, Screen } };

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn init() -> App {
    let home_menu = Menu {
        items: vec![
            MenuItem {
                label: "Compress".into(),
                action: Open(Screen::Compress),
            },
            MenuItem {
                label: "Decompress".into(),
                action: Open(Screen::Decompress),
            },
            MenuItem {
                label: "Inspect".into(),
                action: Open(Screen::Inspect),
            },
            MenuItem {
                label: "Quit".into(),
                action: Quit,
            }
        ],
        list_state: ListState::default().with_selected(Some(0)),
    };

    let compress_menu = Menu {
        items: vec![
            MenuItem {
                label: "Continue".into(),
                action: Open(Screen::Home),
            },
            MenuItem {
                label: "Change algorithm".into(),
                action: Open(Screen::Home),
            },
            MenuItem {
                label: "Compress options".into(),
                action: Open(Screen::Home),
            }
        ],
        list_state: ListState::default().with_selected(Some(0)),
    };

    let decompress_menu = Menu {
        items: vec![
            MenuItem {
                label: "Path".into(),
                action: Open(Screen::Home),
            },
            MenuItem {
                label: "Quit".into(),
                action: Quit,
            }
        ],
        list_state: ListState::default().with_selected(Some(0)),
    };

    let mut app = App::new();

    app.insert_menu(home_menu, Screen::Home);
    app.insert_menu(compress_menu, Screen::Compress);
    app.insert_menu(decompress_menu, Screen::Decompress);

    app
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = init();

    loop {
        terminal.draw(|frame| render(frame, &mut app.current_menu()))?;
        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => app.current_menu().list_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => app.current_menu().list_state.select_previous(),
                KeyCode::Char('q') => {
                    break Ok(());
                }
                KeyCode::Esc => {
                    app.back();
                }
                KeyCode::Enter => {
                    if let Some(index) = app.current_menu().list_state.selected() {
                        match app.current_menu().items[index].action {
                            Open(screen) => {
                                app.open(screen);
                            }
                            Quit => {
                                break Ok(());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, menu: &mut Menu) {
    let items = menu.items.iter().map(|item| item.label.as_str());

    let layout = Layout::vertical([Constraint::Percentage(100)]).margin(1);
    let [top] = frame.area().layout(&layout);

    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(Block::new().borders(Borders::ALL).title("SQUISH"));

    frame.render_stateful_widget(list, top, &mut menu.list_state);
}

struct MenuItem {
    label: String,
    action: Action,
}

struct Menu {
    items: Vec<MenuItem>,
    list_state: ListState,
}

enum Action {
    Open(Screen),
    Quit,
}
