use crate::constants;
use crate::game;
use crate::printer;

const DEFAULT_SCENE: game::Scene = game::Scene::PE;
const DEFAULT_COB_TIME: i32 = 318;
const DEFAULT_ROOF_COB_ROW: i32 = 3;

fn validate_garg_x_range(min_max_garg_x: &mut (f32, f32)) -> Result<game::GargXRange, ()> {
    match game::GargXRange::of_min_max_garg_pos(*min_max_garg_x) {
        game::GargXRange::Cancelled => {
            printer::print_warning("x坐标<401的巨人不会投掷小鬼, 跳过计算.");
            Err(())
        }
        game::GargXRange::Modified { min, max } => {
            printer::print_warning(
                format!("x坐标<401的巨人不会投掷小鬼, 改用{}~{}计算.", min, max).as_str(),
            );
            *min_max_garg_x = (min, max);
            Ok(game::GargXRange::Modified { min, max })
        }
        game::GargXRange::Ok { min, max } => Ok(game::GargXRange::Ok { min, max }),
    }
}

pub struct Parser {
    scene: game::Scene,
    ice_and_cob_times: game::IceAndCobTimes,
    min_max_garg_x: (f32, f32),
}

pub enum ParseResult {
    Unmatched,
    Matched,
}

impl Parser {
    pub fn new() -> Parser {
        println!("{}", HELLO_TEXT);
        let scene = DEFAULT_SCENE;
        let ice_and_cob_times =
            game::IceAndCobTimes::of_ice_times_and_cob_time(&vec![], DEFAULT_COB_TIME).unwrap();
        let min_max_garg_x = game::min_max_garg_x(&ice_and_cob_times).unwrap();
        Parser {
            scene,
            ice_and_cob_times,
            min_max_garg_x,
        }
    }

    pub fn parse_help(&self, input: &str) -> ParseResult {
        if input == "help" || input == "?" || input == "？" {
            println!("{}", HELP_TEXT);
            ParseResult::Matched
        } else {
            ParseResult::Unmatched
        }
    }

    pub fn parse_about(&self, input: &str) -> ParseResult {
        if input == "about" {
            println!("{}", ABOUT_TEXT);
            ParseResult::Matched
        } else {
            ParseResult::Unmatched
        }
    }

    pub fn parse_scene(&mut self, input: &str) -> ParseResult {
        match input {
            "de" | "ne" => {
                self.scene = game::Scene::DE;
                println!("已设置为前院场合.");
                ParseResult::Matched
            }
            "pe" | "fe" => {
                self.scene = game::Scene::PE;
                println!("已设置为后院场合.");
                ParseResult::Matched
            }
            "re" | "me" => {
                self.scene = game::Scene::RE;
                println!("已设置为屋顶场合.");
                ParseResult::Matched
            }
            _ => ParseResult::Unmatched,
        }
    }

