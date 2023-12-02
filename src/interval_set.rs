use std::cmp::{max, min};

/// [Interval] represents interval equivalent to start..=stop
#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    start: isize,
    stop: isize,
}

impl Interval {
    #[inline(always)]
    fn new(start: isize, stop: isize) -> Self {
        Self { start, stop }
    }

    // Does this start completely before other? Disjoint
    #[inline(always)]
    fn left_disjoint(&self, other: &Interval) -> bool {
        self.stop < other.start && self.start < other.start
    }

    #[inline(always)]
    fn right_disjoint(&self, other: &Interval) -> bool {
        self.start > other.stop && self.stop > other.stop
    }

    // Return the interval computed from combining this and other.
    #[inline(always)]
    fn union(&self, other: &Interval) -> Interval {
        Self {
            start: min(self.start, other.start),
            stop: max(self.stop, other.stop),
        }
    }

    // Are two intervals adjacent such as 0..41 and 42..42?
    #[inline(always)]
    fn adjacent(&self, other: &Interval) -> bool {
        self.start == other.stop + 1 || self.stop == other.start - 1
    }

    // Are both ranges disjoint? I.e., no overlap?
    #[inline(always)]
    fn disjoint(&self, other: &Interval) -> bool {
        self.left_disjoint(other) || self.right_disjoint(other)
    }
}

pub struct IntervalSet {
    pub intervals: Vec<Interval>,
    pub read_only: bool,
}

impl IntervalSet {
    #[inline(always)]
    pub fn new() -> Self {
        Self { intervals: vec![], read_only: false }
    }

    #[inline(always)]
    pub fn add_one(&mut self, v: isize) {
        self.add_range(v, v)
    }

    #[inline(always)]
    pub fn add_range(&mut self, l: isize, r: isize) {
        if self.read_only {
            panic!("modify a read only interval set!.")
        }
        self.add_interval(Interval::new(l, r))
    }

    /// insert an interval and guarantee that interval sets are ordered and
    /// elements adjacent with each other or disjoint with each other.
    #[inline]
    fn add_interval(&mut self, addition: Interval) {
        if addition.stop < addition.start {
            return;
        }
        // find insert position in list
        let mut pos = 0;
        while let Some(interval) = self.intervals.get_mut(pos) {
            if *interval == addition {
                return;
            }
            if addition.adjacent(interval) || !addition.disjoint(interval) {
                // next to each other, make a single larger interval
                *interval = addition.union(interval);
                // make sure we didn't just create an interval that
                // should be merged with next interval in list.
                loop {
                    pos += 1;
                    let next = match self.intervals.get(pos) {
                        Some(i) => i,
                        None => break
                    };
                    if !interval.adjacent(next) && interval.disjoint(next) {
                        break;
                    }
                    // if we bump up against or overlap next, merge
                    self.intervals[pos - 1] = interval.union(next);
                    self.intervals.remove(pos);
                }
                return;
            }
            if addition.left_disjoint(interval) {
                self.intervals.insert(pos, addition);
                return;
            }
            // if disjoint and right behind current interval, a future iteration will handle it.
            pos += 1;
        }
        // ok, must be after last interval (and disjoint from last interval)
        // just add it.
        self.intervals.push(addition);
    }
}