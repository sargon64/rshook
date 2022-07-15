// #![allow(non_camel_case_types, dead_code, non_snake_case)]

// use std::{fmt::{Formatter, Debug, Error}};

// const WM_KEYUP: u32 = 0x0101;
// const WM_KEYDOWN: u32 = 0x0100;
// const WM_SYSKEYDOWN: u32 = 0x0104;
// const WM_SYSKEYUP: u32 = 0x0105;
// const WH_KEYBOARD_LL: u32 = 13;

// #[derive(Debug, Default)]
// #[repr(C)]
// pub struct POINT {
//     pub x: i32,
//     pub y: i32,
// }

// #[derive(Debug, Default)]
// #[repr(C)]
// pub struct MSG {
//     pub hwnd: i32,
//     pub message: u32,
//     pub wParam: u64,
//     pub lParam: i64,
//     pub time: u64,
//     pub pt: POINT,
// }

// #[derive(Debug, Default)]
// #[repr(C)]
// pub struct KBDLLHOOKSTRUCT {
//     pub vk_code: u64,
//     pub scan_code: u64,
//     pub flags: u64,
//     pub time: u64,
//     pub dw_extra_info: u64,
// }

// #[link(name = "user32")]
// extern "system" {
//     pub fn SetWindowsHookExW(idhook: u32, lpfn: extern "system" fn(u32, u32, i64) -> u32, hmod: u32, dwThreadId: u32) -> u32;
//     pub fn CallNextHookEx(hhk: u32, ncode: u32, wparam: u32, lparam: i64) -> u32;
//     pub fn PostMessageA(hwnd: u32, msg: u64, wparam: u64, lparam: i64) -> i32;
//     pub fn GetMessageA(lpMsg: *mut MSG, hwnd: u32, wMsgFilterMin: u32, wMsgFilterMax: u32) -> i32;
//     pub fn TranslateMessage(lpMsg: *mut MSG) -> i32;
//     pub fn DispatchMessageA(lpMsg: *mut MSG) -> i32;
//     pub fn GetModuleHandleW(lpModuleName: u16) -> u32;
//     pub fn UnhookWindowsHookEx(hhk: u32) -> i32;
// }

// extern "system" fn LowLevelKeyboardProc(nCode: u32, wParam: u32, lParam: i64) -> u32 {
//     let send;
//     let p_kbdllhookstruct = unsafe { &mut *(lParam as *mut KBDLLHOOKSTRUCT) };
//     let p_key;
//     match wParam {
//         WM_KEYUP => {
//             send = true;
//             p_key = p_kbdllhookstruct.vk_code;
//         }
//         WM_KEYDOWN => {
//             send = true;
//             p_key = p_kbdllhookstruct.vk_code;
//         }
//         _ => {
//             p_key = 0;
//             send = false;
//         }
//     }
//     if send {
//         unsafe { PostMessageA(0, wParam.into(), p_key, 0) };
//     }
//     unsafe { CallNextHookEx(0, nCode, wParam, lParam) }
// }

// fn LowLevelKeyboardProc_install() -> u32 {
//     unsafe {
//         SetWindowsHookExW(WH_KEYBOARD_LL, LowLevelKeyboardProc, GetModuleHandleW(0), 0)
//     }
// }

use std::default;

// fn LowLevelKEyboardProc_uninstall(_hhk: u32){
//     unsafe {
//         UnhookWindowsHookEx(_hhk);
//     }
// }
// fn main() {
//     let _hhk = LowLevelKeyboardProc_install();
//     let mut msg = MSG::default();
//     unsafe {
//         while ( GetMessageA(&mut msg, 0, 0, 0) ) != 0 {
//             TranslateMessage(&mut msg);
//             DispatchMessageA(&mut msg);
//             println!("{:?}", msg);
//         }
//     }
//     println!("{:?}", msg);
//     LowLevelKEyboardProc_uninstall(_hhk);
// }
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

fn main() {
    unsafe {
        LowLevelKeyboardProc_install();
        loop {
            let msg = Wait_until();
            println!("{:?}", KeyMSG::from(msg));
        }
    }
}