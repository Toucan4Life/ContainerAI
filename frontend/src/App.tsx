import { useState } from "react";
import GameScreen from "./Game";
import GameOver from "./GameOver";
import Help from "./Help";
import Setup from "./Setup";
import { useGame } from "./useGame";

export default function App() {
  const { snap, act, undo, stepBack, start, leave, speed, setSpeed, error, dismissError } = useGame();
  const [help, setHelp] = useState(false);

  return (
    <>
      {snap ? (
        <GameScreen
          snap={snap}
          act={act}
          undo={undo}
          stepBack={stepBack}
          speed={speed}
          setSpeed={setSpeed}
          onLeave={leave}
          onHelp={() => setHelp(true)}
        />
      ) : (
        <Setup onStart={start} onHelp={() => setHelp(true)} error={error} />
      )}
      {snap?.gameOver && <GameOver snap={snap} onAgain={leave} />}
      {snap && error && (
        <div className="error-banner" role="alert" onClick={dismissError}>{error}</div>
      )}
      {help && <Help onClose={() => setHelp(false)} />}
    </>
  );
}
