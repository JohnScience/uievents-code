#![doc=include_str!("../README.md")]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

/// Writing System Keys as defined in
/// <https://www.w3.org/TR/2017/CR-uievents-code-20170601/#key-alphanumeric-writing-system>.
pub mod writing_system {
    pub const BACKQUOTE: &'static str = "Backquote";
    pub const BACKSLASH: &'static str = "Backslash";
    pub const BACKSPACE: &'static str = "Backspace";
    pub const BRACKET_LEFT: &'static str = "BracketLeft";
    pub const BRACKET_RIGHT: &'static str = "BracketRight";
    pub const COMMA: &'static str = "Comma";
    pub const DIGIT0: &'static str = "Digit0";
    pub const DIGIT1: &'static str = "Digit1";
    pub const DIGIT2: &'static str = "Digit2";
    pub const DIGIT3: &'static str = "Digit3";
    pub const DIGIT4: &'static str = "Digit4";
    pub const DIGIT5: &'static str = "Digit5";
    pub const DIGIT6: &'static str = "Digit6";
    pub const DIGIT7: &'static str = "Digit7";
    pub const DIGIT8: &'static str = "Digit8";
    pub const DIGIT9: &'static str = "Digit9";
    pub const EQUAL: &'static str = "Equal";
    pub const INTL_BACKSLASH: &'static str = "IntlBackslash";
    pub const INTL_RO: &'static str = "IntlRo";
    pub const INTL_YEN: &'static str = "IntlYen";
    pub const KEY_A: &'static str = "KeyA";
    pub const KEY_B: &'static str = "KeyB";
    pub const KEY_C: &'static str = "KeyC";
    pub const KEY_D: &'static str = "KeyD";
    pub const KEY_E: &'static str = "KeyE";
    pub const KEY_F: &'static str = "KeyF";
    pub const KEY_G: &'static str = "KeyG";
    pub const KEY_H: &'static str = "KeyH";
    pub const KEY_I: &'static str = "KeyI";
    pub const KEY_J: &'static str = "KeyJ";
    pub const KEY_K: &'static str = "KeyK";
    pub const KEY_L: &'static str = "KeyL";
    pub const KEY_M: &'static str = "KeyM";
    pub const KEY_N: &'static str = "KeyN";
    pub const KEY_O: &'static str = "KeyO";
    pub const KEY_P: &'static str = "KeyP";
    pub const KEY_Q: &'static str = "KeyQ";
    pub const KEY_R: &'static str = "KeyR";
    pub const KEY_S: &'static str = "KeyS";
    pub const KEY_T: &'static str = "KeyT";
    pub const KEY_U: &'static str = "KeyU";
    pub const KEY_V: &'static str = "KeyV";
    pub const KEY_W: &'static str = "KeyW";
    pub const KEY_X: &'static str = "KeyX";
    pub const KEY_Y: &'static str = "KeyY";
    pub const KEY_Z: &'static str = "KeyZ";
    pub const MINUS: &'static str = "Minus";
    pub const PERIOD: &'static str = "Period";
    pub const QUOTE: &'static str = "Quote";
    pub const SEMICOLON: &'static str = "Semicolon";
    pub const SLASH: &'static str = "Slash";
}
pub use writing_system::*;

/// Functional Keys as defined in
/// <https://www.w3.org/TR/2017/CR-uievents-code-20170601/#key-alphanumeric-functional>.
pub mod functional {
    pub mod alphanumeric {
        pub const ALT_LEFT: &'static str = "AltLeft";
        pub const ALT_RIGHT: &'static str = "AltRight";
        pub const CAPS_LOCK: &'static str = "CapsLock";
        pub const CONTEXT_MENU: &'static str = "ContextMenu";
        pub const CONTROL_LEFT: &'static str = "ControlLeft";
        pub const CONTROL_RIGHT: &'static str = "ControlRight";
        pub const ENTER: &'static str = "Enter";
        pub const META_LEFT: &'static str = "MetaLeft";
        pub const META_RIGHT: &'static str = "MetaRight";
        pub const SHIFT_LEFT: &'static str = "ShiftLeft";
        pub const SHIFT_RIGHT: &'static str = "ShiftRight";
        pub const SPACE: &'static str = "Space";
        pub const TAB: &'static str = "Tab";
    }
    pub use alphanumeric::*;

