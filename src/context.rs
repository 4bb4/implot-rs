//! Module for handling the ImPlot context. This is modeled quite directly after how
//! this is dealt with in imgui-rs, because it follows the same concepts and doing this
//! also helps readability if one is already familiar with the imgui code.

use parking_lot::ReentrantMutex;

use crate::sys;
use crate::PlotUi;
/// An implot context.
///
/// A context is required to do most of the things this library provides. While this was created
/// implicitly in earlier versions of the library, it is now created explicitly. These contexts
/// cannot currently be disabled through the high level API. This could be implemented though,
/// if you need multiple contexts that you can switch around between, file an issue.
pub struct Context {
    raw: *mut sys::ImPlotContext,
}

lazy_static::lazy_static! {
    // This mutex is used to guard any accesses to the context
    static ref CTX_MUTEX: ReentrantMutex<()> = ReentrantMutex::new(());
}

fn no_current_context() -> bool {
    let ctx = unsafe { sys::ImPlot_GetCurrentContext() };
    ctx.is_null()
}

impl Context {
    /// Create a context. This will also activate the context in ImPlot, and hence creating
    /// a second context when one already exists is an error and will panic.
    pub fn create() -> Self {
        let _guard = CTX_MUTEX.lock();
        assert!(
            no_current_context(),
            "A new active context cannot be created, because another one already exists"
        );

        let ctx = unsafe { sys::ImPlot_CreateContext() };
        unsafe {
            sys::ImPlot_SetCurrentContext(ctx);
        }
        Self { raw: ctx }
    }

    /// Get a "plot ui" struct, this will be used to build actual plots and is quite
    /// analogous to imgui-rs' "Ui" struct.
    pub fn get_plot_ui(&self) -> PlotUi {
        PlotUi { context: self }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        let _guard = CTX_MUTEX.lock();
        unsafe {
            sys::ImPlot_DestroyContext(self.raw);
        }
    }
}
