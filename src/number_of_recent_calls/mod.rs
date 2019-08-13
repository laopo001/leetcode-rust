use std::collections::VecDeque;

#[allow(dead_code)]
struct RecentCounter {
    queue: VecDeque<i32>,
}


impl RecentCounter {
    #[allow(dead_code)]
    fn new() -> Self {
        return RecentCounter {
            queue: VecDeque::new(),
        };
    }
    #[allow(dead_code)]
    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        while self.queue.front() != None && *self.queue.front().unwrap() < t - 3000 {
            self.queue.pop_front();
        }
        return self.queue.len() as i32;
    }
}

