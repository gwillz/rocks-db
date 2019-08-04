extern crate iui;
extern crate clipboard;

use std::rc::Rc;
use std::cell::RefCell;
use iui::prelude::*;
use iui::controls::{Label, MultilineEntry, Button, Spacer, HorizontalBox, VerticalBox};
use clipboard::{ClipboardProvider, ClipboardContext};

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
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    
    let output_label = Label::new(&ui, "...");
    let mut input_entry = MultilineEntry::new(&ui);
    let mut action_button = Button::new(&ui, "Go");
    let mut copy_button = Button::new(&ui, "Copy");
    
    let mut layout = VerticalBox::new(&ui);
    layout.append(&ui, input_entry.clone(), LayoutStrategy::Stretchy);
    layout.append(&ui, output_label.clone(), LayoutStrategy::Compact);
    layout.append(&ui, Spacer::new(&ui), LayoutStrategy::Stretchy);
    
    let mut bbox = HorizontalBox::new(&ui);
    bbox.append(&ui, action_button.clone(), LayoutStrategy::Stretchy);
    bbox.append(&ui, Spacer::new(&ui), LayoutStrategy::Stretchy);
    bbox.append(&ui, copy_button.clone(), LayoutStrategy::Stretchy);
    
    layout.append(&ui, bbox.clone(), LayoutStrategy::Compact);
    
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
    
    input_entry.on_changed(&ui, {
        let state = state.clone();
        move |val| {
            state.borrow_mut().input = val;
        }
    });
    
    action_button.on_clicked(&ui, {
        let state = state.clone();
        move |_| {
            let mut state = state.borrow_mut();
            state.output = state.db.replace(&state.input);
        }
    });
    
    copy_button.on_clicked(&ui, {
        let state = state.clone();
        move |_| {
            let state = state.borrow();
            clipboard.set_contents(state.output.to_owned()).unwrap();
        }
    });
    
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, {
        let ui = ui.clone();
        let mut output_label = output_label.clone();
        
        move || {
            let state = state.borrow();
            output_label.set_text(&ui, &format!("{}", state.output));
        }
    });
    
    event_loop.run(&ui);
}
