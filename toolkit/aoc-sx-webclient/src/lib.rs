use std::time::Duration;

use aoc_sx_core::exercise::{ExerciseDay, ExercisePart, ExerciseYear};
use cookie_store::CookieStore;
use scraper::{Html, Selector};
use ureq::{Agent, AgentBuilder, Cookie};
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Network error: {0}")]
    NetworkError(String),
}

const TIMEOUT_DURATION: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub struct Client {
    agent: Agent,
}

impl Client {
    pub fn new(session_token: String) -> Self {
        let cookie = Cookie::build(("session", session_token))
            .domain(".adventofcode.com")
            .build();
        let mut store = CookieStore::default();
        let url = Url::try_from("https://adventofcode.com").unwrap();
        store.insert_raw(&cookie, &url).unwrap();

        let agent = AgentBuilder::new()
            .timeout_read(TIMEOUT_DURATION)
            .timeout_write(TIMEOUT_DURATION)
            .cookie_store(store)
            .build();

        Self { agent }
    }

    fn exercise_page_to_url(&self, year: ExerciseYear, day: ExerciseDay) -> Url {
        Url::try_from(&format!("https://adventofcode.com/{year}/day/{day}")[..]).unwrap()
    }

    pub fn send_answer(
        &self,
        answer: &str,
        year: ExerciseYear,
        day: ExerciseDay,
        part: ExercisePart,
    ) -> Result<PuzzleAnswer, Error> {
        let response = self
            .agent
            .post(self.get_exercise_answer_url(year, day).as_str())
            .send_form(&[("level", part.as_level()), ("answer", answer)])
            .map_err(|e| Error::NetworkError(e.to_string()))?;

        let body = response.into_string().unwrap();
        let document = Html::parse_document(&body);
        let selector = Selector::parse("article").unwrap();
        let node = document.root_element().select(&selector).next().unwrap();
        let node_text = node.text().collect::<String>();

        if node_text.contains("You gave an answer too recently") {
            Ok(PuzzleAnswer::TooManyRequests(node_text))
        } else if node_text.contains("That's not the right answer") {
            Ok(PuzzleAnswer::Failed(node_text))
        } else {
            Ok(PuzzleAnswer::Success(node_text))
        }
    }

    pub fn fetch_input_page(
        &self,
        year: ExerciseYear,
        day: ExerciseDay,
    ) -> Result<PuzzleInput, Error> {
        let input_url = self.get_exercise_input_url(year, day);

        let input_response = self
            .agent
            .get(input_url.as_str())
            .call()
            .map_err(|e| Error::NetworkError(e.to_string()))?;
        let input_body = input_response.into_string().unwrap();

        Ok(PuzzleInput(input_body))
    }

    pub fn fetch_exercise_page(
        &self,
        year: ExerciseYear,
        day: ExerciseDay,
    ) -> Result<ExercisePage, Error> {
        let page_url = self.exercise_page_to_url(year, day);
        let input_url = self.get_exercise_input_url(year, day);

        let page_response = self
            .agent
            .get(page_url.as_str())
            .call()
            .map_err(|e| Error::NetworkError(e.to_string()))?;
        let page_body = page_response.into_string().unwrap();

        let puzzle_input = self.fetch_input_page(year, day)?;
        Ok(ExercisePage {
            input_url,
            page_url,
            page_content: HtmlContent(page_body),
            puzzle_input,
        })
    }

    fn get_exercise_input_url(&self, year: ExerciseYear, day: ExerciseDay) -> Url {
        Url::try_from(&format!("{}/input", self.exercise_page_to_url(year, day))[..]).unwrap()
    }

    fn get_exercise_answer_url(&self, year: ExerciseYear, day: ExerciseDay) -> Url {
        Url::try_from(&format!("{}/answer", self.exercise_page_to_url(year, day))[..]).unwrap()
    }
}

#[derive(Debug)]
pub struct HtmlContent(String);

impl HtmlContent {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct PuzzleInput(String);

impl PuzzleInput {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct ExercisePage {
    pub page_url: Url,
    pub input_url: Url,
    pub page_content: HtmlContent,
    pub puzzle_input: PuzzleInput,
}

#[derive(Debug)]
pub enum PuzzleAnswer {
    Success(String),
    Failed(String),
    TooManyRequests(String),
}
