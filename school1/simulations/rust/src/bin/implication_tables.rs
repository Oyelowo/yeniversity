fn main() {
    let values = [true, false];

    println!("p q p->q q->p p<->q");
    for &p in &values {
        for &q in &values {
            let p_implies_q = !p || q;
            let q_implies_p = !q || p;
            let equivalent = p == q;

            println!(
                "{} {} {} {} {}",
                tf(p),
                tf(q),
                tf(p_implies_q),
                tf(q_implies_p),
                tf(equivalent)
            );
        }
    }
}

fn tf(value: bool) -> char {
    if value { 'T' } else { 'F' }
}