    pub fn parse_wave(&mut self, input: &str) -> ParseResult {
        match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["wave", all_args @ ..] => {
                match all_args {
                    [] => {
                        printer::print_ice_times_and_cob_time(
                            &self.ice_and_cob_times,
                            self.min_max_garg_x,
                            false,
                        );
                    }
                    [ice_times @ .., cob_time] => {
                        let Ok ((ice_times, cob_time)) = combine_results(
                            Parser::parse_ice_times(ice_times),
                            Parser::parse_cob_time(cob_time),
                        ) else {
                            return ParseResult::Matched;
                        };
                        match game::IceAndCobTimes::of_ice_times_and_cob_time(&ice_times, cob_time)
                        {
                            Err(err) => {
                                printer::print_error(err.as_str());
                            }
                            Ok(ice_and_cob_times) => {
                                match game::min_max_garg_x(&ice_and_cob_times) {
                                    Err(err) => printer::print_error(err.as_str()),
                                    Ok((min_x, max_x)) => {
                                        self.ice_and_cob_times = ice_and_cob_times;
                                        self.min_max_garg_x = (min_x, max_x);
                                        printer::print_ice_times_and_cob_time(
                                            &self.ice_and_cob_times,
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
            [command, all_args @ ..] => {
                let delay_mode = match *command {
                    "delay1" => Some(game::DelayMode::Delay1),
                    "delay2" => Some(game::DelayMode::Delay2),
                    "delay3" => Some(game::DelayMode::Delay3),
                    "delay" => None,
                    _ => {
                        return ParseResult::Unmatched;
                    }
                };
                let (cob_and_garg_rows, mut min_max_garg_x, ice_flag, explode_to_print): (
                    Vec<(game::Cob, Vec<i32>)>,
                    _,
                    _,
                    _,
                ) = if self.scene != game::Scene::RE {
                    match all_args {
                        [">", ..] if *command == "delay" => {
                            printer::print_error("请提供炮落点行、炮落点列");
                            return ParseResult::Matched;
                        }
                        [_, ">", ..] if *command == "delay" => {
                            printer::print_error("请提供炮落点列");
                            return ParseResult::Matched;
                        }
                        [hit_row, hit_col, ">", garg_pos_args @ ..] if *command == "delay" => {
                            let Ok((hit_row, hit_col)) =  combine_results(
                                    Parser::parse_hit_row(hit_row, &self.scene.all_rows()),
                                    Parser::parse_hit_col(hit_col),
                                ) else {
                                    return ParseResult::Matched;
                                };
                            let Ok((garg_rows, min_max_garg_x, ice_flag)) = Parser::parse_garg_pos(
                                        garg_pos_args,
                                        &self.scene.garg_rows_for_cob(hit_row),
                                ) else {
                                    return ParseResult::Matched;
                                };
                            let cob = game::Cob::Ground {
                                row: hit_row,
                                col: hit_col,
                            };
                            (
                                vec![(cob.clone(), garg_rows)],
                                min_max_garg_x.unwrap_or(self.min_max_garg_x),
                                ice_flag.unwrap_or(self.ice_and_cob_times.is_iced()),
                                Some(game::Explode::of_cob(&cob, &self.scene)),
                            )
                        }
                        [] => {
                            printer::print_error("请提供炮落点列");
                            return ParseResult::Matched;
                        }
                        [hit_col] => {
                            let Ok(hit_col) = Parser::parse_hit_col(hit_col) else {
                                    return ParseResult::Matched;
                                };
                            (
                                self.scene
                                    .hit_row_and_garg_rows_of_delay_mode(
                                        &delay_mode.unwrap_or(
                                            self.scene.default_delay_mode(hit_col, None),
                                        ),
                                    )
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
                                    .collect(),
                                self.min_max_garg_x,
                                self.ice_and_cob_times.is_iced(),
                                None,
                            )
                        }
                        _ => {
                            printer::print_too_many_arguments_error();
                            return ParseResult::Matched;
                        }
                    }
                } else {
                    match all_args {
                        [">", ..] if *command == "delay" => {
                            printer::print_error("请提供炮落点行、炮落点列、炮尾所在列");
                            return ParseResult::Matched;
                        }
                        [_, ">", ..] if *command == "delay" => {
                            printer::print_error("请提供炮落点列、炮尾所在列");
                            return ParseResult::Matched;
                        }
                        [_, _, ">", ..] if *command == "delay" => {
                            printer::print_error("请提供炮尾所在列");
                            return ParseResult::Matched;
                        }
                        [hit_row, hit_col, cob_col, ">", garg_pos_args @ ..]
                            if *command == "delay" =>
                        {
                            let Ok((hit_row, hit_col, cob_col)) = combine_results3(
                                    Parser::parse_hit_row(hit_row, &self.scene.all_rows()),
                                    Parser::parse_hit_col(hit_col),
                                    Parser::parse_cob_col(cob_col),
                                ) else {
                                    return ParseResult::Matched;
                                };
                            let Ok((garg_rows, min_max_garg_x, ice_flag)) = Parser::parse_garg_pos(
                                    garg_pos_args,
                                    &self.scene.garg_rows_for_cob(hit_row),
                                ) else {
                                    return ParseResult::Matched;
                                };
                            let cob = game::Cob::Roof {
                                row: hit_row,
                                col: hit_col,
                                cob_col: cob_col,
                                cob_row: DEFAULT_ROOF_COB_ROW,
                            };
                            (
                                vec![(cob.clone(), garg_rows)],
                                min_max_garg_x.unwrap_or(self.min_max_garg_x),
                                ice_flag.unwrap_or(self.ice_and_cob_times.is_iced()),
                                Some(game::Explode::of_cob(&cob, &self.scene)),
                            )
                        }
                        [] => {
                            printer::print_error("请提供炮落点列、炮尾所在行");
                            return ParseResult::Matched;
                        }
                        [_] => {
                            printer::print_error("请提供炮尾所在行");
                            return ParseResult::Matched;
                        }
                        [hit_col, cob_col] => {
                            let Ok((hit_col, cob_col)) = combine_results(
                                    Parser::parse_hit_col(hit_col),
                                    Parser::parse_cob_col(cob_col),
                                ) else {
                                    return ParseResult::Matched;
                                };
                            (
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
                                                cob_row: DEFAULT_ROOF_COB_ROW,
                                            },
                                            garg_rows.clone(),
                                        )
                                    })
                                    .collect(),
                                self.min_max_garg_x,
                                self.ice_and_cob_times.is_iced(),
                                None,
                            )
                        }
                        _ => {
                            printer::print_too_many_arguments_error();
                            return ParseResult::Matched;
                        }
                    }
                };
                let Ok(garg_x_range) = validate_garg_x_range(&mut min_max_garg_x)
                else {
                    return ParseResult::Matched;
                };
                let explode_and_garg_rows: Vec<(game::Explode, &Vec<i32>)> = cob_and_garg_rows
                    .iter()
                    .map(|(cob, garg_rows)| (game::Explode::of_cob(cob, &self.scene), garg_rows))
                    .collect();
                let (eat, intercept) =
                    game::judge(&garg_x_range, &explode_and_garg_rows, ice_flag, &self.scene);
                printer::print_cob_calc_setting(
                    &cob_and_garg_rows,
                    explode_to_print,
                    if (min_max_garg_x) != self.min_max_garg_x {
                        Some(min_max_garg_x)
                    } else {
                        None
                    },
                    None,
                );
                printer::print_eat_and_intercept(&eat, &intercept);
                ParseResult::Matched
            }
            _ => ParseResult::Unmatched,
        }
    }

    pub fn parse_doom(&self, input: &str) -> ParseResult {
        match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["doom", all_args @ ..] => match all_args {
                [] => {
                    printer::print_error("请提供核所在行、核所在列");
                    ParseResult::Matched
                }
                [_] => {
                    printer::print_error("请提供核所在列");
                    ParseResult::Matched
                }
                [doom_row, doom_col, extra_args @ ..] => {
                    let Ok((doom_row, doom_col)) = combine_results(
                    Parser::parse_doom_row(doom_row, &self.scene.all_rows()),
                    Parser::parse_doom_col(doom_col),
                    ) else {
                        return ParseResult::Matched;
                    };
                    let explode = game::Explode::of_doom(
                        &game::Doom {
                            row: doom_row,
                            col: doom_col,
                        },
                        &self.scene,
                    );
                    let (garg_rows, mut min_max_garg_x, ice_flag, explode_to_print) =
                        match extra_args {
                            [] => (
                                self.scene.garg_rows_for_doom(doom_row),
                                self.min_max_garg_x,
                                self.ice_and_cob_times.is_iced(),
                                None,
                            ),
                            [">", garg_pos_args @ ..] => {
                                let Ok((garg_rows, min_max_garg_x, ice_flag)) = Parser::parse_garg_pos(
                                    garg_pos_args,
                                    &self.scene.garg_rows_for_doom(doom_row),
                                ) else {
                                    return ParseResult::Matched;
                                };
                                (
                                    garg_rows,
                                    min_max_garg_x.unwrap_or(self.min_max_garg_x),
                                    ice_flag.unwrap_or(self.ice_and_cob_times.is_iced()),
                                    Some(&explode),
                                )
                            }
                            _ => {
                                printer::print_too_many_arguments_error();
                                return ParseResult::Matched;
                            }
                        };
                    let Ok(garg_x_range) = validate_garg_x_range(&mut min_max_garg_x)
                    else {
                        return ParseResult::Matched;
                    };
                    let (mut eat, mut intercept) = game::judge(
                        &garg_x_range,
                        &vec![(explode.clone(), &garg_rows)],
                        ice_flag,
                        &self.scene,
                    );
                    eat.shift_to_plant_intercept();
                    intercept.shift_to_plant_intercept();
                    printer::print_doom_calc_setting(
                        doom_row,
                        &garg_rows,
                        explode_to_print,
                        if min_max_garg_x != self.min_max_garg_x {
                            Some(min_max_garg_x)
                        } else {
                            None
                        },
                    );
                    printer::print_eat_and_intercept(&eat, &intercept);
                    ParseResult::Matched
                }
            },
            _ => ParseResult::Unmatched,
        }
    }

    pub fn parse_hit_or_nohit(&self, input: &str) -> ParseResult {
        match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["hit", all_args @ ..] | ["nohit", all_args @ ..] => {
                let (min_max_garg_x, cob_dist) = if self.scene != game::Scene::RE {
                    match all_args {
                        [] => (self.min_max_garg_x, self.scene.cob_dist(None)),
                        [delay_time] => {
                            let Ok(delay_time) = Parser::parse_delay_time(delay_time)
                            else {
                                return ParseResult::Matched;
                            };
                            match game::IceAndCobTimes::of_ice_times_and_cob_time(
                                &self.ice_and_cob_times.ice_times,
                                self.ice_and_cob_times.cob_time + delay_time,
                            ) {
                                Err(err) => {
                                    printer::print_error(err.as_str());
                                    return ParseResult::Matched;
                                }
                                Ok(ice_and_cob_times) => {
                                    match game::min_max_garg_x(&ice_and_cob_times) {
                                        Err(err) => {
                                            printer::print_error(err.as_str());
                                            return ParseResult::Matched;
                                        }
                                        Ok((min_garg_x, max_garg_x)) => {
                                            printer::print_ice_times_and_cob_time(
                                                &ice_and_cob_times,
                                                (min_garg_x, max_garg_x),
                                                true,
                                            );
                                            ((min_garg_x, max_garg_x), self.scene.cob_dist(None))
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            printer::print_too_many_arguments_error();
                            return ParseResult::Matched;
                        }
                    }
                } else {
                    match all_args {
                        [] => {
                            printer::print_error("请提供炮尾所在列");
                            return ParseResult::Matched;
                        }
                        [cob_col] => {
                            let Ok(cob_col) = Parser::parse_cob_col(cob_col)
                            else {
                                return ParseResult::Matched;
                            };
                            (self.min_max_garg_x, self.scene.cob_dist(Some(cob_col)))
                        }
                        [delay_time, cob_col] => {
                            let Ok((delay_time, cob_col)) = combine_results(
                                Parser::parse_delay_time(delay_time),
                                Parser::parse_cob_col(cob_col),
                            ) else {
                                return ParseResult::Matched;
                            };
                            match game::IceAndCobTimes::of_ice_times_and_cob_time(
                                &self.ice_and_cob_times.ice_times,
                                self.ice_and_cob_times.cob_time + delay_time,
                            ) {
                                Err(err) => {
                                    printer::print_error(err.as_str());
                                    return ParseResult::Matched;
                                }
                                Ok(ice_and_cob_times) => {
                                    match game::min_max_garg_x(&ice_and_cob_times) {
                                        Err(err) => {
                                            printer::print_error(err.as_str());
                                            return ParseResult::Matched;
                                        }
                                        Ok((min_garg_x, max_garg_x)) => {
                                            printer::print_ice_times_and_cob_time(
                                                &ice_and_cob_times,
                                                (min_garg_x, max_garg_x),
                                                true,
                                            );
                                            (
                                                (min_garg_x, max_garg_x),
                                                self.scene.cob_dist(Some(cob_col)),
                                            )
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            printer::print_too_many_arguments_error();
                            return ParseResult::Matched;
                        }
                    }
                };
                if input.starts_with("hit") {
                    printer::print_hit_cob_dist(&self.scene, min_max_garg_x.1 as i32, cob_dist)
                } else if input.starts_with("nohit") {
                    printer::print_nohit_cob_dist(&self.scene, min_max_garg_x.0 as i32, cob_dist);
                };
                ParseResult::Matched
            }
            _ => ParseResult::Unmatched,
        }
    }

    pub fn parse_find_max_delay(&self, input: &str) -> ParseResult {
        match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["max", all_args @ ..] => {
                let (cob_list, garg_rows, mut min_max_garg_x, ice_flag) = if self.scene
                    != game::Scene::RE
                {
                    match all_args {
                        [] | [">", ..] => {
                            printer::print_error("请提供炮落点行、炮落点列范围(逗号分隔)");
                            return ParseResult::Matched;
                        }
                        [_] | [_, ">", ..] => {
                            printer::print_error("请提供炮落点列范围(逗号分隔最小、最大值)");
                            return ParseResult::Matched;
                        }
                        [hit_row, min_max_hit_col, ">", garg_pos_args @ ..] => {
                            let Ok((hit_row, (min_hit_col, max_hit_col))) = combine_results(
                                Parser::parse_hit_row(hit_row, &self.scene.all_rows()),
                                Parser::parse_min_max_hit_col(min_max_hit_col),
                            ) else {
                                return ParseResult::Matched;
                            };
                            let Ok((garg_rows, min_max_garg_x, ice_flag)) = Parser::parse_garg_pos(
                                garg_pos_args,
                                &self.scene.garg_rows_for_cob(hit_row),
                            ) else {
                                return ParseResult::Matched;
                            };
                            (
                                ((min_hit_col * 80.).round() as i32
                                    ..=(max_hit_col * 80.).round() as i32)
                                    .map(|v| game::Cob::Ground {
                                        row: hit_row,
                                        col: v as f32 / 80.,
                                    })
                                    .collect::<Vec<game::Cob>>(),
                                garg_rows,
                                min_max_garg_x.unwrap_or(self.min_max_garg_x),
                                ice_flag.unwrap_or(self.ice_and_cob_times.is_iced()),
                            )
                        }
                        _ => {
                            printer::print_bad_format_error();
                            return ParseResult::Matched;
                        }
                    }
                } else {
                    match all_args {
                        [] | [">", ..] => {
                            printer::print_error(
                                "请提供炮落点行、炮落点列范围(逗号分隔)、炮尾所在列",
                            );
                            return ParseResult::Matched;
                        }
                        [_] | [_, ">", ..] => {
                            printer::print_error(
                                "请提供炮落点列范围(逗号分隔最小、最大值)、炮尾所在列",
                            );
                            return ParseResult::Matched;
                        }
                        [_, _] | [_, _, ">", ..] => {
                            printer::print_error("请提供炮尾所在列");
                            return ParseResult::Matched;
                        }
                        [hit_row, min_max_hit_col, cob_col, ">", garg_pos_args @ ..] => {
                            let Ok((hit_row, (min_hit_col, max_hit_col), cob_col)) = combine_results3(
                                Parser::parse_hit_row(hit_row, &self.scene.all_rows()),
                                Parser::parse_min_max_hit_col(min_max_hit_col),
                                Parser::parse_cob_col(cob_col),
                            ) else {
                                return ParseResult::Matched;
                            };
                            let Ok((garg_rows, min_max_garg_x, ice_flag)) = Parser::parse_garg_pos(
                                garg_pos_args,
                                &self.scene.garg_rows_for_cob(hit_row),
                            ) else {
                                return ParseResult::Matched;
                            };
                            (
                                ((min_hit_col * 80.).round() as i32
                                    ..=(max_hit_col * 80.).round() as i32)
                                    .map(|v| game::Cob::Roof {
                                        row: hit_row,
                                        col: v as f32 / 80.,
                                        cob_col,
                                        cob_row: DEFAULT_ROOF_COB_ROW,
                                    })
                                    .collect::<Vec<game::Cob>>(),
                                garg_rows,
                                min_max_garg_x.unwrap_or(self.min_max_garg_x),
                                ice_flag.unwrap_or(self.ice_and_cob_times.is_iced()),
                            )
                        }
                        _ => {
                            printer::print_bad_format_error();
                            return ParseResult::Matched;
                        }
                    }
                };
                if cob_list.is_empty() {
                    return ParseResult::Matched;
                }
                let Ok(garg_x_range) = validate_garg_x_range(&mut min_max_garg_x)
                else {
                    return ParseResult::Matched;
                };
                let mut max_delay: Option<i32> = None;
                let mut cob_cols: Vec<f32> = vec![];
                let mut eat = game::Eat::Empty;
                let mut intercept = game::Intercept::Empty;
                for cob in &cob_list {
                    let (new_eat, new_intercept) = game::judge(
                        &garg_x_range,
                        &vec![(game::Explode::of_cob(cob, &self.scene), &garg_rows)],
                        ice_flag,
                        &self.scene,
                    );
                    match (
                        max_delay,
                        game::safe_intercept_interval(&new_eat, &new_intercept),
                    ) {
                        (_, None) => {}
                        (None, Some((_, new_max))) => {
                            max_delay = Some(new_max);
                            cob_cols = vec![cob.col()];
                            eat = new_eat;
                            intercept = new_intercept;
                        }
                        (Some(prev_max), Some((_, new_max))) => {
                            if new_max > prev_max {
                                max_delay = Some(new_max);
                                cob_cols = vec![cob.col()];
                                eat = new_eat;
                                intercept = new_intercept;
                            } else if new_max == prev_max {
                                cob_cols.push(cob.col());
                            }
                        }
                    };
                }
                printer::print_cob_calc_setting(
                    &vec![(cob_list[0].clone(), garg_rows)],
                    None,
                    if min_max_garg_x != self.min_max_garg_x {
                        Some(min_max_garg_x)
                    } else {
                        None
                    },
                    Some((cob_list[0].col(), cob_list.last().unwrap().col())),
                );
                match cob_cols.as_mut_slice() {
                    [] => {
                        println!("无法无伤拦截.");
                    }
                    [cob_cols @ ..] => {
                        cob_cols.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        println!(
                            "延迟最大的炮落点: {}列",
                            cob_cols
                                .iter()
                                .map(|cob_col| { format!("{}", cob_col) })
                                .collect::<Vec<String>>()
                                .join(", ")
                        );
                        printer::print_eat_and_intercept(&eat, &intercept);
                    }
                };
                ParseResult::Matched
            }
            _ => ParseResult::Unmatched,
        }
    }

    pub fn parse_garg_x_range_of_imp_x(&self, input: &str) -> ParseResult {
        match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["imp", all_args @ ..] => match all_args {
                [] => {
                    printer::print_error("请提供小鬼x坐标（整数）");
                    ParseResult::Matched
                }
                [imp_x] => {
                    let Ok(imp_x) = imp_x.parse::<i32>() else {
                        printer::print_error_with_input("小鬼x坐标应为整数", &imp_x);
                        return ParseResult::Matched;
                    };
                    let Some((min_garg_x, max_garg_x)) = constants::min_max_garg_pos_of_imp_x(imp_x) else {
                        printer::print_error_with_input(format!("应满足{}≤小鬼x坐标≤{}", constants::MIN_IMP_X, constants::MAX_IMP_X).as_str(), imp_x.to_string().as_str());
                        return ParseResult::Matched;
                    };
                    println!("巨人x范围: {:.3}~{:.3}", min_garg_x, max_garg_x);
                    ParseResult::Matched
                }
                _ => {
                    printer::print_too_many_arguments_error();
                    ParseResult::Matched
                }
            },
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

    fn parse_hit_row(hit_row: &&str, valid_hit_rows: &[i32]) -> Result<i32, ()> {
        match hit_row.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input("炮落点行应为整数", hit_row);
                Err(())
            }
            Ok(hit_row) if !(valid_hit_rows.contains(&hit_row)) => {
                printer::print_error_with_input(
                    format!("炮落点行超出范围{:?}", valid_hit_rows).as_str(),
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
            Ok(hit_col) => match game::hit_col_matching_int_pixel(hit_col) {
                None => Ok(hit_col),
                Some(corrected_hit_col) => {
                    printer::print_warning(
                        format!(
                            "当前落点列{}×80不是整数, 改用{}列计算.",
                            hit_col, corrected_hit_col
                        )
                        .as_str(),
                    );
                    Ok(corrected_hit_col)
                }
            },
        }
    }

    fn parse_min_max_hit_col(min_max_hit_col: &&str) -> Result<(f32, f32), ()> {
        match min_max_hit_col
            .replace("，", ",")
            .split(",")
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => {
                printer::print_error("请提供炮落点列最小值、最大值");
                Err(())
            }
            [_] => {
                printer::print_error("请提供炮落点列最大值");
                Err(())
            }
            [min_hit_col, max_hit_col] => {
                match combine_results(
                    Parser::parse_hit_col(min_hit_col),
                    Parser::parse_hit_col(max_hit_col),
                ) {
                    Err(_) => Err(()),
                    Ok((min_hit_col, max_hit_col)) => {
                        let min_hit_pixel = (min_hit_col * 80.).round() as i32;
                        let max_hit_pixel = (max_hit_col * 80.).round() as i32;
                        if min_hit_pixel > max_hit_pixel {
                            printer::print_error_with_input(
                                "应满足炮落点列最小值≤最大值",
                                format!("{}, {}", min_hit_col, max_hit_col).as_str(),
                            );
                            return Err(());
                        }
                        Ok((min_hit_col, max_hit_col))
                    }
                }
            }
            _ => {
                printer::print_error_with_input("参数过多", &min_max_hit_col);
                Err(())
            }
        }
    }

    fn parse_cob_col(cob_col: &&str) -> Result<i32, ()> {
        match cob_col.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input("炮尾所在列应为整数", cob_col);
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

    fn parse_doom_row(doom_row: &&str, valid_doom_rows: &[i32]) -> Result<i32, ()> {
        match doom_row.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input("核所在行应为整数", doom_row);
                Err(())
            }
            Ok(doom_row) if !(valid_doom_rows.contains(&doom_row)) => {
                printer::print_error_with_input(
                    format!("核所在行超出范围{:?}", valid_doom_rows).as_str(),
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

    // 返回值: 巨人所在行, 巨人x范围, 原速/减速
    fn parse_garg_pos(
        garg_pos_args: &[&str],
        valid_garg_rows: &[i32],
    ) -> Result<(Vec<i32>, Option<(f32, f32)>, Option<bool>), ()> {
        match garg_pos_args {
            [] => {
                printer::print_error("请提供巨人所在行、x坐标范围(可选)、速度模式(u/i, 可选)");
                Err(())
            }
            [garg_rows] => {
                Parser::parse_garg_rows(garg_rows, valid_garg_rows).map(|v| (v, None, None))
            }
            [garg_rows, min_max_garg_x] => combine_results3(
                Parser::parse_garg_rows(garg_rows, valid_garg_rows),
                Parser::parse_min_max_garg_x(min_max_garg_x).map(|v| Some(v)),
                Ok(None),
            ),
            [garg_rows, min_max_garg_x, ice_flag] => combine_results3(
                Parser::parse_garg_rows(garg_rows, valid_garg_rows),
                Parser::parse_min_max_garg_x(min_max_garg_x).map(|v| Some(v)),
                Parser::parse_ice_flag(ice_flag).map(|v| Some(v)),
            ),
            _ => {
                printer::print_too_many_arguments_error();
                Err(())
            }
        }
    }

    fn parse_garg_rows(garg_rows: &&str, valid_garg_rows: &[i32]) -> Result<Vec<i32>, ()> {
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
                    .filter(|&v| valid_garg_rows.contains(v))
                    .cloned()
                    .collect::<Vec<i32>>();
                if filtered_garg_rows.is_empty() {
                    printer::print_error_with_input(
                        format!("巨人所在行均超出范围{:?}", valid_garg_rows).as_str(),
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
            [min_garg_x, max_garg_x] => {
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
            _ => {
                printer::print_error_with_input("参数过多", &min_max_garg_x);
                Err(())
            }
        }
    }

    fn parse_ice_flag(ice_flag: &&str) -> Result<bool, ()> {
        if *ice_flag == "u" {
            Ok(false)
        } else if *ice_flag == "i" {
            Ok(true)
        } else {
            printer::print_error_with_input("计算模式应为u/i(原速/减速)", &ice_flag);
            Err(())
        }
    }
}

fn combine_results<T, U, E>(r1: Result<T, E>, r2: Result<U, E>) -> Result<(T, U), E> {
    match (r1, r2) {
        (Err(err), _) | (_, Err(err)) => Err(err),
        (Ok(v1), Ok(v2)) => Ok((v1, v2)),
    }
}

fn combine_results3<T, U, V, E>(
    r1: Result<T, E>,
    r2: Result<U, E>,
    r3: Result<V, E>,
) -> Result<(T, U, V), E> {
    match (r1, r2, r3) {
        (Err(err), _, _) | (_, Err(err), _) | (_, _, Err(err)) => Err(err),
        (Ok(v1), Ok(v2), Ok(v3)) => Ok((v1, v2, v3)),
    }
}

const HELLO_TEXT: &str = r#"本程序源码以MIT许可证发布:
https://github.com/Rottenham/pvz-interception-calculator-rust

欢迎使用拦截计算器v2.0.4.
当前场合: 后院.
输入问号查看帮助; 按↑键显示上次输入的指令.

计算结果默认为【炮激活→炮拦截】的情况.
若为【植物激活→炮拦截】, 需额外-1; 若为【炮激活→植物拦截】, 需额外+1."#;

const ABOUT_TEXT: &str = r#"MIT 许可证

版权 (c) 2023 Crescendo

特此免费授予任何获得本软件副本和相关文档文件（下称“软件”）的人不受限制地处置该软件的权利，包括不受限制地使用、复制、修改、合并、发布、分发、转授许可和/或出售该软件副本，以及再授权被配发了本软件的人如上的权利，惟须遵守条件如下：

上述版权声明和本许可声明应包含在该软件的所有副本或实质成分中。

本软件是“如此”提供的，没有任何形式的明示或暗示的保证，包括但不限于对适销性、特定用途的适用性和不侵权的保证。在任何情况下，作者或版权持有人都不对任何索赔、损害或其他责任负责，无论这些追责来自合同、侵权或其它行为中，还是产生于、源于或有关于本软件以及本软件的使用或其它处置。


请注意，拦截计算器_无法确保_100%的计算精度，其原因包括：
1. 所用的巨人位移数据并非100%精确；
2. 所用的仅取坐标极值的拦截区间计算方式并非100%精确。

在极端情况下，计算结果与实际情况可能存在1~2cs左右的偏差，敬请谅解。

除上述在技术上难以解决的问题外，拦截计算器约定在能力所及的范围内尽可能接近游戏情况。"#;

const HELP_TEXT: &str = r#"
de/pe/re                             设置场合

wave                                 查看当前用冰、激活时机
wave 冰时机.. 激活时机               设置用冰、激活时机（用冰时机可为0个或多个）
                                     例：$ wave 1 400 800 -> 1、400用冰，800激活

delay 炮列数 (炮尾列)                计算可拦区间、最早啃食、最早可冰（屋顶场合需指定炮尾所在列）
                                     例：$ delay 8.8 -> 非屋顶场合计算落8.8列的拦截炮
                                         $ delay 3.5 4 -> 屋顶场合计算落3.5列的45列炮
delay 炮行数 炮列数 (炮尾列)
  > 巨人所在行 (巨人x范围) (u/i)     计算炮拦截特定巨人（可指定按原速/减速计算）
                                     例：$ delay 1 8.8 > 2 -> 计算(1,8.8)的炮拦截2路巨人
                                         $ delay 1 8.8 > 1,2 700,800 -> 计算(1,8.8)的炮拦截1、2路x为700~800的巨人
                                         $ delay 1 8.8 > 1,2 700,800 u -> 同上，但指定按原速计算

doom 核行数 核列数
  (> 巨人所在行 (巨人x范围) (u/i))   计算核武拦截特定巨人（">"及之后部分可选, 可指定按原速/减速计算）
                                     例：$ doom 3 8 -> 计算3-8核武
                                         $ doom 3 8 > 2,5 700,800 -> 计算3-8核武拦截2、5路x为700~800的巨人

hit (炮尾列) (延迟)                  计算刚好全伤巨人的炮落点（可指定炮延时生效时机）
                                     例：$ hit -> 计算全伤巨人的炮落点
                                         $ wave 300 $ hit 50 -> 计算350cs时全伤巨人的炮落点
                                         $ wave 300 $ hit -50 -> 计算250cs时全伤巨人的炮落点

nohit (炮尾列) (延迟)                计算刚好不伤巨人的炮落点（可指定炮延时生效时机）

max 炮行数 炮列数范围
  > 巨人所在行 (巨人x范围) (u/i)     寻找无伤拦截可延迟最多的炮落点列（可指定按原速/减速计算）
                                     例：$ max 1 7,7.5 > 1,2 -> 寻找1路7~7.5列炮拦截1、2路巨人可延迟最多的落点

imp 小鬼x坐标                        计算投掷该坐标小鬼的巨人x范围

?/help                               显示此帮助
about                                关于拦截计算器"#;
