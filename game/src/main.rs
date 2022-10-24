//imports from outside crates
use bevy::app::AppExit;
use bevy::sprite::collide_aabb::Collision;
use bevy::{prelude::*, window::PresentMode};
use bevy::render::camera::RenderTarget;



//imports from local creates
mod util;
use crate::util::*;

mod active_util;
use crate::active_util::*;

mod ai;
use crate::ai::*;
#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);
const START_TIME: f32=10.;

struct Manager{
    room_number: i8,
    wall_id: i8,
    enemy_id: i8,
}


fn create_level( 
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    level: Vec<Descriptor>){
        for desc in level{
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(desc.width, desc.height)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(desc.x_pos, desc.y_pos, 2.),
                    ..default()
                },
                ..default()
            })
            .insert(Object::new(0, desc.width, desc.height));
        }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: TITLE.to_string(),
            width: 1280.,
            height: 720.,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        //.add_system(show_popup)
        .add_system(apply_collisions)
        .add_system(
            move_player
                .after(setup)
                .after(show_timer)
                .before(apply_collisions)
            )
        .add_system(
            update_positions
                .after(apply_collisions)
        )
        .add_system(
            move_enemies
                .after(move_player)
                .before(apply_collisions)
        )
        .add_system(my_cursor_system)
        .add_system(show_timer)
        //.add_system(calculate_sight)
        //.add_system(attack)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let images = &[
        "jacob.png",
        "bailey.png",
        "brian.png",
        "ethan.png",
        "jack.png",
        "gio.png",
        "zach.png",
    ];
    commands.spawn_bundle(Camera2dBundle::default());
    let mut time: f32 = 0.0;
    for image in images {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(*image),
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            })
            .insert(PopupTimer(Timer::from_seconds(time, false)));
        time += 5.0;
    }

    commands.insert_resource(Clock {
        // create the repeating timer
        timer: Timer::from_seconds(15.0, true),
    });

    //This is for the overlay
    //Putting comments for every object so we know which is which. This is a bad idea for future levels but for now but it gets a basis going.
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
        texture: asset_server.load("Room_1.png"),
        transform: Transform::from_xyz(912., 500., 0.),
        ..default()
    });

    
    commands.spawn_bundle(
        TextBundle::from_section(
           "", 
            TextStyle {
                font_size: 100.0,
                color: Color::WHITE,
                font: asset_server.load("mrsmonster.ttf")
            }
        )
    )
    .insert(Style {
        align_self: AlignSelf::FlexEnd,
        position_type: PositionType::Absolute,
        position: UiRect {
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        },
        ..default()
    })
    .insert(ClockText);

    //Player(spawns slightly above origin now, starting tile of map centered on origin.)
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(PLAYER_SZ, PLAYER_SZ)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 64., 900.),
                ..default()
            },
            ..default()
        })
        .insert(ActiveObject::new(100, 25))
        .insert(Player);
    

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(PLAYER_SZ, PLAYER_SZ)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(75., 100., 700.),
                ..default()
            },
            ..default()
        })
        .insert(ActiveObject::new(100, 25))
        .insert(Enemy);

        commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(PLAYER_SZ, PLAYER_SZ)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(75., 500., 700.),
                ..default()
            },
            ..default()
        })
        .insert(ActiveObject::new(100, 25))
        .insert(Enemy);
    
        commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(PLAYER_SZ, PLAYER_SZ)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(500., 100., 700.),
                ..default()
            },
            ..default()
        })
        .insert(ActiveObject::new(100, 25))
        .insert(Enemy);
    //improved code to spawn in all walls of a level
    let mut level = get_level(1);
    create_level(commands, asset_server, texture_atlases, level);


}

fn show_popup(time: Res<Time>, mut popup: Query<(&mut PopupTimer, &mut Transform)>) {
    let mut count = 1.0;
    for (mut timer, mut transform) in popup.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            transform.translation.z = count;
        }
        count += 1.0;
    }
}

