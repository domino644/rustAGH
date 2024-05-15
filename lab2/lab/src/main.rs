fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut start: usize = 0;
    let mut end: usize = input.len();
    for (i, ch) in input.chars().enumerate() {
        if ch != ' ' {
            start = i;
            break;
        }
    }
    for i in (start..input.len()).rev() {
        if input.as_bytes()[i] != b' ' {
            end = i;
            break;
        }
    }
    if start <= end {
        return String::from(&input[start..end + 1]);
    }
    return String::from("");
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    return input.to_string() + " world!";
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars", "balloons")
}

fn main() {
    assert_eq!(trim_me("Hello!     "), "Hello!");
    assert_eq!(trim_me("  What's up!"), "What's up!");
    assert_eq!(trim_me("   Hola!  "), "Hola!");

    assert_eq!(compose_me("Hello"), "Hello world!");
    assert_eq!(compose_me("Goodbye"), "Goodbye world!");

    assert_eq!(
        replace_me("I think cars are cool"),
        "I think balloons are cool"
    );
    assert_eq!(
        replace_me("I love to look at cars"),
        "I love to look at balloons"
    );
}
