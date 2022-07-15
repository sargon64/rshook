#![allow(non_camel_case_types, dead_code, non_snake_case)]

use std::{fmt::{Formatter, Debug, Error}};

const WM_KEYUP: u32 = 0x0101;
const WM_KEYDOWN: u32 = 0x0100;
const WM_SYSKEYDOWN: u32 = 0x0104;
const WM_SYSKEYUP: u32 = 0x0105;

#[derive(Debug, Default)]
#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct MSG {
    pub hwnd: i32,
    pub message: u32,
    pub wParam: u64,
    pub lParam: i64,
    pub time: u64,
    pub pt: POINT,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct KBDLLHOOKSTRUCT {
    pub vk_code: u64,
    pub scan_code: u64,
    pub flags: u64,
    pub time: u64,
    pub dw_extra_info: u64,
}

#[link(name = "user32")]
extern "system" {
    pub fn SetWindowsHookExW(idhook: u32, lpfn: extern "system" fn(u32, u32, i64) -> u32, hmod: u32, dwThreadId: u32) -> u32;
    pub fn CallNextHookEx(hhk: u32, ncode: u32, wparam: u32, lparam: i64) -> u32;
    pub fn PostMessageA(hwnd: u32, msg: u64, wparam: u64, lparam: i64) -> i32;
    pub fn GetMessageA(lpMsg: *mut MSG, hwnd: u32, wMsgFilterMin: u32, wMsgFilterMax: u32) -> i32;
    pub fn TranslateMessage(lpMsg: *mut MSG) -> i32;
    pub fn DispatchMessageA(lpMsg: *mut MSG) -> i32;
    
}

extern "system" fn LowLevelKeyboardProc(nCode: u32, wParam: u32, lParam: i64) -> u32 {
    let send;
    let p_kbdllhookstruct = unsafe { &mut *(lParam as *mut KBDLLHOOKSTRUCT) };
    match wParam {
        WM_KEYUP => {
            send = true;
        }
        WM_KEYDOWN => {
            send = true;
        }
        _ => {
            send = true;
        }
    }
    if send {
        unsafe { PostMessageA(0, wParam.into(), p_kbdllhookstruct.scan_code, 0) };
    }
    unsafe { CallNextHookEx(0, nCode, wParam, lParam) }
}

fn LowLevelKeyboardProc_install() {
    unsafe {
        SetWindowsHookExW(0xD, LowLevelKeyboardProc, 0, 0);
    }
}

fn main() {
    LowLevelKeyboardProc_install();
    let mut msg = MSG::default();
    unsafe {
        while ( GetMessageA(&mut msg, 0, 0, 0) ) != 0 {
            TranslateMessage(&mut msg);
            DispatchMessageA(&mut msg);
            println!("{:?}", msg.wParam);
        }
    }
}