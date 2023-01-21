use wgpu::include_wgsl;

/// Wraps the internal fields and implementation
/// of a [crate::fxaa::FxaaEffect].
pub(super) struct FxaaEffectInner {
    pub(super) pipeline: wgpu::RenderPipeline,
    pub(super) bind_group: wgpu::BindGroup,
    pub(super) target: wgpu::TextureView,
    pub(super) format: wgpu::TextureFormat,
    pub(super) size: wgpu::Extent3d,
}

impl FxaaEffectInner {
    /// Creates a new [FxaaEffectInner].
    ///
    /// Arguments:
    ///
    /// * `device`: The current graphics device.
    /// * `size`: The size of the frame which will later be processed.
    /// * `format`: The texture format of the post-processed frame.
    ///
    /// Returns:
    ///
    /// The new [FxaaEffectInner].
    pub(super) fn new(
        device: &wgpu::Device,
        size: &wgpu::Extent3d,
        format: wgpu::TextureFormat,
    ) -> Self {
        let texture_desc = wgpu::TextureDescriptor {
            size: *size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: None,
        };

        let target = device
            .create_texture(&wgpu::TextureDescriptor {
                format,
                ..texture_desc
            })
            .create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&target),
            }],
            layout: &bind_group_layout,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let fxaa_shader = device.create_shader_module(include_wgsl!("shader/fxaa.wgsl"));

        let fxaa_shader_vert = wgpu::VertexState {
            module: &fxaa_shader,
            entry_point: "vs_main",
            buffers: &[],
        };

        let fxaa_shader_frag = wgpu::FragmentState {
            module: &fxaa_shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: fxaa_shader_vert,
            fragment: Some(fxaa_shader_frag),
            primitive: Default::default(),
            multisample: Default::default(),
            depth_stencil: None,
            multiview: None,
        });

        Self {
            pipeline,
            bind_group,
            target,
            format,
            size: *size,
        }
    }

    /// Resizes the [FxaaEffectInner] after creation.
    ///
    /// This should be called when the main surface is resized, so that
    /// no new [FxaaEffectInner] must be created which is slightly faster.
    ///
    /// Arguments:
    ///
    /// * `device`: The current graphics device.
    /// * `size`: The new size to which the effect should be resized.
    pub(crate) fn resize(&mut self, device: &wgpu::Device, size: &wgpu::Extent3d) {
        self.size = *size;

        let texture_desc = wgpu::TextureDescriptor {
            size: self.size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: None,
        };

        self.target = device
            .create_texture(&wgpu::TextureDescriptor {
                format: self.format,
                ..texture_desc
            })
            .create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }],
        });

        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&self.target),
            }],
            layout: &bind_group_layout,
        });
    }
}
