use crate::game;
use dyn_fmt::AsStrFormatExt;
use std::io::Write;
use std::str;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[cfg(feature = "en")]
use crate::lang::en::*;

#[cfg(feature = "zh")]
use crate::lang::zh::*;

pub fn print_warning(str: &str) {
    print_colored(format!("{}: {}", WARNING, str).as_str(), Color::Yellow);
}

fn print_colored(str: &str, color: Color) {
    let print_colored_internal = |stdout: &mut StandardStream| {
        stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
        write!(stdout, "{}", str)
    };
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    if print_colored_internal(&mut stdout).is_err() {
        println!("{}", str); // 如果出现错误，改用普通打印
    }
    stdout.reset().unwrap_or_default();
    println!();
}

pub fn print_error(error: &str) {
    println!("{INPUT_ERROR}: {error}");
}

pub fn print_error_with_input(error: &str, input: &str) {
    println!("{INPUT_ERROR}: {error} ({INPUT_ERROR_GOT}: {input})")
}

pub fn print_too_many_arguments_error() {
    println!("{INPUT_ERROR_TOO_MANY_ARGUMENTS}");
}

pub fn print_bad_format_error() {
    println!("{INPUT_ERROR_BAD_FORMAT}");
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
        print_warning(CANNOT_HIT_ALL_GARG);
    }
    println!(
        "{}: {}{}",
        if delayed { DELAY_SETTING } else { SETTING },
        match ice_times.as_slice() {
            [] => NO_ICE.to_string(),
            ice_times => format!("{:?}{ICE}", ice_times),
        },
        if delayed {
            format!(" {}{COB_EFFECTIVE}", cob_time)
        } else {
            format!(" {}{COB_ACTIVATE}", cob_time)
        }
    );
    println!("{GARG_X_RANGE}: [{:.3}, {:.3}]", min_garg_x, max_garg_x);
}

pub fn print_cob_calc_setting(
    cob_and_garg_rows: &[(game::Cob, Vec<i32>)],
    explode: Option<game::Explode>,
    modified_min_max_garg_x: Option<(f32, f32)>,
    cob_col_range: Option<(f32, f32)>,
) {
    println!(
        "{CALCULATION_SETTING}: {}{}{}{}",
        cob_and_garg_rows
            .iter()
            .map(|(cob, garg_rows)| {
                COB_GARG_ROWS.format(&[cob.row().to_string(), format!("{:?}", garg_rows)])
            })
            .collect::<Vec<String>>()
            .join(", "),
        if let Some((min_cob_col, max_cob_col)) = cob_col_range {
            ", ".to_owned() + &COB_COL_RANGE.format(&[min_cob_col, max_cob_col])
        } else {
            "".to_string()
        },
        if let Some(explode) = explode {
            format!(
                ", {EXPLOSION_CENTER}x={} y={}",
                explode.range.center.x, explode.range.center.y
            )
        } else {
            "".to_string()
        },
        if let Some((min_garg_x, max_garg_x)) = modified_min_max_garg_x {
            format!(", {GARG}x={}~{}", min_garg_x, max_garg_x)
        } else {
            "".to_string()
        }
    );
}

pub fn print_doom_calc_setting(
    doom_row: i32,
    garg_rows: &[i32],
    explode: Option<&game::Explode>,
    modified_min_max_garg_x: Option<(f32, f32)>,
) {
    println!(
        "{CALCULATION_SETTING}: {}{}{}",
        DOOM_GARG_ROWS.format(&[doom_row.to_string(), format!("{:?}", garg_rows)]),
        if let Some(explode) = explode {
            format!(
                ", {EXPLOSION_CENTER}x={} y={}",
                explode.range.center.x, explode.range.center.y
            )
        } else {
            "".to_string()
        },
        if let Some(modified_min_max_garg_x) = modified_min_max_garg_x {
            format!(
                ", {GARG}x={}~{}",
                modified_min_max_garg_x.0, modified_min_max_garg_x.1
            )
        } else {
            "".to_string()
        }
    );
}

