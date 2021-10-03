//! This example demonstrates how heatmaps are to be used. For more general
//! features of the libray, see the line_plots example.

use imgui::{CollapsingHeader, Ui};
use implot::{ImPlotPoint, Plot, PlotHeatmap, PlotUi};

pub fn show_basic_heatmap(ui: &Ui, plot_ui: &PlotUi) {
    ui.text("This header shows a simple heatmap");
    let content_width = ui.window_content_region_width();
    Plot::new("Heatmap plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size([content_width, 300.0])
        .build(plot_ui, || {
            let values = (0..100).map(|x| 0.1 * x as f64).collect::<Vec<_>>();
            PlotHeatmap::new("my favourite heatmap")
                // If you omit the with_scale call, the range will be computed based on the values
                .with_scale(0.0, 10.0)
                .with_drawing_area(
                    ImPlotPoint { x: -1.0, y: -1.0 },
                    ImPlotPoint { x: 1.0, y: 1.0 },
                )
                .plot(&values, 10, 10);
        });
}

pub fn show_demo_headers(ui: &Ui, plot_ui: &PlotUi) {
    if CollapsingHeader::new("Heatmap: Basic").build(ui) {
        show_basic_heatmap(ui, plot_ui);
    }
}
