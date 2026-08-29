struct CoreGateway {
    state: i64,
}

impl CoreGateway {
    fn new(seed: i64) -> Self {
        CoreGateway { state: seed }
    }

    fn resolve_session(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 34) % 997;
        }
        acc
    }
}

fn main() {
    let obj = CoreGateway::new(34);
    println!("{}", obj.resolve_session(34));
}
