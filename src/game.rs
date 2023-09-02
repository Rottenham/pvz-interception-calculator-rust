use crate::constants;
use std::{cmp, ops::Add};

const GRAVITY: Vec2 = Vec2 { x: 0., y: -0.05 };
const COL_WIDTH: i32 = 80;
const COB_RADIUS: i32 = 115;
const COB_HIT_RANGE: i32 = 1;
const DOOM_RADIUS: i32 = 250;
const DOOM_HIT_RANGE: i32 = 3;
const GARG_THROW_IMP_THRES: f32 = 401.;
const IMP_DEFENSE_SHIFT: IntVec2 = IntVec2 { x: 36, y: 0 };
const IMP_DEFENSE_WIDTH: i32 = 42;
const IMP_DEFENSE_HEIGHT: i32 = 115;
const MIN_GARG_START_POS: f32 = 845.;
const MAX_GARG_START_POS: f32 = 854.;
pub const MIN_GARG_X: f32 = -152.; // 如果 x <= -152., 巨人将进家
pub const MAX_GARG_X: f32 = MAX_GARG_START_POS;
const MIN_ICE_TIME_FOR_UNICED: i32 = 400;
const MAX_ICE_TIME_FOR_UNICED: i32 = 600;
const MIN_ICE_TIME_FOR_ICED: i32 = 300;
const MAX_ICE_TIME_FOR_ICED: i32 = 400;
pub const ICE_SLOW_TOTAL_TIME: i32 = 2000;
const DE_COB_DIST: CobDist = CobDist {
    hit_above: 111,
    hit_same: 125,
    hit_below: 125,
};
const PE_COB_DIST: CobDist = CobDist {
    hit_above: 118,
    hit_same: 125,
    hit_below: 125,
};
const RE_COB_DIST: [CobDist; 8] = [
    CobDist {
        hit_above: 125,
        hit_same: 124,
        hit_below: 84,
    },
    CobDist {
        hit_above: 125,
        hit_same: 125,
        hit_below: 102,
    },
    CobDist {
        hit_above: 125,
        hit_same: 125,
        hit_below: 114,
    },
    CobDist {
        hit_above: 125,
        hit_same: 125,
        hit_below: 121,
    },
    CobDist {
        hit_above: 124,
        hit_same: 125,
        hit_below: 124,
    },
    CobDist {
        hit_above: 121,
        hit_same: 125,
        hit_below: 125,
    },
    CobDist {
        hit_above: 118,
        hit_same: 125,
        hit_below: 125,
    },
    CobDist {
        hit_above: 118,
        hit_same: 125,
        hit_below: 125,
    },
];
const FLOAT_INT_DIFF_TOLERANCE: f32 = 0.01;

#[derive(Debug)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntVec2 {
    pub x: i32,
    pub y: i32,
}

impl Add for IntVec2 {
    type Output = IntVec2;

