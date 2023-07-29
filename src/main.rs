use anyhow::anyhow;
use chrono::{Datelike, DateTime, NaiveDateTime, Utc};
use log::info;
use regex::Regex;
use scraper::ElementRef;
use crate::api::{Label, Menu, MenuAPI, Price};
use crate::config::Config;

mod config;
mod api;

fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    let config: Config = Config::parse()?;
    config.check()?;

    let api: MenuAPI = MenuAPI::new(config.api_remote);

    let response = reqwest::blocking::get(config.website_remote);
    if response.is_err() {
        return Err(anyhow!("Unable to fetch menu plan from website remote"))
    }

    let response = response.unwrap();
    let text = response.text().expect("should be able to get text");
    let document = scraper::Html::parse_document(&text);

    let label_regex = Regex::new(r".*label-(?<label>\w+).*").expect("should parse regex");
    let date_selector = scraper::Selector::parse("span.date").expect("should parse selector");
    let menu_title_selector = scraper::Selector::parse("h2.menu-title").expect("should parse selector");
    let menu_description_selector = scraper::Selector::parse("p.menu-description").expect("should parse selector");
    let menu_labels_selector = scraper::Selector::parse("div.menu-labels > span.label").expect("should parse selector");
    let menu_prices_selector = scraper::Selector::parse("span.price").expect("should parse selector");
    let menu_price_val_selector = scraper::Selector::parse("span.val").expect("should parse selector");
    let menu_price_desc_selector = scraper::Selector::parse("span.desc").expect("should parse selector");

    let current_date = Utc::now();

    let dates = document.select(&date_selector).enumerate().filter_map(|(index, date)| {
        let month_date = date.inner_html();
        let date = NaiveDateTime::parse_from_str(format!("{month_date}{} 0:0:0", current_date.year()).as_str(), "%d.%m.%Y %H:%M:%S");
        if date.is_err() {
            return None
        }
        let date = DateTime::from_utc(date.expect("should be valid date"), Utc);

        let menu_selector = scraper::Selector::parse(format!("div.menu-plan-grid#menu-plan-tab{} div.item-content", index + 1).as_str()).expect("should parse selector");
        let menus = document.select(&menu_selector).enumerate().map(|(channel, menu)| {
            let title = menu.select(&menu_title_selector).filter_map(parse_element_text).collect::<String>();
            let description = menu.select(&menu_description_selector).filter_map(parse_element_text).collect::<String>();
            let labels = menu.select(&menu_labels_selector).map(|label| {
                label.value().attrs.values()
                    .filter_map(|attr| label_regex.captures(attr).and_then(|c| c.name("label")))
                    .map(|regex_match| regex_match.as_str().to_string())
                    .collect::<String>()
                    .into()
            }).collect::<Vec<Label>>();
            let label = labels.first().map_or(Label::Missing, |l| l.clone());
            drop(labels);
            let prices = menu.select(&menu_prices_selector).filter_map(|price| {
                let val = price.select(&menu_price_val_selector).filter_map(parse_element_text).filter_map(|val| val.parse::<f32>().ok()).collect::<Vec<f32>>();
                let desc = price.select(&menu_price_desc_selector).filter_map(parse_element_text).collect::<String>();
                let Some(val) = val.first() else {
                    return None
                };
                Some(Price { price: *val, tag: desc })
            }).collect::<Vec<Price>>();
            Menu {
                label,
                date,
                prices,
                title: title.replace("-\n", ""),
                description: description.replace("-\n", ""),
                channel: channel as u8
            }
        }).collect::<Vec<Menu>>();
        Some(menus)
    }).collect::<Vec<Vec<Menu>>>();
    api.submit_menus(dates);
    info!("Exiting");
    Ok(())
}

fn parse_element_text(element: ElementRef) -> Option<&str> {
    element.text().collect::<Vec<_>>().first().cloned()
}