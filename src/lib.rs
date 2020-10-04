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
pub use sys::imgui::Condition;
use sys::imgui::{im_str, ImString};
// TODO(4bb4) facade-wrap these
pub use sys::{ImPlotLimits, ImPlotPoint, ImPlotRange, ImVec2, ImVec4};

const DEFAULT_PLOT_SIZE_X: f32 = 400.0;
const DEFAULT_PLOT_SIZE_Y: f32 = 400.0;

// --- Enum definitions --------------------------------------------------------------------------
// Things that are to be combined like flags are done using bitflags, and things that are meant
// as enumerations in the traditional sense are plain enums.

bitflags! {
    /// Flags for customizing plot behavior and interaction. Documentation copied from implot.h for
    /// convenience. ImPlot itself also has a "CanvasOnly" flag, which can be emulated here with
    /// the combination of `NO_LEGEND`, `NO_MENUS`, `NO_BOX_SELECT` and `NO_MOUSE_POSITION`.
    #[repr(transparent)]
    pub struct PlotFlags: u32 {
        /// "Default" according to original docs
        const NONE = sys::ImPlotFlags__ImPlotFlags_None;
        /// Plot items will not be highlighted when their legend entry is hovered
        const NO_LEGEND = sys::ImPlotFlags__ImPlotFlags_NoLegend;
        /// The user will not be able to open context menus with double-right click
        const NO_MENUS = sys::ImPlotFlags__ImPlotFlags_NoMenus;
        /// The user will not be able to box-select with right-mouse
        const NO_BOX_SELECT = sys::ImPlotFlags__ImPlotFlags_NoBoxSelect;
        /// The mouse position, in plot coordinates, will not be displayed
        const NO_MOUSE_POSITION = sys::ImPlotFlags__ImPlotFlags_NoMousePos;
        /// Plot items will not be highlighted when their legend entry is hovered
        const NO_HIGHLIGHT = sys::ImPlotFlags__ImPlotFlags_NoHighlight;
        /// A child window region will not be used to capture mouse scroll (can boost performance
        /// for single ImGui window applications)
        const NO_CHILD = sys::ImPlotFlags__ImPlotFlags_NoChild;
        /// Enable a 2nd y axis
        const Y_AXIS_2 = sys::ImPlotFlags__ImPlotFlags_YAxis2;
        /// Enable a 3nd y axis
        const Y_AXIS_3 = sys::ImPlotFlags__ImPlotFlags_YAxis3;
        /// The user will be able to draw query rects with middle-mouse
        const QUERY = sys::ImPlotFlags__ImPlotFlags_Query;
        /// The default mouse cursor will be replaced with a crosshair when hovered
        const CROSSHAIRS = sys::ImPlotFlags__ImPlotFlags_Crosshairs;
        /// Plot data outside the plot area will be culled from rendering
        const ANTIALIASED = sys::ImPlotFlags__ImPlotFlags_AntiAliased;
    }
}

bitflags! {
    /// Axis flags. Documentation copied from implot.h for convenience. ImPlot itself also
    /// has `Lock`, which combines `LOCK_MIN` and `LOCK_MAX`, and `NoDecorations`, which combines
    /// `NO_GRID_LINES`, `NO_TICK_MARKS` and `NO_TICK_LABELS`.
    #[repr(transparent)]
    pub struct AxisFlags: u32 {
        /// "Default" according to original docs
        const NONE = sys::ImPlotAxisFlags__ImPlotAxisFlags_None;
        /// Grid lines will not be displayed
        const NO_GRID_LINES = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoGridLines;
        /// Tick marks will not be displayed
        const NO_TICK_MARKS = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoTickMarks;
        /// Text labels will not be displayed
        const NO_TICK_LABELS = sys::ImPlotAxisFlags__ImPlotAxisFlags_NoTickLabels;
        /// A logartithmic (base 10) axis scale will be used (mutually exclusive with AxisFlags::TIME)
        const LOG_SCALE = sys::ImPlotAxisFlags__ImPlotAxisFlags_LogScale;
        /// Axis will display date/time formatted labels (mutually exclusive with AxisFlags::LOG_SCALE)
        const TIME = sys::ImPlotAxisFlags__ImPlotAxisFlags_Time;
        /// The axis will be inverted
        const INVERT = sys::ImPlotAxisFlags__ImPlotAxisFlags_Invert;
        /// The axis minimum value will be locked when panning/zooming
        const LOCK_MIN = sys::ImPlotAxisFlags__ImPlotAxisFlags_LockMin;
        /// The axis maximum value will be locked when panning/zooming
        const LOCK_MAX = sys::ImPlotAxisFlags__ImPlotAxisFlags_LockMax;
    }
}

