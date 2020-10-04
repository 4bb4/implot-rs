//! This example demonstrates how bar plots are to be used. For more general
//! features of the libray, see the line_plots example.

use imgui::{im_str, CollapsingHeader, Condition, Ui, Window};
use implot::{Context, Plot, PlotBars};

mod support;

fn show_basic_vertical_plot(ui: &Ui) {
    ui.text(im_str!("This header shows a simple vertical bar plot."));
    let content_width = ui.window_content_region_width();
    Plot::new("Vertical bar plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(|| {
            // If this is called outside a plot build callback, the program will panic.
            let axis_positions = vec![0.2, 0.4, 0.6, 0.8];
            let values = vec![0.1, 0.2, 0.3, 0.4];
            PlotBars::new("legend label")
                .with_bar_width(0.1)
                .plot(&axis_positions, &values);
        });
}

fn show_basic_horizontal_plot(ui: &Ui) {
    ui.text(im_str!("This header shows a simple horizontal bar plot."));
    let content_width = ui.window_content_region_width();
    Plot::new("Horizontal bar plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(|| {
            // If this is called outside a plot build callback, the program will panic.
            let axis_positions = vec![0.2, 0.4, 0.6, 0.8];
            let values = vec![0.1, 0.2, 0.3, 0.4];
            PlotBars::new("legend label")
                .with_bar_width(0.05)
                .with_horizontal_bars()
                .plot(&axis_positions, &values);
        });
}

fn main() {
    let system = support::init(file!());
    let mut showing_demo = false;
    let _plotcontext = Context::create(); // TODO(4bb4) use this as soon as things have been adapted
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Bar plots example"))
            .size([430.0, 450.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello from implot-rs!"));
                ui.text_wrapped(im_str!(
                    "The headers here demo the bar plotting features of the library. \
                    Have a look at the example source code to see how they are implemented.\n\
                    Check out the demo from ImPlot itself first \
                    (by enabling the 'Show demo' checkbox) for instructions \
                    on how to interact with ImPlot plots."
                ));

                ui.checkbox(im_str!("Show demo"), &mut showing_demo);

                // Show individual examples in collapsed headers
                if CollapsingHeader::new(im_str!("Basic vertical plot")).build(&ui) {
                    show_basic_vertical_plot(&ui);
                }

                if CollapsingHeader::new(im_str!("Basic horizontal plot")).build(&ui) {
                    show_basic_horizontal_plot(&ui);
                }
            });

        if showing_demo {
            implot::show_demo_window(&mut showing_demo);
        }
    });
}
