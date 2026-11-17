import { ReactNode, useState } from "react";
import {
  Anchor,
  ArrowRight,
  ChevronDown,
  Coins,
  Factory,
  Gavel,
  Hammer,
  Landmark,
  LoaderCircle,
  Package,
  Shuffle,
  Ship,
  Undo2,
  Warehouse,
} from "lucide-react";
import {
  Action,
  COLOR_LABELS,
  Counts,
  LOT_NUMERALS,
  LegalAction,
  Phase,
  Snapshot,
  lotPrice,
  factoryCost,
  FACTORY_LOTS,
  HARBOR_LOTS,
  warehouseCost,
  total,
} from "./engine";
import { Chips, Swatch, money, seatColor } from "./ui";
import { RuleKey } from "./rules";

type Act = (id: number) => void;
type Props = { snap: Snapshot; act: Act };

type Of<T extends Action["type"]> = Extract<LegalAction, { type: T }>;

const of = <T extends Action["type"]>(snap: Snapshot, type: T): Of<T>[] =>
  snap.legal.filter((a): a is Of<T> => a.type === type);

function ActButton({
  label,
  sub,
  icon,
  onClick,
  primary,
  disabled,
  title,
  compact,
  rule,
}: {
  label: ReactNode;
  sub?: ReactNode;
  icon?: ReactNode;
  onClick?: () => void;
  primary?: boolean;
  disabled?: boolean;
  title?: string;
  compact?: boolean;
  rule?: RuleKey;
}) {
  const note = title ?? (compact && typeof sub === "string" ? sub : undefined);
  return (
    <button
      className={`action-button ${primary ? "action-primary" : ""} ${compact ? "action-compact" : ""}`}
      onClick={onClick}
      disabled={disabled}
      title={rule ? undefined : note}
      data-rule={rule}
      data-rule-note={rule ? note : undefined}
    >
      {icon}
      <span>
        <strong>{label}</strong>
        {sub && !compact && <small>{sub}</small>}
      </span>
      {!disabled && !compact && <ArrowRight size={15} />}
    </button>
  );
}

/** Why no factory purchase is on offer, in the order a player would fix it. */
function buyBlockedReason(snap: Snapshot): string {
  const me = snap.players[snap.viewer];
  const stock = me.harborLots.reduce((n, lot) => n + total(lot), 0) + me.reserveHarbor;
  if (me.warehouses <= stock) return "No room in your harbor (build a warehouse)";
  const others = snap.players.filter((_, seat) => seat !== snap.viewer);
  const prices = others.flatMap((p) =>
    p.factoryLots.flatMap((lot, i) => (total(lot) > 0 ? [lotPrice("factory", i)] : [])),
  );
  if (prices.length === 0) return "Opponents have nothing in their factory lots";
  if (me.cash !== null && Math.min(...prices) > me.cash) return "Not enough cash for the cheapest container";
  return "Nothing to buy right now";
}

function Fold({ title, count, children }: { title: string; count: number; children: ReactNode }) {
  if (count === 0) return null;
  return (
    <details className="fold">
      <summary>
        {title} <span className="fold-count">{count}</span>
      </summary>
      <div className="action-buttons">{children}</div>
    </details>
  );
}

function Group({ title, children }: { title: string; children: ReactNode }) {
  return (
    <div className="action-group">
      <div className="group-title">{title}</div>
      <div className="action-buttons">{children}</div>
    </div>
  );
}

function Shell({
  snap,
  title,
  rule,
  hint,
  children,
}: {
  snap: Snapshot;
  title: string;
  rule?: RuleKey;
  hint?: ReactNode;
  children?: ReactNode;
}) {
  const cash = snap.players[snap.viewer].cash;
  return (
    <section className="action-panel">
      <div className="panel-heading" data-rule={rule}>
        <span>{title}</span>
        <small>{cash !== null && `YOUR CASH ${money(cash)}`}</small>
      </div>
      {hint && <p className="panel-hint">{hint}</p>}
      {children}
    </section>
  );
}

