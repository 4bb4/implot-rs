//! This example demonstrates how the text plotting features are to be used. For more general
//! features of the libray, see the line_plots example.

use imgui::{im_str, CollapsingHeader, Ui};
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

pub fn show_demo_headers(ui: &Ui, plot_ui: &PlotUi) {
    if CollapsingHeader::new(im_str!("Text plot: Basic")).build(&ui) {
        show_basic_plot(&ui, &plot_ui);
    }
}
