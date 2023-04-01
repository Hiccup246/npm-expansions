# 💬 npm-expansions

<!-- ![](https://img.shields.io/github/license/Hiccup246/npm-expansions)
![](https://img.shields.io/github/languages/code-size/Hiccup246/npm-expansions)
![](https://img.shields.io/github/actions/workflow/status/hiccup246/npm-expansions/unit-tests.yml?branch=main&label=Unit%20Tests)
![](https://img.shields.io/github/actions/workflow/status/hiccup246/npm-expansions/style-check.yml?branch=main&label=Style%20Check) -->

<!-- ![site-screenshot](https://raw.githubusercontent.com/Hiccup246/npm-expansions/main/nginx-reverse-proxy/static/site-screenshot.webp) -->

A website and JSON API that allows npm expansions to be generated and searched. An NPM expansion represents the words with make up the NPM acronym e.g. "Nice People Meet".

The website is written in vanilla HTML, CSS, and JS, built using [rust]((https://www.rust-lang.org/)), and served via [NGINX]((https://www.nginx.com/)). The JSON API is built using rust. Both the JSON API and NGINX server are deployed using docker and the infrastructure service [fly.io](https://fly.io/).

<br>

# 🧭 Project Goals
- To learn about rust as a language, specifically ownership and type rules
- To learn about the nitty-gritty details of the HTTP protocol
- To learn about docker deployment
- To learn about nginx and reverse proxies

<br>

# 🗺️ Understanding the project
The project is made up of a JSON web server built in rust and a static website built with rust and hosted using nginx.

## NPM Expansions
The JSON web server is located within the `npm-expansions` directory and is responsible for serving JSON responses via TCP to routes with the prefix `/api`.

## Nginx reverse proxy
The nginx web server is located within `nginx-reverse-proxy` alongside the rust project to build the project's static site. The server is responsible for serving the static site and reverse proxying requests to the JSON web server. The nginx server itself is configured by `npm-expansions.conf` for production and `npm-expansions.dev.conf` for development.

The rust project within `nginx-reverse-proxy` is responsible for taking all the assets within the `pages` and `static` directories and performing minification on all `CS`, `JS`, and `HTML` files. After minification the assets are placed into directories called `minified_pages` and `minified_static` directories.

<br>

# 🕹️ Usage
The static website can be accessed at `https://www.npm-expansions.com` and requests to the JSON API can be made to `https://npm-expansions.com/api`. The JSON API supports the following routes:
- GET /api/random - Returns a random expansion in the format `{ "npm-expansion": "Nonce Pseudo Manager" }`
- GET /api/all - Returns all npm expansions in array format `["Nobody Pieces Moons", "Nibble Pickles Matches"]`
- GET /api/search?query=abc - Returns the top 10 matching expansions to the provided search query in array format `["Nobody Pieces Moons", "Nibble Pickles Matches"]`

<br>

# 🔧 Development
This project contains two applications, an npm-expansions JSON web server, and an nginx reverse proxy server. To develop this project first ensure you have the following programs installed:
- A rust nightly build
- Docker

Then follow the steps below:
1. Navigate to the `npm-expansions` directory and start the web server with the command `DEV=true cargo run`
2. Start docker and start the reverse proxy by navigating to the `nginx-reverse-proxy` directory and running the command `docker-compose up --build`

<br>

# 🛰️ Deployment
This project is configured to be deployed to [fly.io](https://fly.io/). To deploy follow the steps below:
- Ensure you have a fly.io account
- Ensure you have authenticated via the fly.io CLI
- Create two fly.io Apps by running the command `fly launch` and using the names `npm-expansions` and `npm-expansions-reverse-proxy`
- Navigate to the nginx-reverse-proxy directory and run the command `fly deploy`
- Navigate to the npm-expansions directory and run the command `fly deploy`

<br>

# 📋 ToDo
1. ### Documentation
    1. Update controller function documentation comments to be more detailed about response format and returned values i.e. 200 vs 406
    2. Adding documentation comments for all public structs, enums, and functions and the crate as a whole
3. ### Refactoring
    1. Read up about rust coding styles and idiomatic rust and refactor code to fit this style
    2. Consider replacing a lot of `if let Ok(x) = y`, `if let Err(x) = y`, `if let Some(x) = y` and `if let None = y` patterns
    with bubble-up patterns like `?`
4. ### Long term todo
- Review readme and decide if any upgrades are needed
- Add project to google search console
- Update jameswatt.io with this project
- Implement PostGres database with docker for npm expansions
- Handle incoming requests in an optimized way e.g. threads or asynchronous events
- Background routine to check if the official npm expansions repo has updated its `expansions.txt` file and to update this project's equivalent file with any changes. Note that the last change to the expansions text file occurred two years ago.
- Add server logs for each request and failure
- Refactor NpmExpansions to be instantiated once in the main and then passed around. This will increase performance
  and allow the dynamic updating of the expansions.txt file in a separate thread.
- Routinely updating expansions.txt could be done at startup and during execution using a second thread with message parsing