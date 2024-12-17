use serde::{Deserialize, Serialize};
use serde_json::{Error};

#[derive(Deserialize, Serialize, Clone)]
pub struct Contributor {
    pub total: u32,
    pub author: Author,
    pub weeks: Vec<Week>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Author {
    pub login: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Week {
    pub w: u32,
    pub a: u32,
    pub d: u32,
    pub c: u32,
}

pub fn serialize_contributors(json_content: &str) -> Result<Vec<Contributor>, Error> {
    serde_json::from_str(json_content)
}

#[cfg(test)]
mod tests {
    use crate::github::contributors::serialize_contributors;

    #[test]
    fn should_parse_string_to_contributors() {
        let json_content = r#"
        [
            {
                "total": 1,
                "author": {
                    "login": "octocat"
                },
                "weeks": [
                    {
                        "w": 1590403200,
                        "a": 0,
                        "d": 0,
                        "c": 1
                    }
                ]
            }
        ]
        "#;
        let contributors = serialize_contributors(json_content).unwrap();
        assert_eq!(contributors.len(), 1);
        assert_eq!(contributors[0].total, 1);
        assert_eq!(contributors[0].author.login, "octocat");
        assert_eq!(contributors[0].weeks.len(), 1);
        assert_eq!(contributors[0].weeks[0].w, 1590403200);
        assert_eq!(contributors[0].weeks[0].a, 0);
        assert_eq!(contributors[0].weeks[0].d, 0);
        assert_eq!(contributors[0].weeks[0].c, 1);

    }
}
