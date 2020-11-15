//! This example demonstrates how heatmaps are to be used. For more general
//! features of the libray, see the line_plots example.

use imgui::{im_str, CollapsingHeader, Condition, Ui, Window};
use implot::{Plot, PlotHeatmap, PlotUi};

pub fn show_basic_heatmap(ui: &Ui, plot_ui: &PlotUi) {
    ui.text(im_str!("This header shows a simple heatmap"));
    let content_width = ui.window_content_region_width();
    Plot::new("Heatmap plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(plot_ui, || {
            let values = (0..100).map(|x| 0.1 * x as f64).collect::<Vec<_>>();
            PlotHeatmap::new("my favourite heatmap")
                // If you omit the with_scale call, the range will be computed based on the values
                .with_scale(0.0, 10.0)
                .plot(&values, 10, 10);
        });
}

pub fn show_demo_window(ui: &Ui, plot_ui: &PlotUi) {
    Window::new(im_str!("Heatmaps example"))
        .size([430.0, 450.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text(im_str!("Hello from implot-rs!"));
            ui.text_wrapped(im_str!(
                "The headers here demo the heatmap plotting features of the library. \
                 Have a look at the example source code to see how they are implemented.\n\
                 Check out the demo from ImPlot itself first \
                 for instructions on how to interact with ImPlot plots."
            ));

            // Show individual examples in collapsed headers
            if CollapsingHeader::new(im_str!("Basic vertical plot")).build(&ui) {
                show_basic_heatmap(&ui, &plot_ui);
            }
        });
}
