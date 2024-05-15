struct Rectangle {
    width : f64,
    height : f64
}
 
impl Rectangle {
    fn new(width : f64, height : f64) -> Rectangle {
        Rectangle{width, height}
    }
 
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
 
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test_new_rectangle() {
        // given
        let width = 4.5;
        let height = 5.7;
 
        // when
        let r = Rectangle::new(width, height);
 
        // then
        assert!((r.width - width).abs() < f64::EPSILON && (r.height - height).abs() < f64::EPSILON);
    }
}
 
fn main() {
    // nothing is here
}