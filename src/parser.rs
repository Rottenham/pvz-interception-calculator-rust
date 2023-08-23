use crate::game;
use crate::printer;

const DEFAULT_COB_TIME: i32 = 318;
const DEFAULT_ROOF_COB_ROW: i32 = 3;

fn validate_hit_col(hit_col: f32) -> f32 {
    if let Some(corrected_hit_col) = game::hit_col_matching_int_pixel(hit_col) {
        printer::print_warning(
            format!("该炮落点列×80不是整数, 改用{}列计算.", corrected_hit_col).as_str(),
        );
        corrected_hit_col
    } else {
        hit_col
    }
}

fn validate_garg_x_range(min_max_garg_x: &mut (f32, f32)) -> Result<game::GargXRange, ()> {
    let garg_x_range = game::validate_garg_x_range(*min_max_garg_x);
    match garg_x_range {
        game::GargXRange::Cancelled => {
            printer::print_warning("x坐标<401的巨人不会投掷小鬼, 跳过计算.");
            Err(())
        }
        game::GargXRange::Modified { min, max } => {
            printer::print_warning(
                format!("x坐标<401的巨人不会投掷小鬼, 改用{}~{}计算.", min, max).as_str(),
            );
            *min_max_garg_x = (min, max);
            Ok(garg_x_range)
        }
        game::GargXRange::Ok { min: _, max: _ } => Ok(garg_x_range),
    }
}

pub struct Parser {
    scene: game::Scene,
    ice_times: Vec<i32>,
    cob_time: i32,
    min_max_garg_x: (f32, f32),
}

pub enum ParseResult {
    Unmatched,
    Matched,
}

impl Parser {
    pub fn new() -> Parser {
        println!(
            "欢迎使用拦截计算器v2.0.\n当前场合: 后院.\n输入问号查看帮助; 按↑键显示上次输入的指令.\n\n计算结果默认为【炮激活→炮拦截】的情况.\n若为【植物激活→炮拦截】, 需额外-1; 若为【炮激活→植物拦截】, 需额外+1."
        );
        Parser {
            scene: game::Scene::PE,
            ice_times: Vec::new(),
            cob_time: DEFAULT_COB_TIME,
            min_max_garg_x: game::min_max_garg_x(&vec![], DEFAULT_COB_TIME).unwrap(),
        }
    }

    pub fn parse_help(&mut self, input: &str) -> ParseResult {
        if input.contains("help") || input.contains("?") || input.contains("？")  {
            println!("{}", HELP_TEXT);
            ParseResult::Matched
        } else {
            ParseResult::Unmatched
        }
    }

    pub fn parse_scene(&mut self, input: &str) -> ParseResult {
        match input.to_uppercase().as_str() {
            "DE" | "NE" => {
                self.scene = game::Scene::DE;
                println!("已设置为前院场合.");
                ParseResult::Matched
            }
            "PE" | "FE" => {
                self.scene = game::Scene::PE;
                println!("已设置为后院场合.");
                ParseResult::Matched
            }
            "RE" | "ME" => {
                self.scene = game::Scene::RE;
                println!("已设置为屋顶场合.");
                ParseResult::Matched
            }
            _ => ParseResult::Unmatched,
        }
    }

