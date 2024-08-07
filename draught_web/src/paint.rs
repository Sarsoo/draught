//! Components for painting board states onto HTML canvases
 
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;

use std::f64;

use crate::log;
use draughtlib::{Board, BrdIdx, PieceIterator, Game};

use draughtlib::Team::*;
use draughtlib::Strength::*;

/// Default hex colour value for white square background
const WHITE_SQUARE: &str = "#FFFFFF";
/// Default hex colour value for black square background
const BLACK_SQUARE: &str = "#000000";

/// Default hex colour value for outline of black squares
const SQUARE_OUTLINE: &str = "#9c9c9c";
/// Line width when outlining black squares as proportion of min cell dimension
const OUTLINE_WIDTH: f64 = 0.05;
/// Whether to outline black squares
const DRAW_OUTLINE: bool = true;

/// Default hex colour value for white pieces
const WHITE_PIECE: &str = "#dbdbdb";
/// Default hex colour value for black pieces
const BLACK_PIECE: &str = "#ed0000";
/// Default hex colour value for selected piece
const SELECTED_PIECE: &str = "#fffd78";

/// Default hex colour value for white piece outline
const WHITE_PIECE_OUTLINE: &str = "#9c9c9c";
/// Default hex colour value for black piece outline
const BLACK_PIECE_OUTLINE: &str = "#a60000";
/// Default hex colour value for selected piece outline
// const SELECTED_PIECE_OUTLINE: &str = "#dedc73";
const SELECTED_PIECE_OUTLINE: &str = "#d1cf45";
/// Default hex colour value for black piece outline
const KING_OUTLINE: &str = "#ffea00";
/// Whether to outline pieces
const DRAW_PIECE_OUTLINES: bool = true;
/// Line width for outlining pieces as proportion of piece radius
const PIECE_OUTLINE_PROPORTION: f64 = 0.25;
/// Proportion of square that piece fills as proportion of min cell dimension
const PIECE_PROPORTION: f64 = 0.6;

/// Used to paint boards onto HTML canvases
#[wasm_bindgen]
#[derive(Debug)]
pub struct Painter {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    selected_idx: Option<BrdIdx>,

    white_square: JsValue,
    black_square: JsValue,

    white_piece: JsValue,
    black_piece: JsValue,
    selected_piece: JsValue,

    white_piece_line: JsValue,
    black_piece_line: JsValue,
    selected_piece_line: JsValue,
    king_line: JsValue,

    piece_lines: bool,
    piece_line_proportion: f64,

    square_outline: JsValue,
    outline_width: f64,
    draw_outline: bool,

    width: u32,
    height: u32,
}

impl Painter {
    /// Get a canvas by element ID
    fn get_canvas(canvas_id: &str) -> HtmlCanvasElement {
        // JS WINDOW
        let window = match web_sys::window(){
            Some(win) => win,
            None => panic!("No Js window returned"),
        };
        // JS DOCUMENT
        let document = match window.document() {
            Some(doc) => doc,
            None => panic!("No Js window document returned"),
        };

        // CANVAS
        let canvas = match document.get_element_by_id(canvas_id) {
            Some(el) => el,
            None => panic!("No element found for {}", canvas_id),
        };
        let canvas = match canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
            Ok(el) => el,
            Err(err) => panic!("Failed to cast canvas {:?}", err),
        };

