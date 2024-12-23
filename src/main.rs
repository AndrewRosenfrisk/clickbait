use rand::Rng;
use std::{fmt::Write, io::stdin};

fn main() -> std::fmt::Result {
    let templates: Vec<Box<dyn Template>> = vec![
        Box::new(WordTemplate {
            values: (
                "{noun}".to_owned(),
                vec![
                    "Athlete".to_owned(),
                    "Clown".to_owned(),
                    "Shovel".to_owned(),
                    "Paleo Diet".to_owned(),
                    "Doctor".to_owned(),
                    "Parent".to_owned(),
                    "Cat".to_owned(),
                    "Dog".to_owned(),
                    "Chicken".to_owned(),
                    "Robot".to_owned(),
                    "Video Game".to_owned(),
                    "Avocado".to_owned(),
                    "Plastic Straw".to_owned(),
                    "Serial Killer".to_owned(),
                    "Telephone Psychic".to_owned(),
                ],
            ),
        }),
        Box::new(WordTemplate {
            values: (
                "{objpro}".to_owned(),
                vec!["Her".to_owned(), "Him".to_owned(), "Them".to_owned()],
            ),
        }),
        Box::new(WordTemplate {
            values: (
                "{pospro}".to_owned(),
                vec!["Her".to_owned(), "His".to_owned(), "Their".to_owned()],
            ),
        }),
        Box::new(WordTemplate {
            values: (
                "{perpro}".to_owned(),
                vec![
                    "She Was".to_owned(),
                    "He Was".to_owned(),
                    "They Were".to_owned(),
                ],
            ),
        }),
        Box::new(WordTemplate {
            values: (
                "{state}".to_owned(),
                vec![
                    "California".to_owned(),
                    "Texas".to_owned(),
                    "Florida".to_owned(),
                    "New York".to_owned(),
                    "Pennsylvania".to_owned(),
                    "Illinois".to_owned(),
                    "Ohio".to_owned(),
                    "Georgia".to_owned(),
                    "North Carolina".to_owned(),
                    "Michigan".to_owned(),
                ],
            ),
        }),
        Box::new(WordTemplate {
            values: (
                "{place}".to_owned(),
                vec![
                    "House".to_owned(),
                    "Attic".to_owned(),
                    "Bank Deposit Box".to_owned(),
                    "School".to_owned(),
                    "Basement".to_owned(),
                    "Workplace".to_owned(),
                    "Donut Shop".to_owned(),
                    "Apocalypse Bunker".to_owned(),
                ],
            ),
        }),
        Box::new(WordTemplate {
            values: (
                "{when}".to_owned(),
                vec![
                    "Soon".to_owned(),
                    "This Year".to_owned(),
                    "Later Today".to_owned(),
                    "RIGHT NOW".to_owned(),
                    "Next Week".to_owned(),
                ],
            ),
        }),
        Box::new(NumberTemplate {
            values: ("{num}".to_owned(), 7, 15),
        }),
        Box::new(RangeTemplate {
            values: ("{range}".to_owned(), 3, 19),
        }),
        Box::new(RangeValueTemplate {
            values: ("{range_value}".to_owned(), 1),
        }),
    ];

    let headlines = vec![
        "Are Millenials Killing The {noun} Industry?".to_owned(),
        "Without This {noun}, {noun}s Could Kill you {when}".to_owned(),
        "Big Companies Hate {objpro}! See How This {state} {noun} Invented a Cheaper {noun}"
            .to_owned(),
        "You Won't Believe What This {state} {noun} Found in {pospro} {place}".to_owned(),
        "What {noun}s Don't Want You To Know About {noun}s".to_owned(),
        "{num} Gift Ideas to Give Your {noun} From {state}".to_owned(),
        "{range} Reasons Why {noun}s Are More Interesting Than You Think (Number {range_value} Will Suprise You!)".to_owned(),
        "This {state} {noun} Didn't Think Robots Would Take {pospro} Job. {perpro} Wrong."
            .to_owned(),
    ];

    println!("Our website needs to trick people into looking at ads!");
    println!("Enter the number of clickbait headlines to generate:");

    let mut choice: u32;
    'input: loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        choice = input.trim().parse::<u32>().unwrap_or(0);

        if choice == 0 {
            println!("Invalid entry. Please enter a positive whole number and try again:");
            continue;
        } else {
            break 'input;
        }
    }

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    for _ in 0..choice {
        let index = rng.gen_range(0..headlines.len());
        let mut working_headline = headlines[index].clone();
        let mut buffer = String::new();
        let mut range_value = None;

        for template in &templates {
            let pattern = template.get_pattern();

            if working_headline.contains(&pattern) {
                for el in working_headline.split_inclusive(&pattern) {
                    if el.contains(&pattern) {
                        let chosen_temp_value = template.get_replacement_value(range_value);
                        if template.get_pattern() == "{range}".to_owned() {
                            range_value = Some(chosen_temp_value.parse::<u8>().unwrap_or(1));
                        }
                        write!(&mut buffer, "{}", el.replace(&pattern, &chosen_temp_value))?;
                    } else {
                        write!(&mut buffer, "{}", el)?;
                    }
                }
            }
            if !buffer.is_empty() {
                working_headline = buffer.clone();
                buffer.clear();
            }
        }
        println!("{}", working_headline);
    }

    let website = vec![
        "wobsite",
        "blag",
        "Facebuuk",
        "Googles",
        "Facesbook",
        "Tweedie",
        "Pastagram",
    ][rng.gen_range(0..6)];

    let when = templates[6].get_replacement_value(None);

    println!("\nPost these to our {} {} or you're fired!", website, when);

    Ok(())
}
struct WordTemplate {
    values: (String, Vec<String>),
}
impl Template for WordTemplate {
    fn get_replacement_value(&self, _upper_bound: Option<u8>) -> String {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let length = self.values.1.len();
        self.values.1[rng.gen_range(0..length)].clone()
    }
    fn get_pattern(&self) -> String {
        self.values.0.clone()
    }
}
struct NumberTemplate {
    values: (String, u8, u8),
}
impl Template for NumberTemplate {
    fn get_replacement_value(&self, _upper_bound: Option<u8>) -> String {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        rng.gen_range(self.values.1..self.values.2).to_string()
    }
    fn get_pattern(&self) -> String {
        self.values.0.clone()
    }
}
struct RangeTemplate {
    values: (String, u8, u8),
}
impl Template for RangeTemplate {
    fn get_replacement_value(&self, _upper_bound: Option<u8>) -> String {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        rng.gen_range(self.values.1..self.values.2).to_string()
    }
    fn get_pattern(&self) -> String {
        self.values.0.clone()
    }
}
struct RangeValueTemplate {
    values: (String, u8),
}
impl Template for RangeValueTemplate {
    fn get_replacement_value(&self, upper_bound: Option<u8>) -> String {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        rng.gen_range(self.values.1..upper_bound.unwrap_or(1))
            .to_string()
    }
    fn get_pattern(&self) -> String {
        self.values.0.clone()
    }
}
trait Template {
    fn get_replacement_value(&self, upper_bound: Option<u8>) -> String;
    fn get_pattern(&self) -> String;
}
