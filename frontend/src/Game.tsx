import { useEffect, useMemo, useRef, useState } from "react";
import { Bot, CircleHelp, Coins, Landmark, Package, Ship, Warehouse } from "lucide-react";
import ActionPanel from "./Actions";
import Table, { Pick } from "./Table";
import { COLOR_LABELS, LogEntry, PlayerView, Snapshot, botName, cardValue, total } from "./engine";
import { RichText, Swatch, money, seatColor } from "./ui";
import type { Speed } from "./useGame";

/** Which containers on the table the player may click right now, and the action each click plays. */
export function buildPick(snap: Snapshot): Pick {
  const ids = new Map<string, number>();
  const phase = snap.phase;
  for (const a of snap.legal) {
    if (a.type === "Buy" && phase.type === "Buy") {
      ids.set(`${phase.seller}|${phase.district}|${a.lot}|${a.color}`, a.id);
    } else if (a.type === "Seize" && phase.type === "Seize") {
      ids.set(`${phase.debtor}|${phase.place}|${a.lot}|${a.color}`, a.id);
    } else if (a.type === "BidContainer") {
      ids.set(`${snap.viewer}|${a.district}|${a.lot}|${a.color}`, a.id);
    }
  }
  return (player, area, lot, color) => ids.get(`${player}|${area}|${lot}|${color}`);
}

export type Props = {
  snap: Snapshot;
  act: (id: number) => void;
  undo: () => void;
  stepBack: () => void;
  speed: Speed;
  setSpeed: (speed: Speed) => void;
  onLeave: () => void;
  onHelp: () => void;
};

export default function GameScreen({ snap, act, undo, stepBack, speed, setSpeed, onLeave, onHelp }: Props) {
  const pick = useMemo(() => buildPick(snap), [snap]);

  return (
    <main className="game-shell">
      <header className="topbar">
        <div className="brand"><span className="brand-mark"><Ship size={17} /></span> CONTAINER</div>
        <div className="round-indicator">
          <span className="live-dot" /> ROUND {snap.round}
          {snap.lastTurn && <span className="last-turn">FINAL TURN</span>}
        </div>
        <div className="top-actions">
          <div className="segmented small" aria-label="Opponent speed">
            {(["slow", "normal", "fast"] as Speed[]).map((s) => (
              <button key={s} className={speed === s ? "on" : ""} onClick={() => setSpeed(s)}>{s}</button>
            ))}
          </div>
          <button className="text-button" onClick={onLeave}>Leave game</button>
          <button className="help-button" onClick={onHelp} aria-label="How to play"><CircleHelp size={18} /></button>
        </div>
      </header>

      <div className="game-layout">
        <Sidebar snap={snap} />
        <section className="board-area">
          <Table snap={snap} pick={pick} act={act} />
        </section>
        <aside className="right-column">
          <ActionPanel snap={snap} act={act} undo={undo} stepBack={stepBack} />
          <LogPanel snap={snap} />
        </aside>
      </div>
    </main>
  );
}

function Sidebar({ snap }: { snap: Snapshot }) {
  return (
    <aside className="sidebar">
      <div className="sidebar-title">
        <span>TABLE</span>
        <span className="player-count">{snap.numPlayers} / 5</span>
      </div>
      <div className="player-list">
        {snap.players.map((p, seat) => (
          <PlayerCard key={seat} snap={snap} seat={seat} player={p} />
        ))}
      </div>
      <SupplyPanel snap={snap} />
      <ScoringCard snap={snap} />
    </aside>
  );
}

function PlayerCard({ snap, seat, player }: { snap: Snapshot; seat: number; player: PlayerView }) {
  const seatInfo = snap.seats[seat];
  const isBot = seatInfo.kind !== "human";
  const active = seat === snap.activePlayer && !snap.gameOver;
  const deciding = seat === snap.currentPlayer && !snap.gameOver;
  return (
    <div className={`player-card ${active ? "player-card-active" : ""}`}>
      <div className="player-card-heading">
        <span className="avatar" style={{ background: seatColor(seat) }}>
          {isBot ? <Bot size={15} /> : seatInfo.name.charAt(0).toUpperCase()}
        </span>
        <div>
          <strong>{seatInfo.name}</strong>
          <span className="player-kind">{isBot ? `${botName(seatInfo.kind)} AI` : "You"}</span>
        </div>
        {active && <span className="turn-pill">TURN</span>}
        {!active && deciding && <span className="turn-pill pill-deciding">DECIDING</span>}
      </div>
      <div className="player-stats">
        <span data-rule="cash"><Coins size={14} /> {player.cash === null ? "hidden" : money(player.cash)}</span>
        <span data-rule="loans"><Landmark size={14} /> {player.loans}</span>
        <span data-rule="warehouses"><Warehouse size={14} /> {player.warehouses}</span>
      </div>
    </div>
  );
}

