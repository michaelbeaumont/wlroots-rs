use libc;
use wlroots_sys::{wlr_backend, wlr_headless_backend_create, wlr_headless_add_output,
                  wlr_headless_add_input_device, wlr_input_device_is_headless,
                  wlr_output_is_headless, wlr_input_device_type, wl_display};

use {backend::UnsafeRenderSetupFunction,
     input,
     output::{self, Output},
     utils::Handleable};

/// In this backend the only resource the compositor uses is the Wayland file descriptor.
/// It doesn't try to grab actual keyboard/pointers and it doesn't render anything.
///
/// This backend is useful for testing as you can easily add "fake" inputs and outputs.
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Headless {
    pub(crate) backend: *mut wlr_backend
}

impl Headless {
    /// Creates a headless backend.
    ///
    /// A headless backend has no outputs or inputs by default.
    pub unsafe fn new(display: *mut wl_display,
                      render_setup_func: Option<UnsafeRenderSetupFunction>)
                      -> Self {
        let backend = wlr_headless_backend_create(display, render_setup_func);
        if backend.is_null() {
            panic!("Could not construct Headless backend");
        }
        Headless { backend }
    }


    /// Create a new headless output backed by an in-memory EGL framebuffer.
    ///
    /// You can read pixels from this framebuffer via `Renderer::read_pixels`
    /// but it is otherwise not displayed.
    pub fn add_output(&self, width: libc::c_uint, height: libc::c_uint) -> Option<output::Handle> {
        unsafe {
            let output_ptr = wlr_headless_add_output(self.backend, width, height);
            if output_ptr.is_null() {
                None
            } else {
                Some(output::Handle::from_ptr(output_ptr))
            }
        }
    }

    /// Creates a new input device.
    ///
    /// The caller is responsible for manually raising any event signals on the
    /// new input device if it wants to simulate input events.
    pub fn add_input_device(&self, input_type: wlr_input_device_type) -> Option<input::Handle> {
        unsafe {
            let device = wlr_headless_add_input_device(self.backend, input_type);
            if device.is_null() {
                None
            } else {
                Some(input::Device { device }.device())
            }
        }
    }

    pub fn is_headless_input_device(&self, input_device: &input::Device) -> bool {
        unsafe {
            wlr_input_device_is_headless(input_device.as_ptr())
        }
    }

    pub fn is_headless_output(&self, output: &Output) -> bool {
        unsafe {
            wlr_output_is_headless(output.as_ptr())
        }
    }

    pub unsafe fn as_ptr(&self) -> *mut wlr_backend {
        self.backend
    }
}
