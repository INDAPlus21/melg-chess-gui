use eliasfl_chess::{Color as Colour, Game, GameState, Piece as PieceType, Position};
use ggez::event::MouseButton;
use ggez::graphics::{self, Color, DrawParam};
use ggez::{event, timer};
use ggez::{Context, GameResult};
use std::collections::HashMap;
use std::path;

/// A chess board is 8x8 tiles.
const GRID_SIZE: (i16, i16) = (8, 8);
/// Suitible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (45, 45);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    (GRID_SIZE.0 + 10) as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: Color = Color::new(228.0 / 255.0, 196.0 / 255.0, 108.0 / 255.0, 1.0);
const WHITE: Color = Color::new(188.0 / 255.0, 140.0 / 255.0, 76.0 / 255.0, 1.0);
const SELECTED_TILE: Color = Color::new(209.0 / 255.0, 161.0 / 255.0, 29.0 / 255.0, 1.0);
const MOVABLE_TILE: Color = Color::new(209.0 / 255.0, 62.0 / 255.0, 29.0 / 255.0, 1.0);

/// GUI logic and event implementation structure.
struct AppState {
    sprites: HashMap<PieceType, graphics::Image>,
    game: Game,
    selected_tile: Option<Position>,
    highlighted_tiles: Vec<Position>,
    white_time: f32,
    black_time: f32,
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let sprites = AppState::load_sprites();
        let mut loaded_sprites: HashMap<PieceType, graphics::Image> = Default::default();
        let game = Game::new();

        // Load sprites from string
        for _sprite in sprites.iter() {
            loaded_sprites.insert(
                *_sprite.0,
                graphics::Image::new(ctx, _sprite.1.clone()).unwrap(),
            );
        }

        let state = AppState {
            sprites: loaded_sprites,
            game,
            selected_tile: None,
            highlighted_tiles: Default::default(),
            white_time: 10.0 * 60.0, // 10 minutes
            black_time: 10.0 * 60.0,
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
        // Decrease time
        if self.white_time > 0.0 && self.black_time > 0.0 {
            match self.game.active_color {
                Colour::White => self.white_time -= timer::delta(_ctx).as_secs_f32(),
                Colour::Black => self.black_time -= timer::delta(_ctx).as_secs_f32(),
            }

            // Prevent negative time
            self.white_time = self.white_time.max(0.0);
            self.black_time = self.black_time.max(0.0);
        }

        Ok(())
    }

    /// Draw window
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear interface with gray background colour
        graphics::clear(ctx, Color::BLUE);

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
            graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;

