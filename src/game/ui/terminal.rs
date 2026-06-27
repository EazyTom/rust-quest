//! Terminal setup: UTF-8 and ANSI colors (especially on Windows).
//!
//! Call `setup()` once at startup before any styled output.

/// Whether emoji rendering is enabled (false = ASCII fallback).
static mut USE_EMOJI: bool = true;

pub fn setup() {
    #[cfg(windows)]
    enable_windows_vt();
    // LEARN: UTF-8 source files + modern terminal = emoji support.
}

pub fn use_emoji() -> bool {
    unsafe { USE_EMOJI }
}

pub fn set_ascii_fallback() {
    unsafe { USE_EMOJI = false };
    eprintln!("Tip: use Windows Terminal or WezTerm for full emoji and colors.");
}

#[cfg(windows)]
fn enable_windows_vt() {
    // LEARN: Windows needs virtual terminal processing for ANSI escape codes.
    use std::io::{self, Write};

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn SetConsoleOutputCP(w_code_page_id: u32) -> i32;
        fn GetStdHandle(n_std_handle: u32) -> *mut std::ffi::c_void;
        fn SetConsoleMode(h_console: *mut std::ffi::c_void, mode: u32) -> i32;
        fn GetConsoleMode(h_console: *mut std::ffi::c_void, mode: *mut u32) -> i32;
    }

    const STD_OUTPUT_HANDLE: u32 = 0xfffffff5;
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

    unsafe {
        SetConsoleOutputCP(65001);
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if !handle.is_null() {
            let mut mode: u32 = 0;
            if GetConsoleMode(handle, &mut mode) != 0 {
                SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }
    }
    let _ = io::stdout().flush();
}

#[cfg(not(windows))]
fn enable_windows_vt() {}
