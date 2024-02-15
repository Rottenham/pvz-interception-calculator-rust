// validate_garg_x_range
pub const GARG_X_RANGE_CANCELLED: &str = "x坐标<401的巨人不会投掷小鬼, 跳过计算.";
pub const GARG_X_RANGE_MODIFIED: &str = "x坐标<401的巨人不会投掷小鬼, 改用{}~{}计算.";

// parse_scene
pub const SET_FRONTYARD: &str = "已设置为前院场合.";
pub const SET_BACKYARD: &str = "已设置为后院场合.";
pub const SET_ROOF: &str = "已设置为屋顶场合.";

// parse_delay
pub const NEED_HIT_ROW_HIT_COL: &str = "请提供炮落点行、炮落点列";
pub const NEED_HIT_COL: &str = "请提供炮落点列";
pub const NEED_HIT_ROW_HIT_COL_COB_COL: &str = "请提供炮落点行、炮落点列、炮尾所在列";
pub const NEED_HIT_COL_COB_COL: &str = "请提供炮落点列、炮尾所在列";
pub const NEED_COB_COL: &str = "请提供炮尾所在列";

// parse_doom
pub const NEED_DOOM_ROW_DOOM_COL: &str = "请提供核所在行、核所在列";
pub const NEED_DOOM_ROW: &str = "请提供核所在列";

// parse_find_max_delay
pub const NEED_HIT_ROW_HIT_COL_RANGE: &str = "请提供炮落点行、炮落点列范围(逗号分隔)";
pub const NEED_HIT_COL_RANGE: &str = "请提供炮落点列范围(逗号分隔最小、最大值)";
pub const NEED_HIT_ROW_HIT_COL_RANGE_COB_COL: &str =
    "请提供炮落点行、炮落点列范围(逗号分隔)、炮尾所在列";
pub const NEED_HIT_COL_RANGE_COB_COL: &str = "请提供炮落点列范围(逗号分隔最小、最大值)、炮尾所在列";
pub const CANNOT_INTERCEPT_WITHOUT_HARM: &str = "无法无伤拦截.";
pub const HIT_COL_WITH_MAX_DELAY: &str = "延迟最大的炮落点";

// parse_garg_x_range_of_imp_x
pub const NEED_IMP_X_RANGE: &str = "请提供小鬼x坐标(整数)";
pub const IMP_X_SHOULD_BE_INTEGER: &str = "小鬼x坐标应为整数";
pub const IMP_X_SHOULD_BE_IN_RANGE: &str = "应满足{}≤小鬼x坐标≤{}";

// parse_ice_times
pub const ICE_TIMES_SHOULD_BE_INTEGER: &str = "用冰时机应为整数";

// parse_cob_time
pub const COB_TIME_SHOULD_BE_INTEGER: &str = "激活时机应为整数";
pub const COB_TIME_SHOULD_BE_NON_NEGATIVE: &str = "激活时机应≥0";

// parse_delay_time
pub const DELAY_TIME_SHOULD_BE_INTEGER: &str = "炮生效延时应为整数";

// parse hit row
pub const HIT_ROW_SHOULD_BE_INTEGER: &str = "炮落点行应为整数";
pub const HIT_ROW_OUT_OF_RANGE: &str = "炮落点行超出范围{}";

// parse hit col
pub const HIT_COL_SHOULD_BE_NUMBER: &str = "炮落点列应为数字";
pub const HIT_COL_SHOULD_BE_IN_RANGE: &str = "应满足0≤炮落点列<10";
pub const HIT_COL_TIMES_EIGHTY_NOT_INTEGER: &str = "当前落点列{}×80不是整数, 改用{}列计算.";

// parse_min_max_hit_col
pub const NEED_MIN_MAX_HIT_COL: &str = "请提供炮落点列最小值、最大值";
pub const NEED_MAX_HIT_COL: &str = "请提供炮落点列最大值";
pub const MIN_COL_SHOULD_BE_SMALLER_THAN_MAX_COL: &str = "应满足炮落点列最小值≤最大值";

// parse_cob_col
pub const COB_COL_SHOULD_BE_INTEGER: &str = "炮尾所在列应为整数";
pub const COB_COL_SHOULD_BE_IN_RANGE: &str = "应满足1≤炮尾所在列≤8";

// parse_doom_row
pub const DOOM_ROW_SHOULD_BE_INTEGER: &str = "核所在行应为整数";
pub const DOOM_ROW_OUT_OF_RANGE: &str = "核所在行超出范围{}";

