# ContainerAI

A digital version of the board game **Container**, built to:

1. Play the game through a UI, including against AI opponents.
2. Train AI agents to play it.

## Stack

- **Game engine:** Rust, for fast simulation during training.
- **Training:** Python with PyTorch (CUDA).
- **UI:** to be decided.

## Development

The project is developed in a devcontainer (Rust, Python 3.12, Node, PyTorch with CUDA).

Requirements on the host:

- Docker
- An NVIDIA GPU with the driver and the [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html)

Open the folder in VS Code and choose **Reopen in Container**. After setup, check the GPU:

```sh
python -c "import torch; print(torch.cuda.is_available())"
```

## Engine

The rules engine is the library crate `crates/container-engine` (10th Anniversary rules, 3–5 players).

- `Game::new(GameConfig)`, `legal_actions()`, `apply(action)`, `observation(viewer)` and `results()` are the main entry points. The action model is a flat legal-action list.
- `observation` hides what the viewer cannot know: other players' cash, scoring cards and face-down bids.
- `GameConfig::with_max_bid_revisions(n)` enables bluff cards and bid revisions in delivery auctions. The default, 0, places each bid once, hidden.
- A table where every player is broke at the loan cap can no longer progress, and the engine does not declare a stalemate. Trainers should cap steps or turns.

```sh
cargo test -p container-engine
```

## Rules decisions

