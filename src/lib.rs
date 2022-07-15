#[allow(dead_code)]

#[derive(Debug, Default)]
#[repr(u32)]
pub enum KeyEvents {
    KeyDown = 0x0100,
    KeyUp = 0x0101,
    SysKeyDown = 0x0104,
    SysKeyUp = 0x0105,
    #[default]
    None = 0
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct _POINT {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct _MSG {
    pub hwnd: i32,
    pub message: u32,
    pub wParam: u64,
    pub lParam: i64,
    pub time: u64,
    pub pt: _POINT,
}

pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default)]
pub struct KeyMSG {
    pub event: KeyEvents,
    pub key: i64,
}

impl From<_MSG> for KeyMSG {
    fn from(msg: _MSG) -> Self {
        KeyMSG {
            event: match msg.wParam {
                0x0100 => KeyEvents::KeyDown,
                0x0101 => KeyEvents::KeyUp,
                0x0104 => KeyEvents::SysKeyDown,
                0x0105 => KeyEvents::SysKeyUp,
                _ => KeyEvents::None,
            },
            key: msg.lParam as i64,
        }
    }
}

extern "C" {
    fn LowLevelKeyboardProc_install() -> u32;
    fn Wait_until() -> _MSG;
}

/// Returns the hook handle.
fn install_hook() -> u32 {
    unsafe {
        LowLevelKeyboardProc_install()
    }
}

/// waits untill a key is pressed.
fn wait_until_keyboard_input() -> KeyMSG {
    unsafe {
        KeyMSG::from(Wait_until())
    }
}