function LoanButtons({ snap, act }: Props) {
  const request = of(snap, "RequestLoan")[0];
  const repay = of(snap, "RepayLoan")[0];
  if (!request && !repay) return null;
  return (
    <Group title="Money · free">
      {request && (
        <ActButton
          compact
          icon={<Landmark size={15} />}
          rule="loans"
          label="Take $10 loan"
          sub="Each loan costs $11 at the end"
          onClick={() => act(request.id)}
        />
      )}
      {repay && (
        <ActButton
          compact
          icon={<Landmark size={15} />}
          rule="loans"
          label="Repay a loan"
          sub="Pay $10 back to the supply"
          onClick={() => act(repay.id)}
        />
      )}
    </Group>
  );
}

export default function ActionPanel({ snap, act, undo, stepBack }: Props & { undo: () => void; stepBack: () => void }) {
  if (snap.legal.length === 0) return <Waiting snap={snap} />;
  const midAction = snap.phase.type !== "Main";
  return (
    <>
      <PhasePanel snap={snap} act={act} stepBack={stepBack} />
      {snap.canUndo && snap.phase.type !== "GameOver" && (
        <button
          className="undo-button"
          onClick={undo}
          title={midAction ? "Go back to the start of this action" : "Take back your last action this turn"}
        >
          <Undo2 size={14} /> {midAction ? "Cancel this action" : "Undo last action"}
        </button>
      )}
    </>
  );
}

function PhasePanel({ snap, act, stepBack }: Props & { stepBack: () => void }) {
  const phase = snap.phase;
  switch (phase.type) {
    case "Main":
      return <MainPanel snap={snap} act={act} />;
    case "Produce":
      return <ProducePanel snap={snap} act={act} phase={phase} />;
    case "Buy":
      return <BuyPanel snap={snap} act={act} phase={phase} />;
    case "Load":
      return <LoadPanel snap={snap} act={act} />;
    case "Arrange":
      return <ArrangePanel snap={snap} act={act} phase={phase} stepBack={stepBack} />;
    case "Bid":
      return <BidPanel key={`${phase.bidder}-${phase.runoff}`} snap={snap} act={act} phase={phase} />;
    case "Decide":
      return <DecidePanel snap={snap} act={act} phase={phase} />;
    case "Seize":
      return <SeizePanel snap={snap} phase={phase} />;
    case "Distribute":
      return <DistributePanel snap={snap} act={act} phase={phase} />;
    case "BankBid":
      return <BankBidPanel key={`${phase.kind}-${phase.lot}`} snap={snap} act={act} phase={phase} />;
    case "GameOver":
      return null;
  }
}

function waitingText(snap: Snapshot): string {
  const name = (seat: number) => snap.seats[seat].name;
  const phase = snap.phase;
  switch (phase.type) {
    case "Main":
      return `${name(snap.activePlayer)} is taking a turn · ${snap.turn.actionsLeft} action${snap.turn.actionsLeft === 1 ? "" : "s"} left`;
    case "Produce":
      return `${name(snap.activePlayer)} is choosing what to produce`;
    case "Buy":
      return `${name(snap.activePlayer)} is buying from ${name(phase.seller)}`;
    case "Load":
      return `${name(snap.activePlayer)} is loading the ship`;
    case "Arrange":
      return `${name(phase.owner)} is laying out containers`;
    case "Bid":
      return `${name(phase.bidder)} is bidding on ${name(phase.deliverer)}'s cargo`;
    case "Decide":
      return `${name(phase.deliverer)} is settling the auction`;
    case "Seize":
      return `${name(phase.debtor)} defaulted · the Bank is seizing containers`;
    case "Distribute":
      return `${name(phase.decider)} is dealing out containers at the Bank`;
    case "BankBid":
      return `${name(snap.activePlayer)} is bidding at the Bank`;
    case "GameOver":
      return "Game over";
  }
}

