extern crate clipboard;

use clipboard::{ClipboardContext, ClipboardProvider};
use std::time::Duration;
use strsim::levenshtein;

struct QnA {
    question: String,
    answer: String,
}

fn find_closest_match<'a>(input: &str, qna_vec: &'a Vec<QnA>) -> Option<&'a QnA> {
    let mut closest_match: Option<&QnA> = None;
    let mut min_distance = usize::max_value();

    for qna in qna_vec {
        let distance = levenshtein(input, &qna.question);
        if distance < min_distance {
            min_distance = distance;
            closest_match = Some(qna);
        }
    }

    closest_match
}

fn is_answer_in_qna_list(answer: &str, qna_list: &Vec<QnA>) -> bool {
    for qna in qna_list {
        if qna.answer == answer {
            return true; // The answer is in the qna_list
        }
    }
    false // The answer is not in the qna_list
}

fn main() {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut current_contents = clipboard.get_contents().unwrap();
    println!("Current clipboard contents: {}", current_contents);

    loop {
        let new_contents = clipboard.get_contents().unwrap();
        if new_contents != current_contents {
            // The clipboard has changed. Handle the change here.
            println!("Clipboard contents changed: {}", new_contents);
            current_contents = new_contents;
            let qna_list = vec![
                QnA {
                    question: "What’s the third letter of the alphabet".to_string(),
                    answer: "c".to_string(),
                },
                QnA {
                    question: "What’s frozen water".to_string(),
                    answer: "ice".to_string(),
                },
                QnA {
                    question: "In what year did Garry’s mod released on steam".to_string(),
                    answer: "2004".to_string(),
                },
                QnA {
                    question: "I can be the sun, a bird, or simply sand. What am i".to_string(),
                    answer: "clock ".to_string(),
                },
                QnA {
                    question: "What’s the number that come after 8425".to_string(),
                    answer: "8426".to_string(),
                },
                QnA {
                    question: "What’s the first name of Mario".to_string(),
                    answer: "mario".to_string(),
                },
                QnA {
                    question: "How many orange is there is MU_CLUE".to_string(),
                    answer: "3".to_string(),
                },
                QnA {
                    question: "How many seasons exist".to_string(),
                    answer: "4".to_string(),
                },
                QnA {
                    question: "Who’s the Owner of the server".to_string(),
                    answer: "dimentio".to_string(),
                },
                QnA {
                    question: "Say capybara".to_string(),
                    answer: "capybara".to_string(),
                },
                QnA {
                    question: "In which game, do you follow the story of friends who are trapped inside a cursed School".to_string(),
                    answer: "corpse party".to_string(),
                },
                QnA {
                    question: "How many achivements are in the server".to_string(),
                    answer: "55".to_string(),
                },
                QnA {
                    question: "How many gong are there in MU_CLUE".to_string(),
                    answer: "4".to_string(),
                },
                QnA {
                    question: "In which year Quebec Games was created".to_string(),
                    answer: "2015".to_string(),
                },
                QnA {
                    question: "How many color is there in a rainbow".to_string(),
                    answer: "7".to_string(),
                },
                QnA {
                    question: "What is a dakimakura?".to_string(),
                    answer: "body pillow".to_string(),
                },
                QnA {
                    question: "What’s the number that come before 8425".to_string(),
                    answer: "8424 ".to_string(),
                },
                QnA {
                    question: "What’s the universal answer".to_string(),
                    answer: "42".to_string(),
                },
                QnA {
                    question: "What’s the capital of Australia".to_string(),
                    answer: "camberra".to_string(),
                },
                QnA {
                    question: "What does http stand for".to_string(),
                    answer: "hypertext transfer protocol".to_string(),
                },
                QnA {
                    question: "The lost code".to_string(),
                    answer: "4 8 15 16 23 42".to_string(),
                },
                QnA {
                    question: "Who’s the protagonist of the legend of zelda".to_string(),
                    answer: "link".to_string(),
                },
                QnA {
                    question: "What nintendo was selling before videogames".to_string(),
                    answer: "cards".to_string(),
                },
                QnA {
                    question: "What are the true antagonist in luigi’s mansion".to_string(),
                    answer: "boo".to_string(),
                },
                QnA {
                    question: "How many stars are they in Mario 64".to_string(),
                    answer: "120".to_string(),
                },
                QnA {
                    question: "How many stars are they in Mario 64 DS".to_string(),
                    answer: "150".to_string(),
                },
                QnA {
                    question: "Which perk give you more health in COD Zombie".to_string(),
                    answer: "juggernog".to_string(),
                },
                QnA {
                    question: "Who is the editor of Link : The Face Of Evil ?".to_string(),
                    answer: "philips".to_string(),
                },
                QnA {
                    question: "Who is the true antagonist in Resident Evil 5".to_string(),
                    answer: "albert wesker".to_string(),
                },
                QnA {
                    question: "Say my name".to_string(),
                    answer: "heisenberg ".to_string(),
                },
                QnA {
                    question: "Do chu ffink i’m cuwute UwU".to_string(),
                    answer: "yeah".to_string(),
                },
                QnA {
                    question: "What’s the forbidden play but also a masterpiece".to_string(),
                    // TODO: yo
                    answer: "UNKNOWN".to_string(),
                },
                QnA {
                    question: "How many Pokèmon does Generation 1 feature".to_string(),
                    answer: "151".to_string(),
                },
                QnA {
                    question: "How many protagonists are in GTA 5".to_string(),
                    answer: "3".to_string(),
                },
                QnA {
                    question: "What is the currency of Roblox".to_string(),
                    answer: "robux".to_string(),
                },
                QnA {
                    question: "How many unique Moons in total can you collect in Mario Odyssey".to_string(),
                    answer: "880".to_string(),
                },
                QnA {
                    question: "Who’s the Author of One Piece".to_string(),
                    answer: "eiichiro oda".to_string(),
                },
                QnA {
                    question: "Which city is always saved by Batman".to_string(),
                    answer: "gotham city".to_string(),
                },
                QnA {
                    question: "Who’s Batman’s Butler".to_string(),
                    answer: "alfred pennyworth".to_string(),
                },
                QnA {
                    question: "What is the nearest planet to the sun".to_string(),
                    answer: "mercury".to_string(),
                },
                QnA {
                    question: "How many elements are there in the periodic table".to_string(),
                    answer: "118".to_string(),
                },
                QnA {
                    question: "Oncology focuses on what disease".to_string(),
                    answer: "cancer".to_string(),
                },
                QnA {
                    question: "What does the letter E represent in E=MC".to_string(),
                    answer: "energy".to_string(),
                },
                QnA {
                    question: "Mycology is the scientific study of what".to_string(),
                    answer: "fungi".to_string(),
                },
                QnA {
                    question: "Which is the intruder in this list that contains elements from the periodic table : americium / francium / mexicanium / germanium".to_string(),
                    answer: "mexicanium".to_string(),
                },
                QnA {
                    question: "What element is a diamond composed of".to_string(),
                    answer: "carbon".to_string(),
                },
                QnA {
                    question: "COD is an acronym of which popular video game series".to_string(),
                    answer: "call of duty".to_string(),
                },
                QnA {
                    question: "In Uncharted 2, how many hidden items you can collect in the game".to_string(),
                    answer: "101".to_string(),
                },
                QnA {
                    question: "What color is pacman".to_string(),
                    answer: "yellow".to_string(),
                },
                QnA {
                    question: "What is the agent number given to the main character in Hitman".to_string(),
                    answer: "47".to_string(),
                },
                QnA {
                    question: "What color is the A button on a standard N64 controller".to_string(),
                    answer: "blue".to_string(),
                },
                QnA {
                    question: "What color is the B button on a standard N64 controller".to_string(),
                    answer: "green".to_string(),
                },
                QnA {
                    question: "What is the other name of the villain Dr. Robotnik".to_string(),
                    answer: "eggman".to_string(),
                },
                QnA {
                    question: "In which game serie can you find Raccoon City".to_string(),
                    answer: "resident evil".to_string(),
                },
                QnA {
                    question: "What is the name of the developers of the ‘Halo’ series".to_string(),
                    answer: "bungie".to_string(),
                },
                QnA {
                    question: "How many players can simultaneously play in Super Mario Bros Wii".to_string(),
                    answer: "4".to_string(),
                },
                QnA {
                    question: "How many players can simultaneously play in Super Mario Bros Wii U".to_string(),
                    answer: "5".to_string(),
                },
                QnA {
                    question: "What’s the fastest land animal".to_string(),
                    answer: "cheetah".to_string(),
                },
                QnA {
                    question: "What is the most consumed manufactured drink in the world".to_string(),
                    answer: "tea".to_string(),
                },
                QnA {
                    question: "Dump, floater, and wipe are terms used in which team sport".to_string(),
                    answer: "volleyball".to_string(),
                },
                QnA {
                    question: "What’s the largest bone in the human body".to_string(),
                    answer: "femur".to_string(),
                },
                QnA {
                    question: "How many bones do sharks have".to_string(),
                    answer: "0".to_string(),
                },
                QnA {
                    question: "How many oceans are there on Earth".to_string(),
                    answer: "5".to_string(),
                },
                QnA {
                    question: "What is pi, written out to 5 decimal places".to_string(),
                    answer: "3.14159".to_string(),
                },
                QnA {
                    question: "What color is chlorophyll ?".to_string(),
                    answer: "green".to_string(),
                },
                QnA {
                    question: "What is the hottest planet in our solar system".to_string(),
                    answer: "venus".to_string(),
                },
                QnA {
                    question: "Is there a blue Teletubby".to_string(),
                    answer: "no".to_string(),
                },
                QnA {
                    question: "In Romeo and Juliet what family is Juliet from".to_string(),
                    answer: "capulet".to_string(),
                },
                QnA {
                    question: "What’s the top number on a fraction called".to_string(),
                    answer: "numerator".to_string(),
                },
                QnA {
                    question: "In what horror film does a guy on a motorcycle smash into an invisible artificial wall".to_string(),
                    answer: "cabin in the woods".to_string(),
                },
                QnA {
                    question: "If you drop me I’m sure to crack, but give me a smile and I’ll always smile back. What am I".to_string(),
                    answer: "mirror".to_string(),
                },
                QnA {
                    question: "The more you take, the more you leave behind. What are they".to_string(),
                    answer: "footsteps".to_string(),
                },
                QnA {
                    question: "What has a head and a tail but no body".to_string(),
                    answer: "coin".to_string(),
                },
                QnA {
                    question: "How many question do you have to answer".to_string(),
                    answer: "11".to_string(),
                },
                QnA {
                    question: "What has many teeth, but can’t bite".to_string(),
                    answer: "comb".to_string(),
                },
                QnA {
                    question: "It belongs to you, but other people use it more than you do. What is it".to_string(),
                    answer: "your name".to_string(),
                },
                QnA {
                    question: "I follow you all the time and copy your every move, but you can’t touch me or catch me. What am I".to_string(),
                    answer: "your shadow".to_string(),
                },
                QnA {
                    question: "What can you break, even if you never pick it up or touch it".to_string(),
                    answer: "promise".to_string(),
                },
                QnA {
                    question: "What country does anime come from".to_string(),
                    answer: "japan ".to_string(),
                },
                QnA {
                    question: "How many types of Pokémon are there".to_string(),
                    answer: "18".to_string(),
                },
            ];

            if is_answer_in_qna_list(&current_contents, &qna_list) {
                println!("The answer is in the qna_list");
            } else {
                match find_closest_match(current_contents.as_str(), &qna_list) {
                    Some(qna) => {
                        println!(
                            "Closest match: Question: {} Answer: {}",
                            qna.question, qna.answer
                        );
                        clipboard.set_contents(qna.answer.clone()).unwrap();
                    }
                    None => {
                        println!("No matching question found.");
                    }
                }
            }
        }

        // Poll the clipboard at regular intervals (e.g., every 2 seconds).
        std::thread::sleep(Duration::from_secs(2));
    }
}
