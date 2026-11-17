import { Anchor, Coins, Factory, Gavel, Landmark, Mountain, Ship, Waves, Warehouse } from "lucide-react";
import {
  Counts,
  District,
  LOT_NUMERALS,
  PlayerView,
  SHIP_CAPACITY,
  Snapshot,
  lotPrice,
  total,
} from "./engine";
import { Chips, Swatch, money, seatColor } from "./ui";

export type Area = "factory" | "harbor" | "ship" | "holding" | "island";
export type Pick = (player: number, area: Area, lot: number, color: number) => number | undefined;

type Common = { snap: Snapshot; pick: Pick; act: (id: number) => void };

/** The shared table: every harbor around, the Island, the Ocean and the Bank in the middle. */
export default function Table({ snap, pick, act }: Common) {
  const me = snap.viewer;
  const others = snap.players.map((_, seat) => seat).filter((seat) => seat !== me);
  return (
    <div className="table">
      <div className="table-seats">
        {others.map((seat) => (
          <Harbor key={seat} snap={snap} seat={seat} pick={pick} act={act} />
        ))}
      </div>
      <div className="table-middle">
        <div className="table-waters">
          <IslandZone snap={snap} pick={pick} act={act} />
          <OceanZone snap={snap} pick={pick} act={act} />
        </div>
        <BankZone snap={snap} pick={pick} act={act} />
      </div>
      <Harbor snap={snap} seat={me} pick={pick} act={act} mine />
    </div>
  );
}

function ShipToken({ snap, seat, pick, act }: Common & { seat: number }) {
  const p = snap.players[seat];
  const colour = seatColor(seat);
  return (
    <div className="ship-token" style={{ borderColor: colour }}>
      <div className="ship-token-head" data-rule="ship">
        <Ship size={15} style={{ color: colour }} />
        <strong>{snap.seats[seat].name}</strong>
        <small>{total(p.cargo)}/{SHIP_CAPACITY}</small>
      </div>
      <div className="container-stack">
        <Chips
          counts={p.cargo}
          pick={(color) => pick(seat, "ship", 0, color)}
          onPick={act}
          empty={<span className="empty-lot">empty hold</span>}
        />
      </div>
    </div>
  );
}

function ShipsAt({
  snap,
  where,
  pick,
  act,
}: Common & { where: (ship: PlayerView["ship"]) => boolean }) {
  const seats = snap.players.map((_, s) => s).filter((s) => where(snap.players[s].ship));
  return (
    <>
      {seats.map((seat) => (
        <ShipToken key={seat} snap={snap} seat={seat} pick={pick} act={act} />
      ))}
    </>
  );
}

function Storage({ kind, used, limit }: { kind: District; used: number; limit: number }) {
  return <span className={`storage ${used >= limit ? "storage-full" : ""}`} data-rule={kind === "factory" ? "factoryDistrict" : "harborDistrict"}>stock {used}/{limit}</span>;
}

function Harbor({ snap, seat, pick, act, mine }: Common & { seat: number; mine?: boolean }) {
  const p = snap.players[seat];
  const info = snap.seats[seat];
  const colour = seatColor(seat);
  const active = seat === snap.activePlayer && !snap.gameOver;
  const factoryCount = p.factories.filter(Boolean).length;
  const factoryStock = p.factoryLots.reduce((n, l) => n + total(l), 0) + p.reserveFactory;
  const harborStock = p.harborLots.reduce((n, l) => n + total(l), 0) + p.reserveHarbor;

  const district = (kind: District, lots: Counts[]) => (
    <div className="district">
      <div className="district-head">
        <span data-rule={kind === "factory" ? "factoryDistrict" : "harborDistrict"}>
          {kind === "factory" ? <Factory size={13} /> : <Anchor size={13} />}
          {kind === "factory" ? "FACTORIES" : "HARBOR"}
        </span>
        {kind === "factory" ? (
          <Storage kind="factory" used={factoryStock} limit={factoryCount * 2} />
        ) : (
          <Storage kind="harbor" used={harborStock} limit={p.warehouses} />
        )}
      </div>
      <div className={`lots lots-${lots.length}`}>
        {lots.map((lot, i) => (
          <div className="lot" key={i}>
            <div className="lot-price">{money(lotPrice(kind, i))}</div>
            <div className="container-stack">
              <Chips counts={lot} pick={(c) => pick(seat, kind, i, c)} onPick={act} empty={<span />} />
            </div>
          </div>
        ))}
      </div>
    </div>
  );

  return (
    <section
      className={`harbor ${mine ? "harbor-mine" : ""} ${active ? "harbor-active" : ""}`}
      style={{ borderTopColor: colour }}
    >
      <header className="harbor-head">
        <strong>
          <i className="dot" style={{ background: colour }} />
          {mine ? `${info.name} (you)` : info.name}
        </strong>
        <span className="harbor-facts">
          <span data-rule="factories" className="harbor-factories">{p.factories.map((has, c) => has && <Swatch key={c} color={c} size={11} />)}</span>
          <span data-rule="warehouses"><Warehouse size={13} /> {p.warehouses}</span>
          <span data-rule="cash"><Coins size={13} /> {p.cash === null ? "?" : money(p.cash)}</span>
          <span data-rule="loans"><Landmark size={13} /> {p.loans}</span>
        </span>
      </header>
      {district("factory", p.factoryLots)}
      {district("harbor", p.harborLots)}
      <div className="harbor-foot">
        <div className="dock">
          <div className="district-head"><span data-rule="dock"><Anchor size={13} /> DOCK</span></div>
          <ShipsAt
            snap={snap}
            pick={pick}
            act={act}
            where={(s) => s.at === "harbor" && s.owner === seat}
          />
        </div>
      </div>
    </section>
  );
}

