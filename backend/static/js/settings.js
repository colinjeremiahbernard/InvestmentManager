const token = localStorage.getItem("token");

if (!token) {
    location = "/login";
}

const headers = {
    Authorization: `Bearer ${token}`,
    "Content-Type": "application/json"
};

async function loadProfile() {

    const response = await fetch("/api/me", {
        headers
    });

    if (!response.ok) {
        location = "/login";
        return;
    }

    const user = await response.json();

    firstName.value = user.first_name;
    lastName.value = user.last_name;
    username.value = user.username;
    email.value = user.email;
}

document
.getElementById("settingsForm")
.addEventListener("submit", async function(e){

    e.preventDefault();

    const response = await fetch("/api/me",{

        method:"PUT",

        headers,

        body:JSON.stringify({

            first_name:firstName.value,
            last_name:lastName.value,
            username:username.value,
            email:email.value

        })

    });

    if(!response.ok){

        alert("Unable to update profile.");

        return;

    }

    alert("Settings updated successfully.");

});

loadProfile();