    pub fn parse_wave(&mut self, input: &str) -> ParseResult {
        match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["wave", extra_args @ ..] => {
                match extra_args {
                    [] => {
                        printer::print_ice_times_and_cob_time(
                            &self.ice_times,
                            self.cob_time,
                            self.min_max_garg_x,
                            false,
                        );
                    }
                    [ice_times @ .., cob_time] => {
                        match (
                            Parser::parse_ice_times(ice_times),
                            Parser::parse_cob_time(cob_time),
                        ) {
                            (Err(_), _) | (_, Err(_)) => {}
                            (Ok(ice_times), Ok(cob_time)) => {
                                let mut ice_times = ice_times
                                    .into_iter()
                                    .filter(|&x| x > 0 && x <= cob_time)
                                    .collect::<Vec<i32>>();
                                ice_times.sort();
                                match game::min_max_garg_x(&ice_times, cob_time) {
                                    Err(err) => printer::print_error(err.as_str()),
                                    Ok((min_x, max_x)) => {
                                        self.ice_times = ice_times;
                                        self.cob_time = cob_time;
                                        self.min_max_garg_x = (min_x, max_x);
                                        printer::print_ice_times_and_cob_time(
                                            &self.ice_times,
                                            self.cob_time,
                                            self.min_max_garg_x,
                                            false,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                ParseResult::Matched
            }
            _ => ParseResult::Unmatched,
        }
    }

    pub fn parse_delay(&self, input: &str) -> ParseResult {
        match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [command, extra_args @ ..] => {
                let delay_mode = match *command {
                    "delay1" => Some(game::DelayMode::Delay1),
                    "delay2" => Some(game::DelayMode::Delay2),
                    "delay3" => Some(game::DelayMode::Delay3),
                    "delay" => None,
                    _ => {
                        return ParseResult::Unmatched;
                    }
                };
                let cob_and_garg_rows: Vec<(game::Cob, Vec<i32>)> = if self.scene != game::Scene::RE
                {
                    match extra_args {
                        [] => {
                            printer::print_error("请提供炮落点列");
                            return ParseResult::Matched;
                        }
                        [hit_col, ..] => {
                            match Parser::parse_hit_col(hit_col) {
                                Err(_) => {
                                    return ParseResult::Matched;
                                }
                                Ok(hit_col) => {
                                    let hit_col = validate_hit_col(hit_col);
                                    self.scene
                                        .hit_row_and_garg_rows_of_delay_mode(&delay_mode.unwrap_or(
                                            self.scene.default_delay_mode(hit_col, None),
                                        ))
                                        .iter()
                                        .map(|(hit_row, garg_rows)| {
                                            (
                                                game::Cob::Ground {
                                                    row: *hit_row,
                                                    col: hit_col,
                                                },
                                                garg_rows.clone(),
                                            )
                                        })
                                        .collect()
                                }
                            }
                        }
                    }
                } else {
                    match extra_args {
                        [] => {
                            printer::print_error("请提供炮落点列、炮尾所在行");
                            return ParseResult::Matched;
                        }
                        [_] => {
                            printer::print_error("请提供炮尾所在行");
                            return ParseResult::Matched;
                        }
                        [hit_col, cob_col, ..] => {
                            match (
                                Parser::parse_hit_col(hit_col),
                                Parser::parse_cob_col(cob_col),
                            ) {
                                (Err(_), _) | (_, Err(_)) => {
                                    return ParseResult::Matched;
                                }
                                (Ok(hit_col), Ok(cob_col)) => {
                                    let hit_col = validate_hit_col(hit_col);
                                    self.scene
                                        .hit_row_and_garg_rows_of_delay_mode(&delay_mode.unwrap_or(
                                            self.scene.default_delay_mode(hit_col, Some(cob_col)),
                                        ))
                                        .iter()
                                        .map(|(hit_row, garg_rows)| {
                                            (
                                                game::Cob::Roof {
                                                    row: *hit_row,
                                                    col: hit_col,
                                                    cob_col,
                                                    cob_row: Some(DEFAULT_ROOF_COB_ROW),
                                                },
                                                garg_rows.clone(),
                                            )
                                        })
                                        .collect()
                                }
                            }
                        }
                    }
                };
                let explode_and_garg_rows: Vec<(game::Explode, &Vec<i32>)> = cob_and_garg_rows
                    .iter()
                    .map(|(cob, garg_rows)| (game::Explode::of_cob(cob, &self.scene), garg_rows))
                    .collect();
                let mut min_max_garg_x = self.min_max_garg_x;
                match validate_garg_x_range(&mut min_max_garg_x) {
                    Err(_) => {}
                    Ok(garg_x_range) => {
                        let (eat, intercept) = game::judge(
                            &garg_x_range,
                            &explode_and_garg_rows,
                            self.is_iced(),
                            &self.scene,
                        );
                        printer::print_cob_calc_setting(
                            &cob_and_garg_rows,
                            if (min_max_garg_x) != self.min_max_garg_x {
                                Some(min_max_garg_x)
                            } else {
                                None
                            },
                        );
                        printer::print_eat_and_intercept(&eat, &intercept);
                    }
                };
                ParseResult::Matched
            }
            _ => ParseResult::Unmatched,
        }
    }

    pub fn parse_cob(&self, input: &str) -> ParseResult {
        match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["cob", all_args @ ..] => {
                let (cob, valid_rows, extra_args) = if self.scene != game::Scene::RE {
                    match all_args {
                        [] | [">", ..] => {
                            printer::print_error("请提供炮落点行、炮落点列");
                            return ParseResult::Matched;
                        }
                        [_] | [_, ">", ..] => {
                            printer::print_error("请提供炮落点列");
                            return ParseResult::Matched;
                        }
                        [hit_row, hit_col, extra_args @ ..] => {
                            match (
                                Parser::parse_hit_row(hit_row, &self.scene.all_rows()),
                                Parser::parse_hit_col(hit_col),
                            ) {
                                (Err(_), _) | (_, Err(_)) => {
                                    return ParseResult::Matched;
                                }
                                (Ok(hit_row), Ok(hit_col)) => (
                                    game::Cob::Ground {
                                        row: hit_row,
                                        col: hit_col,
                                    },
                                    self.scene.garg_rows_for_cob(hit_row),
                                    extra_args,
                                ),
                            }
                        }
                    }
                } else {
                    match all_args {
                        [] | [">", ..] => {
                            printer::print_error("请提供炮落点行、炮落点列、炮尾所在列");
                            return ParseResult::Matched;
                        }
                        [_] | [_, ">", ..] => {
                            printer::print_error("请提供炮落点列、炮尾所在列");
                            return ParseResult::Matched;
                        }
                        [_, _] | [_, _, ">", ..] => {
                            printer::print_error("请提供炮尾所在列");
                            return ParseResult::Matched;
                        }
                        [hit_row, hit_col, cob_col, extra_args @ ..] => {
                            match (
                                Parser::parse_hit_row(hit_row, &self.scene.all_rows()),
                                Parser::parse_hit_col(hit_col),
                                Parser::parse_cob_col(cob_col),
                            ) {
                                (Err(_), _, _) | (_, Err(_), _) | (_, _, Err(_)) => {
                                    return ParseResult::Matched;
                                }
                                (Ok(hit_row), Ok(hit_col), Ok(cob_col)) => (
                                    game::Cob::Roof {
                                        row: hit_row,
                                        col: hit_col,
                                        cob_col: cob_col,
                                        cob_row: Some(DEFAULT_ROOF_COB_ROW),
                                    },
                                    self.scene.garg_rows_for_cob(hit_row),
                                    extra_args,
                                ),
                            }
                        }
                    }
                };
                let (mut min_max_garg_x, garg_rows) = match extra_args {
                    [">", garg_pos_args @ ..] => {
                        match Parser::parse_garg_pos(garg_pos_args, &valid_rows) {
                            Err(_) => {
                                return ParseResult::Matched;
                            }
                            Ok((garg_rows, min_max_garg_x)) => (min_max_garg_x, garg_rows),
                        }
                    }
                    _ => (self.min_max_garg_x, valid_rows),
                };
                match validate_garg_x_range(&mut min_max_garg_x) {
                    Err(_) => {}
                    Ok(garg_x_range) => {
                        let (eat, intercept) = game::judge(
                            &garg_x_range,
                            &vec![(game::Explode::of_cob(&cob, &self.scene), &garg_rows)],
                            self.is_iced(),
                            &self.scene,
                        );
                        printer::print_cob_calc_setting(
                            &vec![(cob, garg_rows)],
                            if min_max_garg_x != self.min_max_garg_x {
                                Some(min_max_garg_x)
                            } else {
                                None
                            },
                        );
                        printer::print_eat_and_intercept(&eat, &intercept);
                    }
                };
                ParseResult::Matched
            }
            _ => ParseResult::Unmatched,
        }
    }

    pub fn parse_doom(&self, input: &str) -> ParseResult {
        match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["doom"] => {
                printer::print_error("请提供核所在行、核所在列");
                ParseResult::Matched
            }
            ["doom", _] => {
                printer::print_error("请提供核所在列");
                ParseResult::Matched
            }
            ["doom", doom_row, doom_col, extra_args @ ..] => {
                let (doom_row, doom_col) = match (
                    Parser::parse_doom_row(doom_row, &self.scene.all_rows()),
                    Parser::parse_doom_col(doom_col),
                ) {
                    (Err(_), _) | (_, Err(_)) => {
                        return ParseResult::Matched;
                    }
                    (Ok(doom_row), Ok(doom_col)) => (doom_row, doom_col),
                };
                let (mut min_max_garg_x, garg_rows) = match extra_args {
                    [">", garg_pos_args @ ..] => match Parser::parse_garg_pos(
                        garg_pos_args,
                        &self.scene.garg_rows_for_doom(doom_row),
                    ) {
                        Err(_) => {
                            return ParseResult::Matched;
                        }
                        Ok((garg_rows, min_max_garg_x)) => (min_max_garg_x, garg_rows),
                    },
                    _ => (self.min_max_garg_x, self.scene.garg_rows_for_doom(doom_row)),
                };
                match validate_garg_x_range(&mut min_max_garg_x) {
                    Err(_) => {}
                    Ok(garg_x_range) => {
                        let (mut eat, mut intercept) = game::judge(
                            &garg_x_range,
                            &vec![(
                                game::Explode::of_doom(
                                    &game::Doom {
                                        row: doom_row,
                                        col: doom_col,
                                    },
                                    &self.scene,
                                ),
                                &garg_rows,
                            )],
                            self.is_iced(),
                            &self.scene,
                        );
                        eat.shift_to_plant_intercept();
                        intercept.shift_to_plant_intercept();
                        printer::print_doom_calc_setting(
                            doom_row,
                            &garg_rows,
                            if min_max_garg_x != self.min_max_garg_x {
                                Some(min_max_garg_x)
                            } else {
                                None
                            },
                        );
                        printer::print_eat_and_intercept(&eat, &intercept);
                    }
                };
                ParseResult::Matched
            }
            _ => ParseResult::Unmatched,
        }
    }

    pub fn parse_hit_or_nohit(&self, input: &str) -> ParseResult {
        match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["hit", extra_args @ ..] | ["nohit", extra_args @ ..] => {
                let (min_max_garg_x, cob_dist) = if self.scene != game::Scene::RE {
                    match extra_args {
                        [] => (self.min_max_garg_x, self.scene.cob_dist(None)),
                        [delay_time, ..] => match Parser::parse_delay_time(delay_time) {
                            Err(_) => {
                                return ParseResult::Matched;
                            }
                            Ok(delay_time) => {
                                match game::min_max_garg_x(
                                    &self.ice_times,
                                    self.cob_time + delay_time,
                                ) {
                                    Err(err) => {
                                        printer::print_error(err.as_str());
                                        return ParseResult::Matched;
                                    }
                                    Ok((min_garg_x, max_garg_x)) => {
                                        printer::print_ice_times_and_cob_time(
                                            &self.ice_times,
                                            self.cob_time + delay_time,
                                            (min_garg_x, max_garg_x),
                                            true,
                                        );
                                        ((min_garg_x, max_garg_x), self.scene.cob_dist(None))
                                    }
                                }
                            }
                        },
                    }
                } else {
                    match extra_args {
                        [] => {
                            printer::print_error("请提供炮尾所在列");
                            return ParseResult::Matched;
                        }
                        [cob_col] => match Parser::parse_cob_col(cob_col) {
                            Err(_) => {
                                return ParseResult::Matched;
                            }
                            Ok(cob_col) => {
                                (self.min_max_garg_x, self.scene.cob_dist(Some(cob_col)))
                            }
                        },
                        [delay_time, cob_col, ..] => {
                            match (
                                Parser::parse_delay_time(delay_time),
                                Parser::parse_cob_col(cob_col),
                            ) {
                                (Err(_), _) | (_, Err(_)) => {
                                    return ParseResult::Matched;
                                }
                                (Ok(delay), Ok(cob_col)) => match game::min_max_garg_x(
                                    &self.ice_times,
                                    self.cob_time + delay,
                                ) {
                                    Err(err) => {
                                        printer::print_error(err.as_str());
                                        return ParseResult::Matched;
                                    }
                                    Ok((min_garg_x, max_garg_x)) => {
                                        printer::print_ice_times_and_cob_time(
                                            &self.ice_times,
                                            self.cob_time + delay,
                                            (min_garg_x, max_garg_x),
                                            true,
                                        );
                                        (
                                            (min_garg_x, max_garg_x),
                                            self.scene.cob_dist(Some(cob_col)),
                                        )
                                    }
                                },
                            }
                        }
                    }
                };
                let (min_garg_x, max_garg_x) = (min_max_garg_x.0 as i32, min_max_garg_x.1 as i32);
                if input.starts_with("hit") {
                    printer::print_hit_cob_dist(&self.scene, max_garg_x, cob_dist)
                } else if input.starts_with("nohit") {
                    printer::print_nohit_cob_dist(&self.scene, min_garg_x, cob_dist);
                };
                ParseResult::Matched
            }
            _ => ParseResult::Unmatched,
        }
    }

    fn parse_ice_times(ice_times: &[&str]) -> Result<Vec<i32>, ()> {
        match ice_times
            .iter()
            .map(|&s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
        {
            Err(_) => {
                printer::print_error_with_input("用冰时机应为整数", ice_times.join(" ").as_str());
                Err(())
            }
            Ok(ice_times) => Ok(ice_times),
        }
    }

    fn parse_cob_time(cob_time: &&str) -> Result<i32, ()> {
        match cob_time.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input("激活时机应为整数", *cob_time);
                Err(())
            }
            Ok(cob_time) if cob_time < 0 => {
                printer::print_error_with_input("激活时机应≥0", cob_time.to_string().as_str());
                Err(())
            }
            Ok(cob_time) => Ok(cob_time),
        }
    }

    fn parse_delay_time(delay_time: &&str) -> Result<i32, ()> {
        match delay_time.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input("炮生效延时应为整数", delay_time);
                Err(())
            }
            Ok(delay) => Ok(delay),
        }
    }

    fn parse_hit_row(hit_row: &&str, valid_rows: &[i32]) -> Result<i32, ()> {
        match hit_row.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input("炮落点行应为整数", hit_row);
                Err(())
            }
            Ok(hit_row) if !(valid_rows.contains(&hit_row)) => {
                printer::print_error_with_input(
                    format!("炮落点行超出范围{:?}", valid_rows).as_str(),
                    hit_row.to_string().as_str(),
                );
                Err(())
            }
            Ok(hit_row) => Ok(hit_row),
        }
    }

    fn parse_hit_col(hit_col: &&str) -> Result<f32, ()> {
        match hit_col.parse::<f32>() {
            Err(_) => {
                printer::print_error_with_input("炮落点列应为数字", hit_col);
                Err(())
            }
            Ok(hit_col) if !(hit_col >= 0. && hit_col < 10.) => {
                printer::print_error_with_input(
                    "应满足0≤炮落点列<10",
                    hit_col.to_string().as_str(),
                );
                Err(())
            }
            Ok(hit_col) => Ok(hit_col),
        }
    }

    fn parse_cob_col(cob_col: &&str) -> Result<i32, ()> {
        match cob_col.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input("炮落点列应为整数", cob_col);
                Err(())
            }
            Ok(cob_col) if !(cob_col >= 1 && cob_col <= 8) => {
                printer::print_error_with_input(
                    "应满足1≤炮尾所在列≤8",
                    cob_col.to_string().as_str(),
                );
                Err(())
            }
            Ok(cob_col) => Ok(cob_col),
        }
    }

    fn parse_doom_row(doom_row: &&str, valid_rows: &[i32]) -> Result<i32, ()> {
        match doom_row.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input("核所在行应为整数", doom_row);
                Err(())
            }
            Ok(doom_row) if !(valid_rows.contains(&doom_row)) => {
                printer::print_error_with_input(
                    format!("核所在行超出范围{:?}", valid_rows).as_str(),
                    doom_row.to_string().as_str(),
                );
                Err(())
            }
            Ok(doom_row) => Ok(doom_row),
        }
    }

    fn parse_doom_col(doom_col: &&str) -> Result<i32, ()> {
        match doom_col.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input("核所在列应为整数", &doom_col);
                Err(())
            }
            Ok(doom_col) if !(doom_col >= 1 && doom_col <= 9) => {
                printer::print_error_with_input(
                    "应满足1≤核所在列≤9",
                    doom_col.to_string().as_str(),
                );
                Err(())
            }
            Ok(doom_col) => Ok(doom_col),
        }
    }

    fn parse_garg_pos(
        garg_pos_args: &[&str],
        valid_rows: &[i32],
    ) -> Result<(Vec<i32>, (f32, f32)), ()> {
        match garg_pos_args {
            [] => {
                printer::print_error("请提供巨人所在行、x坐标范围(逗号分隔多值)");
                Err(())
            }
            [_] => {
                printer::print_error("请提供巨人x坐标范围(逗号分隔最小、最大值)");
                Err(())
            }
            [garg_rows, min_max_garg_x, ..] => {
                match (
                    Parser::parse_garg_rows(garg_rows, valid_rows),
                    Parser::parse_min_max_garg_x(min_max_garg_x),
                ) {
                    (Err(_), _) | (_, Err(_)) => Err(()),
                    (Ok(garg_rows), Ok(min_max_garg_x)) => Ok((garg_rows, min_max_garg_x)),
                }
            }
        }
    }

    fn parse_garg_rows(garg_rows: &&str, valid_rows: &[i32]) -> Result<Vec<i32>, ()> {
        match garg_rows
            .replace("，", ",")
            .split(",")
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
        {
            Err(_) => {
                printer::print_error_with_input("巨人所在行应为逗号分隔的整数", garg_rows);
                Err(())
            }
            Ok(garg_rows) => {
                let filtered_garg_rows = garg_rows
                    .iter()
                    .filter(|&v| valid_rows.contains(v))
                    .cloned()
                    .collect::<Vec<i32>>();
                if filtered_garg_rows.is_empty() {
                    printer::print_error_with_input(
                        format!("巨人所在行均超出范围{:?}", valid_rows).as_str(),
                        format!("{:?}", garg_rows).as_str(),
                    );
                    Err(())
                } else {
                    Ok(filtered_garg_rows)
                }
            }
        }
    }

    fn parse_min_max_garg_x(min_max_garg_x: &&str) -> Result<(f32, f32), ()> {
        match min_max_garg_x
            .replace("，", ",")
            .split(",")
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => {
                printer::print_error("请提供巨人x坐标最小值、最大值");
                Err(())
            }
            [_] => {
                printer::print_error("请提供巨人x坐标最大值");
                Err(())
            }
            [min_garg_x, max_garg_x, ..] => {
                match (min_garg_x.parse::<f32>(), max_garg_x.parse::<f32>()) {
                    (Err(_), _) => {
                        printer::print_error_with_input("巨人x坐标最小值应为数字", min_garg_x);
                        Err(())
                    }
                    (_, Err(_)) => {
                        printer::print_error_with_input("巨人x坐标最大值应为数字", max_garg_x);
                        Err(())
                    }
                    (Ok(min_garg_x), Ok(max_garg_x)) if min_garg_x > max_garg_x => {
                        printer::print_error_with_input(
                            "应满足巨人x坐标最小值≤最大值",
                            format!("{}, {}", min_garg_x, max_garg_x).as_str(),
                        );
                        Err(())
                    }
                    (Ok(min_garg_x), _) if min_garg_x <= game::MIN_GARG_X => {
                        printer::print_error_with_input(
                            format!("应满足巨人x坐标最小值>{}", game::MIN_GARG_X).as_str(),
                            format!("{}", min_garg_x).as_str(),
                        );
                        Err(())
                    }
                    (_, Ok(max_garg_x)) if max_garg_x > game::MAX_GARG_X => {
                        printer::print_error_with_input(
                            format!("应满足巨人x坐标最大值≤{}", game::MAX_GARG_X).as_str(),
                            format!("{}", max_garg_x).as_str(),
                        );
                        Err(())
                    }
                    (Ok(min_garg_x), Ok(max_garg_x)) => Ok((min_garg_x, max_garg_x)),
                }
            }
        }
    }

    fn is_iced(&self) -> bool {
        game::is_iced(self.ice_times.last().cloned(), self.cob_time)
    }
}

