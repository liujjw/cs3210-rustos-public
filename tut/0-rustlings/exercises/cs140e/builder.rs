// FIXME: Make me pass! Diff budget: 30 lines.

#[derive(Default)]
struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn string<S: AsRef<str>>(&mut self, s: S) -> &mut Self {
        self.string = Some(s.as_ref().to_owned());
        self
    }

    fn number(&mut self, n: usize) -> &mut Self {
        self.number = Some(n);
        self
    }
}

impl ToString for Builder {
    fn to_string(&self) -> String {
        if (self.number.is_none()) {
            (self.string.as_ref().unwrap_or(&"".to_string())).to_owned()
        } else if (self.string.is_none()) {
            self.number.unwrap().to_string()
        } else {
            self.string.as_ref().unwrap().to_owned() + " " + 
                &self.number.unwrap().to_string()
        }
    }
}

// Do not modify this function.
#[test]
fn builder() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default().string("heap!".to_owned()).to_string();

    assert_eq!(c, "heap!");
}
