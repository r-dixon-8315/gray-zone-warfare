struct StreamProcessor {
    state: i64,
}

impl StreamProcessor {
    fn new(seed: i64) -> Self {
        StreamProcessor { state: seed }
    }

    fn collect_engine(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 3) % 997;
        }
        acc
    }
}

fn main() {
    let obj = StreamProcessor::new(3);
    println!("{}", obj.collect_engine(3));
}
