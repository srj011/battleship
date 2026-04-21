export type Coord = {
    row: number;
    col: number;
};

export type ShipType = 'carrier' | 'battleship' | 'destroyer' | 'submarine' | 'patrolboat';

export type Direction = 'horizontal' | 'vertical';

export type CellView =
    | { type: 'unknown' }
    | { type: 'empty' }
    | { type: 'hit'; ship_type: ShipType }
    | { type: 'miss' }
    | { type: 'blocked' }
    | { type: 'ship'; ship_type: ShipType };

export type BoardView = {
    cells: CellView[][];
};

export type PreviewCell =
    | { type: 'empty' }
    | { type: 'blocked' }
    | { type: 'placed'; ship_type: ShipType }
    | { type: 'preview-valid'; ship_type: ShipType }
    | { type: 'preview-invalid'; ship_type: ShipType };

export type PreviewBoard = {
    cells: PreviewCell[][];
};

export type Player = 'player1' | 'player2';

export type ShotResult = 'hit' | 'miss' | 'sunk' | 'alreadyshot';

export type DamageInfo = {
    ship_type: ShipType;
    total: number;
};

export type ShotOutcome = {
    result: ShotResult;
    blocked: Coord[];
    sunk_ship?: ShipType;
    damage?: DamageInfo;
};

export type TurnEvent = {
    player: Player;
    coord: Coord;
    outcome: ShotOutcome;
};

export type GameStatus =
    | { type: 'placing_ships' }
    | { type: 'ongoing' }
    | { type: 'finished'; winner: Player };

export type ShipStatus = {
    ship_type: ShipType;
    damage?: number;
    sunk: boolean;
};

export type FleetView = {
    ships: ShipStatus[];
};

export type ShipPlacement = {
    ship_type: 'carrier' | 'battleship' | 'destroyer' | 'submarine' | 'patrolboat';
    start: Coord;
    direction: 'horizontal' | 'vertical';
};

// Server Message
/* ----------------------------------------------------------------------------------- */
export type GameState = {
    type: 'game_state';
    player: Player;
    turn: Player;
    status: GameStatus;
    player_board: BoardView;
    opponent_board: BoardView;
    player_fleet: FleetView;
    opponent_fleet: FleetView;
    opponent_joined: boolean;
    player_ready: boolean;
    opponent_ready: boolean;
    player_rematch_ready: boolean;
    opponent_rematch_ready: boolean;
};

export type GameUpdate = {
    type: 'game_update';
    event: TurnEvent;
    turn: Player;
    status: GameStatus;
    player_board: BoardView;
    opponent_board: BoardView;
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
/* ----------------------------------------------------------------------------------- */

// Client Message
/* ----------------------------------------------------------------------------------- */
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

export type RestartMessage = {
    type: 'restart';
};

export type ClientMessage = RandomFleetMessage | PlaceFleetMessage | FireMessage | RestartMessage;
/* ----------------------------------------------------------------------------------- */

export type ConnectionState =
    | 'idle'
    | 'connecting'
    | 'connected'
    | 'reconnecting'
    | 'unreachable'
    | 'invalid-session';

export type Connection = {
    state: ConnectionState;
    attempt: number;
};

export type Event =
    | { type: 'CONNECT' }
    | { type: 'CONNECTED' }
    | { type: 'DISCONNECTED' }
    | { type: 'RETRY' }
    | { type: 'MAX_RETRIES' }
    | { type: 'INVALID_SESSION' }
    | { type: 'LEAVE' };
