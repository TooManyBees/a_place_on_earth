extern crate rand;
use rand::seq::SliceRandom;
use chrono::{Datelike, Weekday};

fn today() -> &'static str {
    match chrono::Local::today().weekday() {
        Weekday::Sun => "Sunday",
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
    }
}

fn random_declaration(join: bool) -> String {
    const STATEMENTS: &[(Option<&'static str>, &'static str)] = &[
        (Some("it's a"), "good day"),
        (Some("it"), "feels like a good day"),
        (Some("it's a"), "great day"),
        (Some("it"), "sure is a fine day"),
        (None, "I think it's a day"),
    ];
    let mut rng = rand::thread_rng();
    STATEMENTS.choose(&mut rng).map(|&pair| {
        if join {
            if let Some(s) = pair.0 {
                format!("{} {}", s, pair.1)
            } else {
                pair.1.into()
            }
        } else {
            pair.1.into()
        }
    }).unwrap()
}

fn random_question() -> &'static str {
    const QUESTIONS: &[&'static str] = &[
        "Is now a good time",
        "Is today a good day",
        "Think it's a good day",
        "Do you think it's a good day",
    ];
    let mut rng = rand::thread_rng();
    *QUESTIONS.choose(&mut rng).unwrap()
}

fn random_action() -> String {
    const ACTIONS: &[&'static str] = &[
//        "to be singing %S in your head",
        "to get %S stuck in your head",
        "to have %S stuck in your head",
        "for getting %S stuck in your head",
        "for %S to be stuck in your head",
    ];
    const SONGS: &[(&'static str, u8)] = &[
        ("'Heaven Is A Place On Earth'", 3),
        ("Belinda Carlisle's 'Heaven Is A Place On Earth'", 1),
//        ("'Heaven Is A Place On Earth'", 1),
    ];
    let mut rng = rand::thread_rng();
    let action = ACTIONS.choose(&mut rng).unwrap();
    let song = SONGS
        .choose_weighted(&mut rng, |pair| pair.1)
        .map(|pair| pair.0)
        .unwrap();
    action.replace("%S", song)
}

fn random_incidental() -> String {
    const INCIDENTALS: &[&'static str] = &[
        "Well it's %D, so",
        "Well it's a %D, so",
        "Is it %D? Then",
    ];
    let mut rng = rand::thread_rng();
    INCIDENTALS.choose(&mut rng).unwrap().replace("%D", &today())
}

fn random_answer(join: bool) -> String {
    const ANSWERS: &[(Option<&'static str>, &'static str)] = &[
        (None, "I think so."),
        (None, "yeah."),
        (None, "yup!"),
        (None, "heck yeah."),
        (Some("it"), "feels like it."),
        (Some("it"), "sure feels like it."),
        (None, "today is THE perfect day."),
    ];
    let mut rng = rand::thread_rng();
    ANSWERS
        .choose(&mut rng)
        .map(|pair| {
            if join {
                if let Some(s) = pair.0 {
                    format!("{} {}", s, pair.1)
                } else {
                    String::from(pair.1)
                }
            } else {
                String::from(pair.1)
            }
        })
        .unwrap()
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_ascii_uppercase().to_string() + c.as_str(),
    }
}

const VARIANTS: &[fn() -> String] = &[
    || {
        format!(
            "It's {day}, so {declaration} {action}.",
            day = today(),
            declaration = random_declaration(true),
            action = random_action()
        )
    },
    || {
        format!(
            "{day}? {declaration} {action}.",
            day = today(),
            declaration = capitalize(&random_declaration(false)),
            action = random_action()
        )
    },
    || {
        format!(
            "{day} again? {declaration} {action}.",
            day = today(),
            declaration = capitalize(&random_declaration(false)),
            action = random_action()
        )
    },
    || {
        format!(
            "{question} {action}? {answer}",
            question = random_question(),
            action = random_action(),
            answer = capitalize(&random_answer(false))
        )
    },
    || {
        format!(
            "{question} {action}? {incidental} {answer}",
            question = random_question(),
            action = random_action(),
            incidental = random_incidental(),
            answer = random_answer(true)
        )
    },
];

pub fn random_post() -> String {
    let mut rng = rand::thread_rng();
    VARIANTS.choose(&mut rng).unwrap()()
}
