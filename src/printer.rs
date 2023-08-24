use crate::game;
use std::io::Write;
use std::str;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const YELLOW: Color = Color::Rgb(229, 229, 77);

pub fn print_warning(str: &str) {
    print_colored(format!("注意: {}", str).as_str(), YELLOW);
}

fn print_colored(str: &str, color: Color) {
    let print_colored_internal = |stdout: &mut StandardStream| {
        stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(stdout, "{}", str)
    };
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    if let Err(_) = print_colored_internal(&mut stdout) {
        println!("{}", str); // 如果出现错误，改用普通打印
    }
    stdout.reset().unwrap_or_default();
    println!("");
}

pub fn print_error(error: &str) {
    println!("输入有误: {error}");
}

pub fn print_error_with_input(error: &str, input: &str) {
    println!("输入有误: {error} (当前为: {input})")
}

pub fn print_too_many_arguments_error() {
    println!("提供的参数过多. 输入问号查看帮助.");
}

pub fn print_bad_format_error() {
    println!("输入格式有误. 输入问号查看帮助.");
}

pub fn print_ice_times_and_cob_time(
    game::IceAndCobTimes {
        ice_times,
        cob_time,
    }: &game::IceAndCobTimes,
    min_max_garg_x: (f32, f32),
    delayed: bool,
) {
    let (min_garg_x, max_garg_x) = min_max_garg_x;
    if max_garg_x as i32 > 817 {
        print_warning("此时机无法全伤巨人.");
    }
    println!(
        "{}: {}{}",
        if delayed {
            "延时设定"
        } else {
            "当前设定"
        },
        match ice_times.as_slice() {
            [] => "不用冰".to_string(),
            ice_times => format!("{:?}冰", ice_times),
        },
        if delayed {
            format!(" {}炮生效", cob_time)
        } else {
            format!(" {}激活", cob_time)
        }
    );
    println!("巨人坐标范围: [{:.3}, {:.3}]", min_garg_x, max_garg_x);
}

pub fn print_cob_calc_setting(
    cob_and_garg_rows: &[(game::Cob, Vec<i32>)],
    modified_min_max_garg_x: Option<(f32, f32)>,
    cob_col_range: Option<(f32, f32)>,
) {
    println!(
        "计算设定: {}{}{}",
        cob_and_garg_rows
            .iter()
            .map(|(cob, garg_rows)| { format!("{}炮炸{:?}路", cob.row(), garg_rows) })
            .collect::<Vec<String>>()
            .join(", "),
        if let Some((min_cob_col, max_cob_col)) = cob_col_range {
            format!(", 落点{}~{}列", min_cob_col, max_cob_col)
        } else {
            "".to_string()
        },
        if let Some((min_garg_x, max_garg_x)) = modified_min_max_garg_x {
            format!(", 巨人x {}~{}", min_garg_x, max_garg_x)
        } else {
            "".to_string()
        }
    );
}

pub fn print_doom_calc_setting(
    doom_row: i32,
    garg_rows: &[i32],
    modified_min_max_garg_x: Option<(f32, f32)>,
) {
    println!(
        "计算设定: {}核炸{:?}路{}",
        doom_row,
        garg_rows,
        if let Some(modified_min_max_garg_x) = modified_min_max_garg_x {
            format!(
                ", 巨人x {}~{}",
                modified_min_max_garg_x.0, modified_min_max_garg_x.1
            )
        } else {
            "".to_string()
        }
    );
}

pub fn print_eat_and_intercept(eat: &game::Eat, intercept: &game::Intercept) {
    print!("可拦区间: ");
    match intercept {
        game::Intercept::Empty | game::Intercept::OnlyHighIndexImp | game::Intercept::Fail => {
            print_colored("无法拦截", YELLOW)
        }
        game::Intercept::Success { min, max } => {
            print!("{}~{}", min, max,);
            match game::unsafe_intercept_interval(&eat, &intercept) {
                None => println!(""),
                Some((min, max)) => {
                    print_colored(format!(" ({}~{}有伤)", min, max).as_str(), YELLOW)
                }
            }
        }
    };
    println!(
        "最早啃食: {}",
        match &eat {
            game::Eat::Empty => "不啃食".to_string(),
            game::Eat::OnlyEat(eat) | game::Eat::Both { eat, iceable: _ } => eat.to_string(),
        }
    );
    println!(
        "最早可冰: {}",
        match &eat {
            game::Eat::Empty | game::Eat::OnlyEat(_) => "不可冰".to_string(),
            game::Eat::Both { eat: _, iceable } => iceable.to_string(),
        }
    );
}

pub fn print_hit_cob_dist(scene: &game::Scene, max_garg_x: i32, cob_dist: &game::CobDist) {
    match scene {
        game::Scene::DE | game::Scene::PE => {
            println!(
                "全伤本行&下行: {} ({}列)",
                max_garg_x - cob_dist.hit_same,
                ((max_garg_x - cob_dist.hit_same) as f32) / 80.
            );
            println!(
                "全伤三行: {} ({}列)",
                max_garg_x - cob_dist.hit_above,
                ((max_garg_x - cob_dist.hit_above) as f32) / 80.
            );
        }
        game::Scene::RE => {
            println!(
                "全伤上行: {} ({}列)",
                max_garg_x - cob_dist.hit_above,
                ((max_garg_x - cob_dist.hit_above) as f32) / 80.
            );
            println!(
                "全伤本行: {} ({}列)",
                max_garg_x - cob_dist.hit_same,
                ((max_garg_x - cob_dist.hit_same) as f32) / 80.
            );
            println!(
                "全伤下行: {} ({}列)",
                max_garg_x - cob_dist.hit_below,
                ((max_garg_x - cob_dist.hit_below) as f32) / 80.
            );
        }
    }
}

pub fn print_nohit_cob_dist(scene: &game::Scene, min_garg_x: i32, cob_dist: &game::CobDist) {
    match scene {
        game::Scene::DE | game::Scene::PE => {
            println!(
                "不伤本行&下行: {} ({}列)",
                min_garg_x - cob_dist.hit_same - 1,
                ((min_garg_x - cob_dist.hit_same - 1) as f32) / 80.
            );
            println!(
                "不伤三行: {} ({}列)",
                min_garg_x - cob_dist.hit_above - 1,
                ((min_garg_x - cob_dist.hit_above - 1) as f32) / 80.
            );
        }
        game::Scene::RE => {
            println!(
                "不伤上行: {} ({}列)",
                min_garg_x - cob_dist.hit_above - 1,
                ((min_garg_x - cob_dist.hit_above - 1) as f32) / 80.
            );
            println!(
                "不伤本行: {} ({}列)",
                min_garg_x - cob_dist.hit_same - 1,
                ((min_garg_x - cob_dist.hit_same - 1) as f32) / 80.
            );
            println!(
                "不伤下行: {} ({}列)",
                min_garg_x - cob_dist.hit_below - 1,
                ((min_garg_x - cob_dist.hit_below - 1) as f32) / 80.
            );
        }
    }
}
