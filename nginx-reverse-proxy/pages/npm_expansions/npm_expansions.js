async function generateRandomExpansion() {
    const randomExpansionResponse = await fetch("/api/random");
    const randomExpansionJSONResponse = await randomExpansionResponse.json();

    document.querySelector(".npm-expansion").innerHTML = randomExpansionJSONResponse["npmExpansion"];
}

async function loadExpansionIntoTextArea() {
    const randomExpansionResponse = await fetch("/api/random");
    const randomExpansionJSONResponse = await randomExpansionResponse.json();

    document.querySelector(".results-expansions-list").innerHTML = randomExpansionJSONResponse["npmExpansion"];
}

async function searchExpansions(query) {
    const searchExpansionResponse = await fetch(`/api/search?query=${query.target.value}`);
    const searchExpansionJSONResponse = await searchExpansionResponse.json();
    
    const textareaString = searchExpansionJSONResponse.reduce("", (acc, expansion) => acc + (expansion + "\n"));
    document.querySelector(".results-expansions-list").innerHTML = textareaString;
}

async function loadAllExpansions() {
    const allExpansionResponse = await fetch("/api/all");
    const allExpansionJSONResponse = await allExpansionResponse.json();

    const textareaString = allExpansionJSONResponse.reduce("", (acc, expansion) => acc + (expansion + "\n"));
    document.querySelector(".results-expansions-list").innerHTML = textareaString;
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
    searchTerm => searchExpansions(searchTerm), // an expensive function
    100
);

generateRandomExpansion();
loadExpansionIntoTextArea();

const input = document.querySelector(".results-expansions-list");
input.addEventListener("input", debouncedSearchState);
