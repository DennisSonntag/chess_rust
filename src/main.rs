use bevy::prelude::*;
use std::collections::HashMap;

const WINDOW_SIZE: f32 = 600.0;
const SQUARE_SIZE: f32 = WINDOW_SIZE / 8.;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pieces {
	None,
	King,
	Queen,
	Bishop,
	Knight,
	Rook,
	Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PieceColor {
	White,
	Black,
	None,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Piece {
	row: u8,
	col: u8,
	selected: bool,
	piece: Pieces,
	color: PieceColor,
}

#[derive(Component)]
pub struct SelectedSquare {
	row: u8,
	col: u8,
}

#[derive(Component)]
pub struct MoveSquare {
	row: u8,
	col: u8,
}

#[derive(Resource, Debug)]
struct BoardResource {
	board: [Piece; 64],
}

impl FromWorld for BoardResource {
	fn from_world(_: &mut World) -> Self {
		let board = load_position_from_fen(String::from(
			"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
		));

		BoardResource { board }
	}
}

#[derive(Resource, Debug)]
struct SelectedResource {
	selected: bool,
	index: u8,
}

impl FromWorld for SelectedResource {
	fn from_world(_: &mut World) -> Self {
		SelectedResource {
			selected: false,
			index: 0,
		}
	}
}

fn load_position_from_fen(fen: String) -> [Piece; 64] {
	let mut board: [Piece; 64] = [Piece {
		piece: Pieces::None,
		color: PieceColor::None,
		selected: false,
		row: 0,
		col: 0,
	}; 64];

	let mut piece_type_from_symbol: HashMap<char, Pieces> = HashMap::new();
	piece_type_from_symbol.insert('k', Pieces::King);
	piece_type_from_symbol.insert('p', Pieces::Pawn);
	piece_type_from_symbol.insert('n', Pieces::Knight);
	piece_type_from_symbol.insert('b', Pieces::Bishop);
	piece_type_from_symbol.insert('r', Pieces::Rook);
	piece_type_from_symbol.insert('q', Pieces::Queen);

	let fen_data: Vec<&str> = fen.split(" ").collect();
	let fen_board: Vec<&str> = fen_data[0].split("/").collect();

	let mut col: i32 = 0;
	let mut row: i32 = 8;

	for row_data in fen_board {
		col = -1;
		row -= 1;
		for i in row_data.chars() {
			if i.is_digit(10) {
				col += i as i32;
				if col >= 7 {
					continue;
				}
			} else {
				col += 1;
			}
			let piece_color = if i.is_uppercase() {
				PieceColor::White
			} else if i.is_lowercase() {
				PieceColor::Black
			} else {
				PieceColor::None
			};

			let lower_char = &i.to_lowercase().to_string().chars().next().unwrap();
			let mut piece_type = Pieces::None;
			if piece_type_from_symbol.contains_key(lower_char) {
				piece_type = *piece_type_from_symbol.get(lower_char).unwrap();
			}
			board[(row * 8 + col) as usize] = Piece {
				piece: piece_type,
				color: piece_color,
				selected: false,
				row: row as u8,
				col: col as u8,
			}
		}
	}

	return board;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
	White,
	Black,
}

struct MoveEvent;

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
		.add_startup_system(first_draw_board)
		.insert_resource(Msaa { samples: 4 })
		.add_event::<MoveEvent>()
		.init_resource::<BoardResource>()
		.init_resource::<SelectedResource>()
		.add_state(TurnState::White)
		.add_system(select_piece)
		.add_system(move_piece)
		.add_system(draw_board)
		.run();
}

fn setup_camera(mut commands: Commands) {
	commands.spawn(Camera2dBundle::default());
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
		return Piece {
			piece,
			color: PieceColor::Black,
			selected: false,
			row: 0,
			col: 0,
		};
	} else {
		return Piece {
			piece,
			color: PieceColor::White,
			selected: false,
			row: 0,
			col: 0,
		};
	}
}

