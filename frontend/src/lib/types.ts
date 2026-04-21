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
    sunk_ship: ShipType | null;
    damage: DamageInfo | null;
};

export type TurnEvent = {
    player: Player;
    coord: Coord;
    outcome: ShotOutcome;
};

export type GameStatus =
    | { type: 'placing_ships' }
    | { type: 'ongoing' }
    | { type: 'finished'; winner: Player }
    | { type: 'abandoned'; winner: Player | null };

export type ShipStatus = {
    ship_type: ShipType;
    damage: number | null;
    sunk: boolean;
};

export type FleetView = {
    ships: ShipStatus[];
};

export type ShipPlacement = {
    ship_type: ShipType;
    start: Coord;
    direction: Direction;
};

export type DisconnectInfo = {
    player: Player;
    disconnected_at: number;
};

export type RematchState = { type: 'idle' } | { type: 'requested'; by: Player };

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
    opponent_present: boolean;
    player_ready: boolean;
    opponent_ready: boolean;
    rematch_state: RematchState;
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

export type PlayerDisconnected = {
    type: 'player_disconnected';
    info: DisconnectInfo;
};

export type PlayerReconnected = {
    type: 'player_reconnected';
    player: Player;
};

export type RematchCancelled = {
    type: 'rematch_cancelled';
    player: Player;
};

export type RematchRejected = {
    type: 'rematch_rejected';
    player: Player;
};

export type ErrorMessage = {
    type: 'error';
    message: string;
};

export type ServerMessage =
    | GameState
    | GameUpdate
    | RandomFleet
    | PlayerDisconnected
    | PlayerReconnected
    | RematchCancelled
    | RematchRejected
    | ErrorMessage;
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

export type RequestRematchMessage = {
    type: 'request_rematch';
};

export type CancelRematchMessage = {
    type: 'cancel_rematch';
};

export type RejectRematchMessage = {
    type: 'reject_rematch';
};

export type LeaveGameMessage = {
    type: 'leave_game';
};

export type ClientMessage =
    | RandomFleetMessage
    | PlaceFleetMessage
    | FireMessage
    | RequestRematchMessage
    | CancelRematchMessage
    | RejectRematchMessage
    | LeaveGameMessage;
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

export type NotificationType = 'info' | 'success' | 'error';

export type Notification = {
    id: string;
    title: string;
    message?: string;
    type?: NotificationType;
};

export type ApiErrorCode =
    | 'session_not_found'
    | 'invalid_player'
    | 'invalid_message'
    | 'invalid_coordinates'
    | 'not_players_turn'
    | 'game_already_finished'
    | 'invalid_game_state'
    | 'game_full'
    | 'invalid_placement'
    | 'internal_error';
