pub fn find_common_items(item: String) -> Vec<char> {
    let (part_one, part_two) = item.split_at(item.len() / 2);

    let mut result: Vec<char> = Vec::new();
    for c in part_one.chars().into_iter() {
        if part_two.contains(c) && !result.contains(&c) {
            result.push(c);
        }
    }
    result
}

pub fn find_common_items_in_group(items: Vec<String>) -> Vec<char> {
    let common_chars_1 = find_common_items_between_2_strings(items[0].clone(), items[1].clone());
    let common_chars_2 = find_common_items_between_2_strings(items[0].clone(), items[2].clone());

    find_common_items_between_2_strings(common_chars_1, common_chars_2)
        .chars()
        .collect()
}

fn find_common_items_between_2_strings(item_1: String, item_2: String) -> String {
    let mut result: String = String::from("");
    for el_1 in item_1.chars() {
        for el_2 in item_2.chars() {
            if el_1 == el_2 && !result.contains(el_1) {
                result.push(el_1);
            }
        }
    }
    result
}

pub fn calculate_priority(item: &char) -> Option<u32> {
    match item.to_digit(36) {
        Some(r) => {
            let mut code = r - 9;
            if item.is_uppercase() {
                code += 26;
            }
            Some(code)
        }
        None => None,
    }
}

#[test]
fn find_common_items_should_find_char_a() {
    let input = "aBCaFG".to_string();
    let output = find_common_items(input);
    assert_eq!(&char::from('a'), output.first().unwrap());
}

#[test]
fn calculate_priority_should_calculate_for_lowercase() {
    assert_eq!(1, calculate_priority(&char::from('a')).unwrap());
    assert_eq!(2, calculate_priority(&char::from('b')).unwrap());
    assert_eq!(6, calculate_priority(&char::from('f')).unwrap());
    assert_eq!(25, calculate_priority(&char::from('y')).unwrap());
    assert_eq!(26, calculate_priority(&char::from('z')).unwrap());
}

#[test]
fn calculate_priority_should_calculate_for_uppercase() {
    assert_eq!(27, calculate_priority(&char::from('A')).unwrap());
    assert_eq!(28, calculate_priority(&char::from('B')).unwrap());
    assert_eq!(32, calculate_priority(&char::from('F')).unwrap());
    assert_eq!(51, calculate_priority(&char::from('Y')).unwrap());
    assert_eq!(52, calculate_priority(&char::from('Z')).unwrap());
}

#[test]
fn find_common_items_between_2_strings_should_find_a() {
    let string_1 = String::from("Aaxa");
    let string_2 = String::from("abcdf");
    assert_eq!(
        String::from("a"),
        find_common_items_between_2_strings(string_1, string_2)
    );
}
