use std::collections::VecDeque;

pub struct Stack(VecDeque<i32>);

impl Stack {
    pub fn receive_push_from_other(&mut self, other: &mut Self) {
        if let Some(top) = other.0.pop_front() {
            self.0.push_front(top)
        }
    }

    pub fn swap(&mut self) {
        let len = self.0.len();
        if len >= 2 {
            self.0.swap(0, 1)
        }
    }

    pub fn rotate(&mut self) {
        if let Some(top) = self.0.pop_front() {
            self.0.push_back(top);
        }
    }

    // front = top = first
    // back = bot = last
    pub fn reverse_rotate(&mut self) {
        if let Some(bot) = self.0.pop_back() {
            self.0.push_front(bot);
        }
    }

    pub fn from_iter<I: Iterator<Item = i32>>(iter: I) -> Self {
        Stack(VecDeque::from_iter(iter))
    }

    pub fn with_capacity(cap: usize) -> Self {
        Stack(VecDeque::with_capacity(cap))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_sorted(&mut self) -> bool {
        self.0.make_contiguous().windows(2).all(|w| w[0] < w[1])
    }
}
