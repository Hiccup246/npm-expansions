async function generateRandomExpansion() {
    const randomExpansionResponse = await fetch("/api/random");
    const randomExpansionJSONResponse = await randomExpansionResponse.json();

    document.querySelector(".npm-expansion").innerHTML = randomExpansionJSONResponse["npm-expansion"];
}

async function loadExpansionIntoTextArea() {
    const randomExpansionResponse = await fetch("/api/random");
    const randomExpansionJSONResponse = await randomExpansionResponse.json();

    document.querySelector(".results-expansions-list").innerHTML = randomExpansionJSONResponse["npm-expansion"];
}

async function searchExpansions(query) {
    const searchExpansionResponse = await fetch(`/api/search?query=${query.target.value}`);
    const searchExpansionJSONResponse = await searchExpansionResponse.json();
    
    const textareaString = searchExpansionJSONResponse.reduce((acc, expansion) => acc + (expansion + "\n\n"), "");
    const textarea = document.querySelector(".results-expansions-list");
    textarea.innerHTML = textareaString;
    textarea.setAttribute("rows", searchExpansionJSONResponse.length);
}

async function loadAllExpansions() {
    const allExpansionResponse = await fetch("/api/all");
    const allExpansionJSONResponse = await allExpansionResponse.json();

    const textareaString = allExpansionJSONResponse.reduce((acc, expansion) => acc + (expansion + "\n\n"), "");
    const textarea = document.querySelector(".results-expansions-list");
    textarea.innerHTML = textareaString;
    textarea.setAttribute("rows", allExpansionJSONResponse.length);
}

function copyExpansionsToClipboard() {
    const expansions = document.querySelector(".results-expansions-list").innerHTML;
    navigator.clipboard.writeText(expansions);
}

function debounce (context, func, delay) {
    let timeout;

    return (...arguments) => {
        if (timeout) {
        clearTimeout(timeout);
        }

        timeout = setTimeout(() => {
        func.apply(context, arguments);
        }, delay);
    };
};

const debouncedSearchState = debounce(
    this,
    searchTerm => searchExpansions(searchTerm),
    500
);

generateRandomExpansion();
loadExpansionIntoTextArea();

const input = document.querySelector(".search-input");
input.addEventListener("input", debouncedSearchState);
