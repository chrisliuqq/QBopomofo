//! COM DLL entry points for Windows TSF registration
//!
//! A TSF input method is a COM DLL that exports:
//! - DllGetClassObject: Returns the class factory
//! - DllCanUnloadNow: Whether the DLL can be unloaded
//! - DllRegisterServer / DllUnregisterServer: System registration
//!
//! These exports are only compiled on Windows.

/// GUID string for QBopomofo text service.
/// {A7E3B4C1-9F2D-4E5A-B8C6-1D3F5A7E9B2C}
pub const CLSID_QBOPOMOFO_STR: &str = "{A7E3B4C1-9F2D-4E5A-B8C6-1D3F5A7E9B2C}";

/// Display name shown in Windows language settings
pub const DISPLAY_NAME: &str = "QBopomofo Q注音輸入法";

/// Language ID for Traditional Chinese (Taiwan)
pub const LANG_ID: u16 = 0x0404; // zh-TW

// -------------------------------------------------------
// COM DLL exports — only compiled on Windows
// -------------------------------------------------------
// The full COM implementation requires:
// 1. IClassFactory to create QBopomofoTextService instances
// 2. DllGetClassObject export
// 3. ITfTextInputProcessorEx implementation on QBopomofoTextService
// 4. ITfKeyEventSink implementation for keyboard handling
//
// These will be implemented when building on a Windows machine
// using the `windows` crate's `#[implement]` macro.
//
// See: https://github.com/chewing/windows-chewing-tsf for reference.
