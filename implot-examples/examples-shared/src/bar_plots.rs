//! This example demonstrates how bar plots are to be used. For more general
//! features of the libray, see the line_plots example.

use imgui::{im_str, CollapsingHeader, Ui};
use implot::{Plot, PlotBars, PlotUi};

pub fn show_basic_vertical_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!("This header shows a simple vertical bar plot."));
    let content_width = ui.window_content_region_width();
    Plot::new("Vertical bar plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(plot_ui, || {
            // If this is called outside a plot build callback, the program will panic.
            let axis_positions = vec![0.2, 0.4, 0.6, 0.8];
            let values = vec![0.1, 0.2, 0.3, 0.4];
            PlotBars::new("legend label")
                .with_bar_width(0.1)
                .plot(&axis_positions, &values);
        });
}

pub fn show_basic_horizontal_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!("This header shows a simple horizontal bar plot."));
    let content_width = ui.window_content_region_width();
    Plot::new("Horizontal bar plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(plot_ui, || {
            // If this is called outside a plot build callback, the program will panic.
            let axis_positions = vec![0.2, 0.4, 0.6, 0.8];
            let values = vec![0.1, 0.2, 0.3, 0.4];
            PlotBars::new("legend label")
                .with_bar_width(0.05)
                .with_horizontal_bars()
                .plot(&axis_positions, &values);
        });
}

pub fn show_demo_headers(ui: &Ui, plot_ui: &PlotUi) {
    if CollapsingHeader::new(im_str!("Bar plots: Basic vertical")).build(&ui) {
        show_basic_vertical_plot(&ui, &plot_ui);
    }
    if CollapsingHeader::new(im_str!("Bar plots: Basic horizontal")).build(&ui) {
        show_basic_horizontal_plot(&ui, &plot_ui);
    }
}
