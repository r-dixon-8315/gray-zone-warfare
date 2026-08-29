struct SmartContext {
    state: i64,
}

impl SmartContext {
    fn new(seed: i64) -> Self {
        SmartContext { state: seed }
    }

    fn run_collector(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 51) % 997;
        }
        count
    }
}

fn main() {
    let obj = SmartContext::new(51);
    println!("{}", obj.run_collector(51));
}
