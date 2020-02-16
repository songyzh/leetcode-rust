use crate::util::vec::*;

fn insertion_sort(v: &mut Vec<i32>) {
    for i in 1..v.len() {
        let value = v[i];
        let mut j = i as i32 - 1;
        while j >= 0 && v[j as usize] > value {
            v[j as usize + 1] = v[j as usize];
            j -= 1;
        }
        v[(j + 1) as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut shuffled = get_shuffled_vec();
        insertion_sort(&mut shuffled);
        assert_eq!(get_sorted_vec(), shuffled);
    }
}
