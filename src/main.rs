use bevy::prelude::*;




const WINDOW_SIZE:f32 = 600.0;
const SQUARE_SIZE: f32 = WINDOW_SIZE / 8.;

#[derive(Debug, Clone, Copy)]
enum Pieces {
    None = 0,

    King = 1,
    Queen = 2,
    Bishop = 3,
    Knight = 4,
    Rook = 5,
    Pawn = 6,
    //
    // Black = 16,
    // White = 8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PieceColor {
    White = 1,
    Black = 0,
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    piece: Pieces,
    color: i8,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Chess".to_string(),
                width: WINDOW_SIZE,
                height: WINDOW_SIZE,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_board)
        .add_startup_system(setup_board)
        .add_system(window_resize_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    println!("Window size was: {},{}", window.width(), window.height());

    // window.set_resolution(1280., 720.);
}

fn spawn_board(mut commands: Commands) {
    for row in 1..=8 {
        for col in 1..=8 {
            let color = if (row + col) % 2 == 0 {
                Color::rgb_u8(162, 110, 91)
            } else {
                Color::rgb_u8(236, 210, 185)
            };
            commands.spawn((SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        -(WINDOW_SIZE / 2.) + ((col as f32 - 0.5) as f32 * SQUARE_SIZE),
                        -(WINDOW_SIZE / 2.) + ((row as f32 - 0.5) as f32 * SQUARE_SIZE),
                        0.0,
                    ),
                    scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, 0.0),
                    ..default()
                },
                sprite: Sprite { color, ..default() },
                ..default()
            },));
        }
    }
}

fn piece(piece: Pieces, color: PieceColor) -> Piece {
    if color == PieceColor::Black {
        return Piece { piece, color: 1 }
    } else {
        return Piece { piece, color: 0 }
    }
}

fn setup_board(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut board: [Piece; 64] = [Piece {
        piece: Pieces::None,
        color: 0,
    }; 64];

    board[0] = piece(Pieces::Rook, PieceColor::White);
    board[1] = piece(Pieces::Knight, PieceColor::White);
    board[2] = piece(Pieces::Bishop, PieceColor::White);
    board[3] = piece(Pieces::Queen, PieceColor::White);
    board[4] = piece(Pieces::King, PieceColor::White);
    board[5] = piece(Pieces::Bishop, PieceColor::White);
    board[6] = piece(Pieces::Knight, PieceColor::White);
    board[7] = piece(Pieces::Rook, PieceColor::White);

    for i in 7..=15 {
        board[i] = piece(Pieces::Pawn, PieceColor::White);
    }

    for i in 48..=55 {
        board[i] = piece(Pieces::Pawn, PieceColor::Black);
    }
    board[56] = piece(Pieces::Rook, PieceColor::Black);
    board[57] = piece(Pieces::Knight, PieceColor::Black);
    board[58] = piece(Pieces::Bishop, PieceColor::Black);
    board[59] = piece(Pieces::Queen, PieceColor::Black);
    board[60] = piece(Pieces::King, PieceColor::Black);
    board[61] = piece(Pieces::Bishop, PieceColor::Black);
    board[62] = piece(Pieces::Knight, PieceColor::Black);
    board[63] = piece(Pieces::Rook, PieceColor::Black);

    let texture_handle = asset_server.load("pieces.png");

    for (i, el) in board.iter().enumerate() {
        if el.piece as i32 != 0 {
            let row = i / 8;
            let col = i % 8;

            let texture_atlas = TextureAtlas::from_grid(
                texture_handle.clone(),
                Vec2::new(333.3, 333.3),
                2,
                1,
                None,
                Some(Vec2::new(
                    (el.piece as i32 - 1) as f32 * 333.3,
                    el.color as f32 * 333.3,
                )),
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            commands.spawn((SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(
                        //x
                        -((WINDOW_SIZE / 2.) - (col as f32 * SQUARE_SIZE) - (SQUARE_SIZE / 2.)),
                        //y
                        // -(250. - (row as f32 * (SQUARE_SIZE / 2.))),
                        -((WINDOW_SIZE / 2.) - (row as f32 * SQUARE_SIZE) - (SQUARE_SIZE / 2.)),
                        0.0,
                    ),
                    scale: Vec3::splat(WINDOW_SIZE / 2500.),
                    ..default()
                },

                ..default()
            },));
        }
    }
}
