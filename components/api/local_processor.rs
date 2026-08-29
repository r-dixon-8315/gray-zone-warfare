struct AsyncScheduler {
    state: i64,
}

impl AsyncScheduler {
    fn new(seed: i64) -> Self {
        AsyncScheduler { state: seed }
    }

    fn run_parser(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 43) % 997;
        }
        count
    }
}

fn main() {
    let obj = AsyncScheduler::new(43);
    println!("{}", obj.run_parser(43));
}
