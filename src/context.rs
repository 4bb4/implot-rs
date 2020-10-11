// TODO(4bb4) Do this properly.
// I already added a simple Context struct that can be created once and used as long as it is not
// dropped here for initial tests - this is of course neither threadsafe nor otherwise safe to use
// unless one "does it right", so it's not a real solution.
//
// The context should have to be created, and ideally it should be difficult to impossible
// to do things without having a context. implot-rs makes it so that there is a context and
// that context has a "frame()" function that returns a Ui, and that Ui is then used to create
// widgets. Windows are built with a build() function that takes a reference to that Ui as an
// argument, but also have a begin() function that take a context and put it in their token.
//
// I think I'll mirror that here, except that we don't need a frame() function, it's enough
// to create a context once and then keep passing it around. I'll hence need a mutex and
// a mechansim similar (or equal) to what imgui-rs does for making sure there can only be
// a single context. Implementation could go roughly like this:
//
// - Add a mutex for modifying context things
// - Make creation and drop functions use that mutex
// - Change Plot, PlotLine, PlotScatter, PlotBars, PlotText to all require a context.
//   I think I'll call this PlotUi to mimmick imgui-rs' Ui.
// - Think about what this means in terms of the stacks and things like is_plot_hovered() -
//   they should also only work when there is a context available.
//
/// An implot context.
///
/// A context is required to do most of the things this library provides. While this was created
/// implicitly in earlier versions of the library, it is now created explicitly.
pub struct Context {
    raw: *mut sys::ImPlotContext,
}

impl Context {
    /// Create a context.
    pub fn create() -> Self {
        let ctx = unsafe { sys::ImPlot_CreateContext() };
        unsafe {
            sys::ImPlot_SetCurrentContext(ctx);
        }
        Self { raw: ctx }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            sys::ImPlot_DestroyContext(self.raw);
        }
    }
}
