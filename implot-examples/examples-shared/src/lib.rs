pub mod bar_plots;
pub mod heatmaps;
pub mod line_plots;
pub mod scatter_plots;
pub mod stairs_plots;
mod stem_plots;
pub mod text_plots;

use imgui::{im_str, Condition, Ui, Window};
use implot::PlotUi;

pub fn show_demos(ui: &Ui, plot_ui: &PlotUi) {
    Window::new(im_str!("implot-rs demo"))
        .size([430.0, 450.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text(im_str!("Hello from implot-rs!"));
            ui.text_wrapped(im_str!(
                "The headers here demo the plotting features of the library.\
                 Have a look at the example source code to see how they are implemented.\n\
                 Check out the demo from ImPlot itself first for instructions on how to\
                 interact with ImPlot plots."
            ));

            ui.separator();
            ui.text(im_str!("Bar plots:"));
            bar_plots::show_demo_headers(ui, plot_ui);

            ui.separator();
            ui.text(im_str!("Line plots:"));
            line_plots::show_demo_headers(ui, plot_ui);

            ui.separator();
            ui.text(im_str!("Scatter plots:"));
            scatter_plots::show_demo_headers(ui, plot_ui);

            ui.separator();
            ui.text(im_str!("Text plots:"));
            text_plots::show_demo_headers(ui, plot_ui);

            ui.separator();
            ui.text(im_str!("Stairs plots:"));
            stairs_plots::show_demo_headers(ui, plot_ui);

            ui.separator();
            ui.text(im_str!("Heatmaps:"));
            heatmaps::show_demo_headers(ui, plot_ui);

            ui.separator();
            ui.text(im_str!("Stem plots:"));
            stem_plots::show_demo_headers(ui, plot_ui);
        });
}
