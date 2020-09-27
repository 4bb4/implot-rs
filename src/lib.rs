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
use std::convert::TryFrom;
pub use sys::imgui::Condition;
use sys::imgui::{im_str, ImString};
// TODO(4bb4) facade-wrap these
pub use sys::{ImPlotLimits, ImPlotPoint, ImPlotRange, ImVec4};

const DEFAULT_PLOT_SIZE_X: f32 = 400.0;
const DEFAULT_PLOT_SIZE_Y: f32 = 400.0;

// --- Enum definitions --------------------------------------------------------------------------
// Things that are to be combined like flags are done using bitflags, and things that are meant
// as enumerations in the traditional sense are plain enums.

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

bitflags! {
    /// Axis flags. Documentation copied from implot.h for convenience.
    #[repr(transparent)]
    pub struct Marker: u32 {
        /// no marker
        const NONE   = sys::ImPlotMarker__ImPlotMarker_None;
        /// a circle marker will be rendered at each point
        const CIRCLE = sys::ImPlotMarker__ImPlotMarker_Circle;
        /// a square maker will be rendered at each point
        const SQUARE = sys::ImPlotMarker__ImPlotMarker_Square;
        /// a diamond marker will be rendered at each point
        const DIAMOND = sys::ImPlotMarker__ImPlotMarker_Diamond;
        /// an upward-pointing triangle marker will up rendered at each point
        const UP = sys::ImPlotMarker__ImPlotMarker_Up;
        /// an downward-pointing triangle marker will up rendered at each point
        const DOWN = sys::ImPlotMarker__ImPlotMarker_Down;
        /// an leftward-pointing triangle marker will up rendered at each point
        const LEFT = sys::ImPlotMarker__ImPlotMarker_Left;
        /// an rightward-pointing triangle marker will up rendered at each point
        const RIGHT = sys::ImPlotMarker__ImPlotMarker_Right;
        /// a cross marker will be rendered at each point (not filled)
        const CROSS = sys::ImPlotMarker__ImPlotMarker_Cross;
        /// a plus marker will be rendered at each point (not filled)
        const PLUS = sys::ImPlotMarker__ImPlotMarker_Plus;
        /// a asterisk marker will be rendered at each point (not filled)
        const ASTERISK = sys::ImPlotMarker__ImPlotMarker_Asterisk;
    }
}

/// Colorable plot elements. These are called "ImPlotCol" in ImPlot itself, but I found that
/// name somewhat confusing because we are not referring to colors, but _which_ thing can
/// be colored - hence I added the "Element".
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum PlotColorElement {
    /// Plot line/outline color (defaults to next unused color in current colormap)
    Line = sys::ImPlotCol__ImPlotCol_Line,
    /// Plot fill color for bars (defaults to the current line color)
    Fill = sys::ImPlotCol__ImPlotCol_Fill,
    /// Marker outline color (defaults to the current line color)
    MarkerOutline = sys::ImPlotCol__ImPlotCol_MarkerOutline,
    /// Marker fill color (defaults to the current line color)
    MarkerFill = sys::ImPlotCol__ImPlotCol_MarkerFill,
    /// Error bar color (defaults to text color)
    ErrorBar = sys::ImPlotCol__ImPlotCol_ErrorBar,
    /// Plot frame background color (defaults to FRAME_BG)
    FrameBg = sys::ImPlotCol__ImPlotCol_FrameBg,
    /// Plot area background color (defaults to WINDOW_BG)
    PlotBg = sys::ImPlotCol__ImPlotCol_PlotBg,
    /// Plot area border color (defaults to text color)
    PlotBorder = sys::ImPlotCol__ImPlotCol_PlotBorder,
    /// X-axis grid/label color (defaults to 25% text color)
    XAxis = sys::ImPlotCol__ImPlotCol_XAxis,
    /// Y-axis grid/label color (defaults to 25% text color)
    YAxis = sys::ImPlotCol__ImPlotCol_YAxis,
    /// 2nd y-axis grid/label color (defaults to 25% text color)
    YAxis2 = sys::ImPlotCol__ImPlotCol_YAxis2,
    /// 3rd y-axis grid/label color (defaults to 25% text color)
    YAxis3 = sys::ImPlotCol__ImPlotCol_YAxis3,
    /// Box-selection color (defaults to yellow)
    Selection = sys::ImPlotCol__ImPlotCol_Selection,
    /// Box-query color (defaults to green)
    Query = sys::ImPlotCol__ImPlotCol_Query,
}

