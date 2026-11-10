import { useCallback, useEffect, useRef, useState } from "react";
import { GameClient, GameConfig, Snapshot } from "./engine";

export type Speed = "slow" | "normal" | "fast";

const DELAY: Record<Speed, number> = { slow: 800, normal: 300, fast: 40 };

/** Owns the running game and lets the computer opponents play whenever it is their turn. */
export function useGame() {
  const client = useRef<GameClient | null>(null);
  const [snap, setSnap] = useState<Snapshot | null>(null);
  const [speed, setSpeed] = useState<Speed>("normal");
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(() => {
    if (client.current) setSnap(client.current.snapshot());
  }, []);

  const start = useCallback((config: GameConfig) => {
    client.current?.dispose();
    try {
      client.current = new GameClient(config);
      setError(null);
      setSnap(client.current.snapshot());
    } catch (e) {
      client.current = null;
      setSnap(null);
      setError(String(e));
    }
  }, []);

  const leave = useCallback(() => {
    client.current?.dispose();
    client.current = null;
    setSnap(null);
  }, []);

  const act = useCallback(
    (id: number) => {
      try {
        client.current?.act(id);
        refresh();
      } catch (e) {
        setError(String(e));
      }
    },
    [refresh],
  );

  const botTurn =
    snap !== null && !snap.gameOver && snap.seats[snap.currentPlayer].kind !== "human";

  useEffect(() => {
    if (!botTurn) return;
    const timer = setTimeout(() => {
      try {
        client.current?.step();
        refresh();
      } catch (e) {
        setError(String(e));
      }
    }, DELAY[speed]);
    return () => clearTimeout(timer);
  }, [botTurn, snap, speed, refresh]);

  return { snap, act, start, leave, speed, setSpeed, error, dismissError: () => setError(null) };
}
