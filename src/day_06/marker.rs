use std::collections::VecDeque;

pub fn calculate_marker(input: &str, amount_of_chars: usize) -> usize {
    let mut vec: VecDeque<char> = VecDeque::new();
    let mut index: usize = 0;
    for ch in input.chars().into_iter() {
        if vec.len() == amount_of_chars {
            if all_elements_different(&vec) {
                break;
            }
            vec.pop_front();
        }
        vec.push_back(ch);
        index += 1;
    }
    index
}

fn all_elements_different(input: &VecDeque<char>) -> bool {
    let mut buffer: String = "".to_string();
    for el in input.iter() {
        let element_cloned = el.clone();
        if buffer.contains(element_cloned) {
            return false;
        }
        buffer.push(element_cloned);
    }
    true
}

#[test]
fn all_elements_different_should_return_true() {
    let elements: VecDeque<char> = VecDeque::from(['j', 'i']);

    assert_eq!(true, all_elements_different(&elements));
}

#[test]
fn all_elements_different_should_return_false() {
    let elements: VecDeque<char> = VecDeque::from(['k', 'i', 'h', 'k']);

    assert_eq!(false, all_elements_different(&elements));
}

#[test]
fn calculate_marker_should_return_7() {
    let seq = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    assert_eq!(7, calculate_marker(seq, 4));
}

#[test]
fn calculate_marker_should_return_5() {
    let seq = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    assert_eq!(5, calculate_marker(seq, 4));
}

#[test]
fn calculate_marker_should_return_6() {
    let seq = "nppdvjthqldpwncqszvftbrmjlhg";

    assert_eq!(6, calculate_marker(seq, 4));
}

#[test]
fn calculate_marker_should_return_10() {
    let seq = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    assert_eq!(10, calculate_marker(seq, 4));
}

#[test]
fn calculate_marker_should_return_11() {
    let seq = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    assert_eq!(11, calculate_marker(seq, 4));
}

#[test]
fn calculate_marker_should_return_19() {
    let seq = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    assert_eq!(19, calculate_marker(seq, 14));
}

#[test]
fn calculate_marker_should_return_23() {
    let seq_1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let seq_2 = "nppdvjthqldpwncqszvftbrmjlhg";

    assert_eq!(23, calculate_marker(seq_1, 14));
    assert_eq!(23, calculate_marker(seq_2, 14));
}

#[test]
fn calculate_marker_should_return_29() {
    let seq = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    assert_eq!(29, calculate_marker(seq, 14));
}

#[test]
fn calculate_marker_should_return_26() {
    let seq = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    assert_eq!(26, calculate_marker(seq, 14));
}
