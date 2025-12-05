#[derive(Clone, Debug, Default)]
pub struct Problem {
    pub database_ranges: Vec<DatabaseRange>,
    pub ingredient_ids: Vec<u64>,
}

impl Problem {
    pub fn new() -> Problem {
        Problem {
            database_ranges: vec![],
            ingredient_ids: vec![],
        }
    }

    pub fn add_range(&mut self, range: DatabaseRange) {
        self.database_ranges.push(range);
    }

    pub fn add_ingredient(&mut self, id: u64) {
        self.ingredient_ids.push(id);
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DatabaseRange {
    pub start: u64,
    pub end: u64,
}

impl DatabaseRange {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, item: u64) -> bool {
        self.start <= item && self.end >= item
    }

    pub fn range_size(&self) -> u64 {
        self.end - self.start + 1
    }
}

impl std::fmt::Display for DatabaseRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_digits = u64::checked_ilog10(self.start).unwrap();
        let end_digits = u64::checked_ilog10(self.end).unwrap();
        write!(
            f,
            "[{} ({}) - {} ({})] ",
            self.start, start_digits, self.end, end_digits
        )
    }
}

#[derive(Debug)]
pub struct DatabaseMultiRange {
    pub range: DatabaseRange,
    pub next: Option<Box<DatabaseMultiRange>>,
}

impl DatabaseMultiRange {
    pub fn new(initial_range: DatabaseRange) -> Self {
        DatabaseMultiRange {
            range: initial_range,
            next: None,
        }
    }

    pub fn merge(&mut self, other: DatabaseRange) {
        if self.range.contains(other.start) {
            // starts in this range
            if self.range.contains(other.end) {
                // do nothing (other is subrange ipf thois range)
                return;
            }
            self.recursive_step(other);
        } else if self.range.contains(other.end) {
            // other range starts before this range, but ends inside this range
            self.range.start = other.start;
        } else if self.range.start > other.start {
            if self.range.start > other.end {
                // other range starts and ends before this range
                let old = self.range;
                self.range = other;
                let mut created_range = DatabaseMultiRange::new(old);
                if let Some(next) = self.next.take() {
                    created_range.next = Some(next);
                }
                self.next = Some(Box::new(created_range))
            } else {
                // other is super range of this range
                self.range.start = other.start;
                self.recursive_step(other);
            }
        } else if let Some(mut next) = self.next.take() {
            // starts after this range, merge with next if any
            next.merge(other);
            self.next = Some(next);
        } else {
            // starts after this range and there is no next: append to tail
            self.next = Some(Box::new(DatabaseMultiRange::new(other)))
        }
    }

    fn recursive_step(&mut self, other: DatabaseRange){
        loop {
            if let Some(next) = self.next.take() {
                if other.end < next.range.start {
                    // other range ends before start of next range
                    self.range.end = other.end;
                    self.next = Some(next);
                    break;
                } else if other.end > next.range.end {
                    // other range ends after this iteration of next => go recursively on next of next
                    self.next = next.next;
                } else {
                    // other range ends inside this iteration of next
                    self.range.end = next.range.end;
                    self.next = next.next;
                    break;
                }
            } else {
                // other range end s after last range
                self.range.end = other.end;
                break;
            }
        }
    }
}

impl std::fmt::Display for DatabaseMultiRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.range)?;
        if let Some(next) = self.next.as_ref() {
            write!(f, "{next}")?;
        }
        Ok(())
    }
}
