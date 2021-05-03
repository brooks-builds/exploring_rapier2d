use std::collections::HashMap;

use entity_data::EntityData;
use eyre::Result;
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawMode, DrawParam, MeshBuilder, BLACK, WHITE};
use ggez::{timer, Context};
use nalgebra::{Isometry2, Vector2};
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use rapier2d::dynamics::{
    BodyStatus, CCDSolver, IntegrationParameters, JointSet, MassProperties, RigidBody,
    RigidBodyBuilder, RigidBodySet,
};
use rapier2d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase};
use rapier2d::math::Point;
use rapier2d::pipeline::PhysicsPipeline;

mod entity_data;
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
    rng: ThreadRng,
    data: EntityData,
}

impl MainState {
    pub fn new(context: &mut Context) -> Result<Self> {
        let pipeline = PhysicsPipeline::new();
        let gravity = vector2::Vector2::new(0.0, 25.0);
        let mut integration_parameters = IntegrationParameters::default();
        integration_parameters.max_position_iterations = 4;
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();
        let mut joints = JointSet::new();
        let mut ccd_solver = CCDSolver::new();
        // We ignore physics hooks and contact events for now.
        let physics_hooks = ();
        let event_handler = ();

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
            rng: thread_rng(),
            data: EntityData::new(),
        })
    }

    fn create_ball(&mut self, context: &mut Context) {
        let (width, height) = graphics::drawable_size(context);
        let position = vector2::Vector2::new(self.rng.gen_range(20.0..width - 20.0), -10.0);
        let color = Color::from_rgb(
            self.rng.gen_range(150..255),
            self.rng.gen_range(150..255),
            self.rng.gen_range(150..255),
        );
        let data_id = self.data.insert_ball(10.0, color);
        let body = RigidBodyBuilder::new(BodyStatus::Dynamic)
            .position(Isometry2::new(position, 0.0))
            .user_data(data_id)
            .build();
        let body_handle = self.bodies.insert(body);
        let radius = 10.0;
        let collider = ColliderBuilder::ball(radius).restitution(1.0).build();

        self.colliders
            .insert(collider, body_handle, &mut self.bodies);
    }

    pub fn setup(&mut self, context: &mut Context) {
        self.create_platform(context);
        self.create_left_wall(context);
        self.create_right_wall(context);
        // self.create_pillar(context);
    }

    fn create_platform(&mut self, context: &mut Context) {
        let (width, height) = graphics::drawable_size(context);
        let position = vector2::Vector2::new(width / 2.0, height - 10.0);
        let id = self.data.insert_platform(0.0, height - 10.0, width, 10.0);
        let body = RigidBodyBuilder::new_static()
            .position(Isometry2::new(position, 0.0))
            .user_data(id)
            .build();

        let body_handle = self.bodies.insert(body);
        let collider = ColliderBuilder::cuboid(width / 2.0, 5.0).build();

        self.colliders
            .insert(collider, body_handle, &mut self.bodies);
    }

    fn create_left_wall(&mut self, context: &mut Context) {
        let (_width, height) = graphics::drawable_size(context);
        let position = Isometry2::new(vector2::Vector2::new(2.5, height / 2.0), 0.0);
        let id = self.data.insert_platform(0.0, 0.0, 5.0, height);
        let body = RigidBodyBuilder::new_static()
            .position(position)
            .user_data(id)
            .build();
        let body_handle = self.bodies.insert(body);
        let collider = ColliderBuilder::cuboid(2.5, height / 2.0).build();
        self.colliders
            .insert(collider, body_handle, &mut self.bodies);
    }

    fn create_right_wall(&mut self, context: &mut Context) {
        let (width, height) = graphics::drawable_size(context);
        let position = Isometry2::new(vector2::Vector2::new(width - 2.5, height / 2.0), 0.0);
        let id = self.data.insert_platform(width - 5.0, 0.0, 5.0, height);
        let body = RigidBodyBuilder::new_static()
            .position(position)
            .user_data(id)
            .build();
        let body_handle = self.bodies.insert(body);
        let collider = ColliderBuilder::cuboid(2.5, height / 2.0).build();
        self.colliders
            .insert(collider, body_handle, &mut self.bodies);
    }

    fn create_pillar(&mut self, context: &mut Context) {
        let (width, height) = graphics::drawable_size(context);
        let position = Isometry2::new(
            vector2::Vector2::new(width / 2.0, height - height * 0.25),
            0.0,
        );
        let id = self
            .data
            .insert_platform(width * 0.25, height * 0.5, width / 2.0, height / 2.0);
        let body = RigidBodyBuilder::new_static()
            .position(position)
            .user_data(id)
            .build();
        let body_handle = self.bodies.insert(body);
        let collider = ColliderBuilder::cuboid(width * 0.25, height * 0.25).build();
        self.colliders
            .insert(collider, body_handle, &mut self.bodies);
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
        if self.bodies.len() < 3400 {
            self.create_ball(context);
        } else if self.bodies.len() == 3400 {
            dbg!("done");
        }
        // if timer::ticks(context) % 200 == 0 {
        //     let fps = timer::fps(context);
        //     if fps > 60.0 {
        //         self.create_ball(context);
        //         dbg!(self.bodies.len());
        //         dbg!(timer::fps(&context));
        //     }
        // }
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(context, BLACK);
        let mut mesh_builder = MeshBuilder::new();

        for (handle, body) in self.bodies.iter() {
            let x = body.world_com.x;
            let y = body.world_com.y;
            let data_id = body.user_data;
            match self.data.get_data_type(data_id) {
                entity_data::DataType::Ball => {
                    let radius = self.data.get_radius(data_id);
                    let location = [x, y];
                    let color = self.data.get_color(data_id);
                    mesh_builder.circle(DrawMode::fill(), location, radius, 0.1, color);
                }
                entity_data::DataType::Platform => {
                    let rect = self.data.get_rect(data_id);
                    mesh_builder.rectangle(DrawMode::fill(), rect, WHITE);
                }
                entity_data::DataType::None => (),
            }
        }

        let mesh = mesh_builder.build(context)?;
        graphics::draw(context, &mesh, DrawParam::default())?;

        graphics::present(context)
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        if let KeyCode::Space = keycode {
            let screenshot = graphics::screenshot(context).unwrap();
            screenshot
                .encode(context, graphics::ImageFormat::Png, "/screenshot.png")
                .unwrap();
        }
    }
}
