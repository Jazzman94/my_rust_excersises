fn main() {
    let n: u32 = 100;
    let mut fib: Vec<u32> = vec![1,1];

    for number in 2..n {
        fib.push(fib[number as usize -2] + fib[number as usize -1]);
    }
    println!("{}th is {}",n,fib[n as usize - 1])
}
