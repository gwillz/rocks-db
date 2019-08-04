extern crate iui;

use iui::prelude::*;
use iui::controls::{Label, MultilineEntry, Button, Spacer, VerticalBox};
use std::rc::Rc;
use std::cell::RefCell;

#[path="../lib.rs"]
mod lib;
use lib::RockDB;

const DB_PATH: &str = "description-database.txt";

struct State {
    input: String,
    output: String,
    db: RockDB,
}

fn main() {
    let ui = UI::init().unwrap();
    
    let mut db = RockDB::new();
    
    let label = Label::new(&ui, "...");
    let mut input = MultilineEntry::new(&ui);
    let mut button = Button::new(&ui, "Go");
    
    let mut layout = VerticalBox::new(&ui);
    
    layout.append(&ui, input.clone(), LayoutStrategy::Stretchy);
    layout.append(&ui, label.clone(), LayoutStrategy::Compact);
    layout.append(&ui, Spacer::new(&ui), LayoutStrategy::Stretchy);
    layout.append(&ui, button.clone(), LayoutStrategy::Compact);
    
    let mut window = Window::new(&ui,
        "Rocks DB", 400, 300, WindowType::NoMenubar);
    
    window.set_child(&ui, layout);
    window.show(&ui);
    
    match db.load(DB_PATH) {
        Ok(_) => {},
        Err(e) => {
            window.modal_err(&ui, "Error", format!("{}", e).as_ref());
            ui.quit();
        }
    };
    
    let state = Rc::new(RefCell::new(State {
        input: "".into(),
        output: "".into(),
        db: db,
    }));
    
    input.on_changed(&ui, {
        let state = state.clone();
        move |val| {
            state.borrow_mut().input = val;
        }
    });
    
    button.on_clicked(&ui, {
        let state = state.clone();
        move |_| {
            let mut state = state.borrow_mut();
            state.output = state.db.replace(&state.input);
        }
    });
    
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, {
        let ui = ui.clone();
        let mut label = label.clone();
        
        move || {
            let state = state.borrow();
            label.set_text(&ui, &format!("{}", state.output));
        }
    });
    
    event_loop.run(&ui);
}
