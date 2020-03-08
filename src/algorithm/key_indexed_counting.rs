use crate::util::vec::*;

fn key_indexed_counting_sort(v: Vec<usize>) -> Vec<usize> {
    let max = v.iter().max().unwrap();
    let mut count = vec![0; max + 2];
    // count
    for t in &v {
        count[t + 1] += 1;
    }
    // transform
    for i in 1..count.len() {
        count[i] += count[i - 1];
    }
    // distribute
    let mut aux = vec![0; v.len()];
    // reverse to keep the sort stable
    for i in 0..v.len() {
        aux[count[v[i]]] = v[i];
        count[v[i]] += 1;
    }
    return aux;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut shuffled = get_shuffled_vec().into_iter().map(|x| x as usize).collect();
        let sorted: Vec<i32> = key_indexed_counting_sort(shuffled)
            .into_iter()
            .map(|x| x as i32)
            .collect();
        assert_eq!(get_sorted_vec(), sorted);
    }
}
