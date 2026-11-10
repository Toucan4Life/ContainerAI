import { useEffect, useState } from "react";
import { ArrowRight, Bot, CircleHelp, Plus, Ship, Sparkles, X } from "lucide-react";
import { GameConfig, OpponentInfo, getOpponents } from "./engine";
import { seatColor } from "./ui";

const BOT_NAMES = ["Mara", "Otto", "Ines", "Kofi"];
const LENGTHS: { id: GameConfig["length"]; label: string }[] = [
  { id: "short", label: "Short" },
  { id: "standard", label: "Standard" },
  { id: "extended", label: "Extended" },
];

type Props = { onStart: (config: GameConfig) => void; onHelp: () => void; error: string | null };

export default function Setup({ onStart, onHelp, error }: Props) {
  const [opponents, setOpponents] = useState<OpponentInfo[] | null>(null);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [name, setName] = useState("You");
  const [kinds, setKinds] = useState<string[]>(["simple", "simple"]);
  const [length, setLength] = useState<GameConfig["length"]>("standard");
  const [seed, setSeed] = useState("");

  useEffect(() => {
    getOpponents().then(setOpponents, (e) => setLoadError(String(e)));
  }, []);

  const start = () => {
    const parsed = Number.parseInt(seed, 10);
    onStart({
      seed: Number.isFinite(parsed) && parsed >= 0 ? parsed : Math.floor(Math.random() * 2 ** 32),
      length,
      players: [
        { name: name.trim() || "You", kind: "human" },
        ...kinds.map((kind, i) => ({ name: BOT_NAMES[i], kind })),
      ],
    });
  };

  const setKind = (index: number, kind: string) =>
    setKinds((current) => current.map((k, i) => (i === index ? kind : k)));

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
            <div className="seat-row" key={index}>
              <span className="avatar" style={{ background: seatColor(index + 1) }}><Bot size={15} /></span>
              <strong>{BOT_NAMES[index]}</strong>
              <select
                className="select"
                value={kind}
                onChange={(e) => setKind(index, e.target.value)}
                aria-label={`Opponent ${BOT_NAMES[index]}`}
              >
                {(opponents ?? [{ id: kind, name: kind, description: "" }]).map((o) => (
                  <option key={o.id} value={o.id}>{o.name}</option>
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
          ))}
          {kinds.length < 4 && (
            <button className="add-seat" onClick={() => setKinds((c) => [...c, "simple"])}>
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
        <button className="primary-button" onClick={start} disabled={!opponents}>
          {opponents ? "Set sail" : "Loading engine…"} <ArrowRight size={18} />
        </button>
      </section>
      <button className="setup-footer" onClick={onHelp}>
        <CircleHelp size={14} /> How to play <span>·</span> {kinds.length + 1} players
      </button>
    </main>
  );
}