function Waiting({ snap }: { snap: Snapshot }) {
  return (
    <section className="action-panel action-waiting">
      <LoaderCircle size={18} className="spin" />
      <span
        className="waiting-dot"
        style={{ background: seatColor(snap.currentPlayer) }}
        aria-hidden
      />
      <strong>{waitingText(snap)}</strong>
    </section>
  );
}

function MainPanel({ snap, act }: Props) {
  const name = (seat: number) => snap.seats[seat].name;
  const produce = of(snap, "Produce")[0];
  const factories = of(snap, "BuildFactory");
  const warehouse = of(snap, "BuildWarehouse")[0];
  const purchases = of(snap, "FactoryPurchase");
  const harbor = of(snap, "HarborPurchase")[0];
  const sails = of(snap, "Sail");
  const reprices = of(snap, "Reprice");
  const starts = of(snap, "StartAuction");
  const outbids = of(snap, "Outbid");
  const bankBlocked = snap.turn.wonAuction
    ? "You won a Bank auction this turn, so the Bank can't be called"
    : snap.turn.bankCalled
      ? "The Bank was already called this turn"
      : "No Bank lot to bid on";
  const me = snap.players[snap.viewer];
  const ownedFactories = me.factories.filter(Boolean).length;

  const sailLabel = (to: Of<"Sail">["to"]) => {
    switch (to.at) {
      case "ocean":
        return { label: "Sail out to sea", sub: "Leave the dock" };
      case "island":
        return { label: "Deliver to the Island", sub: "Auction the cargo to your opponents" };
      case "bank":
        return { label: "Sail to the Bank", sub: "Load winnings and bid" };
      case "harbor":
        return { label: `Sail to ${name(to.owner)}'s harbor`, sub: "Buy their harbor containers" };
    }
  };

  const harborInBuy = harbor && me.ship.at !== "bank" ? harbor : undefined;
  const harborLabel =
    me.ship.at === "bank"
      ? { label: "Load from holding area", sub: "Free: move Bank winnings onto the ship" }
      : {
          label: me.ship.at === "harbor" ? `Buy at ${name(me.ship.owner)}'s harbor` : "Harbor purchase",
          sub: "Buy containers onto your ship",
        };

  const auctionOf = (kind: "container" | "cash") => snap.bank.auctions.find((a) => a.kind === kind);

  const [open, setOpen] = useState<string | null>(null);
  const choose = (run: () => void) => {
    setOpen(null);
    run();
  };
  const pick = (id: number) => () => choose(() => act(id));
  const sub = (title: string, buttons: ReactNode) => (
    <div className="menu-sub">
      <div className="menu-sub-title">{title}</div>
      <div className="action-buttons">{buttons}</div>
    </div>
  );
  const bankAction = (a: Of<"StartAuction">) => (
    <ActButton
      compact
      rule={a.kind === "container" ? "containerLot" : "cashLot"}
      key={a.id}
      icon={<Gavel size={15} />}
      label={a.kind === "container" ? (
        <span className="lot-label">Lot {LOT_NUMERALS[a.lot]} <Chips counts={snap.bank.containerLots[a.lot]} /></span>
      ) : `Lot ${LOT_NUMERALS[a.lot]} · ${money(snap.bank.cashLots[a.lot])}`}
      sub={a.kind === "container" ? `Bid cash for container lot ${LOT_NUMERALS[a.lot]}` : `Bid containers for the ${money(snap.bank.cashLots[a.lot])} cash lot`}
      onClick={pick(a.id)}
    />
  );
  const tabs = [
    {
      key: "buy",
      label: "Buy",
      icon: <Coins size={15} />,
      count: purchases.length + (harborInBuy ? 1 : 0),
      blocked: buyBlockedReason(snap),
      options: (
        <div className="action-buttons">
          {harborInBuy && (
            <ActButton
              compact
              rule="buy"
              icon={<Anchor size={15} />}
              label={me.ship.at === "harbor" ? `${name(me.ship.owner)}'s harbor` : "Harbor"}
              sub={me.ship.at === "harbor" ? `Buy at ${name(me.ship.owner)}'s harbor onto your ship` : "Buy containers onto your ship"}
              onClick={pick(harborInBuy.id)}
            />
          )}
          {purchases.map((a) => (
            <ActButton
              compact
              rule="buy"
              key={a.id}
              icon={<Coins size={15} />}
              label={a.seller === snap.viewer ? "Mine" : name(a.seller)}
              sub={a.seller === snap.viewer ? "Your own factories" : `${name(a.seller)}'s factory district`}
              onClick={pick(a.id)}
            />
          ))}
        </div>
      ),
    },
    {
      key: "sail",
      label: "Sail",
      icon: <Ship size={15} />,
      count: sails.length,
      blocked: "Your ship cannot sail now",
      options: (
        <div className="action-buttons">
          {sails.map((a) => {
            const { label, sub } = sailLabel(a.to);
            const short = a.to.at === "harbor" ? `${name(a.to.owner)}'s harbor` : a.to.at === "island" ? "Island" : a.to.at === "bank" ? "Bank" : "Open sea";
            return (
              <ActButton compact rule="sail" key={a.id} icon={<Ship size={15} />} label={short} sub={`${label}: ${sub}`} primary={a.to.at === "island"} onClick={pick(a.id)} />
            );
          })}
        </div>
      ),
    },
    {
      key: "build",
      label: "Build",
      icon: <Factory size={15} />,
      count: factories.length + (warehouse ? 1 : 0),
      blocked: "Nothing to build",
      options: (
        <>
          {factories.length > 0 &&
            sub(
              `Factory · ${money(factoryCost(ownedFactories))}`,
              factories.map((a) => (
                <ActButton
                  compact
                  rule="factories"
                  key={a.id}
                  icon={<Swatch color={a.color} size={14} />}
                  label={COLOR_LABELS[a.color]}
                  sub={`${COLOR_LABELS[a.color]} factory. Costs ${money(factoryCost(ownedFactories))}, adds 2 storage`}
                  onClick={pick(a.id)}
                />
              )),
            )}
          {warehouse &&
            sub(
              "Warehouse",
              <ActButton
                compact
                rule="warehouses"
                icon={<Warehouse size={15} />}
                label={money(warehouseCost(me.warehouses))}
                sub={`Warehouse. Costs ${money(warehouseCost(me.warehouses))}, adds harbor storage`}
                onClick={pick(warehouse.id)}
              />,
            )}
        </>
      ),
    },
    {
      key: "bank",
      label: "Call bank",
      icon: <Gavel size={15} />,
      count: starts.length + outbids.length,
      blocked: bankBlocked,
      options: (
        <>
          {outbids.length > 0 &&
            sub(
              "Outbid",
              outbids.map((a) => {
                const live = auctionOf(a.kind);
                return (
                  <ActButton
                    compact
                    rule={a.kind === "container" ? "containerLot" : "cashLot"}
                    key={a.id}
                    icon={<Gavel size={15} />}
                    label={a.kind === "container" ? "Container lot" : "Cash lot"}
                    sub={live ? `${name(live.holder)} holds lot ${LOT_NUMERALS[live.lot]}` : undefined}
                    onClick={pick(a.id)}
                  />
                );
              }),
            )}
          {starts.some((a) => a.kind === "container") &&
            sub("Container lots · bid cash", starts.filter((a) => a.kind === "container").map(bankAction))}
          {starts.some((a) => a.kind === "cash") &&
            sub("Cash lots · bid containers", starts.filter((a) => a.kind === "cash").map(bankAction))}
        </>
      ),
    },
    {
      key: "reprice",
      label: "Reprice",
      icon: <Shuffle size={15} />,
      count: reprices.length,
      blocked: "Nothing to reprice",
      options: (
        <div className="action-buttons">
          {reprices.map((a) => (
            <ActButton
              compact
              rule="reprice"
              key={a.id}
              icon={<Shuffle size={15} />}
              label={a.district === "factory" ? "Factory" : "Harbor"}
              sub={`Lay out the ${a.district} district again`}
              onClick={pick(a.id)}
            />
          ))}
        </div>
      ),
    },
  ];
  const openTab = tabs.find((t) => t.key === open);

  return (
    <Shell
      snap={snap}
      title="YOUR TURN"
      rule="turn"
      hint={
        <>
          <span className="pips">
            {[0, 1].map((i) => (
              <i key={i} className={i < snap.turn.actionsLeft ? "pip-on" : ""} />
            ))}
          </span>
          {snap.turn.actionsLeft} action{snap.turn.actionsLeft === 1 ? "" : "s"} left. Loans are free.
        </>
      }
    >
      <div className="action-groups">
        <div className="menu-tabs">
          {produce ? (
            <ActButton compact primary rule="produce" icon={<Package size={15} />} label="Produce" sub="Pay $1 wages, take containers from your factories" onClick={() => choose(() => act(produce.id))} />
          ) : (
            <ActButton compact disabled rule="produce" icon={<Package size={15} />} label="Produce" sub={snap.turn.produced ? "Already produced this turn" : "No room or nothing to produce"} />
          )}
          {harbor && !harborInBuy && (
            <ActButton compact rule="buy" icon={<Anchor size={15} />} label={harborLabel.label} sub={harborLabel.sub} onClick={() => choose(() => act(harbor.id))} />
          )}
          {tabs.map((t) => (
            <button
              key={t.key}
              className={`action-button action-compact menu-tab ${open === t.key ? "menu-tab-open" : ""}`}
              disabled={t.count === 0}
              data-rule={t.key}
              data-rule-note={t.count === 0 ? t.blocked : undefined}
              onClick={() => setOpen(open === t.key ? null : t.key)}
            >
              {t.icon}
              <span><strong>{t.label}</strong></span>
              {t.count > 0 && <ChevronDown size={13} className="menu-chevron" />}
            </button>
          ))}
        </div>
        {openTab && openTab.count > 0 && (
          <div className="menu-options">
            <div className="group-title">{openTab.label}</div>
            {openTab.options}
          </div>
        )}

        <LoanButtons snap={snap} act={act} />
      </div>
    </Shell>
  );
}

