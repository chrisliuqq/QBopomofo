//! TSF Text Service implementation
//!
//! Core text service struct that wraps the chewing engine.
//! On Windows, this implements COM interfaces for TSF integration.

use chewing::editor::Editor;

/// QBopomofo TSF Text Service
///
/// Wraps the chewing Editor and manages TSF lifecycle.
/// On Windows, COM interfaces are implemented via the `windows` crate:
/// - ITfTextInputProcessorEx (activation/deactivation)
/// - ITfKeyEventSink (keyboard input)
/// - ITfCompositionSink (composition lifecycle)
pub struct QBopomofoTextService {
    /// The chewing editor instance (handles input state machine)
    editor: Option<Editor>,
    /// Whether the service is currently active
    activated: bool,
}

impl QBopomofoTextService {
    pub fn new() -> Self {
        Self {
            editor: None,
            activated: false,
        }
    }

    /// Called when the input method is activated.
    /// Initializes the chewing editor with dictionary data.
    pub fn activate(&mut self) {
        // Editor will be initialized with dictionary path on Windows
        // For now, just mark as activated
        self.activated = true;
    }

    /// Called when the input method is deactivated.
    pub fn deactivate(&mut self) {
        self.editor = None;
        self.activated = false;
    }

    pub fn is_activated(&self) -> bool {
        self.activated
    }

    pub fn editor(&self) -> Option<&Editor> {
        self.editor.as_ref()
    }

    pub fn editor_mut(&mut self) -> Option<&mut Editor> {
        self.editor.as_mut()
    }
}
