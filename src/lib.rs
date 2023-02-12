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
pub use functional::*;

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
pub use self::non_standard_intl::*;

/// Special Keys as defined in
/// <https://www.w3.org/TR/2017/CR-uievents-code-20170601/#key-legacy>.
/// Currently, only the `Unidentified` key is defined.
pub mod special {
    pub const UNIDENTIFIED: &'static str = "Unidentified";
}
pub use self::special::*;

/// Enum with various [`KeyboardEvent.code`] values
/// as per the latest the latest (as of 11 February, 2023) [W3C Candidate Recommendation, 01 June 2017].
///
/// [`KeyboardEvent.code`]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
/// [W3C Candidate Recommendation, 01 June 2017]: https://www.w3.org/TR/2017/CR-uievents-code-20170601/#code-value-tables
#[cfg(any(feature = "enum", doc))]
#[cfg_attr(doc_cfg, doc(cfg(feature = "enum")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum KeyboardEventCode {
    /// Corresponds to [BACKQUOTE][crate::BACKQUOTE].
    Backquote,
    /// Corresponds to [BACKSLASH][crate::BACKSLASH].
    Backslash,
    /// Corresponds to [BACKSPACE][crate::BACKSPACE].
    Backspace,
    /// Corresponds to [BRACKET_LEFT][crate::BRACKET_LEFT].
    BracketLeft,
    /// Corresponds to [BRACKET_RIGHT][crate::BRACKET_RIGHT].
    BracketRight,
    /// Corresponds to [COMMA][crate::COMMA].
    Comma,
    /// Corresponds to [DIGIT0][crate::DIGIT0].
    Digit0,
    /// Corresponds to [DIGIT1][crate::DIGIT1].
    Digit1,
    /// Corresponds to [DIGIT2][crate::DIGIT2].
    Digit2,
    /// Corresponds to [DIGIT3][crate::DIGIT3].
    Digit3,
    /// Corresponds to [DIGIT4][crate::DIGIT4].
    Digit4,
    /// Corresponds to [DIGIT5][crate::DIGIT5].
    Digit5,
    /// Corresponds to [DIGIT6][crate::DIGIT6].
    Digit6,
    /// Corresponds to [DIGIT7][crate::DIGIT7].
    Digit7,
    /// Corresponds to [DIGIT8][crate::DIGIT8].
    Digit8,
    /// Corresponds to [DIGIT9][crate::DIGIT9].
    Digit9,
    /// Corresponds to [EQUAL][crate::EQUAL].
    Equal,
    /// Corresponds to [INTL_BACKSLASH][crate::INTL_BACKSLASH].
    IntlBackslash,
    /// Corresponds to [INTL_RO][crate::INTL_RO].
    IntlRo,
    /// Corresponds to [INTL_YEN][crate::INTL_YEN].
    IntlYen,
    /// Corresponds to [KEY_A][crate::KEY_A].
    KeyA,
    /// Corresponds to [KEY_B][crate::KEY_B].
    KeyB,
    /// Corresponds to [KEY_C][crate::KEY_C].
    KeyC,
    /// Corresponds to [KEY_D][crate::KEY_D].
    KeyD,
    /// Corresponds to [KEY_E][crate::KEY_E].
    KeyE,
    /// Corresponds to [KEY_F][crate::KEY_F].
    KeyF,
    /// Corresponds to [KEY_G][crate::KEY_G].
    KeyG,
    /// Corresponds to [KEY_H][crate::KEY_H].
    KeyH,
    /// Corresponds to [KEY_I][crate::KEY_I].
    KeyI,
    /// Corresponds to [KEY_J][crate::KEY_J].
    KeyJ,
    /// Corresponds to [KEY_K][crate::KEY_K].
    KeyK,
    /// Corresponds to [KEY_L][crate::KEY_L].
    KeyL,
    /// Corresponds to [KEY_M][crate::KEY_M].
    KeyM,
    /// Corresponds to [KEY_N][crate::KEY_N].
    KeyN,
    /// Corresponds to [KEY_O][crate::KEY_O].
    KeyO,
    /// Corresponds to [KEY_P][crate::KEY_P].
    KeyP,
    /// Corresponds to [KEY_Q][crate::KEY_Q].
    KeyQ,
    /// Corresponds to [KEY_R][crate::KEY_R].
    KeyR,
    /// Corresponds to [KEY_S][crate::KEY_S].
    KeyS,
    /// Corresponds to [KEY_T][crate::KEY_T].
    KeyT,
    /// Corresponds to [KEY_U][crate::KEY_U].
    KeyU,
    /// Corresponds to [KEY_V][crate::KEY_V].
    KeyV,
    /// Corresponds to [KEY_W][crate::KEY_W].
    KeyW,
    /// Corresponds to [KEY_X][crate::KEY_X].
    KeyX,
    /// Corresponds to [KEY_Y][crate::KEY_Y].
    KeyY,
    /// Corresponds to [KEY_Z][crate::KEY_Z].
    KeyZ,
    /// Corresponds to [MINUS][crate::MINUS].
    Minus,
    /// Corresponds to [PERIOD][crate::PERIOD].
    Period,
    /// Corresponds to [QUOTE][crate::QUOTE].
    Quote,
    /// Corresponds to [SEMICOLON][crate::SEMICOLON].
    Semicolon,
    /// Corresponds to [SLASH][crate::SLASH].
    Slash,
    /// Corresponds to [ALT_LEFT][crate::ALT_LEFT].
    AltLeft,
    /// Corresponds to [ALT_RIGHT][crate::ALT_RIGHT].
    AltRight,
    /// Corresponds to [CAPS_LOCK][crate::CAPS_LOCK].
    CapsLock,
    /// Corresponds to [CONTEXT_MENU][crate::CONTEXT_MENU].
    ContextMenu,
    /// Corresponds to [CONTROL_LEFT][crate::CONTROL_LEFT].
    ControlLeft,
    /// Corresponds to [CONTROL_RIGHT][crate::CONTROL_RIGHT].
    ControlRight,
    /// Corresponds to [ENTER][crate::ENTER].
    Enter,
    /// Corresponds to [META_LEFT][crate::META_LEFT].
    MetaLeft,
    /// Corresponds to [META_RIGHT][crate::META_RIGHT].
    MetaRight,
    /// Corresponds to [SHIFT_LEFT][crate::SHIFT_LEFT].
    ShiftLeft,
    /// Corresponds to [SHIFT_RIGHT][crate::SHIFT_RIGHT].
    ShiftRight,
    /// Corresponds to [SPACE][crate::SPACE].
    Space,
    /// Corresponds to [TAB][crate::TAB].
    Tab,
    /// Corresponds to [CONVERT][crate::CONVERT].
    Convert,
    /// Corresponds to [KANA_MODE][crate::KANA_MODE].
    KanaMode,
    /// Corresponds to [LANG1][crate::LANG1].
    Lang1,
    /// Corresponds to [LANG2][crate::LANG2].
    Lang2,
    /// Corresponds to [LANG3][crate::LANG3].
    Lang3,
    /// Corresponds to [LANG4][crate::LANG4].
    Lang4,
    /// Corresponds to [LANG5][crate::LANG5].
    Lang5,
    /// Corresponds to [NON_CONVERT][crate::NON_CONVERT].
    NonConvert,
    /// Corresponds to [DELETE][crate::DELETE].
    Delete,
    /// Corresponds to [END][crate::END].
    End,
    /// Corresponds to [HELP][crate::HELP].
    Help,
    /// Corresponds to [HOME][crate::HOME].
    Home,
    /// Corresponds to [INSERT][crate::INSERT].
    Insert,
    /// Corresponds to [PAGE_DOWN][crate::PAGE_DOWN].
    PageDown,
    /// Corresponds to [PAGE_UP][crate::PAGE_UP].
    PageUp,
    /// Corresponds to [ARROW_DOWN][crate::ARROW_DOWN].
    ArrowDown,
    /// Corresponds to [ARROW_LEFT][crate::ARROW_LEFT].
    ArrowLeft,
    /// Corresponds to [ARROW_RIGHT][crate::ARROW_RIGHT].
    ArrowRight,
    /// Corresponds to [ARROW_UP][crate::ARROW_UP].
    ArrowUp,
    /// Corresponds to [NUM_LOCK][crate::NUM_LOCK].
    NumLock,
    /// Corresponds to [NUMPAD0][crate::NUMPAD0].
    Numpad0,
    /// Corresponds to [NUMPAD1][crate::NUMPAD1].
    Numpad1,
    /// Corresponds to [NUMPAD2][crate::NUMPAD2].
    Numpad2,
    /// Corresponds to [NUMPAD3][crate::NUMPAD3].
    Numpad3,
    /// Corresponds to [NUMPAD4][crate::NUMPAD4].
    Numpad4,
    /// Corresponds to [NUMPAD5][crate::NUMPAD5].
    Numpad5,
    /// Corresponds to [NUMPAD6][crate::NUMPAD6].
    Numpad6,
    /// Corresponds to [NUMPAD7][crate::NUMPAD7].
    Numpad7,
    /// Corresponds to [NUMPAD8][crate::NUMPAD8].
    Numpad8,
    /// Corresponds to [NUMPAD9][crate::NUMPAD9].
    Numpad9,
    /// Corresponds to [NUMPAD_ADD][crate::NUMPAD_ADD].
    NumpadAdd,
    /// Corresponds to [NUMPAD_BACKSPACE][crate::NUMPAD_BACKSPACE].
    NumpadBackspace,
    /// Corresponds to [NUMPAD_CLEAR][crate::NUMPAD_CLEAR].
    NumpadClear,
    /// Corresponds to [NUMPAD_CLEAR_ENTRY][crate::NUMPAD_CLEAR_ENTRY].
    NumpadClearEntry,
    /// Corresponds to [NUMPAD_COMMA][crate::NUMPAD_COMMA].
    NumpadComma,
    /// Corresponds to [NUMPAD_DECIMAL][crate::NUMPAD_DECIMAL].
    NumpadDecimal,
    /// Corresponds to [NUMPAD_DIVIDE][crate::NUMPAD_DIVIDE].
    NumpadDivide,
    /// Corresponds to [NUMPAD_ENTER][crate::NUMPAD_ENTER].
    NumpadEnter,
    /// Corresponds to [NUMPAD_EQUAL][crate::NUMPAD_EQUAL].
    NumpadEqual,
    /// Corresponds to [NUMPAD_HASH][crate::NUMPAD_HASH].
    NumpadHash,
    /// Corresponds to [NUMPAD_MEMORY_ADD][crate::NUMPAD_MEMORY_ADD].
    NumpadMemoryAdd,
    /// Corresponds to [NUMPAD_MEMORY_CLEAR][crate::NUMPAD_MEMORY_CLEAR].
    NumpadMemoryClear,
    /// Corresponds to [NUMPAD_MEMORY_RECALL][crate::NUMPAD_MEMORY_RECALL].
    NumpadMemoryRecall,
    /// Corresponds to [NUMPAD_MEMORY_STORE][crate::NUMPAD_MEMORY_STORE].
    NumpadMemoryStore,
    /// Corresponds to [NUMPAD_MEMORY_SUBTRACT][crate::NUMPAD_MEMORY_SUBTRACT].
    NumpadMemorySubtract,
    /// Corresponds to [NUMPAD_MULTIPLY][crate::NUMPAD_MULTIPLY].
    NumpadMultiply,
    /// Corresponds to [NUMPAD_PAREN_LEFT][crate::NUMPAD_PAREN_LEFT].
    NumpadParenLeft,
    /// Corresponds to [NUMPAD_PAREN_RIGHT][crate::NUMPAD_PAREN_RIGHT].
    NumpadParenRight,
    /// Corresponds to [NUMPAD_STAR][crate::NUMPAD_STAR].
    NumpadStar,
    /// Corresponds to [NUMPAD_SUBTRACT][crate::NUMPAD_SUBTRACT].
    NumpadSubtract,
    /// Corresponds to [ESCAPE][crate::ESCAPE].
    Escape,
    /// Corresponds to [F1][crate::F1].
    F1,
    /// Corresponds to [F2][crate::F2].
    F2,
    /// Corresponds to [F3][crate::F3].
    F3,
    /// Corresponds to [F4][crate::F4].
    F4,
    /// Corresponds to [F5][crate::F5].
    F5,
    /// Corresponds to [F6][crate::F6].
    F6,
    /// Corresponds to [F7][crate::F7].
    F7,
    /// Corresponds to [F8][crate::F8].
    F8,
    /// Corresponds to [F9][crate::F9].
    F9,
    /// Corresponds to [F10][crate::F10].
    F10,
    /// Corresponds to [F11][crate::F11].
    F11,
    /// Corresponds to [F12][crate::F12].
    F12,
    /// Corresponds to [F13][crate::F13].
    F13,
    /// Corresponds to [F14][crate::F14].
    F14,
    /// Corresponds to [F15][crate::F15].
    F15,
    /// Corresponds to [F16][crate::F16].
    F16,
    /// Corresponds to [F17][crate::F17].
    F17,
    /// Corresponds to [F18][crate::F18].
    F18,
    /// Corresponds to [F19][crate::F19].
    F19,
    /// Corresponds to [FN][crate::FN].
    Fn,
    /// Corresponds to [FN_LOCK][crate::FN_LOCK].
    FnLock,
    /// Corresponds to [PRINT_SCREEN][crate::PRINT_SCREEN].
    PrintScreen,
    /// Corresponds to [SCROLL_LOCK][crate::SCROLL_LOCK].
    ScrollLock,
    /// Corresponds to [PAUSE][crate::PAUSE].
    Pause,
    /// Corresponds to [BROWSER_BACK][crate::BROWSER_BACK].
    BrowserBack,
    /// Corresponds to [BROWSER_FAVORITES][crate::BROWSER_FAVORITES].
    BrowserFavorites,
    /// Corresponds to [BROWSER_FORWARD][crate::BROWSER_FORWARD].
    BrowserForward,
    /// Corresponds to [BROWSER_HOME][crate::BROWSER_HOME].
    BrowserHome,
    /// Corresponds to [BROWSER_REFRESH][crate::BROWSER_REFRESH].
    BrowserRefresh,
    /// Corresponds to [BROWSER_SEARCH][crate::BROWSER_SEARCH].
    BrowserSearch,
    /// Corresponds to [BROWSER_STOP][crate::BROWSER_STOP].
    BrowserStop,
    /// Corresponds to [EJECT][crate::EJECT].
    Eject,
    /// Corresponds to [LAUNCH_APP1][crate::LAUNCH_APP1].
    LaunchApp1,
    /// Corresponds to [LAUNCH_APP2][crate::LAUNCH_APP2].
    LaunchApp2,
    /// Corresponds to [LAUNCH_MAIL][crate::LAUNCH_MAIL].
    LaunchMail,
    /// Corresponds to [MEDIA_PLAY_PAUSE][crate::MEDIA_PLAY_PAUSE].
    MediaPlayPause,
    /// Corresponds to [MEDIA_SELECT][crate::MEDIA_SELECT].
    MediaSelect,
    /// Corresponds to [MEDIA_STOP][crate::MEDIA_STOP].
    MediaStop,
    /// Corresponds to [MEDIA_TRACK_NEXT][crate::MEDIA_TRACK_NEXT].
    MediaTrackNext,
    /// Corresponds to [MEDIA_TRACK_PREVIOUS][crate::MEDIA_TRACK_PREVIOUS].
    MediaTrackPrevious,
    /// Corresponds to [POWER][crate::POWER].
    Power,
    /// Corresponds to [SLEEP][crate::SLEEP].
    Sleep,
    /// Corresponds to [AUDIO_VOLUME_DOWN][crate::AUDIO_VOLUME_DOWN].
    AudioVolumeDown,
    /// Corresponds to [AUDIO_VOLUME_MUTE][crate::AUDIO_VOLUME_MUTE].
    AudioVolumeMute,
    /// Corresponds to [AUDIO_VOLUME_UP][crate::AUDIO_VOLUME_UP].
    AudioVolumeUp,
    /// Corresponds to [WAKE_UP][crate::WAKE_UP].
    WakeUp,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [HYPER][crate::HYPER].
    Hyper,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [SUPER][crate::SUPER].
    Super,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [TURBO][crate::TURBO].
    Turbo,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [ABORT][crate::ABORT].
    Abort,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [RESUME][crate::RESUME].
    Resume,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [SUSPEND][crate::SUSPEND].
    Suspend,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [AGAIN][crate::AGAIN].
    Again,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [COPY][crate::COPY].
    Copy,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [CUT][crate::CUT].
    Cut,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [FIND][crate::FIND].
    Find,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [OPEN][crate::OPEN].
    Open,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [PASTE][crate::PASTE].
    Paste,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [PROPS][crate::PROPS].
    Props,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [SELECT][crate::SELECT].
    Select,
    #[cfg(any(feature = "legacy", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "legacy")))]
    /// Corresponds to [UNDO][crate::UNDO].
    Undo,
    #[cfg(any(feature = "non_standard_intl", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "non_standard_intl")))]
    /// Corresponds to [HIRAGANA][crate::HIRAGANA].
    Hiragana,
    #[cfg(any(feature = "non_standard_intl", doc))]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "non_standard_intl")))]
    /// Corresponds to [KATAKANA][crate::KATAKANA].
    Katakana,
    /// Corresponds to [UNIDENTIFIED][crate::UNIDENTIFIED].
    Unidentified,
}
