//! QBopomofo Windows Text Service (TSF)
//!
//! This crate implements a Windows Text Services Framework (TSF) input method.
//! When compiled as a DLL on Windows, it registers as a COM server and provides
//! Chinese Bopomofo input via the chewing engine.
//!
//! ## Architecture
//!
//! - `text_service.rs` — Main TSF text service definition
//! - `key_event.rs` — Windows VK → chewing KeyboardEvent mapping
//! - `com.rs` — COM registration constants and DLL exports (Windows only)
//!
//! The engine is linked directly as a Rust crate (zero FFI overhead).

pub mod com;
pub mod key_event;
pub mod text_service;
