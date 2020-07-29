window.addEventListener("load", function() {
    if (localStorage.getItem("data-theme")) {
        document.documentElement.setAttribute("data-theme", localStorage.getItem("data-theme"));
    }
    let overlay = document.getElementById("info-popup-overlay");
    overlay.addEventListener("click", function(e) {
        if (e.target == overlay) {
            closePopup();
        }
    });
    let stored = localStorage.getItem("popup");
    if (stored != null) {
        let last_opened = Date.parse(stored);
        // If it's been 3 hours, reopen the popup
        if (Date.now() - last_opened > 3 * 60 * 60 * 1000) {
            document.getElementById("info-popup-overlay").classList.remove("hidden");
        }
    } else {
        document.getElementById("info-popup-overlay").classList.remove("hidden");
    }
});

function closePopup() {
    document.getElementById("info-popup-overlay").classList.add("hidden");
    localStorage.setItem("popup", (new Date()).toISOString());
}