function ProducePanel({ snap, act, phase }: Props & { phase: Extract<Phase, { type: "Produce" }> }) {
  const options = of(snap, "ProduceColor");
  const picked = phase.picked.flatMap((n, color) => Array<number>(n).fill(color));
  const waiting = phase.candidates
    .map((n, color) => ({ n, color }))
    .filter(({ n, color }) => n > 0 && !options.some((o) => o.color === color) && !phase.picked[color]);
  return (
    <Shell
      snap={snap}
      title="PRODUCE"
      rule="produce"
      hint={`Your factories can make more than fits. Choose ${phase.toPick} more container${phase.toPick === 1 ? "" : "s"}.`}
    >
      {picked.length > 0 && (
        <div className="hand-row">
          <span className="group-title">Picked</span>
          {picked.map((c, i) => <Swatch key={i} color={c} size={18} />)}
        </div>
      )}
      <div className="action-buttons">
        {options.map((a) => (
          <ActButton
            key={a.id}
            icon={<Swatch color={a.color} size={18} />}
            label={COLOR_LABELS[a.color]}
            sub={`${phase.candidates[a.color]} available`}
            onClick={() => act(a.id)}
          />
        ))}
        {waiting.map(({ color }) => (
          <ActButton
            key={color}
            disabled
            icon={<Swatch color={color} size={18} />}
            label={COLOR_LABELS[color]}
            sub="Pick it after another color"
            title="Colors are picked in order, so this one is offered once you have picked an earlier color."
          />
        ))}
      </div>
    </Shell>
  );
}

