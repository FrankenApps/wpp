use crate::Frame;

use super::GrayscaleEffect;

/// The [GrayscaleFrame] will be used to resolve the [GrayscaleEffect].
pub struct GrayscaleFrame<'a> {
    pub(super) target: &'a mut GrayscaleEffect,
    pub(super) device: &'a wgpu::Device,
    pub(super) queue: &'a wgpu::Queue,
    pub(super) output_view: &'a wgpu::TextureView,
}
impl<'a> Frame for GrayscaleFrame<'a> {
    /// Resolves the [GrayscaleEffect] on this [GrayscaleFrame].
    fn resolve(self) {
        std::mem::drop(self);
    }
}
impl<'a> std::ops::Deref for GrayscaleFrame<'a> {
    type Target = wgpu::TextureView;
    fn deref(&self) -> &Self::Target {
        match self.target.inner {
            None => self.output_view,
            Some(ref inner) => &inner.target,
        }
    }
}
impl<'a> Drop for GrayscaleFrame<'a> {
    fn drop(&mut self) {
        if let Some(ref mut inner) = self.target.inner {
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: self.output_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                    label: None,
                });
                rpass.set_pipeline(&inner.pipeline);
                rpass.set_bind_group(0, &inner.bind_group, &[]);
                rpass.draw(0..6, 0..1);
            }
            self.queue.submit(Some(encoder.finish()));
        }
    }
}
