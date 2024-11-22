pub fn sortedSubsegments(k: i32, a: &[i32], queries: &[Vec<i32>]) -> i32 {
    let mut v = a.to_vec();
    let mut l: &mut [i32] = &mut v;

    for query in queries {
        &l[query[0] as usize..query[1] as usize + 1].sort();
    }

    l[k as usize]
}