fn calculate_sight(
    time: Res<Time>,
    player: Query<&Transform, With<Player>>,
    objects: Query<(&Object, &Transform), With<Object>>,
    input: Res<Input<KeyCode>>,
) {
    //TODO: make a struct for all of the sight lines for a given object
    // hold a Vec containing lines
    // hold a reference to the object
    // loop through each of these when doing checks
    let origin = player.single();
    let x_pos = origin.translation.x;
    let y_pos = origin.translation.y;

    if input.pressed(KeyCode::Q) {
        let sight_distance = 300.0;
        let mut sight_lines = Vec::new();
        let mut object_lines = Vec::new();

        for (o, t) in objects.iter() {
            //v1 and v2 hold the endpoints for line of sight
            let v1: Vec2;
            let v2: Vec2;
            //v3 is the third point for the two sides to be used for collision
            let v3: Vec2;

            if x_pos > t.translation.x {
                if y_pos >= t.translation.y {
                    //top left point
                    v1 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                    //bottom right point
                    v2 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y - o.height / 2.,
                    );
                    //top right point
                    v3 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                } else {
                    //top right point
                    v1 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                    //bottom left point
                    v2 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y - o.height / 2.,
                    );
                    //bottom right point
                    v3 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y - o.height / 2.,
                    );
                }
            //MAYBE code for when y's are equal
            } else {
                if y_pos > t.translation.y {
                    //top right point
                    v1 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                    //bottom left point
                    v2 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y - o.height / 2.,
                    );
                    //top left point
                    v3 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                } else {
                    //top left point
                    v1 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y + o.height / 2.,
                    );
                    //bottom right point
                    v2 = Vec2::new(
                        t.translation.x + o.width / 2.,
                        t.translation.y - o.height / 2.,
                    );
                    //bottom left point
                    v3 = Vec2::new(
                        t.translation.x - o.width / 2.,
                        t.translation.y - o.height / 2.,
                    );
                }
                //MAYBE code for when y's are equal
            }
            //MAYBE code for when x's are equal

            //generate lines of sight
            let s1 = Line::new(Vec2::new(x_pos, y_pos), v1, o.id);
            let s2 = Line::new(Vec2::new(x_pos, y_pos), v2, o.id);
            //MAYBE third line of sight to corner

            //track whether these are in range
            let mut in_range = false;
            if s1.length_squared() < sight_distance * sight_distance {
                sight_lines.push(s1);
                in_range = true;
            }
            if s2.length_squared() < sight_distance * sight_distance {
                sight_lines.push(s2);
                in_range = true;
            }
            if in_range {
                let o1 = Line::new(v1, v3, o.id);
                let o2 = Line::new(v2, v3, o.id);
                object_lines.push(o1);
                object_lines.push(o2);
            }
        }
        determine_visibility(sight_lines, object_lines);
    }
}
//we will also need to implement collisions between 2 active objects, that is where we will do rigidbody collisions
//I'm not sure whether that should run before or after object collisions
fn apply_collisions(
    mut actives: Query<(&mut ActiveObject,&Transform), With<ActiveObject>>,
    objects: Query<(&Object, &Transform), With<Object>>,
    //will want to use something different later
    mut exit: EventWriter<AppExit>,
){
    //loop through all objects that move
    for (mut active, transform) in actives.iter_mut(){

        for (o, t) in objects.iter() {
            let res = bevy::sprite::collide_aabb::collide(
                active.projected_position,
                //need to change this to get the size of whatever the object is
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
                t.translation,
                Vec2::new(o.width, o.height),
            );
            if res.is_some() {
                let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
                match coll_type {
                    Collision::Left => {
                        active.velocity.x = 0.;
                        active.projected_position.x = t.translation.x - (o.width / 2.) - PLAYER_SZ / 2.;
                    }
                    Collision::Right => {
                        active.velocity.x = 0.;
                        active.projected_position.x = t.translation.x + (o.width / 2.) + PLAYER_SZ / 2.;
                    }
                    Collision::Top => {
                        if active.velocity.y < 0. {
                            //if falling down
                            active.velocity.y = 0.; //stop vertical velocity
                            active.grounded = true;
                        }
                        active.projected_position.y = t.translation.y + (o.height / 2.) + PLAYER_SZ / 2.;
                        if o.id == 1 {
                            //deal damage if the collision is with a spike
                            exit.send(AppExit);
                        }
                    }
                    Collision::Bottom => {
                        active.velocity.y = 0.;
                        active.projected_position.y = t.translation.y - (o.height / 2.) - PLAYER_SZ / 2.;
                    }
                    Collision::Inside => {
                        println!("NEED TO DETERMINE HOW TO DEAL WITH THIS");
                        active.velocity = Vec2::new(0., 0.);
                    }
                }
            }
        }  
    }
}
//used for debugging and finding tile coordinates, nothing else. Player start tile is considered (0,0) for sanity.
fn my_cursor_system(
    mouse_input: Res<Input<MouseButton>>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        if mouse_input.just_pressed(MouseButton::Left) {
        eprintln!("World coords: {}/{}", (world_pos.x/32.).round(), ((world_pos.y/32.) - 1.).round());
        }
    }
}

