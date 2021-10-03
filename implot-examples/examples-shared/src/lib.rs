pub mod bar_plots;
pub mod heatmaps;
pub mod line_plots;
pub mod scatter_plots;
pub mod stairs_plots;
mod stem_plots;
pub mod text_plots;

use imgui::{Condition, Ui, Window};
use implot::PlotUi;

/// State of the demo code
pub struct DemoState {
    /// State of the line plots demo
    line_plots: line_plots::LinePlotDemoState,
}

impl DemoState {
    /// Create a new demo code state object with default values in it.
    pub fn new() -> Self {
        Self {
            line_plots: line_plots::LinePlotDemoState::new(),
        }
    }

    /// Show all the demos
    pub fn show_demos(&mut self, ui: &Ui, plot_ui: &PlotUi) {
        // Most of the demos are currently still stateless, so the code here mostly just calls into
        // the modules. The line plots demo is stateful though. Things will be refactored soon to
        // make all the individual demos stateful to unify things more.
        Window::new("implot-rs demo")
            .size([430.0, 450.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text("Hello from implot-rs!");
                ui.text_wrapped(
                    "The headers here demo the plotting features of the library.\
                 Have a look at the example source code to see how they are implemented.\n\
                 Check out the demo from ImPlot itself first for instructions on how to\
                 interact with ImPlot plots.",
                );

                ui.separator();
                ui.text("Bar plots:");
                bar_plots::show_demo_headers(ui, plot_ui);

                ui.separator();
                ui.text("Line plots:");
                // The line plots demo is stateful
                self.line_plots.show_demo_headers(ui, plot_ui);

                ui.separator();
                ui.text("Scatter plots:");
                scatter_plots::show_demo_headers(ui, plot_ui);

                ui.separator();
                ui.text("Text plots:");
                text_plots::show_demo_headers(ui, plot_ui);

                ui.separator();
                ui.text("Stairs plots:");
                stairs_plots::show_demo_headers(ui, plot_ui);

                ui.separator();
                ui.text("Heatmaps:");
                heatmaps::show_demo_headers(ui, plot_ui);

                ui.separator();
                ui.text("Stem plots:");
                stem_plots::show_demo_headers(ui, plot_ui);
            });
    }
}

impl Default for DemoState {
    fn default() -> Self {
        Self::new()
    }
}
