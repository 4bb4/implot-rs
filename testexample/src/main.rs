use imgui::*;
use implot;

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

                // Draw a plot
                implot::Plot::new("Demo plot")
                    .size(400.0, 300.0)
                    .x_label("awesome x label")
                    .y_label("awesome y label")
                    .build(|| {
                        implot::plot_line(&vec![2.0, 2.0], &vec![2.0, 1.0], "Left eye");
                        implot::plot_line(&vec![4.0, 4.0], &vec![2.0, 1.0], "Right eye");

                        let x_values = vec![1.0, 2.0, 4.0, 5.0];
                        let y_values = vec![1.0, 0.0, 0.0, 1.0];
                        implot::plot_line(&x_values, &y_values, "Mouth");
                    });
            });

        if showing_demo {
            implot::show_demo_window(&mut showing_demo);
        }
    });
}
