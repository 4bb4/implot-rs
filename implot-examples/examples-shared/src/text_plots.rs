//! This example demonstrates how the text plotting features are to be used. For more general
//! features of the libray, see the line_plots example.

use imgui::{im_str, CollapsingHeader, Condition, Ui, Window};
use implot::{Plot, PlotText, PlotUi};

pub fn show_basic_plot(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!(
        "This header just plots some text with as little code as possible."
    ));
    let content_width = ui.window_content_region_width();
    Plot::new("Simple text plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(plot_ui, || {
            // The text passed to "new" is what gets displayed.
            let x_position: f64 = 0.5;
            let y_position: f64 = 0.2;
            let vertical: bool = false;
            PlotText::new("horizontal displayed text").plot(x_position, y_position, vertical);

            // The text passed to "new" is what gets displayed.
            let x_position: f64 = 0.2;
            let y_position: f64 = 0.2;
            let vertical: bool = true;
            PlotText::new("vertical displayed text").plot(x_position, y_position, vertical);
        });
}

pub fn show_demo_window(ui: &Ui, plot_ui: &PlotUi) {
    Window::new(im_str!("Text plots example"))
        .size([430.0, 450.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text(im_str!("Hello from implot-rs!"));
            ui.text_wrapped(im_str!(
                "The headers here demo the text plotting features of the library. \
                    Have a look at the example source code to see how they are implemented.\n\
                    Check out the demo from ImPlot itself first \
                    for instructions on how to interact with ImPlot plots."
            ));

            // Show individual examples in collapsed headers
            if CollapsingHeader::new(im_str!("Basic text plot")).build(&ui) {
                show_basic_plot(&ui, &plot_ui);
            }
        });
}