/// Colormap choice. Documentation copied from implot.h for convenience.
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Colormap {
    /// ImPlot default colormap (n=10). Called "Standard" here because Default is reserved.
    Standard = sys::ImPlotColormap__ImPlotColormap_Default,
    /// a.k.a. matplotlib "Set1" (n=9)
    Dark = sys::ImPlotColormap__ImPlotColormap_Dark,
    /// a.k.a. matplotlib "Pastel1" (n=9)
    Pastel = sys::ImPlotColormap__ImPlotColormap_Pastel,
    /// a.k.a. matplotlib "Paired" (n=12)
    Paired = sys::ImPlotColormap__ImPlotColormap_Paired,
    /// a.k.a. matplotlib "viridis" (n=11)
    Viridis = sys::ImPlotColormap__ImPlotColormap_Viridis,
    /// a.k.a. matplotlib "plasma" (n=11)
    Plasma = sys::ImPlotColormap__ImPlotColormap_Plasma,
    /// a.k.a. matplotlib/MATLAB "hot" (n=11)
    Hot = sys::ImPlotColormap__ImPlotColormap_Hot,
    /// a.k.a. matplotlib/MATLAB "cool" (n=11)
    Cool = sys::ImPlotColormap__ImPlotColormap_Cool,
    /// a.k.a. matplotlib/MATLAB "pink" (n=11)
    Pink = sys::ImPlotColormap__ImPlotColormap_Pink,
    /// a.k.a. MATLAB "jet" (n=11)
    Jet = sys::ImPlotColormap__ImPlotColormap_Jet,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum StyleVar {
    /// f32, line weight in pixels
    LineWeight = sys::ImPlotStyleVar__ImPlotStyleVar_LineWeight,
    /// u32,  marker specification
    Marker = sys::ImPlotStyleVar__ImPlotStyleVar_Marker,
    /// f32, marker size in pixels (roughly the marker's "radius")
    MarkerSize = sys::ImPlotStyleVar__ImPlotStyleVar_MarkerSize,
    /// f32, outline weight of markers in pixels
    MarkerWeight = sys::ImPlotStyleVar__ImPlotStyleVar_MarkerWeight,
    /// f32, error bar whisker width in pixels
    ErrorBarSize = sys::ImPlotStyleVar__ImPlotStyleVar_ErrorBarSize,
    /// f32, error bar whisker weight in pixels
    ErrorBarWeight = sys::ImPlotStyleVar__ImPlotStyleVar_ErrorBarWeight,
    /// f32, digital channels bit height (at 1) in pixels
    DigitalBitHeight = sys::ImPlotStyleVar__ImPlotStyleVar_DigitalBitHeight,
    /// f32, digital channels bit padding gap in pixels
    DigitalBitGap = sys::ImPlotStyleVar__ImPlotStyleVar_DigitalBitGap,
}

// --- Main plot structure -----------------------------------------------------------------------
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
    x_limits: Option<ImPlotRange>,
    /// Y axis limits, if present
    y_limits: Option<ImPlotRange>,
    /// Condition on which the x limits are set
    x_limit_condition: Option<Condition>,
    /// Condition on which the y limits are set (first y axis for now)
    y_limit_condition: Option<Condition>,
    /// Positions for custom X axis ticks, if any
    x_tick_positions: Option<Vec<f64>>,
    /// Labels for custom X axis ticks, if any. I'd prefer to store these together
    /// with the positions in one vector of an algebraic data type, but this would mean extra
    /// copies when it comes time to draw the plot because the C++ library expects separate lists.
    /// The data is stored as ImStrings because those are null-terminated, and since we have to
    /// convert to null-terminated data anyway, we may as well do that directly instead of cloning
    /// Strings and converting them afterwards.
    x_tick_labels: Option<Vec<ImString>>,
    /// Whether to also show the default X ticks when showing custom ticks or not
    show_x_default_ticks: bool,
    /// Positions for custom Y axis ticks, if any
    y_tick_positions: Option<Vec<f64>>,
    /// Labels for custom Y axis ticks, if any. I'd prefer to store these together
    /// with the positions in one vector of an algebraic data type, but this would mean extra
    /// copies when it comes time to draw the plot because the C++ library expects separate lists.
    /// The data is stored as ImStrings because those are null-terminated, and since we have to
    /// convert to null-terminated data anyway, we may as well do that directly instead of cloning
    /// Strings and converting them afterwards.
    y_tick_labels: Option<Vec<ImString>>,
    /// Whether to also show the default Y ticks when showing custom ticks or not
    show_y_default_ticks: bool,
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
            x_tick_positions: None,
            x_tick_labels: None,
            show_x_default_ticks: false,
            y_tick_positions: None,
            y_tick_labels: None,
            show_y_default_ticks: false,
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
    pub fn x_limits(mut self, limits: &ImPlotRange, condition: Condition) -> Self {
        self.x_limits = Some(*limits);
        self.x_limit_condition = Some(condition);
        self
    }

    /// Set the y limits of the plot
    #[inline]
    pub fn y_limits(mut self, limits: &ImPlotRange, condition: Condition) -> Self {
        self.y_limits = Some(*limits);
        self.y_limit_condition = Some(condition);
        self
    }

    /// Set X ticks without labels for the plot. The vector contains one label each in
    /// the form of a tuple `(label_position, label_string)`. The `show_default` setting
    /// determines whether the default ticks are also shown.
    #[inline]
    pub fn x_ticks(mut self, ticks: &Vec<f64>, show_default: bool) -> Self {
        self.x_tick_positions = Some(ticks.clone());
        self.show_x_default_ticks = show_default;
        self
    }

    /// Set X ticks without labels for the plot. The vector contains one label each in
    /// the form of a tuple `(label_position, label_string)`. The `show_default` setting
    /// determines whether the default ticks are also shown.
    #[inline]
    pub fn y_ticks(mut self, ticks: &Vec<f64>, show_default: bool) -> Self {
        self.y_tick_positions = Some(ticks.clone());
        self.show_y_default_ticks = show_default;
        self
    }

    /// Set X ticks with labels for the plot. The vector contains one position and label
    /// each in the form of a tuple `(label_position, label_string)`. The `show_default`
    /// setting determines whether the default ticks are also shown.
    #[inline]
    pub fn x_ticks_with_labels(
        mut self,
        tick_labels: &Vec<(f64, String)>,
        show_default: bool,
    ) -> Self {
        self.x_tick_positions = Some(tick_labels.iter().map(|x| x.0).collect());
        self.x_tick_labels = Some(tick_labels.iter().map(|x| im_str!("{}", x.1)).collect());
        self.show_x_default_ticks = show_default;
        self
    }

    /// Set Y ticks with labels for the plot. The vector contains one position and label
    /// each in the form of a tuple `(label_position, label_string)`. The `show_default`
    /// setting determines whether the default ticks are also shown.
    #[inline]
    pub fn y_ticks_with_labels(
        mut self,
        tick_labels: &Vec<(f64, String)>,
        show_default: bool,
    ) -> Self {
        self.y_tick_positions = Some(tick_labels.iter().map(|x| x.0).collect());
        self.y_tick_labels = Some(tick_labels.iter().map(|x| im_str!("{}", x.1)).collect());
        self.show_y_default_ticks = show_default;
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

    /// Internal helper function to set axis limits in case they are specified.
    fn maybe_set_axis_limits(&self) {
        // Set X limits if specified
        if let (Some(limits), Some(condition)) = (self.x_limits, self.x_limit_condition) {
            unsafe {
                sys::ImPlot_SetNextPlotLimitsX(limits.Min, limits.Max, condition as sys::ImGuiCond);
            }
        }

        // Set X limits if specified
        if let (Some(limits), Some(condition)) = (self.y_limits, self.y_limit_condition) {
            // TODO(4bb4) allow for specification of multiple y limits, not just the first
            let selected_y_axis = 0;
            unsafe {
                sys::ImPlot_SetNextPlotLimitsY(
                    limits.Min,
                    limits.Max,
                    condition as sys::ImGuiCond,
                    selected_y_axis,
                );
            }
        }
    }

    /// Internal helper function to set tick labels in case they are specified. This does the
    /// preparation work that is the same for both the X and Y axis plots, then calls the
    /// "set next plot ticks" wrapper functions for both X and Y.
    fn maybe_set_tick_labels(&self) {
        // Show x ticks if they are available
        if self.x_tick_positions.is_some() && self.x_tick_positions.as_ref().unwrap().len() > 0 {
            let mut pointer_vec; // The vector of pointers we create has to have a longer lifetime
            let labels_pointer = if let Some(labels_value) = &self.x_tick_labels {
                pointer_vec = labels_value
                    .iter()
                    .map(|x| x.as_ptr() as *const i8)
                    .collect::<Vec<*const i8>>();
                pointer_vec.as_mut_ptr()
            } else {
                std::ptr::null_mut()
            };

            unsafe {
                sys::ImPlot_SetNextPlotTicksXdoublePtr(
                    self.x_tick_positions.as_ref().unwrap().as_ptr(),
                    self.x_tick_positions.as_ref().unwrap().len() as i32,
                    labels_pointer,
                    self.show_x_default_ticks,
                )
            }
        }

        if self.y_tick_positions.is_some() && self.y_tick_positions.as_ref().unwrap().len() > 0 {
            let mut pointer_vec; // The vector of pointers we create has to have a longer lifetime
            let labels_pointer = if let Some(labels_value) = &self.y_tick_labels {
                pointer_vec = labels_value
                    .iter()
                    .map(|x| x.as_ptr() as *const i8)
                    .collect::<Vec<*const i8>>();
                pointer_vec.as_mut_ptr()
            } else {
                std::ptr::null_mut()
            };

            unsafe {
                sys::ImPlot_SetNextPlotTicksYdoublePtr(
                    self.y_tick_positions.as_ref().unwrap().as_ptr(),
                    self.y_tick_positions.as_ref().unwrap().len() as i32,
                    labels_pointer,
                    self.show_y_default_ticks,
                    0, // y axis selection, TODO(4bb4) make this configurable
                )
            }
        }
    }

    /// Attempt to show the plot. If this returns a token, the plot will actually
    /// be drawn. In this case, use the drawing functionality to draw things on the
    /// plot, and then call `end()` on the token when done with the plot.
    /// If none was returned, that means the plot is not rendered.
    ///
    /// For a convenient implementation of all this, use [`build()`](struct.Plot.html#method.build)
    /// instead.
    pub fn begin(&self) -> Option<PlotToken> {
        self.maybe_set_axis_limits();
        self.maybe_set_tick_labels();

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

// --- Actual plotting functionality -------------------------------------------------------------
/// Struct to provide functionality for plotting a line in a plot.
pub struct PlotLine {
    /// Label to show in the legend for this line
    label: String,
}

impl PlotLine {
    /// Create a new line to be plotted. Does not draw anything yet.
    pub fn new(label: &str) -> Self {
        Self {
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
            sys::ImPlot_PlotLinedoublePtrdoublePtr(
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

/// Struct to provide functionality for creating a scatter plot
pub struct PlotScatter {
    /// Label to show in the legend for this scatter plot
    label: String,
}

impl PlotScatter {
    /// Create a new scatter plot to be shown. Does not draw anything yet.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_owned(),
        }
    }

    /// Draw a previously-created scatter plot. Use this in closures passed to
    /// [`Plot::build()`](struct.Plot.html#method.build)
    pub fn plot(&self, x: &Vec<f64>, y: &Vec<f64>) {
        // If there is no data to plot, we stop here
        if x.len().min(y.len()) == 0 {
            return;
        }
        unsafe {
            sys::ImPlot_PlotScatterdoublePtrdoublePtr(
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

/// Struct to provide bar plotting functionality.
pub struct PlotBars {
    /// Label to show in the legend for this line
    label: String,

    /// Width of the bars, in plot coordinate terms
    bar_width: f64,

    /// Horizontal bar mode
    horizontal_bars: bool,
}

impl PlotBars {
    /// Create a new bar plot to be shown. Defaults to drawing vertical bars.
    /// Does not draw anything yet.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_owned(),
            bar_width: 0.67, // Default value taken from C++ implot
            horizontal_bars: false,
        }
    }

    /// Set the width of the bars
    pub fn with_bar_width(mut self, bar_width: f64) -> Self {
        self.bar_width = bar_width;
        self
    }

    /// Set the bars to be horizontal (default is vertical)
    pub fn with_horizontal_bars(mut self) -> Self {
        self.horizontal_bars = true;
        self
    }

    /// Draw a previously-created bar plot. Use this in closures passed to
    /// [`Plot::build()`](struct.Plot.html#method.build). The `axis_positions`
    /// specify where on the corersponding axis (X for vertical mode, Y for horizontal mode) the
    /// bar is drawn, and the `bar_values` specify what values the bars have.
    pub fn plot(&self, axis_positions: &Vec<f64>, bar_values: &Vec<f64>) {
        let number_of_points = axis_positions.len().min(bar_values.len());
        // If there is no data to plot, we stop here
        if number_of_points == 0 {
            return;
        }
        unsafe {
            // C++ implot has separate functions for the two variants, but the interfaces
            // are the same, so they are unified here. The x and y values have different
            // meanings though, hence the swapping around before they are passed to the
            // plotting function.
            let (plot_function, x, y);
            if self.horizontal_bars {
                plot_function = sys::ImPlot_PlotBarsHdoublePtrdoublePtr
                    as unsafe extern "C" fn(*const i8, *const f64, *const f64, i32, f64, i32, i32);
                x = bar_values;
                y = axis_positions;
            } else {
                plot_function = sys::ImPlot_PlotBarsdoublePtrdoublePtr
                    as unsafe extern "C" fn(*const i8, *const f64, *const f64, i32, f64, i32, i32);
                x = axis_positions;
                y = bar_values;
            };

            plot_function(
                im_str!("{}", self.label).as_ptr() as *const i8,
                x.as_ptr(),
                y.as_ptr(),
                number_of_points as i32, // "as" casts saturate as of Rust 1.45. This is safe here.
                self.bar_width,
                0,                                 // No offset
                std::mem::size_of::<f64>() as i32, // Stride, set to one f64 for the standard use case
            );
        }
    }
}

/// Struct to provide functionality for adding text within a plot
pub struct PlotText {
    /// Label to show in plot
    label: String,

    /// X component of the pixel offset to be used. Will be used independently of the actual plot
    /// scaling. Defaults to 0.
    pixel_offset_x: f32,

    /// Y component of the pixel offset to be used. Will be used independently of the actual plot
    /// scaling. Defaults to 0.
    pixel_offset_y: f32,
}

impl PlotText {
    /// Create a new text label to be shown. Does not draw anything yet.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.into(),
            pixel_offset_x: 0.0,
            pixel_offset_y: 0.0,
        }
    }

    /// Add a pixel offset to the text to be plotted. This offset will be independent of the
    /// scaling of the plot itself.
    pub fn with_pixel_offset(mut self, offset_x: f32, offset_y: f32) -> Self {
        self.pixel_offset_x = offset_x;
        self.pixel_offset_y = offset_y;
        self
    }

    /// Draw the text label in the plot at the given position, optionally vertically. Use this in
    /// closures passed to [`Plot::build()`](struct.Plot.html#method.build)
    pub fn plot(&self, x: f64, y: f64, vertical: bool) {
        // If there is nothing to show, don't do anything
        if self.label == "" {
            return;
        }

        unsafe {
            sys::ImPlot_PlotTextdouble(
                im_str!("{}", self.label).as_ptr() as *const i8,
                x,
                y,
                vertical,
                sys::ImVec2 {
                    x: self.pixel_offset_x,
                    y: self.pixel_offset_y,
                },
            );
        }
    }
}

// --- Color maps -----------------------------------------------------------------------------
/// Switch to one of the built-in preset colormaps. If samples is greater than 1, the map will be
/// linearly resampled.
pub fn set_colormap_from_preset(preset: Colormap, samples: u32) {
    unsafe {
        // "as" casts saturate as of Rust 1.45. This is safe here, and at least the enum
        // values are not expected to go outside the range of an i32 anyway, so there is no
        // risk of changed values.
        sys::ImPlot_SetColormapPlotColormap(preset as i32, samples as i32);
    }
}

/// Set a custom colormap in the form of a vector of colors.
pub fn set_colormap_from_vec(colors: Vec<ImVec4>) {
    unsafe {
        sys::ImPlot_SetColormapVec4Ptr(colors.as_ptr(), colors.len() as i32);
    }
}

// --- Push/pop utils -------------------------------------------------------------------------
// Currently not in a struct yet. imgui-rs has some smarts about dealing with stacks, in particular
// leak detection, which I'd like to replicate here at some point.
/// Push a style color to the stack, giving an element and the four components of the color.
/// The components should be between 0.0 (no intensity) and 1.0 (full intensity).
/// The return value is a token that gets used for removing the style color from the stack again:
/// ```no_run
/// # use implot::{push_style_color, PlotColorElement};
/// let pushed_var = push_style_color(&PlotColorElement::Line, 1.0, 1.0, 1.0, 0.2);
/// // Plot some things
/// pushed_var.pop();
/// ```
pub fn push_style_color(
    element: &PlotColorElement,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
) -> StyleColorToken {
    unsafe {
        sys::ImPlot_PushStyleColorVec4(
            *element as sys::ImPlotCol,
            sys::ImVec4 {
                x: red,
                y: green,
                z: blue,
                w: alpha,
            },
        );
    }
    StyleColorToken { was_popped: false }
}

/// Tracks a change pushed to the style color stack
pub struct StyleColorToken {
    /// Whether this token has been popped or not.
    /// TODO(4bb4) figure out if it is a good idea to warn about this not being popped when it is
    /// dropped - this may not be a good idea since users may want to push some style vars for
    /// longer durations.
    was_popped: bool,
}

impl StyleColorToken {
    pub fn pop(mut self) {
        if self.was_popped {
            panic!("Attempted to pop a style color token twice.")
        }
        self.was_popped = true;
        unsafe {
            sys::ImPlot_PopStyleColor(1);
        }
    }
}

/// Push a f32 style variable to the stack. The returned token is used for removing
/// the variable from the stack again:
/// ```no_run
/// # use implot::{push_style_var_f32, StyleVar};
/// let pushed_var = push_style_var_f32(&StyleVar::LineWeight, 11.0);
/// // Plot some things
/// pushed_var.pop();
/// ```
pub fn push_style_var_f32(element: &StyleVar, value: f32) -> StyleVarToken {
    unsafe {
        sys::ImPlot_PushStyleVarFloat(*element as sys::ImPlotStyleVar, value);
    }
    StyleVarToken { was_popped: false }
}

/// Push an u32 style variable to the stack. The only u32 style variable is Marker
/// at the moment, for that, use something like
/// ```no_run
/// # use implot::{push_style_var_u32, StyleVar, Marker};
/// let markerchoice = push_style_var_u32(&StyleVar::Marker, Marker::CROSS.bits());
/// // plot things
/// markerchoice.pop()
/// ```
pub fn push_style_var_u32(element: &StyleVar, value: u32) -> StyleVarToken {
    // It is a bit funky that we take an i32 here, but the enum that gets created
    // by bindgen contains u32 values, so we do the same but convert them to the
    // internal i32 values here. Since this could overflow if a too large u32 value
    // was passed, we do a safe conversion here, panicking if it fails.
    let value_i32 =
        i32::try_from(value).expect("Invalid style variable passed, has to fit in an i32");
    unsafe {
        sys::ImPlot_PushStyleVarInt(*element as sys::ImPlotStyleVar, value_i32);
    }
    StyleVarToken { was_popped: false }
}

/// Tracks a change pushed to the style variable stack
pub struct StyleVarToken {
    /// Whether this token has been popped or not.
    /// TODO(4bb4) figure out if it is a good idea to warn about this not being popped when it is
    /// dropped - this may not be a good idea since users may want to push some style vars for
    /// longer durations.
    was_popped: bool,
}

impl StyleVarToken {
    /// Pop this token from the stack.
    pub fn pop(mut self) {
        if self.was_popped {
            panic!("Attempted to pop a style var token twice.")
        }
        self.was_popped = true;
        unsafe {
            sys::ImPlot_PopStyleVar(1);
        }
    }
}

// --- Miscellaneous -----------------------------------------------------------------------------
/// Returns true if the plot area in the current or most recent plot is hovered.
pub fn is_plot_hovered() -> bool {
    unsafe { sys::ImPlot_IsPlotHovered() }
}

/// Returns true if the current or most recent plot is queried
pub fn is_plot_queried() -> bool {
    unsafe { sys::ImPlot_IsPlotQueried() }
}

/// Returns the mouse position in x,y coordinates of the current or most recent plot. Currently
/// pertains to whatever Y axis was most recently selected. TODO(4bb4) add y axis selection
pub fn get_plot_mouse_position() -> ImPlotPoint {
    let y_axis_selection = 0;
    unsafe { sys::ImPlot_GetPlotMousePos(y_axis_selection) }
}

/// Returns the current or most recent plot axis range. Currently pertains to whatever Y axis was
/// most recently selected. TODO(4bb4) add y axis selection
pub fn get_plot_limits() -> ImPlotLimits {
    let y_axis_selection = 0;
    unsafe { sys::ImPlot_GetPlotLimits(y_axis_selection) }
}

/// Returns the query limits of the current or most recent plot.  Currently pertains to whatever Y
/// axis was most recently selected. TODO(4bb4) add y axis selection
pub fn get_plot_query() -> ImPlotLimits {
    let y_axis_selection = 0;
    unsafe { sys::ImPlot_GetPlotQuery(y_axis_selection) }
}

// --- Demo window -------------------------------------------------------------------------------
/// Show the demo window for poking around what functionality implot has to
/// offer. Note that not all of this is necessarily implemented in implot-rs
/// already - if you find something missing you'd really like, raise an issue.
// This requires implot_demo.cpp to be in the list of sources in implot-sys.
pub fn show_demo_window(show: &mut bool) {
    unsafe {
        implot_sys::ImPlot_ShowDemoWindow(show);
    }
}
