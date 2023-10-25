// validate_garg_x_range
pub const GARG_X_RANGE_CANCELLED: &str =
    "gargantuars with x < 401 do not throw imps; calculation skipped.";
pub const GARG_X_RANGE_MODIFIED: &str =
    "gargantuars with x < 401 do not throw imps; x = {}~{} is used instead.";

// parse_scene
pub const SET_FRONTYARD: &str = "Scene has been set to Frontyard.";
pub const SET_BACKYARD: &str = "Scene has been set to Backyard.";
pub const SET_ROOF: &str = "Scene has been set to Roof.";

// parse_delay
pub const NEED_HIT_ROW_HIT_COL: &str = "Please provide cob hit row and cob hit col.";
pub const NEED_HIT_COL: &str = "Please provide cob hit col.";
pub const NEED_HIT_ROW_HIT_COL_COB_COL: &str =
    "Please provide cob hit row, cob hit col, and cob tail col.";
pub const NEED_HIT_COL_COB_COL: &str = "Please provide cob hit col and cob tail col.";
pub const NEED_COB_COL: &str = "Please provide cob tail col.";

// parse_doom
pub const NEED_DOOM_ROW_DOOM_COL: &str = "Please provide doom row and doom col.";
pub const NEED_DOOM_ROW: &str = "Please provide doom row.";

// parse_find_max_delay
pub const NEED_HIT_ROW_HIT_COL_RANGE: &str =
    "Please provide cob hit row and comma-separated cob hit col range.";
pub const NEED_HIT_COL_RANGE: &str = "Please provide comma-separated cob hit col range.";
pub const NEED_HIT_ROW_HIT_COL_RANGE_COB_COL: &str =
    "Please provide cob hit row, comma-separated cob hit col range, and cob tail col.";
pub const NEED_HIT_COL_RANGE_COB_COL: &str =
    "Please provide comma-separated cob hit col range and cob tail col.";
pub const CANNOT_INTERCEPT_WITHOUT_HARM: &str = "Cannot intercept without causing harm.";
pub const HIT_COL_WITH_MAX_DELAY: &str = "Cob hit col with max delay";

// parse_garg_x_range_of_imp_x
pub const NEED_IMP_X_RANGE: &str =
    "Please provide comma-separated imp x range (imp x must be integer).";
pub const IMP_X_SHOULD_BE_INTEGER: &str = "imp x should be integer";
pub const IMP_X_SHOULD_BE_IN_RANGE: &str = "should satisfy {} ≤ imp x ≤ {}";

// parse_ice_times
pub const ICE_TIMES_SHOULD_BE_INTEGER: &str = "ice times should be integer";

// parse_cob_time
pub const COB_TIME_SHOULD_BE_INTEGER: &str = "cob time should be integer";
pub const COB_TIME_SHOULD_BE_NON_NEGATIVE: &str = "cob time should ≥ 0";

// parse_delay_time
pub const DELAY_TIME_SHOULD_BE_INTEGER: &str = "delay time should be integer";

// parse hit row
pub const HIT_ROW_SHOULD_BE_INTEGER: &str = "hit row should be integer";
pub const HIT_ROW_OUT_OF_RANGE: &str = "hit row is out of range {}";

// parse hit col
pub const HIT_COL_SHOULD_BE_NUMBER: &str = "hit col should be number";
pub const HIT_COL_SHOULD_BE_IN_RANGE: &str = "should satisfy 0 ≤ hit col < 10";
pub const HIT_COL_TIMES_EIGHTY_NOT_INTEGER: &str =
    "current hit col {} * 80 is not an integer; using col {} instead.";

// parse_min_max_hit_col
pub const NEED_MIN_MAX_HIT_COL: &str = "Please provide min and max hit col.";
pub const NEED_MAX_HIT_COL: &str = "Please provide max hit col.";
pub const MIN_COL_SHOULD_BE_SMALLER_THAN_MAX_COL: &str = "should satisfy min hit col ≤ max hit col";