            // Draw piece
            if self.game.board.contains_key(position) {
                let sprite = self
                    .sprites
                    .get(self.game.board.get(position).as_ref().unwrap())
                    .unwrap();

                graphics::draw(
                    ctx,
                    sprite,
                    (ggez::mint::Point2 {
                        x: ((position.file as i16 - 1) * GRID_CELL_SIZE.0) as f32, // Remove one as api i 1-8 instead of 0-7
                        y: ((position.rank as i16 - 1) * GRID_CELL_SIZE.1) as f32,
                    },),
                )?;
            }
        }

        // Draw reset text
        let reset_text = graphics::Text::new(
            graphics::TextFragment::from("Reset").scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        graphics::draw(
            ctx,
            &reset_text,
            DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: (GRID_CELL_SIZE.0 * 8 + 10) as f32,
                    y: 10f32,
                }),
        )?;

        // Draw turn text
        let turn_text = graphics::Text::new(
            graphics::TextFragment::from(format!("{:?}'s turn", self.game.active_color))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        graphics::draw(
            ctx,
            &turn_text,
            DrawParam::default()
                .color(match self.game.active_color {
                    Colour::White => Color::WHITE,
                    Colour::Black => Color::BLACK,
                })
                .dest(ggez::mint::Point2 {
                    x: (GRID_CELL_SIZE.0 * 8 + 10) as f32,
                    y: (GRID_CELL_SIZE.1 + 10) as f32,
                }),
        )?;

        // Promotion
        let promotion_text = graphics::Text::new(
            graphics::TextFragment::from("Choose piece to promote to:")
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        graphics::draw(
            ctx,
            &promotion_text,
            DrawParam::default()
                .color([0.0, 0.0, 0.0, 1.0].into())
                .dest(ggez::mint::Point2 {
                    x: (GRID_CELL_SIZE.0 * 8 + 10) as f32,
                    y: (GRID_CELL_SIZE.0 * 2 + 10) as f32,
                }),
        )?;

        draw_promotion_icons(self, ctx);

        // Draw win text
        if self.game.get_game_state() == GameState::CheckMate {
            let win_text = graphics::Text::new(
                graphics::TextFragment::from(format!("{:?} has won!", self.game.active_color))
                    .scale(graphics::PxScale { x: 60.0, y: 60.0 }),
            );

            graphics::draw(
                ctx,
                &win_text,
                DrawParam::default()
                    .color(match self.game.active_color {
                        Colour::White => Color::WHITE,
                        Colour::Black => Color::BLACK,
                    })
                    .dest(ggez::mint::Point2 {
                        x: (GRID_CELL_SIZE.0 * 2) as f32,
                        y: (GRID_CELL_SIZE.1) as f32 * 3.5,
                    }),
            )?;
        }

        // Time text
        let turn_text = graphics::Text::new(
            graphics::TextFragment::from(format!(
                "Time left: {}",
                parse_time(match self.game.active_color {
                    Colour::White => self.white_time,
                    Colour::Black => self.black_time,
                })
            ))
            .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
        );

        graphics::draw(
            ctx,
            &turn_text,
            DrawParam::default()
                .color(match self.game.active_color {
                    Colour::White => Color::WHITE,
                    Colour::Black => Color::BLACK,
                })
                .dest(ggez::mint::Point2 {
                    x: (GRID_CELL_SIZE.0 * 8 + 10) as f32,
                    y: (GRID_CELL_SIZE.1 * 4 + 10) as f32,
                }),
        )?;

        // Draw time over text
        if self.white_time == 0.0 || self.black_time == 0.0 {
            let time_over_text = graphics::Text::new(
                graphics::TextFragment::from(format!(
                    "{:?} has won as the time ran out!",
                    !self.game.active_color
                ))
                .scale(graphics::PxScale { x: 30.0, y: 30.0 }),
            );

            graphics::draw(
                ctx,
                &time_over_text,
                DrawParam::default()
                    .color(match self.game.active_color {
                        Colour::White => Color::WHITE,
                        Colour::Black => Color::BLACK,
                    })
                    .dest(ggez::mint::Point2 {
                        x: 10.0,
                        y: (GRID_CELL_SIZE.1) as f32 * 3.5,
                    }),
            )?;
        }

        // Render updated graphics
        graphics::present(ctx)?;

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            let x_tile = (x / GRID_CELL_SIZE.0 as f32) as i32;
            let y_tile = (y / GRID_CELL_SIZE.1 as f32) as i32;

            // Inside board
            if x_tile < 8 && y_tile < 8 {
                let position = &Position {
                    file: (x_tile + 1) as u8,
                    rank: (y_tile + 1) as u8,
                };
                if self.game.board.contains_key(position) {
                    let piece = self.game.board.get(position).unwrap();

                    // Select piece to move
                    if get_colour_from_piece(piece.to_owned()) == self.game.active_color {
                        self.selected_tile = Some(position.to_owned());
                        let moves = self.game.get_possible_moves(position.to_string());

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
            } else if x_tile < 10 && y_tile < 1 {
                // Click reset button
                self.game = Game::new();
                self.selected_tile = None;
                self.highlighted_tiles = Default::default();
                self.white_time = 10.0 * 60.0;
                self.black_time = 10.0 * 60.0;
            } else if y_tile == 3 {
                // Select promotion
                let selected_piece = match x_tile {
                    8 => Some(PieceType::Queen(self.game.active_color)),
                    9 => Some(PieceType::Knight(self.game.active_color)),
                    10 => Some(PieceType::Rook(self.game.active_color)),
                    11 => Some(PieceType::Bishop(self.game.active_color)),
                    _ => None,
                };

                if selected_piece.is_some() {
                    self.game.promotion[match self.game.active_color {
                        Colour::White => 0,
                        Colour::Black => 1,
                    }] = selected_piece.unwrap();
                }
            }
        } else if button == MouseButton::Right {
            // Unselect piece
            self.selected_tile = None;
            self.highlighted_tiles = Default::default();
        }
    }
}

// Parses time from seconds to MM:SS:MSMS
fn parse_time(time: f32) -> String {
    let minutes = (time / 60.0).floor();
    let seconds = (time - minutes * 60.0).floor();
    let milliseconds = ((time - minutes * 60.0 - seconds) * 60.0).round();

    // Add 0 if less than 10
    let mut second_string = seconds.to_string();
    if second_string.len() == 1 {
        second_string = format!("0{}", second_string);
    }

    let mut millisecond_string = milliseconds.to_string();
    if millisecond_string.len() == 1 {
        millisecond_string = format!("0{}", millisecond_string);
    }

    return format!("{}:{}:{}", minutes, second_string, millisecond_string);
}

fn draw_promotion_icons(state: &mut AppState, ctx: &mut Context) {
    let selected_colour = state.game.active_color;
    let selected_promotion = match selected_colour {
        Colour::White => state.game.promotion[0],
        Colour::Black => state.game.promotion[1],
    };

    draw_promotion_icon(
        state,
        ctx,
        PieceType::Queen,
        0,
        selected_promotion == PieceType::Queen(selected_colour),
    );
    draw_promotion_icon(
        state,
        ctx,
        PieceType::Knight,
        1,
        selected_promotion == PieceType::Knight(selected_colour),
    );
    draw_promotion_icon(
        state,
        ctx,
        PieceType::Rook,
        2,
        selected_promotion == PieceType::Rook(selected_colour),
    );
    draw_promotion_icon(
        state,
        ctx,
        PieceType::Bishop,
        3,
        selected_promotion == PieceType::Bishop(selected_colour),
    );
}

fn draw_promotion_icon(
    state: &mut AppState,
    ctx: &mut Context,
    piece: fn(eliasfl_chess::Color) -> PieceType,
    x: i16,
    selected: bool,
) {
    // Draw background to show that piece is selected
    if selected {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new_i32(
                ((GRID_CELL_SIZE.0) * (8 + x)) as i32, // Remove one as api i 1-8 instead of 0-7
                (GRID_CELL_SIZE.1 * 3) as i32,
                GRID_CELL_SIZE.0 as i32,
                GRID_CELL_SIZE.1 as i32,
            ),
            SELECTED_TILE,
        )
        .unwrap();
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },)).unwrap();
    }

    // Draw piece
    graphics::draw(
        ctx,
        state.sprites.get(&piece(state.game.active_color)).unwrap(),
        (ggez::mint::Point2 {
            x: ((GRID_CELL_SIZE.0) * (8 + x)) as f32, // Remove one as api i 1-8 instead of 0-7
            y: (GRID_CELL_SIZE.1 * 3) as f32,
        },),
    )
    .unwrap();
}

fn move_to_tile(state: &mut AppState, position: &Position) {
    // Prevent moving when time is over
    if state.white_time == 0.0 || state.black_time == 0.0 {
        return;
    }

    if state.highlighted_tiles.contains(position) {
        state
            .game
            .make_move(
                state.selected_tile.unwrap().to_string(),
                position.to_string(),
            )
            .unwrap();

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

    let context_builder = ggez::ContextBuilder::new("chess", "marcus")
        .add_resource_path(resource_dir) // Import image files to GGEZ
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Chess") // Set window title
                .icon("/icon.png"), // Set application icon
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
