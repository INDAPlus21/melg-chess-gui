use eliasfl_chess::{Color as Colour, Game, GameState, Piece as PieceType, Position};
use ggez::event;
use ggez::event::MouseButton;
use ggez::graphics::{self, Color, DrawMode, DrawParam};
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::path;

/// A chess board is 8x8 tiles.
const GRID_SIZE: (i16, i16) = (8, 8);
/// Suitible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (45, 45);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32 + 300 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32 + 300 as f32,
);

// GUI Color representations
const BLACK: Color = Color::new(228.0 / 255.0, 196.0 / 255.0, 108.0 / 255.0, 1.0);
const WHITE: Color = Color::new(188.0 / 255.0, 140.0 / 255.0, 76.0 / 255.0, 1.0);
const SELECTED_TILE: Color = Color::new(209.0 / 255.0, 161.0 / 255.0, 29.0 / 255.0, 1.0);
const MOVABLE_TILE: Color = Color::new(209.0 / 255.0, 62.0 / 255.0, 29.0 / 255.0, 1.0);

/// GUI logic and event implementation structure.
struct AppState {
    sprites: HashMap<PieceType, graphics::Image>,
    board: Game,
    selected_tile: Option<Position>,
    highlighted_tiles: Vec<Position>,
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let sprites = AppState::load_sprites();
        let mut loaded_sprites: HashMap<PieceType, graphics::Image> = Default::default();
        let board = Game::new();

        // Load sprites from string
        for _sprite in sprites.iter() {
            loaded_sprites.insert(
                *_sprite.0,
                graphics::Image::new(ctx, _sprite.1.clone()).unwrap(),
            );
        }

        let state = AppState {
            sprites: loaded_sprites,
            board,
            selected_tile: None,
            highlighted_tiles: Default::default(),
        };

        Ok(state)
    }

    /// Loads chess piece images into hashmap.
    fn load_sprites() -> HashMap<PieceType, String> {
        let mut sprites = HashMap::new();
        sprites.insert(
            PieceType::King(Colour::Black),
            "/black_king.png".to_string(),
        );
        sprites.insert(
            PieceType::Queen(Colour::Black),
            "/black_queen.png".to_string(),
        );
        sprites.insert(
            PieceType::Rook(Colour::Black),
            "/black_rook.png".to_string(),
        );
        sprites.insert(
            PieceType::Pawn(Colour::Black),
            "/black_pawn.png".to_string(),
        );
        sprites.insert(
            PieceType::Bishop(Colour::Black),
            "/black_bishop.png".to_string(),
        );
        sprites.insert(
            PieceType::Knight(Colour::Black),
            "/black_knight.png".to_string(),
        );
        sprites.insert(
            PieceType::King(Colour::White),
            "/white_king.png".to_string(),
        );
        sprites.insert(
            PieceType::Queen(Colour::White),
            "/white_queen.png".to_string(),
        );
        sprites.insert(
            PieceType::Rook(Colour::White),
            "/white_rook.png".to_string(),
        );
        sprites.insert(
            PieceType::Pawn(Colour::White),
            "/white_pawn.png".to_string(),
        );
        sprites.insert(
            PieceType::Bishop(Colour::White),
            "/white_bishop.png".to_string(),
        );
        sprites.insert(
            PieceType::Knight(Colour::White),
            "/white_knight.png".to_string(),
        );
        sprites
    }
}

/// Implement each stage of the application event loop.
impl event::EventHandler<ggez::GameError> for AppState {
    /// For updating game logic, which front-end doesn't handle.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw window
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear interface with gray background colour
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        // Create text representation
        let state_text = graphics::Text::new(
            graphics::TextFragment::from(format!("Game is {:?}.", self.board.get_game_state()))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        // get size of text
        let text_dimensions = state_text.dimensions(ctx);
        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect::new(
                (SCREEN_SIZE.0 - text_dimensions.x as f32) / 2f32 as f32 - 8.0,
                (SCREEN_SIZE.0 - text_dimensions.y as f32) / 2f32 as f32,
                text_dimensions.x as f32 + 16.0,
                text_dimensions.y as f32,
            ),
            [1.0, 1.0, 1.0, 1.0].into(),
        )?;

        // Draw background
        graphics::draw(ctx, &background_box, DrawParam::default());

