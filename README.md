# 💬 npm-expansions
A simple JSON rest API providing random expansions of the NPM acronym with an associated website to boot!

<br>

# 🧭 Project Goals
- To learn about rust as a language, specifically ownership and type rules
- To learn about thread management in rust
- To learn the nitty gritty details of HTTP and server implementation

<br>

# 📋 Short Term ToDo
1. ### Error Handling
    1. Update main to render 500 if any errors occur
2. ### Website Functionality
    1. Design NPM expansions home page, 404 page and 500 page in figma
    2. Implement figma designs using current boilerplate html, css and js files
3. ### Tests
    1. Refactor main tests
    2. Add tests for all public untested functions

<br>

# 🗺️ Long Term ToDo
1. Implement husky with testing and formatting
2. Implement CICD with github actions
- Handle incoming requests in an optimised way e.g. threads or asynchronous events
- Before server starts minify/uglify all javascript code (https://crates.io/crates/minifier)
- Polished readme which outlines the goal of the project, contains a screenshot of the website and a description of the API with sections on usage, development and installation
- Background routine to check if the official npm expansions repo has updated its `expansions.txt` file and to update this project's equivalent file with any changes. Note that the last change to the expansions text file occurred two years ago.
- Have a struct with gathers/reads static files into collections before server starts to enhance performance of static file serving
- Setup NPM expansions struct before server begins to save processing at runtime
- Add documentation comments for all public functions

<br>

# 💭 Reminders
- Remember some issues could be client caused. If they parse incorrect headers thats 400 not 500
- Have src folder for code and dist folder for minified output
- Routinely updating expansions.txt could be done at startup and during execution using a second thread with message parsing