fn first_draw_board(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	board: ResMut<BoardResource>,
) {
	let texture_handle = asset_server.load("pieces.png");

	for (i, el) in board.board.iter().enumerate() {
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
					(el.color as i32) as f32 * 333.3,
				)),
			);
			let texture_atlas_handle = texture_atlases.add(texture_atlas);

			commands
				.spawn(SpriteSheetBundle {
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
				})
				.insert(Piece {
					piece: el.piece,
					color: el.color,
					selected: false,
					row: row as u8,
					col: col as u8,
				});
		}
	}
}

fn draw_board(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut pieces: Query<(Entity, &mut Piece, &mut Transform)>,
	board: ResMut<BoardResource>,
) {
	let texture_handle = asset_server.load("pieces.png");

	for (entity, _, _) in &mut pieces {
		commands.entity(entity).despawn_recursive();
	}

	for (i, el) in board.board.iter().enumerate() {
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
					(el.color as i32) as f32 * 333.3,
				)),
			);
			let texture_atlas_handle = texture_atlases.add(texture_atlas);

			commands
				.spawn(SpriteSheetBundle {
					texture_atlas: texture_atlas_handle,
					transform: Transform {
						translation: Vec3::new(
							//x
							-((WINDOW_SIZE / 2.) - (col as f32 * SQUARE_SIZE) - (SQUARE_SIZE / 2.)),
							//y
							-((WINDOW_SIZE / 2.) - (row as f32 * SQUARE_SIZE) - (SQUARE_SIZE / 2.)),
							0.0,
						),
						scale: Vec3::splat(WINDOW_SIZE / 2500.),
						..default()
					},

					..default()
				})
				.insert(Piece {
					piece: el.piece,
					color: el.color,
					selected: false,
					row: row as u8,
					col: col as u8,
				});
		}
	}
}

fn highlight_square(
	selected_squares: &mut Query<(Entity, &mut SelectedSquare)>,
	commands: &mut Commands,
	board: &[Piece; 64],
) {
	for (entity, _) in selected_squares {
		commands.entity(entity).despawn_recursive();
	}

	for (i, el) in board.iter().enumerate() {
		if el.selected == true {
			let row = (i / 8) as u8;
			let col = (i % 8) as u8;
			commands
				.spawn((SpriteBundle {
					transform: Transform {
						translation: Vec3::new(
							-(WINDOW_SIZE / 2.) + (((col + 1) as f32 - 0.5) as f32 * SQUARE_SIZE),
							-(WINDOW_SIZE / 2.) + (((row + 1) as f32 - 0.5) as f32 * SQUARE_SIZE),
							0.0,
						),
						scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, 0.0),
						..default()
					},
					sprite: Sprite {
						color: Color::rgba_u8(190, 147, 85, 150),
						..default()
					},
					..default()
				},))
				.insert(SelectedSquare { row, col });
		}
	}
}

