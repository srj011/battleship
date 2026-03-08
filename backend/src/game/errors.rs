#[derive(Debug)]
pub enum PlacementError {
    ShipOutOfBounds,
    ShipOverlap,
    ShipAlreadyPlaced,
    InvalidFleetSize,
}

#[derive(Debug)]
pub enum GameError {
    GameAlreadyFinished,
    NotPlayersTurn,
    InvalidGameState,
    Placement(PlacementError),
}

impl From<PlacementError> for GameError {
    fn from(err: PlacementError) -> Self {
        GameError::Placement(err)
    }
}
