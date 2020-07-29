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
});

function closePopup() {
    document.getElementById("info-popup").remove();
    document.getElementById("info-popup-overlay").remove();
}