    pub mod japanese_and_korean {
        pub const CONVERT: &'static str = "Convert";
        pub const KANA_MODE: &'static str = "KanaMode";
        pub const LANG1: &'static str = "Lang1";
        pub const LANG2: &'static str = "Lang2";
        pub const LANG3: &'static str = "Lang3";
        pub const LANG4: &'static str = "Lang4";
        pub const LANG5: &'static str = "Lang5";
        pub const NON_CONVERT: &'static str = "NonConvert";
    }
    pub use japanese_and_korean::*;
}

/// Control Pad Keys as defined in
/// <https://www.w3.org/TR/2017/CR-uievents-code-20170601/#key-controlpad-section>.
pub mod control_pad {
    pub const DELETE: &'static str = "Delete";
    pub const END: &'static str = "End";
    pub const HELP: &'static str = "Help";
    pub const HOME: &'static str = "Home";
    pub const INSERT: &'static str = "Insert";
    pub const PAGE_DOWN: &'static str = "PageDown";
    pub const PAGE_UP: &'static str = "PageUp";
}
pub use self::control_pad::*;

/// Arrow Pad Keys as defined in
/// <https://www.w3.org/TR/2017/CR-uievents-code-20170601/#key-arrowpad-section>
pub mod arrow_pad {
    pub const ARROW_DOWN: &'static str = "ArrowDown";
    pub const ARROW_LEFT: &'static str = "ArrowLeft";
    pub const ARROW_RIGHT: &'static str = "ArrowRight";
    pub const ARROW_UP: &'static str = "ArrowUp";
}
pub use self::arrow_pad::*;

/// Numpad Keys as defined in
/// <https://www.w3.org/TR/2017/CR-uievents-code-20170601/#key-numpad-section>
pub mod numpad {
    pub const NUM_LOCK: &'static str = "NumLock";
    pub const NUMPAD0: &'static str = "Numpad0";
    pub const NUMPAD1: &'static str = "Numpad1";
    pub const NUMPAD2: &'static str = "Numpad2";
    pub const NUMPAD3: &'static str = "Numpad3";
    pub const NUMPAD4: &'static str = "Numpad4";
    pub const NUMPAD5: &'static str = "Numpad5";
    pub const NUMPAD6: &'static str = "Numpad6";
    pub const NUMPAD7: &'static str = "Numpad7";
    pub const NUMPAD8: &'static str = "Numpad8";
    pub const NUMPAD9: &'static str = "Numpad9";
    pub const NUMPAD_ADD: &'static str = "NumpadAdd";
    pub const NUMPAD_BACKSPACE: &'static str = "NumpadBackspace";
    pub const NUMPAD_CLEAR: &'static str = "NumpadClear";
    pub const NUMPAD_CLEAR_ENTRY: &'static str = "NumpadClearEntry";
    pub const NUMPAD_COMMA: &'static str = "NumpadComma";
    pub const NUMPAD_DECIMAL: &'static str = "NumpadDecimal";
    pub const NUMPAD_DIVIDE: &'static str = "NumpadDivide";
    pub const NUMPAD_ENTER: &'static str = "NumpadEnter";
    pub const NUMPAD_EQUAL: &'static str = "NumpadEqual";
    pub const NUMPAD_HASH: &'static str = "NumpadHash";
    pub const NUMPAD_MEMORY_ADD: &'static str = "NumpadMemoryAdd";
    pub const NUMPAD_MEMORY_CLEAR: &'static str = "NumpadMemoryClear";
    pub const NUMPAD_MEMORY_RECALL: &'static str = "NumpadMemoryRecall";
    pub const NUMPAD_MEMORY_STORE: &'static str = "NumpadMemoryStore";
    pub const NUMPAD_MEMORY_SUBTRACT: &'static str = "NumpadMemorySubtract";
    pub const NUMPAD_MULTIPLY: &'static str = "NumpadMultiply";
    pub const NUMPAD_PAREN_LEFT: &'static str = "NumpadParenLeft";
    pub const NUMPAD_PAREN_RIGHT: &'static str = "NumpadParenRight";
    pub const NUMPAD_STAR: &'static str = "NumpadStar";
    pub const NUMPAD_SUBTRACT: &'static str = "NumpadSubtract";
}
pub use self::numpad::*;

