# p002 - Even Fibonacci numbers

> 400 万之内所有偶数的斐波那契数字之和。
>
> Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
>
> 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, …
>
> By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

## 解法1: 枚举法

- 先准备向量列出 `1、2` 项，并初始值相应的值和索引
- 开始便利，终止条件为索引到 `4_000_000`
- 将第 `n` 项推入向量，并判断条件来累累加
- 索引 `+1`

```rust
fn main() {
    let mut fib = vec![1, 2];

    let mut i = 2;
    let mut sum = 2;

    loop {
        let c = fib[i - 1] + fib[i - 2];
        if c >= 4_000_000 {
            break;
        }
        fib.push(c);
        if c % 2 == 0 {
            sum += c;
        }
        i += 1;
    }

    println!("{}", sum);
}
```

## 解法2: 枚举法 + 函数式

- 先枚举好
- 再函数式一步到位
- `.sum::()` 是一个泛型函数

```rust
for i in 2.. {
    let c = fib[i - 1] + fib[i - 2];
    if c >= 4_000_000 {
        break;
    }
    fib.push(c);
}

println!("{}", fib.iter().filter(|&x| x % 2 == 0).sum::<u32>());
````