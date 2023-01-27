use std::borrow::Cow;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use wpp::{Effect, Frame};

/// The [WGSL](https://www.w3.org/TR/WGSL/) shader for this example.
const SHADER: &'static str = r##"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"##;

/// Represents the different available post-processing effects.
#[allow(unused)]
enum EffectType {
    /// The [wpp::grayscale::GrayscaleEffect].
    Grayscale,

    /// The [wpp::fxaa::FxaaEffect].
    Fxaa,
}

/// Runs the post-proccessing demonstration.
///
/// Arguments:
///
/// * `effect`: Allows selection of which post-processing effect to apply.
/// * `enable_effect`: Wether the post-processing effect should be enabled.
/// * `event_loop`: The `winit` [EventLoop].
/// * `window`: The [Window] from `winit`.
async fn run(effect: EffectType, enable_effect: bool, event_loop: EventLoop<()>, window: Window) {
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let surface = unsafe {
        instance
            .create_surface(&window)
            .expect("Failed to create surface.")
    };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(SHADER)),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let capabilities = surface.get_capabilities(&adapter);

    let swapchain_format = capabilities.formats[0];
    let alpha_mode = capabilities.alpha_modes[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode,
        view_formats: vec![swapchain_format],
    };

    surface.configure(&device, &config);

    let extent = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        ..Default::default()
    };

    // Create the post-processing effect.
    let mut handler = match effect {
        EffectType::Grayscale => wpp::fxaa::FxaaEffect::new(&device, &extent, swapchain_format),
        EffectType::Fxaa => wpp::fxaa::FxaaEffect::new(&device, &extent, swapchain_format),
    };

    event_loop.run(move |event, _, control_flow| {
        let _ = (&instance, &adapter, &shader, &pipeline_layout);

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);

                let extent = wgpu::Extent3d {
                    width: size.width,
                    height: size.height,
                    ..Default::default()
                };

                // Resize the post-processing effect accordingly.
                handler.resize(&device, &extent);

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                // Create a new frame for post-processing.
                let process_frame = if enable_effect {
                    Some(handler.start_frame(&device, &queue, &view))
                } else {
                    None
                };

                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: if let Some(frame) = &process_frame {
                                frame
                            } else {
                                &view
                            },
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));

                if let Some(frame) = process_frame {
                    // Resolve the post-processing on the frame after rendering the triangle.
                    frame.resolve();
                }

                // Finally present the post-processed frame.
                frame.present();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

/// Runs the example.
fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();

    pollster::block_on(run(EffectType::Fxaa, true, event_loop, window));
}
