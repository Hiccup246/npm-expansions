# 💬 npm-expansions

<!-- ![](https://img.shields.io/github/license/Hiccup246/npm-expansions)
![](https://img.shields.io/github/languages/code-size/Hiccup246/npm-expansions) -->

A simple JSON rest API providing random expansions of the NPM acronym with an associated website to boot!

<br>

# 🧭 Project Goals
- To learn about rust as a language, specifically ownership and type rules
- To learn about thread management in rust
- To learn the nitty gritty details of HTTP and server implementation
- To learn about docker deployment
- To learn about reverse proxies

<br>

# Development
1. Start web server with `DEV=true cargo run`
2. Start proxy with `docker-compose up --build`

<br>

# 📋 Short Term ToDo
1. ### Website Functionality
    1. Implement OG and Twitter SEO tags
2. ### Documentation
    1. Update controller function documentation comments to be more detailed about response format and returned values i.e. 200 vs 406
    2. Adding documentation comments for all public structs, enums and functions and the crate as a whole
3. ### Readme
    1. Polished readme which outlines the goal of the project, contains a screenshot of the website and a description of the API with sections on usage, development and installation
4. ### Refactoring
    1. Read up about rust coding styles and idiomatic rust and refactor code to fit this style
    2. Consider replacing a lot of `if let Ok(x) = y`, `if let Err(x) = y`, `if let Some(x) = y` and `if let None = y` patterns
    with bubble up patterns like `?`
<br>

# 🗺️ Long Term ToDo
1. Update jameswatt.io with this project
2. Implement PostGres database with docker for npm expansions
- Handle incoming requests in an optimised way e.g. threads or asynchronous events
- Background routine to check if the official npm expansions repo has updated its `expansions.txt` file and to update this project's equivalent file with any changes. Note that the last change to the expansions text file occurred two years ago.
- Add server logs for each request and failure
- Refactor NpmExpansions to be instantiated once in the main and then passed around. This will increase performance
  and allow the dynamic updating of the expansions.txt file in a separate thread.
- Routinely updating expansions.txt could be done at startup and during execution using a second thread with message parsing