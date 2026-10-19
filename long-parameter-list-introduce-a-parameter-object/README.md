# Long parameter list: introduce a parameter object, in Rust

> Group the values that travel together, so a new one stops touching every signature.

[Watch the video](https://www.youtube.com/watch?v=pZ98AB5_Yuc) · [Step through it interactively](https://illbebackend.dev/lessons/long-parameter-list-introduce-a-parameter-object/rust/)

## The problem

The carrier charges 20% more for fragile parcels. A fragile bool joins both signatures, both calls and one new line in price.

## Steps

Each folder holds the complete `shipping.rs` after that step, and every one compiles. Diff two neighbouring folders to see a single refactoring.

| Step | What changes | Move |
|---|---|---|
| [01](step-01-a-quote-with-nine-parameters/) | A quote with nine parameters: Checkout passes two warehouse constants, the customer's address and five cart values to quote. Quote hands seven of them to price. | Before |
| [02](step-02-one-flag-five-edits/) | One flag, five edits: The carrier charges 20% more for fragile parcels. A fragile bool joins both signatures, both calls and one new line in price. | Code smell |
| [03](step-03-two-bools-side-by-side/) | Two bools side by side: Four signature and call edits came before the one line that uses fragile. Express and fragile now sit next to each other in both signatures. | Code smell |
| [04](step-04-back-it-out-then-group/) | Back it out, then group: Fragile comes out again. Three groups of values travel together through every list, and each one becomes a struct first. | Plan |
| [05](step-05-the-origin-becomes-an-address/) | The origin becomes an Address: An Address struct holds a postal code and a country. Quote and price borrow one called from, and checkout builds it from the warehouse constants. | Introduce Parameter Object |
| [06](step-06-the-destination-is-an-address-too/) | The destination is an Address too: The customer's postal code and country become a second Address called to. Both methods take from and to. | Introduce Parameter Object |
| [07](step-07-four-floats-become-a-parcel/) | Four floats become a Parcel: A Parcel struct holds the weight and the three box sizes. Quote and price take four parameters each, and checkout builds the parcel from the cart. | Introduce Parameter Object |
| [08](step-08-name-the-weight-rule/) | Name the weight rule: The volumetric weight and the larger-of-two rule move into billable_kg. Price multiplies the rate by it. | Extract Method |
| [09](step-09-the-rule-moves-onto-parcel/) | The rule moves onto Parcel: billable_kg moves into an impl block on Parcel and reads its own fields. Price asks the parcel for it. | Move Method |
| [10](step-10-fragile-in-three-edits/) | Fragile, in three edits: Fragile joins Parcel, checkout copies the cart's flag, and price adds the surcharge. Both signatures and the call inside quote didn't change. | Payoff |
| [11](step-11-why-the-clumps-paid-off/) | Why the clumps paid off: Each clump is one struct, so a new parcel fact joins Parcel. Express is a parameter and fragile lives in the parcel. | Payoff |

## Takeaways

- **Use it when:** The same group of values goes, in the same order, through several functions.
- **Skip it when:** A function takes three or four unrelated values used only there, like a log call with a level, a message and an error.
- **Spot it by:** The same names repeated across signatures, runs of same-typed parameters, a function that only passes its parameters on.
- **Already in Rust:** A Duration holds seconds and nanoseconds as one value, so sleep takes a single argument.
