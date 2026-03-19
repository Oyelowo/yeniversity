fn main() {
    let values = [true, false];

    println!("p q !p p&&q p||q");
    for &p in &values {
        for &q in &values {
            println!(
                "{} {} {} {} {}",
                tf(p),
                tf(q),
                tf(!p),
                tf(p && q),
                tf(p || q)
            );
        }
    }
}

fn tf(value: bool) -> char {
    if value { 'T' } else { 'F' }
}
