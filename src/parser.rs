use crate::constants;
use crate::game;
use crate::printer;
use dyn_fmt::AsStrFormatExt;

#[cfg(feature = "en")]
use crate::lang::en::*;

#[cfg(feature = "zh")]
use crate::lang::zh::*;

const DEFAULT_SCENE: game::Scene = game::Scene::PE;
const DEFAULT_COB_TIME: i32 = 318;
const DEFAULT_ROOF_COB_ROW: i32 = 3;

fn validate_garg_x_range(min_max_garg_x: &mut (f32, f32)) -> Result<game::GargXRange, ()> {
    match game::GargXRange::of_min_max_garg_pos(*min_max_garg_x) {
        game::GargXRange::Cancelled => {
            printer::print_warning(GARG_X_RANGE_CANCELLED);
            Err(())
        }
        game::GargXRange::Modified { min, max } => {
            printer::print_warning(GARG_X_RANGE_MODIFIED.format(&[min, max]).as_str());
            *min_max_garg_x = (min, max);
            Ok(game::GargXRange::Modified { min, max })
        }
        game::GargXRange::Ok { min, max } => Ok(game::GargXRange::Modified { min, max }),
    }
}

struct ParsedGargPos {
    garg_rows: Vec<i32>,
    min_max_garg_x: Option<(f32, f32)>,
    ice_flag: Option<bool>,
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

impl Default for Parser {
    fn default() -> Self {
        println!("{}", HELLO);
        let scene = DEFAULT_SCENE;
        let ice_and_cob_times =
            game::IceAndCobTimes::of_ice_times_and_cob_time(&[], DEFAULT_COB_TIME).unwrap();
        let min_max_garg_x = game::min_max_garg_x(&ice_and_cob_times).unwrap();
        Parser {
            scene,
            ice_and_cob_times,
            min_max_garg_x,
        }
    }
}

impl Parser {
    pub fn parse_help(&self, input: &str) -> ParseResult {
        if input == "help" || input == "?" || input == "？" {
            println!("{}", HELP);
            ParseResult::Matched
        } else {
            ParseResult::Unmatched
        }
    }

    pub fn parse_about(&self, input: &str) -> ParseResult {
        if input == "about" {
            println!("{}", ABOUT);
            ParseResult::Matched
        } else {
            ParseResult::Unmatched
        }
    }