        canvas
    }

    /// Get a 2D canvas context for a given canvas
    fn get_canvas_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {

        // CANVAS CONTEXT
        let context = match canvas.get_context("2d") {
            Ok(op) => match op {
                // UNWRAP OPTION
                Some(object) => object,
                None => panic!("Nothing found when unwrapping canvas context"),
            },
            Err(err) => panic!("Error when getting canvas context: {:?}", err),
        };
        // CAST CONTEXT
        let context = match context.dyn_into::<web_sys::CanvasRenderingContext2d>() {
            Ok(dyn_cast) => dyn_cast,
            Err(cast_err) => panic!("Error when casting canvas context: {:?}", cast_err)
        };

        context
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Painter {

    /// Default constructor which queries for canvas by ID
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new(width: u32, height: u32, canvas_id: &str) -> Painter {

        let canvas = Painter::get_canvas(canvas_id);

        canvas.set_width(width);
        canvas.set_height(height);

        let context = Painter::get_canvas_context(&canvas);

        Painter {
            canvas,
            context,
            width, height,
            selected_idx: None,

            white_square: JsValue::from_str(WHITE_SQUARE),
            black_square: JsValue::from_str(BLACK_SQUARE),
            
            white_piece: JsValue::from_str(WHITE_PIECE),
            black_piece: JsValue::from_str(BLACK_PIECE),
            selected_piece: JsValue::from_str(SELECTED_PIECE),

            white_piece_line: JsValue::from_str(WHITE_PIECE_OUTLINE),
            black_piece_line: JsValue::from_str(BLACK_PIECE_OUTLINE),
            selected_piece_line: JsValue::from_str(SELECTED_PIECE_OUTLINE),
            king_line: JsValue::from_str(KING_OUTLINE),
            piece_lines: DRAW_PIECE_OUTLINES,
            piece_line_proportion: PIECE_OUTLINE_PROPORTION,

            square_outline: JsValue::from_str(SQUARE_OUTLINE),
            outline_width: OUTLINE_WIDTH,
            draw_outline: DRAW_OUTLINE,
        }
    }

    /// Constructor with given canvas element
    pub fn new_with_canvas(width: u32, height: u32, canvas: HtmlCanvasElement) -> Painter {
        canvas.set_width(width);
        canvas.set_height(height);

        let context = Painter::get_canvas_context(&canvas);

        Painter {
            canvas,
            context,
            width, height,
            selected_idx: None,

            white_square: JsValue::from_str(WHITE_SQUARE),
            black_square: JsValue::from_str(BLACK_SQUARE),
            
            white_piece: JsValue::from_str(WHITE_PIECE),
            black_piece: JsValue::from_str(BLACK_PIECE),
            selected_piece: JsValue::from_str(SELECTED_PIECE),

            white_piece_line: JsValue::from_str(WHITE_PIECE_OUTLINE),
            black_piece_line: JsValue::from_str(BLACK_PIECE_OUTLINE),
            selected_piece_line: JsValue::from_str(SELECTED_PIECE_OUTLINE),
            king_line: JsValue::from_str(KING_OUTLINE),
            piece_lines: DRAW_PIECE_OUTLINES,
            piece_line_proportion: PIECE_OUTLINE_PROPORTION,

            square_outline: JsValue::from_str(SQUARE_OUTLINE),
            outline_width: OUTLINE_WIDTH,
            draw_outline: DRAW_OUTLINE,
        }
    }

    /// Set selected piece by board index
    pub fn set_selected(&mut self, idx: &BrdIdx) {
        self.selected_idx = Option::Some(*idx);
    }

    /// Clear selected piece by board index
    pub fn clear_selected(&mut self) {
        self.selected_idx = None;
    }

    /// Set new square outline colour value
    pub fn set_square_outline(&mut self, value: JsValue) {
        self.square_outline = value;
    }

    /// Set new line width for outlining squares
    pub fn set_outline_width(&mut self, value: f64) {
        self.outline_width = value;
    }

    /// Set whether squares are outlined
    pub fn set_draw_outline(&mut self, value: bool) {
        self.draw_outline = value;
    }

    /// Reset the canvas dimensions to the given width and height
    pub fn reset_dimensions(&self) {
        self.canvas.set_width(self.width);
        self.canvas.set_height(self.height);
    }

    /// Check whether given canvas dimensions divide evenly by given board dimenions
    pub fn validate_board_dim(&self, board: &Board) -> bool {
        let mut ans = true;

        if self.height as usize % board.height != 0 {
            log!("Canvas and board heights do not evenly divide, Canvas({}) / Board({}) = {} px/cell", self.height, board.height, self.height as f32 / board.height as f32);
            ans = false;
        }

        if self.width as usize % board.width != 0 {
            log!("Canvas and board widths do not evenly divide, Canvas({}) / Board({}) = {} px/cell", self.width, board.width, self.width as f32 / board.width as f32);
            ans = false;
        }

        ans
    }

    pub fn draw_current(&self, game: &Game)
    {
        self.draw(game.current_board());
    }

    /// Draw a board onto the canvas
    pub fn draw(&self, board: &Board) {

        self.validate_board_dim(board);

        let cell_height = self.height as usize / board.height;
        let cell_width = self.width as usize / board.width;
        
        let min_dimension = usize::min(cell_width, cell_height) as f64;

        let cell_radius = min_dimension * PIECE_PROPORTION / 2.0;

        let piece_outline = cell_radius * self.piece_line_proportion;
        let square_outline = min_dimension * self.outline_width;

        self.context.set_fill_style(&self.white_square);
        self.context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

        self.context.set_fill_style(&self.black_square);
        self.context.set_stroke_style(&self.square_outline);
        self.context.set_line_width(square_outline);

        // Draw black squares onto canvas
        for i in 0..board.height {
            for j in 0..board.width {

                if i % 2 == 0 {
                    if j % 2 == 1 {
                        self.context.fill_rect(
                            (j * cell_width) as f64, 
                            (i * cell_height) as f64, 
                            cell_width as f64, 
                            cell_height as f64
                        );

                        if self.draw_outline {
                            self.context.stroke_rect(
                                (j * cell_width) as f64, 
                                (i * cell_height) as f64, 
                                cell_width as f64, 
                                cell_height as f64
                            );
                        }
                    }
                }
                else {
                    if j % 2 == 0 {
                        self.context.fill_rect(
                            (j * cell_width) as f64, 
                            (i * cell_height) as f64, 
                            cell_width as f64, 
                            cell_height as f64
                        );

                        if self.draw_outline {
                            self.context.stroke_rect(
                                (j * cell_width) as f64, 
                                (i * cell_height) as f64, 
                                cell_width as f64, 
                                cell_height as f64
                            );
                        }
                    }
                }
            }
        }

        // Draw pieces onto canvas
        for (idx, square) in PieceIterator::new(board) {
            match square.occupant {
                Some(piece) => {

                    let brd_idx = board.board_index(idx);

                    match piece.team {
                        Black => {
                            self.context.set_fill_style(&self.black_piece);
                            self.context.set_stroke_style(&self.black_piece_line);
                        },
                        White => {
                            self.context.set_fill_style(&self.white_piece);
                            self.context.set_stroke_style(&self.white_piece_line);
                        },
                    }

                    if piece.strength == King {
                        self.context.set_stroke_style(&self.king_line);
                    }

                    let center_x: f64 = (brd_idx.col as f64 * cell_width as f64) + (cell_width as f64) / 2.0;
                    let center_y: f64 = (brd_idx.row as f64 * cell_height as f64) + (cell_height as f64) / 2.0;

                    self.context.begin_path();
                    match self.context.arc(
                        center_x, 
                        center_y, 
                        cell_radius, // radius 
                        0.0, // start angle
                        f64::consts::PI * 2.0) // end angle
                    {
                        Ok(res) => res,
                        Err(err) => log!("Failed to draw piece, idx: {}, square: {:?}, {:?}", idx, square, err),
                    };
                    self.context.fill();

                    if self.piece_lines {
                        self.context.set_line_width(piece_outline);
                        self.context.stroke()
                    }

                },
                None => panic!("No piece found when attempting to draw, idx: {}, square: {:?}", idx, square),
            }
        }

        if let Some(selected_idx) = self.selected_idx {
            self.context.set_fill_style(&self.selected_piece);
            self.context.set_stroke_style(&self.selected_piece_line);

            let center_x: f64 = (selected_idx.col as f64 * cell_width as f64) + (cell_width as f64) / 2.0;
            let center_y: f64 = (selected_idx.row as f64 * cell_height as f64) + (cell_height as f64) / 2.0;

            self.context.begin_path();
            match self.context.arc(
                center_x, 
                center_y, 
                cell_radius, // radius 
                0.0, // start angle
                f64::consts::PI * 2.0) // end angle
            {
                Ok(res) => res,
                Err(err) => log!("Failed to paint selected piece, idx: {}, {:?}", selected_idx, err),
            };
            self.context.fill();

            if self.piece_lines {
                self.context.set_line_width(piece_outline);
                self.context.stroke()
            }
        }
    }
}