// parse_cob_col
pub const COB_COL_SHOULD_BE_INTEGER: &str = "cob tail col should be integer";
pub const COB_COL_SHOULD_BE_IN_RANGE: &str = "should satisfy 1 ≤ cob tail col ≤ 8";

// parse_doom_row
pub const DOOM_ROW_SHOULD_BE_INTEGER: &str = "doom row should be integer";
pub const DOOM_ROW_OUT_OF_RANGE: &str = "doom row is out of range {}";

// parse_doom_col
pub const DOOM_COL_SHOULD_BE_INTEGER: &str = "doom col should be integer";
pub const DOOM_COL_SHOULD_BE_IN_RANGE: &str = "should satisfy 1 ≤ doom col ≤ 9";

// parse_garg_pos
pub const NEED_GARG_ROWS_X_RANGE_ICE_FLAG: &str =
    "Please provide garg row(s), garg x range (optional), and ice mode (u/i, optional).";
pub const GARG_ROWS_SHOULD_BE_INTEGER: &str = "garg rows should be comma-separated integers";
pub const GARG_ROWS_ALL_OUT_OF_RANGE: &str = "all garg rows are out of range {}";

// parse_min_max_garg_x
pub const NEED_MIN_MAX_GARG_X: &str = "Please provide min and max garg x.";
pub const NEED_MAX_GARG_X: &str = "Please provide max garg x.";
pub const MIN_GARG_X_SHOULD_BE_NUMBER: &str = "min garg x should be number";
pub const MAX_GARG_X_SHOULD_BE_NUMBER: &str = "max garg x should be number";
pub const MIN_GARG_X_SHOULD_BE_SMALLER_THAN_MAX_GARG_X: &str =
    "should satisfy min garg x ≤ max garg x";
pub const MIN_GARG_X_SHOULD_BE_LARGER_THAN_LOWER_BOUND: &str = "min garg x should > {}";
pub const MAX_GARG_X_SHOULD_BE_SMALLER_THAN_UPPER_BOUND: &str = "max garg x should ≤ {}";

// parse_ice_flag
pub const ICE_FLAG_SHOULD_BE_U_OR_I: &str = "ice mode should be u or i";

// print_ice_times_and_cob_time
pub const DELAY_SETTING: &str = "Delay setting";
pub const SETTING: &str = "Setting";
pub const CANNOT_HIT_ALL_GARG: &str = "Cannot hit all gargantuars at this tick.";
pub const NO_ICE: &str = "No ice";
pub const ICE: &str = " ice";
pub const COB_EFFECTIVE: &str = " cob effective";
pub const COB_ACTIVATE: &str = " cob effective";
pub const GARG_X_RANGE: &str = "Garg x range";

// print_cob_calc_setting
pub const CALCULATION_SETTING: &str = "Calc setting";
pub const COB_GARG_ROWS: &str = "cob hit row {} for row {} gargs";
pub const COB_COL_RANGE: &str = "cob hit col {}~{}";
pub const EXPLOSION_CENTER: &str = "explosion center ";
pub const GARG: &str = "garg ";

// print_doom_calc_setting
pub const DOOM_GARG_ROWS: &str = "row {} doom for row {} gargs";

// print_eat_and_intercept
pub const INTERCEPTABLE_INTERVAL: &str = "Interceptable";
pub const CANNOT_INTERCEPT: &str = "cannot intercept";
pub const WILL_CAUSE_HARM: &str = " will cause harm";
pub const EARLIEST_EAT: &str = "Earliest eat";
pub const DOES_NOT_EAT: &str = "does not eat";
pub const EARLIEST_ICEABLE: &str = "Earliest iceable";
pub const NOT_ICEABLE: &str = "not iceable";

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

Except for the above mentioned technical difficulties, Interception Calculator is committed to be as close to the actual game as possible."#;

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
