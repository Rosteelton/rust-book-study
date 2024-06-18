use crate::State::Draft;

pub struct Post {
    content: String,
    state: State,
}

#[derive(Clone)]
enum State {
    Draft,
    ReviewRequested,
    Approved,
}

impl State {
    pub fn review_request(&self) -> State {
        match self {
            Draft => State::ReviewRequested,
            _ => self.clone(),
        }
    }

    pub fn approve(&self) -> State {
        match self {
            State::ReviewRequested => State::Approved,
            _ => self.clone(),
        }
    }
}

impl Post {
    pub fn new() -> Post {
        Post {
            content: String::new(),
            state: Draft,
        }
    }

    pub fn add_text(&mut self, str: &str) -> () {
        self.content.push_str(str);
    }

    pub fn content(&self) -> &str {
        match self.state {
            State::Approved => &self.content,
            _ => "",
        }
    }

    pub fn request_review(&mut self) -> () {
        self.state = self.state.review_request()
    }

    pub fn approve(&mut self) -> () {
        self.state = self.state.approve()
    }
}
