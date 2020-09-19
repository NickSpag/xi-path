use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io;
use std::mem;
use std::path::{Path, PathBuf};

use sdl2::keyboard::Keycode;

use xi_core_lib::{ 
    ViewId, 
    BufferId,
    view::View, 
    editor::Editor,
    tabs::Counter,
    file:: {FileManager, FileError },
    event_context::EventContext, 
    config::ConfigManager, 
    styles::ThemeStyleMap, 
    width_cache::WidthCache 
};

use xi_rope::Rope;



pub struct Session {
    frontend: XiPathFrontend,
    backend: XiCore,
}

impl Session {
    pub fn new() -> Self {
        let config_dir = PathBuf::from("/Users/nickspagnola/Development/Projects/xi-path/dev/config");
        let extra_dir = PathBuf::from(todo!());

        let frontend = XiPathFrontend::new_with_pathfinder_renderer(); 
        Session {
            frontend,
            backend: XiCore::new_direct(frontend, Some(config_dir), Some(extras_dir))
        }
    }
}

// view-related
impl Session {
    pub fn add_new_view(&mut self, path: Option<PathBuf>) -> Result<ViewId, FileError> {
        let view_id = self.next_view_id();
        let buffer_id = self.next_buffer_id();

        let rope = match path.as_ref() {
            Some(p) => self.file_manager.open(p, buffer_id)?,
            None => Rope::from(""),
        };

        let editor = RefCell::new(Editor::with_text(rope));
        let view = RefCell::new(View::new(view_id, buffer_id));

        self.editors.insert(buffer_id, editor);
        self.views.insert(view_id, view);

        Ok(view_id)
    }

