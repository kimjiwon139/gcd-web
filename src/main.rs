fn gcd(mut n: u64, mut m: u64)  -> u64 {
    assert!(n!=0 && m!=0);
    while m != 0 {
        if m<n {
            let t = m;
            m=n;
            n=t;
        }
        m = m%n;
    }
    n
}

fn main() {
    let k: u64 = gcd(12, 24);
    println!("gcd is {}",k);
}


#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);

    assert_eq!(gcd(2*3*5*7*11,3*5*17),3*5);
}