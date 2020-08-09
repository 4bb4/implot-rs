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
pub use sys::imgui::Condition;

/// Struct to represent an ImPlot. This is the main construct used to contain all kinds of plots in ImPlot.
///
/// `Plot` is to be used (within an imgui window) with the following pattern:
/// ```no_run
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
    /// Size of the plot in x direction, in the same units imgui uses.
    size_x: f32,
    /// Size of the plot in y direction, in the same units imgui uses.
    size_y: f32,
    /// Label of the x axis, shown on the bottom
    x_label: String,
    /// Label of the y axis, shown on the left
    y_label: String,
    /// X axis limits, if present
    x_limits: Option<[f64; 2]>,
    /// Y axis limits, if present
    y_limits: Option<[f64; 2]>,
    /// Condition on which the x limits are set
    x_limit_condition: Option<Condition>,
    /// Condition on which the y limits are set (first y axis for now)
    y_limit_condition: Option<Condition>,
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
            size_x: 400.0,
            size_y: 400.0,
            x_label: "".to_owned(),
            y_label: "".to_owned(),
            x_limits: None,
            y_limits: None,
            x_limit_condition: None,
            y_limit_condition: None,
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

    /// Set the x limits of the plot
    #[inline]
    pub fn x_limits(mut self, x_min: f64, x_max: f64, condition: Condition) -> Self {
        self.x_limits = Some([x_min, x_max]);
        self.x_limit_condition = Some(condition);
        self
    }

    /// Set the y limits of the plot
    #[inline]
    pub fn y_limits(mut self, y_min: f64, y_max: f64, condition: Condition) -> Self {
        self.y_limits = Some([y_min, y_max]);
        self.y_limit_condition = Some(condition);
        self
    }

    /// Attempt to show the plot. If this returns a token, the plot will actually
    /// be drawn. In this case, use the drawing functionality to draw things on the
    /// plot, and then call `end()` on the token when done with the plot.
    /// If none was returned, that means the plot is not rendered.
    ///
    /// For a convenient implementation of all this, use [`build()`](struct.Plot.html#method.build)
    /// instead.
    pub fn begin(&self) -> Option<PlotToken> {
        if let (Some(limits), Some(condition)) = (self.x_limits, self.x_limit_condition) {
            unsafe {
                sys::ImPlot_SetNextPlotLimitsX(limits[0], limits[1], condition as sys::ImGuiCond);
            }
        }
        if let (Some(limits), Some(condition)) = (self.y_limits, self.y_limit_condition) {
            // TODO(4bb4) allow for specification of multiple y limits, not just the first
            unsafe {
                sys::ImPlot_SetNextPlotLimitsY(
                    limits[0],
                    limits[1],
                    condition as sys::ImGuiCond,
                    0,
                );
            }
        }
        let should_render = unsafe {
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
        };

        if should_render {
            Some(PlotToken {
                plot_title: self.title.clone(),
                has_ended: false,
            })
        } else {
            // In contrast with imgui windows, end() does not have to be
            // called if we don't render. This is more like an imgui popup modal.
            None
        }
    }

    /// Creates a window and runs a closure to construct the contents.
    ///
    /// Note: the closure is not called if ImPlot::BeginPlot() returned
    /// false - TODO(4bb4) figure out if this is if things are not rendered
    pub fn build<F: FnOnce()>(self, f: F) {
        if let Some(token) = self.begin() {
            f();
            token.end()
        }
    }
}

/// Tracks a plot that must be ended by calling `.end()`
pub struct PlotToken {
    /// For better error messages
    plot_title: String,
    /// Whether end() has been called on this already or not
    has_ended: bool,
}

impl PlotToken {
    /// End a previously begin()'ed plot.
    pub fn end(mut self) {
        self.has_ended = true;
        unsafe { sys::ImPlot_EndPlot() };
    }
}

impl Drop for PlotToken {
    fn drop(&mut self) {
        if !self.has_ended && !std::thread::panicking() {
            panic!(
                "Warning: A PlotToken for plot \"{}\" was not called end() on",
                self.plot_title
            );
        }
    }
}

/// Struct to provide functionality for plotting a line in a plot.
pub struct PlotLine {
    /// Label to show in the legend for this line
    label: String,
}

impl PlotLine {
    pub fn new(label: &str) -> Self {
        PlotLine {
            label: label.to_owned(),
        }
    }

    /// Plot a line. Use this in closures passed to [`Plot::build()`](struct.Plot.html#method.build)
    pub fn plot(&self, x: &Vec<f64>, y: &Vec<f64>) {
        unsafe {
            implot_sys::ImPlot_PlotLinedoublePtrdoublePtr(
                im_str!("{}", self.label).as_ptr() as *const i8,
                x.as_ptr(),
                y.as_ptr(),
                x.len().min(y.len()) as i32, // "as" casts saturate as of Rust 1.45. This is safe here.
                0,                           // No offset
                std::mem::size_of::<f64>() as i32, // Stride, set to one f64 for the standard use case
            );
        }
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
