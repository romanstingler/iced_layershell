use iced_core::keyboard;
use iced_core::mouse;
use smithay_client_toolkit::seat::keyboard::{Keysym, Modifiers};

pub(crate) fn modifiers_to_iced(mods: Modifiers) -> keyboard::Modifiers {
    let mut result = keyboard::Modifiers::empty();
    if mods.shift {
        result |= keyboard::Modifiers::SHIFT;
    }
    if mods.ctrl {
        result |= keyboard::Modifiers::CTRL;
    }
    if mods.alt {
        result |= keyboard::Modifiers::ALT;
    }
    if mods.logo {
        result |= keyboard::Modifiers::LOGO;
    }
    result
}

pub(crate) fn pointer_button_to_iced(button: u32) -> mouse::Button {
    match button {
        0x110 => mouse::Button::Left,   // BTN_LEFT
        0x111 => mouse::Button::Right,  // BTN_RIGHT
        0x112 => mouse::Button::Middle, // BTN_MIDDLE
        other => mouse::Button::Other(other as u16),
    }
}

/// Convert keysym to iced Key. This returns the BASE key (unmodified),
/// used for the `key` field in KeyPressed. The keysym represents the
/// logical key regardless of Ctrl/Alt modifiers.
pub(crate) fn keysym_to_iced_key(keysym: Keysym) -> keyboard::Key {
    if let Some(named) = keysym_to_named(keysym) {
        return keyboard::Key::Named(named);
    }
    // For ASCII-range keysyms, convert directly to character.
    // This ensures Ctrl+C produces Key::Character("c"), not Key::Character("\x03").
    let raw = keysym.raw();
    if (0x20..=0x7e).contains(&raw) {
        return keyboard::Key::Character(
            String::from(char::from(raw as u8)).into(),
        );
    }
    // For non-ASCII keysyms (e.g. accented characters), try xkb mapping
    if let Some(c) = keysym_to_char(raw) {
        return keyboard::Key::Character(String::from(c).into());
    }
    keyboard::Key::Unidentified
}

fn keysym_to_named(keysym: Keysym) -> Option<keyboard::key::Named> {
    use keyboard::key::Named;

    // XKB keysym constants
    Some(match keysym.raw() {
        // Function keys
        0xff08 => Named::Backspace,
        0xff09 => Named::Tab,
        0xff0d => Named::Enter,
        0xff1b => Named::Escape,
        0xffff => Named::Delete,

        // Navigation
        0xff50 => Named::Home,
        0xff51 => Named::ArrowLeft,
        0xff52 => Named::ArrowUp,
        0xff53 => Named::ArrowRight,
        0xff54 => Named::ArrowDown,
        0xff55 => Named::PageUp,
        0xff56 => Named::PageDown,
        0xff57 => Named::End,

        // Modifiers
        0xffe1 | 0xffe2 => Named::Shift,
        0xffe3 | 0xffe4 => Named::Control,
        0xffe5 => Named::CapsLock,
        0xffe7 | 0xffe8 => Named::Super,
        0xffe9 | 0xffea => Named::Alt,

        // Editing
        0xff63 => Named::Insert,

        // Whitespace
        0x0020 => Named::Space,

        // Function keys F1-F12
        0xffbe => Named::F1,
        0xffbf => Named::F2,
        0xffc0 => Named::F3,
        0xffc1 => Named::F4,
        0xffc2 => Named::F5,
        0xffc3 => Named::F6,
        0xffc4 => Named::F7,
        0xffc5 => Named::F8,
        0xffc6 => Named::F9,
        0xffc7 => Named::F10,
        0xffc8 => Named::F11,
        0xffc9 => Named::F12,

        // Media keys
        0x1008ff11 => Named::AudioVolumeDown,
        0x1008ff12 => Named::AudioVolumeMute,
        0x1008ff13 => Named::AudioVolumeUp,
        0x1008ff14 => Named::MediaPlayPause,
        0x1008ff15 => Named::MediaStop,
        0x1008ff16 => Named::MediaTrackPrevious,
        0x1008ff17 => Named::MediaTrackNext,

        // Misc
        0xff61 => Named::PrintScreen,
        0xff13 => Named::Pause,
        0xff14 => Named::ScrollLock,
        0xff67 => Named::ContextMenu,
        0xff7f => Named::NumLock,

        _ => return None,
    })
}

/// Convert XKB keysym to Unicode character (for non-ASCII keysyms).
fn keysym_to_char(keysym: u32) -> Option<char> {
    // Unicode keysyms: 0x01000000 + codepoint
    if keysym >= 0x01000000 {
        return char::from_u32(keysym - 0x01000000);
    }
    // Latin-1 supplement: keysyms 0xa0-0xff map to Unicode U+00A0-U+00FF
    if (0xa0..=0xff).contains(&keysym) {
        return char::from_u32(keysym);
    }
    None
}

/// Get the text that should be inserted for a key event.
/// Filters out control characters that shouldn't produce text input.
pub(crate) fn key_utf8_to_text(utf8: Option<&str>) -> Option<iced_core::SmolStr> {
    utf8.filter(|s| !s.is_empty() && s.chars().all(|c| !c.is_control()))
        .map(iced_core::SmolStr::from)
}
