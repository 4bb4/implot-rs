use imgui::*;
use implot;

mod support;

fn main() {
    let system = support::init(file!());
    system.main_loop(move |_, ui| {
        Window::new(im_str!("Hello world"))
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));

                // Demo some implot stuff :D
                let x_values = vec![1.0, 2.0, 4.0, 5.0];
                let y_values = vec![1.0, 0.0, 0.0, 1.0];
                let plot = implot::Plot::new();
                if plot.begin() {
                    implot::plot_line(&vec![2.0, 2.0], &vec![2.0, 1.0], "Left eye");
                    implot::plot_line(&vec![4.0, 4.0], &vec![2.0, 1.0], "Right eye");
                    implot::plot_line(&x_values, &y_values, "Mouth");
                    plot.end();
                }
            });
    });
}
