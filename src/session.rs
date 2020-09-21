use std::{path::PathBuf};
use sdl2::{keyboard::Keycode, Sdl};

use xi_core_lib::{ ViewId, XiCore};

use frontend::frontend::{XiPathFrontend};

pub struct Session {
    frontend: XiPathFrontend,
    backend: XiCore,
}

impl Session {
    pub fn new(sdl_context: &Sdl) -> Self {
        let config_dir = PathBuf::from("/Users/nickspagnola/Development/Projects/xi-path/dev/config");
        let extra_dir = PathBuf::from("todo");

        let frontend = XiPathFrontend::new_with_pathfinder_renderer(&sdl_context); 
        let clone = frontend.clone();
        Session {
            frontend,
            backend: XiCore::new_direct(Box::new(clone), Some(config_dir), Some(extra_dir))
        }
    }
}

// view-related
impl Session {

    pub (crate) fn first_render(&self) {
        self.frontend.first_render();
    }

    pub (crate) fn add_new_view(&mut self, path: Option<PathBuf>) -> ViewId {
        match self.backend.new_view(path) {
            Ok(v) => v,
            Err(e) => panic!("new_view errored out"),
        }
    }

    pub (crate) fn insert(&mut self, view_id: &ViewId, keycode: Keycode) {
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

        //todo tell backend with viewId there was an insert
        println!("{}", insertable_key);
    }
}