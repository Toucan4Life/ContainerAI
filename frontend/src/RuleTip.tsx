import { useEffect, useState } from "react";
import { RULES, RuleKey } from "./rules";

type Tip = { key: RuleKey; note?: string; top: number; left: number };

const WIDTH = 320;
const DELAY = 600;

/** Shows the rules of whatever element carries `data-rule` under the pointer. */
export default function RuleTip() {
  const [tip, setTip] = useState<Tip | null>(null);

  useEffect(() => {
    let timer: number | undefined;
    let shown: HTMLElement | null = null;
    const show = (target: EventTarget | null) => {
      const el = target instanceof Element ? target.closest<HTMLElement>("[data-rule]") : null;
      if (el === shown) return;
      shown = el;
      window.clearTimeout(timer);
      setTip(null);
      if (el) timer = window.setTimeout(() => open(el), DELAY);
    };
    const open = (el: HTMLElement) => {
      const key = el.dataset.rule as RuleKey | undefined;
      if (!key || !(key in RULES) || !el.isConnected) return;
      const box = el.getBoundingClientRect();
      const left = Math.min(Math.max(8, box.left), window.innerWidth - WIDTH - 8);
      const below = box.bottom + 8;
      const top = below + 200 > window.innerHeight ? Math.max(8, box.top - 8 - 240) : below;
      setTip({ key, note: el.dataset.ruleNote, top, left });
    };
    const over = (e: MouseEvent) => show(e.target);
    // Tapping a control focuses it on touch screens, which must not pop a tip up.
    const focus = (e: FocusEvent) => window.matchMedia("(hover: hover)").matches && show(e.target);
    const hide = () => {
      window.clearTimeout(timer);
      shown = null;
      setTip(null);
    };
    document.addEventListener("mouseover", over);
    document.addEventListener("focusin", focus);
    document.addEventListener("focusout", hide);
    document.addEventListener("scroll", hide, true);
    document.addEventListener("mousedown", hide);
    return () => {
      window.clearTimeout(timer);
      document.removeEventListener("mouseover", over);
      document.removeEventListener("focusin", focus);
      document.removeEventListener("focusout", hide);
      document.removeEventListener("scroll", hide, true);
      document.removeEventListener("mousedown", hide);
    };
  }, []);

  if (!tip) return null;
  const rule = RULES[tip.key];
  return (
    <div className="rule-tip" role="tooltip" style={{ top: tip.top, left: tip.left, width: WIDTH }}>
      <b>{rule.title}</b>
      {tip.note && <em>{tip.note}</em>}
      <ul>
        {rule.lines.map((line, i) => (
          <li key={i}>{line}</li>
        ))}
      </ul>
    </div>
  );
}