    fn add(self, other: IntVec2) -> IntVec2 {
        IntVec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Rectangle {
    upper_left: IntVec2,
    width: i32,
    height: i32,
}

#[derive(Debug, Clone)]
pub struct Circle {
    pub center: IntVec2,
    radius: i32,
}

fn circle_rectangle_intersect(
    Circle {
        center: IntVec2 { x: cir_x, y: cir_y },
        radius,
    }: &Circle,
    Rectangle {
        upper_left: IntVec2 {
            x: rect_x,
            y: rect_y,
        },
        width: rect_w,
        height: rect_h,
    }: &Rectangle,
) -> bool {
    let x = if *cir_x < *rect_x {
        *rect_x
    } else if *cir_x > *rect_x + *rect_w {
        *rect_x + *rect_w
    } else {
        *cir_x
    };
    let y = if *cir_y < *rect_y {
        *rect_y
    } else if *cir_y > *rect_y + *rect_h {
        *rect_y + *rect_h
    } else {
        *cir_y
    };
    ((*cir_x - x).pow(2) + (*cir_y - y).pow(2)) <= radius.pow(2)
}

#[derive(Clone)]
pub enum DelayMode {
    Delay1, // 拦一行（下）
    Delay2, // 拦两行（本、下）
    Delay3, // 拦三行（上、本、下）
}

pub struct CobDist {
    pub hit_above: i32, // 炸上行巨人炮距
    pub hit_same: i32,  // 炸本行巨人炮距
    pub hit_below: i32, // 炸下行巨人炮距
}

#[derive(PartialEq, Eq)]
pub enum Scene {
    DE,
    PE,
    RE,
}

impl Scene {
    pub fn all_rows(&self) -> Vec<i32> {
        match self {
            Scene::DE | Scene::RE => vec![1, 2, 3, 4, 5],
            Scene::PE => vec![1, 2, 3, 4, 5, 6],
        }
    }

    pub fn garg_rows_for_cob(&self, hit_row: i32) -> Vec<i32> {
        let mut garg_rows = self
            .hittable_rows(hit_row, COB_HIT_RANGE)
            .into_iter()
            .collect::<Vec<i32>>();
        garg_rows.sort();
        garg_rows
    }

    pub fn garg_rows_for_doom(&self, doom_row: i32) -> Vec<i32> {
        let mut garg_rows = self
            .hittable_rows(doom_row, DOOM_HIT_RANGE)
            .into_iter()
            .collect::<Vec<i32>>();
        garg_rows.sort();
        garg_rows
    }

    fn hittable_rows(&self, hit_row: i32, hit_range: i32) -> Vec<i32> {
        let garg_rows = match self {
            Scene::DE | Scene::RE => vec![1, 2, 3, 4, 5],
            Scene::PE => vec![1, 2, 5, 6],
        };
        garg_rows
            .into_iter()
            .filter(|&v| (v - hit_row).abs() <= hit_range)
            .collect()
    }

    pub fn default_delay_mode(&self, hit_col: f32, cob_col: Option<i32>) -> DelayMode {
        match self {
            Scene::DE => {
                if hit_col <= 5. {
                    DelayMode::Delay3
                } else {
                    DelayMode::Delay2
                }
            }
            Scene::PE => DelayMode::Delay2,
            Scene::RE => {
                if hit_col <= 5. {
                    DelayMode::Delay3
                } else {
                    if cob_col.expect("需指定炮尾所在列.") <= 4 {
                        DelayMode::Delay3
                    } else {
                        DelayMode::Delay2
                    }
                }
            }
        }
    }

    pub fn hit_row_and_garg_rows_of_delay_mode(
        &self,
        delay_mode: &DelayMode,
    ) -> Vec<(i32, Vec<i32>)> {
        match delay_mode {
            DelayMode::Delay1 => match self {
                Scene::DE => vec![(1, vec![2]), (4, vec![5])],
                Scene::PE => vec![(1, vec![2]), (5, vec![6])],
                Scene::RE => vec![(2, vec![3]), (4, vec![5])],
            },
            DelayMode::Delay2 => match self {
                Scene::DE => vec![(1, vec![1, 2]), (4, vec![4, 5])],
                Scene::PE => vec![(1, vec![1, 2]), (5, vec![5, 6])],
                Scene::RE => vec![(2, vec![2, 3]), (4, vec![4, 5])],
            },
            DelayMode::Delay3 => match self {
                Scene::DE => vec![(2, vec![1, 2, 3]), (4, vec![3, 4, 5])],
                Scene::PE => vec![(2, vec![1, 2]), (6, vec![5, 6])],
                Scene::RE => vec![(2, vec![1, 2, 3]), (4, vec![3, 4, 5])],
            },
        }
    }

    pub fn cob_dist(&self, cob_col: Option<i32>) -> &CobDist {
        match self {
            Scene::DE => &DE_COB_DIST,
            Scene::PE => &PE_COB_DIST,
            Scene::RE => RE_COB_DIST
                .get((cob_col.expect("需指定炮尾所在列.") - 1) as usize)
                .expect("没有对应该炮尾所在列的炮距数据."),
        }
    }

    fn row_height(&self) -> i32 {
        match self {
            Scene::DE => 100,
            Scene::PE | Scene::RE => 85,
        }
    }

    fn zombie_base_y(&self) -> i32 {
        match self {
            Scene::DE | Scene::PE => 50,
            Scene::RE => 40,
        }
    }

    pub fn is_roof(&self) -> bool {
        *self == Scene::RE
    }
}

#[derive(Clone)]
pub enum Cob {
    Ground {
        row: i32,
        col: f32,
    },
    Roof {
        row: i32,
        col: f32,
        cob_col: i32,
        cob_row: i32,
    },
}

impl Cob {
    pub fn row(&self) -> i32 {
        match self {
            Cob::Ground { row, col: _ } => *row,
            Cob::Roof {
                row,
                col: _,
                cob_col: _,
                cob_row: _,
            } => *row,
        }
    }
    pub fn col(&self) -> f32 {
        match self {
            Cob::Ground { row: _, col } => *col,
            Cob::Roof {
                row: _,
                col,
                cob_col: _,
                cob_row: _,
            } => *col,
        }
    }
}

pub struct Doom {
    pub row: i32,
    pub col: i32,
}

#[derive(Debug, Clone)]
pub struct Explode {
    pub range: Circle,
    hittable_rows: Vec<i32>,
}

impl Explode {
    pub fn of_cob(cob: &Cob, scene: &Scene) -> Explode {
        let row_height = scene.row_height();
        match cob {
            Cob::Ground { row, col } => {
                let mut x = (col * COL_WIDTH as f32).round() as i32;
                x = if x >= 7 { x - 7 } else { x - 6 };
                let y = 120 + ((row - 1) * row_height);
                Explode {
                    range: Circle {
                        center: IntVec2 { x, y },
                        radius: COB_RADIUS,
                    },
                    hittable_rows: scene.hittable_rows(*row, COB_HIT_RANGE),
                }
            }
            Cob::Roof {
                row,
                col,
                cob_col,
                cob_row,
            } => {
                let mut x = (col * COL_WIDTH as f32).round() as i32;
                let mut y = 209 + (row - 1) * row_height;

                let step1: i32;
                if x <= 206 {
                    step1 = 0;
                } else if x >= 527 {
                    step1 = 5;
                } else {
                    step1 = (x - 127) / 80;
                }
                y -= step1 * 20;

                let (left_edge, right_edge, step2_shift): (i32, i32, i32);
                if *cob_col == 1 {
                    left_edge = 87;
                    right_edge = 524;
                    step2_shift = 0;
                } else if *cob_col >= 7 {
                    left_edge = 510;
                    right_edge = 523;
                    step2_shift = 5;
                } else {
                    left_edge = 80 * *cob_col - 13;
                    right_edge = 524;
                    step2_shift = 5;
                }

                let step2: i32;
                if x <= left_edge {
                    step2 = 0;
                } else if x >= right_edge {
                    step2 = (right_edge - left_edge + 3) / 4 - step2_shift;
                } else {
                    step2 = (x - left_edge + 3) / 4 - step2_shift;
                }
                y -= step2;

                if x == left_edge && *cob_col >= 2 && *cob_col <= 6 {
                    if *cob_row >= 3 && *cob_row <= 5 {
                        y += 5;
                    }
                    if *cob_row == 3 && *cob_col == 6 {
                        y -= 5;
                    }
                }

                y = cmp::max(y, 0);
                x = if x >= 7 { x - 7 } else { x - 6 };
                Explode {
                    range: Circle {
                        center: IntVec2 { x, y },
                        radius: COB_RADIUS,
                    },
                    hittable_rows: scene.hittable_rows(*row, COB_HIT_RANGE),
                }
            }
        }
    }

    pub fn of_doom(Doom { row, col }: &Doom, scene: &Scene) -> Explode {
        Explode {
            range: Circle {
                center: IntVec2 {
                    x: col * COL_WIDTH,
                    y: 120 + (row - 1) * scene.row_height(),
                },
                radius: DOOM_RADIUS,
            },
            hittable_rows: scene.hittable_rows(*row, DOOM_HIT_RANGE),
        }
    }
}

#[derive(Debug)]
enum ImpState {
    S0,
    S71,
    S72 { countdown: i32 },
}

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
    h: f32,
    y_shift: f32,
    row: i32,
}

impl Position {
    fn interceptable(
        &self,
        Explode {
            range,
            hittable_rows,
        }: &Explode,
    ) -> bool {
        hittable_rows.contains(&self.row)
            && circle_rectangle_intersect(
                range,
                &Rectangle {
                    upper_left: IntVec2 {
                        x: self.x as i32,
                        y: (self.y - self.h + self.y_shift) as i32,
                    } + IMP_DEFENSE_SHIFT,
                    width: IMP_DEFENSE_WIDTH,
                    height: IMP_DEFENSE_HEIGHT,
                },
            )
    }
}

#[derive(Debug)]
struct Imp {
    state: ImpState,
    velocity: Vec2,
    position: Position,
    exist_time: i32,
}

#[derive(PartialEq, Debug)]
pub enum Eat {
    Empty,
    OnlyEat(i32),
    Both { eat: i32, iceable: i32 },
}

impl Eat {
    fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Empty, any) | (any, Self::Empty) => any,
            (Self::OnlyEat(eat), Self::OnlyEat(other_eat))
            | (Self::Both { eat, iceable: _ }, Self::OnlyEat(other_eat))
            | (
                Self::OnlyEat(eat),
                Self::Both {
                    eat: other_eat,
                    iceable: _,
                },
            ) => Self::OnlyEat(cmp::min(eat, other_eat)),
            (
                Self::Both { eat, iceable },
                Self::Both {
                    eat: other_eat,
                    iceable: other_iceable,
                },
            ) => Self::Both {
                eat: (cmp::min(eat, other_eat)),
                iceable: (cmp::max(iceable, other_iceable)),
            },
        }
    }

    pub fn shift_to_plant_intercept(&mut self) {
        match self {
            Eat::Empty => {}
            Eat::OnlyEat(eat) => *self = Eat::OnlyEat(*eat + 1),
            Eat::Both { eat, iceable } => {
                *self = Eat::Both {
                    eat: *eat + 1,
                    iceable: *iceable,
                }
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Intercept {
    Empty,
    Fail,
    OnlyHighIndexImp,
    Success { min: i32, max: i32 },
}

impl Intercept {
    fn update(&mut self, curr_time: i32, position: &Position, explode: &Explode) {
        let interceptable = position.interceptable(explode);
        match self {
            Self::Empty | Self::Fail => {
                if interceptable {
                    *self = Self::OnlyHighIndexImp;
                } else {
                    *self = Self::Fail
                }
            }
            Self::OnlyHighIndexImp => {
                if interceptable {
                    *self = Self::Success {
                        min: curr_time,
                        max: curr_time,
                    }
                }
            }
            Self::Success { min: _, max } => {
                if interceptable {
                    *max = curr_time;
                }
            }
        }
    }

    fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Empty, any) | (any, Self::Empty) => any,
            (
                Self::Success { min, max },
                Self::Success {
                    min: other_min,
                    max: other_max,
                },
            ) => {
                let new_min = cmp::max(min, other_min);
                let new_max = cmp::min(max, other_max);
                if new_min <= new_max {
                    Self::Success {
                        min: new_min,
                        max: new_max,
                    }
                } else {
                    Self::Fail
                }
            }
            _ => Self::Fail,
        }
    }

    pub fn shift_to_plant_intercept(&mut self) {
        match self {
            Intercept::Empty | Intercept::Fail | Intercept::OnlyHighIndexImp => {}
            Intercept::Success { min, max } => {
                *self = Intercept::Success {
                    min: *min + 1,
                    max: *max + 1,
                }
            }
        }
    }
}

