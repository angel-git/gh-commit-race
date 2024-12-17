use crate::github::contributors::Contributor;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Commits {
    pub first_week: u32,
    pub last_week: u32,
    pub total_weeks: u32,
    pub weekly_commits: HashMap<u32, WeeklyCommits>,
    pub sum_commits: HashMap<u32, SumWeeklyCommits>,
}

#[derive(Clone)]
pub struct WeeklyCommits {
    authors: HashMap<String, u32>,
}

#[derive(Clone)]
pub struct SumWeeklyCommits {
    pub authors: HashMap<String, u32>,
}

impl Commits {
    pub fn new() -> Self {
        Commits {
            first_week: 0,
            last_week: 0,
            total_weeks: 0,
            weekly_commits: HashMap::new(),
            sum_commits: HashMap::new(),
        }
    }
}

pub fn get_commits_per_week(contributors: Vec<Contributor>) -> Commits {
    let mut commits = Commits::new();

    for contributor in contributors {
        let mut weeks = contributor.weeks;
        weeks.sort_by_key(|w| w.w);
        commits.first_week = weeks.first().unwrap().w;
        commits.last_week = weeks.last().unwrap().w;
        commits.total_weeks = weeks.len() as u32;
        let author = contributor.author.login;

        for (i, week) in weeks.iter().enumerate() {
            commits
                .weekly_commits
                .entry(week.w)
                .or_insert_with(|| WeeklyCommits {
                    authors: HashMap::new(),
                })
                .authors
                .entry(author.clone())
                .and_modify(|c| *c = week.c)
                .or_insert(week.c);

            let sum_commit = if i == 0 {
                weeks[i].c
            } else {
                let current_week_commits = weeks[i].c;

                let previous_week_commits = commits
                    .sum_commits
                    .get(&weeks[i - 1].w)
                    .unwrap()
                    .authors
                    .get(&author)
                    .unwrap();

                current_week_commits + previous_week_commits
            };
            commits
                .sum_commits
                .entry(week.w)
                .or_insert_with(|| SumWeeklyCommits {
                    authors: HashMap::new(),
                })
                .authors
                .entry(author.clone())
                .and_modify(|c| *c += sum_commit)
                .or_insert(sum_commit);
        }
    }

    commits
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github::contributors::{Author, Week};

    #[test]
    fn should_return_commits_per_week() {
        let contributors = vec![
            Contributor {
                total: 1,
                weeks: vec![
                    Week {
                        w: 1361059200,
                        a: 0,
                        d: 0,
                        c: 1,
                    },
                    Week {
                        w: 1361664000,
                        a: 0,
                        d: 0,
                        c: 0,
                    }, 
                    Week {
                        w: 1362268800,
                        a: 0,
                        d: 0,
                        c: 0,
                    },
                ],
                author: Author {
                    login: "octocat".to_string(),
                },
            },
            Contributor {
                total: 4,
                weeks: vec![
                    Week {
                        w: 1361059200,
                        a: 0,
                        d: 0,
                        c: 3,
                    },
                    Week {
                        w: 1361664000,
                        a: 0,
                        d: 0,
                        c: 1,
                    }, 
                    Week {
                        w: 1362268800,
                        a: 0,
                        d: 0,
                        c: 1,
                    },
                ],
                author: Author {
                    login: "octobot".to_string(),
                },
            },
        ];

        let commits = get_commits_per_week(contributors);
        assert_eq!(commits.weekly_commits.len(), 3);
        assert_eq!(commits.sum_commits.len(), 3);
        assert_eq!(commits.first_week, 1361059200);
        assert_eq!(commits.last_week, 1362268800);
        assert_eq!(commits.total_weeks, 3);
        assert_eq!(
            commits
                .weekly_commits
                .get(&1361059200)
                .unwrap()
                .authors
                .len(),
            2
        );
        assert_eq!(
            commits
                .weekly_commits
                .get(&1361059200)
                .unwrap()
                .authors
                .get("octocat")
                .unwrap(),
            &1
        );
        assert_eq!(
            commits
                .weekly_commits
                .get(&1361059200)
                .unwrap()
                .authors
                .get("octobot")
                .unwrap(),
            &3
        );

        assert_eq!(
            commits
                .sum_commits
                .get(&1362268800)
                .unwrap()
                .authors
                .get("octocat")
                .unwrap(),
            &1
        );
        assert_eq!(
            commits
                .sum_commits
                .get(&1362268800)
                .unwrap()
                .authors
                .get("octobot")
                .unwrap(),
            &5
        );
    }
}