The engine plays the [10th Anniversary rulebook](https://bghub.org/r/container.pdf), the developer's errata and the rulings the project owner gave while it was built. This section records every point where the rulebook leaves a question open or where the engine had to commit to one reading of it, and what the errata changed. Rules the rulebook states plainly are not repeated. Every decision is covered by tests in `crates/container-engine/src/tests/`, in files named after the area, and `crates/container-engine/tests/playouts.rs` plays whole games.

### Sources

| Tag | Meaning |
|---|---|
| **Errata** | The developer's errata. It overrides the rulebook and every earlier decision. |
| **Ruling** | The owner answered a question about it. A ruling overrides the rulebook text and any assumption. |
| **Reading** | The rulebook speaks to it, but not precisely enough, so its text was interpreted. |
| **Assumption** | The rulebook is silent, so the choice was made while building the engine. *Announced* means the owner was told in the conversation and did not object. |
| **Rulebook** | Stated by the rulebook. Listed only where the engine's modelling of it is not obvious. |

### Errata

The errata arrived after the owner's first two rounds of rulings, so it overrides them. All of it is applied. One earlier ruling had to be reversed, and the other items confirm what was already decided.

| Errata | What the engine does | Earlier decision |
|---|---|---|
| **Loading Bank containers.** While docked at the Bank, a Harbor Purchase action loads containers from the holding area onto the ship, for free. | `HarborPurchase` while docked at the Bank loads containers from the holding area onto the ship. It is free, costs one of the two actions, must load at least one container, and is limited by the room on the ship. The optional free load on arrival at the Bank stays. | **Reversed.** The owner had ruled that sailing to the Bank is barred while the holding area is empty (`sail_bank_empty`). Now that a docked ship can load, waiting at the Bank makes sense, so sailing to the Bank is always legal. |
| **Bidding $0 in Bank auctions.** A new auction needs a bid of at least $1 or 1 container. | A container lot opens with at least $1 and a cash lot with at least 1 container. An outbid needs $1, or 1 container, more than the tile holds. | Confirms the ruling on opening bids. |
| **Containers on a bid tile** can't be sold, repriced or seized in a default. They leave the tile only when the auction is won or an opponent outbids. | They leave the lots when they go on the tile, so they can't be bought, repriced or seized. A default with nothing else to seize is forgiven. Their reserve tokens still count against storage limits. | None. The developer notes say these rules are unchanged. |
| **$0 bluff cards** only disguise how many cards a bid has. They have no other effect and can be removed. | Disguise only (see Bluff cards below). With `max_bid_revisions = 0`, the default, no cards are used at all. | Confirms the ruling to include them. |
| **Delivery minimum bid**, added by the owner next to the errata: it is $0. | A bid is `0..=cash`. A player without cash can only bid $0. Bids of $0 all round are a tie and go to a runoff. A buyout at $0 costs nothing. | None. The rulebook states no minimum. |

Two points go beyond the errata's text. A Harbor Purchase at the Bank must load at least one container, like every paid purchase action (an owner ruling), because a paid load of nothing would only duplicate the no-op Reprice. And reversing `sail_bank_empty` is a consequence drawn from the errata, not something the owner stated.

Tests: `docked_at_the_bank_a_harbor_purchase_loads_the_holding_area_for_free`, `a_harbor_purchase_at_the_bank_needs_something_to_load_and_room_for_it`, `a_ship_docked_at_the_bank_loads_the_containers_of_an_auction_it_won` and `the_bank_can_be_reached_with_nothing_to_collect` (sail); `a_container_lot_auction_opens_with_a_bid_of_at_least_one_dollar` (auction); `a_cash_lot_auction_opens_with_a_bid_of_at_least_one_container`, `the_containers_on_the_tile_are_not_laid_out_again_by_repricing`, `the_containers_on_the_tile_cannot_be_bought` and `the_containers_on_the_tile_cannot_be_seized_so_a_default_with_nothing_else_is_forgiven` (container_bids); `without_revisions_a_bid_is_the_cash_alone` (bluff); `a_player_without_cash_can_only_bid_nothing`, `bids_of_nothing_all_round_are_a_tie` and `a_buyout_at_zero_costs_nothing` (delivery).

### Decision log

#### Modelling

| Decision | Source |
|---|---|
| The engine is a library: no CLI, UI or Python bindings. Only the 10th Anniversary rules are played. The rulebook's Classic variant (the first-edition rules) is out of scope. | Ruling |
| Actions are micro: a flat list of small legal actions, one small choice per `apply`, with a half-finished action kept in the state. Whole-action alternatives (a `Produce` with every arrangement in one action, say) were compared and rejected, as they give thousands of options per decision and no fixed action mask. | Ruling |
| `is_legal` is the single source of truth. An illegal `apply` returns an error and changes nothing. | Assumption |
| Every outcome has exactly one action sequence. Colors are picked in a fixed order, containers are laid out one at a time in a fixed order with the lots never going down within a color, and purchases, loads and seizures go lot by lot and color by color. | Assumption |
| A step with nothing left to choose ends by itself: a Produce that fits entirely asks for no colors, and a purchase or load ends once nothing more can be bought or loaded. | Assumption |
| All randomness is at setup, from a seeded xoshiro256** generator: the Bank's starting colors, the starting factories, the scoring cards and the first player. A seed replays the same game. | Assumption |
| The first player is random, which is the rulebook's alternative to "the player who last captained a seagoing vessel". | Reading |
| The secret simultaneous bids of a delivery auction are sequential hidden decisions, clockwise from the deliverer's left, revealed together. | Assumption |
| Cash is an integer, change is free and the supply never runs out. The card counts the owner supplied (36, 34, 26, 20 and 25 cards of $1, $2, $5, $10 and $20) are not enforced. The denominations only decide how a bid can be shown as cards. | Reading, Ruling |
| The player "on your right" is the seat that plays just before you, since turns go clockwise. | Reading |

#### Turn

| Decision | Source |
|---|---|
| A turn is: pay interest, win a Bank auction, then exactly two actions. The same action may be taken twice, except Produce and Call Bank, which are once per turn. | Rulebook |
| There is no Pass action. A Reprice that leaves the arrangement as it was is the pass. `Reprice(Factory)` is always legal, and `Reprice(Harbor)` needs a container in the harbor. | Ruling |

#### Build, produce and purchases

| Decision | Source |
|---|---|
| Factory costs are $0, $6, $9 and $12, with a storage limit of 2 per factory. Warehouse costs are $0, $4, $5, $6 and $7, with a harbor limit of 1 per warehouse. Lot prices are $1 to $4 in the factory district and $2 to $6 in the harbor district. These numbers were read from the rulebook's pictures, checked again, and confirmed by the owner. | Ruling |
| Produce is legal whenever the $1 wages can be paid, even if it yields nothing (no room, or the supply has none of your colors). | Ruling |
| A factory whose color is out of the supply yields nothing. | Reading |
| A paid purchase or load action (Factory Purchase, Harbor Purchase, a load at the Bank) must buy or load at least one container. The free action on arrival may be skipped. | Ruling |
| Containers are bought one at a time, in lot and color order, and the seller is paid the lot price of each. The buyer's harbor room counts reserve tokens. Afterwards the buyer's harbor district is laid out again. | Assumption |
| Nothing tracks who produced a container, so a player may buy, and ship, a container of a color they produce themselves. The rulebook's "you can never buy or ship your own containers" holds because a purchase from your own districts and a visit to your own harbor are never allowed. | Reading |

#### Sailing

| Decision | Source |
|---|---|
| Sailing to the Island with an empty ship is forbidden. The rulebook doesn't rule against it, but it is meaningless. | Ruling |
| Sailing to the Bank from the ocean is always legal, even with nothing in the holding area. | Errata, as a consequence |
| Sailing to an opponent's harbor is legal even if nothing there can be bought. | Assumption |
| The free anchor action at a harbor or the Bank is optional, and is skipped by itself when nothing could be bought or loaded. | Reading |

#### Delivery auctions

| Decision | Source |
|---|---|
| Opponents bid from $0 up to the cash they hold. A bid leaves the hand when it goes on the table, so it can't be spent on anything else, but a bidder may take a loan first. | Ruling, Assumption |
| Bids of $0 all round are a tie. | Assumption, announced |
| A tie gets one runoff. Each tied bidder adds cash on top of their first bid, which stays on the table, and the other bidders sit out. A tie after the runoff goes to the deliverer's choice. | Rulebook |
| The deliverer decides after the reveal even when there is no tie: accept a bidder, or buy out. Accepting pays the bid plus an equal subsidy from the supply and refunds the other tied bids. A buyout pays the bid into the Bank's cash lots, keeps the cargo, refunds all tied bids, and needs the cash. | Rulebook |

#### Bluff cards and bid revisions

| Decision | Source |
|---|---|
| Bluff cards are pure disguise: 2 per player, with no other effect. They come back to the hand after each reveal and can be reused, in a runoff too. | Errata, Rulebook |
| Revisions are off by default (`max_bid_revisions = 0`): every bid is placed once, hidden, then revealed, and no cards are used. The owner plans 2 for human play. | Ruling |
| With a cap above 0 a bid is laid out as a stack: the cash amount, how many cash cards it is shown as (any number the denominations allow, so $5 can be 1, 3, 4 or 5 cards but never 2), and 0 to 2 bluff cards. Only the number of cards in each stack is public, once every stack is down. | Ruling, Assumption |
| Once every stack is down, the bidders look the stacks over in turn, starting with the first bidder. Each may be content, revise (take the stack back and lay out another, while revisions are left) or take a loan. A revision clears everyone's "content" mark, so everybody looks again. When nobody wants to, or can, revise any more, the stacks are revealed together. | Ruling, Assumption |
| A revision may spend the cash of the stack it replaces. | Assumption |
| A runoff follows the same protocol. Only the added stack is laid out, each bidder's revisions are counted afresh, and the first bid stays on the table. | Assumption |

#### Loans, interest and default

| Decision | Source |
|---|---|
| Loans are free actions. They are offered at a player's own decision points: between their actions, when the deliverer decides, and to each bidder while laying out cash or looking at the stacks. They are not offered in the middle of a purchase, load, arrangement or Bank bid, as a loan taken earlier gives the same result because costs are known beforehand. | Assumption |
| A player may request at most 2 loans and repay at most 2 in one of their own turns, which stops repay-and-borrow loops. | Ruling |
| Repaying is only possible on the player's own turn, between actions, after interest. | Rulebook |
| Interest is due on the loans held when the turn begins. A player who is short and holds fewer than 2 loans takes a loan to pay it, and that loan does not count as a request. | Reading |
| A player at 2 loans who can't pay it all pays what they can. The rest is default: one container is seized for each loan whose interest is unpaid. | Reading |
| A seizure takes a container from the first non-empty place in this order: scoring area on the Island, ship, holding area, harbor, factory. The player on the debtor's right chooses which container, and which color goes in each Bank container lot. Containers on a bid tile can't be seized. The debtor keeps the loan cards. With nothing to seize, the interest is forgiven for that turn. | Rulebook, Errata |
| Interest, and any default, come before a won auction is collected. The cash on a bid tile can't pay interest, and won winnings can't pay it or be seized for it. | Rulebook |

#### Off-Shore Bank

| Decision | Source |
|---|---|
| A payment is dealt $1 or 1 container at a time, starting at lot I for every payment, wrapping around and skipping a lot with an auction token of the kind being paid. The payer chooses the colors per lot, except in a default, where the player on the debtor's right does. | Rulebook |
| Call Bank is once per turn, and not in a turn in which an auction was won or the game was ended. | Rulebook |
| An auction can only start on a lot that holds something. A holder can't outbid themselves. | Ruling, Reading |
| A container lot is bid for with cash, face up on the cash tile, and that cash can't be spent, even on interest. The opening bid is at least $1 and an outbid at least $1 more. | Rulebook, Errata |
| A cash lot is bid for with containers from the factory and harbor districts, and only their number matters. The opening bid is at least 1 container and an outbid at least 1 more. Reserve tokens mark where they came from and count against storage limits. A bid holds at most 10 containers, as the box has 10 reserve tokens. | Rulebook, Errata, Assumption (the cap, announced) |
| Containers are picked for a bid in a fixed order: the factory district first, then lot by lot and color by color. | Assumption |
| An outbid holder gets the cash or containers back. Only the returned containers are laid out again, in any lots of the district they came from, and the holder decides where. | Assumption, announced |
| The holder wins at the start of their next turn, after interest. A won container lot goes to the holding area and a won cash lot to the hand. The winner pays into the other kind of lots, and chooses which colors go where. | Rulebook |

#### Game end

| Decision | Source |
|---|---|
| The game ends when the supply is out of 2 colors. Only Produce lowers the supply, so the check comes right after Produce. The active player finishes their turn, so a second action is still taken, and no further interest is paid. | Rulebook |
| No Call Bank in the turn that ends the game, in either order. Once a Produce has emptied the second color, Call Bank is barred. A Produce that would end the game is illegal if the Bank was already called this turn, so colors that would end the game can't be chosen then. | Ruling |
| Winning an auction at the start of a turn doesn't stop the game ending that turn by Produce. The ban is on Call Bank, which a won auction doesn't involve. | Reading |
| Active auctions go to the highest bidder when the game ends. Won containers go to the holding area, worth $3 each. A cash-lot winner's containers go to the Bank with no choice, as they no longer score. | Rulebook, Assumption |

#### Final scoring

| Decision | Source |
|---|---|
| Three scoring cards are read from the rulebook (White, Blue and Green have the $10) and two are inferred: all five are rotations of the color cycle Red, Blue, Yellow, White, Green. A card's $10 color is followed by the two-value color, then $6, $4 and $2. The owner accepted the two inferred cards. | Ruling |
| On a tie for the most common color, the engine discards for the player: the two-value color if it is among the tied colors, otherwise the cheapest tied color. That is the best choice for the player, so there is no decision to ask. | Reading |
| A score can be negative, as each loan costs $11. The most cash wins, ties go to the most containers in the factory district, and remaining ties share the win. | Rulebook |

#### Observation

| Decision | Source |
|---|---|
| An observation hides other players' cash and scoring cards until the game is over, and hides face-down bids. A hand's card count is not tracked, so cash stays fully hidden. | Assumption, announced |
| With bluff cards, only the number of cards in each stack is public. The viewer sees the split of their own stack. | Errata, Assumption |
| The seed and the list of legal actions are not part of an observation. | Assumption |

#### Known limit

| Decision | Source |
|---|---|
| A table where every player is broke at the loan cap with no containers can't progress. The rulebook has no rule for it and the engine does not declare a stalemate. About 0.17% of the playouts with a sensible policy reach this state, so trainers should cap steps or turns. | Assumption |

### Points to confirm

1. Sailing to the Bank is always legal. It follows from the errata and replaces the owner's `sail_bank_empty` ruling, but the owner has not confirmed that consequence.
2. A Harbor Purchase at the Bank must load at least one container.
3. A turn that wins a Bank auction may still end the game by Produce.
4. A cash-lot bid holds at most 10 containers, and after an outbid only the returned containers are laid out again. Both were announced and never explicitly confirmed.
