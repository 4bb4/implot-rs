//! # Plot elements module
//!
//! This module defines the various structs that can be used for drawing different things such
//! as lines, bars, scatter plots and text in a plot. For the module to create plots themselves,
//! see `plot`.
use crate::sys;
use imgui::{im_str, ImString};
use std::os::raw::c_char;

pub use crate::sys::ImPlotPoint;

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
    pub fn plot(&self, x: &[f64], y: &[f64]) {
        // If there is no data to plot, we stop here
        if x.len().min(y.len()) == 0 {
            return;
        }
        unsafe {
            sys::ImPlot_PlotLinedoublePtrdoublePtr(
                im_str!("{}", self.label).as_ptr() as *const c_char,
                x.as_ptr(),
                y.as_ptr(),
                x.len().min(y.len()) as i32, // "as" casts saturate as of Rust 1.45. This is safe here.
                0,                           // No offset
                std::mem::size_of::<f64>() as i32, // Stride, set to one f64 for the standard use case
            );
        }
    }
}

/// Struct to provide functionality for plotting a line in a plot with stairs style.
pub struct PlotStairs {
    /// Label to show in the legend for this line
    label: String,
}

impl PlotStairs {
    /// Create a new line to be plotted. Does not draw anything yet.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_owned(),
        }
    }

    /// Plot a stairs style line. Use this in closures passed to
    /// [`Plot::build()`](struct.Plot.html#method.build)
    pub fn plot(&self, x: &[f64], y: &[f64]) {
        // If there is no data to plot, we stop here
        if x.len().min(y.len()) == 0 {
            return;
        }
        unsafe {
            sys::ImPlot_PlotStairsdoublePtrdoublePtr(
                im_str!("{}", self.label).as_ptr() as *const c_char,
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
    pub fn plot(&self, x: &[f64], y: &[f64]) {
        // If there is no data to plot, we stop here
        if x.len().min(y.len()) == 0 {
            return;
        }
        unsafe {
            sys::ImPlot_PlotScatterdoublePtrdoublePtr(
                im_str!("{}", self.label).as_ptr() as *const c_char,
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
    /// specify where on the corresponding axis (X for vertical mode, Y for horizontal mode) the
    /// bar is drawn, and the `bar_values` specify what values the bars have.
    pub fn plot(&self, axis_positions: &[f64], bar_values: &[f64]) {
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
                    as unsafe extern "C" fn(*const c_char, *const f64, *const f64, i32, f64, i32, i32);
                x = bar_values;
                y = axis_positions;
            } else {
                plot_function = sys::ImPlot_PlotBarsdoublePtrdoublePtr
                    as unsafe extern "C" fn(*const c_char, *const f64, *const f64, i32, f64, i32, i32);
                x = axis_positions;
                y = bar_values;
            };

            plot_function(
                im_str!("{}", self.label).as_ptr() as *const c_char,
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
        if self.label.is_empty() {
            return;
        }

        unsafe {
            sys::ImPlot_PlotText(
                im_str!("{}", self.label).as_ptr() as *const c_char,
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

/// Struct to provide functionality for creating headmaps.
pub struct PlotHeatmap {
    /// Label to show in plot
    label: String,

    /// Scale range of the values shown. If this is set to `None`, the scale
    /// is computed based on the values given to the `plot` function. If there
    /// is a value, the tuple is interpreted as `(minimum, maximum)`.
    scale_range: Option<(f64, f64)>,

    /// Label C style format string, this is shown when a a value point is hovered.
    /// None means don't show a label. The label is stored directly as an ImString because
    /// that is what's needed for the plot call anyway. Conversion is done in the setter.
    label_format: Option<ImString>,

    /// Lower left point for the bounding rectangle. This is called `bounds_min` in the C++ code.
    drawarea_lower_left: ImPlotPoint,

    /// Upper right point for the bounding rectangle. This is called `bounds_max` in the C++ code.
    drawarea_upper_right: ImPlotPoint,
}

impl PlotHeatmap {
    /// Create a new heatmap to be shown. Uses the same defaults as the C++ version (see code for
    /// what those are), aside from the `scale_min` and `scale_max` values, which default to
    /// `None`, which is interpreted as "automatically make the scale fit the data". Does not draw
    /// anything yet.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_owned(),
            scale_range: None,
            label_format: Some(im_str!("%.1f").to_owned()),
            drawarea_lower_left: ImPlotPoint { x: 0.0, y: 0.0 },
            drawarea_upper_right: ImPlotPoint { x: 1.0, y: 1.0 },
        }
    }

    /// Specify the scale for the shown colors by minimum and maximum value.
    pub fn with_scale(mut self, scale_min: f64, scale_max: f64) -> Self {
        self.scale_range = Some((scale_min, scale_max));
        self
    }

    /// Specify the label format for hovered data points.. `None` means no label is shown.
    pub fn with_label_format(mut self, label_format: Option<&str>) -> Self {
        self.label_format = label_format.map(|x| im_str!("{}", x));
        self
    }

    /// Specify the drawing area as the lower left and upper right point
    pub fn with_drawing_area(mut self, lower_left: ImPlotPoint, upper_right: ImPlotPoint) -> Self {
        self.drawarea_lower_left = lower_left;
        self.drawarea_upper_right = upper_right;
        self
    }

    /// Plot the heatmap, with the given values (assumed to be in row-major order),
    /// number of rows and number of columns.
    pub fn plot(&self, values: &[f64], number_of_rows: u32, number_of_cols: u32) {
        // If no range was given, determine that range
        let scale_range = self.scale_range.unwrap_or_else(|| {
            let mut min_seen = values[0];
            let mut max_seen = values[0];
            values.iter().for_each(|value| {
                min_seen = min_seen.min(*value);
                max_seen = max_seen.max(*value);
            });
            (min_seen, max_seen)
        });

        unsafe {
            sys::ImPlot_PlotHeatmapdoublePtr(
                im_str!("{}", self.label).as_ptr() as *const c_char,
                values.as_ptr(),
                number_of_rows as i32, // Not sure why C++ code uses a signed value here
                number_of_cols as i32, // Not sure why C++ code uses a signed value here
                scale_range.0,
                scale_range.1,
                // "no label" is taken as null pointer in the C++ code, but we're using
                // option types in the Rust bindings because they are more idiomatic.
                if self.label_format.is_some() {
                    self.label_format.as_ref().unwrap().as_ptr() as *const c_char
                } else {
                    std::ptr::null()
                },
                self.drawarea_lower_left,
                self.drawarea_upper_right,
            );
        }
    }
}

/// Struct to provide stem plotting functionality.
pub struct PlotStems {
    /// Label to show in the legend for this line
    label: String,

    /// Reference value for the y value, which the stems are "with respect to"
    reference_y: f64,
}

impl PlotStems {
    /// Create a new stem plot to be shown. Does not draw anything by itself, call
    /// [`PlotStems::plot`] on the struct for that.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_owned(),
            reference_y: 0.0, // Default value taken from C++ implot
        }
    }

    /// Set the reference y value for the stems
    pub fn with_reference_y(mut self, reference_y: f64) -> Self {
        self.reference_y = reference_y;
        self
    }

    /// Draw a previously-created stem plot. Use this in closures passed to
    /// [`Plot::build()`](struct.Plot.html#method.build). The `axis_positions` specify where on the
    /// X axis the stems are drawn, and the `stem_values` specify what values the stems have.
    pub fn plot(&self, axis_positions: &[f64], stem_values: &[f64]) {
        let number_of_points = axis_positions.len().min(stem_values.len());
        // If there is no data to plot, we stop here
        if number_of_points == 0 {
            return;
        }
        unsafe {
            sys::ImPlot_PlotStemsdoublePtrdoublePtr(
                im_str!("{}", self.label).as_ptr() as *const c_char,
                axis_positions.as_ptr(),
                stem_values.as_ptr(),
                number_of_points as i32, // "as" casts saturate as of Rust 1.45. This is safe here.
                self.reference_y,
                0,                                 // No offset
                std::mem::size_of::<f64>() as i32, // Stride, set to one f64 for the standard use case
            );
        }
    }
}
