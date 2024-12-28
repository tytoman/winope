use std::ptr::null_mut;
use std::char::{
    decode_utf16,
    REPLACEMENT_CHARACTER
};
use winapi::{
    um::winuser::{
        EnumWindows,
        SetWindowPos,
        GetWindowRect,
        GetWindowTextW,
        IsWindowVisible,
    },
    shared::windef::{HWND, RECT},
};
use clap::{
    Parser,
    Subcommand,
};

#[derive(Parser)]
#[command(about="A CLI for resizing and moving windows")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all window handles and titles
    List,
    /// <handle> <width> <height> Resize a window
    Size {
        handle: u32,
        width: i32,
        height: i32,
    },
    /// <handle> <x> <y> Move a window
    Move {
        handle: u32,
        x: i32,
        y: i32,
    },
}

fn utf16toutf8(s: &[u16]) -> String {
    decode_utf16(s.iter().take_while(|&i| *i != 0).cloned()).map(|r| r.unwrap_or(REPLACEMENT_CHARACTER)).collect()
}

unsafe extern "system" fn print_window(hwnd: HWND, _: isize) -> i32 {
    if IsWindowVisible(hwnd) == 0 {
        return 1;
    }

    let mut name_buf = [0u16; 1024];
    GetWindowTextW(hwnd, name_buf.as_mut_ptr(), name_buf.len() as i32);
    let win_text: String = utf16toutf8(&name_buf);
    if win_text.is_empty() {
        return 1;
    }
    let handle = hwnd as u32;
    println!("{handle:>8} | {win_text}");
    1
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::List => {
            println!("  Handle | Title");
            println!("---------+--------------------------------");
            unsafe {
                EnumWindows(Some(print_window), 0);
            }
        }
        Commands::Size { handle, width, height } => {
            let hwnd: HWND = handle as _;

            let mut rect =  RECT { left: 0, top: 0, right: 0, bottom: 0 };
            unsafe {
                GetWindowRect(hwnd, &mut rect);
            }

            let result = unsafe {
                SetWindowPos(hwnd, null_mut(), rect.left, rect.top, width, height, 0)
            };

            if result == 0 {
                println!("Invalid handle");
            } else {
                println!("Window resized to ({width}, {height})");
            }
        }
        Commands::Move { handle, x, y } => {
            let hwnd: HWND = handle as _;

            let result = unsafe {
                SetWindowPos(hwnd, null_mut(), x, y, 0, 0, 1)
            };

            if result == 0 {
                println!("Invalid handle");
            } else {
                println!("Window moved to ({x}, {y})");
            }
        }
    }
}