const HELP_TEXT: &str = r#"
de/pe/re                         设置场合

wave                             查看当前用冰、激活时机
wave [冰时机..] [激活时机]       设置用冰、激活时机（用冰时机可为0个或多个）
                                   例：$ wave 1 400 800 -> 1、400用冰，800激活

delay [炮列数] (炮尾列)          计算可拦区间、最早啃食、最早可冰（屋顶场合需指定炮尾所在列）
                                   例：$ delay 8.8 -> 非屋顶场合计算落8.8列的拦截炮
                                       $ delay 3.5 4 -> 屋顶场合计算落3.5列的45列炮

cob [炮行数] [炮列数] (炮尾列)
  > [巨人所在行] [巨人x范围]     计算特定炮拦截特定巨人（">"及之后部分可选）
                                   例：$ cob 1 8.8 -> 计算(1, 8.8)的拦截炮（与delay类似）
                                       $ cob 1 8.8 > 2 700,800 -> 计算(1, 8.8)的炮拦截2路x为700~800的巨人
                                       $ cob 1 8.8 > 1,2 700,800 -> 计算(1, 8.8)的炮拦截1,2路x为700~800的巨人

doom [核行数] [核列数]
  > [巨人所在行] [巨人x范围]     计算核武拦截特定巨人（">"及之后部分可选）
                                   例：$ doom 3 8 -> 计算3-8核武（与delay类似）
                                       $ doom 3 8 > 2,5 700,800 -> 计算3-8核武拦截2,5路x为700~800的巨人

hit (炮尾列) (延迟)              计算刚好全伤巨人的炮落点（可指定炮延时生效时机）
                                   例：$ hit -> 计算全伤巨人的炮落点
                                       $ wave 300 \ $ hit 50 -> 计算350cs时全伤巨人的炮落点
                                       $ wave 300 \ $ hit -50 -> 计算250cs时全伤巨人的炮落点

nohit (炮尾列) (延迟)            计算刚好不伤巨人的炮落点（可指定炮延时生效时机）"#;
