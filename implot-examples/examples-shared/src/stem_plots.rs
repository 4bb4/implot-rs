//! This example demonstrates how stem plots are to be used. For more general
//! features of the libray, see the line_plots example.

use imgui::{CollapsingHeader, Ui};
use implot::{Plot, PlotStems, PlotUi};

pub fn show_basic_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text("This header shows a simple stem plot.");
    let content_width = ui.window_content_region_width();
    Plot::new("Stem plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size([content_width, 300.0])
        .build(plot_ui, || {
            // If this is called outside a plot build callback, the program will panic.
            let axis_positions = vec![0.2, 0.4, 0.6, 0.8, 0.9, 0.93];
            let values = vec![0.1, 0.2, 0.3, 0.4, 0.3, 0.8];
            PlotStems::new("legend label")
                .with_reference_y(0.1)
                .plot(&axis_positions, &values);
        });
}

pub fn show_demo_headers(ui: &Ui, plot_ui: &PlotUi) {
    if CollapsingHeader::new("Stem plots").build(ui) {
        show_basic_plot(ui, plot_ui);
    }
}
