use eyre::Result;
use ggez::event::EventHandler;
use ggez::graphics::{self, DrawMode, DrawParam, MeshBuilder, BLACK, WHITE};
use ggez::{timer, Context};
use nalgebra::{Isometry2, Vector2};
use rapier2d::dynamics::{
    BodyStatus, CCDSolver, IntegrationParameters, JointSet, RigidBody, RigidBodyBuilder,
    RigidBodySet,
};
use rapier2d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase};
use rapier2d::pipeline::PhysicsPipeline;

mod vector2;

pub struct MainState {
    pipeline: PhysicsPipeline,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    joints: JointSet,
    ccd_solver: CCDSolver,
}

impl MainState {
    pub fn new(context: &mut Context) -> Result<Self> {
        let pipeline = PhysicsPipeline::new();
        let gravity = vector2::Vector2::new(0.0, 255.81);
        let integration_parameters = IntegrationParameters::default();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();
        let mut joints = JointSet::new();
        let mut ccd_solver = CCDSolver::new();
        // We ignore physics hooks and contact events for now.
        let physics_hooks = ();
        let event_handler = ();

        let rigid_body = RigidBodyBuilder::new(BodyStatus::Dynamic)
            // The rigid body translation.
            // Default: zero vector.
            .translation(0.0, 5.0)
            // The rigid body rotation.
            // Default: no rotation.
            .rotation(5.0)
            // The rigid body position. Will override `.translation(...)` and `.rotation(...)`.
            // Default: the identity isometry.
            .position(Isometry2::new(vector2::Vector2::new(500.0, 200.0), 0.4))
            // The linear velocity of this body.
            // Default: zero velocity.
            // .linvel(1.0, 2.0)
            // The angular velocity of this body.
            // Default: zero velocity.
            // .angvel(2.0)
            // Whether or not this body can sleep.
            // Default: true
            .can_sleep(true)
            // Whether or not CCD is enabled for this rigid-body.
            // Default: false
            .ccd_enabled(false)
            // .additional_mass(-10.5)
            // All done, actually build the rigid-body.
            .build();

        let body_handle = bodies.insert(rigid_body);
        let collider = ColliderBuilder::ball(10.0)
            .density(0.5)
            .restitution(1.0)
            .build();
        let platform = RigidBodyBuilder::new_static()
            .position(Isometry2::new(vector2::Vector2::new(500.0, 300.0), 0.0))
            .build();
        let platform_handle = bodies.insert(platform);
        let static_collider = ColliderBuilder::ball(10.0).build();
        let collider_handle = colliders.insert(collider, body_handle, &mut bodies);
        let static_handle = colliders.insert(static_collider, platform_handle, &mut bodies);

        Ok(Self {
            gravity,
            pipeline,
            integration_parameters,
            broad_phase,
            narrow_phase,
            bodies,
            colliders,
            joints,
            ccd_solver,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        while timer::check_update_time(context, 60) {
            let physics_hooks = ();
            let event_handler = ();
            self.pipeline.step(
                &self.gravity,
                &self.integration_parameters,
                &mut self.broad_phase,
                &mut self.narrow_phase,
                &mut self.bodies,
                &mut self.colliders,
                &mut self.joints,
                &mut self.ccd_solver,
                &physics_hooks,
                &event_handler,
            );
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(context, BLACK);
        let mut mesh_builder = MeshBuilder::new();

        for (handle, body) in self.bodies.iter() {
            let x = body.world_com.x;
            let y = body.world_com.y;
            let radius = 10.0;
            let location = [x, y];
            mesh_builder.circle(DrawMode::fill(), location, radius, 0.1, WHITE);
        }

        let mesh = mesh_builder.build(context)?;

        graphics::draw(context, &mesh, DrawParam::default())?;
        graphics::present(context)
    }
}
