#![allow(warnings)]
use rand::Rng;

use cgmath::{Matrix3, Matrix4, Point3, Rad, Vector3};
use glam::{
    f32::{Mat3, Vec3},
    Mat4,
};
use std::{error::Error, sync::Arc, time::Instant};
use vulkano::{
    buffer::{
        allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo},
        Buffer, BufferCreateInfo, BufferUsage, Subbuffer,
    },
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderPassBeginInfo,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, DescriptorSet, WriteDescriptorSet,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, DeviceOwned,
        Queue, QueueCreateInfo, QueueFlags,
    },
    format::Format,
    image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            depth_stencil::{DepthState, DepthStencilState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    query::{QueryControlFlags, QueryPool, QueryPoolCreateInfo, QueryType},
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    shader::EntryPoint,
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Validated, VulkanError, VulkanLibrary,
};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceEvent, DeviceId, ElementState, RawKeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    monitor::Fullscreen,
    window::{Window, WindowAttributes, WindowId},
};

mod display_mods;
use display_mods::{oclock, record_nanos, Groupable};

mod f32_3;
use f32_3::gen_f32_3;

mod f64_3;
use f64_3::{gen_f64_3, mltply_f64_3, nrmlz_f64_3};

mod positions;
use positions::{Normal, Position};

mod shapes;
mod u_modular;

mod magma_ocean;
use magma_ocean::Stone;

mod anomaly;
use anomaly::{add_particle_by, e, progress, q, view, Anomaly, LS_F64, TS_F64};

mod moving_around;
use moving_around::{
    move_elevation, move_forwards, move_sideways, rotate_horizontal, rotate_up, rotate_vertical,
};

pub struct Bv {
    pub v: Subbuffer<[Position]>,
    pub n: Subbuffer<[Normal]>,
    pub i: Subbuffer<[u32]>,
}

fn main() -> Result<(), impl Error> {
    // The start of this example is exactly the same as `triangle`. You should read the `triangle`
    // example if you haven't done so yet.

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new(&event_loop);

    event_loop.run_app(&mut app)
}

struct App {
    instance: Arc<Instance>,
    device: Arc<Device>,
    query_pool: Arc<QueryPool>,
    queue: Arc<Queue>,
    memory_allocator: Arc<StandardMemoryAllocator>,
    descriptor_set_allocator: Arc<StandardDescriptorSetAllocator>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    uniform_buffer_allocator: SubbufferAllocator,
    rcx: Option<RenderContext>,
    u61qate: U61qate,
}

struct RenderContext {
    window: Arc<Box<dyn Window>>,
    swapchain: Arc<Swapchain>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    vs: EntryPoint,
    fs: EntryPoint,
    pipeline: Arc<GraphicsPipeline>,
    recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    rotation_start: Instant,
}

struct U61qate {
    view_point: Position,
    u61q: Anomaly,
    center: Position,
    up_direction: Position,
    rot_static: bool,
    moving_forward: bool,
    moving_backward: bool,
    moving_left: bool,
    moving_right: bool,
    moving_up: bool,
    moving_down: bool,
    rotating_left: bool,
    rotating_right: bool,
    turning_left: bool,
    turning_right: bool,
    turning_up: bool,
    turning_down: bool,
}

