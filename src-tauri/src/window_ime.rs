#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{HWND, LPARAM, WPARAM},
    UI::{
        Input::{
            Ime::{
                HIMC, IME_CMODE_ALPHANUMERIC, IME_CONVERSION_MODE, IME_SENTENCE_MODE,
                IME_SMODE_NONE, ImmGetContext, ImmGetConversionStatus, ImmGetOpenStatus,
                ImmReleaseContext, ImmSetConversionStatus, ImmSetOpenStatus,
            },
            KeyboardAndMouse::{
                ACTIVATE_KEYBOARD_LAYOUT_FLAGS, GetKeyboardLayout, HKL, KLF_ACTIVATE,
                KLF_SUBSTITUTE_OK, LoadKeyboardLayoutW,
            },
        },
        WindowsAndMessaging::{
            GetForegroundWindow, GetWindowThreadProcessId, PostMessageW, WM_INPUTLANGCHANGEREQUEST,
        },
    },
};
#[cfg(target_os = "windows")]
use windows::core::w;

#[cfg(target_os = "windows")]
const ENGLISH_US_LANG_ID: usize = 0x0409;

#[derive(Clone, Copy)]
pub struct ImeSnapshot {
    layout: usize,
    open: bool,
    conversion_mode: u32,
    sentence_mode: u32,
}

#[cfg(target_os = "windows")]
impl ImeSnapshot {
    fn from_window(hwnd: HWND) -> eyre::Result<Self> {
        let context = ImeContext::from_window(hwnd);
        let (open, conversion_mode, sentence_mode) = read_ime_status(context.as_ref());

        Ok(Self {
            layout: window_layout(hwnd)?.0 as usize,
            open,
            conversion_mode,
            sentence_mode,
        })
    }
}

#[cfg(not(target_os = "windows"))]
impl ImeSnapshot {
    fn empty() -> Self {
        Self {
            layout: 0,
            open: false,
            conversion_mode: 0,
            sentence_mode: 0,
        }
    }
}

pub fn capture_window_ime(
    #[cfg_attr(not(target_os = "windows"), allow(unused_variables))] hwnd: WindowHandle,
) -> eyre::Result<ImeSnapshot> {
    #[cfg(target_os = "windows")]
    return ImeSnapshot::from_window(hwnd);

    #[cfg(not(target_os = "windows"))]
    Ok(ImeSnapshot::empty())
}

pub fn foreground_window() -> eyre::Result<WindowHandle> {
    #[cfg(target_os = "windows")]
    {
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd.is_invalid() {
            return Err(eyre::eyre!("failed to get foreground window"));
        }

        return Ok(hwnd);
    }

    #[cfg(not(target_os = "windows"))]
    Err(eyre::eyre!("unsupported platform"))
}

pub fn restore_window_ime(
    #[cfg_attr(not(target_os = "windows"), allow(unused_variables))] hwnd: WindowHandle,
    #[cfg_attr(not(target_os = "windows"), allow(unused_variables))] snapshot: ImeSnapshot,
) -> eyre::Result<()> {
    #[cfg(target_os = "windows")]
    {
        request_window_layout_change(hwnd, HKL(snapshot.layout as *mut core::ffi::c_void))?;
        set_window_ime_status(hwnd, snapshot);
    }

    Ok(())
}

pub fn set_window_ime_to_english(
    #[cfg_attr(not(target_os = "windows"), allow(unused_variables))] hwnd: WindowHandle,
) {
    #[cfg(target_os = "windows")]
    {
        let Some(context) = ImeContext::from_window(hwnd) else {
            return;
        };

        unsafe {
            let _ = ImmSetConversionStatus(context.handle, IME_CMODE_ALPHANUMERIC, IME_SMODE_NONE);
            let _ = ImmSetOpenStatus(context.handle, false);
        }
    }
}

pub fn switch_window_to_english_layout(
    #[cfg_attr(not(target_os = "windows"), allow(unused_variables))] hwnd: WindowHandle,
) -> eyre::Result<()> {
    #[cfg(target_os = "windows")]
    {
        if is_english_us_layout(window_layout(hwnd)?) {
            return Ok(());
        }

        request_window_layout_change(hwnd, load_english_us_layout()?)?;
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub type WindowHandle = HWND;

#[cfg(not(target_os = "windows"))]
pub type WindowHandle = ();

#[cfg(target_os = "windows")]
pub fn window_handle_from_ptr(ptr: *mut core::ffi::c_void) -> WindowHandle {
    HWND(ptr)
}

#[cfg(target_os = "windows")]
struct ImeContext {
    hwnd: HWND,
    handle: HIMC,
}

#[cfg(target_os = "windows")]
impl ImeContext {
    fn from_window(hwnd: HWND) -> Option<Self> {
        let handle = unsafe { ImmGetContext(hwnd) };
        (!handle.is_invalid()).then_some(Self { hwnd, handle })
    }
}

#[cfg(target_os = "windows")]
impl Drop for ImeContext {
    fn drop(&mut self) {
        unsafe {
            let _ = ImmReleaseContext(self.hwnd, self.handle);
        }
    }
}

#[cfg(target_os = "windows")]
fn is_english_us_layout(layout: HKL) -> bool {
    layout.0 as usize & 0xffff == ENGLISH_US_LANG_ID
}

#[cfg(target_os = "windows")]
fn load_english_us_layout() -> eyre::Result<HKL> {
    let flags = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(KLF_ACTIVATE.0 | KLF_SUBSTITUTE_OK.0);
    let layout = unsafe { LoadKeyboardLayoutW(w!("00000409"), flags)? };
    if layout.is_invalid() {
        return Err(eyre::eyre!("failed to load en-US keyboard layout"));
    }

    Ok(layout)
}

#[cfg(target_os = "windows")]
fn read_ime_status(context: Option<&ImeContext>) -> (bool, u32, u32) {
    let Some(context) = context else {
        return (false, 0, 0);
    };

    let open = unsafe { ImmGetOpenStatus(context.handle).as_bool() };
    let mut conversion = IME_CONVERSION_MODE(0);
    let mut sentence = IME_SENTENCE_MODE(0);
    unsafe {
        let _ = ImmGetConversionStatus(context.handle, Some(&mut conversion), Some(&mut sentence));
    }

    (open, conversion.0, sentence.0)
}

#[cfg(target_os = "windows")]
fn request_window_layout_change(hwnd: HWND, layout: HKL) -> eyre::Result<()> {
    let layout_param = LPARAM(layout.0 as isize);
    unsafe {
        PostMessageW(
            Some(hwnd),
            WM_INPUTLANGCHANGEREQUEST,
            WPARAM(0),
            layout_param,
        )?
    };
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_window_ime_status(hwnd: HWND, snapshot: ImeSnapshot) {
    let Some(context) = ImeContext::from_window(hwnd) else {
        return;
    };

    unsafe {
        let _ = ImmSetConversionStatus(
            context.handle,
            IME_CONVERSION_MODE(snapshot.conversion_mode),
            IME_SENTENCE_MODE(snapshot.sentence_mode),
        );
        let _ = ImmSetOpenStatus(context.handle, snapshot.open);
    }
}

#[cfg(target_os = "windows")]
fn window_layout(hwnd: HWND) -> eyre::Result<HKL> {
    let thread_id = unsafe { GetWindowThreadProcessId(hwnd, None) };
    if thread_id == 0 {
        return Err(eyre::eyre!("failed to get window thread"));
    }

    Ok(unsafe { GetKeyboardLayout(thread_id) })
}
