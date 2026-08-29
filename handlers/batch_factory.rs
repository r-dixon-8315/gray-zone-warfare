struct CoreController {
    state: i64,
}

impl CoreController {
    fn new(seed: i64) -> Self {
        CoreController { state: seed }
    }

    fn load_resolver(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 59) % 997;
        }
        count
    }
}

fn main() {
    let obj = CoreController::new(59);
    println!("{}", obj.load_resolver(59));
}