fn select_piece(
	buttons: Res<Input<MouseButton>>,
	windows: Res<Windows>,
	mut commands: Commands,
	board: ResMut<BoardResource>,
	mut selected_squares: Query<(Entity, &mut SelectedSquare)>,
	mut moved_squares: Query<(Entity, &mut MoveSquare)>,
	turn_sate: Res<State<TurnState>>,
) {
	let window = windows.get_primary().unwrap();

	if let Some(_position) = window.cursor_position() {
		if buttons.just_pressed(MouseButton::Left) {
			let col = ((_position[0] / 75.).floor()) as u8;
			let row = ((_position[1] / 75.).floor()) as u8;

			let index = (row * 8 + col) as usize;
			// println!("{:?}", turn_sate.current());

			let mut new_board = board.board;

			if new_board[index].selected == false && new_board[index].piece != Pieces::None {
				if (turn_sate.current() == &TurnState::White
					&& new_board[index].color == PieceColor::White)
					|| (turn_sate.current() == &TurnState::Black
						&& new_board[index].color == PieceColor::Black)
				{
					for i in 0..63 {
						new_board[i].selected = false;
					}
					new_board[index].selected = true;
					commands.remove_resource::<SelectedResource>();
					commands.insert_resource(SelectedResource {
						selected: true,
						index: index as u8,
					});
					highlight_square(&mut selected_squares, &mut commands, &new_board);
					for (entity, _) in &mut moved_squares {
						commands.entity(entity).despawn_recursive();
					}
				} else {
					for (entity, _) in &mut moved_squares {
						commands.entity(entity).despawn_recursive();
					}
				}
			} else if new_board[index].selected == true {
				new_board[index].selected = false;
				commands.remove_resource::<SelectedResource>();
				commands.insert_resource(SelectedResource {
					selected: false,
					index: index as u8,
				});
				for (entity, _) in &mut moved_squares {
					commands.entity(entity).despawn_recursive();
				}
			} else if new_board[index].piece == Pieces::None {
				for i in 0..63 {
					new_board[i].selected = false;
				}
				commands.remove_resource::<SelectedResource>();
				commands.insert_resource(SelectedResource {
					selected: false,
					index: index as u8,
				});
				for (entity, _) in &mut moved_squares {
					commands.entity(entity).despawn_recursive();
				}
			}

			commands.remove_resource::<BoardResource>();
			commands.insert_resource(BoardResource { board: new_board });
		}
	} else {
		// cursor is not inside the window
	}
}

fn move_piece(
	buttons: Res<Input<MouseButton>>,
	windows: Res<Windows>,
	mut commands: Commands,
	is_selected: ResMut<SelectedResource>,
	board: ResMut<BoardResource>,
	mut selected_squares: Query<(Entity, &mut SelectedSquare)>,
	mut turn_state: ResMut<State<TurnState>>,
	asset_server: Res<AssetServer>,
	audio: Res<Audio>,
	mut move_writer: EventWriter<MoveEvent>,
) {
	let window = windows.get_primary().unwrap();

	if let Some(_position) = window.cursor_position() {
		if buttons.just_pressed(MouseButton::Left) {
			let col = ((_position[0] / 75.).floor()) as u8;
			let row = ((_position[1] / 75.).floor()) as u8;

			let index = (row * 8 + col) as usize;

			let mut new_board = board.board;

			for (entity, _) in &mut selected_squares {
				commands.entity(entity).despawn_recursive();
			}

			if is_selected.selected == true && new_board[index].piece == Pieces::None {
				new_board[index] = piece(
					new_board[is_selected.index as usize].piece,
					new_board[is_selected.index as usize].color,
				);
				new_board[is_selected.index as usize] = piece(Pieces::None, PieceColor::None);
				commands
					.spawn((SpriteBundle {
						transform: Transform {
							translation: Vec3::new(
								-(WINDOW_SIZE / 2.)
									+ (((col + 1) as f32 - 0.5) as f32 * SQUARE_SIZE),
								-(WINDOW_SIZE / 2.)
									+ (((row + 1) as f32 - 0.5) as f32 * SQUARE_SIZE),
								0.0,
							),
							scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, 0.0),
							..default()
						},
						sprite: Sprite {
							color: Color::rgba_u8(190, 147, 85, 150),
							..default()
						},
						..default()
					},))
					.insert(MoveSquare { row, col });
				audio.play(asset_server.load("Move.ogg"));
				if turn_state.current() == &TurnState::White {
					turn_state.set(TurnState::Black).unwrap();
				} else if turn_state.current() == &TurnState::Black {
					turn_state.set(TurnState::White).unwrap();
				}
			} else {
				// println!("Not");
			}

			commands.remove_resource::<BoardResource>();
			commands.insert_resource(BoardResource { board: new_board });

			// move_writer.send(MoveEvent);
		}
	} else {
		// cursor is not inside the window
	}
}
