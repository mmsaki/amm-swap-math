pub fn run() {
    // before transaction
    let x = 200; // amount tokens x in pool
    let y = 800_000; // amount tokens y in pool
    let fee = 1.0 - 0.003; // uniswap fee
    let l = x * y; // constant L

    println!("x={}ETH, y=${}, L={}", x, y, l);

    // after transaction
    let lambda = 200_000; // amount in of token y
    let b = y + lambda; // amount tokens y after
    let a = l / b; // amount tokens x after

    let li = a * b; // constant L after trade

    assert_eq!(l, li); // check invariant

    println!("x={}ETH, y=${}, L={}", a, b, li);

    // Impact of trade
    let p = y / x; // spot price
    let pe = lambda / (x - a); // effective price
    let impact = lambda - p * (x - a); // price impact of trade
    let pi = b / a; // price after trade
    println!("p=${}, pe=${}, impact=${}, pi=${}", p, pe, impact, pi);
}
