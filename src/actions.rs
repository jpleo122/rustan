use crate::board::{Board, Player};
use crate::coordinates::HexVertex;
use crate::game::GameState;

#[derive(Debug, Clone)]
pub struct ActionError(String);

pub trait Action {

    fn apply(&self, state: GameState) -> Result<GameState, ActionError>;
    fn reverse(&self, state: GameState) -> Result<GameState, ActionError>;
}

pub struct PlaceSettlement {
    player: Player,
    hex_vertex: HexVertex
}

impl PlaceSettlement {
    fn can_place_settlement(&self, board: &Board) -> bool {
        true
    }
}

impl Action for PlaceSettlement {

    fn apply(&self, state: GameState) -> Result<GameState, ActionError> {

        // see if player can place settlement (ie must be two edges away)
        // if they can, then update board and send ok
        // if not have action error

        if self.can_place_settlement(&state.board) {
            return Ok(state)
        }
        Err(ActionError("Settlement must be 2 edges away".to_string()))
    }

    fn reverse(&self, state: GameState) -> Result<GameState, ActionError> {
        Ok(state)
    }
}