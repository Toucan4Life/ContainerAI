import { ReactNode } from "react";
import { COLORS, COLOR_LABELS, Counts } from "./engine";

export const SEAT_COLORS = ["#e86f51", "#6c8cff", "#f5b84b", "#6ed39a", "#c58bff"];

export const seatColor = (seat: number) => SEAT_COLORS[seat % SEAT_COLORS.length];

export function Swatch({ color, size = 14 }: { color: number; size?: number }) {
  return (
    <span
      className={`swatch container-${COLORS[color]}`}
      style={{ width: size, height: size }}
      title={COLOR_LABELS[color]}
    />
  );
}

type ChipsProps = {
  counts: Counts;
  /** The action id for clicking a container of this color, if it can be clicked. */
  pick?: (color: number) => number | undefined;
  onPick?: (id: number) => void;
  empty?: ReactNode;
};

/** One chip per container. A chip is a button when the player may click it. */
export function Chips({ counts, pick, onPick, empty }: ChipsProps) {
  const chips: ReactNode[] = [];
  counts.forEach((n, color) => {
    const id = pick?.(color);
    for (let i = 0; i < n; i++) {
      const className = `container container-${COLORS[color]}`;
      if (id !== undefined && onPick) {
        chips.push(
          <button
            key={`${color}-${i}`}
            className={`${className} container-pick`}
            title={`Take a ${COLOR_LABELS[color].toLowerCase()} container`}
            onClick={() => onPick(id)}
          />,
        );
      } else {
        chips.push(<span key={`${color}-${i}`} className={className} title={COLOR_LABELS[color]} />);
      }
    }
  });
  if (chips.length === 0) return <>{empty ?? <span className="empty-lot">Empty</span>}</>;
  return <>{chips}</>;
}

export const money = (n: number) => `$${n}`;

/** Draws `{c<index>}` markers as container icons and colors each player's name in their seat color. */
export function RichText({ text, names = [], viewer = -1 }: { text: string; names?: string[]; viewer?: number }) {
  const escaped = names.map((n) => n.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
  const pattern = new RegExp(`\\{c(\\d)\\}${escaped.length ? `|\\b(${escaped.join("|")})(?![\\w])` : ""}|\\b([Yy]our)\\b`, "g");
  const out: ReactNode[] = [];
  let last = 0;
  for (const m of text.matchAll(pattern)) {
    if (m.index! > last) out.push(text.slice(last, m.index));
    if (m[1] !== undefined) {
      const c = Number(m[1]);
      out.push(<span key={m.index} className={`container container-${COLORS[c]} log-icon`} title={COLOR_LABELS[c]} />);
    } else if (m[2] === undefined) {
      out.push(<b key={m.index} className="log-name" style={{ color: seatColor(viewer) }}>{m[0]}</b>);
    } else {
      const seat = names.indexOf(m[2]);
      out.push(<b key={m.index} className="log-name" style={{ color: seatColor(seat) }}>{m[2]}</b>);
    }
    last = m.index! + m[0].length;
  }
  out.push(text.slice(last));
  return <>{out}</>;
}