impl App {
    fn new(event_loop: &EventLoop) -> Self {
        let library = VulkanLibrary::new().unwrap();
        let required_extensions = Surface::required_extensions(event_loop).unwrap();
        let instance = Instance::new(
            &library,
            &InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: &required_extensions,
                ..Default::default()
            },
        )
        .unwrap();

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && p.presentation_support(i as u32, event_loop).unwrap()
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .unwrap();

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        let (device, mut queues) = Device::new(
            &physical_device,
            &DeviceCreateInfo {
                enabled_extensions: &device_extensions,
                queue_create_infos: &[QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .unwrap();

        let queue = queues.next().unwrap();

        let memory_allocator = Arc::new(StandardMemoryAllocator::new(&device, &Default::default()));
        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            &device,
            &Default::default(),
        ));

        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            &device,
            &Default::default(),
        ));

        let uniform_buffer_allocator = SubbufferAllocator::new(
            &memory_allocator,
            &SubbufferAllocatorCreateInfo {
                buffer_usage: BufferUsage::UNIFORM_BUFFER,
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
        );

        let mut rng = rand::thread_rng();

        let mut anomaly = Anomaly {
            anomaly: vec![],
            component: vec![],
            force: vec![],
        };

        let k = 10;

        for _ in 0..k {
            add_particle_by(
                &mut anomaly,
                e(
                    gen_f32_3(0.0, 69.0, &mut rng),
                    mltply_f64_3(nrmlz_f64_3(gen_f64_3(0.0, 10.0, &mut rng)), LS_F64),
                    true,
                ),
            );
            add_particle_by(
                &mut anomaly,
                q(
                    gen_f32_3(0.0, 69.0, &mut rng),
                    mltply_f64_3(nrmlz_f64_3(gen_f64_3(0.0, 10.0, &mut rng)), LS_F64),
                    true,
                    true,
                    rng.gen_range(0..3),
                    rng.gen_range(0..1),
                ),
            );
        }

        // Create a query pool for occlusion queries, with 3 slots.
        let query_pool = QueryPool::new(
            &device,
            &QueryPoolCreateInfo {
                query_count: 60,
                ..QueryPoolCreateInfo::query_type(QueryType::Occlusion)
            },
        )
        .unwrap();

        App {
            instance,
            device,
            query_pool,
            queue,
            memory_allocator,
            descriptor_set_allocator,
            command_buffer_allocator,
            uniform_buffer_allocator,
            rcx: None,
            u61qate: U61qate {
                u61q: anomaly,
                view_point: Position {
                    position: [0.0, -1.0, 1.0],
                },

                center: Position {
                    position: [0.0, 0.0, 0.0],
                },

                up_direction: Position {
                    position: [0.0, -1.0, 0.0],
                },

                rot_static: true,
                moving_forward: false,
                moving_backward: false,
                moving_left: false,
                moving_right: false,
                moving_up: false,
                moving_down: false,
                rotating_left: false,
                rotating_right: false,
                turning_left: false,
                turning_right: false,
                turning_up: false,
                turning_down: false,
            },
        }
    }
}