pub fn print_eat_and_intercept(eat: &game::Eat, intercept: &game::Intercept) {
    print!("{INTERCEPTABLE_INTERVAL}: ");
    match intercept {
        game::Intercept::Empty | game::Intercept::OnlyHighIndexImp | game::Intercept::Fail => {
            print_colored(CANNOT_INTERCEPT, Color::Yellow)
        }
        game::Intercept::Success { min, max } => {
            print!("{}~{}", min, max,);
            match game::unsafe_intercept_interval(eat, intercept) {
                None => println!(),
                Some((min, max)) => print_colored(
                    format!(" ({}~{}{WILL_CAUSE_HARM})", min, max).as_str(),
                    Color::Yellow,
                ),
            }
        }
    };
    println!(
        "{EARLIEST_EAT}: {}",
        match &eat {
            game::Eat::Empty => DOES_NOT_EAT.to_string(),
            game::Eat::OnlyEat(eat) | game::Eat::Both { eat, iceable: _ } => eat.to_string(),
        }
    );
    println!(
        "{EARLIEST_ICEABLE}: {}",
        match &eat {
            game::Eat::Empty | game::Eat::OnlyEat(_) => NOT_ICEABLE.to_string(),
            game::Eat::Both { eat: _, iceable } => iceable.to_string(),
        }
    );
}

pub fn print_hit_cob_dist(scene: &game::Scene, max_garg_x: i32, cob_dist: &game::CobDist) {
    match scene {
        game::Scene::DE | game::Scene::PE => {
            println!(
                "{HIT_SAME_AND_LOWER}: {} ({})",
                max_garg_x - cob_dist.hit_same,
                COL.format(&[((max_garg_x - cob_dist.hit_same) as f32) / 80.])
            );
            println!(
                "{HIT_ALL_THREE_ROWS}: {} ({})",
                max_garg_x - cob_dist.hit_above,
                COL.format(&[((max_garg_x - cob_dist.hit_above) as f32) / 80.])
            );
        }
        game::Scene::RE => {
            println!(
                "{HIT_UPPER_ROW}: {} ({})",
                max_garg_x - cob_dist.hit_above,
                COL.format(&[((max_garg_x - cob_dist.hit_above) as f32) / 80.])
            );
            println!(
                "{HIT_SAME_ROW}: {} ({})",
                max_garg_x - cob_dist.hit_same,
                COL.format(&[((max_garg_x - cob_dist.hit_same) as f32) / 80.])
            );
            println!(
                "{HIT_LOWER_ROW}: {} ({})",
                max_garg_x - cob_dist.hit_below,
                COL.format(&[((max_garg_x - cob_dist.hit_below) as f32) / 80.])
            );
        }
    }
}

pub fn print_nohit_cob_dist(scene: &game::Scene, min_garg_x: i32, cob_dist: &game::CobDist) {
    match scene {
        game::Scene::DE | game::Scene::PE => {
            println!(
                "{NOT_HIT_SAME_AND_LOWER}: {} ({})",
                min_garg_x - cob_dist.hit_same - 1,
                COL.format(&[((min_garg_x - cob_dist.hit_same - 1) as f32) / 80.])
            );
            println!(
                "{NOT_HIT_UPPER_ROW}: {} ({})",
                min_garg_x - cob_dist.hit_above - 1,
                COL.format(&[((min_garg_x - cob_dist.hit_above - 1) as f32) / 80.])
            );
        }
        game::Scene::RE => {
            println!(
                "{NOT_HIT_UPPER_ROW}: {} ({})",
                min_garg_x - cob_dist.hit_above - 1,
                COL.format(&[((min_garg_x - cob_dist.hit_above - 1) as f32) / 80.])
            );
            println!(
                "{NOT_HIT_SAME_ROW}: {} ({})",
                min_garg_x - cob_dist.hit_same - 1,
                COL.format(&[((min_garg_x - cob_dist.hit_same - 1) as f32) / 80.])
            );
            println!(
                "{NOT_HIT_LOWER_ROW}: {} ({})",
                min_garg_x - cob_dist.hit_below - 1,
                COL.format(&[((min_garg_x - cob_dist.hit_below - 1) as f32) / 80.])
            );
        }
    }
}
