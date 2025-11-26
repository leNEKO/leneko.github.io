use core::fmt;
use std::{collections::HashMap, fs::File, path::PathBuf};

use anyhow::{Context, Result};
use chrono::{Datelike, Months, NaiveDate, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{Techno, TechnoKey, Technos};

#[derive(Deserialize, Serialize, Debug, JsonSchema, Hash, PartialEq, Eq, Clone)]
pub struct Locale(String);

#[derive(Deserialize, Serialize, Debug, JsonSchema, Clone)]
pub struct LocalizedString(HashMap<Locale, String>);

#[test]
fn test_localized_data_unserialize() -> Result<()> {
    #[derive(Deserialize, Serialize, Debug, JsonSchema)]
    struct MyData {
        title: LocalizedString,
        content: LocalizedString,
    }

    let input = r#"
        title:
            en: my title
            fr: mon titre
        content:
            en: my content
            fr: mon contenu
    "#;

    let actual: MyData = serde_yaml::from_str(input)?;
    assert_eq!(
        "mon contenu",
        actual.content.0.get(&Locale("fr".into())).unwrap()
    );
    dbg!(actual);

    Ok(())
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Entry {
    title: LocalizedString,
    comment: Option<LocalizedString>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema, Clone)]
pub struct JobTitle(LocalizedString);

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Education {
    formations: Vec<Entry>,
    languages: Vec<Entry>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Address {
    cp: String,
    city: String,
    gmap: Url,
    street: String,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Contact {
    pub firstname: String,
    pub lastname: String,
    email: String, // TODO: Email type
    website: Url,
    github: Url,
    phone: Option<String>,
    birthday: NaiveDate,
    career_start: NaiveDate,
    avalaible_date: Option<NaiveDate>,
    dispo_delay: Option<u8>, // in months
    pub job_title: JobTitle,
    address: Address,
    mobility: Option<LocalizedString>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Period {
    start: NaiveDate,
    end: NaiveDate,
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.start.year(), self.end.year())
    }
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Task(LocalizedString);

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Company {
    name: String,
    link: Option<Url>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Experience {
    pub period: Period,
    job_title: JobTitle,
    company: Option<Company>,
    tasks: Vec<Task>,
    technos: Vec<TechnoKey>,
}

impl Experience {
    pub fn get_technos(&self, technos: &Technos) -> Result<Vec<Techno>> {
        let mut results: Vec<Techno> = Vec::new();
        for key in &self.technos {
            let techno = technos.0.get(key).context("not found")?;
            results.push(techno.clone());
        }

        Ok(results)
    }
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Experiences(pub Vec<Experience>);

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Cv {
    pub gtag: String,
    pub contact: Contact,
    pub education: Education,
    pub experiences: Experiences,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    job_title: JobTitle,
    available_date: Option<String>,
    elapsed: u32, // years
    mobility: Option<LocalizedString>,
}

impl Cv {
    pub fn from_yaml(path: &PathBuf) -> Result<Self> {
        Ok(serde_yaml::from_reader(File::open(path)?) //
            .expect("Invalid path"))
    }

    pub fn get_info(&self) -> Result<Info> {
        let now = Utc::now().date_naive();
        let elapsed = now.years_since(self.contact.career_start).unwrap();

        let available_date = match self.contact.avalaible_date {
            Some(d) => Some(d),
            None => match self.contact.dispo_delay {
                Some(d) => now.checked_add_months(Months::new(d.into())),
                None => None,
            },
        };

        Ok(Info {
            job_title: self.contact.job_title.clone(),
            available_date: available_date.map(|d| d.format("%B %Y").to_string()),
            elapsed,
            mobility: self.contact.mobility.clone(),
        })
    }
}
