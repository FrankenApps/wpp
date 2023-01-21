use crate::Effect;

use super::{fxaa_effect_inner::FxaaEffectInner, FxaaFrame};

/// The
/// [fast approximate anti-aliasing](https://en.wikipedia.org/wiki/Fast_approximate_anti-aliasing)
/// effect.
pub struct FxaaEffect {
    pub(super) inner: Option<FxaaEffectInner>,
}

impl FxaaEffect {
    /// Creates a new [FxaaEffect].
    ///
    /// Arguments:
    ///
    /// * `device`: The current graphics device.
    /// * `size`: The size of the frame which will later be processed.
    /// * `format`: The texture format of the post-processed frame.
    ///
    /// Returns:
    ///
    /// The new [FxaaEffect].
    pub fn new(
        device: &wgpu::Device,
        size: &wgpu::Extent3d,
        format: wgpu::TextureFormat,
    ) -> FxaaEffect {
        let inner = Some(FxaaEffectInner::new(device, size, format));

        FxaaEffect { inner }
    }
}

impl Effect for FxaaEffect {
    type Frame<'a> = FxaaFrame<'a>;

    /// Resizes the [FxaaEffect] after creation.
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

    /// Creates a new [FxaaFrame].
    ///
    /// Arguments:
    ///
    /// * `device`: The current graphics device.
    /// * `queue`: The command queue where the post-processing should be applied.
    /// * `output_view`: The view into which the frame will be resolved.
    ///
    /// Returns:
    ///
    /// The new [FxaaFrame].
    fn start_frame<'a>(
        &'a mut self,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        output_view: &'a wgpu::TextureView,
    ) -> FxaaFrame<'a> {
        FxaaFrame {
            target: self,
            device,
            queue,
            output_view,
        }
    }
}