        // Draw tiles
        for i in 0..64 {
            let position = &Position {
                file: (i % 8 + 1) as u8, // Add one as api i 1-8 instead of 0-7
                rank: (i / 8 + 1) as u8,
            };

            let colour;
            if self.selected_tile.is_some() && self.selected_tile.unwrap() == position.to_owned() {
                colour = SELECTED_TILE;
            } else if self.highlighted_tiles.contains(position) {
                colour = MOVABLE_TILE;
            } else {
                colour = match i % 2 {
                    0 => match i / 8 {
                        _row if _row % 2 == 0 => WHITE,
                        _ => BLACK,
                    },
                    _ => match i / 8 {
                        _row if _row % 2 == 0 => BLACK,
                        _ => WHITE,
                    },
                };
            };

            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new_i32(
                    i % 8 * GRID_CELL_SIZE.0 as i32,
                    i / 8 * GRID_CELL_SIZE.1 as i32,
                    GRID_CELL_SIZE.0 as i32,
                    GRID_CELL_SIZE.1 as i32,
                ),
                colour,
            )?;
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },));

            // Draw piece
            if self.board.board.contains_key(position) {
                let sprite = self
                    .sprites
                    .get(self.board.board.get(position).as_ref().unwrap())
                    .unwrap();

                graphics::draw(
                    ctx,
                    sprite,
                    (ggez::mint::Point2 {
                        x: ((position.file as i16 - 1) * GRID_CELL_SIZE.0) as f32, // Remove one as api i 1-8 instead of 0-7
                        y: ((position.rank as i16 - 1) * GRID_CELL_SIZE.1) as f32,
                    },),
                );
            }
        }

        // draw text with dark gray colouring and center position
        graphics::draw(
            ctx,
            &state_text,
            DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: (SCREEN_SIZE.0 - text_dimensions.x as f32) / 2f32 as f32,
                    y: (SCREEN_SIZE.0 - text_dimensions.y as f32) / 2f32 as f32,
                }),
        );

        // Render updated graphics
        graphics::present(ctx)?;

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            let x_tile = (x / GRID_CELL_SIZE.0 as f32) as i32;
            let y_tile = (y / GRID_CELL_SIZE.1 as f32) as i32;

            // Inside board
            if x_tile < 8 && y_tile < 8 {
                let position = &Position {
                    file: (x_tile + 1) as u8,
                    rank: (y_tile + 1) as u8,
                };
                if self.board.board.contains_key(position) {
                    let piece = self.board.board.get(position).unwrap();

                    // Select piece to move
                    if get_colour_from_piece(piece.to_owned()) == self.board.active_color {
                        self.selected_tile = Some(position.to_owned());
                        let moves = self.board.get_possible_moves(position.to_string());

                        if moves.is_none() {
                            self.highlighted_tiles = Default::default();
                        } else {
                            let mut position_moves: Vec<Position> = Default::default();

                            // Convert from strings to positions
                            for _move in moves.unwrap().iter() {
                                position_moves
                                    .push(Position::from_string(_move.to_owned()).unwrap());
                            }
                            self.highlighted_tiles = position_moves;
                        }
                    } else {
                        // Attack move
                        move_to_tile(self, position);
                    }
                } else {
                    // Passive move
                    move_to_tile(self, position);
                }
            }
        } else if button == MouseButton::Right {
            // Unselect piece
            self.selected_tile = None;
            self.highlighted_tiles = Default::default();
        }
    }
}

fn move_to_tile(state: &mut AppState, position: &Position) {
    if state.highlighted_tiles.contains(position) {
        state.board.make_move(
            state.selected_tile.unwrap().to_string(),
            position.to_string(),
        );

        // Deselect tile
        state.selected_tile = None;
        state.highlighted_tiles = Default::default();
    }
}

// Elias why didn't you make the Piece::color method public!?
fn get_colour_from_piece(piece: PieceType) -> Colour {
    use Colour::*;
    use PieceType::*;
    match piece {
        King(White) | Queen(White) | Rook(White) | Bishop(White) | Knight(White) | Pawn(White) => {
            White
        }
        _ => Black,
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ggez::ContextBuilder::new("schack", "eskil")
        .add_resource_path(resource_dir) // Import image files to GGEZ
        .window_setup(
            ggez::conf::WindowSetup::default().title("Schack"), // Set window title "Schack"
                                                                //.icon("/icon.ico"), // Set application icon
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimenstions
                .resizable(false), // Fixate window size
        );
    let (mut context, event_loop) = context_builder.build()?;

    let state = AppState::new(&mut context)?;
    event::run(context, event_loop, state) // Run window event loop
}
