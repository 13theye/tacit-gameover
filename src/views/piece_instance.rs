// src/views/piece_instance.rs
//
//
// Defining the individual pieces on the screen

use crate::models::PieceType;
use nannou::prelude::*;

pub enum RotationDirection {
    Cw,
    Ccw,
}

// Board position of a piece
#[derive(Debug, Copy, Clone)]
pub struct BoardPosition {
    pub x: isize,
    pub y: isize,
}

type Cells = [(isize, isize); 4];

pub struct PieceInstance {
    pub typ: PieceType,
    pub color: Rgba,
    pub rot_idx: usize, // rotation index
    pub position: BoardPosition,
}

impl PieceInstance {
    pub fn new(typ: PieceType, color: Rgba, position: BoardPosition) -> Self {
        Self {
            typ,
            color,
            rot_idx: 0,
            position,
        }
    }

    pub fn cells(&self) -> &Cells {
        self.typ.get_rotation(self.rot_idx)
    }

    fn rotate(&mut self, direction: RotationDirection) -> &Cells {
        let count = self.typ.rotation_count();

        let inx = match direction {
            RotationDirection::Cw => (self.rot_idx + 1) % count,
            RotationDirection::Ccw => (self.rot_idx + count - 1) % count,
        };

        self.rot_idx = inx;
        self.typ.get_rotation(inx)
    }
}
