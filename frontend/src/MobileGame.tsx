import { ReactNode, useEffect, useMemo, useRef, useState } from "react";
import { Bot, ChevronRight, Gauge, Landmark, Menu, Ship, Waves } from "lucide-react";
import ActionPanel from "./Actions";
import { Props, ScoringCard, SupplyPanel, buildPick } from "./Game";
import Sheet from "./Sheet";
import { BankZone, Harbor, IslandZone, OceanZone } from "./Table";
import { Phase, Snapshot, cardValue } from "./engine";
import { RULES } from "./rules";
import { RichText, Swatch, money, seatColor } from "./ui";
import type { Speed } from "./useGame";

/** A seat's harbor, or one of the two shared areas. */
type Place = number | "sea" | "bank";
/** Where the player has to look right now, and whether they must tap containers there. */
type Focus = { place: Place; pick: boolean };

const SPEEDS: Speed[] = ["slow", "normal", "fast"];

function seizeFocus(snap: Snapshot, phase: Extract<Phase, { type: "Seize" }>): Focus {
  const ship = snap.players[phase.debtor].ship;
  switch (phase.place) {
    case "harbor":
    case "factory":
      return { place: phase.debtor, pick: true };
    case "island":
      return { place: "sea", pick: true };
    case "holding":
      return { place: "bank", pick: true };
    case "ship":
      return { place: ship.at === "harbor" ? ship.owner : ship.at === "bank" ? "bank" : "sea", pick: true };
  }
}

/** The place that matters for the decision the player is facing, if there is one. */
function focusOf(snap: Snapshot): Focus | null {
  if (snap.legal.length === 0) return null;
  const phase = snap.phase;
  switch (phase.type) {
    case "Main":
    case "Produce":
      return { place: snap.viewer, pick: false };
    case "Arrange":
      return { place: phase.owner, pick: false };
    case "Buy":
      return { place: phase.seller, pick: true };
    case "Load":
    case "Distribute":
      return { place: "bank", pick: false };
    case "Bid":
    case "Decide":
      return { place: "sea", pick: false };
    case "BankBid":
      return phase.kind === "cash" ? { place: snap.viewer, pick: true } : { place: "bank", pick: false };
    case "Seize":
      return seizeFocus(snap, phase);
    default:
      return null;
  }
}

const where = (snap: Snapshot, place: Place) =>
  place === "sea"
    ? "at Sea"
    : place === "bank"
      ? "at the Bank"
      : place === snap.viewer
        ? "in your harbor"
        : `in ${snap.seats[place].name}'s harbor`;