pub enum GargXRange {
    Cancelled,
    Modified { min: f32, max: f32 },
    Ok { min: f32, max: f32 },
}

impl GargXRange {
    pub fn of_min_max_garg_pos(min_max_garg_pos: (f32, f32)) -> GargXRange {
        let (min_garg_x, max_garg_x) = min_max_garg_pos;
        if min_garg_x < GARG_THROW_IMP_THRES {
            if max_garg_x < GARG_THROW_IMP_THRES {
                GargXRange::Cancelled
            } else {
                GargXRange::Modified {
                    min: GARG_THROW_IMP_THRES,
                    max: max_garg_x,
                }
            }
        } else {
            GargXRange::Ok {
                min: min_garg_x,
                max: max_garg_x,
            }
        }
    }

    fn to_list(&self) -> Vec<f32> {
        match self {
            GargXRange::Cancelled => vec![],
            GargXRange::Modified { min, max } | GargXRange::Ok { min, max } => vec![*min, *max],
        }
    }
}

pub fn judge(
    garg_x_range: &GargXRange,
    explode_and_garg_rows: &[(Explode, &Vec<i32>)],
    iced: bool,
    scene: &Scene,
) -> (Eat, Intercept) {
    let mut eat = Eat::Empty;
    let mut intercept = Intercept::Empty;
    for (explode, garg_rows) in explode_and_garg_rows {
        for garg_x in garg_x_range.to_list() {
            // 只考虑巨人x极值得到的最早啃/冰绝对精确，但可拦区间并非绝对精确，可能存在接近边界的反例值
            for &garg_row in *garg_rows {
                for rnd in [0, 100] {
                    // rnd 单调影响y初速，只需考虑极值
                    let (new_eat, new_intercept) = judge_internal(
                        &Vec2 {
                            x: garg_x,
                            y: (scene.zombie_base_y() + (garg_row - 1) * scene.row_height()) as f32,
                        },
                        garg_row,
                        rnd,
                        iced,
                        scene.is_roof(),
                        explode,
                    );
                    eat = eat.merge(new_eat);
                    intercept = intercept.merge(new_intercept);
                }
            }
        }
    }
    (eat, intercept)
}

