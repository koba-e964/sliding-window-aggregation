/// F must give a semigroup: it is required that F satisfies associativity.
/// Commutativity and existence of the identity are not required.
/// Reference: https://scrapbox.io/data-structures/Sliding_Window_Aggregation
pub struct SWAg<T, F> {
    back: Vec<T>,
    back_agg: Vec<T>,
    front: Vec<T>,
    front_agg: Vec<T>,
    f: F,
}

impl<T: Copy, F: Fn(T, T) -> T> SWAg<T, F> {
    pub fn new(f: F) -> Self {
        SWAg {
            back: Vec::new(),
            back_agg: Vec::new(),
            front: Vec::new(),
            front_agg: Vec::new(),
            f: f,
        }
    }
    pub fn fold_all(&self) -> Option<T> {
        match (self.back_agg.last(), self.front_agg.last()) {
            (None, None) => None,
            (None, x) => x.cloned(),
            (x, None) => x.cloned(),
            (Some(&x), Some(&y)) => Some((self.f)(x, y)),
        }
    }
    pub fn push_back(&mut self, x: T) {
        let last = self.front_agg.last().cloned();
        self.front.push(x);
        self.front_agg.push(match last {
            None => x,
            Some(y) => (self.f)(y, x),
        });
    }
    pub fn pop_front(&mut self) -> Option<T> {
        match self.back.pop() {
            Some(x) => return Some(x),
            None => (),
        };
        std::mem::swap(&mut self.front, &mut self.back);
        self.back.reverse();
        let returned = self.back.pop();
        let len = self.back.len();
        self.back_agg = Vec::with_capacity(self.back.len());
        if len > 0 {
            self.back_agg.push(self.back[0]);
            let mut cur = self.back[0];
            for i in 1..len {
                cur = (self.f)(self.back[i], cur);
                self.back_agg.push(cur);
            }
        }
        self.front = Vec::new();
        self.front_agg = Vec::new();
        returned
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fold_all_works() {
        let mut swag = SWAg::<i32, _>::new(|x, y| x + y);
        swag.push_back(3);
        swag.push_back(4);
        swag.push_back(1);
        assert_eq!(swag.fold_all(), Some(8));
    }
    #[test]
    fn fold_all_not_commutative_works() {
        let mut swag = SWAg::<(i32, i32), _>::new(|(x, b), (y, c)| (c * x + y, b * c));
        swag.push_back((3, 10));
        swag.push_back((4, 10));
        swag.push_back((1, 10));
        assert_eq!(swag.fold_all(), Some((341, 1000)));
    }
    #[test]
    fn fold_all_with_pop_front_works() {
        let mut swag = SWAg::<(i32, i32), _>::new(|(x, b), (y, c)| (c * x + y, b * c));
        swag.push_back((3, 10));
        swag.push_back((4, 10));
        swag.push_back((5, 10));
        swag.push_back((6, 10));
        assert_eq!(swag.pop_front(), Some((3, 10))); // 4 5 6 |
        swag.push_back((1, 10)); // 4 5 6 | 1
        assert_eq!(swag.fold_all(), Some((4561, 10000)));
    }
}
