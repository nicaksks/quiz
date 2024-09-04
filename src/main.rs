use rand::Rng;
use serde::Deserialize;
use std::{fs, time::Duration};

#[derive(Debug, Deserialize)]
struct ParseQuestions {
    questions: Vec<Questions>,
}

#[derive(Debug, Clone, Deserialize)]
struct Questions {
    question: String,
    answers: Vec<String>,
}

struct Quiz;
impl Quiz {
    fn question(&self) -> Questions {
        self.random()
    }

    fn random(&self) -> Questions {
        let json = self.read_json().unwrap();
        let questions = json.questions;
        let number = rand::thread_rng().gen_range(0..questions.len());
        questions[number].clone()
    }

    fn read_json(&self) -> Result<ParseQuestions, ()> {
        if let Ok(json_file) = fs::read_to_string("./src/questions.json") {
            let json: ParseQuestions = serde_json::from_str(&json_file).unwrap();
            Ok(json)
        } else {
            panic!("Err -> File not found.")
        }
    }
}

struct Database {
    correct: u64,
    incorrect: u64,
}

impl Database {
    fn get_correct(&self) -> u64 {
        self.correct
    }

    fn set_correct(&mut self) {
        self.correct += 1
    }

    fn get_incorrect(&self) -> u64 {
        self.incorrect
    }

    fn set_incorrect(&mut self) {
        self.incorrect += 1
    }
}

const RULES: [&str; 3] = [
    "Responda as perguntas de forma corretas.",
    "Caso a resposta da pergunta esteja incorreta, uma nova pergunta vai aparecer dentro de 5 segundos.",
    "Cada resposta correta vale 1 ponto",
];

#[allow(unused_must_use)]
fn clear_terminal() {
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .arg("/c")
            .arg("cls")
            .status();
    } else {
        std::process::Command::new("clear").status();
    }
}

#[tokio::main]
async fn main() {
    let mut db = Database {
        correct: 0,
        incorrect: 0,
    };

    println!("~ ~ REGRAS ~ ~ \n{}\n", RULES.join("\n"));

    loop {
        let Questions { question, answers } = Quiz.question();
        println!("~ ~ PERGUNTA ~ ~ \n{}\n", question);

        let mut answer_personal = String::new();
        println!("~ ~ RESPOSTA ~ ~ {}", answer_personal);
        std::io::stdin().read_line(&mut answer_personal).unwrap();

        let correct_answer = answers
            .into_iter()
            .find(|x| x.to_lowercase() == answer_personal.trim().to_lowercase());

        if correct_answer.is_some() {
            db.set_correct();
            println!("\n ~ ~ Você acertou a resposta! ~ ~");
            println!("ദ്ദി(｡•̀ ,<)~✩‧₊ ── °🥂⋆.ೃ🪩*• ── `⎚⩊⎚´ -✧")
        } else {
            db.set_incorrect();
            println!("\n ~ ~ Resposta incorreta! ~ ~");
            println!("(っ◞‸◟ c) ── °🥂⋆.ೃ🪩*• ── ｡°(°.◜ᯅ◝°)°｡")
        }

        println!("\n˚ ༘ ೀ⋆｡˚ִֶָ𓂃 ࣪˖ ִֶָPontuação atual ་༘࿐. ܁₊ ⊹ . ܁˖ . ܁");
        println!("╰┈➤ {} Acertos", db.get_correct());
        println!("╰┈➤ {} Erros", db.get_incorrect());

        println!("\n ╰› Uma nova pergunta vai aparecer dentro de 5 segundos ⤸");

        tokio::time::sleep(Duration::from_secs(5)).await;
        clear_terminal()
    }
}