// 默认炮激活、炮拦截
fn judge_internal(
    garg_pos: &Vec2,
    garg_row: i32,
    rnd: i32,
    iced: bool,
    roof: bool,
    explode: &Explode,
) -> (Eat, Intercept) {
    if garg_pos.x < GARG_THROW_IMP_THRES || (garg_pos.x < 501. && rnd != 0) {
        return (Eat::Empty, Intercept::Empty);
    }
    let eat_loop = if iced { 8 } else { 4 };
    let imp_spawn_time = if iced { 210 } else { 105 };
    let y_shift = |x: f32, roof: bool| {
        if !roof || x >= 400. {
            0.
        } else {
            (400. - x) / 4.
        }
    };
    let mut imp = Imp {
        state: ImpState::S71,
        position: Position {
            x: garg_pos.x - 133.,
            y: garg_pos.y,
            h: 88.,
            y_shift: y_shift(garg_pos.x - 133., roof),
            row: garg_row,
        },
        velocity: Vec2 {
            x: -3.,
            y: (garg_pos.x - 360. - (if roof { 180. } else { 0. }) - rnd as f32) / 120.,
        },
        exist_time: 0,
    };
    let mut eat = Eat::Empty;
    let mut intercept = Intercept::Empty;
    for tick in (imp_spawn_time + 1).. {
        imp.exist_time += 1;
        match imp.state {
            ImpState::S71 => {
                imp.velocity = imp.velocity + GRAVITY;
                imp.position.x += imp.velocity.x;
                imp.position.h += imp.velocity.y;
                imp.position.y_shift = y_shift(imp.position.x, roof);
                if ((imp.position.h + imp.position.y_shift) as i32) < 0 {
                    imp.position.h = 0.;
                    imp.state = ImpState::S72 {
                        countdown: (if iced { 50 } else { 25 }),
                    }
                }
            }
            ImpState::S72 { countdown } => {
                imp.state = ImpState::S72 {
                    countdown: (countdown - 1),
                };
                if countdown - 1 == 0 {
                    imp.state = ImpState::S0;
                    if imp.exist_time % eat_loop == 0 {
                        eat = Eat::OnlyEat(tick);
                    }
                }
            }
            ImpState::S0 => {
                let diff_til_next_multiple = |num: i32, multiplier: i32| {
                    let remainder = num % multiplier;
                    if remainder == 0 {
                        0
                    } else {
                        multiplier - remainder
                    }
                };
                eat = Eat::Both {
                    eat: match eat {
                        Eat::OnlyEat(eat) | Eat::Both { eat, iceable: _ } => eat,
                        Eat::Empty => cmp::min(
                            diff_til_next_multiple(imp.exist_time, eat_loop) + tick,
                            diff_til_next_multiple(imp.exist_time - 1, eat_loop) + tick,
                        ),
                    },
                    iceable: tick + 1,
                };
                break;
            }
        }
        intercept.update(tick, &imp.position, explode);
    }
    (eat, intercept)
}