function BuyPanel({ snap, act, phase }: Props & { phase: Extract<Phase, { type: "Buy" }> }) {
  const done = of(snap, "DoneBuying")[0];
  const seller = snap.seats[phase.seller].name;
  return (
    <Shell
      snap={snap}
      title="BUYING"
      rule="buy"
      hint={`Click containers in ${seller}'s ${phase.district} district to buy them. The price is shown on each lot.${phase.optional ? " You may buy nothing." : ""}`}
    >
      <div className="panel-row">
        <div>
          <div className="group-title">Bought so far</div>
          <div className="container-stack"><Chips counts={phase.bought} empty={<span className="empty-lot">Nothing yet</span>} /></div>
        </div>
        {done && (
          <ActButton primary label="Done buying" sub={`${total(phase.bought)} container${total(phase.bought) === 1 ? "" : "s"}`} onClick={() => act(done.id)} />
        )}
      </div>
    </Shell>
  );
}

function LoadPanel({ snap, act }: Props) {
  const done = of(snap, "DoneLoading")[0];
  const holding = snap.players[snap.viewer].holding;
  return (
    <Shell snap={snap} title="LOADING" rule="holding" hint="Move containers from your holding area onto the ship. Loading is free.">
      <div className="action-buttons">
        {of(snap, "Load").map((a) => (
          <ActButton
            key={a.id}
            icon={<Swatch color={a.color} size={18} />}
            label={`Load ${COLOR_LABELS[a.color].toLowerCase()}`}
            sub={`${holding[a.color]} in holding`}
            onClick={() => act(a.id)}
          />
        ))}
        {done && <ActButton primary label="Done loading" onClick={() => act(done.id)} />}
      </div>
    </Shell>
  );
}

