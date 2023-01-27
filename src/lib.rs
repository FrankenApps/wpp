//! Flexible and reusable post-processing effects for [`wgpu`](https://wgpu.rs/).
//!
//! This library will provide a collection of post-processing effects that you can easily integrate into your rendering pipeline.
//!
//! ## Effects
//!
//! * A simple [grayscale] effect.
//!
//! ## Usage
//! For example the _grayscale_ effect can be used in principle like so:
//! ```rust
//! # use winit::event::{Event, WindowEvent};
//! # use winit::event_loop::EventLoop;
//! # use winit::window::Window;
//! #
//! use crate::wpp::{Effect, Frame};
//! #
//! # fn main() { pollster::block_on(run()); }
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//!
//! // Initialize winit and wgpu
//! let event_loop = EventLoop::new();
//! let window = winit::window::Window::new(&event_loop).unwrap();
//! let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
//! let surface = unsafe { instance.create_surface(&window).unwrap() };
//! let adapter = instance.request_adapter(&Default::default()).await.unwrap();
//! let (device, queue) = adapter.request_device(&Default::default(), None).await.unwrap();
//! let capabilities = surface.get_capabilities(&adapter);
//! let swapchain_format = capabilities.formats[0];
//! let mut config = wgpu::SurfaceConfiguration {
//!     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//!     format: swapchain_format,
//!     width: window.inner_size().width,
//!     height: window.inner_size().height,
//!     present_mode: wgpu::PresentMode::Fifo,
//!     alpha_mode: wgpu::CompositeAlphaMode::Opaque,
//!     view_formats: vec![swapchain_format],
//! };
//! surface.configure(&device, &config);
//!
//! let extent = wgpu::Extent3d {
//!     width: window.inner_size().width,
//!     height: window.inner_size().height,
//!     ..Default::default()
//! };
//!
//! // Create the post-processing effect.
//! let mut handler = wpp::grayscale::GrayscaleEffect::new(&device, &extent, swapchain_format);
//!
//! // The event / render loop.
//! event_loop.run(move |event, _, control_flow| {
//!     match event {
//!         Event::WindowEvent {
//!             event: WindowEvent::Resized(size),
//!             ..
//!         } => {
//!             let extent = wgpu::Extent3d {
//!                 width: size.width,
//!                 height: size.height,
//!                 ..Default::default()
//!             };
//!
//!             // Resize the post-processing effect accordingly.
//!             handler.resize(&device, &extent);
//!         
//!             window.request_redraw();
//!         }
//!         Event::RedrawRequested(_) => {
//!             let frame = surface
//!                 .get_current_texture()
//!                 .unwrap();
//!             let view = frame
//!                 .texture
//!                 .create_view(&wgpu::TextureViewDescriptor::default());
//!
//!             // Create a new frame for post-processing.
//!             let process_frame = handler.start_frame(&device, &queue, &view);
//!
//!             // Render into the `process_frame`.
//!             // [...]
//!
//!             // Resolve the post-processing on the frame after rendering.
//!             process_frame.resolve();
//!
//!             // Finally present the post-processed frame.
//!             frame.present();
//!     
//!             # *control_flow = winit::event_loop::ControlFlow::Exit;
//!         }
//!         _ => {}
//!     }
//! });
//! #
//! # }
//! ```

#![deny(missing_docs)]

mod effect;
pub use effect::Effect;

mod frame;
pub use frame::Frame;

/// Provides the
/// [fast approximate anti-aliasing](https://en.wikipedia.org/wiki/Fast_approximate_anti-aliasing)
/// effect.
#[cfg(feature = "fxaa")]
pub mod fxaa;

/// Transforms colorized frames into a simple grayscale version.
#[cfg(feature = "grayscale")]
pub mod grayscale;
