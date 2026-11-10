import init, { Game, opponents as listOpponents } from "./wasm/container_web.js";

export type Counts = [number, number, number, number, number];
export type District = "factory" | "harbor";
export type AuctionKind = "container" | "cash";
export type ShipAt =
  | { at: "ocean" | "island" | "bank" }
  | { at: "harbor"; owner: number };

export const COLORS = ["red", "blue", "yellow", "white", "green"] as const;
export const COLOR_LABELS = ["Red", "Blue", "Yellow", "White", "Green"] as const;
export const FACTORY_LOTS = 4;
export const HARBOR_LOTS = 5;
export const SHIP_CAPACITY = 5;
export const LOT_NUMERALS = ["I", "II", "III", "IV", "V"];

export const lotPrice = (district: District, lot: number) =>
  district === "factory" ? lot + 1 : lot + 2;

// Cost of the next building, indexed by how many the player already owns (mirrors the engine's build.rs).
const FACTORY_COSTS = [0, 6, 9, 12];
const WAREHOUSE_COSTS = [0, 4, 5, 6, 7];
export const factoryCost = (owned: number) => FACTORY_COSTS[owned];
export const warehouseCost = (owned: number) => WAREHOUSE_COSTS[owned];

export type Action =
  | { type: "Produce" }
  | { type: "ProduceColor"; color: number }
  | { type: "BuildFactory"; color: number }
  | { type: "BuildWarehouse" }
  | { type: "FactoryPurchase"; seller: number }
  | { type: "Buy"; lot: number; color: number }
  | { type: "DoneBuying" }
  | { type: "Sail"; to: ShipAt }
  | { type: "HarborPurchase" }
  | { type: "Load"; color: number }
  | { type: "DoneLoading" }
  | { type: "Reprice"; district: District }
  | { type: "Place"; lot: number }
  | { type: "Bid"; amount: number }
  | { type: "Accept"; player: number }
  | { type: "BuyOut" }
  | { type: "RequestLoan" }
  | { type: "RepayLoan" }
  | { type: "Seize"; lot: number; color: number }
  | { type: "Deal"; color: number }
  | { type: "StartAuction"; kind: AuctionKind; lot: number }
  | { type: "Outbid"; kind: AuctionKind }
  | { type: "BidCash"; amount: number }
  | { type: "BidContainer"; district: District; lot: number; color: number }
  | { type: "DoneBidding" };

export type LegalAction = Action & { id: number };

export type SeizePlace = "island" | "ship" | "holding" | "harbor" | "factory";

export type Phase =
  | { type: "Main" }
  | { type: "Produce"; candidates: Counts; toPick: number; picked: Counts }
  | { type: "Buy"; seller: number; district: District; bought: Counts; optional: boolean }
  | { type: "Load"; loaded: number; optional: boolean }
  | { type: "Arrange"; owner: number; district: District; hand: Counts; minLot: number; later: Counts }
  | {
      type: "Bid";
      deliverer: number;
      bidder: number;
      contenders: boolean[];
      runoff: boolean;
      base: number;
      bids: number[];
      placed: boolean[];
    }
  | { type: "Decide"; deliverer: number; top: number; tied: boolean[] }
  | { type: "Seize"; debtor: number; remaining: number; seized: Counts; place: SeizePlace }
  | { type: "Distribute"; decider: number; hand: Counts; dealt: number; wonLot: number | null }
  | {
      type: "BankBid";
      kind: AuctionKind;
      lot: number;
      min: number;
      picked: { factory: Counts; harbor: Counts };
    }
  | { type: "GameOver" };

export type PlayerView = {
  cash: number | null;
  scoringCard: number | null;
  loans: number;
  factories: boolean[];
  warehouses: number;
  factoryLots: Counts[];
  harborLots: Counts[];
  ship: ShipAt;
  cargo: Counts;
  holding: Counts;
  island: Counts;
  reserveFactory: number;
  reserveHarbor: number;
};

export type BankAuction = {
  kind: AuctionKind;
  lot: number;
  holder: number;
  bid: { cash: number } | { factory: Counts; harbor: Counts };
};

export type Score = {
  cash: number;
  island: number;
  leftovers: number;
  loans: number;
  total: number;
  discarded: number | null;
};

export type LogEntry = { player: number | null; text: string; round: number; detail: string[] };

export type Snapshot = {
  viewer: number;
  numPlayers: number;
  firstPlayer: number;
  activePlayer: number;
  currentPlayer: number;
  lastTurn: boolean;
  round: number;
  canUndo: boolean;
  canStepBack: boolean;
  gameOver: boolean;
  turn: {
    actionsLeft: number;
    produced: boolean;
    loansRequested: number;
    loansRepaid: number;
    bankCalled: boolean;
    wonAuction: boolean;
  };
  seats: { name: string; kind: string }[];
  players: PlayerView[];
  bank: { containerLots: Counts[]; cashLots: number[]; auctions: BankAuction[] };
  supply: { containers: Counts; factories: Counts; warehouses: number };
  phase: Phase;
  legal: LegalAction[];
  log: LogEntry[];
  results: { scores: Score[]; winners: number[] } | null;
};

export type OpponentInfo = { id: string; name: string; description: string };

export type SeatConfig = { name: string; kind: string };

export type GameConfig = {
  seed: number;
  length: "short" | "standard" | "extended";
  players: SeatConfig[];
};

let ready: Promise<unknown> | null = null;

/** Loads the WebAssembly engine once. */
export function loadEngine(): Promise<unknown> {
  ready ??= init();
  return ready;
}

export async function getOpponents(): Promise<OpponentInfo[]> {
  await loadEngine();
  return JSON.parse(listOpponents());
}

/** A game running in the Rust engine. */
export class GameClient {
  private game: Game;

  constructor(config: GameConfig) {
    this.game = new Game(JSON.stringify(config));
  }

  snapshot(): Snapshot {
    return JSON.parse(this.game.snapshot());
  }

  act(id: number) {
    this.game.act(id);
  }

  stepBack() {
    this.game.step_back();
  }

  undo() {
    this.game.undo();
  }

  step(): boolean {
    return this.game.step();
  }

  dispose() {
    this.game.free();
  }
}

export const total = (c: Counts) => c.reduce((a, b) => a + b, 0);

export const isHuman = (snap: Snapshot, seat: number) => snap.seats[seat].kind === "human";

/** What one container of a color is worth to the holder of this scoring card, at the end of the game. */
export function cardValue(ten: number, color: number, fullSet: boolean): number {
  switch (((color - ten) % 5 + 5) % 5) {
    case 0:
      return 10;
    case 1:
      return fullSet ? 10 : 5;
    case 2:
      return 6;
    case 3:
      return 4;
    default:
      return 2;
  }
}
