use crate::core::commits::{get_commits_per_week, Commits, SumWeeklyCommits};
use crate::github::api::get_contributors;
use crate::github::contributors::serialize_contributors;
use std::fs;
use tui_input::Input;

pub enum InputMode {
    Normal,
    Editing,
}
pub struct App {
    pub author_blacklist: Vec<String>,
    pub commits: Option<Commits>,
    pub current_tick: u32,
    pub current_tick_authors: Option<Vec<(String, u32)>>,
    pub current_week: Option<u32>,
    pub error: Option<String>,
    pub input: Input,
    pub input_mode: InputMode,
    pub repository_url: String,
    pub should_load_repository: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new(author_blacklist: Vec<String>) -> Self {
        Self {
            author_blacklist,
            commits: None,
            current_tick: 0,
            current_tick_authors: None,
            current_week: None,
            error: None,
            input: Input::default(),
            input_mode: InputMode::Editing,
            repository_url: "".into(),
            should_load_repository: false,
            should_quit: false,
        }
    }

    pub fn on_tick(&mut self, total_ticks: u32) {
        if self.should_load_repository {
            self.load_repository_insights();
            self.should_load_repository = false;
        }
        if !self.should_load_repository
            && self.commits.is_some()
            && self.current_tick <= total_ticks
        {
            let sum_weekly = self.get_week_on_tick(self.current_tick, total_ticks);
            let authors = self.get_sorted_authors(sum_weekly.0);
            self.current_week = Some(*sum_weekly.1);
            self.current_tick_authors = Some(authors);
            self.current_tick += 1;
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn load_repository_insights(&mut self) {
        match get_contributors(
            format!(
                "https://github.com/{}/graphs/contributors-data",
                &self.repository_url.to_string()
            )
            .as_str(),
        ) {
            Ok(content) => {
                let contributors = serialize_contributors(content.as_str()).unwrap();
                let commits = get_commits_per_week(contributors, self.author_blacklist.clone());
                self.error = None;
                self.commits = Some(commits);
            }
            Err(e) => {
                self.input_mode = InputMode::Editing;
                self.error = Some(format!("{}", e));
            }
        }
    }

    pub fn load_repository_insights_from_json(&mut self, json_path_file: &str) {
        let file_content_result = fs::read_to_string(json_path_file);
        match file_content_result {
            Ok(file_content) => {
                let contributors = serialize_contributors(file_content.as_str()).unwrap();
                let commits = get_commits_per_week(contributors, self.author_blacklist.clone());
                self.error = None;
                self.commits = Some(commits);
                self.input_mode = InputMode::Normal;
            }
            Err(e) => {
                self.error = Some(format!("{}", e));
            }
        }
    }

    fn get_week_on_tick(&self, tick_count: u32, total_ticks: u32) -> (&SumWeeklyCommits, &u32) {
        let commits = self.commits.as_ref().unwrap();
        let weeks_per_tick = (commits.total_weeks as f64 / total_ticks as f64).ceil() as u32;
        let week_index = (tick_count * weeks_per_tick) as usize;
        let mut commits_keys: Vec<&u32> = commits.sum_commits.keys().collect();
        commits_keys.sort();
        if week_index >= commits_keys.len() {
            (
                commits
                    .sum_commits
                    .get(commits_keys.last().unwrap())
                    .unwrap(),
                commits_keys.last().unwrap(),
            )
        } else {
            let week = commits_keys.get(week_index).unwrap();
            (commits.sum_commits.get(week).unwrap(), week)
        }
    }

    fn get_sorted_authors(&self, sum_weekly_commits: &SumWeeklyCommits) -> Vec<(String, u32)> {
        let authors: Vec<(&String, &u32)> = sum_weekly_commits.authors.iter().collect();
        let mut sorted_authors: Vec<(String, u32)> =
            authors.iter().map(|(a, c)| (a.to_string(), **c)).collect();
        sorted_authors.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_authors
    }
}
