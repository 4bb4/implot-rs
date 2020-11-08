//! This example demonstrates how stairs plots are to be used. They are almost the same as line
//! plots, so head over to the line plots example for more info.
//!
use imgui::{im_str, CollapsingHeader, Condition, Ui, Window};
use implot::{Plot, PlotStairs, PlotUi};

pub fn show_basic_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text_wrapped(im_str!(
        "This header just plots a stairs-style line with as little code as possible."
    ));
    let content_width = ui.window_content_region_width();
    Plot::new("Simple stairs plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(plot_ui, || {
            // If this is called outside a plot build callback, the program will panic.
            let x_positions = vec![0.1, 0.2, 0.5];
            let y_positions = vec![0.1, 0.3, 0.9];
            PlotStairs::new("legend label").plot(&x_positions, &y_positions);
        });
}

pub fn show_demo_window(ui: &Ui, plot_ui: &PlotUi) {
    Window::new(im_str!("Stairs plots example"))
        .size([430.0, 450.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text(im_str!("Hello from implot-rs!"));
            ui.text_wrapped(im_str!(
                "The headers here demo the stairs plotting features of the library. \
                 Have a look at the example source code to see how they are implemented.\n\
                 Check out the demo from ImPlot itself first \
                 for instructions on how to interact with ImPlot plots."
            ));

            if CollapsingHeader::new(im_str!("Basic stairs plot")).build(&ui) {
                show_basic_plot(&ui, &plot_ui);
            }
        });
}
