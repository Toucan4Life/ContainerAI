import { useEffect, useState } from "react";
import { ArrowRight, Bot, CircleHelp, Plus, Ship, Sparkles, X } from "lucide-react";
import { GameConfig, OpponentInfo, ensureNets, getOpponents } from "./engine";
import { seatColor } from "./ui";

const BOT_NAMES = ["Mara", "Otto", "Ines", "Kofi"];
// The bots in the menu, weakest first, with the line shown under each picker. The engine's full
// description of a bot is the tooltip of that line. The other bots stay in the engine, out of the menu.
// The id `consortium` is the net shown as "Banker".
const MENU_OPPONENTS: Record<string, string> = {
  solitaire: "The solo variant's bot: a d20 decides prices and buys.",
  shipper: "Lives off deliveries: buys cheap, sails when it pays.",
  showcase: "Stocks its harbor with many colors and resells them.",
  producer: "Makes the most containers and sells them cheap, from $1.",
  consortium: "Works the Bank, builds nothing, lists its stock at top prices.",
};
// The seat a new opponent starts as: the first, weakest bot of the menu.
const DEFAULT_OPPONENT = Object.keys(MENU_OPPONENTS)[0];
const LENGTHS: { id: GameConfig["length"]; label: string }[] = [
  { id: "short", label: "Short" },
  { id: "standard", label: "Standard" },
  { id: "extended", label: "Extended" },
];

type Props = { onStart: (config: GameConfig) => Promise<void>; onHelp: () => void; error: string | null };

export default function Setup({ onStart, onHelp, error }: Props) {
  const [opponents, setOpponents] = useState<OpponentInfo[] | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [name, setName] = useState("You");
  const [kinds, setKinds] = useState<string[]>([DEFAULT_OPPONENT, DEFAULT_OPPONENT]);
  const [length, setLength] = useState<GameConfig["length"]>("standard");
  const [seed, setSeed] = useState("");
  const [starting, setStarting] = useState(false);

  useEffect(() => {
    getOpponents().then(
      (available) => setOpponents(available.filter((opponent) => opponent.id in MENU_OPPONENTS)),
      (e) => setLoadError(String(e)),
    );
  }, []);

  const start = async () => {
    const parsed = Number.parseInt(seed, 10);
    setStarting(true);
    try {
      await onStart({
        seed: Number.isFinite(parsed) && parsed >= 0 ? parsed : Math.floor(Math.random() * 2 ** 32),
        length,
        players: [
          { name: name.trim() || "You", kind: "human" },
          ...kinds.map((kind, i) => ({ name: BOT_NAMES[i], kind })),
        ],
      });
    } finally {
      setStarting(false);
    }
  };

  const setKind = (index: number, kind: string) => {
    // Fetch the opponent's net now, so that starting the game does not wait for it. A failure is
    // reported when the game starts.
    ensureNets([kind]).catch(() => {});
    setKinds((current) => current.map((k, i) => (i === index ? kind : k)));
  };

  return (
    <main className="setup-page">
      <div className="setup-brand">
        <span className="brand-mark"><Ship size={20} /></span> CONTAINER
      </div>
      <section className="setup-card">
        <div className="eyebrow"><Sparkles size={15} /> DIGITAL TABLE</div>
        <h1>Build your fleet.<br /><em>Own the harbor.</em></h1>
        <p className="setup-copy">A strategic game of production, shipping, and ruthless negotiation.</p>
        <div className="setup-rule" />

        <label className="field-label" htmlFor="player-name">PLAYERS</label>
        <div className="seat-list">
          <div className="seat-row">
            <span className="avatar" style={{ background: seatColor(0) }}>
              {name.trim().charAt(0).toUpperCase() || "Y"}
            </span>
            <input
              id="player-name"
              className="name-input"
              value={name}
              maxLength={14}
              onChange={(e) => setName(e.target.value)}
              aria-label="Your name"
            />
            <span className="seat-tag">HUMAN</span>
            <span className="icon-spacer" />
          </div>
          {kinds.map((kind, index) => (
            <div className="seat-item" key={index}>
              <div className="seat-row">
                <span className="avatar" style={{ background: seatColor(index + 1) }}><Bot size={15} /></span>
                <strong>{BOT_NAMES[index]}</strong>
                <select
                  className="select"
                  value={kind}
                  onChange={(e) => setKind(index, e.target.value)}
                  aria-label={`Opponent ${BOT_NAMES[index]}`}
                  aria-describedby={`seat-note-${index}`}
                >
                  {(opponents ?? [{ id: kind, name: kind, description: "", elo: 0, net: null }]).map((o) => (
                    <option key={o.id} value={o.id}>{o.name} · ELO {o.elo}</option>
                  ))}
                </select>
                {index === kinds.length - 1 && kinds.length > 2 ? (
                  <button
                    className="icon-button"
                    aria-label={`Remove ${BOT_NAMES[index]}`}
                    onClick={() => setKinds((c) => c.slice(0, -1))}
                  >
                    <X size={16} />
                  </button>
                ) : (
                  <span className="icon-spacer" />
                )}
              </div>
              <p
                id={`seat-note-${index}`}
                className="seat-note"
                title={opponents?.find((o) => o.id === kind)?.description}
              >
                {MENU_OPPONENTS[kind]}
              </p>
            </div>
          ))}
          {kinds.length < 4 && (
            <button className="add-seat" onClick={() => setKinds((c) => [...c, DEFAULT_OPPONENT])}>
              <Plus size={16} /> Add opponent <span>({kinds.length + 1}/5)</span>
            </button>
          )}
        </div>

        <div className="setup-options">
          <div>
            <span>GAME LENGTH</span>
            <div className="segmented">
              {LENGTHS.map((l) => (
                <button key={l.id} className={length === l.id ? "on" : ""} onClick={() => setLength(l.id)}>
                  {l.label}
                </button>
              ))}
            </div>
          </div>
          <div>
            <span>SEED</span>
            <input
              className="seed-input"
              value={seed}
              inputMode="numeric"
              placeholder="Random"
              onChange={(e) => setSeed(e.target.value.replace(/\D/g, "").slice(0, 9))}
              aria-label="Seed"
            />
          </div>
        </div>

        {(error || loadError) && <p className="error-text">{error ?? loadError}</p>}
        <button className="primary-button" onClick={start} disabled={!opponents || starting}>
          {!opponents ? "Loading engine…" : starting ? "Loading opponent…" : "Set sail"} <ArrowRight size={18} />
        </button>
      </section>
      <button className="setup-footer" onClick={onHelp}>
        <CircleHelp size={14} /> How to play <span>·</span> {kinds.length + 1} players
      </button>
    </main>
  );
}
