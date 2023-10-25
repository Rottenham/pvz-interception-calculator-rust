pub const CANNOT_HIT_ALL_GARG: &str = "此时机无法全伤巨人.";

pub const GARG_X_RANGE_CANCELLED: &str = "x坐标<401的巨人不会投掷小鬼, 跳过计算.";

pub const GARG_X_RANGE_MODIFIED: &str = "x坐标<401的巨人不会投掷小鬼, 改用{}~{}计算.";

// print_ice_times_and_cob_time
pub const DELAY_SETTING: &str = "延时设定";
pub const SETTING: &str = "当前设定";
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

特此免费授予任何获得本软件副本和相关文档文件（下称“软件”）的人不受限制地处置该软件的权利，包括不受限制地使用、复制、修改、合并、发布、分发、转授许可和/或出售该软件副本，以及再授权被配发了本软件的人如上的权利，惟须遵守条件如下：

上述版权声明和本许可声明应包含在该软件的所有副本或实质成分中。

本软件是“如此”提供的，没有任何形式的明示或暗示的保证，包括但不限于对适销性、特定用途的适用性和不侵权的保证。在任何情况下，作者或版权持有人都不对任何索赔、损害或其他责任负责，无论这些追责来自合同、侵权或其它行为中，还是产生于、源于或有关于本软件以及本软件的使用或其它处置。


请注意，拦截计算器_无法确保_100%的计算精度，其原因包括：
1. 所用的巨人位移数据并非100%精确；
2. 所用的仅取坐标极值的拦截区间计算方式并非100%精确。

在极端情况下，计算结果与实际情况可能存在1~2cs左右的偏差，敬请谅解。

除上述在技术上难以解决的问题外，拦截计算器约定在能力所及的范围内尽可能接近游戏情况。"#;

pub const HELLO: &str = r#"本程序源码以MIT许可证发布:
https://github.com/Rottenham/pvz-interception-calculator-rust

欢迎使用拦截计算器v2.0.6.
当前场合: 后院.
输入问号查看帮助; 按↑键显示上次输入的指令.

计算结果默认为【炮激活→炮拦截】的情况.
若为【植物激活→炮拦截】, 需额外-1; 若为【炮激活→植物拦截】, 需额外+1."#;
