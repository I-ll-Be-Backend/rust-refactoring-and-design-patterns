# KISS: replace a rule engine with plain code, in Rust

> Turn a clever rule engine back into code you read top to bottom.

[Watch the video](https://www.youtube.com/watch?v=cfWPLWyF6nA) · [Step through it interactively](https://illbebackend.dev/lessons/kiss-unwind-a-clever-abstraction/rust/)

## The problem

Electric cars now park the first two hours free. The ev rule only works if it runs after base and before cap.

## Steps

Each folder holds the complete `parking_fee.rs` after that step, and every one compiles. Diff two neighbouring folders to see a single refactoring.

| Step | What changes | Move |
|---|---|---|
| [01](step-01-a-rule-engine-for-one-fee/) | A rule engine for one fee: charge works out a parking fee by running named rule functions from a static map, in the order a slice of strings gives. | Before |
| [02](step-02-the-new-rule-has-to-sit-in-the-right-slo/) | The new rule has to sit in the right slot: Electric cars now park the first two hours free. The ev rule only works if it runs after base and before cap. | Code smell |
| [03](step-03-back-it-out-then-simplify/) | Back it out, then simplify: The ev rule comes out. We take the rule map apart one safe step at a time and add electric cars afterwards. | Plan |
| [04](step-04-there-is-only-one-plan/) | There is only one plan: fee no longer takes a plan. The single slice moves inside it, and charge only passes the ticket. | Remove Parameter |
| [05](step-05-three-lines-instead-of-a-loop/) | Three lines instead of a loop: The loop over a fixed slice becomes three calls, written in the order the slice gave. | Unroll Loop |
| [06](step-06-inline-the-base-rule/) | Inline the base rule: The lookup for base is replaced with the body of its closure. | Inline Function |
| [07](step-07-inline-the-cap-rule/) | Inline the cap rule: The cap lookup becomes the smaller of the fee and twenty. | Inline Function |
| [08](step-08-inline-the-grace-rule/) | Inline the grace rule: The last lookup becomes the fifteen minute check. fee no longer reads the map. | Inline Function |
| [09](step-09-delete-the-rule-map/) | Delete the rule map: The Rule type and the RULES map go. Nothing read them. | Remove Dead Code |
| [10](step-10-check-the-grace-period-first/) | Check the grace period first: The fifteen minute check slides to the top and returns zero. The rest becomes one expression. | Guard Clause |
| [11](step-11-charge-does-the-work-itself/) | charge does the work itself: charge only forwarded to fee, so fee's body moves into charge and fee goes. | Inline Function |
| [12](step-12-name-the-numbers/) | Name the numbers: 3.0, 20.0 and 15 become HOURLY_RATE, DAILY_CAP and GRACE_MINUTES constants. | Replace Magic Number |
| [13](step-13-electric-cars-in-reading-order/) | Electric cars, in reading order: Electric cars take two hours off before the rate and the cap apply. The grace check and the other constants didn't change. | Payoff |

## Takeaways

- **Use it when:** An abstraction has one user and one configuration.
- **Skip it when:** The rules truly vary at runtime, like per-garage pricing loaded from config or rules other teams plug in.
- **Spot it by:** String keys looked up in a map, a loop over steps that never change, code you must run in your head.
