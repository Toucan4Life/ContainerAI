export type RuleEntry = { title: string; lines: string[] };

/** Every rule the engine plays, grouped by the board element or action it governs. */
export const RULES = {
  turn: {
    title: "Turn",
    lines: [
      "A turn: pay interest on your loans, collect a Bank auction you won, then take exactly two actions.",
      "The same action may be taken twice, except Produce and Call Bank, which are once per turn.",
      "There is no Pass. A Reprice that changes nothing is the pass.",
      "Loans are free and can be taken between actions.",
    ],
  },
  produce: {
    title: "Produce",
    lines: [
      "Pay $1 wages, then each factory you own yields a container of its color into your factory lots.",
      "Each factory stores at most 2 containers; a full factory yields nothing.",
      "A color that is out of the supply yields nothing.",
      "Once per turn. Legal whenever you can pay the wages, even if it yields nothing.",
      "Only Produce lowers the supply, so the game-end check comes right after it.",
    ],
  },
  buy: {
    title: "Buy containers",
    lines: [
      "Factory Purchase: buy from opponents' factory lots onto your harbor. Harbor Purchase: buy from the harbor lots of the harbor your ship is docked at, onto your ship.",
      "The seller is paid the lot price of each container; you buy one at a time.",
      "A paid purchase must buy at least one container.",
      "Your harbor holds at most 1 container per warehouse. Afterwards your harbor is laid out again.",
      "You can never buy from, or sail to, your own districts.",
    ],
  },
  sail: {
    title: "Sail",
    lines: [
      "Your ship moves between your dock, the open sea, opponents' harbors, the Island and the Off-Shore Bank.",
      "Sailing to the Island with an empty ship is not allowed.",
      "Sailing to the Bank is always allowed, even with nothing to collect.",
      "Sailing to an opponent's harbor is allowed even if nothing there can be bought.",
      "On arrival at a harbor or the Bank you get a free, optional anchor action to buy or load.",
    ],
  },
  build: {
    title: "Build",
    lines: [
      "Factory costs: $0, $6, $9, $12. Each factory stores 2 containers.",
      "Warehouse costs: $0, $4, $5, $6, $7. Each warehouse holds 1 container in your harbor.",
      "Factories and warehouses come from the supply.",
    ],
  },
  reprice: {
    title: "Reprice",
    lines: [
      "Lay out the containers of your factory or harbor district again in any lots.",
      "Higher lots charge more: $1-$4 in the factory district, $2-$6 in the harbor district.",
      "Reprice(Factory) is always legal; Reprice(Harbor) needs a container in your harbor.",
      "Repricing without changes is the pass action.",
    ],
  },
  bank: {
    title: "Call the Bank",
    lines: [
      "Start an auction on a Bank lot that holds something. Once per turn.",
      "Not allowed in a turn in which you won an auction or the game ended.",
      "Container lots are bid for with cash; cash lots with containers from your districts.",
      "A new bid is at least $1 / 1 container, an outbid at least $1 / 1 container more. You can't outbid yourself.",
      "You win at the start of your next turn, after interest.",
    ],
  },
  loans: {
    title: "Loans",
    lines: [
      "Taking a loan is a free action and gives you $10.",
      "Each loan costs $11 at the end of the game (final score can be negative).",
      "Interest is due at the start of your turn on the loans you hold.",
      "You may take at most 2 loans and repay at most 2 in one of your own turns.",
      "Repaying is only on your own turn, between actions, after interest.",
      "If you are short and hold fewer than 2 loans, a loan is taken automatically to pay interest.",
      "At 2 loans, unpaid interest is a default: the Bank seizes one container for each loan whose interest is unpaid.",
      "Seizure order: Island scoring area, ship, holding area, harbor, factory. The player on your right chooses which container and the Bank lot colors.",
      "Containers on a bid tile and won winnings can't be seized. With nothing to seize, the interest is forgiven.",
    ],
  },
  cash: {
    title: "Cash",
    lines: [
      "Opponents' cash is hidden until the game is over.",
      "Cash on a bid tile can't be spent, even on interest.",
      "The most cash at the end wins after scoring the Island and subtracting $11 per loan.",
    ],
  },
  warehouses: {
    title: "Warehouses",
    lines: [
      "Each warehouse holds 1 container in your harbor district.",
      "Costs: $0, $4, $5, $6, $7 for the successive warehouses.",
      "Reserve tokens from Bank cash-lot bids count against the limit.",
    ],
  },
  factories: {
    title: "Factories",
    lines: [
      "Each factory has a color and yields one container of it when you Produce.",
      "Storage limit: 2 containers per factory.",
      "Costs: $0, $6, $9, $12.",
      "If a color runs out of the supply, its factory yields nothing.",
    ],
  },
  factoryDistrict: {
    title: "Factory district",
    lines: [
      "Where produced containers wait for sale. Opponents buy from here with a Factory Purchase at the lot prices ($1-$4).",
      "Storage limit: 2 per factory. Reserve tokens from Bank bids count against it.",
      "Reprice lays the district out again; containers on a Bank bid tile can't be sold or repriced.",
      "Ties for the win go to the most containers in the factory district.",
    ],
  },
  harborDistrict: {
    title: "Harbor district",
    lines: [
      "Containers you bought wait here. Visiting opponents buy from the lots with a Harbor Purchase at the lot prices ($2-$6).",
      "Limit: 1 container per warehouse, counting reserve tokens.",
      "Your harbor is laid out again after each of your purchases.",
    ],
  },
  lot: {
    title: "Lot price",
    lines: [
      "A buyer pays this price to the owner for each container bought from the lot.",
      "Factory lots cost $1-$4, harbor lots $2-$6.",
      "Containers are bought one at a time, in lot and color order.",
    ],
  },
  dock: {
    title: "Dock",
    lines: [
      "Ships docked at this harbor wait here. A visiting ship may buy from the harbor district with a Harbor Purchase.",
      "You can never buy or ship your own containers: visiting your own harbor is never allowed.",
    ],
  },
  ship: {
    title: "Ship",
    lines: [
      "Holds up to 5 containers.",
      "Load containers from your holding area for free while at the Bank, or buy them at opponents' harbors.",
      "Deliver to the Island to auction the cargo to your opponents.",
      "A seizure takes from the ship if the Island has nothing to take.",
    ],
  },
  island: {
    title: "The Island",
    lines: [
      "Containers delivered here score for their owner according to the secret scoring card.",
      "Your most common color here is discarded and scores nothing.",
      "On a tie for most common, the engine discards for you: the two-value color if tied, otherwise the cheapest tied color.",
      "Sailing here with an empty ship is not allowed.",
      "Seizures take from the Island scoring area first.",
    ],
  },
  delivery: {
    title: "Delivery auction",
    lines: [
      "When you deliver to the Island, each opponent places one hidden bid from $0 up to their cash. Bids are revealed together and can't be revised.",
      "A bidder may take a loan first. A placed bid leaves their hand.",
      "You then decide: accept a bidder (you get the bid plus an equal subsidy from the supply, other tied bids are refunded) or buy out (you pay the bid into the Bank's cash lots and keep the cargo).",
      "All bids at $0 tie. A tie gets one runoff: tied bidders add cash on top of their first bid. A tie after the runoff goes to your choice.",
      "A buyout needs the cash; at $0 it costs nothing.",
    ],
  },
  ocean: {
    title: "Open sea",
    lines: [
      "Ships travel through the open sea between harbors, the Bank and the Island.",
      "Sailing out of the dock puts your ship here.",
    ],
  },
  bankZone: {
    title: "Off-Shore Bank",
    lines: [
      "Holds container lots (bid cash) and cash lots (bid containers).",
      "Payments are dealt $1 or 1 container at a time starting at lot I, wrapping around and skipping lots with an auction token of that kind.",
      "The payer chooses the colors per lot (in a default, the player on the debtor's right does).",
      "Call Bank is once per turn, and not in a turn when you won an auction or the game ended.",
      "Active auctions go to the highest bidder when the game ends; won containers are worth $3 each.",
    ],
  },
  containerLot: {
    title: "Container lot",
    lines: [
      "Bid cash, face up on the cash tile. That cash can't be spent, even on interest.",
      "Opening bid at least $1; an outbid at least $1 more. An auction can only start on a lot that holds something.",
      "An outbid holder gets the cash back.",
      "A won lot goes to your holding area. You pay into the cash lots and choose the colors.",
    ],
  },
  cashLot: {
    title: "Cash lot",
    lines: [
      "Bid containers from your factory and harbor districts. Only their number matters; a bid holds at most 10.",
      "Opening bid at least 1 container; an outbid at least 1 more.",
      "Reserve tokens mark where the containers came from and count against storage limits.",
      "An outbid holder gets the containers back and lays them out in the district they came from.",
      "A won cash lot goes to your hand.",
    ],
  },
  holding: {
    title: "Holding area",
    lines: [
      "Containers you won at the Bank wait here until your ship loads them.",
      "While docked at the Bank, a Harbor Purchase loads them for free (at least one container, limited by room on the ship).",
      "The free load on arrival at the Bank is optional.",
      "At game end, won containers go here and are worth $3 each. Seizures take from here after the ship.",
    ],
  },
  supply: {
    title: "Supply",
    lines: [
      "Containers, factories and warehouses not yet in play. Cash never runs out.",
      "The game ends when the supply is out of 2 container colors. Only Produce lowers it.",
      "The active player finishes their turn (a second action is still taken) and no further interest is paid.",
      "No Call Bank in the turn that ends the game, in either order.",
      "A Produce that would end the game is illegal if the Bank was already called this turn.",
    ],
  },
  scoringCard: {
    title: "Scoring card",
    lines: [
      "Secret until the game ends. Each container on the Island scores according to its color on your card.",
      "Your $10 color is followed by the two-value color, then $6, $4 and $2 (the color cycle Red, Blue, Yellow, White, Green).",
      "Your most common color on the Island is discarded.",
      "Final score = scoring + cash - $11 per loan. Ties go to the most containers in the factory district; remaining ties share the win.",
    ],
  },
  events: {
    title: "Events",
    lines: ["The log of everything that happened. Hover an entry for its round and details."],
  },
  seize: {
    title: "Default",
    lines: [
      "One container is seized for each loan whose interest is unpaid.",
      "Order: Island scoring area, ship, holding area, harbor, factory.",
      "The player on the debtor's right chooses which container and which color goes in each Bank container lot.",
      "You keep the loan cards. Containers on a bid tile can't be seized.",
    ],
  },
  turnOrder: {
    title: "Seats",
    lines: [
      "Turns go clockwise. The first player is random.",
      "The player 'on your right' is the seat that plays just before you.",
      "Other players' cash and scoring cards are hidden until the game ends.",
    ],
  },
  gameEnd: {
    title: "Game end",
    lines: [
      "The game ends when two container colors run out.",
      "Highest total wins: Island scoring + cash - $11 per loan. Ties go to the most containers in the factory district.",
      "If every player is broke, owes every loan and only idle moves remain, the game ends when a turn ends.",
    ],
  },
} satisfies Record<string, RuleEntry>;

export type RuleKey = keyof typeof RULES;
