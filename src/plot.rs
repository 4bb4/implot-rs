//! # Plot module
//!
//! This module defines the `Plot` struct, which is used to create a 2D plot that will
//! contain all other objects that can be created using this library.
use crate::{Context, PlotLocation, PlotOrientation, PlotUi, YAxisChoice, NUMBER_OF_Y_AXES};
use bitflags::bitflags;
pub use imgui::Condition;
use imgui::{im_str, ImString};
use implot_sys as sys;
pub use sys::{ImPlotLimits, ImPlotPoint, ImPlotRange, ImVec2, ImVec4};

const DEFAULT_PLOT_SIZE_X: f32 = 400.0;
const DEFAULT_PLOT_SIZE_Y: f32 = 400.0;

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
        /// Use an aspect ratio of 1:1 for the plot
        const AXIS_EQUAL = sys::ImPlotFlags__ImPlotFlags_Equal;
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

/// Struct to represent an ImPlot. This is the main construct used to contain all kinds of plots in ImPlot.
///
/// `Plot` is to be used (within an imgui window) with the following pattern:
/// ```no_run
/// # use implot;
/// let plotting_context = implot::Context::create();
/// let plot_ui = plotting_context.get_plot_ui();
/// implot::Plot::new("my title")
///     .size(300.0, 200.0) // other things such as .x_label("some_label") can be added too
///     .build(&plot_ui, || {
///         // Do things such as plotting lines
///     });
///
/// ```
/// (If you are coming from the C++ implementation or the C bindings: build() calls both
/// begin() and end() internally)
pub struct Plot {
    /// Title of the plot, shown on top. Stored as ImString because that's what we'll use
    /// afterwards, and this ensures the ImString itself will stay alive long enough for the plot.
    title: ImString,
    /// Size of the plot in x direction, in the same units imgui uses.
    size_x: f32,
    /// Size of the plot in y direction, in the same units imgui uses.
    size_y: f32,
    /// Label of the x axis, shown on the bottom. Stored as ImString because that's what we'll use
    /// afterwards, and this ensures the ImString itself will stay alive long enough for the plot.
    x_label: ImString,
    /// Label of the y axis, shown on the left. Stored as ImString because that's what we'll use
    /// afterwards, and this ensures the ImString itself will stay alive long enough for the plot.
    y_label: ImString,
    /// X axis limits, if present
    x_limits: Option<ImPlotRange>,
    /// Y axis limits, if present
    y_limits: [Option<ImPlotRange>; NUMBER_OF_Y_AXES],
    /// Condition on which the x limits are set
    x_limit_condition: Option<Condition>,
    /// Condition on which the y limits are set for each of the axes
    y_limit_condition: [Option<Condition>; NUMBER_OF_Y_AXES],
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
    y_tick_positions: [Option<Vec<f64>>; NUMBER_OF_Y_AXES],
    /// Labels for custom Y axis ticks, if any. I'd prefer to store these together
    /// with the positions in one vector of an algebraic data type, but this would mean extra
    /// copies when it comes time to draw the plot because the C++ library expects separate lists.
    /// The data is stored as ImStrings because those are null-terminated, and since we have to
    /// convert to null-terminated data anyway, we may as well do that directly instead of cloning
    /// Strings and converting them afterwards.
    y_tick_labels: [Option<Vec<ImString>>; NUMBER_OF_Y_AXES],
    /// Whether to also show the default Y ticks when showing custom ticks or not
    show_y_default_ticks: [bool; NUMBER_OF_Y_AXES],
    /// Configuration for the legend, if specified. The tuple contains location, orientation
    /// and a boolean (true means legend is outside of plot, false means within). If nothing
    /// is set, implot's defaults are used. Note also  that if these are set, then implot's
    /// interactive legend configuration does not work because it is overridden by the settings
    /// here.
    legend_configuration: Option<(PlotLocation, PlotOrientation, bool)>,
    /// Flags relating to the plot TODO(4bb4) make those into bitflags
    plot_flags: sys::ImPlotFlags,
    /// Flags relating to the X axis of the plot TODO(4bb4) make those into bitflags
    x_flags: sys::ImPlotAxisFlags,
    /// Flags relating to the each of the Y axes of the plot TODO(4bb4) make those into bitflags
    y_flags: [sys::ImPlotAxisFlags; NUMBER_OF_Y_AXES],
}

