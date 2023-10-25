pub const CANNOT_HIT_ALL_GARG: &str = "Cannot hit all gargantuars at this tick.";

pub const GARG_X_RANGE_CANCELLED: &str =
    "Gargantuars with x < 401 do not throw imps. Calculation skipped.";

pub const GARG_X_RANGE_MODIFIED: &str =
    "Gargantuars with x < 401 do not throw imps. x = {}~{} is used instead.";

// print_ice_times_and_cob_time
pub const DELAY_SETTING: &str = "Delay setting";
pub const SETTING: &str = "Setting";
pub const NO_ICE: &str = "No ice";
pub const ICE: &str = " ice";
pub const COB_EFFECTIVE: &str = " cob effective";
pub const COB_ACTIVATE: &str = " cob effective";
pub const GARG_X_RANGE: &str = "Garg x range";

// print_cob_calc_setting
pub const CALCULATION_SETTING: &str = "Calc setting";
pub const COB_GARG_ROWS: &str = "fire cob to row {} for row {} gargs";
pub const COB_COL_RANGE: &str = "fire cob to col {}~{}";
pub const EXPLOSION_CENTER: &str = "explosion center ";
pub const GARG: &str = "garg ";

// print_doom_calc_setting
pub const DOOM_GARG_ROWS: &str = "row {} doom for row {} gargs";

// print_eat_and_intercept
pub const INTERCEPTABLE_INTERVAL: &str = "Interceptable";
pub const CANNOT_INTERCEPT: &str = "Cannot intercept";
pub const WILL_CAUSE_HARM: &str = " will cause harm";
pub const EARLIEST_EAT: &str = "Earliest eat";
pub const DOES_NOT_EAT: &str = "Does not eat";
pub const EARLIEST_ICEABLE: &str = "Earliest iceable";
pub const NOT_ICEABLE: &str = "Not iceable";

// print_hit_cob_dist
pub const COL: &str = "col {}";
pub const HIT_SAME_AND_LOWER: &str = "Hit same & lower rows";
pub const HIT_ALL_THREE_ROWS: &str = "Hit three rows";
pub const HIT_UPPER_ROW: &str = "Hit upper row";
pub const HIT_SAME_ROW: &str = "Hit same row";
pub const HIT_LOWER_ROW: &str = "Hit lower row";

// print_nohit_cob_dist
pub const NOT_HIT_SAME_AND_LOWER: &str = "Not hit same & lower rows";
pub const NOT_HIT_UPPER_ROW: &str = "Not hit upper row";
pub const NOT_HIT_SAME_ROW: &str = "Not hit same row";
pub const NOT_HIT_LOWER_ROW: &str = "Not hit lower row";

// printer.rs
pub const WARNING: &str = "Warning";
pub const INPUT_ERROR: &str = "Input error";
pub const INPUT_ERROR_BAD_FORMAT: &str = "Invalid input format. Type '?' for help.";
pub const INPUT_ERROR_GOT: &str = "got";
pub const INPUT_ERROR_TOO_MANY_ARGUMENTS: &str = "Too many arguments. Type '?' for help.";

// parser.rs
pub const ABOUT: &str = r#"MIT License

Copyright (c) 2023 Crescendo

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


Please note that Interception Calculator IS NOT 100% accurate, as:
1. It uses gargantuar displacement data that is not 100% accurate;
2. It only takes into account gargantuars with min / max x in interception calculation, which is not 100% accurate.

In extreme cases, calculations might differ from actual results by 1~2cs.

Except for the above mentioned technical difficulties, Interception Calculator is comitted to be as close to the actual game as possible."#;

pub const HELLO: &str = r#"Source code is available under MIT license:
https://github.com/Rottenham/pvz-interception-calculator-rust

Interception Calculator v2.0.6
Current scene: Backyard.
Type '?' for help; press ↑ to show previous commands.

Calculation results are based on [cob activation -> cob interception] by default.
For [ash activation -> cob interception], subtract 1 from the results.
For [cob activation -> ash interception], add 1 to the results. "#;

// main.rs
pub const UNKNOWN_COMMAND: &str = "Unknown command. Type '?' for help.";
pub const ERROR: &str = "Error";
