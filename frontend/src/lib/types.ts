export type ShipType = 'carrier' | 'battleship' | 'destroyer' | 'submarine' | 'patrolboat';

export type Direction = 'horizontal' | 'vertical';

export type CellView =
    | { type: 'unknown' }
    | { type: 'empty' }
    | { type: 'hit' }
    | { type: 'miss' }
    | { type: 'blocked' }
    | { type: 'ship'; ship_type: ShipType };

export type BoardView = {
    cells: CellView[][];
};

export type Coord = {
    row: number;
    col: number;
};

export type Player = 'player1' | 'player2';

export type ShotResult = 'hit' | 'miss' | 'sunk' | 'alreadyshot';

export type TurnEvent = {
    player: Player;
    coord: Coord;
    result: ShotResult;
};

export type GameStatus =
    | { type: 'placing_ships' }
    | { type: 'ongoing' }
    | { type: 'finished'; winner: Player };

export type GameState = {
    type: 'game_state';
    player: Player;
    turn: Player;
    status: GameStatus;
    player_board: BoardView;
    opponent_board: BoardView;
};

export type GameUpdate = {
    type: 'game_update';
    event: TurnEvent;
    turn: Player;
    status: GameStatus;
    player_board: BoardView;
    opponent_board: BoardView;
};

export type ShipPlacement = {
    ship_type: 'carrier' | 'battleship' | 'destroyer' | 'submarine' | 'patrolboat';
    start: Coord;
    direction: 'horizontal' | 'vertical';
};

export type RandomFleet = {
    type: 'random_fleet';
    fleet: ShipPlacement[];
};

export type ErrorMessage = {
    type: 'error';
    message: string;
};

export type ServerMessage = GameState | GameUpdate | RandomFleet | ErrorMessage;

export type GameSnapshot = {
    turn: Player;
    status: GameStatus;
    player_board: BoardView;
    opponent_board: BoardView;
};

export type RandomFleetMessage = {
    type: 'random_fleet';
};

export type PlaceFleetMessage = {
    type: 'place_fleet';
    fleet: ShipPlacement[];
};

export type FireMessage = {
    type: 'fire';
    coord: Coord;
};

export type ClientMessage = RandomFleetMessage | PlaceFleetMessage | FireMessage;

export type PreviewCell =
    | { type: 'empty' }
    | { type: 'blocked' }
    | { type: 'placed'; ship_type: ShipType }
    | { type: 'preview-valid'; ship_type: ShipType }
    | { type: 'preview-invalid'; ship_type: ShipType };

export type PreviewBoard = {
    cells: PreviewCell[][];
};
