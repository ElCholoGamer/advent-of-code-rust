use num::Num;

pub fn gcd<N: Num + Copy>(a: N, b: N) -> N {
    if b == N::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn mcm<N: Num + Copy>(numbers: &[N]) -> N {
    let mut result = N::one();
    for &num in numbers {
        let divisor = gcd(result, num);
        result = (result * num) / divisor;
    }

    result
}