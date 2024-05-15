fn len_longer_array(a: &[i32], b: &[i32]) -> usize {
    if a.len() > b.len() {
        a.len()
    } else {
        b.len()
    }
}
fn longer_array<'a>(a: &'a [i32], b: &'a [i32]) -> &'a [i32] {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
fn main() {
    let tab1: [i32; 5] = [1, 2, 3, 4, 5];
    let tab2: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("{:?}", longer_array(&tab1, &tab2));
}