// parse_doom_col
pub const DOOM_COL_SHOULD_BE_INTEGER: &str = "核所在列应为整数";
pub const DOOM_COL_SHOULD_BE_IN_RANGE: &str = "应满足1≤核所在列≤9";

// parse_garg_pos
pub const NEED_GARG_ROWS_X_RANGE_ICE_FLAG: &str =
    "请提供巨人所在行、x坐标范围(可选)、速度模式(u/i, 可选)";
pub const GARG_ROWS_SHOULD_BE_INTEGER: &str = "巨人所在行应为逗号分隔的整数";
pub const GARG_ROWS_ALL_OUT_OF_RANGE: &str = "巨人所在行均超出范围{}";

// parse_min_max_garg_x
pub const NEED_MIN_MAX_GARG_X: &str = "请提供巨人x坐标最小值、最大值";
pub const NEED_MAX_GARG_X: &str = "请提供巨人x坐标最大值";
pub const MIN_GARG_X_SHOULD_BE_NUMBER: &str = "巨人x坐标最小值应为数字";
pub const MAX_GARG_X_SHOULD_BE_NUMBER: &str = "巨人x坐标最大值应为数字";
pub const MIN_GARG_X_SHOULD_BE_SMALLER_THAN_MAX_GARG_X: &str = "应满足巨人x坐标最小值≤最大值";
pub const MIN_GARG_X_SHOULD_BE_LARGER_THAN_LOWER_BOUND: &str = "应满足巨人x坐标最小值>{}";
pub const MAX_GARG_X_SHOULD_BE_SMALLER_THAN_UPPER_BOUND: &str = "应满足巨人x坐标最大值≤{}";

// parse_ice_flag
pub const ICE_FLAG_SHOULD_BE_U_OR_I: &str = "计算模式应为u/i(原速/减速)";

// print_ice_times_and_cob_time
pub const DELAY_SETTING: &str = "延时设定";
pub const SETTING: &str = "当前设定";
pub const CANNOT_HIT_ALL_GARG: &str = "此时机无法全伤巨人.";
pub const NO_ICE: &str = "不用冰";
pub const ICE: &str = "冰";
pub const COB_EFFECTIVE: &str = "炮生效";
pub const COB_ACTIVATE: &str = "激活";
pub const GARG_X_RANGE: &str = "巨人坐标范围";

// print_cob_calc_setting
pub const CALCULATION_SETTING: &str = "计算设定";
pub const COB_GARG_ROWS: &str = "{}炮炸{}路";
pub const COB_COL_RANGE: &str = "落点{}~{}列";
pub const EXPLOSION_CENTER: &str = "爆心";
pub const GARG: &str = "巨人";

// print_doom_calc_setting
pub const DOOM_GARG_ROWS: &str = "{}核炸{}路";

// print_eat_and_intercept
pub const INTERCEPTABLE_INTERVAL: &str = "可拦区间";
pub const CANNOT_INTERCEPT: &str = "无法拦截";
pub const WILL_CAUSE_HARM: &str = "有伤";
pub const EARLIEST_EAT: &str = "最早啃食";
pub const DOES_NOT_EAT: &str = "不啃食";
pub const EARLIEST_ICEABLE: &str = "最早可冰";
pub const NOT_ICEABLE: &str = "不可冰";

// print_hit_cob_dist
pub const COL: &str = "{}列";
pub const HIT_SAME_AND_LOWER: &str = "全伤本行&下行";
pub const HIT_ALL_THREE_ROWS: &str = "全伤三行";
pub const HIT_UPPER_ROW: &str = "全伤上行";
pub const HIT_SAME_ROW: &str = "全伤本行";
pub const HIT_LOWER_ROW: &str = "全伤下行";

// print_nohit_cob_dist
pub const NOT_HIT_SAME_AND_LOWER: &str = "不伤本行&下行";
pub const NOT_HIT_UPPER_ROW: &str = "不伤上行";
pub const NOT_HIT_SAME_ROW: &str = "不伤本行";
pub const NOT_HIT_LOWER_ROW: &str = "不伤下行";

// printer.rs
pub const WARNING: &str = "注意";
pub const INPUT_ERROR: &str = "输入有误";
pub const INPUT_ERROR_BAD_FORMAT: &str = "输入格式有误. 输入问号查看帮助.";
pub const INPUT_ERROR_GOT: &str = "当前为";
pub const INPUT_ERROR_TOO_MANY_ARGUMENTS: &str = "提供的参数过多. 输入问号查看帮助.";

// parser.rs
pub const ABOUT: &str = r#"MIT 许可证

