use std::ptr;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowExW, SetWindowPos, HWND_TOPMOST, SWP_NOSIZE, SWP_NOMOVE, SWP_NOACTIVATE, SWP_SHOWWINDOW,
};

// Function to find a window by its title
fn find_window_by_title(title: &str) -> Option<HWND> {
    let title_wide: Vec<u16> = title.encode_utf16().chain(Some(0)).collect(); // Convert to null-terminated UTF-16
    let hwnd_result = unsafe {
        FindWindowExW(
            HWND(ptr::null_mut()), 
            HWND(ptr::null_mut()), 
            None,
            PCWSTR(title_wide.as_ptr()),
        )
    };

    match hwnd_result {
        Ok(hwnd) => Some(hwnd),
        Err(_) => None,
    }
}

// Function to set a window always on top
fn set_window_always_on_top(hwnd: HWND) {
    unsafe {
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0,
            0,
            0,
            0,
            SWP_NOSIZE | SWP_NOMOVE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
        );
    }
}

// Usage function to set a window always on top by title
pub fn always_on_top(window_title: &str) {
    if let Some(hwnd) = find_window_by_title("Untitled - Notepad") {
        set_window_always_on_top(hwnd);
        println!("Window set to always on top.");
    } else {
        panic!("Window not found.");
    }
}
