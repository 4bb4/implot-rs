//! This example demonstrates how stairs plots are to be used. They are almost the same as line
//! plots, so head over to the line plots example for more info.
//!
use imgui::{CollapsingHeader, Ui};
use implot::{Plot, PlotStairs, PlotUi};

pub fn show_basic_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text_wrapped("This header just plots a stairs-style line with as little code as possible.");
    let content_width = ui.window_content_region_width();
    Plot::new("Simple stairs plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size([content_width, 300.0])
        .build(plot_ui, || {
            // If this is called outside a plot build callback, the program will panic.
            let x_positions = vec![0.1, 0.2, 0.5];
            let y_positions = vec![0.1, 0.3, 0.9];
            PlotStairs::new("legend label").plot(&x_positions, &y_positions);
        });
}

pub fn show_demo_headers(ui: &Ui, plot_ui: &PlotUi) {
    if CollapsingHeader::new("Stairs plot: Basic").build(ui) {
        show_basic_plot(ui, plot_ui);
    }
}
