use std::{
    ops::{Add, AddAssign},
    process::Output,
};

// Napisz funkcję max, która zwróci największą wartość tablicy zawierającej dowolne typy liczbowe.
// Funkcja powinna zwracać Some(max) lub None w przypadku pustej tablicy.
fn max<T: Ord + Copy>(arr: &[T]) -> Option<T> {
    if arr.is_empty() {
        return None;
    }
    let mut max = arr[0];
    for val in arr {
        max = std::cmp::max(max, *val)
    }
    return Some(max);
}

// (*) Napisz funkcję mean, która wyznaczy średnią arytmentyczną elementów tablicy zawierającej dowolne typy liczbowe.
fn mean<T>(values: &[T]) -> f64
where
    T: Into<f64> + Copy,
{
    let sum: f64 = values.iter().map(|&x| x.into()).sum();
    let count = values.len() as f64;
    sum / count
}

struct Pair<T: Add> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Pair<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {}
