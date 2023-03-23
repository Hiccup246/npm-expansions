async function generateRandomExpansion() {
    const randomExpansionResponse = await fetch("/random");
    const randomExpansionJSONResponse = await randomExpansionResponse.json();

    document.querySelector(".npm-expansion").innerHTML = randomExpansionJSONResponse["npmExpansion"];
}

generateRandomExpansion();