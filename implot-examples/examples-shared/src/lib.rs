pub mod bar_plots;
pub mod line_plots;
pub mod scatter_plots;
pub mod text_plots;

use imgui::Ui;
use implot::PlotUi;

pub fn show_demos(ui: &Ui, plot_ui: &PlotUi) {
    bar_plots::show_demo_window(ui, plot_ui);
    line_plots::show_demo_window(ui, plot_ui);
    scatter_plots::show_demo_window(ui, plot_ui);
    text_plots::show_demo_window(ui, plot_ui);
}