impl Plot {
    /// Create a new plot with some defaults set. Does not draw anything yet.
    /// Note that this uses antialiasing by default, unlike the C++ API. If you are seeing artifacts or weird rendering, try disabling it.
    pub fn new(title: &str) -> Self {
        // Needed for initialization, see https://github.com/rust-lang/rust/issues/49147
        const POS_NONE: Option<Vec<f64>> = None;
        const TICK_NONE: Option<Vec<ImString>> = None;

        // TODO(4bb4) question these defaults, maybe remove some of them
        Self {
            title: im_str!("{}", title),
            size_x: DEFAULT_PLOT_SIZE_X,
            size_y: DEFAULT_PLOT_SIZE_Y,
            x_label: im_str!("").into(),
            y_label: im_str!("").into(),
            x_limits: None,
            y_limits: [None; NUMBER_OF_Y_AXES],
            x_limit_condition: None,
            y_limit_condition: [None; NUMBER_OF_Y_AXES],
            x_tick_positions: None,
            x_tick_labels: None,
            show_x_default_ticks: false,
            y_tick_positions: [POS_NONE; NUMBER_OF_Y_AXES],
            y_tick_labels: [TICK_NONE; NUMBER_OF_Y_AXES],
            show_y_default_ticks: [false; NUMBER_OF_Y_AXES],
            legend_configuration: None,
            plot_flags: PlotFlags::ANTIALIASED.bits() as sys::ImPlotFlags,
            x_flags: AxisFlags::NONE.bits() as sys::ImPlotAxisFlags,
            y_flags: [AxisFlags::NONE.bits() as sys::ImPlotAxisFlags; NUMBER_OF_Y_AXES],
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
        self.x_label = im_str!("{}", label);
        self
    }

    /// Set the y label of the plot
    #[inline]
    pub fn y_label(mut self, label: &str) -> Self {
        self.y_label = im_str!("{}", label);
        self
    }

    /// Set the x limits of the plot
    #[inline]
    pub fn x_limits(mut self, limits: &ImPlotRange, condition: Condition) -> Self {
        self.x_limits = Some(*limits);
        self.x_limit_condition = Some(condition);
        self
    }

    /// Set the Y limits of the plot for the given Y axis. Call multiple times
    /// to set for multiple axes.
    #[inline]
    pub fn y_limits(
        mut self,
        limits: &ImPlotRange,
        y_axis_choice: YAxisChoice,
        condition: Condition,
    ) -> Self {
        let axis_index = y_axis_choice as usize;
        self.y_limits[axis_index] = Some(*limits);
        self.y_limit_condition[axis_index] = Some(condition);
        self
    }

    /// Set X ticks without labels for the plot. The vector contains one label each in
    /// the form of a tuple `(label_position, label_string)`. The `show_default` setting
    /// determines whether the default ticks are also shown.
    #[inline]
    pub fn x_ticks(mut self, ticks: &[f64], show_default: bool) -> Self {
        self.x_tick_positions = Some(ticks.into());
        self.show_x_default_ticks = show_default;
        self
    }

    /// Set X ticks without labels for the plot. The vector contains one label each in
    /// the form of a tuple `(label_position, label_string)`. The `show_default` setting
    /// determines whether the default ticks are also shown.
    #[inline]
    pub fn y_ticks(
        mut self,
        y_axis_choice: YAxisChoice,
        ticks: &[f64],
        show_default: bool,
    ) -> Self {
        let axis_index = y_axis_choice as usize;
        self.y_tick_positions[axis_index] = Some(ticks.into());
        self.show_y_default_ticks[axis_index] = show_default;
        self
    }

    /// Set X ticks with labels for the plot. The vector contains one position and label
    /// each in the form of a tuple `(label_position, label_string)`. The `show_default`
    /// setting determines whether the default ticks are also shown.
    #[inline]
    pub fn x_ticks_with_labels(
        mut self,
        tick_labels: &[(f64, String)],
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
        y_axis_choice: YAxisChoice,
        tick_labels: &[(f64, String)],
        show_default: bool,
    ) -> Self {
        let axis_index = y_axis_choice as usize;
        self.y_tick_positions[axis_index] = Some(tick_labels.iter().map(|x| x.0).collect());
        self.y_tick_labels[axis_index] =
            Some(tick_labels.iter().map(|x| im_str!("{}", x.1)).collect());
        self.show_y_default_ticks[axis_index] = show_default;
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

    /// Set the axis flags for the selected Y axis in this plot
    #[inline]
    pub fn with_y_axis_flags(mut self, y_axis_choice: YAxisChoice, flags: &AxisFlags) -> Self {
        let axis_index = y_axis_choice as usize;
        self.y_flags[axis_index] = flags.bits() as sys::ImPlotAxisFlags;
        self
    }

    /// Set the legend location, orientation and whether it is to be drawn outside the plot
    #[inline]
    pub fn with_legend_location(
        mut self,
        location: &PlotLocation,
        orientation: &PlotOrientation,
        outside: bool,
    ) -> Self {
        self.legend_configuration = Some((*location, *orientation, outside));
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

        // Set Y limits if specified
        self.y_limits
            .iter()
            .zip(self.y_limit_condition.iter())
            .enumerate()
            .for_each(|(k, (limits, condition))| {
                if let (Some(limits), Some(condition)) = (limits, condition) {
                    unsafe {
                        sys::ImPlot_SetNextPlotLimitsY(
                            limits.Min,
                            limits.Max,
                            *condition as sys::ImGuiCond,
                            k as i32,
                        );
                    }
                }
            });
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

        self.y_tick_positions
            .iter()
            .zip(self.y_tick_labels.iter())
            .zip(self.show_y_default_ticks.iter())
            .enumerate()
            .for_each(|(k, ((positions, labels), show_defaults))| {
                if positions.is_some() && positions.as_ref().unwrap().len() > 0 {
                    // The vector of pointers we create has to have a longer lifetime
                    let mut pointer_vec;
                    let labels_pointer = if let Some(labels_value) = &labels {
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
                            positions.as_ref().unwrap().as_ptr(),
                            positions.as_ref().unwrap().len() as i32,
                            labels_pointer,
                            *show_defaults,
                            k as i32,
                        )
                    }
                }
            });
    }

    /// Attempt to show the plot. If this returns a token, the plot will actually
    /// be drawn. In this case, use the drawing functionality to draw things on the
    /// plot, and then call `end()` on the token when done with the plot.
    /// If none was returned, that means the plot is not rendered.
    ///
    /// For a convenient implementation of all this, use [`build()`](struct.Plot.html#method.build)
    /// instead.
    pub fn begin(&self, plot_ui: &PlotUi) -> Option<PlotToken> {
        self.maybe_set_axis_limits();
        self.maybe_set_tick_labels();

        let should_render = unsafe {
            sys::ImPlot_BeginPlot(
                self.title.as_ptr(),
                self.x_label.as_ptr(),
                self.y_label.as_ptr(),
                sys::ImVec2 {
                    x: self.size_x as f32,
                    y: self.size_y as f32,
                },
                self.plot_flags,
                self.x_flags,
                self.y_flags[0],
                self.y_flags[1],
                self.y_flags[2],
            )
        };

        if should_render {
            // Configure legend location, if one was set. This has to be called between begin() and
            // end(), but since only the last call to it actually affects the outcome, I'm adding
            // it here instead of as a freestanding function. If this is too restrictive (for
            // example, if you want to set the location based on code running _during_ the plotting
            // for some reason), file an issue and we'll move it.
            if let Some(legend_config) = &self.legend_configuration {
                // We introduce variables with typechecks here to safeguard against accidental
                // changes in order in the config tuple
                let location: PlotLocation = legend_config.0;
                let orientation: PlotOrientation = legend_config.1;
                let outside_plot: bool = legend_config.2;
                unsafe {
                    sys::ImPlot_SetLegendLocation(location as i32, orientation as i32, outside_plot)
                }
            }

            Some(PlotToken {
                context: plot_ui.context,
                plot_title: self.title.clone(),
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
    pub fn build<F: FnOnce()>(self, plot_ui: &PlotUi, f: F) {
        if let Some(token) = self.begin(plot_ui) {
            f();
            token.end()
        }
    }
}

/// Tracks a plot that must be ended by calling `.end()`
pub struct PlotToken {
    context: *const Context,
    /// For better error messages
    plot_title: ImString,
}

impl PlotToken {
    /// End a previously begin()'ed plot.
    pub fn end(mut self) {
        self.context = std::ptr::null();
        unsafe { sys::ImPlot_EndPlot() };
    }
}

impl Drop for PlotToken {
    fn drop(&mut self) {
        if !self.context.is_null() && !std::thread::panicking() {
            panic!(
                "Warning: A PlotToken for plot \"{}\" was not called end() on",
                self.plot_title
            );
        }
    }
}