pub struct IceAndCobTimes {
    pub ice_times: Vec<i32>,
    pub cob_time: i32,
}

impl IceAndCobTimes {
    pub fn of_ice_times_and_cob_time(
        ice_times: &[i32],
        cob_time: i32,
    ) -> Result<IceAndCobTimes, String> {
        if cob_time < 0 {
            return Err(format!("炮生效时间应≥0 (当前为: {cob_time})"));
        }
        let mut ice_times = ice_times
            .iter()
            .filter(|&&v| v >= 0 && v <= cob_time)
            .cloned()
            .collect::<Vec<i32>>();
        ice_times.sort();
        Ok(IceAndCobTimes {
            ice_times,
            cob_time,
        })
    }

    pub fn is_iced(&self) -> bool {
        match (self.ice_times.last(), self.cob_time) {
            (None, _) => false,
            (Some(last_ice_time), cob_time) => cob_time - last_ice_time <= ICE_SLOW_TOTAL_TIME,
        }
    }
}

pub fn min_max_garg_x(
    IceAndCobTimes {
        ice_times,
        cob_time,
    }: &IceAndCobTimes,
) -> Result<(f32, f32), String> {
    let (min_half_ticks, max_half_ticks) = min_max_garg_walk_in_half_ticks(&ice_times, *cob_time);
    match (
        constants::garg_slow_of_half_ticks(min_half_ticks),
        constants::garg_fast_of_half_ticks(max_half_ticks),
    ) {
        (None, _) => Err(format!(
            "巨人最短行走时间[{}]超出数据范围({}~{})",
            min_half_ticks as f32 / 2.,
            0,
            constants::GARG_DATA_SIZE - 1
        )),
        (_, None) => Err(format!(
            "巨人最长行走时间[{}]超出数据范围({}~{})",
            max_half_ticks as f32 / 2.,
            0,
            constants::GARG_DATA_SIZE - 1
        )),
        (Some(min_walk), Some(max_walk)) => {
            Ok((MIN_GARG_START_POS - max_walk, MAX_GARG_START_POS - min_walk))
        }
    }
}

