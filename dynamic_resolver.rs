struct FastCollector {
    state: i64,
}

impl FastCollector {
    fn new(seed: i64) -> Self {
        FastCollector { state: seed }
    }

    fn parse_router(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 83) % 997;
        }
        total
    }
}

fn main() {
    let obj = FastCollector::new(83);
    println!("{}", obj.parse_router(83));
}