    pub fn insert(&mut self, view_id: &ViewId, keycode: Keycode) {
        let insertable_key = match keycode {
            Keycode::A => keycode,
            //Keycode::Backspace => {}
            //Keycode::Tab => {}
            // Keycode::Return => {}
            // Keycode::Escape => {}
            Keycode::Space => keycode,
            Keycode::Exclaim => keycode,
            Keycode::Quotedbl => keycode,
            Keycode::Hash => keycode,
            Keycode::Dollar => keycode,
            Keycode::Percent => keycode,
            Keycode::Ampersand => keycode,
            Keycode::Quote => keycode,
            Keycode::LeftParen => keycode,
            Keycode::RightParen => keycode,
            Keycode::Asterisk => keycode,
            Keycode::Plus => keycode,
            Keycode::Comma => keycode,
            Keycode::Minus => keycode,
            Keycode::Period => keycode,
            Keycode::Slash => keycode,
            Keycode::Num0 => keycode,
            Keycode::Num1 => keycode,
            Keycode::Num2 => keycode,
            Keycode::Num3 => keycode,
            Keycode::Num4 => keycode,
            Keycode::Num5 => keycode,
            Keycode::Num6 => keycode,
            Keycode::Num7 => keycode,
            Keycode::Num8 => keycode,
            Keycode::Num9 => keycode,
            Keycode::Colon => keycode,
            Keycode::Semicolon => keycode,
            Keycode::Less => keycode,
            Keycode::Equals => keycode,
            Keycode::Greater => keycode,
            Keycode::Question => keycode,
            Keycode::At => keycode,
            Keycode::LeftBracket => keycode,
            Keycode::Backslash => keycode,
            Keycode::RightBracket => keycode,
            Keycode::Caret => keycode,
            Keycode::Underscore => keycode,
            Keycode::Backquote => keycode,
            Keycode::B => keycode,
            Keycode::C => keycode,
            Keycode::D => keycode,
            Keycode::E => keycode,
            Keycode::F => keycode,
            Keycode::G => keycode,
            Keycode::H => keycode,
            Keycode::I => keycode,
            Keycode::J => keycode,
            Keycode::K => keycode,
            Keycode::L => keycode,
            Keycode::M => keycode,
            Keycode::N => keycode,
            Keycode::O => keycode,
            Keycode::P => keycode,
            Keycode::Q => keycode,
            Keycode::R => keycode,
            Keycode::S => keycode,
            Keycode::T => keycode,
            Keycode::U => keycode,
            Keycode::V => keycode,
            Keycode::W => keycode,
            Keycode::X => keycode,
            Keycode::Y => keycode,
            Keycode::Z => keycode,
            // Keycode::Delete => {}
            // Keycode::CapsLock => {}
            // Keycode::F1 => {}
            // Keycode::F2 => {}
            // Keycode::F3 => {}
            // Keycode::F4 => {}
            // Keycode::F5 => {}
            // Keycode::F6 => {}
            // Keycode::F7 => {}
            // Keycode::F8 => {}
            // Keycode::F9 => {}
            // Keycode::F10 => {}
            // Keycode::F11 => {}
            // Keycode::F12 => {}
            // Keycode::PrintScreen => {}
            // Keycode::ScrollLock => {}
            // Keycode::Pause => {}
            // Keycode::Insert => {}
            // Keycode::Home => {}
            // Keycode::PageUp => {}
            // Keycode::End => {}
            // Keycode::PageDown => {}
            // Keycode::Right => {}
            // Keycode::Left => {}
            // Keycode::Down => {}
            // Keycode::Up => {}
            // Keycode::NumLockClear => {}
            // Keycode::KpDivide => {}
            // Keycode::KpMultiply => {}
            // Keycode::KpMinus => {}
            // Keycode::KpPlus => {}
            // Keycode::KpEnter => {}
            // Keycode::Kp1 => {}
            // Keycode::Kp2 => {}
            // Keycode::Kp3 => {}
            // Keycode::Kp4 => {}
            // Keycode::Kp5 => {}
            // Keycode::Kp6 => {}
            // Keycode::Kp7 => {}
            // Keycode::Kp8 => {}
            // Keycode::Kp9 => {}
            // Keycode::Kp0 => {}
            // Keycode::KpPeriod => {}
            // Keycode::Application => {}
            // Keycode::Power => {}
            // Keycode::KpEquals => {}
            // Keycode::F13 => {}
            // Keycode::F14 => {}
            // Keycode::F15 => {}
            // Keycode::F16 => {}
            // Keycode::F17 => {}
            // Keycode::F18 => {}
            // Keycode::F19 => {}
            // Keycode::F20 => {}
            // Keycode::F21 => {}
            // Keycode::F22 => {}
            // Keycode::F23 => {}
            // Keycode::F24 => {}
            // Keycode::Execute => {}
            // Keycode::Help => {}
            // Keycode::Menu => {}
            // Keycode::Select => {}
            // Keycode::Stop => {}
            // Keycode::Again => {}
            // Keycode::Undo => {}
            // Keycode::Cut => {}
            // Keycode::Copy => {}
            // Keycode::Paste => {}
            // Keycode::Find => {}
            // Keycode::Mute => {}
            // Keycode::VolumeUp => {}
            // Keycode::VolumeDown => {}
            // Keycode::KpComma => {}
            // Keycode::KpEqualsAS400 => {}
            // Keycode::AltErase => {}
            // Keycode::Sysreq => {}
            // Keycode::Cancel => {}
            // Keycode::Clear => {}
            // Keycode::Prior => {}
            // Keycode::Return2 => {}
            // Keycode::Separator => {}
            // Keycode::Out => {}
            // Keycode::Oper => {}
            // Keycode::ClearAgain => {}
            // Keycode::CrSel => {}
            // Keycode::ExSel => {}
            // Keycode::Kp00 => {}
            // Keycode::Kp000 => {}
            // Keycode::ThousandsSeparator => {}
            // Keycode::DecimalSeparator => {}
            // Keycode::CurrencyUnit => {}
            // Keycode::CurrencySubUnit => {}
            // Keycode::KpLeftParen => {}
            // Keycode::KpRightParen => {}
            // Keycode::KpLeftBrace => {}
            // Keycode::KpRightBrace => {}
            // Keycode::KpTab => {}
            // Keycode::KpBackspace => {}
            // Keycode::KpA => {}
            // Keycode::KpB => {}
            // Keycode::KpC => {}
            // Keycode::KpD => {}
            // Keycode::KpE => {}
            // Keycode::KpF => {}
            // Keycode::KpXor => {}
            // Keycode::KpPower => {}
            // Keycode::KpPercent => {}
            // Keycode::KpLess => {}
            // Keycode::KpGreater => {}
            // Keycode::KpAmpersand => {}
            // Keycode::KpDblAmpersand => {}
            // Keycode::KpVerticalBar => {}
            // Keycode::KpDblVerticalBar => {}
            // Keycode::KpColon => {}
            // Keycode::KpHash => {}
            // Keycode::KpSpace => {}
            // Keycode::KpAt => {}
            // Keycode::KpExclam => {}
            // Keycode::KpMemStore => {}
            // Keycode::KpMemRecall => {}
            // Keycode::KpMemClear => {}
            // Keycode::KpMemAdd => {}
            // Keycode::KpMemSubtract => {}
            // Keycode::KpMemMultiply => {}
            // Keycode::KpMemDivide => {}
            // Keycode::KpPlusMinus => {}
            // Keycode::KpClear => {}
            // Keycode::KpClearEntry => {}
            // Keycode::KpBinary => {}
            // Keycode::KpOctal => {}
            // Keycode::KpDecimal => {}
            // Keycode::KpHexadecimal => {}
            // Keycode::LCtrl => {}
            // Keycode::LShift => {}
            // Keycode::LAlt => {}
            // Keycode::LGui => {}
            // Keycode::RCtrl => {}
            // Keycode::RShift => {}
            // Keycode::RAlt => {}
            // Keycode::RGui => {}
            // Keycode::Mode => {}
            // Keycode::AudioNext => {}
            // Keycode::AudioPrev => {}
            // Keycode::AudioStop => {}
            // Keycode::AudioPlay => {}
            // Keycode::AudioMute => {}
            // Keycode::MediaSelect => {}
            // Keycode::Www => {}
            // Keycode::Mail => {}
            // Keycode::Calculator => {}
            // Keycode::Computer => {}
            // Keycode::AcSearch => {}
            // Keycode::AcHome => {}
            // Keycode::AcBack => {}
            // Keycode::AcForward => {}
            // Keycode::AcStop => {}
            // Keycode::AcRefresh => {}
            // Keycode::AcBookmarks => {}
            // Keycode::BrightnessDown => {}
            // Keycode::BrightnessUp => {}
            // Keycode::DisplaySwitch => {}
            // Keycode::KbdIllumToggle => {}
            // Keycode::KbdIllumDown => {}
            // Keycode::KbdIllumUp => {}
            // Keycode::Eject => {}
            // Keycode::Sleep => {}
            _ => return,
        };

        println!("{}", insertable_key);
    }

    pub fn make_context(&self, view_id: ViewId) -> Option<EventContext> {
        self.views.get(&view_id).map(|view| {
            let buffer_id = view.borrow().get_buffer_id();

            let editor = &self.editors[&buffer_id];
            let info = self.file_manager.get_info(buffer_id);

            //let plugins =  self.running_plugins.iter().collect::<Vec<_>>();
            let config = self.config_manager.get_buffer_config(buffer_id);
            let language = self.config_manager.get_buffer_language(buffer_id);

            EventContext {
                view_id,
                buffer_id,
                view,
                editor,
                config: &config.items,
                recorder: &self.recorder,
                language,
                info,
                siblings: Vec::new(),
                plugins: Vec::new(),
                client: &self.peer,
                style_map: &self.style_map,
                width_cache: &self.width_cache,
                kill_ring: &self.kill_ring,
                weak_core: self.self_ref.as_ref().unwrap(),
            }
        })
    }
}