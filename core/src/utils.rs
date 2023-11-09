pub fn calc_ata(depth: f32) -> f32 {
    (depth + 10.0) / 10.0
}

pub fn round_f32(num: f32, num_dec: usize) -> f32 {
    let str_num = format!("{:.1$}", num, num_dec);

    str::parse::<f32>(&str_num).unwrap()
}

pub fn n_root(num: f32, root_n: usize) -> f32 {
    f32::powf(num, 1.0 / root_n as f32)
}

mod test {
    use super::*;

    #[test]
    fn test_n_root() {
        assert_eq!(4.0, n_root(16.0, 2))
    }

    #[test]
    fn test_round_f32() {
        assert_eq!(4.006, round_f32(4.005999999, 4))
    }

    #[test]
    fn test_calc_ata() {
        assert_eq!(4.2, calc_ata(32.0))
    }
}
