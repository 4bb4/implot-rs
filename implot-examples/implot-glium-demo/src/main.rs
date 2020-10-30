use examples_shared;
use imgui::{im_str, Condition, Window};
use implot::Context;

// the actual implot samples are in there TODO(4bb4) move to using examples-shared instead
mod support;

fn main() {
    let system = support::init(file!());
    let mut showing_demo = false;
    let mut showing_rust_demo = true;
    let plotcontext = Context::create();
    system.main_loop(move |_, ui| {
        // The context is moved into the closure after creation so plot_ui is valid.
        let plot_ui = plotcontext.get_plot_ui();

        if showing_demo {
            implot::show_demo_window(&mut showing_demo);
        }

        if showing_rust_demo {
            examples_shared::show_demos(ui, &plot_ui);
        }

        Window::new(im_str!("Welcome to the ImPlot-rs demo!"))
            .size([430.0, 450.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.checkbox(im_str!("Show C++ ImPlot demo window"), &mut showing_demo);
                ui.checkbox(
                    im_str!("Show Rust ImPlot demo windows"),
                    &mut showing_rust_demo,
                );
                // TODO(4bb4) ... move windows by default so this is less confusing
                ui.text_wrapped(im_str!(
                    "Note that the windows are stacked, so move this one out of the way to see\
                     the ones beneath it."
                ));
            });
    });
}
