# YAGNI: delete speculative options, in Rust

> Delete the settings nobody uses, so each new column is a two-line change.

[Watch the video](https://www.youtube.com/watch?v=05fNDrl7SZM) · [Step through it interactively](https://illbebackend.dev/lessons/yagni-delete-speculative-options/rust/)

## The problem

The warehouse wants the postal code as a third column. The header and all three arms of the match on format change.

## Steps

Each folder holds the complete `export.rs` after that step, and every one compiles. Diff two neighbouring folders to see a single refactoring.

| Step | What changes | Move |
|---|---|---|
| [01](step-01-an-export-with-three-settings/) | An export with three settings: OrderExport copies a format, a delimiter and a header flag from config.toml. NightlyJob uploads the result to the warehouse. | Before |
| [02](step-02-one-column-four-edits/) | One column, four edits: The warehouse wants the postal code as a third column. The header and all three arms of the match on format change. | Code smell |
| [03](step-03-two-formats-nobody-reads/) | Two formats nobody reads: The config says csv and the only caller uploads CSV. The JSON and XML arms never run. | Code smell |
| [04](step-04-back-it-out-then-delete/) | Back it out, then delete: The postal code comes out. We remove the options nobody uses first, and add the column afterwards. | Plan |
| [05](step-05-delete-the-xml-arm/) | Delete the XML arm: No config value picks xml and no caller reads it, so the arm goes. | Remove Dead Code |
| [06](step-06-delete-the-json-arm/) | Delete the JSON arm: JSON goes for the same reason. The match is down to csv and a catch-all that returns an error. | Remove Dead Code |
| [07](step-07-format-can-only-be-csv/) | Format can only be csv: The format field goes. The match collapses to the CSV row, export can no longer fail, and format leaves the config. | Remove Field |
| [08](step-08-the-delimiter-is-always-a-comma/) | The delimiter is always a comma: The comma goes straight into the header and the row. The delimiter field and its config line go. | Remove Field |
| [09](step-09-the-header-is-always-on/) | The header is always on: header is always true, so the branch that skips it is dead code. The if goes, then the field and its config line. | Remove Field |
| [10](step-10-nothing-left-to-configure/) | Nothing left to configure: new, ExportConfig and the job's config field go. The export table leaves config.toml. | Remove Parameter |
| [11](step-11-the-postal-code-in-two-edits/) | The postal code, in two edits: One new name in the header and one new value in the row. The pushes and NightlyJob didn't change. | Payoff |
| [12](step-12-a-struct-with-nothing-to-hold/) | A struct with nothing to hold: OrderExport has no fields left. Its one method takes a self it never reads. | Code smell |
| [13](step-13-a-function-is-enough/) | A function is enough: The unit struct and its impl become the module-level function export_orders. NightlyJob calls it directly. | Rust bonus |

## Takeaways

- **Use it when:** An option has one value in the config and no caller uses the other branches.
- **Skip it when:** A second user is real and dated, like a signed contract for a JSON partner, or ops change the value per environment.
- **Spot it by:** Config values that never change, match arms no caller reaches, defaults no caller overrides.
