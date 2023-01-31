#[derive(Debug, PartialEq)]
pub struct Pair
{
    pub range_1: Vec<u8>,
    pub range_2: Vec<u8>
}

pub trait OverlapCalculator
{
    fn full_overlap(&self) -> bool;

    fn partial_overlap(&self) -> bool;
}

impl OverlapCalculator for Pair {
    fn full_overlap(&self) -> bool {
        let fully_contained_within_range_2 = !self.range_1.iter().any(|x| !self.range_2.contains(&x));
        let fully_contained_within_range_1 = !self.range_2.iter().any(|x| !self.range_1.contains(&x));

        fully_contained_within_range_1 || fully_contained_within_range_2
    }

    fn partial_overlap(&self) -> bool {
        self.range_1.iter().any(|x| self.range_2.contains(&x))
    }
}

#[test]
fn full_overlap_calculator_should_return_no_overlap() {
    let pair = Pair {
        range_1: Vec::from([1, 2, 3]),
        range_2: Vec::from([4,5,6])
    };
    assert_eq!(false, pair.full_overlap());
}

#[test]
fn full_overlap_calculator_should_return_overlap() {
    let pair = Pair {
        range_1: Vec::from([1, 2, 3]),
        range_2: Vec::from([2, 3])
    };
    assert_eq!(true, pair.full_overlap());
}

#[test]
fn full_overlap_calculator_should_return_overlap_2() {
    let pair = Pair {
        range_1: Vec::from([4, 5]),
        range_2: Vec::from([1, 2, 3, 4, 5])
    };
    assert_eq!(true, pair.full_overlap());
}

#[test]
fn partial_overlap_calculator_should_return_no_overlap() {
    let pair = Pair {
        range_1: Vec::from([1, 2, 3]),
        range_2: Vec::from([4,5,6])
    };
    assert_eq!(false, pair.partial_overlap());
}

#[test]
fn partial_overlap_calculator_should_return_overlap() {
    let pair = Pair {
        range_1: Vec::from([1, 2, 3]),
        range_2: Vec::from([2, 3, 4, 5, 6])
    };
    assert_eq!(true, pair.partial_overlap());
}

#[test]
fn partial_overlap_calculator_should_return_overlap_2() {
    let pair = Pair {
        range_1: Vec::from([4, 5]),
        range_2: Vec::from([1, 2, 3, 4, 5])
    };
    assert_eq!(true, pair.partial_overlap());
}