function ArrangePanel({ snap, act, phase, stepBack }: Props & { stepBack: () => void; phase: Extract<Phase, { type: "Arrange" }> }) {
  const next = phase.hand.findIndex((n) => n > 0);
  const places = of(snap, "Place");
  return (
    <Shell
      snap={snap}
      title="LAY OUT CONTAINERS"
      rule="reprice"
      hint={
        next >= 0
          ? `Place the next container (${COLOR_LABELS[next].toLowerCase()}) in a ${phase.district} lot. Cheaper lots sell first.`
          : undefined
      }
    >
      <div className="hand-row">
        <span className="group-title">In hand</span>
        <Chips counts={phase.hand} />
      </div>
      <div className="lot-picker">
        {Array.from({ length: phase.district === "factory" ? FACTORY_LOTS : HARBOR_LOTS }, (_, lot) => {
          const a = places.find((p) => p.lot === lot);
          const price = money(lotPrice(phase.district, lot));
          return (
            <button
              key={lot}
              className="lot-choice"
              disabled={!a}
              title={
                a
                  ? `${phase.district === "factory" ? "Factory" : "Harbor"} district, ${price} lot`
                  : `A ${COLOR_LABELS[next].toLowerCase()} container must go in a lot at or above the one used by your previous ${COLOR_LABELS[next].toLowerCase()} container`
              }
              onClick={a ? () => act(a.id) : undefined}
            >
              {next >= 0 && <Swatch color={next} size={12} />}
              <strong>{price}</strong>
            </button>
          );
        })}
      </div>
      {snap.canStepBack && (
        <button className="undo-button" onClick={stepBack}>
          <Undo2 size={14} /> Undo last placement
        </button>
      )}
    </Shell>
  );
}

