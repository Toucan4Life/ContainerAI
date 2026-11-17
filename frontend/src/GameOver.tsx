import { Snapshot } from "./engine";
import { COLOR_LABELS } from "./engine";
import { money, seatColor } from "./ui";

type Props = { snap: Snapshot; onAgain: () => void };

export default function GameOver({ snap, onAgain }: Props) {
  const results = snap.results;
  if (!results) return null;
  const order = results.scores
    .map((score, seat) => ({ score, seat }))
    .sort((a, b) => b.score.total - a.score.total);
  const winners = results.winners.map((s) => snap.seats[s].name).join(" & ");
  const youWon = results.winners.includes(snap.viewer) && snap.seats[snap.viewer].kind === "human";
  return (
    <div className="modal-backdrop">
      <div className="modal" role="dialog" aria-label="Game over">
        <span className="eyebrow">GAME OVER</span>
        <h2>{youWon ? `You win!` : `${winners} wins`}</h2>
        <table className="score-table">
          <thead>
            <tr><th>Player</th><th>Cash</th><th>Island</th><th>Leftovers</th><th>Loans</th><th>Total</th></tr>
          </thead>
          <tbody>
            {order.map(({ score, seat }) => (
              <tr key={seat} className={results.winners.includes(seat) ? "winner-row" : ""}>
                <td>
                  <i className="dot" style={{ background: seatColor(seat) }} />
                  {snap.seats[seat].name}
                  {score.discarded !== null && (
                    <small> · discarded {COLOR_LABELS[score.discarded].toLowerCase()}</small>
                  )}
                </td>
                <td data-label="Cash">{money(score.cash)}</td>
                <td data-label="Island">{money(score.island)}</td>
                <td data-label="Leftovers">{money(score.leftovers)}</td>
                <td data-label="Loans">{money(score.loans)}</td>
                <td data-label="Total"><strong>{money(score.total)}</strong></td>
              </tr>
            ))}
          </tbody>
        </table>
        <button className="primary-button" onClick={onAgain}>Play again</button>
      </div>
    </div>
  );
}