export default function MobileGame({ snap, act, undo, stepBack, speed, setSpeed, onLeave, onHelp }: Props) {
  const pick = useMemo(() => buildPick(snap), [snap]);
  const focus = useMemo(() => focusOf(snap), [snap]);
  const [place, setPlace] = useState<Place>(snap.viewer);
  const [sheet, setSheet] = useState<"events" | "menu" | null>(null);
  const content = useRef<HTMLElement>(null);

  useEffect(() => {
    document.documentElement.classList.add("m-active");
    return () => document.documentElement.classList.remove("m-active");
  }, []);

  // Follow the decision: jump to the place it concerns when a new one starts, but never fight the player mid-decision.
  const focusKey = focus ? `${snap.phase.type}|${focus.place}` : null;
  useEffect(() => {
    if (focus) setPlace(focus.place);
  }, [focusKey]);

  useEffect(() => {
    content.current?.scrollTo({ top: 0 });
  }, [place]);

  const names = snap.seats.map((s) => s.name);
  const last = snap.log[snap.log.length - 1];
  const common = { snap, pick, act };

  return (
    <main className="m-shell">
      <header className="m-top">
        <span className="brand-mark"><Ship size={17} /></span>
        <button className="m-ticker" onClick={() => setSheet("events")} aria-label="Open the event log">
          <span className="m-round">
            <span className="live-dot" /> ROUND {snap.round}
            {snap.lastTurn && <span className="last-turn">FINAL TURN</span>}
          </span>
          <span className="m-last" key={snap.log.length}>
            {last ? <RichText text={last.text} names={names} viewer={snap.viewer} /> : "The game has begun"}
          </span>
        </button>
        <button className="m-icon" onClick={() => setSheet("menu")} aria-label="Menu">
          <Menu size={22} />
        </button>
      </header>

      <nav className="m-strip" aria-label="Places">
        {snap.players.map((_, seat) => (
          <SeatChip
            key={seat}
            snap={snap}
            seat={seat}
            on={place === seat}
            attn={focus?.pick === true && focus.place === seat && place !== seat}
            onClick={() => setPlace(seat)}
          />
        ))}
        <span className="m-sep" aria-hidden />
        <PlaceChip
          icon={<Waves size={16} />}
          label="Sea"
          on={place === "sea"}
          attn={focus?.pick === true && focus.place === "sea" && place !== "sea"}
          onClick={() => setPlace("sea")}
        />
        <PlaceChip
          icon={<Landmark size={16} />}
          label="Bank"
          on={place === "bank"}
          attn={focus?.pick === true && focus.place === "bank" && place !== "bank"}
          badge={snap.bank.auctions.length}
          onClick={() => setPlace("bank")}
        />
      </nav>

      <div className="m-main">
        <section className="m-content" ref={content}>
          <CardRow snap={snap} />
          {typeof place === "number" ? (
            <Harbor {...common} seat={place} mine={place === snap.viewer} />
          ) : place === "sea" ? (
            <>
              <IslandZone {...common} />
              <OceanZone {...common} />
            </>
          ) : (
            <BankZone {...common} />
          )}
        </section>
        {focus?.pick && focus.place !== place && (
          <button className="m-pill" onClick={() => setPlace(focus.place)}>
            Pick containers {where(snap, focus.place)} <ChevronRight size={16} />
          </button>
        )}
      </div>

      <footer className={snap.legal.length === 0 ? "m-dock m-dock-wait" : "m-dock"}>
        <ActionPanel
          snap={snap}
          act={act}
          undo={undo}
          stepBack={stepBack}
          waitingExtra={<SpeedButton speed={speed} setSpeed={setSpeed} />}
        />
      </footer>

      {sheet === "events" && <EventsSheet snap={snap} onClose={() => setSheet(null)} />}
      {sheet === "menu" && (
        <MenuSheet
          snap={snap}
          speed={speed}
          setSpeed={setSpeed}
          onClose={() => setSheet(null)}
          onHelp={() => {
            setSheet(null);
            onHelp();
          }}
          onLeave={onLeave}
        />
      )}
    </main>
  );
}

function SeatChip({ snap, seat, on, attn, onClick }: { snap: Snapshot; seat: number; on: boolean; attn: boolean; onClick: () => void }) {
  const info = snap.seats[seat];
  const me = seat === snap.viewer;
  const cash = snap.players[seat].cash;
  const turn = seat === snap.activePlayer && !snap.gameOver;
  const deciding = !turn && seat === snap.currentPlayer && !snap.gameOver;
  const className = ["m-chip", on && "m-chip-on", turn && "m-chip-turn", deciding && "m-chip-deciding", attn && "m-chip-attn"]
    .filter(Boolean)
    .join(" ");
  return (
    <button
      className={className}
      aria-pressed={on}
      aria-label={`${info.name}${me ? " (you)" : ""}${turn ? ", taking a turn" : deciding ? ", deciding" : ""}`}
      onClick={onClick}
    >
      <span className="m-avatar" style={{ background: seatColor(seat) }}>
        {info.kind === "human" ? info.name.charAt(0).toUpperCase() : <Bot size={15} />}
      </span>
      <span className={me ? "m-chip-label m-chip-cash" : "m-chip-label"}>{me && cash !== null ? money(cash) : info.name}</span>
    </button>
  );
}

function PlaceChip({ icon, label, on, attn, badge, onClick }: { icon: ReactNode; label: string; on: boolean; attn: boolean; badge?: number; onClick: () => void }) {
  const className = ["m-chip", on && "m-chip-on", attn && "m-chip-attn"].filter(Boolean).join(" ");
  return (
    <button className={className} aria-pressed={on} onClick={onClick}>
      <span className="m-avatar m-avatar-place">{icon}</span>
      <span className="m-chip-label">{label}</span>
      {badge ? <i className="m-badge" aria-label={`${badge} live auction${badge === 1 ? "" : "s"}`}>{badge}</i> : null}
    </button>
  );
}

