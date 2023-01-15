/// The frame on which the post-processing will take place.
pub trait Frame {
    /// Resolves the [crate::Effect] on the frame.
    fn resolve(self);
}
