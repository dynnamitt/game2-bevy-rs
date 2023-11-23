use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

const PLAYER_PNG: &str = "piecePurple_border05.png";
const MOVE_FORCE: f32 = 1500.0;
const PX_PER_METER: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<Action>::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PX_PER_METER,
        ))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, moveit)
        .run();
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Move,
    Fight,
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, assets_server: Res<AssetServer>) {
    // entity
    commands.spawn(Camera2dBundle::default());

    //entity
    commands
        .spawn(SpatialBundle::default())
        .with_children(|parent| {
            // TODO fetch from noiseMap
            parent
                .spawn(SpriteBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    sprite: Sprite {
                        color: Color::BEIGE,
                        custom_size: Some(Vec2::new(33.0, 33.0)),
                        ..default()
                    },
                    ..default()
                })
                .insert(RigidBody::Fixed)
                .insert(Collider::default()) // works fine!
                .insert(ExternalForce {
                    force: Vec2::ZERO,
                    torque: 0.0,
                })
                .insert(Damping {
                    linear_damping: 0.6,
                    angular_damping: 5.0,
                })
                .insert(Restitution::coefficient(0.8))
                .insert(GravityScale(0.0));
        });

    //entity
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(-150.0, 0.0, 0.0),
            texture: assets_server.load(PLAYER_PNG),
            ..Default::default()
        })
        .insert(InputManagerBundle {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(VirtualDPad::wasd(), Action::Move)
                .build(),
        })
        // physics from rapier2d
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(32.0))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(Damping {
            linear_damping: 0.6,
            angular_damping: 5.0,
        })
        .insert(Restitution::coefficient(0.8))
        .insert(Player);
}

fn moveit(
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
    time: Res<Time>,
) {
    for (a_state, mut ext_force) in &mut query {
        let axis_vector = a_state.clamped_axis_pair(Action::Move).unwrap().xy();

        ext_force.force = axis_vector * MOVE_FORCE * time.delta_seconds();
    }
}
