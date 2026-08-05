use std::collections::HashMap;

use crate::Menu;

pub struct App {
    menus: HashMap<Screen, Menu>,
    screens: Vec<Screen>,
}

impl App {
    pub fn new() -> App {
        App {
            menus: HashMap::new(),
            screens: vec![Screen::Home],
        }
    }

    pub fn back(&mut self) -> bool {
        if self.screens.len() > 1 {
            self.screens.pop();
            true
        } else {
            false
        }
    }

    pub fn open(&mut self, screen: Screen) {
        self.screens.push(screen);
    }

    pub fn current_menu(&mut self) -> &mut Menu {
        return self.menus.get_mut(&self.screens.last().unwrap()).unwrap();
    }

    pub fn insert_menu(&mut self, menu: Menu, screen: Screen) {
        self.menus.insert(screen, menu);
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Screen {
    Home,
    Compress,
    Decompress,
    Inspect,
}
