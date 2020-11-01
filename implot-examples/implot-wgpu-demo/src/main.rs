use imgui::{im_str, Condition, Window};
use imgui_wgpu::simple_api;
use implot::Context;

struct State {
    height: f32,
    width: f32,
    highdpi_factor: f64,
    plotcontext: implot::Context,

    showing_demo: bool,
    showing_rust_demo: bool,
}

fn main() {
    let config = simple_api::Config {
        on_resize: &|input, state: &mut State, hdpi| {
            state.height = input.height as f32;
            state.width = input.width as f32;
            state.highdpi_factor = hdpi;
        },
        ..Default::default()
    };

    let plotcontext = Context::create();

    let state = State {
        height: 100.0,
        width: 100.0,
        highdpi_factor: 2.0,
        plotcontext,

        showing_demo: false,
        showing_rust_demo: true,
    };

    imgui_wgpu::simple_api::run(config, state, |ui, state| {
        let plot_ui = state.plotcontext.get_plot_ui();

        if state.showing_demo {
            implot::show_demo_window(&mut state.showing_demo);
        }

        if state.showing_rust_demo {
            examples_shared::show_demos(ui, &plot_ui);
        }

        Window::new(im_str!("Welcome to the ImPlot-rs demo!"))
            .size([430.0, 450.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.checkbox(
                    im_str!("Show C++ ImPlot demo window"),
                    &mut state.showing_demo,
                );
                ui.checkbox(
                    im_str!("Show Rust ImPlot demo windows"),
                    &mut state.showing_rust_demo,
                );
                // TODO(4bb4) ... move windows by default so this is less confusing
                ui.text_wrapped(im_str!(
                    "Note that the windows are stacked, so move this one out of the way to see\
                     the ones beneath it."
                ));
            });
    });
}