function IslandZone({ snap, pick, act }: Common) {
  return (
    <section className="zone zone-island">
      <header className="zone-head" data-rule="island"><Mountain size={15} /> THE ISLAND</header>
      <div className="island-rows">
        {snap.players.map((p, seat) => (
          <div className="island-row" key={seat}>
            <i className="dot" style={{ background: seatColor(seat) }} title={snap.seats[seat].name} />
            <div className="container-stack">
              <Chips
                counts={p.island}
                pick={(c) => pick(seat, "island", 0, c)}
                onPick={act}
                empty={<span className="empty-lot">{snap.seats[seat].name}: nothing</span>}
              />
            </div>
          </div>
        ))}
      </div>
      <div className="zone-ships">
        <ShipsAt snap={snap} pick={pick} act={act} where={(s) => s.at === "island"} />
      </div>
    </section>
  );
}

function OceanZone({ snap, pick, act }: Common) {
  return (
    <section className="zone zone-ocean">
      <header className="zone-head" data-rule="ocean"><Waves size={15} /> OPEN SEA</header>
      <div className="zone-ships">
        <ShipsAt snap={snap} pick={pick} act={act} where={(s) => s.at === "ocean"} />
        {snap.players.every((p) => p.ship.at !== "ocean") && <span className="empty-lot">no ships at sea</span>}
      </div>
    </section>
  );
}

function BankZone({ snap, pick, act }: Common) {
  const { containerLots, cashLots, auctions } = snap.bank;
  const tile = (kind: "container" | "cash", lot: number) =>
    auctions.find((a) => a.kind === kind && a.lot === lot);
  const bidText = (bid: (typeof auctions)[number]["bid"]) => {
    if ("cash" in bid) return money(bid.cash);
    const n = total(bid.factory) + total(bid.harbor);
    return `${n} container${n === 1 ? "" : "s"}`;
  };
  const lotView = (kind: "container" | "cash", lot: number, content: JSX.Element) => {
    const a = tile(kind, lot);
    return (
      <div key={`${kind}${lot}`} className={`bank-lot ${a ? "bank-lot-live" : ""}`}>
        <b>{LOT_NUMERALS[lot]}</b>
        {content}
        {a && (
          <small style={{ color: seatColor(a.holder) }}>
            <Gavel size={11} /> {snap.seats[a.holder].name}: {bidText(a.bid)}
          </small>
        )}
      </div>
    );
  };
  return (
    <section className="zone zone-bank">
      <header className="zone-head" data-rule="bankZone"><Landmark size={15} /> OFF-SHORE BANK</header>
      <p className="board-label" data-rule="containerLot">CONTAINER LOTS · bid cash</p>
      <div className="bank-lots">
        {containerLots.map((lot, i) =>
          lotView("container", i, <div className="container-stack"><Chips counts={lot} empty={<span className="empty-lot">empty</span>} /></div>),
        )}
      </div>
      <p className="board-label" data-rule="cashLot">CASH LOTS · bid containers</p>
      <div className="bank-lots">
        {cashLots.map((cash, i) => lotView("cash", i, <span className="cash-amount">{money(cash)}</span>))}
      </div>
      <p className="board-label" data-rule="holding">HOLDING AREAS · where ships load</p>
      <div className="island-rows">
        {snap.players.map((p, seat) => (
          <div className="island-row" key={seat}>
            <i className="dot" style={{ background: seatColor(seat) }} title={snap.seats[seat].name} />
            <div className="container-stack">
              <Chips
                counts={p.holding}
                pick={(c) => pick(seat, "holding", 0, c)}
                onPick={act}
                empty={<span className="empty-lot">{snap.seats[seat].name}: none</span>}
              />
            </div>
          </div>
        ))}
      </div>
      <div className="zone-ships">
        <ShipsAt snap={snap} pick={pick} act={act} where={(s) => s.at === "bank"} />
      </div>
    </section>
  );
}
