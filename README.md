# 💬 npm-expansions
A simple JSON rest API providing random expansions of the NPM acronym with an associated website to boot!

<br>

# 🧭 Project Goals
- To learn about rust as a language, specifically ownership and type rules
- To learn about thread management in rust
- To learn the nitty gritty details of HTTP and server implementation

<br>

# 📋 Short Term ToDo
1. ### Accept Header Parsing
    1. Complete tests for mime_type_parser functions
    2. Complete tests for all accept_header_handler functions
    3. Update main to correctly use accept_header_handler functions. This includes handling results and correctly returning
      500 or 406 errors
2. ### Refactoring
    1. Get main.rs function working again
    2. Refactor route handling perhaps into controller like struct
    3. Have a struct with gathers/reads static files into collections before server starts to enhance performance of static file serving
    4. Setup NPM expansions struct before server begins to save processing at runtime
3. ### Website Functionality
    1. Design NPM expansions home page, 404 page and 500 page in figma
    2. Implement figma designs using current boilerplate html, css and js files

<br>

# 🗺️ Long Term ToDo
1. Implement husky with testing and formatting
2. Implement CICD with github actions
- Handle incoming requests in an optimised way e.g. threads or asynchronous events
- Before server starts minify/uglify all javascript code (https://crates.io/crates/minifier)
- Polished readme which outlines the goal of the project, contains a screenshot of the website and a description of the API with sections on usage, development and installation
- Background routine to check if the official npm expansions repo has updated its `expansions.txt` file and to update this project's equivalent file with any changes. Note that the last change to the expansions text file occurred two years ago.

<br>

# 💭 Reminders
- Remember some issues could be client caused. If they parse incorrect headers thats 400 not 500
- Have src folder for code and dist folder for minified output
- Routinely updating expansions.txt could be done at startup and during execution using a second thread with message parsing