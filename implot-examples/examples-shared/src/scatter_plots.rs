//! This example demonstrates how scatter plots are to be used. For more general
//! features of the libray, see the line_plots example.

use imgui::{im_str, CollapsingHeader, Ui};
use implot::{push_style_var_f32, push_style_var_i32, Marker, Plot, PlotScatter, PlotUi, StyleVar};

pub fn show_basic_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!(
        "This header just draws a scatter plot with as little code as possible."
    ));
    let content_width = ui.window_content_region_width();
    Plot::new("Simple scatter plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size([content_width, 300.0])
        .build(plot_ui, || {
            // If this is called outside a plot build callback, the program will panic.
            let x_positions = vec![0.1, 0.2, 0.1, 0.5, 0.9];
            let y_positions = vec![0.1, 0.1, 0.3, 0.3, 0.9];
            PlotScatter::new("legend label").plot(&x_positions, &y_positions);
        });
}

pub fn show_custom_markers_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!(
        "This header shows how markers can be used in scatter plots."
    ));
    let content_width = ui.window_content_region_width();
    Plot::new("Multi-marker scatter plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size([content_width, 300.0])
        .build(plot_ui, || {
            // Change to cross marker for one scatter plot call
            let x_positions = vec![0.1, 0.2, 0.1, 0.5, 0.9];
            let y_positions = vec![0.1, 0.1, 0.3, 0.3, 0.9];
            let markerchoice = push_style_var_i32(&StyleVar::Marker, Marker::Cross as i32);
            PlotScatter::new("legend label 1").plot(&x_positions, &y_positions);
            markerchoice.pop();

            // One can combine things like marker size and markor choice
            let x_positions = vec![0.4, 0.1];
            let y_positions = vec![0.5, 0.3];
            let marker_choice = push_style_var_i32(&StyleVar::Marker, Marker::Diamond as i32);
            let marker_size = push_style_var_f32(&StyleVar::MarkerSize, 12.0);
            PlotScatter::new("legend label 2").plot(&x_positions, &y_positions);

            // TODO(4bb4) check if these have to be in reverse push order. Does not
            // seem to be the case.
            marker_size.pop();
            marker_choice.pop();
        });
}

pub fn show_demo_headers(ui: &Ui, plot_ui: &PlotUi) {
    if CollapsingHeader::new(im_str!("Basic scatter plot")).build(&ui) {
        show_basic_plot(&ui, &plot_ui);
    }

    if CollapsingHeader::new(im_str!("Custom markers")).build(&ui) {
        show_custom_markers_plot(&ui, &plot_ui);
    }
}