/// Markers, documentation copied from implot.h for convenience.
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum Marker {
    /// no marker
    None = sys::ImPlotMarker__ImPlotMarker_None,
    /// a circle marker will be rendered at each point
    Circle = sys::ImPlotMarker__ImPlotMarker_Circle,
    /// a square maker will be rendered at each point
    Square = sys::ImPlotMarker__ImPlotMarker_Square,
    /// a diamond marker will be rendered at each point
    Diamond = sys::ImPlotMarker__ImPlotMarker_Diamond,
    /// an upward-pointing triangle marker will up rendered at each point
    Up = sys::ImPlotMarker__ImPlotMarker_Up,
    /// an downward-pointing triangle marker will up rendered at each point
    Down = sys::ImPlotMarker__ImPlotMarker_Down,
    /// an leftward-pointing triangle marker will up rendered at each point
    Left = sys::ImPlotMarker__ImPlotMarker_Left,
    /// an rightward-pointing triangle marker will up rendered at each point
    Right = sys::ImPlotMarker__ImPlotMarker_Right,
    /// a cross marker will be rendered at each point (not filled)
    Cross = sys::ImPlotMarker__ImPlotMarker_Cross,
    /// a plus marker will be rendered at each point (not filled)
    Plus = sys::ImPlotMarker__ImPlotMarker_Plus,
    /// a asterisk marker will be rendered at each point (not filled)
    Asterisk = sys::ImPlotMarker__ImPlotMarker_Asterisk,
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
    /// Legend background color (defaults to ImGuiCol_PopupBg)
    LegendBackground = sys::ImPlotCol__ImPlotCol_LegendBg,
    /// Legend border color (defaults to ImPlotCol_PlotBorder)
    LegendBorder = sys::ImPlotCol__ImPlotCol_LegendBorder,
    /// Legend text color (defaults to ImPlotCol_InlayText)
    LegendText = sys::ImPlotCol__ImPlotCol_LegendText,
    /// Plot title text color (defaults to ImGuiCol_Text)
    TitleText = sys::ImPlotCol__ImPlotCol_TitleText,
    /// Color of text appearing inside of plots (defaults to ImGuiCol_Text)
    InlayText = sys::ImPlotCol__ImPlotCol_InlayText,
    /// X-axis label and tick lables color (defaults to ImGuiCol_Text)
    XAxis = sys::ImPlotCol__ImPlotCol_XAxis,
    /// X-axis grid color (defaults to 25% ImPlotCol_XAxis)
    XAxisGrid = sys::ImPlotCol__ImPlotCol_XAxisGrid,
    /// Y-axis label and tick labels color (defaults to ImGuiCol_Text)
    YAxis = sys::ImPlotCol__ImPlotCol_YAxis,
    /// Y-axis grid color (defaults to 25% ImPlotCol_YAxis)
    YAxisGrid = sys::ImPlotCol__ImPlotCol_YAxisGrid,
    /// 2nd y-axis label and tick labels color (defaults to ImGuiCol_Text)
    YAxis2 = sys::ImPlotCol__ImPlotCol_YAxis2,
    /// 2nd y-axis grid/label color (defaults to 25% ImPlotCol_YAxis2)
    YAxisGrid2 = sys::ImPlotCol__ImPlotCol_YAxisGrid2,
    /// 3rd y-axis label and tick labels color (defaults to ImGuiCol_Text)
    YAxis3 = sys::ImPlotCol__ImPlotCol_YAxis3,
    /// 3rd y-axis grid/label color (defaults to 25% ImPlotCol_YAxis3)
    YAxisGrid3 = sys::ImPlotCol__ImPlotCol_YAxisGrid3,
    /// Box-selection color (defaults to yellow)
    Selection = sys::ImPlotCol__ImPlotCol_Selection,
    /// crosshairs color (defaults to ImPlotCol_PlotBorder)
    Crosshairs = sys::ImPlotCol__ImPlotCol_Crosshairs,
    /// Box-query color (defaults to green)
    Query = sys::ImPlotCol__ImPlotCol_Query,
}

