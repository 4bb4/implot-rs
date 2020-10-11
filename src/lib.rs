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

#[macro_use]
extern crate lazy_static;

pub use sys::imgui::Condition;
// TODO(4bb4) facade-wrap these
pub use sys::{ImPlotLimits, ImPlotPoint, ImPlotRange, ImVec2, ImVec4};

// Plot struct and associated enums
pub mod context;
pub mod plot;
pub mod plot_elements;

pub use context::Context;
pub use plot::{AxisFlags, Plot, PlotFlags};
pub use plot_elements::{PlotBars, PlotLine, PlotScatter, PlotText};

// --- Enum definitions --------------------------------------------------------------------------
// Things that are to be combined like flags are done using bitflags, and things that are meant
// as enumerations in the traditional sense are plain enums.

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

// --- Main plot structure -----------------------------------------------------------------------

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