export function SupplyPanel({ snap }: { snap: Snapshot }) {
  const { containers, factories, warehouses } = snap.supply;
  return (
    <div className="side-panel">
      <div className="panel-heading" data-rule="supply"><span><Package size={15} /> SUPPLY</span></div>
      <div className="supply-colors">
        {containers.map((n, color) => (
          <span key={color} className={n === 0 ? "supply-empty" : ""} title={`${COLOR_LABELS[color]} containers left`}>
            <Swatch color={color} size={12} /> {n}
          </span>
        ))}
      </div>
      <p className="tiny">
        {total(factories)} factories · {warehouses} warehouses left. The game ends when two colors run out.
      </p>
    </div>
  );
}

export function ScoringCard({ snap }: { snap: Snapshot }) {
  const ten = snap.players[snap.viewer].scoringCard;
  if (ten === null) return null;
  const order = [0, 1, 2, 3, 4].sort((a, b) => cardValue(ten, b, true) - cardValue(ten, a, true));
  return (
    <div className="side-panel">
      <div className="panel-heading" data-rule="scoringCard"><span>YOUR SCORING CARD</span><small>SECRET</small></div>
      <div className="card-values">
        {order.map((color) => (
          <span key={color}>
            <Swatch color={color} size={12} />
            {money(cardValue(ten, color, true))}
            {cardValue(ten, color, true) !== cardValue(ten, color, false) && (
              <small title="Worth only $5 without a full set"> /{cardValue(ten, color, false)}</small>
            )}
          </span>
        ))}
      </div>
      <p className="tiny">Per container on the Island. Your most common color there is discarded.</p>
    </div>
  );
}

function LogPanel({ snap }: { snap: Snapshot }) {
  const names = snap.seats.map((s) => s.name);
  const viewer = snap.seats.findIndex((s) => s.kind === "human");
  const list = useRef<HTMLDivElement>(null);
  const [tip, setTip] = useState<{ entry: LogEntry; top: number; right: number } | null>(null);
  useEffect(() => {
    const el = list.current;
    if (el) el.scrollTop = el.scrollHeight;
  }, [snap.log.length]);

  const show = (entry: LogEntry, row: HTMLElement) => {
    const box = row.getBoundingClientRect();
    setTip({
      entry,
      top: Math.min(box.top, window.innerHeight - 140),
      right: window.innerWidth - box.left + 10,
    });
  };

  return (
    <div className="side-panel log-panel">
      <div className="panel-heading"><span>EVENTS</span></div>
      <div className="log-list" ref={list} onScroll={() => setTip(null)} onMouseLeave={() => setTip(null)}>
        {snap.log.map((entry, i) => (
          <p
            key={i}
            className="log-entry"
            tabIndex={0}
            onMouseEnter={(e) => show(entry, e.currentTarget)}
            onFocus={(e) => show(entry, e.currentTarget)}
            onBlur={() => setTip(null)}
          >
            {entry.player !== null && <i style={{ background: seatColor(entry.player) }} />}
            <RichText text={entry.text} names={names} viewer={viewer} />
          </p>
        ))}
      </div>
      {tip && (
        <div className="log-tip" role="tooltip" style={{ top: tip.top, right: tip.right }}>
          <b>Round {tip.entry.round}</b>
          {tip.entry.detail.length === 0 && <span>No other changes.</span>}
          {tip.entry.detail.map((line, n) => (
            <span key={n}><RichText text={line} names={names} viewer={viewer} /></span>
          ))}
        </div>
      )}
    </div>
  );
}
