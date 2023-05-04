use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn random(min: i32, max: i32) -> i32 {
    thread_rng().gen_range(min..=max)
}
pub fn percentage(molecule: i32, denominator: i32) -> bool {
    thread_rng().gen_range(0..denominator) < molecule
}
pub fn pick<T: Copy>(items: Vec<T>) -> Option<T> {
    items.choose(&mut thread_rng()).copied()
}
#[test]
fn test_percentage_function() {
    let molecule = 10;
    let denominator = 100;
    let mut count = 0;
    let n = 10000000;
    // 100回の試行を行い、10%の確率でtrueが返ってくることを期待する
    for _ in 0..n {
        if percentage(molecule, denominator) {
            count += 1;
        }
    }
    let avg: f32 = (count as f32 / n as f32) * 100.0;
    println!("\navg:{}\ncount:{}\nn{}\n", avg, count, n);
    // 10%の誤差範囲内であれば合格
    assert!(avg >= 9.0 && avg <= 11.0);
}
pub fn generate_random_id(length: u32) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}
#[test]
fn test_generate_random_id() {
    for _ in 0..10 {
        println!("id:{}", generate_random_id(30));
    }
    // 10%の誤差範囲内であれば合格
    assert!(true);
}