function AmountInput({
  min,
  max,
  value,
  onChange,
}: {
  min: number;
  max: number;
  value: number;
  onChange: (value: number) => void;
}) {
  const clamp = (n: number) => Math.min(max, Math.max(min, Number.isFinite(n) ? n : min));
  return (
    <div className="amount">
      <input
        type="range"
        min={min}
        max={max}
        value={value}
        onChange={(e) => onChange(clamp(Number(e.target.value)))}
        aria-label="Amount"
      />
      <input
        className="amount-number"
        type="number"
        min={min}
        max={max}
        value={value}
        onChange={(e) => onChange(clamp(Number(e.target.value)))}
        aria-label="Amount in dollars"
      />
    </div>
  );
}

function BidPanel({ snap, act, phase }: Props & { phase: Extract<Phase, { type: "Bid" }> }) {
  const bids = of(snap, "Bid");
  const max = bids.length ? Math.max(...bids.map((a) => a.amount)) : 0;
  const [amount, setAmount] = useState(Math.min(max, Math.round(max / 4)));
  const deliverer = snap.seats[phase.deliverer];
  const cargo = snap.players[phase.deliverer].cargo;
  const chosen = bids.find((a) => a.amount === amount);
  return (
    <Shell
      snap={snap}
      title={phase.runoff ? "RUNOFF BID" : "DELIVERY AUCTION"}
      rule="delivery"
      hint={
        <>
          {deliverer.name} delivered {total(cargo)} container{total(cargo) === 1 ? "" : "s"}. Bids are blind and final;
          everyone's bids are revealed together. The highest bid wins.
          {phase.runoff && ` You are tied at ${money(phase.base)}; what you add goes on top.`}
        </>
      }
    >
      <div className="panel-row">
        <div>
          <div className="group-title">Cargo</div>
          <div className="container-stack"><Chips counts={cargo} /></div>
        </div>
        <div className="grow">
          <div className="group-title">Your bid{phase.runoff ? " (added)" : ""}</div>
          <AmountInput min={0} max={max} value={amount} onChange={setAmount} />
        </div>
        <ActButton
          primary
          label={`Bid ${money(amount)}`}
          sub="Hidden until all bids are in"
          disabled={!chosen}
          onClick={() => chosen && act(chosen.id)}
        />
      </div>
      <LoanButtons snap={snap} act={act} />
    </Shell>
  );
}

function DecidePanel({ snap, act, phase }: Props & { phase: Extract<Phase, { type: "Decide" }> }) {
  const accepts = of(snap, "Accept");
  const buyout = of(snap, "BuyOut")[0];
  const tiedCount = phase.tied.filter(Boolean).length;
  return (
    <Shell
      snap={snap}
      title="SETTLE THE AUCTION"
      rule="delivery"
      hint={`The bids are revealed. The highest is ${money(phase.top)}${tiedCount > 1 ? `, tied between ${tiedCount} players: you choose the winner` : ""}. Sell the cargo and you collect ${money(phase.top)} from the buyer plus ${money(phase.top)} from the Bank (${money(2 * phase.top)} in all), or pay the bid to the Bank and keep it.`}
    >
      <div className="action-buttons">
        {accepts.map((a) => (
          <ActButton
            key={a.id}
            primary
            icon={<Coins size={17} />}
            label={`Sell to ${snap.seats[a.player].name}`}
            sub={`You receive ${money(2 * phase.top)}: ${money(phase.top)} from ${snap.seats[a.player].name} + ${money(phase.top)} from the Bank`}
            onClick={() => act(a.id)}
          />
        ))}
        {buyout && (
          <ActButton
            icon={<Package size={17} />}
            label="Buy out"
            sub={`Pay ${money(phase.top)} to the Bank, keep the cargo`}
            onClick={() => act(buyout.id)}
          />
        )}
        {!buyout && (
          <ActButton disabled label="Buy out" sub={`You need ${money(phase.top)} (take a loan?)`} />
        )}
      </div>
      <LoanButtons snap={snap} act={act} />
    </Shell>
  );
}

