use futures::executor::block_on;
use imgui::{Context, FontSource, Ui};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::WinitPlatform;
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub struct System {
    pub event_loop: EventLoop<()>,
    pub imgui: Context,
    pub renderer: Renderer,
    pub platform: WinitPlatform,
    pub font_size: f32,
    pub hidpi_factor: f64,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub window: Window,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
}

pub fn init(title: &str) -> System {
    // Set up window and GPU
    let event_loop = EventLoop::new();
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

    let (window, size, surface) = {
        let window = Window::new(&event_loop).unwrap();
        window.set_inner_size(LogicalSize {
            width: 1280.0,
            height: 720.0,
        });
        window.set_title(title);
        let size = window.inner_size();

        let surface = unsafe { instance.create_surface(&window) };

        (window, size, surface)
    };

    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
    }))
    .unwrap();

    let (device, queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            shader_validation: false,
        },
        None,
    ))
    .unwrap();

    // Set up swap chain
    let sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let swap_chain = device.create_swap_chain(&surface, &sc_desc);

    // Set up dear imgui
    let mut imgui = imgui::Context::create();

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

    //
    // Set up dear imgui wgpu renderer
    //
    let renderer_config = RendererConfig {
        texture_format: sc_desc.format,
        ..Default::default()
    };

    let renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

    System {
        event_loop,
        imgui,
        renderer,
        platform,
        font_size,
        hidpi_factor,
        sc_desc,
        swap_chain,
        window,
        device,
        queue,
        surface,
    }
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) {
        let System {
            event_loop,
            mut imgui,
            mut renderer,
            // Currently not used, but was used pre-refactor
            // mut hidpi_factor,
            mut sc_desc,
            mut platform,
            window,
            mut swap_chain,
            device,
            queue,
            surface,
            ..
        } = self;
        let mut last_frame = Instant::now();
        let mut last_cursor = None;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::ScaleFactorChanged { scale_factor, .. },
                    ..
                } => {
                    // This
                    let _hidpi_factor = scale_factor;
                }
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
                    let mut ui = imgui.frame();

                    // --- Actual drawing code ----------------------------------------------
                    let mut run = true;
                    run_ui(&mut run, &mut ui);
                    if !run {
                        *control_flow = ControlFlow::Exit;
                    }

                    // --- Post-drawing rendering code --------------------------------------
                    let mut encoder: wgpu::CommandEncoder = device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

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
                                    // TODO(4bb4) remove hardcoded values here
                                    r: 0.1,
                                    g: 0.4,
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
                _ => (),
            }

            platform.handle_event(imgui.io_mut(), &window, &event);
        })
    }
}
