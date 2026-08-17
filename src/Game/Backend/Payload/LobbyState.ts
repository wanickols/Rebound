export class LobbyState {
  expected_players: number;
  players: LobbyPlayer[];

  constructor(expected_players: number, players: LobbyPlayer[]) {
    this.expected_players = expected_players;
    this.players = players;
  }

  static from(obj: any): LobbyState {
    return new LobbyState(
      obj.expected_players,
      obj.players.map((p: any) => new LobbyPlayer(p.player_id, p.team_id)),
    );
  }
}

export class LobbyPlayer {
  player_id: number;
  team_id?: number;

  constructor(player_id: number, team_id?: number) {
    this.player_id = player_id;
    this.team_id = team_id;
  }
}
