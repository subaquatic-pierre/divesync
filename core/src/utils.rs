pub fn calc_ata(depth: f64) -> f64 {
    (depth + 10.0) / 10.0
}

pub fn calc_pp(ata: f64, gas_pp: f64) -> f64 {
    ata * gas_pp
}

pub fn round_f32(num: f32, num_dec: usize) -> f32 {
    let str_num = format!("{:.1$}", num, num_dec);

    str::parse::<f32>(&str_num).unwrap()
}

pub fn n_root(num: f32, root_n: usize) -> f32 {
    f32::powf(num, 1.0 / root_n as f32)
}
