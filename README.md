# 💬 npm-expansions
A simple JSON rest API providing random expansions of the NPM acronym with an associated website to boot!

<br>

# 🧭 Project Goals
- To learn about rust as a language, specifically ownership and type rules
- To learn about thread management in rust
- To learn the nitty gritty details of HTTP and server implementation

<br>

# 📋 ToDo
- Essential functionality
    - JSON rest API that picks a random expansion from a list and returns it
    - A basic website with UI that generates a random npm expansion
    - A basic website with UI that allows you to view all npm expansions
- Polished readme which outlines the goal of the project, contains a screenshot of the website and a description of the API with sections on usage, development and installation
- Optional functionality
    - Background routine to check if the official npm expansions repo has updated its `expansions.txt` file and to update this project's equivalent file with any changes. Note that the last change to the expansions text file occurred two years ago.