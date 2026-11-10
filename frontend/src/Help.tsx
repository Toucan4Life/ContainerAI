type Props = { onClose: () => void };

export default function Help({ onClose }: Props) {
  return (
    <div className="modal-backdrop" onClick={onClose}>
      <div className="modal" role="dialog" aria-label="How to play" onClick={(e) => e.stopPropagation()}>
        <span className="eyebrow">HOW TO PLAY</span>
        <h2>Container in a nutshell</h2>
        <ul className="help-list">
          <li>Each turn you get <b>2 actions</b>: produce, build, buy containers, sail, reprice, or call the Bank. Loans are free.</li>
          <li>Produce containers in your factories, then sell them from your factory and harbor districts. Opponents buy at the lot prices.</li>
          <li>Buy containers onto your ship and sail to the <b>Island</b>. Opponents bid blindly for the cargo; you sell it or buy it out.</li>
          <li>Containers on the Island score according to your <b>secret scoring card</b>. Your most common color there is discarded.</li>
          <li>Loans cost $11 at the end. If you can't pay, the Bank seizes containers.</li>
          <li>The game ends when two container colors run out. Highest total wins.</li>
          <li>When a choice involves containers, click them directly on the board.</li>
        </ul>
        <button className="primary-button" onClick={onClose}>Got it</button>
      </div>
    </div>
  );
}