impl ApplicationHandler for App {
    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("u61q")
                        .with_fullscreen(Some(Fullscreen::Borderless(None))),
                )
                .unwrap(),
        );
        let surface = Surface::from_window(&self.instance, &window).unwrap();
        let window_size = window.surface_size();

        let (swapchain, images) = {
            let surface_capabilities = self
                .device
                .physical_device()
                .surface_capabilities(&surface, &Default::default())
                .unwrap();
            let (image_format, _) = self
                .device
                .physical_device()
                .surface_formats(&surface, &Default::default())
                .unwrap()[0];

            Swapchain::new(
                &self.device,
                &surface,
                &SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count.max(2),
                    image_format,
                    image_extent: window_size.into(),
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),
                    ..Default::default()
                },
            )
            .unwrap()
        };

        let render_pass = vulkano::single_pass_renderpass!(
            &self.device,
            attachments: {
                color: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
                depth_stencil: {
                    format: Format::D16_UNORM,
                    samples: 1,
                    load_op: Clear,
                    store_op: DontCare,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {depth_stencil},
            },
        )
        .unwrap();

        let vs = vs::load(&self.device).unwrap().entry_point("main").unwrap();
        let fs = fs::load(&self.device).unwrap().entry_point("main").unwrap();

        let (framebuffers, pipeline) = window_size_dependent_setup(
            window_size,
            &images,
            &render_pass,
            &self.memory_allocator,
            &vs,
            &fs,
        );

        let previous_frame_end = Some(sync::now(self.device.clone()).boxed());

        let rotation_start = Instant::now();

        self.rcx = Some(RenderContext {
            window,
            swapchain,
            render_pass,
            framebuffers,
            vs,
            fs,
            pipeline,
            recreate_swapchain: false,
            previous_frame_end,
            rotation_start,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let rcx = self.rcx.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::SurfaceResized(_) => {
                rcx.recreate_swapchain = true;
            }
            WindowEvent::RedrawRequested => {
                if self.u61qate.moving_forward {
                    move_forwards(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        0.01,
                    );
                }
                if self.u61qate.moving_backward {
                    move_forwards(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        -0.01,
                    );
                }
                if self.u61qate.moving_left {
                    move_sideways(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        -0.01,
                    );
                }
                if self.u61qate.moving_right {
                    move_sideways(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        0.01,
                    );
                }
                if self.u61qate.moving_up {
                    move_elevation(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        0.01,
                    );
                }
                if self.u61qate.moving_down {
                    move_elevation(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        -0.01,
                    );
                }
                if self.u61qate.rotating_left {
                    rotate_up(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        -0.01,
                    );
                }
                if self.u61qate.rotating_right {
                    rotate_up(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        0.01,
                    );
                }
                if self.u61qate.turning_left {
                    rotate_horizontal(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        -0.01,
                    );
                }
                if self.u61qate.turning_right {
                    rotate_horizontal(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        0.01,
                    );
                }
                if self.u61qate.turning_up {
                    rotate_vertical(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        0.01,
                    );
                }
                if self.u61qate.turning_down {
                    rotate_vertical(
                        &mut self.u61qate.view_point,
                        &mut self.u61qate.center,
                        &mut self.u61qate.up_direction,
                        -0.01,
                    );
                }

                progress(&mut self.u61qate.u61q, TS_F64);
                let get = view(&mut self.u61qate.u61q);

                let mut bvs: Vec<Bv> = vec![];

                for mut g in get {
                    let (vertex_buffer, normals_buffer, index_buffer) =
                        load_buffers_short(&mut g, self.memory_allocator.clone());
                    bvs.push(Bv {
                        v: vertex_buffer,
                        n: normals_buffer,
                        i: index_buffer,
                    });
                }

                let window_size = rcx.window.surface_size();

                if window_size.width == 0 || window_size.height == 0 {
                    return;
                }

                rcx.previous_frame_end.as_mut().unwrap().cleanup_finished();

                if rcx.recreate_swapchain {
                    let (new_swapchain, new_images) = rcx
                        .swapchain
                        .recreate(&SwapchainCreateInfo {
                            image_extent: window_size.into(),
                            ..rcx.swapchain.create_info()
                        })
                        .expect("failed to recreate swapchain");

                    rcx.swapchain = new_swapchain;
                    (rcx.framebuffers, rcx.pipeline) = window_size_dependent_setup(
                        window_size,
                        &new_images,
                        &rcx.render_pass,
                        &self.memory_allocator,
                        &rcx.vs,
                        &rcx.fs,
                    );
                    rcx.recreate_swapchain = false;
                }

                let uniform_buffer = {
                    let elapsed = rcx.rotation_start.elapsed();
                    let rotation =
                        elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;
                    let rotation = Mat3::from_rotation_y(rotation as f32);

                    // NOTE: This teapot was meant for OpenGL where the origin is at the lower left
                    // instead the origin is at the upper left in Vulkan, so we reverse the Y axis.
                    let aspect_ratio = rcx.swapchain.image_extent()[0] as f32
                        / rcx.swapchain.image_extent()[1] as f32;

                    let proj = Mat4::perspective_rh_gl(
                        std::f32::consts::FRAC_PI_2,
                        aspect_ratio,
                        0.01,
                        100.0,
                    );
                    let view = Mat4::look_at_rh(
                        Vec3::new(
                            self.u61qate.view_point.position[0],
                            self.u61qate.view_point.position[1],
                            self.u61qate.view_point.position[2],
                        ),
                        Vec3::new(
                            self.u61qate.center.position[0],
                            self.u61qate.center.position[1],
                            self.u61qate.center.position[2],
                        ),
                        Vec3::new(
                            self.u61qate.up_direction.position[0],
                            self.u61qate.up_direction.position[1],
                            self.u61qate.up_direction.position[2],
                        ),
                    );

                    let scale = Mat4::from_scale(Vec3::splat(0.01));

                    let mut rotation = 0.0;
                    if !self.u61qate.rot_static {
                        rotation = elapsed.as_secs() as f64
                            + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;
                    }
                    let rotation = Matrix3::from_angle_y(Rad(rotation as f32));

                    let uniform_data = vs::Data {
                        world: Matrix4::from(rotation).into(),
                        view: (view * scale).to_cols_array_2d(),
                        proj: proj.to_cols_array_2d(),
                    };

                    let buffer = self.uniform_buffer_allocator.allocate_sized().unwrap();
                    *buffer.write().unwrap() = uniform_data;

                    buffer
                };

                let layout = &rcx.pipeline.layout().set_layouts()[0];
                let descriptor_set = DescriptorSet::new(
                    self.descriptor_set_allocator.clone(),
                    layout.clone(),
                    [WriteDescriptorSet::buffer(0, uniform_buffer)],
                    [],
                )
                .unwrap();

                let (image_index, suboptimal, acquire_future) = match acquire_next_image(
                    rcx.swapchain.clone(),
                    None,
                )
                .map_err(Validated::unwrap)
                {
                    Ok(r) => r,
                    Err(VulkanError::OutOfDate) => {
                        rcx.recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("failed to acquire next image: {e}"),
                };

                if suboptimal {
                    rcx.recreate_swapchain = true;
                }

                let mut builder = AutoCommandBufferBuilder::primary(
                    self.command_buffer_allocator.clone(),
                    self.queue.queue_family_index(),
                    CommandBufferUsage::OneTimeSubmit,
                )
                .unwrap();

                unsafe {
                    builder
                        .reset_query_pool(self.query_pool.clone(), 0..3)
                        .unwrap()
                        .begin_render_pass(
                            RenderPassBeginInfo {
                                clear_values: vec![
                                    Some([0.0, 0.0, 1.0, 1.0].into()),
                                    Some(1f32.into()),
                                ],
                                ..RenderPassBeginInfo::framebuffer(
                                    rcx.framebuffers[image_index as usize].clone(),
                                )
                            },
                            Default::default(),
                        )
                        .unwrap()
                        .bind_pipeline_graphics(rcx.pipeline.clone())
                        .unwrap()
                        .bind_descriptor_sets(
                            PipelineBindPoint::Graphics,
                            rcx.pipeline.layout().clone(),
                            0,
                            descriptor_set,
                        )
                        .unwrap();

                    for x in bvs {
                        builder
                            .begin_query(
                                self.query_pool.clone(),
                                0,
                                QueryControlFlags::empty(),
                                // QueryControlFlags::PRECISE,
                            )
                            .unwrap()
                            .bind_vertex_buffers(0, (x.v.clone(), x.n.clone()))
                            .unwrap()
                            .bind_index_buffer(x.i.clone())
                            .unwrap()
                            .draw_indexed(x.i.len() as u32 as u32, 1, 0, 0, 0)
                            .unwrap()
                            .end_query(self.query_pool.clone(), 0)
                            .unwrap();
                    }
                }

                builder.end_render_pass(Default::default()).unwrap();

                let command_buffer = builder.build().unwrap();
                let future = rcx
                    .previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(self.queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(
                        self.queue.clone(),
                        SwapchainPresentInfo::new(rcx.swapchain.clone(), image_index),
                    )
                    .then_signal_fence_and_flush();

                match future.map_err(Validated::unwrap) {
                    Ok(future) => {
                        rcx.previous_frame_end = Some(future.boxed());
                    }
                    Err(VulkanError::OutOfDate) => {
                        rcx.recreate_swapchain = true;
                        rcx.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                    }
                    Err(e) => {
                        println!("failed to flush future: {e}");
                        rcx.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                    }
                }
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &dyn ActiveEventLoop,
        device_id: Option<DeviceId>,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::PointerMotion { delta, .. } => {
                rotate_horizontal(
                    &mut self.u61qate.view_point,
                    &mut self.u61qate.center,
                    &mut self.u61qate.up_direction,
                    delta.0 as f32 / 400.0,
                );
                rotate_vertical(
                    &mut self.u61qate.view_point,
                    &mut self.u61qate.center,
                    &mut self.u61qate.up_direction,
                    delta.1 as f32 / 400.0,
                );
            }
            DeviceEvent::Key(RawKeyEvent {
                physical_key,
                state: ElementState::Pressed,
                ..
            }) => match physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => {
                    self.u61qate.moving_forward = true;
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    self.u61qate.moving_backward = true;
                }
                PhysicalKey::Code(KeyCode::KeyA) => {
                    self.u61qate.moving_left = true;
                }
                PhysicalKey::Code(KeyCode::KeyD) => {
                    self.u61qate.moving_right = true;
                }
                PhysicalKey::Code(KeyCode::KeyR) => {
                    self.u61qate.moving_up = true;
                }
                PhysicalKey::Code(KeyCode::KeyF) => {
                    self.u61qate.moving_down = true;
                }
                PhysicalKey::Code(KeyCode::KeyQ) => {
                    self.u61qate.rotating_left = true;
                }
                PhysicalKey::Code(KeyCode::KeyE) => {
                    self.u61qate.rotating_right = true;
                }
                PhysicalKey::Code(KeyCode::KeyX) => {
                    self.u61qate.turning_left = true;
                }
                PhysicalKey::Code(KeyCode::KeyC) => {
                    self.u61qate.turning_right = true;
                }
                PhysicalKey::Code(KeyCode::KeyT) => {
                    self.u61qate.turning_up = true;
                }
                PhysicalKey::Code(KeyCode::KeyG) => {
                    self.u61qate.turning_down = true;
                }
                PhysicalKey::Code(KeyCode::KeyP) => {
                    if self.u61qate.rot_static {
                        self.u61qate.rot_static = false;
                    } else {
                        self.u61qate.rot_static = true;
                    }
                }
                _ => (),
            },
            DeviceEvent::Key(RawKeyEvent {
                physical_key,
                state: ElementState::Released,
                ..
            }) => match physical_key {
                PhysicalKey::Code(KeyCode::KeyW) => {
                    self.u61qate.moving_forward = false;
                }
                PhysicalKey::Code(KeyCode::KeyS) => {
                    self.u61qate.moving_backward = false;
                }
                PhysicalKey::Code(KeyCode::KeyA) => {
                    self.u61qate.moving_left = false;
                }
                PhysicalKey::Code(KeyCode::KeyD) => {
                    self.u61qate.moving_right = false;
                }
                PhysicalKey::Code(KeyCode::KeyR) => {
                    self.u61qate.moving_up = false;
                }
                PhysicalKey::Code(KeyCode::KeyF) => {
                    self.u61qate.moving_down = false;
                }
                PhysicalKey::Code(KeyCode::KeyQ) => {
                    self.u61qate.rotating_left = false;
                }
                PhysicalKey::Code(KeyCode::KeyE) => {
                    self.u61qate.rotating_right = false;
                }
                PhysicalKey::Code(KeyCode::KeyX) => {
                    self.u61qate.turning_left = false;
                }
                PhysicalKey::Code(KeyCode::KeyC) => {
                    self.u61qate.turning_right = false;
                }
                PhysicalKey::Code(KeyCode::KeyT) => {
                    self.u61qate.turning_up = false;
                }
                PhysicalKey::Code(KeyCode::KeyG) => {
                    self.u61qate.turning_down = false;
                }
                _ => (),
            },
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &dyn ActiveEventLoop) {
        let rcx = self.rcx.as_mut().unwrap();
        rcx.window.request_redraw();
    }
}

/// This function is called once during initialization, then again whenever the window is resized.
fn window_size_dependent_setup(
    window_size: PhysicalSize<u32>,
    images: &[Arc<Image>],
    render_pass: &Arc<RenderPass>,
    memory_allocator: &Arc<StandardMemoryAllocator>,
    vs: &EntryPoint,
    fs: &EntryPoint,
) -> (Vec<Arc<Framebuffer>>, Arc<GraphicsPipeline>) {
    let device = memory_allocator.device();

    let depth_buffer = ImageView::new_default(
        &Image::new(
            memory_allocator,
            &ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::D16_UNORM,
                extent: images[0].extent(),
                usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT | ImageUsage::TRANSIENT_ATTACHMENT,
                ..Default::default()
            },
            &AllocationCreateInfo::default(),
        )
        .unwrap(),
    )
    .unwrap();

    let framebuffers = images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image).unwrap();

            Framebuffer::new(
                render_pass,
                &FramebufferCreateInfo {
                    attachments: &[&view, &depth_buffer],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    // In the triangle example we use a dynamic viewport, as its a simple example. However in the
    // teapot example, we recreate the pipelines with a hardcoded viewport instead. This allows the
    // driver to optimize things, at the cost of slower window resizes.
    // https://computergraphics.stackexchange.com/questions/5742/vulkan-best-way-of-updating-pipeline-viewport
    let pipeline = {
        let vertex_input_state = [Position::per_vertex(), Normal::per_vertex()]
            .definition(vs)
            .unwrap();
        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];
        let layout = PipelineLayout::from_stages(device, &stages).unwrap();
        let subpass = Subpass::new(render_pass, 0).unwrap();

        GraphicsPipeline::new(
            device,
            None,
            &GraphicsPipelineCreateInfo {
                stages: &stages,
                vertex_input_state: Some(&vertex_input_state),
                input_assembly_state: Some(&InputAssemblyState::default()),
                viewport_state: Some(&ViewportState {
                    viewports: &[Viewport {
                        offset: [0.0, 0.0],
                        extent: window_size.into(),
                        min_depth: 0.0,
                        max_depth: 1.0,
                    }],
                    ..Default::default()
                }),
                rasterization_state: Some(&RasterizationState::default()),
                depth_stencil_state: Some(&DepthStencilState {
                    depth: Some(DepthState::simple()),
                    ..Default::default()
                }),
                multisample_state: Some(&MultisampleState::default()),
                color_blend_state: Some(&ColorBlendState {
                    attachments: &[ColorBlendAttachmentState::default()],
                    ..Default::default()
                }),
                subpass: Some((&subpass).into()),
                ..GraphicsPipelineCreateInfo::new(&layout)
            },
        )
        .unwrap()
    };

    (framebuffers, pipeline)
}

fn load_buffers_short(
    stone: &mut Stone,
    memory_allocator: Arc<StandardMemoryAllocator>,
    //) -> (u32, u32, u32) {
) -> (Subbuffer<[Position]>, Subbuffer<[Normal]>, Subbuffer<[u32]>) {
    let vertex_buffer = Buffer::from_iter(
        &memory_allocator,
        &BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        &AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        stone.positions.clone(),
    )
    .unwrap();
    let normals_buffer = Buffer::from_iter(
        &memory_allocator,
        &BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        &AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        stone.normals.clone(),
    )
    .unwrap();
    let index_buffer = Buffer::from_iter(
        &memory_allocator,
        &BufferCreateInfo {
            usage: BufferUsage::INDEX_BUFFER,
            ..Default::default()
        },
        &AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        stone.indices.clone(),
    )
    .unwrap();

    return (vertex_buffer, normals_buffer, index_buffer);
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "./src/vert.glsl",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "./src/frag.glsl",
    }
}
