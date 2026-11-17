import { ReactNode, useEffect, useRef } from "react";
import { X } from "lucide-react";

type Props = { title: string; onClose: () => void; children: ReactNode; stickToEnd?: boolean };

/** A bottom sheet for secondary content on phones: tap the backdrop, the close button, or press Esc to dismiss. */
export default function Sheet({ title, onClose, children, stickToEnd }: Props) {
  const body = useRef<HTMLDivElement>(null);
  const close = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    close.current?.focus({ preventScroll: true });
    if (stickToEnd && body.current) body.current.scrollTop = body.current.scrollHeight;
  }, [stickToEnd]);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && onClose();
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [onClose]);

  return (
    <div className="m-sheet-backdrop" onClick={onClose}>
      <section className="m-sheet" role="dialog" aria-modal="true" aria-label={title} onClick={(e) => e.stopPropagation()}>
        <div className="m-sheet-grab" aria-hidden />
        <header className="m-sheet-head">
          <h2>{title}</h2>
          <button ref={close} className="m-icon" onClick={onClose} aria-label="Close">
            <X size={20} />
          </button>
        </header>
        <div className="m-sheet-body" ref={body}>
          {children}
        </div>
      </section>
    </div>
  );
}
