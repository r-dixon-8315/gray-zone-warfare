struct AsyncResolver {
    state: i64,
}

impl AsyncResolver {
    fn new(seed: i64) -> Self {
        AsyncResolver { state: seed }
    }

    fn collect_worker(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 91) % 997;
        }
        acc
    }
}

fn main() {
    let obj = AsyncResolver::new(91);
    println!("{}", obj.collect_worker(91));
}
