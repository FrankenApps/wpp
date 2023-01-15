use meshtext::{MeshGenerator, MeshText, TextSection};
use rand::{prelude::StdRng, Rng, SeedableRng};
use std::borrow::Cow;
use wgpu::util::DeviceExt;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use wpp::{Effect, Frame};

/// An RGB color with components in range `0..=1`.
type RGBColor = [f32; 3];

/// A point within the 2D plane.
type Point = [f32; 2];

/// The [WGSL](https://www.w3.org/TR/WGSL/) shader for this example.
const SHADER: &'static str = r##"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(in.position.x, in.position.y, 0.0, 1.0);
    out.color = vec4<f32>(in.color.x, in.color.y, in.color.z, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"##;

/// An array of example strings used for rendering the test scene.
const SAMPLE_TEXT: &[&str] = &[
    "ALGOL",
    "BASIC",
    "C",
    "C++",
    "C#",
    "COBOL",
    "Dart",
    "Delphi",
    "Elixir",
    "Erlang",
    "Fortran",
    "Go",
    "Haskell",
    "HTML",
    "Java",
    "JavaScript",
    "Julia",
    "Kotlin",
    "Lisp",
    "Lua",
    "Nim",
    "Objective-C",
    "OCaml",
    "Perl",
    "Php",
    "Prolog",
    "Python",
    "R",
    "Rust",
    "Solidity",
    "SQL",
    "Swift",
    "TypeScript",
];

/// Represents the different available post-processing effects.
enum EffectType {
    /// The [wpp::grayscale::GrayscaleEffect].
    Grayscale,
}

/// Runs the post-proccessing demonstration.
///
/// Arguments:
///
/// * `effect`: Allows selection of which post-processing effect to apply.
/// * `enable_effect`: Wether the post-processing effect should be enabled.
/// * `event_loop`: The `winit` [EventLoop].
/// * `vertex_count`: The number of vertices.
/// * `vertex_data`: The vertex data.
/// * `window`: The [Window] from `winit`.
async fn run(
    effect: EffectType,
    enable_effect: bool,
    event_loop: EventLoop<()>,
    vertex_count: u32,
    vertex_data: &[u8],
    window: Window,
) {
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) };
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
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

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor::default());

    let swapchain_format = surface.get_supported_formats(&adapter)[0];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: vertex_data,
        usage: wgpu::BufferUsages::VERTEX,
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<f32>() as wgpu::BufferAddress * 5,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x2,
                    },
                    wgpu::VertexAttribute {
                        offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                        shader_location: 1,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                ],
            }],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let mut config = wgpu::SurfaceConfiguration {
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        format: swapchain_format,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        width: size.width,
    };

    surface.configure(&device, &config);

    let extent = wgpu::Extent3d {
        width: size.width,
        height: size.height,
        ..Default::default()
    };

    let mut handler = match effect {
        EffectType::Grayscale => {
            wpp::grayscale::GrayscaleEffect::new(&device, &extent, swapchain_format)
        }
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
                                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                                store: true,
                            },
                        })],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    rpass.set_pipeline(&render_pipeline);
                    rpass.draw(0..vertex_count, 0..1);
                }

                queue.submit(Some(encoder.finish()));

                if let Some(frame) = process_frame {
                    frame.resolve();
                }

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

/// Generates the sample text for the test scene.
///
/// Returns:
///
/// A [Vec] containing tuples of [Point] and [RGBColor], representing
/// text position and color.
fn generate_sample_text() -> Vec<(Point, RGBColor)> {
    let font_data = include_bytes!("../assets/font/FiraMono-Regular.ttf");
    let mut generator = MeshGenerator::new(font_data);

    let mut rand = StdRng::seed_from_u64(0);
    let mut color_rand = StdRng::seed_from_u64(0);

    let mut verts = Vec::new();
    for i in 0..SAMPLE_TEXT.len() {
        let result: MeshText = generator
            .generate_section_2d(SAMPLE_TEXT[i], Some(&get_transform(&mut rand)))
            .expect("Failed to generate text section.");

        let color = [
            color_rand.gen_range(0f32..=1f32),
            color_rand.gen_range(0f32..=1f32),
            color_rand.gen_range(0f32..=1f32),
        ];

        for c in result.vertices.chunks(2) {
            verts.push(([c[0], c[1]], color));
        }
    }

    verts
}

/// Generate a random 2D transformation matrix.
///
/// Arguments:
///
/// * `rand`: The [StdRng] used for generating the transformation.
///
/// Returns:
///
/// The transformation matrix as an `[f32; 9]` array.
fn get_transform(rand: &mut StdRng) -> [f32; 9] {
    let rotation = rand.gen_range(0f32..std::f32::consts::PI);
    let scale = rand.gen_range(0.08..=0.25);

    let transl_x = rand.gen_range(-0.8..=0.8);
    let transl_y = rand.gen_range(-0.8..=0.8);

    [
        rotation.cos() * scale,
        rotation.sin() * scale,
        0f32,
        -rotation.sin() * scale,
        rotation.cos() * scale,
        0f32,
        transl_x,
        transl_y,
        1f32,
    ]
}

/// Runs the example.
fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).expect("Failed to create window.");
    window.set_inner_size(winit::dpi::LogicalSize::new(600, 600));

    let text_vertices = generate_sample_text();
    let mut raw_data: Vec<u8> = Vec::new();
    for vert in text_vertices.iter() {
        for c in vert.0.iter() {
            raw_data.extend_from_slice(c.to_le_bytes().as_slice());
        }

        for c in vert.1.iter() {
            raw_data.extend_from_slice(c.to_le_bytes().as_slice());
        }
    }

    pollster::block_on(run(
        EffectType::Grayscale,
        true,
        event_loop,
        text_vertices.len() as u32,
        raw_data.as_slice(),
        window,
    ));
}
