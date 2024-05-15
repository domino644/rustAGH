use Option::*;

// fn sum(x: Option<i32>, y: Option<i32>) -> Option<i32> {
//     match (x, y) {
//         // use tuple to avoid multiple matches
//         (Some(x), Some(y)) => Some(x + y),
//         _ => None,
//     }
// }

fn sum(x : Option<i32>, y : Option<i32>) -> Option<i32> {
    Some(x? + y?)
}

fn main() {
    assert_eq!(sum(Some(1), Some(1)), Some(2));
    assert_eq!(sum(Some(1), None), None);
    assert_eq!(sum(None, None), None);
    assert_eq!(sum(None, Some(2)), None);
}
