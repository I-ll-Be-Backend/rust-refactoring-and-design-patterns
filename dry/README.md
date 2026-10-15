# DRY: one rule, one place, in Rust

> Change a business rule once and have every feature pick it up.

[Watch the video](https://www.youtube.com/watch?v=UsVGgD1mqtI) · [Step through it interactively](https://illbebackend.dev/lessons/dry/rust/)

## The problem

The member limit is added to Checkout and CartBanner by hand. OrderEmail is missed and still uses 50.0.

## Steps

Each folder holds the complete `shipping.rs` after that step, and every one compiles. Diff two neighbouring folders to see a single refactoring.

| Step | What changes | Move |
|---|---|---|
| [01](step-01-three-features-one-rule/) | Three features, one rule: Checkout, CartBanner and OrderEmail each decide on their own whether a cart ships free from 50.0. The fee 4.99 is typed twice. | Before |
| [02](step-02-members-ship-free-from-25/) | Members ship free from 25: The member limit is added to Checkout and CartBanner by hand. OrderEmail is missed and still uses 50.0. | Code smell |
| [03](step-03-undo-then-give-it-a-home/) | Undo, then give it a home: Back the member change out. Gather the limit, the fee and the check into one module, then add members again. | Plan |
| [04](step-04-name-the-limit/) | Name the limit: The four copies of 50.0 become shipping_policy::FREE_FROM. | Extract Constant |
| [05](step-05-name-the-fee/) | Name the fee: The two copies of 4.99 become shipping_policy::FEE. | Extract Constant |
| [06](step-06-name-the-check/) | Name the check: The comparison moves into shipping_policy::ships_free(cart). Checkout calls it. | Extract Function |
| [07](step-07-the-banner-asks-too/) | The banner asks too: CartBanner's hand-written check becomes !shipping_policy::ships_free(cart). | Replace Inline Code with Function Call |
| [08](step-08-the-email-asks-too/) | The email asks too: OrderEmail calls shipping_policy::ships_free(cart). The comparison now exists once. | Replace Inline Code with Function Call |
| [09](step-09-name-the-gap/) | Name the gap: The banner's subtraction moves into shipping_policy::amount_to_free(cart). | Extract Function |
| [10](step-10-hide-the-limit/) | Hide the limit: FREE_FROM loses its pub. Code outside shipping_policy can't read the limit. | Encapsulate Variable |
| [11](step-11-members-in-one-module/) | Members, in one module: The member limit is a new constant and a private free_from(cart) inside shipping_policy. Checkout, CartBanner and OrderEmail don't change. | Payoff |

## Takeaways

- **Use it when:** One business rule, number or decision shows up in more than one feature.
- **Skip it when:** Two lines look alike but follow different rules, like a shipping fee and a gift wrap fee that both cost 4.99.
- **Spot it by:** The same literal or condition in several types, and changes that start with a search.
