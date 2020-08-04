//! # Rust bindings to ImPlot
//!
//! This crate contains idiomatic bindings to the C++ implot library, which
//! use the bindings exposed by the `implot-sys` crate.
pub extern crate implot_sys as sys;
use sys::imgui::im_str;

/// Struct to represent an ImPlot.
pub struct Plot {
    /// Title of the plot, shown on top.
    title: String,
    /// Label of the x axis, shown on the bottom
    x_label: String,
    /// Label of the y axis, shown on the left
    y_label: String,
    /// Size of the plot in x direction, in pixels.
    size_x: u32,
    /// Size of the plot in y direction, in pixels.
    size_y: u32,
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
    pub fn new() -> Self {
        // TODO(4bb4) question these defaults, maybe remove some of them
        Self {
            title: "".to_owned(),
            x_label: "".to_owned(),
            y_label: "".to_owned(),
            size_x: 400,
            size_y: 400,
            plot_flags: 0xFF,
            x_flags: 7,
            y_flags: 7,
            x2_flags: 0,
            y2_flags: 0,
        }
    }

    /// Attempt to show the plot. Only do things with it and call `end()` after that
    /// if this returns `true`.
    pub fn begin(&self) -> bool {
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

    /// End a previously-`begin()`ed plot. Only call if `begin()` returned `true`.
    pub fn end(&self) {
        unsafe { sys::ImPlot_EndPlot() }
    }
}

// TODO(4bb4) convert to struct and add methods to set title and flags
/// Plot a line. Use this between Plot::begin() and Plot::end().
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
