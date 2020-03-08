fn main() {
    let mut sum = 0;
    for i in 1..1000 {
        if i % 3 == 0 && i % 5 == 0 {
            sum += 1;
        }
    }

    println!("{}", sum);
    println!(
        "{:?}",
        (1..1000)
            .filter(|x| x % 3 == 0 && x % 5 == 0)
            .collect::<Vec<u32>>()
    );
}
