use imgui::*;
use implot::{
    get_plot_limits, get_plot_mouse_position, is_plot_hovered, pop_style_color, push_style_color,
};
use implot::{
    AxisFlags, ImPlotLimits, ImPlotPoint, ImPlotRange, Plot, PlotColorElement, PlotFlags, PlotLine,
    PlotText,
};

mod support;

fn main() {
    let system = support::init(file!());
    let mut showing_demo = false;
    system.main_loop(move |_, ui| {
        // Create the window from time imgui example, just... with an added plot
        Window::new(im_str!("Hello world"))
            .size([430.0, 450.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello from implot-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
                ui.checkbox(im_str!("Show demo"), &mut showing_demo);

                // Create some containers for exfiltrating data from the closure below
                let mut hover_pos: Option<ImPlotPoint> = None;
                let mut plot_limits: Option<ImPlotLimits> = None;

                // Draw a plot
                push_style_color(&PlotColorElement::PLOT_BG, 1.0, 1.0, 1.0, 0.2);
                Plot::new("Demo plot")
                    .size(400.0, 300.0)
                    .x_label("awesome x label")
                    .y_label("awesome y label")
                    .x_limits(&ImPlotRange { Min: 0.0, Max: 6.0 }, Condition::FirstUseEver)
                    .y_limits(
                        &ImPlotRange {
                            Min: -1.0,
                            Max: 3.0,
                        },
                        Condition::FirstUseEver,
                    )
                    .with_plot_flags(&(PlotFlags::DEFAULT))
                    .with_y_axis_flags(&(AxisFlags::DEFAULT | AxisFlags::INVERT))
                    .build(|| {
                        // Line plotting
                        PlotLine::new("Left eye").plot(&vec![2.0, 2.0], &vec![2.0, 1.0]);
                        PlotLine::new("Right eye").plot(&vec![4.0, 4.0], &vec![2.0, 1.0]);
                        let x_values = vec![1.0, 2.0, 4.0, 5.0];
                        let y_values = vec![1.0, 0.0, 0.0, 1.0];
                        PlotLine::new("Mouth").plot(&x_values, &y_values);

                        // Text
                        PlotText::new("Text!").plot(2.0, 2.0, false);
                        PlotText::new("Text with offset!")
                            .with_pixel_offset(10.0, 30.0)
                            .plot(2.0, 2.0, false);
                        PlotText::new("Vertical Text!").plot(0.1, 2.5, true);
                        if is_plot_hovered() {
                            hover_pos = Some(get_plot_mouse_position());
                        }
                        plot_limits = Some(get_plot_limits());
                    });

                // Print some previously-exfiltrated info. This is because calling
                // things like is_plot_hovered or get_plot_mouse_position() outside
                // of an actual Plot is not allowed.
                if let Some(pos) = hover_pos {
                    ui.text(im_str!("hovered at {}, {}", pos.x, pos.y));
                }
                if let Some(limits) = plot_limits {
                    ui.text(im_str!(
                        "X limits are {:+10.3}, {:+10.3}",
                        limits.X.Min,
                        limits.X.Max
                    ));
                    ui.text(im_str!(
                        "Y limits are {:+10.3}, {:+10.3}",
                        limits.Y.Min,
                        limits.Y.Max
                    ));
                }
                pop_style_color(1);
            });

        if showing_demo {
            implot::show_demo_window(&mut showing_demo);
        }
    });
}