fn min_max_garg_walk_in_half_ticks(valid_ice_times: &[i32], cob_time: i32) -> (i32, i32) {
    (
        garg_walk_in_half_ticks(
            valid_ice_times,
            cob_time,
            MAX_ICE_TIME_FOR_ICED,
            MAX_ICE_TIME_FOR_UNICED,
        ),
        garg_walk_in_half_ticks(
            valid_ice_times,
            cob_time,
            MIN_ICE_TIME_FOR_ICED,
            MIN_ICE_TIME_FOR_UNICED,
        ),
    )
}

fn garg_walk_in_half_ticks(
    valid_ice_times: &[i32],
    cob_time: i32,
    ice_length_for_iced: i32,
    ice_length_for_uniced: i32,
) -> i32 {
    enum Tick {
        Start(i32),
        Ice { time: i32, length: i32 },
        Cob(i32),
    }

    impl Tick {
        // 返回值单位为 0.5cs
        fn diff_in_half_ticks(old_tick: &Tick, new_tick: &Tick) -> i32 {
            let prorated_walk = |walk: i32, ice_length| {
                let uniced_walk = cmp::max(walk - (ICE_SLOW_TOTAL_TIME - ice_length), 0);
                (walk - uniced_walk) + uniced_walk * 2
            };
            match (new_tick, old_tick) {
                (Tick::Start(_), _) => panic!("Tick::Start只能是被减数"),
                (Tick::Ice { time: _, length: _ }, Tick::Cob(_)) => {
                    panic!("Tick::Cob须晚于Tick::Ice")
                }
                (Tick::Cob(_), Tick::Cob(_)) => panic!("Tick::Cob只能存在一个"),
                (
                    Tick::Ice {
                        time: new_time,
                        length: _,
                    },
                    Tick::Start(old_time),
                ) => cmp::max((new_time - old_time - 1) * 2, 0),
                (
                    Tick::Ice {
                        time: new_time,
                        length: _,
                    },
                    Tick::Ice {
                        time: old_time,
                        length,
                    },
                ) => prorated_walk(cmp::max(new_time - old_time - (length - 1), 0), length),
                (Tick::Cob(new_time), Tick::Start(old_time)) => {
                    cmp::max((new_time - old_time) * 2, 0)
                }
                (
                    Tick::Cob(new_time),
                    Tick::Ice {
                        time: old_time,
                        length,
                    },
                ) => prorated_walk(cmp::max(new_time - old_time - (length - 2), 0), length),
            }
        }
    }

    let mut ticks: Vec<Tick> = vec![Tick::Start(0)];
    let mut prev_ice_time = None;
    for &ice_time in valid_ice_times {
        let iced: bool = match prev_ice_time {
            None => false,
            Some(prev_ice_time) => ice_time - prev_ice_time < ICE_SLOW_TOTAL_TIME,
        };
        ticks.push(Tick::Ice {
            time: ice_time,
            length: if iced {
                ice_length_for_iced
            } else {
                ice_length_for_uniced
            },
        });
        prev_ice_time = Some(ice_time);
    }
    ticks.push(Tick::Cob(cob_time));
    ticks
        .windows(2)
        .map(|pair| Tick::diff_in_half_ticks(&pair[0], &pair[1]))
        .sum::<i32>()
}

