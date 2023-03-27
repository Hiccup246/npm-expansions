# 💬 npm-expansions

<!-- ![](https://img.shields.io/github/license/Hiccup246/npm-expansions)
![](https://img.shields.io/github/languages/code-size/Hiccup246/npm-expansions) -->

A simple JSON rest API providing random expansions of the NPM acronym with an associated website to boot!

<br>

# 🧭 Project Goals
- To learn about rust as a language, specifically ownership and type rules
- To learn about thread management in rust
- To learn the nitty gritty details of HTTP and server implementation

<br>

# 📋 Short Term ToDo
1. ### Website Functionality
    1. Implement search all expansions functionality
    2. Implement copy all expansions to clipboard and view all expansions functionality
    3. Implement OG and Twitter SEO tags
    4. Fix FIFO of assets and fonts
    5. See if background image looks just as good in webp format
    6. Add fade in amination for button hover
    7. Maybe add squeeze or enlarge animation for button click
2. ### Infrastructure
    1. Before server starts minify/uglify all javascript code (https://crates.io/crates/minifier)
    2. Update cargo to have dev and prod configs
    3. Update nginx to have dev and prod configs
2. ### Tests
    1. Add function specific functionality tests for controller functions i.e. checking stream response
    2. Add integration tests for server
3. ### Benchmarks
    1. Read up about the benchmarks directory in rust and whether I should utilise it
4. ### Documentation
    1. Update controller function documentation comments to be more detailed about response format and returned values i.e. 200 vs 406
    2. Consider adding documentation comments for structs and crate as a whole
5. ### Refactoring
    1. Read up about rust coding styles and idiomatic rust and refactor code to fit this style
    2. Consider replacing a lot of `if let Ok(x) = y`, `if let Err(x) = y`, `if let Some(x) = y` and `if let None = y` patterns
    with bubble up patterns like `?` 
<br>

# 🗺️ Long Term ToDo
1. Polished readme which outlines the goal of the project, contains a screenshot of the website and a description of the API with sections on usage, development and installation
2. Implement PostGres database with docker for npm expansions
4. Update jameswatt.io with this project
- Handle incoming requests in an optimised way e.g. threads or asynchronous events
- Polished readme which outlines the goal of the project, contains a screenshot of the website and a description of the API with sections on usage, development and installation
- Background routine to check if the official npm expansions repo has updated its `expansions.txt` file and to update this project's equivalent file with any changes. Note that the last change to the expansions text file occurred two years ago.
- Double check all error handling and panic scenarios and consider making errors more generic or/and more specific
- Add server logs for each request and failure

<br>

# 💭 Reminders
- Remember some issues could be client caused. If they parse incorrect headers thats 400 not 500
- Have src folder for code and dist folder for minified output
- Routinely updating expansions.txt could be done at startup and during execution using a second thread with message parsing