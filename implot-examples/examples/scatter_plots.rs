//! This example demonstrates how scatter plots are to be used. For more general
//! features of the libray, see the line_plots example.

use imgui::{im_str, CollapsingHeader, Condition, Ui, Window};
use implot::{push_style_var_f32, push_style_var_u32, Marker, Plot, PlotScatter, StyleVar};

mod support;

fn show_basic_plot(ui: &Ui) {
    ui.text(im_str!(
        "This header just draws a scatter plot with as little code as possible."
    ));
    let content_width = ui.window_content_region_width();
    Plot::new("Simple scatter plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(|| {
            // If this is called outside a plot build callback, the program will panic.
            let x_positions = vec![0.1, 0.2, 0.1, 0.5, 0.9];
            let y_positions = vec![0.1, 0.1, 0.3, 0.3, 0.9];
            PlotScatter::new("legend label").plot(&x_positions, &y_positions);
        });
}

fn show_custom_markers_plot(ui: &Ui) {
    ui.text(im_str!(
        "This header just plots a line with as little code as possible."
    ));
    let content_width = ui.window_content_region_width();
    Plot::new("Simple scatter plot")
        // The size call could also be omitted, though the defaults don't consider window
        // width, which is why we're not doing so here.
        .size(content_width, 300.0)
        .build(|| {
            // Change to cross marker for one scatter plot call
            let x_positions = vec![0.1, 0.2, 0.1, 0.5, 0.9];
            let y_positions = vec![0.1, 0.1, 0.3, 0.3, 0.9];
            let markerchoice = push_style_var_u32(&StyleVar::Marker, Marker::CROSS.bits());
            PlotScatter::new("legend label 1").plot(&x_positions, &y_positions);
            markerchoice.pop();

            // One can combine things like marker size and markor choice
            let x_positions = vec![0.4, 0.1];
            let y_positions = vec![0.5, 0.3];
            let marker_choice = push_style_var_u32(&StyleVar::Marker, Marker::DIAMOND.bits());
            let marker_size = push_style_var_f32(&StyleVar::MarkerSize, 12.0);
            PlotScatter::new("legend label 2").plot(&x_positions, &y_positions);

            // TODO(4bb4) check if these have to be in reverse push order
            marker_size.pop();
            marker_choice.pop();
        });
}

fn main() {
    let system = support::init(file!());
    let mut showing_demo = false;
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Line plots example"))
            .size([430.0, 450.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello from implot-rs!"));
                ui.text_wrapped(im_str!(
                    "The headers here demo the scatter plotting features of the library. \
                    Have a look at the example source code to see how they are implemented.\n\
                    Check out the demo from ImPlot itself first \
                    (by enabling the 'Show demo' checkbox) for instructions \
                    on how to interact with ImPlot plots."
                ));

                ui.checkbox(im_str!("Show demo"), &mut showing_demo);

                // Show individual examples in collapsed headers
                if CollapsingHeader::new(im_str!("Basic scatter plot")).build(&ui) {
                    show_basic_plot(&ui);
                }

                if CollapsingHeader::new(im_str!("Custom markers")).build(&ui) {
                    show_custom_markers_plot(&ui);
                }
            });

        if showing_demo {
            implot::show_demo_window(&mut showing_demo);
        }
    });
}
