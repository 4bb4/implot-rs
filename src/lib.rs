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
use bitflags::bitflags;
use sys::imgui::im_str;
pub use sys::imgui::Condition;

const DEFAULT_PLOT_SIZE_X: f32 = 400.0;
const DEFAULT_PLOT_SIZE_Y: f32 = 400.0;

bitflags! {
    /// Window hover check option flags. Documentation copied from implot.h for convenience.
    #[repr(transparent)]
    pub struct PlotFlags: u32 {
        /// The mouse position, in plot coordinates, will be displayed in the bottom-right
        const MOUSE_POSITION = sys::ImPlotFlags__ImPlotFlags_MousePos;
        /// A legend will be displayed in the top-left
        const LEGEND = sys::ImPlotFlags__ImPlotFlags_Legend;
        /// Plot items will be highlighted when their legend entry is hovered
        const HIGHLIGHT = sys::ImPlotFlags__ImPlotFlags_Highlight;
        /// The user will be able to box-select with right-mouse
        const BOX_SELECT = sys::ImPlotFlags__ImPlotFlags_BoxSelect;
        /// The user will be able to draw query rects with middle-mouse
        const QUERY = sys::ImPlotFlags__ImPlotFlags_Query;
        /// The user will be able to open a context menu with double-right click
        const CONTEXT_MENU = sys::ImPlotFlags__ImPlotFlags_ContextMenu;
        /// The default mouse cursor will be replaced with a crosshair when hovered
        const CROSSHAIRS = sys::ImPlotFlags__ImPlotFlags_Crosshairs;
        /// Plot data outside the plot area will be culled from rendering
        const CULL_DATA = sys::ImPlotFlags__ImPlotFlags_CullData;
        /// Lines and fills will be anti-aliased (not recommended)
        const ANTIALIASED = sys::ImPlotFlags__ImPlotFlags_AntiAliased;
        /// A child window region will not be used to capture mouse scroll (can boost performance
        /// for single ImGui window applications)
        const NO_CHILD = sys::ImPlotFlags__ImPlotFlags_NoChild;
        /// Enable a 2nd y axis
        const Y_AXIS_2 = sys::ImPlotFlags__ImPlotFlags_YAxis2;
        /// Enable a 3nd y axis
        const Y_AXIS_3 = sys::ImPlotFlags__ImPlotFlags_YAxis3;
        /// Default selection of flags
        const DEFAULT = sys::ImPlotFlags__ImPlotFlags_Default;
    }
}

bitflags! {
    /// Axis flags. Documentation copied from implot.h for convenience.
    #[repr(transparent)]
    pub struct AxisFlags: u32 {
        /// Grid lines will be displayed
        const GRID_LINES = sys::ImPlotAxisFlags__ImPlotAxisFlags_GridLines;
        /// Tick marks will be displayed for each grid line
        const TICK_MARKS = sys::ImPlotAxisFlags__ImPlotAxisFlags_TickMarks;
        /// Text labels will be displayed for each grid line
        const TICK_LABELS = sys::ImPlotAxisFlags__ImPlotAxisFlags_TickLabels;
        /// The axis will be inverted
        const INVERT = sys::ImPlotAxisFlags__ImPlotAxisFlags_Invert;
        /// The axis minimum value will be locked when panning/zooming
        const LOCK_MIN = sys::ImPlotAxisFlags__ImPlotAxisFlags_LockMin;
        /// The axis maximum value will be locked when panning/zooming
        const LOCK_MAX = sys::ImPlotAxisFlags__ImPlotAxisFlags_LockMax;
        /// Grid divisions will adapt to the current pixel size the axis
        const ADAPTIVE = sys::ImPlotAxisFlags__ImPlotAxisFlags_Adaptive;
        /// A logartithmic (base 10) axis scale will be used
        const LOG_SCALE = sys::ImPlotAxisFlags__ImPlotAxisFlags_LogScale;
        /// Scientific notation will be used for tick labels if displayed (WIP, not very good yet)
        const SCIENTIFIC = sys::ImPlotAxisFlags__ImPlotAxisFlags_Scientific;
        /// Default set of flags
        const DEFAULT = sys::ImPlotAxisFlags__ImPlotAxisFlags_Default;
        /// Same as defaults, but without grid lines
        const AUXILIARY = sys::ImPlotAxisFlags__ImPlotAxisFlags_Auxiliary;
    }
}

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
    /// Flags relating to the second y axis of the plot (if present, otherwise ignored)
    /// TODO(4bb4) make those into bitflags
    y2_flags: sys::ImPlotAxisFlags,
    /// Flags relating to the third y axis of the plot (if present, otherwise ignored)
    /// TODO(4bb4) make those into bitflags
    y3_flags: sys::ImPlotAxisFlags,
}

impl Plot {
    /// Create a new plot with some defaults set. Does not draw anything yet.
    pub fn new(title: &str) -> Self {
        // TODO(4bb4) question these defaults, maybe remove some of them
        Self {
            title: title.to_owned(),
            size_x: DEFAULT_PLOT_SIZE_X,
            size_y: DEFAULT_PLOT_SIZE_Y,
            x_label: "".to_owned(),
            y_label: "".to_owned(),
            x_limits: None,
            y_limits: None,
            x_limit_condition: None,
            y_limit_condition: None,
            plot_flags: PlotFlags::DEFAULT.bits() as sys::ImPlotFlags,
            x_flags: AxisFlags::DEFAULT.bits() as sys::ImPlotAxisFlags,
            y_flags: AxisFlags::DEFAULT.bits() as sys::ImPlotAxisFlags,
            y2_flags: AxisFlags::DEFAULT.bits() as sys::ImPlotAxisFlags,
            y3_flags: AxisFlags::DEFAULT.bits() as sys::ImPlotAxisFlags,
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

    /// Set the plot flags, see the help for `PlotFlags` for what the available flags are
    #[inline]
    pub fn with_plot_flags(mut self, flags: &PlotFlags) -> Self {
        self.plot_flags = flags.bits() as sys::ImPlotFlags;
        self
    }

    /// Set the axis flags for the X axis in this plot
    #[inline]
    pub fn with_x_axis_flags(mut self, flags: &AxisFlags) -> Self {
        self.x_flags = flags.bits() as sys::ImPlotAxisFlags;
        self
    }

    /// Set the axis flags for the first Y axis in this plot
    #[inline]
    pub fn with_y_axis_flags(mut self, flags: &AxisFlags) -> Self {
        self.y_flags = flags.bits() as sys::ImPlotAxisFlags;
        self
    }

    /// Set the axis flags for the second Y axis in this plot
    #[inline]
    pub fn with_y2_axis_flags(mut self, flags: &AxisFlags) -> Self {
        self.y2_flags = flags.bits() as sys::ImPlotAxisFlags;
        self
    }

    /// Set the axis flags for the third Y axis in this plot
    #[inline]
    pub fn with_y3_axis_flags(mut self, flags: &AxisFlags) -> Self {
        self.y3_flags = flags.bits() as sys::ImPlotAxisFlags;
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
                self.y2_flags,
                self.y3_flags,
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
        // If there is no data to plot, we stop here
        if x.len().min(y.len()) == 0 {
            return;
        }
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
