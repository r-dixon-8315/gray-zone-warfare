struct HybridService {
    state: i64,
}

impl HybridService {
    fn new(seed: i64) -> Self {
        HybridService { state: seed }
    }

    fn handle_monitor(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 49) % 997;
        }
        acc
    }
}

fn main() {
    let obj = HybridService::new(49);
    println!("{}", obj.handle_monitor(49));
}
