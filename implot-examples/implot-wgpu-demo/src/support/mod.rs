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
    pub surface_conf: wgpu::SurfaceConfiguration,
    pub window: Window,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
}

pub fn init(title: &str) -> System {
    // Set up window and GPU
    let event_loop = EventLoop::new();
    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

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

    let (device, queue) =
        block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

    // Set up swap chain
    let surface_conf = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    surface.configure(&device, &surface_conf);

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
        texture_format: surface_conf.format,
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
        surface_conf,
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
            mut surface_conf,
            mut platform,
            window,
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
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    let size = window.inner_size();

                    // Recreate the swap chain with the new size
                    surface_conf.width = size.width as u32;
                    surface_conf.height = size.height as u32;
                    surface.configure(&device, &surface_conf);
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

                    let frame = match surface.get_current_frame() {
                        Ok(frame) => frame.output,
                        Err(e) => {
                            eprintln!("dropped frame: {:?}", e);
                            return;
                        }
                    };
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

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
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &view,
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

                    drop(rpass);

                    queue.submit(Some(encoder.finish()));
                }
                _ => (),
            }

            platform.handle_event(imgui.io_mut(), &window, &event);
        })
    }
}
