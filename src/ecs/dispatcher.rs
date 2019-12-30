use specs::prelude::*;
use luminance_glfw::GlfwSurface;
use std::sync::Arc;
use std::sync::Mutex;
use crate::ecs::systems::physics_system::PhysicsSystem;
use crate::ecs::systems::gl_system::GLSystem;
use crate::tess_manager::TessManager;

pub struct DoemDispatcher;

impl DoemDispatcher {
    pub fn new<'a, 'b>(surface: GlfwSurface, should_quit: Arc<Mutex<bool>>) -> Dispatcher<'a, 'b> {
        DispatcherBuilder::new()
            .with(PhysicsSystem, "physics_system", &[])
            .with_thread_local(GLSystem::new(surface, should_quit))
            .build()
    }
}