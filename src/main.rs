use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_state::<GameStates>()
        .add_loading_state(
            LoadingState::new(GameStates::AssetLoading).continue_to_state(GameStates::Next),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameStates::AssetLoading)
        .insert_resource(Msaa::Off)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_system(draw_atlas.in_schedule(OnEnter(GameStates::Next)))
        // .add_system(animate_sprite_system.run_if(in_state(GameStates::Next)))
        // .add_system(sprite_movement.run_if(in_state(GameStates::Next)))
        .add_plugins(DefaultPlugins)
        // This plugin maps inputs to an input-type agnostic action-state
        // We need to provide it with an enum which stores the possible actions a player could take
        .add_plugin(InputManagerPlugin::<Action>::default())
        // The InputMap and ActionState components will be added to any entity with the Player component
        .add_startup_system(spawn_player)
        // Read the ActionState in your systems using queries!
        .add_system(jump)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameStates {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
struct GameAssets {
    // if the sheet would have padding, we could set that with `padding_x` and `padding_y`.
    // if there's space between the top left corner of the sheet and the first sprite, we could configure that with `offset_x` and `offset_y`
    #[asset(texture_atlas(tile_size_x = 20., tile_size_y = 20., columns = 1, rows = 1))]
    #[asset(path = "monsters/wee_mons_druid_idle_r_1.png")]
    druid: Handle<TextureAtlas>,
}

fn draw_atlas(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let druid_atlas = texture_atlases
        .get(&game_assets.druid)
        .expect("Failed to find our atlas");

    commands.spawn((
        SpriteBundle {
            texture: druid_atlas.texture.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(5., 5., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        // Direction::Right,
    ));
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Run,
    Jump,
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn(InputManagerBundle::<Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::new([(KeyCode::Space, Action::Jump)]),
        })
        .insert(Player);
}

// Query for the `ActionState` component in your game logic systems!
fn jump(query: Query<&ActionState<Action>, With<Player>>) {
    let action_state = query.single();
    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(Action::Jump) {
        println!("I'm jumping!");
    }
}

// #[derive(Component)]
// struct AnimationTimer(Timer);

// fn animate_sprite_system(
//     time: Res<Time>,
//     mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
// ) {
//     for (mut timer, mut sprite) in &mut query {
//         timer.0.tick(time.delta());
//         if timer.0.finished() {
//             sprite.index = (sprite.index + 1) % 8;
//         }
//     }
// }

// The sprite is animated by changing its translation depending on the time that has passed since
// the last frame.
// fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
//     for (mut direction, mut transform) in &mut sprite_position {
//         match *direction {
//             Direction::Idle => {}
//             Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
//             Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
//             Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
//             Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
//         }
//
//         if let Direction::Idle = *direction {
//             *direction = Direction::Idle;
//         } else if transform.translation.y > 200. {
//             *direction = Direction::Down;
//         } else if transform.translation.y < -200. {
//             *direction = Direction::Up;
//         } else if transform.translation.x > 200. {
//             *direction = Direction::Left;
//         } else if transform.translation.x < -200. {
//             *direction = Direction::Right;
//         }
//     }
// }

// #[derive(Component)]
// enum Direction {
//     Idle,
//     Up,
//     Down,
//     Left,
//     Right,
// }
