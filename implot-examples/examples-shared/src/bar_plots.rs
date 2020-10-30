//! This example demonstrates how bar plots are to be used. For more general
//! features of the libray, see the line_plots example.

use imgui::{im_str, CollapsingHeader, Condition, Ui, Window};
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

pub fn show_demo_window(ui: &Ui, plot_ui: &PlotUi) {
    Window::new(im_str!("Bar plots example"))
        .size([430.0, 450.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text(im_str!("Hello from implot-rs!"));
            ui.text_wrapped(im_str!(
                "The headers here demo the bar plotting features of the library. \
                    Have a look at the example source code to see how they are implemented.\n\
                    Check out the demo from ImPlot itself first \
                    for instructions on how to interact with ImPlot plots."
            ));

            // Show individual examples in collapsed headers
            if CollapsingHeader::new(im_str!("Basic vertical plot")).build(&ui) {
                show_basic_vertical_plot(&ui, &plot_ui);
            }

            if CollapsingHeader::new(im_str!("Basic horizontal plot")).build(&ui) {
                show_basic_horizontal_plot(&ui, &plot_ui);
            }
        });
}