/// Function Keys as defined in
/// <https://www.w3.org/TR/2017/CR-uievents-code-20170601/#key-function-section>
pub mod function {
    pub const ESCAPE: &'static str = "Escape";
    pub const F1: &'static str = "F1";
    pub const F2: &'static str = "F2";
    pub const F3: &'static str = "F3";
    pub const F4: &'static str = "F4";
    pub const F5: &'static str = "F5";
    pub const F6: &'static str = "F6";
    pub const F7: &'static str = "F7";
    pub const F8: &'static str = "F8";
    pub const F9: &'static str = "F9";
    pub const F10: &'static str = "F10";
    pub const F11: &'static str = "F11";
    pub const F12: &'static str = "F12";
    pub const F13: &'static str = "F13";
    pub const F14: &'static str = "F14";
    pub const F15: &'static str = "F15";
    pub const F16: &'static str = "F16";
    pub const F17: &'static str = "F17";
    pub const F18: &'static str = "F18";
    pub const F19: &'static str = "F19";
    pub const FN: &'static str = "Fn";
    pub const FN_LOCK: &'static str = "FnLock";
    pub const PRINT_SCREEN: &'static str = "PrintScreen";
    pub const SCROLL_LOCK: &'static str = "ScrollLock";
    pub const PAUSE: &'static str = "Pause";
}
pub use self::function::*;

/// Media Keys as defined in
/// <https://www.w3.org/TR/2017/CR-uievents-code-20170601/#key-media>.
pub mod media {
    pub const BROWSER_BACK: &'static str = "BrowserBack";
    pub const BROWSER_FAVORITES: &'static str = "BrowserFavorites";
    pub const BROWSER_FORWARD: &'static str = "BrowserForward";
    pub const BROWSER_HOME: &'static str = "BrowserHome";
    pub const BROWSER_REFRESH: &'static str = "BrowserRefresh";
    pub const BROWSER_SEARCH: &'static str = "BrowserSearch";
    pub const BROWSER_STOP: &'static str = "BrowserStop";
    pub const EJECT: &'static str = "Eject";
    pub const LAUNCH_APP1: &'static str = "LaunchApp1";
    pub const LAUNCH_APP2: &'static str = "LaunchApp2";
    pub const LAUNCH_MAIL: &'static str = "LaunchMail";
    pub const MEDIA_PLAY_PAUSE: &'static str = "MediaPlayPause";
    pub const MEDIA_SELECT: &'static str = "MediaSelect";
    pub const MEDIA_STOP: &'static str = "MediaStop";
    pub const MEDIA_TRACK_NEXT: &'static str = "MediaTrackNext";
    pub const MEDIA_TRACK_PREVIOUS: &'static str = "MediaTrackPrevious";
    pub const POWER: &'static str = "Power";
    pub const SLEEP: &'static str = "Sleep";
    pub const AUDIO_VOLUME_DOWN: &'static str = "AudioVolumeDown";
    pub const AUDIO_VOLUME_MUTE: &'static str = "AudioVolumeMute";
    pub const AUDIO_VOLUME_UP: &'static str = "AudioVolumeUp";
    pub const WAKE_UP: &'static str = "WakeUp";
}
pub use self::media::*;

/// Legacy Keys as defined in
/// <https://www.w3.org/TR/2017/CR-uievents-code-20170601/#key-legacy>.
#[cfg(any(feature = "legacy", doc))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
pub mod legacy {
    pub const HYPER: &'static str = "Hyper";
    pub const SUPER: &'static str = "Super";
    pub const TURBO: &'static str = "Turbo";
    pub const ABORT: &'static str = "Abort";
    pub const RESUME: &'static str = "Resume";
    pub const SUSPEND: &'static str = "Suspend";
    pub const AGAIN: &'static str = "Again";
    pub const COPY: &'static str = "Copy";
    pub const CUT: &'static str = "Cut";
    pub const FIND: &'static str = "Find";
    pub const OPEN: &'static str = "Open";
    pub const PASTE: &'static str = "Paste";
    pub const PROPS: &'static str = "Props";
    pub const SELECT: &'static str = "Select";
    pub const UNDO: &'static str = "Undo";
}
#[cfg(any(feature = "legacy", doc))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
pub use self::legacy::*;

/// Non-Standard International Keys as defined in
/// <https://www.w3.org/TR/2017/CR-uievents-code-20170601/#key-legacy>
#[cfg(any(feature = "non_standard_intl", doc))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "non_standard_intl")))]
pub mod non_standard_intl {
    pub const HIRAGANA: &'static str = "Hiragana";
    pub const KATAKANA: &'static str = "Katakana";
}
#[cfg(any(feature = "non_standard_intl", doc))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "non_standard_intl")))]
use self::non_standard_intl::*;

/// Special Keys as defined in
/// <https://www.w3.org/TR/2017/CR-uievents-code-20170601/#key-legacy>.
/// Currently, only the `Unidentified` key is defined.
pub mod special {
    pub const UNIDENTIFIED: &'static str = "Unidentified";
}
pub use self::special::*;