const PLACE_NAMES = {
  island: "Island",
  ship: "ship",
  holding: "holding area",
  harbor: "harbor district",
  factory: "factory district",
};

function SeizePanel({ snap, phase }: { snap: Snapshot; phase: Extract<Phase, { type: "Seize" }> }) {
  const debtor = snap.seats[phase.debtor].name;
  return (
    <Shell
      snap={snap}
      title="DEFAULT"
      rule="seize"
      hint={`${debtor} could not pay interest, so the Bank seizes ${phase.remaining} more container${phase.remaining === 1 ? "" : "s"} from their ${PLACE_NAMES[phase.place]}. As the player on their right, you choose: click containers on their board.`}
    >
      <div className="panel-row">
        <div>
          <div className="group-title">Seized so far</div>
          <div className="container-stack"><Chips counts={phase.seized} empty={<span className="empty-lot">Nothing yet</span>} /></div>
        </div>
      </div>
    </Shell>
  );
}

function DistributePanel({ snap, act, phase }: Props & { phase: Extract<Phase, { type: "Distribute" }> }) {
  return (
    <Shell
      snap={snap}
      title="DEAL AT THE BANK"
      rule="bankZone"
      hint={`Put one container in each Bank container lot, starting with lot ${LOT_NUMERALS[phase.dealt] ?? "I"}. You choose the color.`}
    >
      <div className="action-buttons">
        {of(snap, "Deal").map((a) => (
          <ActButton
            key={a.id}
            icon={<Swatch color={a.color} size={18} />}
            label={`Deal ${COLOR_LABELS[a.color].toLowerCase()}`}
            sub={`Into lot ${LOT_NUMERALS[phase.dealt] ?? "I"} · ${phase.hand[a.color]} in hand`}
            onClick={() => act(a.id)}
          />
        ))}
      </div>
    </Shell>
  );
}

function BankBidPanel({ snap, act, phase }: Props & { phase: Extract<Phase, { type: "BankBid" }> }) {
  const cash = of(snap, "BidCash");
  const max = cash.length ? Math.max(...cash.map((a) => a.amount)) : phase.min;
  const [amount, setAmount] = useState(phase.min);
  const done = of(snap, "DoneBidding")[0];
  const chosen = cash.find((a) => a.amount === amount);
  const picked = phase.picked.factory.map((n, i) => n + phase.picked.harbor[i]) as Counts;

  if (phase.kind === "container") {
    return (
      <Shell
        snap={snap}
        title={`BID FOR CONTAINER LOT ${LOT_NUMERALS[phase.lot]}`}
        rule="containerLot"
        hint={`Put cash on the tile. The lowest bid is ${money(phase.min)}. You get the containers if nobody outbids you.`}
      >
        <div className="panel-row">
          <div className="grow">
            <AmountInput min={phase.min} max={max} value={amount} onChange={setAmount} />
          </div>
          <ActButton
            primary
            label={`Bid ${money(amount)}`}
            disabled={!chosen}
            onClick={() => chosen && act(chosen.id)}
          />
        </div>
      </Shell>
    );
  }
  return (
    <Shell
      snap={snap}
      title={`BID FOR CASH LOT ${LOT_NUMERALS[phase.lot]}`}
      rule="cashLot"
      hint={`Click your own containers (factory or harbor district) to put them on the tile. You need at least ${phase.min}.`}
    >
      <div className="panel-row">
        <div>
          <div className="group-title">On the tile · {total(picked)}</div>
          <div className="container-stack"><Chips counts={picked} empty={<span className="empty-lot">Nothing yet</span>} /></div>
        </div>
        {done ? (
          <ActButton primary label="Place bid" sub={`${total(picked)} containers`} onClick={() => act(done.id)} />
        ) : (
          <ActButton disabled label="Place bid" sub={`Needs ${phase.min}`} />
        )}
      </div>
    </Shell>
  );
}
