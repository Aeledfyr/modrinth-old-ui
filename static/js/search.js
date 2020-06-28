let input = document.getElementById("search-input");

let category_inputs = {
    "technology": false,
    "adventure": false,
    "magic": false,
    "utility": false,
    "decoration": false,
    "library": false,
    "worldgen": false,
    "cursed": false,
    "storage": false,
    "food": false,
    "equipment": false,
    "misc": false,
    "forge": false,
    "fabric": false,
}
let version_inputs = {};
let selectedType  = "relevance";

let resultContainer = document.getElementById("results");

window.onload = function () {
    let categories = document.getElementsByClassName("category-checkbox");

    for (let category of categories) {
        category.addEventListener("change", function(event) {
            // Remove "category-" from id
            let id = event.target.id.substring(9);
            category_inputs[id] = event.target.checked;
            handleSearch(0);
        })
    }

    //Set Initial Values based on URL
    const urlParams = new URLSearchParams(window.location.search);

    if(urlParams.has("q"))
        document.getElementById("search-input").value = decodeURIComponent(urlParams.get("q"));

    if(urlParams.has("f")) {
        let value = decodeURIComponent(urlParams.get("f"));

        for (let key of Object.keys(category_inputs)) {
            if(value.includes(key)) {
                let checkbox = document.getElementById("checkbox-" + key);
                category_inputs[key] = true;
                checkbox.checked = true;
            }
        }
    }

    if(urlParams.has("s")) {
        let value = decodeURIComponent(urlParams.get("s"));

        selectedType = value;
        document.getElementById("filters").value = value;
    }

    let urlVersions = [];

    if(urlParams.has("v")) {
        let versionsString = urlParams.get("v");

        versionsString = versionsString.replace(/ /g, '');
        versionsString = versionsString.replace(/versions=/g, '');

        urlVersions = versionsString.split("OR")
    }

    // Set Version categories from Mojang Launcher Meta

    let releases = document.getElementById("releases");
    let snapshots = document.getElementById("snapshots");
    let archaic = document.getElementById("archaic");

    let xmlHttp = new XMLHttpRequest();

    xmlHttp.onreadystatechange = function() {
        if (xmlHttp.readyState === 4 && xmlHttp.status === 200) {
            let versions = JSON.parse(xmlHttp.responseText);

            for (let version of versions.versions) {
                let versionElement = document.createElement('p');
                versionElement.className = "version";
                versionElement.textContent = version.id;
                versionElement.id = version.id;
                versionElement.addEventListener("click", function() { activateVersion(versionElement) });

                version_inputs[version.id] = false;

                if(version.type === "release")
                    releases.appendChild(versionElement)
                else if (version.type === "snapshot")
                    snapshots.appendChild(versionElement)
                else if (version.type === "old_alpha" || version.type === "old_beta")
                    archaic.appendChild(versionElement)

                if(urlVersions.includes(version.id)) {
                    activateVersion(versionElement);
                }
            }
        }
    }

    xmlHttp.open("GET", "https://launchermeta.mojang.com/mc/game/version_manifest.json", true);
    xmlHttp.send(null);

}

function clearFilters() {
    for (let key in category_inputs) {
        if (category_inputs.hasOwnProperty(key)) {
            if(category_inputs[key]) {
                let checkbox = document.getElementById("checkbox-" + key);
                category_inputs[key] = false;
                checkbox.checked = false;
            }
        }
    }

    for (let key in version_inputs) {
        if (version_inputs.hasOwnProperty(key)) {
            if(version_inputs[key]) {
                let element = document.getElementById(key);
                element.classList.remove("version-active");
                version_inputs[key] = false;
            }
        }
    }

    handleSearch(0);
}

function activateVersion(element) {
    version_inputs[element.id] = !version_inputs[element.id]
    element.classList.toggle("version-active");
    handleSearch(0);
}

function changeSortType(element) {
    selectedType = element.options[element.selectedIndex].value;

    handleSearch(0);
}

let body = document.documentElement;
let backToTop = document.getElementById("backToTop");

let currentlyLoadingExtra = false;
let currentOffset = 0;

function loadExtra() {
    if (body.scrollTop > 400) {
        backToTop.style.display = "block";
    } else {
        backToTop.style.display = "none";
    }


    if(!currentlyLoadingExtra) {
        let scrollOffset = (body.scrollTop) / (body.scrollHeight - body.clientHeight);

        if(scrollOffset > 0.9) {
            currentOffset += 10;
            handleSearch(currentOffset);
            currentlyLoadingExtra = true;
        }
    }
}

function handleSearch(index) {
    let queryString = "search";

    if(input.value.length > 0) {
        queryString += "?q=" + encodeURIComponent(input.value).replace(/%20/g,'+');
    }

    let filterString = "";
    let versionString = "";

    for (let key in category_inputs) {
        if (category_inputs.hasOwnProperty(key)) {

            if(category_inputs[key])
                filterString += key + " AND keywords=";
        }
    }

    let filterTakeOffLength = " AND keywords=".length;

    if(filterString.length > filterTakeOffLength) {
        filterString = filterString.substring(0, filterString.length - filterTakeOffLength)
        queryString += "&f=" + encodeURIComponent( "keywords=" + filterString).replace(/%20/g,'+');
    }

    for (let key in version_inputs)
        if (version_inputs.hasOwnProperty(key))
            if(version_inputs[key])
                versionString += key + " OR versions=";

    let versionTakeOffLength = " OR versions=".length;

    if(versionString.length > versionTakeOffLength) {
        versionString = versionString.substring(0, versionString.length - versionTakeOffLength)
        queryString += "&v=" + encodeURIComponent( "versions=" + versionString).replace(/%20/g,'+');
    }

    if(selectedType)
        queryString += "&s=" + encodeURIComponent(selectedType).replace(/%20/g,'+');


    if(!queryString.includes("?"))
        queryString = queryString.replace("&", "?")

    if(index === 0) {
        let viewString = queryString;

        viewString = viewString.replace("?q={}{}{}", "");
        viewString = viewString.replace("&s=relevance", "");

        if(!viewString.includes("?"))
            viewString = viewString.replace("&", "?")

        window.history.pushState('Search', 'Search', viewString);
    }
    else
        queryString += "&o=" + index;

    let xmlHttp = new XMLHttpRequest();

    xmlHttp.onreadystatechange = function() {
        if (xmlHttp.readyState === 4 && xmlHttp.status === 200) {
            if(index === 0) {
                resultContainer.innerHTML = xmlHttp.responseText;

                currentOffset = 0;
                currentlyLoadingExtra = false;
            }
            else {
                resultContainer.innerHTML += xmlHttp.responseText;
                currentlyLoadingExtra = false;
            }
        }
    }

    xmlHttp.overrideMimeType("text/plain");
    xmlHttp.open("POST", queryString, true);
    xmlHttp.send(null);
}