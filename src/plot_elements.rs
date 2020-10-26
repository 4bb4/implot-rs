//! # Plot elements module
//!
//! This module defines the various structs that can be used for drawing different things such
//! as lines, bars, scatter plots and text in a plot. For the module to create plots themselves,
//! see `plot`.
use crate::sys;
use imgui::im_str;

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
