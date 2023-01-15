use crate::Effect;

use super::{grayscale_effect_inner::GrayscaleEffectInner, GrayscaleFrame};

/// Transforms a colorized input texture into grayscale.
pub struct GrayscaleEffect {
    pub(super) inner: Option<GrayscaleEffectInner>,
}

impl GrayscaleEffect {
    /// Creates a new [GrayscaleEffect].
    ///
    /// Arguments:
    ///
    /// * `device`: The current graphics device.
    /// * `size`: The size of the frame which will later be processed.
    /// * `format`: The texture format of the post-processed frame.
    ///
    /// Returns:
    ///
    /// The new [GrayscaleEffect].
    pub fn new(
        device: &wgpu::Device,
        size: &wgpu::Extent3d,
        format: wgpu::TextureFormat,
    ) -> GrayscaleEffect {
        let inner = Some(GrayscaleEffectInner::new(device, size, format));

        GrayscaleEffect { inner }
    }
}

impl Effect for GrayscaleEffect {
    type Frame<'a> = GrayscaleFrame<'a>;

    /// Resizes the grayscale effect after creation.
    ///
    /// This should be called when the main surface is resized, so that
    /// no new [Effect] must be created which is slightly faster.
    ///
    /// Arguments:
    ///
    /// * `device`: The current graphics device.
    /// * `size`: The new size to which the effect should be resized.
    fn resize(&mut self, device: &wgpu::Device, size: &wgpu::Extent3d) {
        if let Some(ref mut inner) = self.inner {
            inner.resize(device, size);
        }
    }

    /// Creates a new [GrayscaleFrame].
    ///
    /// Arguments:
    ///
    /// * `device`: The current graphics device.
    /// * `queue`: The command queue where the post-processing should be applied.
    /// * `output_view`: The view into which the frame will be resolved.
    ///
    /// Returns:
    ///
    /// The new [GrayscaleFrame].
    fn start_frame<'a>(
        &'a mut self,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        output_view: &'a wgpu::TextureView,
    ) -> GrayscaleFrame<'a> {
        GrayscaleFrame {
            target: self,
            device,
            queue,
            output_view,
        }
    }
}
