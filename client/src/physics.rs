use macroquad::prelude::collections::storage;
use macroquad::prelude::*;
use macroquad_platformer::World as CollisionWorld;
use macroquad_tiled as tiled;
use rapier2d::prelude::*;

use super::constants::consts;
use resources::Resources;
use tiled::Map;

pub struct ExplosionTextures {
    pub deg_90: Texture2D,
    pub fourway: Texture2D,
    pub side: Texture2D,
    pub tail: Texture2D,
    pub threeway: Texture2D,
    pub twoway: Texture2D,
}

pub struct MapSize {
    pub w: u32,
    pub h: u32,
}

impl MapSize {
    pub fn new(w: u32, h: u32) -> Self {
        MapSize { w, h }
    }
}

pub struct Textures {
    pub bomb: Texture2D,
    pub fire: ExplosionTextures,
    pub player: Texture2D,
    pub tileset: Texture2D,
    pub tiled_map_json: String,
    pub tiled_map: Map,
}

pub fn step(resources: &Resources) {
    let gravity = vector![0.0, 0.0];
    let integration_parameters = IntegrationParameters {
        dt: consts::TIMESTEP_RATE as f32,
        ..Default::default()
    };
    let physics_hooks = ();
    let event_handler = ();

    let mut physics_pipeline = resources.get_mut::<PhysicsPipeline>().unwrap();
    let mut island_manager = resources.get_mut::<IslandManager>().unwrap();
    let mut broad_phase = resources.get_mut::<BroadPhase>().unwrap();
    let mut narrow_phase = resources.get_mut::<NarrowPhase>().unwrap();
    let mut rigid_body_set = resources.get_mut::<RigidBodySet>().unwrap();
    let mut collider_set = resources.get_mut::<ColliderSet>().unwrap();
    let mut joint_set = resources.get_mut::<JointSet>().unwrap();
    let mut ccd_solver = resources.get_mut::<CCDSolver>().unwrap();

    physics_pipeline.step(
        &gravity,
        &integration_parameters,
        &mut island_manager,
        &mut broad_phase,
        &mut narrow_phase,
        &mut rigid_body_set,
        &mut collider_set,
        &mut joint_set,
        &mut ccd_solver,
        &physics_hooks,
        &event_handler,
    );
}

async fn textures() -> Result<Textures, macroquad::prelude::FileError> {
    let bomb = load_texture("assets/tiles/bomb.png").await?;
    bomb.set_filter(FilterMode::Nearest);

    let expl_90deg = load_texture("assets/tiles/explosion/explosion-90deg.png").await?;
    expl_90deg.set_filter(FilterMode::Nearest);

    let expl_fourway = load_texture("assets/tiles/explosion/explosion-fourway.png").await?;
    expl_fourway.set_filter(FilterMode::Nearest);

    let expl_side = load_texture("assets/tiles/explosion/explosion-side.png").await?;
    expl_side.set_filter(FilterMode::Nearest);

    let expl_tail = load_texture("assets/tiles/explosion/explosion-tail.png").await?;
    expl_tail.set_filter(FilterMode::Nearest);

    let expl_threeway = load_texture("assets/tiles/explosion/explosion-threeway.png").await?;
    expl_threeway.set_filter(FilterMode::Nearest);

    let expl_twoway = load_texture("assets/tiles/explosion/explosion-twoway.png").await?;
    expl_twoway.set_filter(FilterMode::Nearest);

    let player = load_texture("assets/tiles/ronalds(32x32).png").await?;
    player.set_filter(FilterMode::Nearest);

    let tileset = load_texture("assets/tileset.png").await?;
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("assets/Tiled_BaseMap.json").await.unwrap();
    let tiled_map = tiled::load_map(&tiled_map_json, &[("tileset.png", tileset)], &[]).unwrap();

    Ok(Textures {
        bomb,
        fire: ExplosionTextures {
            deg_90: expl_90deg,
            fourway: expl_fourway,
            side: expl_side,
            tail: expl_tail,
            threeway: expl_threeway,
            twoway: expl_twoway,
        },
        player,
        tileset,
        tiled_map_json,
        tiled_map,
    })
}

pub async fn init_resources() -> Resources {
    let mut resources = Resources::new();
    let loaded_textures = textures().await.unwrap();

    let rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    for (x, y, _) in loaded_textures.tiled_map.tiles("walls", None) {
        let colider_block = ColliderBuilder::cuboid(consts::TILE_SIZE, consts::TILE_SIZE)
            .position(Isometry::new(vector![x as f32, y as f32], 0.))
            .build();
        // static_collider.push(colider_block);
        collider_set.insert(colider_block);
    }

    let mut static_colliders = vec![];
    for (_x, _y, tile) in loaded_textures.tiled_map.tiles("walls", None) {
        static_colliders.push(tile.is_some());
    }

    let mut collision_world = CollisionWorld::new();
    collision_world.add_static_tiled_layer(
        static_colliders,
        32.,
        32.,
        loaded_textures.tiled_map.raw_tiled_map.width as _,
        1,
    );

    let map_size = MapSize::new(
        loaded_textures.tiled_map.raw_tiled_map.tilewidth
            * loaded_textures.tiled_map.raw_tiled_map.width,
        loaded_textures.tiled_map.raw_tiled_map.tileheight
            * loaded_textures.tiled_map.raw_tiled_map.height,
    );

    resources.insert(PhysicsPipeline::new());
    resources.insert(IslandManager::new());
    resources.insert(BroadPhase::new());
    resources.insert(NarrowPhase::new());
    resources.insert(rigid_body_set);
    resources.insert(collider_set);
    resources.insert(JointSet::new());
    resources.insert(CCDSolver::new());
    resources.insert(loaded_textures);
    resources.insert(collision_world);
    resources.insert(map_size);

    return resources;
}