fn update_positions(
    mut actives: Query<(&ActiveObject, &mut Transform), (With<ActiveObject>, Without<Player>)>,
    mut player: Query<(&ActiveObject, &mut Transform),(With<Player>, Without<Object>),>,
    mut cam: Query<&mut Transform, (With<Camera>, Without<Object>, Without<ActiveObject>)>,
){
    //update position of active objects based on projected position from apply_collisions()
    for (o, mut t) in actives.iter_mut(){
        t.translation = o.projected_position;
    }
    //update player position and camera position
    let (mut pl, mut pt) = player.single_mut();
    let mut camera = cam.single_mut();
    pt.translation = pl.projected_position;

    camera.translation.x = pt.translation.x;
    camera.translation.y = pt.translation.y;

}
//temporary code, should just apply gravity until they hit the ground, for now, enemies jump with j
//eventually, enemy movement decisions can be implemented in a separate file, their results will determine which action they take
//ex. for enemy in enemies, 1. calc sight 2. make decision on where to go 3. execute one of the select motion commands
fn move_enemies(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut enemies: Query<
        (&mut ActiveObject, &mut Transform),
        (With<Enemy>, Without<Object>),
    >,
){
    let deltat = time.delta_seconds();
    for (mut enemy, mut et) in enemies.iter_mut(){
        
        let mut change = Vec2::splat(0.);
        if input.pressed(KeyCode::J) && enemy.grounded {
            enemy.velocity.y = 8.;  
            change.y = 8.;
        }
        //if the palyer did not just jump, add gravity to move them downward (collision for gounded found later)
        else{
            enemy.velocity.y += GRAVITY* deltat;
            change.y = enemy.velocity.y;
        }
        //this holds the position the player will end up in if there is no collision 
        enemy.projected_position = et.translation + Vec3::new(change.x, change.y, 0.);
        enemy.grounded = false;
    }
}

fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player: Query<
        (&mut ActiveObject, &mut Transform),
        (With<Player>, Without<Object>),
    >,
) {
    let (mut pl, mut pt) = player.single_mut();

    if input.pressed(KeyCode::A) {
        pl.facing_left = true;
        if pl.velocity.x > -PLAYER_SPEED {
            pl.velocity.x = pl.velocity.x - 20.;
        }
    } else if pl.velocity.x < 0. {
        pl.velocity.x = pl.velocity.x + 20.;
    }

    if input.pressed(KeyCode::D) {
        pl.facing_left = false;
        if pl.velocity.x < PLAYER_SPEED {
            pl.velocity.x = pl.velocity.x + 20.;
        }
    } else if pl.velocity.x > 0. {
        pl.velocity.x = pl.velocity.x - 20.;
    }
    
    let deltat = time.delta_seconds();
    let mut change = Vec2::splat(0.);
    change.x = pl.velocity.x * deltat;
    //the reason that jump height was inconsistent was because this could only happen when on the ground, 
    //and it was multiplied by deltat, so faster framerate meant shorter jump
    //this code does fix the issue, but might create a new one (yay...)
    if input.pressed(KeyCode::Space) && pl.grounded {
        pl.velocity.y = 8.;  
        change.y = 8.;
    }
    //if the palyer did not just jump, add gravity to move them downward (collision for gounded found later)
    else{
        pl.velocity.y += GRAVITY* deltat;
        change.y = pl.velocity.y;
    }
    //this holds the position the player will end up in if there is no collision 
    pl.projected_position = pt.translation + Vec3::new(change.x, change.y, 0.);
    pl.grounded = false;
}

/*
fn attack(
    input: Res<Input<KeyCode>>,
    mut player: Query<
        (&mut Player, &mut Transform, &mut Velocity),
        (With<Player>, Without<Object>),
    >,
    objects: Query<(&Object, &Transform), (With<Object>, Without<Player>)>,
    mut commands: Commands,
) {
    let (pl, pt, pv) = player.single_mut();
    if input.just_pressed(KeyCode::P) {
        let mut hitbox_pos;
        if input.pressed(KeyCode::S) {
            hitbox_pos = Vec3::new(pt.translation.x, pt.translation.y - PLAYER_SZ, 0.);
        } else if pv.velocity.y != 0. {
            hitbox_pos = Vec3::new(pt.translation.x, pt.translation.y + PLAYER_SZ, 0.);
        } else if !pl.facing_left {
            hitbox_pos = Vec3::new(pt.translation.x + PLAYER_SZ, pt.translation.y, 0.);
        } else {
            hitbox_pos = Vec3::new(pt.translation.x - PLAYER_SZ, pt.translation.y, 0.);
        }
        for (_o, t) in objects.iter() {
            let res = bevy::sprite::collide_aabb::collide(
                hitbox_pos,
                Vec2::new(PLAYER_SZ, PLAYER_SZ),
                t.translation,
                Vec2::new(_o.width, _o.height),
            );
            if res.is_some() {
                let coll_type: bevy::sprite::collide_aabb::Collision = res.unwrap();
                match coll_type {
                    Collision::Left => {
                        println!("Attacked object right of player");
                    }
                    Collision::Right => {
                        println!("Attacked object left of player");
                    }
                    Collision::Top => {
                        println!("Attacked object bottom of player");
                    }
                    Collision::Bottom => {
                        println!("Attacked object top of player");
                    }
                    Collision::Inside => {
                        if pt.translation.y - PLAYER_SZ / 2. >= t.translation.y + PLAYER_SZ / 2. {
                            println!("Attacked object below player");
                        } else if pt.translation.x > t.translation.x {
                            println!("Attacked object left of player");
                        } else {
                            println!("Attacked object right of player");
                        }
                    }
                }
            }
        }
    }
}
*/
fn show_timer (time: Res<Time>, mut commands: Commands, asset_server: Res<AssetServer>, mut player: Query<&mut Transform, With<Player>>, mut clock: ResMut<Clock>, mut text: Query<&mut Text, With<ClockText>>,) {
    //create_timer(commands, asset_server, time);
        clock.timer.tick(time.delta());
        let mut time_remaining = (START_TIME - clock.timer.elapsed_secs()).round();
        //println!("{}", time_remaining);
        for mut text in &mut text {
            if time_remaining > 0.0 {
                text.sections[0].value= time_remaining.to_string();
            }
        if clock.timer.finished() {
            println!("Resetting position");
            let mut pt = player.single_mut();
            pt.translation=Vec3::new(0.,64.,0.);
        }
        
        
    }
        
        
    }
