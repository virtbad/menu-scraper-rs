use chrono::{DateTime, Utc};
use serde::{Serialize};
use chrono::serde::ts_milliseconds;
use log::{error, info, warn};
use serde_repr::Serialize_repr;

#[derive(Debug, Clone, Serialize_repr)]
#[repr(u8)]
pub enum Label {
    Missing = 0,
    Vegetarian = 1,
    Vegan = 2,
    OneClimate = 3,
}

impl From<String> for Label {
    fn from(value: String) -> Self {
        match value.as_str() {
            "vegan" => Label::Vegan,
            "vegetarian" => Label::Vegetarian,
            // TODO: Check whether this is really 'one-climate' since I couldn't find any one climate menu at the moment
            "one-climate" => Label::OneClimate,
            _ => Label::Missing
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Menu {
    pub title: String,
    pub description: String,
    #[serde(with = "ts_milliseconds")]
    pub date: DateTime<Utc>,
    pub channel: u8,
    pub label: Label,
    pub prices: Vec<Price>,
}

#[derive(Serialize, Debug)]
pub struct Price {
    pub tag: String,
    pub price: f32,
}


pub struct MenuAPI {
    url: String
}

impl MenuAPI {
    pub fn new(mut url: String) -> Self {
        if !url.ends_with('/') {
            url.push('/');
        }

        Self {
            url
        }
    }

    /// submit new menus to the menu api
    pub fn submit_menus(&self, menus: Vec<Vec<Menu>>) {
        info!("Submitting menus to api");
        let client = reqwest::blocking::Client::new();
        menus.iter().for_each(|day| {
            day.iter().for_each(|menu| {
                info!("Submitting menu \"{}\"", menu.title);
                let response = client.post(format!("{}menu", self.url)).json(&menu).send();
                match response {
                    Ok(response) => {
                        if response.status().is_success() {
                            info!("Successfully submitted new menu to api");
                        } else if response.status().is_server_error() {
                            error!("Internal server error during menu submission");
                        } else if response.status() == 403 {
                            error!("Menus can only be submitted from localhost");
                        } else if response.status() == 409 {
                            warn!("This menu was already submitted")
                        } else {
                            error!("Error during menu submission. Status {:?}", response.status());
                        }
                    },
                    Err(err) => {
                        error!("Error during menu submission: {err:?}");
                    }
                }
            })
        });
    }
}