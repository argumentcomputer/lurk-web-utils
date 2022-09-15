use rustyline::Editor;

use wasm_bindgen::prelude::*;

use crate::lurk_eval::LurkRepl;

use wasm_bindgen::JsCast;
// use xterm_js_rs::addons::fit::FitAddon;
use xterm_js_rs::{OnKeyEvent, Terminal, TerminalOptions, Theme};

const PROMPT: &str = "$ ";

fn prompt(term: &Terminal) {
  term.writeln("");
  term.write(PROMPT);
}

// Keyboard keys
// https://notes.burke.libbey.me/ansi-escape-codes/
const KEY_ENTER: u32 = 13;
const KEY_BACKSPACE: u32 = 8;
const KEY_LEFT_ARROW: u32 = 37;
const KEY_RIGHT_ARROW: u32 = 39;
const KEY_C: u32 = 67;
const KEY_L: u32 = 76;
const KEY_UP_ARROW: u32 = 38;
const KEY_DOWN_ARROW: u32 = 40;

const CURSOR_LEFT: &str = "\x1b[D";
const CURSOR_RIGHT: &str = "\x1b[C";

//const CURSOR_LEFT: &str = "\x03";


#[wasm_bindgen]
pub fn web_repl() -> Result<(), JsValue> {
  let terminal: Terminal = Terminal::new(
    TerminalOptions::new()
      .with_rows(50)
      .with_cursor_blink(true)
      .with_cursor_width(10)
      .with_font_size(20)
      .with_draw_bold_text_in_bright_colors(true)
      .with_right_click_selects_word(true)
      .with_theme(
        Theme::new()
          .with_foreground("#98FB98")
          .with_background("#000000"),
      ),
  );

  let elem = web_sys::window()
    .unwrap()
    .document()
    .unwrap()
    .get_element_by_id("terminal")
    .unwrap();

  terminal.writeln("Supported keys in this example: <Printable-Characters> <Enter> <Backspace> <Left-Arrow> <Right-Arrow> <Up-Arrow> <Down-Arrow> <Ctrl-C> <Ctrl-L>");
  terminal.open(elem.dyn_into()?);
  prompt(&terminal);

  let mut line = String::new();
  let mut cursor_col = 0;

  let term: Terminal = terminal.clone().dyn_into()?;

  let mut rl = Editor::<()>::new().unwrap();
  let mut idx = rl.history().len();

  let mut lurk_repl = LurkRepl::new();

  // Editing in place currently not supported
  let callback = Closure::wrap(Box::new(move |e: OnKeyEvent| {
    let event = e.dom_event();
    match event.key_code() {
      KEY_ENTER => {
        if !line.is_empty() {
          term.writeln("");
          rl.add_history_entry(line.as_str().trim());
          idx = rl.history().len();
          term.writeln(&lurk_repl.execute_lurk(&line).unwrap());
          line.clear();
          cursor_col = 0;
        }
        prompt(&term);
      }
      KEY_BACKSPACE => {
        if cursor_col > 0 {
          term.write("\u{0008} \u{0008}");
          line.pop();
          cursor_col -= 1;
        }
      }
      KEY_LEFT_ARROW => {
        if cursor_col > 0 {
          term.write(CURSOR_LEFT);
          cursor_col -= 1;
        }
      }
      KEY_RIGHT_ARROW => {
        if cursor_col < line.len() {
          term.write(CURSOR_RIGHT);
          cursor_col += 1;
        }
      }
      KEY_L if event.ctrl_key() => term.clear(),
      KEY_C if event.ctrl_key() => {
        prompt(&term);
        line.clear();
        cursor_col = 0;
      }
      KEY_UP_ARROW => {
        if idx == 0 {
          // Do nothing
        } else {
          idx -= 1;
          line = rl.history()[idx].clone();
          term.write("\x1b[2K\r$ ");
          term.write(&line);
          cursor_col = line.len();
        }
      }
      KEY_DOWN_ARROW => {
        term.write("\x1b[2K\r$ ");
        line.clear();
        if idx >= rl.history().len() {
          // Do nothing
        } else {
          idx += 1;
          if idx != rl.history().len() {
            line = rl.history()[idx].clone();
            term.write(&line);
            cursor_col = line.len();
          }
        }
      }
      _ => {
        if !event.alt_key() && !event.alt_key() && !event.ctrl_key() && !event.meta_key() {
          term.write(&event.key());
          line.push_str(&e.key());
          cursor_col += 1;
        }
      }
    }
  }) as Box<dyn FnMut(_)>);

  terminal.on_key(callback.as_ref().unchecked_ref());

  callback.forget();

  //let addon = FitAddon::new();
  //terminal.load_addon(addon.clone().dyn_into::<FitAddon>()?.into());
  //addon.fit();
  terminal.focus();

  Ok(())
}