    pub fn parse_scene(&mut self, input: &str) -> ParseResult {
        match input {
            "de" | "ne" => {
                self.scene = game::Scene::DE;
                println!("{SET_FRONTYARD}");
                ParseResult::Matched
            }
            "pe" | "fe" => {
                self.scene = game::Scene::PE;
                println!("{SET_BACKYARD}");
                ParseResult::Matched
            }
            "re" | "me" => {
                self.scene = game::Scene::RE;
                println!("{SET_ROOF}");
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
                            &self.ice_and_cob_times,
                            self.min_max_garg_x,
                            false,
                        );
                    }
                    [ice_times @ .., cob_time] => {
                        let (Ok(ice_times), Ok(cob_time)) = (
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
                let (cob_and_garg_rows, mut min_max_garg_x, ice_flag, explode_to_print): (
                    Vec<(game::Cob, Vec<i32>)>,
                    _,
                    _,
                    _,
                ) =
                    if !self.scene.is_roof() {
                        match extra_args {
                            [">", ..] if *command == "delay" => {
                                printer::print_error(NEED_HIT_ROW_HIT_COL);
                                return ParseResult::Matched;
                            }
                            [_, ">", ..] if *command == "delay" => {
                                printer::print_error(NEED_HIT_COL);
                                return ParseResult::Matched;
                            }
                            [hit_row, hit_col, ">", garg_pos_args @ ..] if *command == "delay" => {
                                let (Ok(hit_row), Ok(hit_col)) = (
                                    Parser::parse_hit_row(hit_row, &self.scene.all_rows()),
                                    Parser::parse_hit_col(hit_col),
                                ) else {
                                    return ParseResult::Matched;
                                };
                                let Ok(ParsedGargPos {
                                    garg_rows,
                                    min_max_garg_x,
                                    ice_flag,
                                }) = Parser::parse_garg_pos(
                                    garg_pos_args,
                                    &self.scene.garg_rows_for_cob(hit_row),
                                )
                                else {
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
                                printer::print_error(NEED_HIT_COL);
                                return ParseResult::Matched;
                            }
                            [hit_col] => {
                                let Ok(hit_col) = Parser::parse_hit_col(hit_col) else {
                                    return ParseResult::Matched;
                                };
                                (
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
                        match extra_args {
                            [">", ..] if *command == "delay" => {
                                printer::print_error(NEED_HIT_ROW_HIT_COL_COB_COL);
                                return ParseResult::Matched;
                            }
                            [_, ">", ..] if *command == "delay" => {
                                printer::print_error(NEED_HIT_COL_COB_COL);
                                return ParseResult::Matched;
                            }
                            [_, _, ">", ..] if *command == "delay" => {
                                printer::print_error(NEED_COB_COL);
                                return ParseResult::Matched;
                            }
                            [hit_row, hit_col, cob_col, ">", garg_pos_args @ ..]
                                if *command == "delay" =>
                            {
                                let (Ok(hit_row), Ok(hit_col), Ok(cob_col)) = (
                                    Parser::parse_hit_row(hit_row, &self.scene.all_rows()),
                                    Parser::parse_hit_col(hit_col),
                                    Parser::parse_cob_col(cob_col),
                                ) else {
                                    return ParseResult::Matched;
                                };
                                let Ok(ParsedGargPos {
                                    garg_rows,
                                    min_max_garg_x,
                                    ice_flag,
                                }) = Parser::parse_garg_pos(
                                    garg_pos_args,
                                    &self.scene.garg_rows_for_cob(hit_row),
                                )
                                else {
                                    return ParseResult::Matched;
                                };
                                let cob = game::Cob::Roof {
                                    row: hit_row,
                                    col: hit_col,
                                    cob_col,
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
                                printer::print_error(NEED_HIT_COL_COB_COL);
                                return ParseResult::Matched;
                            }
                            [_] => {
                                printer::print_error(NEED_COB_COL);
                                return ParseResult::Matched;
                            }
                            [hit_col, cob_col] => {
                                let (Ok(hit_col), Ok(cob_col)) = (
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
                let Ok(garg_x_range) = validate_garg_x_range(&mut min_max_garg_x) else {
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
            ["doom", extra_args @ ..] => match extra_args {
                [] => {
                    printer::print_error(NEED_DOOM_ROW_DOOM_COL);
                    ParseResult::Matched
                }
                [_] => {
                    printer::print_error(NEED_DOOM_ROW);
                    ParseResult::Matched
                }
                [doom_row, doom_col, garg_pos_args @ ..] => {
                    let (Ok(doom_row), Ok(doom_col)) = (
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
                        match garg_pos_args {
                            [] => (
                                self.scene.garg_rows_for_doom(doom_row),
                                self.min_max_garg_x,
                                self.ice_and_cob_times.is_iced(),
                                None,
                            ),
                            [">", garg_pos_args @ ..] => {
                                let Ok(ParsedGargPos {
                                    garg_rows,
                                    min_max_garg_x,
                                    ice_flag,
                                }) = Parser::parse_garg_pos(
                                    garg_pos_args,
                                    &self.scene.garg_rows_for_doom(doom_row),
                                )
                                else {
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
                    let Ok(garg_x_range) = validate_garg_x_range(&mut min_max_garg_x) else {
                        return ParseResult::Matched;
                    };
                    let (mut eat, mut intercept) = game::judge(
                        &garg_x_range,
                        &[(explode.clone(), &garg_rows)],
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
            ["hit", extra_args @ ..] | ["nohit", extra_args @ ..] => {
                let (min_max_garg_x, cob_dist) = if !self.scene.is_roof() {
                    match extra_args {
                        [] => (self.min_max_garg_x, self.scene.cob_dist(None)),
                        [delay_time] => {
                            let Ok(delay_time) = Parser::parse_delay_time(delay_time) else {
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
                    match extra_args {
                        [] => {
                            printer::print_error(NEED_COB_COL);
                            return ParseResult::Matched;
                        }
                        [cob_col] => {
                            let Ok(cob_col) = Parser::parse_cob_col(cob_col) else {
                                return ParseResult::Matched;
                            };
                            (self.min_max_garg_x, self.scene.cob_dist(Some(cob_col)))
                        }
                        [cob_col, delay_time] => {
                            let (Ok(delay_time), Ok(cob_col)) = (
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
            ["max", extra_args @ ..] => {
                let (cob_list, garg_rows, mut min_max_garg_x, ice_flag) = if !self.scene.is_roof() {
                    match extra_args {
                        [] | [">", ..] => {
                            printer::print_error(NEED_HIT_ROW_HIT_COL_RANGE);
                            return ParseResult::Matched;
                        }
                        [_] | [_, ">", ..] => {
                            printer::print_error(NEED_HIT_COL_RANGE);
                            return ParseResult::Matched;
                        }
                        [hit_row, min_max_hit_col, ">", garg_pos_args @ ..] => {
                            let (Ok(hit_row), Ok((min_hit_col, max_hit_col))) = (
                                Parser::parse_hit_row(hit_row, &self.scene.all_rows()),
                                Parser::parse_min_max_hit_col(min_max_hit_col),
                            ) else {
                                return ParseResult::Matched;
                            };
                            let Ok(ParsedGargPos {
                                garg_rows,
                                min_max_garg_x,
                                ice_flag,
                            }) = Parser::parse_garg_pos(
                                garg_pos_args,
                                &self.scene.garg_rows_for_cob(hit_row),
                            )
                            else {
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
                    match extra_args {
                        [] | [">", ..] => {
                            printer::print_error(NEED_HIT_ROW_HIT_COL_RANGE_COB_COL);
                            return ParseResult::Matched;
                        }
                        [_] | [_, ">", ..] => {
                            printer::print_error(NEED_HIT_COL_RANGE_COB_COL);
                            return ParseResult::Matched;
                        }
                        [_, _] | [_, _, ">", ..] => {
                            printer::print_error(NEED_COB_COL);
                            return ParseResult::Matched;
                        }
                        [hit_row, min_max_hit_col, cob_col, ">", garg_pos_args @ ..] => {
                            let (Ok(hit_row), Ok((min_hit_col, max_hit_col)), Ok(cob_col)) = (
                                Parser::parse_hit_row(hit_row, &self.scene.all_rows()),
                                Parser::parse_min_max_hit_col(min_max_hit_col),
                                Parser::parse_cob_col(cob_col),
                            ) else {
                                return ParseResult::Matched;
                            };
                            let Ok(ParsedGargPos {
                                garg_rows,
                                min_max_garg_x,
                                ice_flag,
                            }) = Parser::parse_garg_pos(
                                garg_pos_args,
                                &self.scene.garg_rows_for_cob(hit_row),
                            )
                            else {
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
                let Ok(garg_x_range) = validate_garg_x_range(&mut min_max_garg_x) else {
                    return ParseResult::Matched;
                };
                let mut max_delay: Option<i32> = None;
                let mut cob_cols: Vec<f32> = vec![];
                let mut eat = game::Eat::Empty;
                let mut intercept = game::Intercept::Empty;
                for cob in &cob_list {
                    let (new_eat, new_intercept) = game::judge(
                        &garg_x_range,
                        &[(game::Explode::of_cob(cob, &self.scene), &garg_rows)],
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
                        (Some(prev_max), Some((_, new_max))) => match new_max.cmp(&prev_max) {
                            std::cmp::Ordering::Greater => {
                                max_delay = Some(new_max);
                                cob_cols = vec![cob.col()];
                                eat = new_eat;
                                intercept = new_intercept;
                            }
                            _ => {
                                cob_cols.push(cob.col());
                            }
                        },
                    };
                }
                printer::print_cob_calc_setting(
                    &[(cob_list[0].clone(), garg_rows)],
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
                        println!("{CANNOT_INTERCEPT_WITHOUT_HARM}");
                    }
                    cob_cols => {
                        cob_cols.sort_by(|a, b| a.partial_cmp(b).unwrap());
                        println!(
                            "{HIT_COL_WITH_MAX_DELAY}: {}",
                            COL.format(&[format!("{:?}", cob_cols)]),
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
            ["imp", extra_args @ ..] => match extra_args {
                [] => {
                    printer::print_error(NEED_IMP_X_RANGE);
                    ParseResult::Matched
                }
                [imp_x] => {
                    let Ok(imp_x) = imp_x.parse::<i32>() else {
                        printer::print_error_with_input(IMP_X_SHOULD_BE_INTEGER, imp_x);
                        return ParseResult::Matched;
                    };
                    let Some((min_garg_x, max_garg_x)) =
                        constants::min_max_garg_pos_of_imp_x(imp_x)
                    else {
                        printer::print_error_with_input(
                            &IMP_X_SHOULD_BE_IN_RANGE
                                .format(&[constants::MIN_IMP_X, constants::MAX_IMP_X]),
                            imp_x.to_string().as_str(),
                        );
                        return ParseResult::Matched;
                    };
                    println!("{GARG_X_RANGE}: {:.3}~{:.3}", min_garg_x, max_garg_x);
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
                printer::print_error_with_input(
                    ICE_TIMES_SHOULD_BE_INTEGER,
                    format!("{:?}", ice_times).as_str(),
                );
                Err(())
            }
            Ok(ice_times) => Ok(ice_times),
        }
    }

    fn parse_cob_time(cob_time: &&str) -> Result<i32, ()> {
        match cob_time.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input(COB_TIME_SHOULD_BE_INTEGER, cob_time);
                Err(())
            }
            Ok(cob_time) if cob_time < 0 => {
                printer::print_error_with_input(
                    COB_TIME_SHOULD_BE_NON_NEGATIVE,
                    cob_time.to_string().as_str(),
                );
                Err(())
            }
            Ok(cob_time) => Ok(cob_time),
        }
    }

    fn parse_delay_time(delay_time: &&str) -> Result<i32, ()> {
        match delay_time.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input(DELAY_TIME_SHOULD_BE_INTEGER, delay_time);
                Err(())
            }
            Ok(delay) => Ok(delay),
        }
    }

    fn parse_hit_row(hit_row: &&str, valid_hit_rows: &[i32]) -> Result<i32, ()> {
        match hit_row.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input(HIT_ROW_SHOULD_BE_INTEGER, hit_row);
                Err(())
            }
            Ok(hit_row) if !(valid_hit_rows.contains(&hit_row)) => {
                printer::print_error_with_input(
                    &HIT_ROW_OUT_OF_RANGE.format(&[format!("{:?}", valid_hit_rows)]),
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
                printer::print_error_with_input(HIT_COL_SHOULD_BE_NUMBER, hit_col);
                Err(())
            }
            Ok(hit_col) if !((0. ..10.).contains(&hit_col)) => {
                printer::print_error_with_input(
                    HIT_COL_SHOULD_BE_IN_RANGE,
                    hit_col.to_string().as_str(),
                );
                Err(())
            }
            Ok(hit_col) => match game::hit_col_matching_int_pixel(hit_col) {
                None => Ok(hit_col),
                Some(corrected_hit_col) => {
                    printer::print_warning(
                        &HIT_COL_TIMES_EIGHTY_NOT_INTEGER.format(&[hit_col, corrected_hit_col]),
                    );
                    Ok(corrected_hit_col)
                }
            },
        }
    }

    fn parse_min_max_hit_col(min_max_hit_col: &&str) -> Result<(f32, f32), ()> {
        match min_max_hit_col
            .replace('，', ",")
            .split(',')
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => {
                printer::print_error(NEED_MIN_MAX_HIT_COL);
                Err(())
            }
            [_] => {
                printer::print_error(NEED_MAX_HIT_COL);
                Err(())
            }
            [min_hit_col, max_hit_col] => {
                match (
                    Parser::parse_hit_col(min_hit_col),
                    Parser::parse_hit_col(max_hit_col),
                ) {
                    (Err(_), _) | (_, Err(_)) => Err(()),
                    (Ok(min_hit_col), Ok(max_hit_col)) => {
                        let min_hit_pixel = (min_hit_col * 80.).round() as i32;
                        let max_hit_pixel = (max_hit_col * 80.).round() as i32;
                        if min_hit_pixel > max_hit_pixel {
                            printer::print_error_with_input(
                                MIN_COL_SHOULD_BE_SMALLER_THAN_MAX_COL,
                                format!("{}, {}", min_hit_col, max_hit_col).as_str(),
                            );
                            return Err(());
                        }
                        Ok((min_hit_col, max_hit_col))
                    }
                }
            }
            _ => {
                printer::print_too_many_arguments_error();
                Err(())
            }
        }
    }

    fn parse_cob_col(cob_col: &&str) -> Result<i32, ()> {
        match cob_col.parse::<i32>() {
            Err(_) => {
                printer::print_error_with_input(COB_COL_SHOULD_BE_INTEGER, cob_col);
                Err(())
            }
            Ok(cob_col) if !((1..=8).contains(&cob_col)) => {
                printer::print_error_with_input(
                    COB_COL_SHOULD_BE_IN_RANGE,
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
                printer::print_error_with_input(DOOM_ROW_SHOULD_BE_INTEGER, doom_row);
                Err(())
            }
            Ok(doom_row) if !(valid_doom_rows.contains(&doom_row)) => {
                printer::print_error_with_input(
                    &DOOM_ROW_OUT_OF_RANGE.format(&[format!("{:?}", valid_doom_rows)]),
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
                printer::print_error_with_input(DOOM_COL_SHOULD_BE_INTEGER, doom_col);
                Err(())
            }
            Ok(doom_col) if !((1..=9).contains(&doom_col)) => {
                printer::print_error_with_input(
                    DOOM_COL_SHOULD_BE_IN_RANGE,
                    doom_col.to_string().as_str(),
                );
                Err(())
            }
            Ok(doom_col) => Ok(doom_col),
        }
    }

    fn parse_garg_pos(
        garg_pos_args: &[&str],
        valid_garg_rows: &[i32],
    ) -> Result<ParsedGargPos, ()> {
        let (garg_rows, min_max_garg_x, ice_flag) = match garg_pos_args {
            [] => {
                printer::print_error(NEED_GARG_ROWS_X_RANGE_ICE_FLAG);
                (Err(()), Err(()), Err(()))
            }
            [garg_rows] => (
                Parser::parse_garg_rows(garg_rows, valid_garg_rows),
                Ok(None),
                Ok(None),
            ),
            [garg_rows, min_max_garg_x] => (
                Parser::parse_garg_rows(garg_rows, valid_garg_rows),
                Parser::parse_min_max_garg_x(min_max_garg_x).map(Some),
                Ok(None),
            ),
            [garg_rows, min_max_garg_x, ice_flag] => (
                Parser::parse_garg_rows(garg_rows, valid_garg_rows),
                Parser::parse_min_max_garg_x(min_max_garg_x).map(Some),
                Parser::parse_ice_flag(ice_flag).map(Some),
            ),
            _ => {
                printer::print_too_many_arguments_error();
                (Err(()), Err(()), Err(()))
            }
        };
        match (garg_rows, min_max_garg_x, ice_flag) {
            (Err(()), _, _) | (_, Err(()), _) | (_, _, Err(())) => Err(()),
            (Ok(garg_rows), Ok(min_max_garg_x), Ok(ice_flag)) => Ok(ParsedGargPos {
                garg_rows,
                min_max_garg_x,
                ice_flag,
            }),
        }
    }

    fn parse_garg_rows(garg_rows: &&str, valid_garg_rows: &[i32]) -> Result<Vec<i32>, ()> {
        match garg_rows
            .replace('，', ",")
            .split(',')
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()
        {
            Err(_) => {
                printer::print_error_with_input(GARG_ROWS_SHOULD_BE_INTEGER, garg_rows);
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
                        &GARG_ROWS_ALL_OUT_OF_RANGE.format(&[format!("{:?}", valid_garg_rows)]),
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
            .replace('，', ",")
            .split(',')
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => {
                printer::print_error(NEED_MIN_MAX_GARG_X);
                Err(())
            }
            [_] => {
                printer::print_error(NEED_MAX_GARG_X);
                Err(())
            }
            [min_garg_x, max_garg_x] => {
                match (min_garg_x.parse::<f32>(), max_garg_x.parse::<f32>()) {
                    (Err(_), _) => {
                        printer::print_error_with_input(MIN_GARG_X_SHOULD_BE_NUMBER, min_garg_x);
                        Err(())
                    }
                    (_, Err(_)) => {
                        printer::print_error_with_input(MAX_GARG_X_SHOULD_BE_NUMBER, max_garg_x);
                        Err(())
                    }
                    (Ok(min_garg_x), Ok(max_garg_x)) if min_garg_x > max_garg_x => {
                        printer::print_error_with_input(
                            MIN_GARG_X_SHOULD_BE_SMALLER_THAN_MAX_GARG_X,
                            format!("{}, {}", min_garg_x, max_garg_x).as_str(),
                        );
                        Err(())
                    }
                    (Ok(min_garg_x), _) if min_garg_x <= game::MIN_GARG_X => {
                        printer::print_error_with_input(
                            &MIN_GARG_X_SHOULD_BE_LARGER_THAN_LOWER_BOUND
                                .format(&[game::MIN_GARG_X]),
                            format!("{}", min_garg_x).as_str(),
                        );
                        Err(())
                    }
                    (_, Ok(max_garg_x)) if max_garg_x > game::MAX_GARG_X => {
                        printer::print_error_with_input(
                            &MAX_GARG_X_SHOULD_BE_SMALLER_THAN_UPPER_BOUND
                                .format(&[game::MAX_GARG_X]),
                            format!("{}", max_garg_x).as_str(),
                        );
                        Err(())
                    }
                    (Ok(min_garg_x), Ok(max_garg_x)) => Ok((min_garg_x, max_garg_x)),
                }
            }
            _ => {
                printer::print_too_many_arguments_error();
                Err(())
            }
        }
    }

    fn parse_ice_flag(ice_mode: &&str) -> Result<bool, ()> {
        if *ice_mode == "u" {
            Ok(false)
        } else if *ice_mode == "i" {
            Ok(true)
        } else {
            printer::print_error_with_input(ICE_FLAG_SHOULD_BE_U_OR_I, ice_mode);
            Err(())
        }
    }
}
