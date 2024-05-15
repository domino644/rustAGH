struct Introduction<'a> {
    intro: &'a str,
}
impl<'a> Introduction<'a> {
    fn print(&self) {
        println!("{}", self.intro);
    }
}
fn get_sample_text() -> &'static str {
    "Just a sample text"
}

fn main() {
    let text =
        String::from("Introduction to a long text. The rest of long text with many sentences.");

    let intro = text
        .split('.')
        .next()
        .expect("Could not find a first sentence.");

    let _i = Introduction { intro };
}
