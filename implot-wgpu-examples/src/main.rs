use imgui::{im_str, CollapsingHeader, Condition, FontSource};
use imgui_wgpu::RendererConfig;
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

// the actual implot samples are in there
mod ui;

async fn run(event_loop: EventLoop<()>, window: Window, swapchain_format: wgpu::TextureFormat) {
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropiate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                shader_validation: true,
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    // Set up dear imgui
    let mut imgui = imgui::Context::create();
    let implot = implot::Context::create();

    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let hidpi_factor = window.scale_factor();

    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    let style = imgui.style_mut();
    style.use_classic_colors();

    let mut renderer = RendererConfig::new()
        .set_texture_format(sc_desc.format)
        .build(&mut imgui, &device, &queue);

    let mut last_frame = Instant::now();
    let mut last_cursor = None;

    let mut showing_demo = false;
    let mut make_fullscreen = false;

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&instance, &adapter, &pipeline_layout);

        let plot_ui = implot.get_plot_ui();

        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the swap chain with the new size
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawEventsCleared => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;

                let frame = match swap_chain.get_current_frame() {
                    Ok(frame) => frame,
                    Err(e) => {
                        eprintln!("dropped frame: {:?}", e);
                        return;
                    }
                };

                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let ui = imgui.frame();

                {
                    let window = imgui::Window::new(im_str!("Hello implot"));
                    let window = if make_fullscreen {
                        let border = 10.0;
                        window.position([0.0, 0.0], Condition::Always).size(
                            [
                                sc_desc.width as f32 / hidpi_factor as f32 - border,
                                sc_desc.height as f32 / hidpi_factor as f32 - border,
                            ],
                            Condition::Always,
                        )
                    } else {
                        window.size([400.0, 300.0], Condition::FirstUseEver)
                    };

                    window.build(&ui, || {
                        ui.text(im_str!("Hello from implot-rs!"));
                        ui.text_wrapped(im_str!(
                            "The headers here demo the line plotting features of the library. \
                    Have a look at the example source code to see how they are implemented.\n\
                    Check out the demo from ImPlot itself first \
                    (by enabling the 'Show demo' checkbox) for instructions \
                    on how to interact with ImPlot plots."
                        ));

                        ui.checkbox(im_str!("Show demo"), &mut showing_demo);
                        ui.checkbox(
                            im_str!("make the implot window fill the whole outer window"),
                            &mut make_fullscreen,
                        );

                        // Show individual examples in collapsed headers
                        if CollapsingHeader::new(im_str!("Basic lineplot")).build(&ui) {
                            ui::show_basic_plot(&ui, &plot_ui);
                        }
                        if CollapsingHeader::new(im_str!("Configurable lineplot")).build(&ui) {
                            ui::show_configurable_plot(&ui, &plot_ui);
                        }
                        if CollapsingHeader::new(im_str!("Querying a plot")).build(&ui) {
                            ui::show_query_features_plot(&ui, &plot_ui);
                        }
                        if CollapsingHeader::new(im_str!("Styling a plot")).build(&ui) {
                            ui::show_style_plot(&ui, &plot_ui);
                        }
                        if CollapsingHeader::new(im_str!("Colormap selection")).build(&ui) {
                            ui::show_colormaps_plot(&ui, &plot_ui);
                        }
                    });
                }

                if showing_demo {
                    implot::show_demo_window(&mut showing_demo);
                }

                let mut encoder: wgpu::CommandEncoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                if last_cursor != Some(ui.mouse_cursor()) {
                    last_cursor = Some(ui.mouse_cursor());
                    platform.prepare_render(&ui, &window);
                }

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.output.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                renderer
                    .render(ui.render(), &queue, &device, &mut rpass)
                    .expect("Rendering failed");

                drop(rpass); // renders to screen on drop, will probaly be changed in wgpu 0.7 or later

                queue.submit(Some(encoder.finish()));
            }
            _ => {}
        }

        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    // Temporarily avoid srgb formats for the swapchain on the web
    futures::executor::block_on(run(event_loop, window, wgpu::TextureFormat::Bgra8UnormSrgb));
}