版权 (c) 2023 Crescendo

特此免费授予任何获得本软件副本和相关文档文件(下称“软件”)的人不受限制地处置该软件的权利，包括不
受限制地使用、复制、修改、合并、发布、分发、转授许可和/或出售该软件副本，以及再授权被配发了本软
件的人如上的权利，惟须遵守条件如下：

上述版权声明和本许可声明应包含在该软件的所有副本或实质成分中。

本软件是“如此”提供的，没有任何形式的明示或暗示的保证，包括但不限于对适销性、特定用途的适用性和不
侵权的保证。在任何情况下，作者或版权持有人都不对任何索赔、损害或其他责任负责，无论这些追责来自合
同、侵权或其它行为中，还是产生于、源于或有关于本软件以及本软件的使用或其它处置。


请注意，拦截计算器_无法确保_100%的计算精度，其原因包括：
1. 所用的巨人位移数据并非100%精确；
2. 所用的仅取坐标极值的拦截区间计算方式并非100%精确。

在极端情况下，计算结果与实际情况可能存在1~2cs左右的偏差，敬请谅解。

除上述在技术上难以解决的问题外，拦截计算器约定在能力所及的范围内尽可能接近游戏情况。"#;

pub const HELLO: &str = r#"本程序源码以MIT许可证发布:
https://github.com/Rottenham/pvz-interception-calculator-rust

欢迎使用拦截计算器v2.0.10.
当前场合: 后院.
输入问号查看帮助; 按↑键显示上次输入的指令.

计算结果默认为炮激活的情况. 若为植物激活, 需额外-1."#;

pub const HELP: &str = r#"
de/pe/re                            设置场合

wave                                查看当前用冰、激活时机
wave 冰时机.. 激活时机              设置用冰、激活时机(用冰时机可为0个或多个)
                                例：$ wave 1 400 800 -> 1、400用冰, 800激活

delay 炮列数 (炮尾列)               计算可拦区间、最早啃食、最早可冰
                                    (屋顶场合需指定炮尾所在列)
                                例：$ delay 8.8 -> 非屋顶计算落8.8列的拦截炮
                                    $ delay 3.5 4 -> 屋顶计算落3.5列的45列炮
delay 炮行数 炮列数 (炮尾列)
  > 巨人所在行 (巨人x范围) (u/i)    计算炮拦截特定巨人(可指定按原速/减速计算)
                                例：$ delay 1 8.8 > 2 -> (1,8.8)炮拦2路巨人
                                    $ delay 1 8.8 > 1,2 700,800 ->
                                        计算(1,8.8)炮拦截1、2路x为700~800的巨人
                                    $ delay 1 8.8 > 1,2 700,800 u ->
                                        同上，但指定按原速计算

doom 核行数 核列数
  (> 巨人所在行 (巨人x范围) (u/i))  计算核武拦截特定巨人
                                    (">"及之后部分可选, 可指定按原速/减速计算)
                                例：$ doom 3 8 -> 计算3-8核武
                                    $ doom 3 8 > 2,5 700,800 ->
                                        计算3-8核武拦截2、5路x为700~800的巨人

hit (炮尾列) (延迟)                 计算刚好全伤巨人的炮落点(可指定炮延时生效)
                                例：$ hit -> 计算全伤巨人的炮落点
                                    $ wave 300 $ hit 50 ->
                                        计算350cs时全伤巨人的炮落点
                                    $ wave 300 $ hit -50 ->
                                        计算250cs时全伤巨人的炮落点

nohit (炮尾列) (延迟)               计算刚好不伤巨人的炮落点(可指定炮延时生效)

max 炮行数 炮列数范围
  > 巨人所在行 (巨人x范围) (u/i)    寻找无伤拦截可延迟最多的炮落点列
                                    (可指定按原速/减速计算)
                                例：$ max 1 7,7.5 > 1,2 ->
                                        找1路7~7.5列炮拦1、2路巨人延迟最多的落点

imp 小鬼x坐标                       计算投掷该坐标小鬼的巨人x范围

?/help                              显示此帮助
about                               关于拦截计算器"#;

// main.rs
pub const UNKNOWN_COMMAND: &str = "未知指令. 输入问号查看帮助.";
pub const ERROR: &str = "出现错误";

// game.rs
pub const GARG_MIN_WALK_OUT_OF_RANGE: &str = "巨人最短行走时间[{}]超出数据范围({}~{})";
pub const GARG_MAX_WALK_OUT_OF_RANGE: &str = "巨人最长行走时间[{}]超出数据范围({}~{})";
