//! # Rust bindings to ImPlot
//!
//! This crate contains idiomatic bindings to the C++ [implot library](https://github.com/epezent/implot),
//! which use the bindings exposed by the `implot-sys` crate. An attempt is made to keep
//! documentation here somewhat self-contained, but when in doubt, the documentation of implot
//! itself (in particular also the demo code [here](https://github.com/epezent/implot/blob/master/implot_demo.cpp))
//! should help as well.
//!
//!
pub extern crate implot_sys as sys;
use sys::imgui::im_str;

/// Struct to represent an ImPlot. This is the main construct used to contain all kinds of plots in ImPlot.
///
/// `Plot` is to be used (within an imgui window) with the following pattern:
/// ```rust
/// # // This doctest fails because we don't have an imgui context to run things in here
/// # use implot;
/// implot::Plot::new("my title")
///     .size(300.0, 200.0) // other things such as .x_label("some_label") can be added too
///     .build( || {
///         // Do things such as plotting lines
///     });
///
/// ```
/// (If you are coming from the C++ implementation or the C bindings: build() calls both
/// begin() and end() internally)
pub struct Plot {
    /// Title of the plot, shown on top.
    title: String,
    /// Label of the x axis, shown on the bottom
    x_label: String,
    /// Label of the y axis, shown on the left
    y_label: String,
    /// Size of the plot in x direction, in the same units imgui uses.
    size_x: f32,
    /// Size of the plot in y direction, in the same units imgui uses.
    size_y: f32,
    /// Flags relating to the plot TODO(4bb4) make those into bitflags
    plot_flags: sys::ImPlotFlags,
    /// Flags relating to the first x axis of the plot TODO(4bb4) make those into bitflags
    x_flags: sys::ImPlotAxisFlags,
    /// Flags relating to the first y axis of the plot TODO(4bb4) make those into bitflags
    y_flags: sys::ImPlotAxisFlags,
    /// Flags relating to the second x axis of the plot (if present, otherwise ignored)
    /// TODO(4bb4) make those into bitflags
    x2_flags: sys::ImPlotAxisFlags,
    /// Flags relating to the second y axis of the plot (if present, otherwise ignored)
    /// TODO(4bb4) make those into bitflags
    y2_flags: sys::ImPlotAxisFlags,
}

impl Plot {
    /// Create a new plot with some defaults set. Does not draw anything yet.
    pub fn new(title: &str) -> Self {
        // TODO(4bb4) question these defaults, maybe remove some of them
        Self {
            title: title.to_owned(),
            x_label: "".to_owned(),
            y_label: "".to_owned(),
            size_x: 400.0,
            size_y: 400.0,
            plot_flags: 0xFF, // TODO(4bb4) define the defaults better
            x_flags: 7,       // TODO(4bb4) define the defaults better
            y_flags: 7,       // TODO(4bb4) define the defaults better
            x2_flags: 0,      // TODO(4bb4) define the defaults better
            y2_flags: 0,      // TODO(4bb4) define the defaults better
        }
    }

    /// Sets the plot size, given as [size_x, size_y]. Units are the same as
    /// what imgui uses. TODO(4b4) ... which is? I'm not sure it's pixels
    #[inline]
    pub fn size(mut self, size_x: f32, size_y: f32) -> Self {
        self.size_x = size_x;
        self.size_y = size_y;
        self
    }

    /// Set the x label of the plot
    #[inline]
    pub fn x_label(mut self, label: &str) -> Self {
        self.x_label = label.to_owned();
        self
    }

    /// Set the y label of the plot
    #[inline]
    pub fn y_label(mut self, label: &str) -> Self {
        self.y_label = label.to_owned();
        self
    }

    /// Attempt to show the plot. Only do things with it and call `end()` after that
    /// if this returns `true`. Not to be used directly, use `build` instead.
    fn begin(&self) -> bool {
        unsafe {
            sys::ImPlot_BeginPlot(
                im_str!("{}", self.title).as_ptr(),
                im_str!("{}", self.x_label).as_ptr(),
                im_str!("{}", self.y_label).as_ptr(),
                sys::ImVec2 {
                    x: self.size_x as f32,
                    y: self.size_y as f32,
                },
                self.plot_flags,
                self.x_flags,
                self.y_flags,
                self.x2_flags,
                self.y2_flags,
            )
        }
    }

    /// End (this) plot. This gets called from build()
    fn end(&self) {
        unsafe { sys::ImPlot_EndPlot() }
    }

    /// Creates a window and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if ImPlot::BeginPlot() returned
    /// false - TODO(4bb4) figure out if this is if things are not rendered
    pub fn build<F: FnOnce()>(self, f: F) {
        if self.begin() {
            f();
            self.end()
        }
    }
}

// TODO(4bb4) convert to struct and add methods to set title and flags
/// Plot a line. Use this in closures passed to [`Plot::build()`](struct.Plot.html#method.build)
pub fn plot_line(x: &Vec<f64>, y: &Vec<f64>, label: &str) {
    unsafe {
        implot_sys::ImPlot_PlotLinedoublePtrdoublePtr(
            im_str!("{}", label).as_ptr() as *const i8,
            x.as_ptr(),
            y.as_ptr(),
            x.len().min(y.len()) as i32, // "as" casts saturate as of Rust 1.45
            0,
            8,
        );
    }
}

/// Show the demo window for poking around what functionality implot has to
/// offer. Note that not all of this is necessarily implemented in implot-rs
/// already - if you find something missing you'd really like, raise an issue.
// This requires implot_demo.cpp to be in the list of sources in implot-sys.
pub fn show_demo_window(show: &mut bool) {
    unsafe {
        implot_sys::ImPlot_ShowDemoWindow(show);
    }
}
