use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};
use tokio::io::{self, BufReader};

#[derive(Debug)]
enum Error {
    /// A random_id was seen twice.
    DuplicateVote,

    /// The invariant `yes + no == total` failed for a candidate.
    /// It's a bit like double entry accounting. :]
    InvariantYesNoSum(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug)]
struct Vote {
    yes: Vec<String>,
    no: Vec<String>,
    random_id: String,
}

#[derive(Default, Debug)]
struct Count {
    total: u32,
    vote_ids: HashSet<String>,
    candidates: HashSet<String>,
    yes: HashMap<String, u32>,
    no: HashMap<String, u32>,
}

// We're printing this as Debug.
#[allow(dead_code)]
#[derive(Default, Debug)]
struct Summary {
    votes: u32,
    confidence_threshold: u32,
    confidence: HashSet<String>,
    yes: HashMap<String, u32>,
}

impl Count {
    fn assert_unique_vote(&mut self, id: String) -> Result<(), Error> {
        if !self.vote_ids.insert(id) {
            Err(Error::DuplicateVote)
        } else {
            Ok(())
        }
    }

    fn include_candidate(&mut self, candidate: &String) {
        self.candidates.insert(candidate.to_string());
    }

    fn increment_yes(&mut self, candidate: &String) {
        if let Some(c) = self.yes.get_mut(candidate) {
            *c += 1;
        } else {
            self.yes.insert(candidate.to_string(), 1);
        }
    }

    fn increment_no(&mut self, candidate: &String) {
        if let Some(c) = self.no.get_mut(candidate) {
            *c += 1;
        } else {
            self.no.insert(candidate.to_string(), 1);
        }
    }
}

impl TryFrom<Count> for Summary {
    type Error = Error;

    fn try_from(value: Count) -> std::result::Result<Self, Self::Error> {
        let confidence_threshold = (f64::from(value.total) / 2f64).ceil() as u32;
        let mut confidence: HashSet<String> = Default::default();

        for candidate in value.candidates.iter() {
            let yes = value.yes.get(candidate).map_or(0, ToOwned::to_owned);
            let no = value.no.get(candidate).map_or(0, ToOwned::to_owned);
            if yes + no != value.total {
                return Err(Error::InvariantYesNoSum(candidate.to_string()));
            }

            if yes >= confidence_threshold {
                confidence.insert(candidate.to_string());
            }
        }

        Ok(Self {
            votes: value.total,
            confidence_threshold,
            confidence,
            yes: value.yes,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut count = Count::default();
    let mut feed = BufReader::new(io::stdin());

    while let Ok(vote) = jsonl::read::<_, Vote>(&mut feed).await {
        count.total += 1;
        for c in &vote.yes {
            count.include_candidate(c);
            count.increment_yes(c);
        }
        for c in &vote.no {
            count.include_candidate(c);
            count.increment_no(c);
        }
        count.assert_unique_vote(vote.random_id)?
    }

    let summary: Summary = count.try_into()?;
    println!("{:#?}", summary);

    Ok(())
}
