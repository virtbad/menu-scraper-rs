# Menu Scraper

This is a web scraper which scrapes the menus of any sv-group restaurant and uploads them to the [menu-api](https://github.com/virtbad/menu-api).
It is able to upload all menus accessible on the sv-group website for a specific restaurant (including those for the coming days of the week).

## Usage

Every time the scraper is run, it will scrape the menus of the current week, upload them to the configured menu-api and exit.

Ideally the scraper is run once a day in something like a cron-job, so that the menus are always up to date.

## Configuration

The configuration of the scraper is stored at `[os-config-dir]/menu-scraper/menu-scraper.toml`. 
Should this file not exist, it will be created with default values which then need to be replaced.

```toml
api_remote = '' # The url of the menu-api
website_remote = '' # The url of the sv-group website which should be scraped
```

> The configuration values can be overwritten by the following environment variables:
> * `API` overwrites the `api_remote` value
> * `WEBSITE` overwrites the `website_remote` value
> 
> When an environment variable is set, it will be used instead of the configuration value.

## Hosting with docker

The menu scraper is also available as a docker image published to the github container registry. It can be pulled with the following command:

```bash
docker pull ghcr.io/virtbad/menu-scraper:latest
```

> Every image has its own tag, which is the same as the version of the scraper. You can find all available tags [here](https://github.com/virtbad/menu-scraper-rs/tags). 
> To get the latest version use the `latest` tag.

### Configuration

Once pulled you need to run the container with the following environment variables to configure it properly. The scraper needs to be able to access the api and the website, so you need to provide the urls to them.

```bash
API="" # The url of the menu-api
WEBSITE="" # The url of the sv-group website which should be scraped
```

# Related Projects

* [menu-api](https://github.com/virtbad/menu-api)
* [menu-website](https://github.com/virtbad/menu-website)
* [menu-telegram-bot](https://github.com/virtbad/menu-website)
* [menu-cli](https://github.com/virtbad/menu-cli)
* [menu-updater](https://github.com/virtbad/menu-updater)

# License

Coming Soon.