/// Colormap choice. Documentation copied from implot.h for convenience.
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Colormap {
    /// ImPlot default colormap (n=10). Called "Standard" here because Default is reserved.
    Standard = sys::ImPlotColormap__ImPlotColormap_Default,
    /// a.k.a. seaborn deep (n=10)
    Deep = sys::ImPlotColormap__ImPlotColormap_Deep,
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
    /// f32, alpha modifier applied to all plot item fills
    FillAlpha = sys::ImPlotStyleVar__ImPlotStyleVar_FillAlpha,
    /// f32, error bar whisker width in pixels
    ErrorBarSize = sys::ImPlotStyleVar__ImPlotStyleVar_ErrorBarSize,
    /// f32, error bar whisker weight in pixels
    ErrorBarWeight = sys::ImPlotStyleVar__ImPlotStyleVar_ErrorBarWeight,
    /// f32, digital channels bit height (at 1) in pixels
    DigitalBitHeight = sys::ImPlotStyleVar__ImPlotStyleVar_DigitalBitHeight,
    /// f32, digital channels bit padding gap in pixels
    DigitalBitGap = sys::ImPlotStyleVar__ImPlotStyleVar_DigitalBitGap,
    /// f32,  thickness of border around plot area
    PlotBorderSize = sys::ImPlotStyleVar__ImPlotStyleVar_PlotBorderSize,
    /// f32,  alpha multiplier applied to minor axis grid lines
    MinorAlpha = sys::ImPlotStyleVar__ImPlotStyleVar_MinorAlpha,
    /// ImVec2, major tick lengths for X and Y axes
    MajorTickLen = sys::ImPlotStyleVar__ImPlotStyleVar_MajorTickLen,
    /// ImVec2, minor tick lengths for X and Y axes
    MinorTickLen = sys::ImPlotStyleVar__ImPlotStyleVar_MinorTickLen,
    /// ImVec2, line thickness of major ticks
    MajorTickSize = sys::ImPlotStyleVar__ImPlotStyleVar_MajorTickSize,
    /// ImVec2, line thickness of minor ticks
    MinorTickSize = sys::ImPlotStyleVar__ImPlotStyleVar_MinorTickSize,
    /// ImVec2, line thickness of major grid lines
    MajorGridSize = sys::ImPlotStyleVar__ImPlotStyleVar_MajorGridSize,
    /// ImVec2, line thickness of minor grid lines
    MinorGridSize = sys::ImPlotStyleVar__ImPlotStyleVar_MinorGridSize,
    /// ImVec2, padding between widget frame and plot area and/or labels
    PlotPadding = sys::ImPlotStyleVar__ImPlotStyleVar_PlotPadding,
    /// ImVec2, padding between axes labels, tick labels, and plot edge
    LabelPadding = sys::ImPlotStyleVar__ImPlotStyleVar_LabelPadding,
    /// ImVec2, legend padding from top-left of plot
    LegendPadding = sys::ImPlotStyleVar__ImPlotStyleVar_LegendPadding,
    /// ImVec2, padding between plot edge and interior info text
    InfoPadding = sys::ImPlotStyleVar__ImPlotStyleVar_InfoPadding,
    /// ImVec2, minimum size plot frame can be when shrunk
    PlotMinSize = sys::ImPlotStyleVar__ImPlotStyleVar_PlotMinSize,
}

// --- Context -----------------------------------------------------------------------------------
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
pub struct Context {
    raw: *mut sys::ImPlotContext,
}

impl Context {
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
            plot_flags: PlotFlags::NONE.bits() as sys::ImPlotFlags,
            x_flags: AxisFlags::NONE.bits() as sys::ImPlotAxisFlags,
            y_flags: AxisFlags::NONE.bits() as sys::ImPlotAxisFlags,
            y2_flags: AxisFlags::NONE.bits() as sys::ImPlotAxisFlags,
            y3_flags: AxisFlags::NONE.bits() as sys::ImPlotAxisFlags,
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

/// Push an u32 style variable to the stack. The only i32 style variable is Marker
/// at the moment, for that, use something like
/// ```no_run
/// # use implot::{push_style_var_i32, StyleVar, Marker};
/// let markerchoice = push_style_var_i32(&StyleVar::Marker, Marker::Cross as i32);
/// // plot things
/// markerchoice.pop()
/// ```
pub fn push_style_var_i32(element: &StyleVar, value: i32) -> StyleVarToken {
    unsafe {
        sys::ImPlot_PushStyleVarInt(*element as sys::ImPlotStyleVar, value);
    }
    StyleVarToken { was_popped: false }
}

/// Push an ImVec2 style variable to the stack. The returned token is used for removing
/// the variable from the stack again.
pub fn push_style_var_imvec2(element: &StyleVar, value: ImVec2) -> StyleVarToken {
    unsafe {
        sys::ImPlot_PushStyleVarVec2(*element as sys::ImPlotStyleVar, value);
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
    let mut point = ImPlotPoint { x: 0.0, y: 0.0 }; // doesn't seem to have default()
    unsafe {
        sys::ImPlot_GetPlotMousePos(&mut point as *mut ImPlotPoint, y_axis_selection);
    }
    point
}

/// Returns the current or most recent plot axis range. Currently pertains to whatever Y axis was
/// most recently selected. TODO(4bb4) add y axis selection
pub fn get_plot_limits() -> ImPlotLimits {
    let y_axis_selection = 0;
    // ImPlotLimits doesn't seem to have default()
    let mut limits = ImPlotLimits {
        X: ImPlotRange { Min: 0.0, Max: 0.0 },
        Y: ImPlotRange { Min: 0.0, Max: 0.0 },
    };
    unsafe {
        sys::ImPlot_GetPlotLimits(&mut limits as *mut ImPlotLimits, y_axis_selection);
    }
    limits
}

/// Returns the query limits of the current or most recent plot.  Currently pertains to whatever Y
/// axis was most recently selected. TODO(4bb4) add y axis selection
pub fn get_plot_query() -> ImPlotLimits {
    let y_axis_selection = 0;
    // ImPlotLimits doesn't seem to have default()
    let mut limits = ImPlotLimits {
        X: ImPlotRange { Min: 0.0, Max: 0.0 },
        Y: ImPlotRange { Min: 0.0, Max: 0.0 },
    };
    unsafe {
        sys::ImPlot_GetPlotQuery(&mut limits as *mut ImPlotLimits, y_axis_selection);
    }
    limits
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
