pub mod common;
mod calculate_pos_distribution;
mod parse_data;

pub use common::ZombieType;

fn ensure_rayon_pool_initialized() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        let _ = rayon::ThreadPoolBuilder::new()
            .stack_size(16 << 20)
            .build_global();
    });
}

lazy_static::lazy_static! {
    static ref ZOMBIE_DB: std::collections::HashMap<ZombieType, common::ZombieData> =
        parse_data::get_zombie_db(include_bytes!("../../assets/data.csv"));
}

pub fn min_max_garg_x_from_zmc(ice_times: &[i32], time: i32) -> (f32, f32) {
    ensure_rayon_pool_initialized();
    let ice_times_i64: Vec<i64> = ice_times.iter().map(|&x| i64::from(x)).collect();
    let d = calculate_pos_distribution::calculate_pos_distribution(
        &ZOMBIE_DB[&ZombieType::Gargantuar],
        &ice_times_i64,
        i64::from(time),
    );
    (d.min as f32, d.max as f32)
}