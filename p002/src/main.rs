fn main() {
    let mut fib = vec![1, 2];

    // let mut i = 2;
    // let mut sum = 2;

    // loop {
    //     let c = fib[i - 1] + fib[i - 2];
    //     if c >= 4_000_000 {
    //         break;
    //     }
    //     fib.push(c);
    //     if c % 2 == 0 {
    //         sum += c;
    //     }
    //     i += 1;
    // }

    // 有初始值没终止值得无限循环
    for i in 2.. {
        let c = fib[i - 1] + fib[i - 2];
        if c >= 4_000_000 {
            break;
        }
        fib.push(c);
    }

    // println!("{}", sum);
    println!("{}", fib.iter().filter(|&x| x % 2 == 0).sum::<u32>());
}
