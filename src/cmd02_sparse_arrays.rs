pub fn matchingStrings(stringList: &[String], queries: &[String]) -> Vec<i32> {
    queries.iter().map(|q| {
        stringList.iter().filter(|s| **s == *q).count() as i32
    }).collect()
}