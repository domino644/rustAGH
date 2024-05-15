use anyhow::anyhow;

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> anyhow::Result<Rectangle> {
        if (width <= 0.0 || height <= 0.0) {
            anyhow::bail!("Rectangle cannot have negative width or height");
        }
        Ok(Rectangle { width, height })
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn bigger(w1: f64, h1: f64, w2: f64, h2: f64) -> anyhow::Result<Rectangle> {
    let r1 = Rectangle::new(w1, h1)?;
    let r2 = Rectangle::new(w2, h2)?;

    if r1.area() > r2.area() {
        Ok(r1)
    } else {
        Ok(r2)
    }
}

fn read_from_file(path: &str) -> anyhow::Result<Rectangle> {
    let s = std::fs::read_to_string(path)?;
    let mut iter = s.split_whitespace();
    let width: f64 = iter
        .next()
        .ok_or(anyhow!("Cannot convert string to width"))?
        .parse()?;
    let height: f64 = iter
        .next()
        .ok_or(anyhow!("Cannot convert string to height"))?
        .parse()?;

    Ok(Rectangle::new(width, height)?)
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
        let r = Rectangle::new(width, height).unwrap();

        // then
        assert!((r.width - width).abs() < f64::EPSILON && (r.height - height).abs() < f64::EPSILON);
    }

    #[test]
    fn test_new_rectangle_with_negative() {
        let r = Rectangle::new(-1.0, 1.0);
        match r {
            Err(s) => assert_eq!(
                "Rectangle cannot have negative width or height",
                s.to_string()
            ),
            Ok(_) => panic!(), // the result shouldn't be Ok
        }
    }

    #[test]
    fn test_bigger() {
        // given
        let w1 = 1.0;
        let h1 = 2.0;
        let w2 = 3.0;
        let h2 = 4.0;

        // when
        let r = bigger(w1, h1, w2, h2);

        // then
        assert!((r.unwrap().area() - (w2 * h2)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bigger_err() {
        // given
        let w1 = 1.0;
        let h1 = -2.0; // wrong width
        let w2 = 3.0;
        let h2 = 4.0;

        // when
        let r = bigger(w1, h1, w2, h2);

        // then
        assert!(r.is_err());
    }
}
fn main() {}
