use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::mem::size_of;
use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{GetWindowRect, SetForegroundWindow, 
    EnumWindows, GetWindowTextW, GetWindowTextLengthW};

use winapi::um::winuser::{mouse_event, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP};
use winapi::um::winuser::SetCursorPos;
use winapi::um::winuser::{INPUT, KEYBDINPUT, SendInput, INPUT_KEYBOARD, KEYEVENTF_KEYUP};

use winapi::um::winuser::GetCursorPos;
use winapi::shared::windef::POINT;

use winapi::um::wingdi::GetPixel;
use winapi::um::winuser::GetDC;

/*
* CONTROL: 0x11, SHIFT: 0x10, RETURN: 0x0d, TAB: 0x09
* F1 ～ F24: 0x70～0x87, ESC: 0x1b
*
* 詳しくは winapi-rs/src/um/winuser.rs
* https://github.com/retep998/winapi-rs/blob/0.3/src/um/winuser.rs
*/

/*
* HWNDでウィンドウをアクティブにする
*/
fn active(hwnd: HWND) {
    unsafe {
        SetForegroundWindow(hwnd);
    }
}

/*
* HWNDのウィンドウのサイズを取得する。
*
* -> left top right bottom
*
* 座標は画面に対して絶対位置
*/
fn get_window_rect(hwnd: HWND) -> RECT {
    let mut rect: RECT;
    unsafe {
        rect = std::mem::zeroed();
        GetWindowRect(hwnd, &mut rect);
    }
    rect
}

/*
* 画面上にあるウィンドウのHWND、タイトルを一覧で取得して返す。
* やり取りにはWINDOW_INFOS、WindowInfoを利用する。
*
*     ex) get_window_infos();
*
*/
pub static mut WINDOW_INFOS: Vec<WindowInfo> = Vec::new();

pub struct WindowInfo {
    pub hwnd: HWND,
    pub title: String,
}

pub unsafe extern "system" fn enum_windows_proc(hwnd: HWND, _lparam: isize) -> i32 {
    let length = GetWindowTextLengthW(hwnd);
    let mut buffer = vec![0; length as usize + 1];
    GetWindowTextW(hwnd, buffer.as_mut_ptr(), length + 1);
    let title = OsString::from_wide(&buffer).into_string().unwrap_or_default();
    if title.len() > 1 {
        let window_info = WindowInfo { hwnd, title };
        WINDOW_INFOS.push(window_info);
    } 
    1
}
pub fn get_window_infos() {
    unsafe {
        EnumWindows(Some(enum_windows_proc), 0);
    }
}


/*
* hwndで示されるウィンドウの所定の位置のピクセルの色を取得する。
* colorの形式は 0xBBGGRR
*/
pub fn get_pixel_color(hwnd: HWND, x: i32, y: i32) -> u32 {
    unsafe {
        let hdc = GetDC(hwnd);
        let color = GetPixel(hdc, x, y);
        color
    }
}

/*
* マウスの移動とクリック
*/
pub fn click(x: i32, y: i32) {
    unsafe {
        SetCursorPos(x, y);
        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    }
}

/*
* マウスのカーソル位置の座標を取得
* 座標は画面に対して絶対位置
*/
pub fn get_cursor_pos() -> POINT {
    let mut pt: POINT = POINT { x:0, y:0 };
    unsafe {
        GetCursorPos(&mut pt);
    }
    pt
}

/*
* キーダウン
*/
pub fn press_key(vk: u16) {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { std::mem::zeroed() },
    };
    unsafe {
        *input.u.ki_mut() = KEYBDINPUT {
            wVk: vk,
            wScan: 0,
            dwFlags: 0,
            time: 0,
            dwExtraInfo: 0,
        };
        SendInput(1, &mut input, size_of::<INPUT>() as i32);
    }
}
/*
* キーアップ
*/
pub fn release_key(vk: u16) {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe { std::mem::zeroed() },
    };
    unsafe {
        *input.u.ki_mut() = KEYBDINPUT {
            wVk: vk,
            wScan: 0,
            dwFlags: KEYEVENTF_KEYUP,
            time: 0,
            dwExtraInfo: 0,
        };
        SendInput(1, &mut input, size_of::<INPUT>() as i32);
    }
}



#[test]
pub fn function_test() {
    get_window_infos();
    unsafe {

        // WINDOW_INFOSはHWNDを含む為 unsafeブロックで利用する必要がある。
        for window_info in &WINDOW_INFOS {
            if window_info.title.contains("無題.txt - メモ帳") {
                println!("HWND: {:?}, タイトル: {}", window_info.hwnd, window_info.title);
                active(window_info.hwnd);
                let rect = get_window_rect(window_info.hwnd);
                println!("{}, {}, {}, {}", rect.left, rect.top, rect.right, rect.bottom);
                //click(rect.left, rect.top);
                click(rect.left+28, rect.top+104);
                println!("color: {:?}", get_pixel_color(window_info.hwnd, 28, 14));
            } 
        }

        let V_KEY: u16 = 0x56;
        let CONTROL_KEY: u16 = 0x11;
        press_key(CONTROL_KEY as u16);
        press_key(V_KEY);
        release_key(V_KEY);
        release_key(CONTROL_KEY as u16);

        press_key(0x0d as u16);
        release_key(0x0d as u16);

        let p = get_cursor_pos();
        println!("position {:?}, {:?}", p.x, p.y);
    }
}