pub fn hit_col_matching_int_pixel(unvalidated_hit_col: f32) -> Option<f32> {
    let pixel = unvalidated_hit_col * 80.;
    let diff_to_int_pixel = f32::min(pixel - pixel.floor(), pixel.ceil() - pixel);
    let is_close_enough_to_int = diff_to_int_pixel <= FLOAT_INT_DIFF_TOLERANCE;
    if is_close_enough_to_int {
        None
    } else {
        Some(pixel.round() / 80.)
    }
}

pub fn unsafe_intercept_interval(eat: &Eat, intercept: &Intercept) -> Option<(i32, i32)> {
    match (&eat, &intercept) {
        (Eat::OnlyEat(eat), Intercept::Success { min, max })
        | (Eat::Both { eat, iceable: _ }, Intercept::Success { min, max })
            if eat <= max =>
        {
            Some((cmp::max(*eat, *min), *max))
        }
        _ => None,
    }
}

pub fn safe_intercept_interval(eat: &Eat, intercept: &Intercept) -> Option<(i32, i32)> {
    match (&eat, &intercept) {
        (Eat::OnlyEat(eat), Intercept::Success { min, max })
        | (Eat::Both { eat, iceable: _ }, Intercept::Success { min, max })
            if eat > min =>
        {
            Some((*min, cmp::min(*eat - 1, *max)))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ground_judge() {
        let scene = Scene::PE;
        let explode = Explode::of_cob(&Cob::Ground { row: 1, col: 8.5 }, &scene);
        let (eat, intercept) =
            judge_internal(&Vec2 { x: 800., y: 50. }, 1, 57, false, false, &explode);
        assert_eq!(
            eat,
            Eat::Both {
                eat: 281,
                iceable: 283
            }
        );
        assert_eq!(intercept, Intercept::Success { min: 107, max: 134 });
        let (eat, intercept) =
            judge_internal(&Vec2 { x: 800., y: 50. }, 1, 45, false, false, &explode);
        assert_eq!(
            eat,
            Eat::Both {
                eat: 285,
                iceable: 286
            }
        );
        assert_eq!(intercept, Intercept::Success { min: 107, max: 133 });
        let (eat, intercept) = judge(
            &GargXRange::Ok {
                min: 800.,
                max: 800.,
            },
            &vec![(explode.clone(), &vec![1])],
            false,
            &scene,
        );
        assert_eq!(
            eat,
            Eat::Both {
                eat: 269,
                iceable: 299
            }
        );
        assert_eq!(intercept, Intercept::Success { min: 107, max: 128 });

        let (eat, intercept) = judge(
            &GargXRange::Ok {
                min: 555.,
                max: 666.,
            },
            &vec![(explode.clone(), &vec![1, 2])],
            true,
            &scene,
        );
        assert_eq!(
            eat,
            Eat::Both {
                eat: 338,
                iceable: 391
            }
        );
        assert_eq!(intercept, Intercept::Fail);
    }

    #[test]
    fn test_roof_judge() {
        let scene = Scene::RE;
        let explode = Explode::of_cob(
            &&Cob::Roof {
                row: 1,
                col: 8.5,
                cob_col: 3,
                cob_row: 3,
            },
            &scene,
        );
        let (eat, intercept) =
            judge_internal(&Vec2 { x: 800., y: 40. }, 1, 50, false, true, &explode);
        assert_eq!(
            eat,
            Eat::Both {
                eat: 241,
                iceable: 240
            }
        );
        assert_eq!(intercept, Intercept::Success { min: 107, max: 167 });
        let (eat, intercept) =
            judge_internal(&Vec2 { x: 800., y: 40. }, 1, 70, false, true, &explode);
        assert_eq!(
            eat,
            Eat::Both {
                eat: 233,
                iceable: 234
            }
        );
        assert_eq!(intercept, Intercept::Success { min: 107, max: 167 });
        let (eat, intercept) = judge(
            &GargXRange::Ok {
                min: 800.,
                max: 800.,
            },
            &vec![(explode.clone(), &vec![1])],
            false,
            &scene,
        );
        assert_eq!(
            eat,
            Eat::Both {
                eat: 225,
                iceable: 255
            }
        );
        assert_eq!(intercept, Intercept::Success { min: 107, max: 167 });

        let (eat, intercept) = judge(
            &GargXRange::Ok {
                min: 555.,
                max: 666.,
            },
            &vec![(explode.clone(), &vec![1, 2])],
            true,
            &scene,
        );
        assert_eq!(
            eat,
            Eat::Both {
                eat: 322,
                iceable: 357
            }
        );
        assert_eq!(intercept, Intercept::Fail);
    }

    #[test]
    fn test_min_max_walk() {
        let (min, max) = min_max_garg_walk_in_half_ticks(&(Vec::new()), 10);
        assert_eq!((min, max), (20, 20));
        let (min, max) = min_max_garg_walk_in_half_ticks(&(vec![1]), 0);
        assert_eq!((min, max), (0, 0));
        let (min, max) = min_max_garg_walk_in_half_ticks(&(vec![1]), 400);
        assert_eq!((min, max), (0, 1));
        let (min, max) = min_max_garg_walk_in_half_ticks(&(vec![1]), 500);
        assert_eq!((min, max), (0, 101));
        let (min, max) = min_max_garg_walk_in_half_ticks(&(vec![1]), 600);
        assert_eq!((min, max), (1, 201));
        let (min, max) = min_max_garg_walk_in_half_ticks(&(vec![1]), 1999);
        assert_eq!((min, max), (1400, 1600));
        let (min, max) = min_max_garg_walk_in_half_ticks(&(vec![1]), 2000);
        assert_eq!((min, max), (1402, 1602));
        let (min, max) = min_max_garg_walk_in_half_ticks(&(vec![1, 2]), 301);
        assert_eq!((min, max), (0, 1));
        let (min, max) = min_max_garg_walk_in_half_ticks(&(vec![1, 2]), 501);
        assert_eq!((min, max), (101, 201));
        let (min, max) = min_max_garg_walk_in_half_ticks(&(vec![1, 500]), 1000);
        assert_eq!((min, max), (102, 302));
        let (min, max) = min_max_garg_walk_in_half_ticks(&(vec![1, 2500]), 3000);
        assert_eq!((min, max), (2400, 2702));
    }
}
