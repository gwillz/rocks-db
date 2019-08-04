extern crate iui;
extern crate clipboard;

use std::rc::Rc;
use std::cell::RefCell;
use iui::prelude::*;
use iui::controls::{Label, MultilineEntry, Button, Spacer};
use iui::controls::{HorizontalBox, VerticalBox};
use clipboard::{ClipboardProvider, ClipboardContext};

#[path="../lib.rs"]
mod lib;
use lib::RockDB;

// Move this to a config? arg parameter?
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
    
    // Widgets.
    let output_label = Label::new(&ui, "...");
    let mut input_entry = MultilineEntry::new(&ui);
    let mut action_button = Button::new(&ui, "Go");
    let mut copy_button = Button::new(&ui, "Copy");
    
    // Button box.
    let mut bbox = HorizontalBox::new(&ui);
    bbox.append(&ui, action_button.clone(), LayoutStrategy::Stretchy);
    bbox.append(&ui, Spacer::new(&ui), LayoutStrategy::Stretchy);
    bbox.append(&ui, copy_button.clone(), LayoutStrategy::Stretchy);
    
    // Layout.
    let mut layout = VerticalBox::new(&ui);
    layout.append(&ui, input_entry.clone(), LayoutStrategy::Stretchy);
    layout.append(&ui, output_label.clone(), LayoutStrategy::Compact);
    layout.append(&ui, Spacer::new(&ui), LayoutStrategy::Stretchy);
    layout.append(&ui, bbox.clone(), LayoutStrategy::Compact);
    
    // Window frame.
    let mut window = Window::new(&ui,
        "Rocks DB", 600, 450, WindowType::NoMenubar);
    window.set_child(&ui, layout);
    window.show(&ui);
    
    // Load up the database, error dialog on failure.
    match db.load(DB_PATH) {
        Ok(_) => {},
        Err(e) => {
            window.modal_err(&ui, "Error", format!("{}", e).as_ref());
            ui.quit();
        }
    };
    
    // Initialise working variables.
    let state = Rc::new(RefCell::new(State {
        input: "".into(),
        output: "".into(),
        db: db,
    }));
    
    // Event listeners.
    
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
            state.output = state.db.convert(&state.input);
        }
    });
    
    copy_button.on_clicked(&ui, {
        let state = state.clone();
        move |_| {
            let state = state.borrow();
            clipboard.set_contents(state.output.to_owned()).unwrap();
        }
    });
    
    
    // Start event loop.
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, {
        let ui = ui.clone();
        let mut output_label = output_label.clone();
        
        // Update output label.
        move || {
            let state = state.borrow();
            output_label.set_text(&ui, &format!("{}", state.output));
        }
    });
    event_loop.run(&ui);
}
