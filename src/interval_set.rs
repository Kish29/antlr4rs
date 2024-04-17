use std::cmp::{max, min};

/// [Interval] represents interval equivalent to start..=stop
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interval {
    pub start: isize,
    pub stop: isize,
}

impl Interval {
    // #[inline(always)]
    fn new(start: isize, stop: isize) -> Self {
        Self { start, stop }
    }
}

#[derive(Debug, Clone)]
pub struct IntervalSet {
    intervals: Vec<Interval>,
    read_only: bool,
}

impl IntervalSet {
    // #[inline(always)]
    pub fn new() -> Self {
        Self { intervals: vec![], read_only: false }
    }

    // #[inline(always)]
    pub fn add_one(&mut self, v: isize) {
        self.add_range(v, v)
    }

    // #[inline(always)]
    pub fn add_range(&mut self, l: isize, r: isize) {
        if self.read_only {
            panic!("modify a read only interval set!.")
        }
        self.add_interval(Interval::new(l, r))
    }

    /// Add interval; i.e., add all integers from a to b to set.
    /// If b<a, do nothing.
    /// Keep list in sorted order (by left range value).
    /// If overlap, combine ranges.  For example,
    /// If this is {1..5, 10..20}, adding 6..7 yields
    /// {1..5, 6..7, 10..20}.  Adding 4..8 yields {1..8, 10..20}.
    // #[inline]
    fn add_interval(&mut self, v: Interval) {
        if v.stop < v.start {
            return;
        }
        // find insert position in list
        let mut pos = 0;
        while let Some(interval) = self.intervals.get_mut(pos) {
            if *interval == v {
                return;
            }
            // v is left disjoint on current.
            if v.stop < interval.start {
                self.intervals.insert(pos, v);
                return;
            }
            // v is adjacent on current.
            if v.stop == interval.start {
                interval.start = v.start;
                return;
            }
            // v is partial overlap on current.
            if v.start <= interval.stop {
                // reassign the more large range one.
                let union = Interval::new(min(v.start, interval.start), max(v.stop, interval.stop));
                *interval = union;

                if pos < self.intervals.len() - 1 {
                    let l = &self.intervals[pos];
                    let r = &self.intervals[pos + 1];
                    // if l contains r
                    if l.stop >= r.stop {
                        // remove next element.
                        self.intervals.remove(pos + 1);
                    } else if l.stop >= r.start {   // if l partial overlap at r
                        self.intervals[pos] = Interval::new(l.start, r.stop);
                        self.intervals.remove(pos + 1);
                    }
                }
                return;
            }
            // if disjoint and right behind current interval, a future iteration will handle it.
            pos += 1;
        }
        // ok, must be after last interval (and disjoint from last interval)
        // just add it.
        self.intervals.push(v);
    }
}