/** The secret scoring card, kept at hand wherever the player looks. */
function CardRow({ snap }: { snap: Snapshot }) {
  const ten = snap.players[snap.viewer].scoringCard;
  if (ten === null) return null;
  const order = [0, 1, 2, 3, 4].sort((a, b) => cardValue(ten, b, true) - cardValue(ten, a, true));
  return (
    <div className="m-card">
      <span className="m-card-label">YOUR CARD</span>
      {order.map((color) => (
        <span key={color} className="m-card-value">
          <Swatch color={color} size={11} />
          {money(cardValue(ten, color, true))}
          {cardValue(ten, color, true) !== cardValue(ten, color, false) && <small>/{cardValue(ten, color, false)}</small>}
        </span>
      ))}
    </div>
  );
}

function SpeedButton({ speed, setSpeed }: { speed: Speed; setSpeed: (speed: Speed) => void }) {
  const next = SPEEDS[(SPEEDS.indexOf(speed) + 1) % SPEEDS.length];
  return (
    <button className="m-speed" onClick={() => setSpeed(next)} aria-label={`Opponent speed ${speed}. Switch to ${next}`}>
      <Gauge size={15} /> {speed}
    </button>
  );
}

function EventsSheet({ snap, onClose }: { snap: Snapshot; onClose: () => void }) {
  const names = snap.seats.map((s) => s.name);
  const [open, setOpen] = useState<number | null>(null);
  return (
    <Sheet title="Events" onClose={onClose} stickToEnd>
      <div className="m-log">
        {snap.log.map((entry, i) => (
          <div key={i}>
            {(i === 0 || snap.log[i - 1].round !== entry.round) && <div className="m-log-round">ROUND {entry.round}</div>}
            <button
              className={open === i ? "m-log-entry m-log-open" : "m-log-entry"}
              aria-expanded={open === i}
              onClick={() => setOpen(open === i ? null : i)}
            >
              <span className="m-log-text">
                <i style={{ background: entry.player === null ? "transparent" : seatColor(entry.player) }} />
                <span><RichText text={entry.text} names={names} viewer={snap.viewer} /></span>
              </span>
              {open === i && (
                <span className="m-log-detail">
                  {entry.detail.length === 0 && <span>No other changes.</span>}
                  {entry.detail.map((line, n) => (
                    <span key={n}><RichText text={line} names={names} viewer={snap.viewer} /></span>
                  ))}
                </span>
              )}
            </button>
          </div>
        ))}
      </div>
    </Sheet>
  );
}

function MenuSheet({
  snap,
  speed,
  setSpeed,
  onClose,
  onHelp,
  onLeave,
}: {
  snap: Snapshot;
  speed: Speed;
  setSpeed: (speed: Speed) => void;
  onClose: () => void;
  onHelp: () => void;
  onLeave: () => void;
}) {
  const [sure, setSure] = useState(false);
  return (
    <Sheet title="Menu" onClose={onClose}>
      <div className="m-menu">
        <SupplyPanel snap={snap} />
        <ScoringCard snap={snap} />

        <section>
          <h3 className="m-menu-title">OPPONENT SPEED</h3>
          <div className="m-seg" role="group" aria-label="Opponent speed">
            {SPEEDS.map((s) => (
              <button key={s} className={speed === s ? "on" : ""} aria-pressed={speed === s} onClick={() => setSpeed(s)}>
                {s}
              </button>
            ))}
          </div>
        </section>

        <div className="m-menu-actions">
          <button className="m-menu-button" onClick={onHelp}>How to play</button>
          <details className="m-rule m-rules-group">
            <summary>Rules reference</summary>
            <div className="m-rules">
              {Object.values(RULES).map((rule) => (
                <details key={rule.title} className="m-rule">
                  <summary>{rule.title}</summary>
                  <ul>
                    {rule.lines.map((line, i) => (
                      <li key={i}>{line}</li>
                    ))}
                  </ul>
                </details>
              ))}
            </div>
          </details>
          <button className={sure ? "m-menu-button m-danger" : "m-menu-button"} onClick={() => (sure ? onLeave() : setSure(true))}>
            {sure ? "Tap again to leave" : "Leave game"}
          </button>
        </div>
      </div>
    </Sheet>
  );
}
