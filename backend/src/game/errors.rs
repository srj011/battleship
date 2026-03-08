#[derive(Debug)]
pub enum PlacementError {
    ShipOutOfBounds,
    ShipOverlap,
    ShipAlreadyPlaced,
}

#[derive(Debug)]
pub enum GameError {
    GameAlreadyFinished,
    NotPlayersTurn,
}
