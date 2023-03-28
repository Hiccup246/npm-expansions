async function generateRandomExpansion() {
    const randomExpansionResponse = await fetch("/api/random");
    const randomExpansionJSONResponse = await randomExpansionResponse.json();

    document.querySelector(".npm-expansion").innerHTML = randomExpansionJSONResponse["npmExpansion"];
}

generateRandomExpansion();