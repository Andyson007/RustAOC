pub struct Reaction<'a> {
    inputs: Box<[(isize, &'a str)]>,
    outputs: Box<[(isize, &'a str)]>,
}

pub struct Reactions<'a> {
    reactions: Vec<Reaction<'a>>,
}

impl Reaction<'_> {
    pub fn solve(&self) {

    }
}
