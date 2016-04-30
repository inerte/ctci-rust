use std::collections::HashSet;
use std::collections::LinkedList;

fn remove_duplicate(list: &LinkedList<u32>) -> LinkedList<u32> {
    let mut data = HashSet::new();
    let mut new_list = LinkedList::new();

    for node in list {
        if !data.contains(node) {
            new_list.push_back(*node);
        }
        data.insert(node);
    }

    return new_list;
}

#[test]
fn single_element_list_should_not_remove_anything() {
    let mut list = LinkedList::new();
    list.push_back(1);
    let unique_list = remove_duplicate(&list);

    assert_eq!(unique_list.len(), 1);
}

#[test]
fn two_element_list_with_same_data_should_uniquefy_to_one_element() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(1);
    let unique_list = remove_duplicate(&list);

    assert_eq!(unique_list.len(), 1);
}

#[test]
fn three_element_list_with_duplicate_data_should_uniquefy() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(1);
    let mut unique_list = remove_duplicate(&list);

    assert_eq!(unique_list.len(), 2);
    let last_element_a = unique_list.pop_back().unwrap();
    assert_eq!(last_element_a, 2);
    let last_element_b = unique_list.pop_back().unwrap();
    assert_eq!(last_element_b, 1);
}
