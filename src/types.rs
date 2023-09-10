use crate::{
    urls::{Route, WIKI_ENDPOINT_URL},
    SAVE_FILE,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{Read, Write},
    path::Path,
};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Merch {
    pub highalch: Option<u64>,
    pub members: bool,
    pub name: String,
    pub id: u64,
    pub value: u64,
    pub lowalch: Option<u64>,
    pub limit: Option<i32>,
    pub high: Option<u64>,
    pub high_time: Option<u64>,
    pub low: Option<u64>,
    pub low_time: Option<u64>,
}

impl From<MappingItem> for Merch {
    fn from(value: MappingItem) -> Self {
        Self {
            highalch: value.highalch,
            members: value.members,
            name: value.name,
            id: value.id,
            value: value.value,
            lowalch: value.lowalch,
            limit: value.limit,
            high: None,
            high_time: None,
            low: None,
            low_time: None,
        }
    }
}

impl PartialEq<Self> for Merch {
    fn eq(&self, other: &Self) -> bool {
        let other_arbitrage = match (other.high, other.highalch) {
            (Some(other_high), Some(other_highalch)) => {
                Some(other_highalch as i64 - other_high as i64)
            }
            _ => None,
        };

        let self_arbitrage = match (self.high, self.highalch) {
            (Some(self_high), Some(self_highalch)) => Some(self_highalch as i64 - self_high as i64),
            _ => None,
        };

        match (self_arbitrage, other_arbitrage) {
            (Some(x), Some(y)) => x == y,
            _ => true,
        }
    }
}

impl Eq for Merch {}

impl PartialOrd<Self> for Merch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Merch {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_arbitrage = match (other.high, other.highalch) {
            (Some(other_high), Some(other_highalch)) => {
                Some(other_highalch as i64 - other_high as i64)
            }
            _ => None,
        };

        let self_arbitrage = match (self.high, self.highalch) {
            (Some(self_high), Some(self_highalch)) => Some(self_highalch as i64 - self_high as i64),
            _ => None,
        };

        match (self_arbitrage, other_arbitrage) {
            (Some(x), Some(y)) => x.cmp(&y),
            _ => Ordering::Equal,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MappingItem {
    pub highalch: Option<u64>,
    pub members: bool,
    pub name: String,
    pub examine: String,
    pub id: u64,
    pub value: u64,
    pub icon: String,
    pub lowalch: Option<u64>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestResponse {
    pub data: HashMap<u64, LatestGEData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct LatestGEData {
    pub high: Option<u64>,
    pub highTime: Option<u64>,
    pub low: Option<u64>,
    pub lowTime: Option<u64>,
}

impl TryFrom<String> for LatestResponse {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value)
            .map_err(|err| format!("Failed to deserialize JSON. {}", err.to_string()))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Stats {
    avg_value: u64,
    item_count: u64,
}

pub struct Context {
    pub items: BTreeMap<u64, Merch>,
    pub client: Client,
}

impl Context {
    /// Build them Items DB in memory.
    pub async fn build_items_db(&mut self) -> Result<(), String> {
        if Path::new(SAVE_FILE).exists() {
            let read_result = self.read(SAVE_FILE);
            if read_result.is_ok() {
                println!(
                    "Loaded items DB from disk ({}). Stats: {:#?}",
                    SAVE_FILE,
                    self.compute_stats()
                );
                return Ok(());
            } else {
                println!(
                    "ERROR: Failed to read items from disk ({}). Reason: {:?}",
                    SAVE_FILE,
                    read_result.err()
                );
            }
        }

        let response = self
            .get(Route::Mapping)
            .await
            .map_err(|err| err.to_string())?;
        let items: Vec<MappingItem> =
            serde_json::from_str(&response).map_err(|err| err.to_string())?;
        for item in items {
            let merch = Merch::from(item);
            let duplicate = self.items.insert(merch.id, merch).is_some();
            if duplicate {
                println!("ERROR: duplicate entry detected in Context::items");
            }
        }

        println!(
            "Loaded items DB from wiki. Stats:  {:#?}",
            self.compute_stats()
        );
        Ok(())
    }

    pub async fn get(&self, route: Route) -> Result<String, reqwest::Error> {
        let mut resource = WIKI_ENDPOINT_URL.to_string();
        resource.push_str(&route.endpoint());
        let response = self.client.get(resource).send().await?;

        response.text().await
    }

    pub fn flush(&self, file_name: &str) -> std::io::Result<()> {
        // Serialize the BTreeMap
        let serialized_map = serde_json::to_string(&self.items).unwrap();

        // Open a file in write-only mode
        let mut file = File::create(file_name)?;

        // Write the serialized BTreeMap to the file
        file.write_all(serialized_map.as_bytes())?;

        Ok(())
    }

    pub fn read(&mut self, file_name: &str) -> std::io::Result<()> {
        // Open the file in read-only mode
        let mut file = File::open(file_name)?;

        // Initialize a string to hold file content
        let mut contents = String::new();

        // Read file content to string
        file.read_to_string(&mut contents)?;

        // Deserialize the string content to a BTreeMap<u64, Item>
        self.items = serde_json::from_str(&contents).unwrap();

        Ok(())
    }

    pub fn compute_stats(&self) -> Stats {
        let value_sum: u64 = self.items.iter().map(|item| item.1.value).sum();
        let avg_value = value_sum / self.items.len() as u64;

        Stats {
            avg_value,
            item_count: self.items.len() as u64,
        }
    }
}
