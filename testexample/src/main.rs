use imgui::*;
use implot_sys;

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

                // TODO(4bb4) Replace this with safe bindings once those are written

                let x_values: [f64; 4] = [1.0, 2.0, 4.0, 5.0];
                let y_values: [f64; 4] = [1.0, 0.0, 0.0, 1.0];
                unsafe {
                    if implot_sys::ImPlot_BeginPlot(
                        im_str!("My Plot").as_ptr() as *const i8,
                        im_str!("x").as_ptr() as *const i8,
                        im_str!("y").as_ptr() as *const i8,
                        implot_sys::ImVec2 { x: 600.0, y: 600.0 },
                        0xFF,
                        7,
                        7,
                        0,
                        0,
                    ) {
                        implot_sys::ImPlot_PlotLinedoublePtrdoublePtr(
                            im_str!("Mouth").as_ptr() as *const i8,
                            x_values.as_ptr(),
                            y_values.as_ptr(),
                            x_values.len() as i32,
                            0,
                            8,
                        );
                        implot_sys::ImPlot_PlotLinedoublePtrdoublePtr(
                            im_str!("Left eye").as_ptr() as *const i8,
                            [2.0, 2.0].as_ptr(),
                            [2.0, 1.0].as_ptr(),
                            2i32,
                            0,
                            8,
                        );
                        implot_sys::ImPlot_PlotLinedoublePtrdoublePtr(
                            im_str!("Right eye").as_ptr() as *const i8,
                            [4.0, 4.0].as_ptr(),
                            [2.0, 1.0].as_ptr(),
                            2i32,
                            0,
                            8,
                        );
                        implot_sys::ImPlot_EndPlot();
                    }
                }
            });
    });
}
