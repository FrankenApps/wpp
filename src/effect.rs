/// A post-processing effect.
pub trait Effect {
    /// The associated frame of the this [Effect].
    type Frame<'a>
    where
        Self: 'a;

    /// Resizes the post-processing effect after creation.
    ///
    /// This should be called when the main surface is resized, so that
    /// no new [Effect] must be created which is slightly faster.
    ///
    /// Arguments:
    ///
    /// * `device`: The current graphics device.
    /// * `size`: The new size to which the effect should be resized.
    fn resize(&mut self, device: &wgpu::Device, size: &wgpu::Extent3d);

    /// Creates a new [crate::Frame].
    ///
    /// Arguments:
    ///
    /// * `device`: The current graphics device.
    /// * `queue`: The command queue where the post-processing should be applied.
    /// * `output_view`: The view into which the frame will be resolved.
    ///
    /// Returns:
    ///
    /// The new [crate::Frame].
    fn start_frame<'a>(
        &'a mut self,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        output_view: &'a wgpu::TextureView,
    ) -> Self::Frame<'a>;
}
