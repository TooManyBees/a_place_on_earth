extern crate rand;
use rand::seq::SliceRandom;

const QUESTIONS: &[&'static str] = &[
    "Is now a good time",
    "Is today a good day",
    "Think it's a good day",
];

const ACTIONS: &[&'static str] = &[
    "to be singing %S",
    "to sing %S",
    "to get %S stuck in your head",
    "for %S to be stuck in your head",
];

const SONGS: &[&'static str] = &[
    "Heaven Is A Place On Earth",
];

const ANSWERS: &[&'static str] = &[
    "I think so",
    "Yeah",
    "Yup!",
    "Heck yeah",
    "Feels like it",
    "Sure feels like it",
];

const STATEMENTS: &[&'static str] = &[
    "Good day",
    "It's a good day",
    "Feels like a good day",
    "Great day",
    "Sure is a fine day",
];

pub fn random_statement() -> String {
    let mut rng = rand::thread_rng();
    let statement = STATEMENTS.choose(&mut rng).unwrap();
    let action = random_action();
    let mut r = String::from(*statement);
    r.push(' ');
    r.push_str(&action);
    r.push('.');
    r
}

pub fn random_question() -> String {
    let mut rng = rand::thread_rng();
    let question = QUESTIONS.choose(&mut rng).unwrap();
    let action = random_action();
    let answer = ANSWERS.choose(&mut rng).unwrap();
    let mut r = String::from(*question);
    r.push(' ');
    r.push_str(&action);
    r.push('?');
    r.push_str(&answer);
    r
}

fn random_action() -> String {
    let mut rng = rand::thread_rng();
    let action = ACTIONS.choose(&mut rng).unwrap();
    let song = SONGS.choose(&mut rng).unwrap();
    action.replace("%S", song)
}