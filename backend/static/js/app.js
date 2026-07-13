document.addEventListener("DOMContentLoaded", () => {

    const logout = document.getElementById("logoutLink");

    if (!logout)
        return;

    logout.addEventListener("click", e => {

        e.preventDefault();

        localStorage.removeItem("token");

        window.location = "/login";

    });

});