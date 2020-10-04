//! This example demonstrates how the text plotting features are to be used. For more general
//! features of the libray, see the line_plots example.

use imgui::{im_str, CollapsingHeader, Condition, Ui, Window};
use implot::{Context, Plot, PlotText};

mod support;

fn show_basic_plot(ui: &Ui) {
    ui.text(im_str!(
        "This header just plots some text with as little code as possible."
    ));
    let content_width = ui.window_content_region_width();
    Plot::new("Simple text plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(|| {
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

fn main() {
    let system = support::init(file!());
    let mut showing_demo = false;
    let _plotcontext = Context::create(); // TODO(4bb4) use this as soon as things have been adapted
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Text plots example"))
            .size([430.0, 450.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello from implot-rs!"));
                ui.text_wrapped(im_str!(
                    "The headers here demo the text plotting features of the library. \
                    Have a look at the example source code to see how they are implemented.\n\
                    Check out the demo from ImPlot itself first \
                    (by enabling the 'Show demo' checkbox) for instructions \
                    on how to interact with ImPlot plots."
                ));

                ui.checkbox(im_str!("Show demo"), &mut showing_demo);

                // Show individual examples in collapsed headers
                if CollapsingHeader::new(im_str!("Basic text plot")).build(&ui) {
                    show_basic_plot(&ui);
                }
            });

        if showing_demo {
            implot::show_demo_window(&mut showing_demo);
        }
    });
}
