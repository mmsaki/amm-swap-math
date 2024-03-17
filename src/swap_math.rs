pub fn run() {
    // before transaction
    let x: f32 = 200.0; // amount tokens x in pool
    let y: f32 = 800_000.0; // amount tokens y in pool
    let fee = 1.0 - 0.003; // uniswap fee
    let l = x * y; // constant L

    println!("pool before swap");
    println!("x={}ETH, y=${}, L={}", x, y, l);

    // after transaction
    let lambda: f32 = 200_000.0; // amount in of token y
    let amount_in = (x * fee * lambda) / (y + fee * lambda);
    let x1 = x - amount_in; // amount tokens x after
    let y1 = y + lambda; // amount tokens y after

    let li = x1 * y1; // constant L after trade

    assert!(li > l);

    println!("\npool after swap");
    println!("x={}ETH, y=${}, L={}", x1, y1, li);

    // Impact of trade
    let p = y / x; // spot price
    let pe = lambda / (x - x1); // effective price
    let impact = pe * (x - x1) - p * (x - x1); // price impact of trade
    let pi = y1 / x1; // new price after trade

    println!("\nprice impact of swap");
    println!("p=${}, pe=${}, i=${}, pi=${}", p, pe, impact, pi);
}
