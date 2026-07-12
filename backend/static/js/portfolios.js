const token = localStorage.getItem("token");

if (!token) {
    location = "/login";
}

async function loadPortfolios() {

    try {

        const response = await fetch("/api/portfolios", {

            headers: {
                Authorization: `Bearer ${token}`
            }

        });

        if (!response.ok)
            throw new Error("Unable to load portfolios.");

        const portfolios = await response.json();

        const container =
            document.getElementById("portfolioContainer");

        container.innerHTML = "";

        if (portfolios.length === 0) {

            container.innerHTML = `

<div class="col-12">

<div class="alert alert-info">

You haven't created any portfolios yet.

</div>

</div>

`;

            return;

        }

        portfolios.forEach(p => {

            container.innerHTML += `

<div class="col-lg-4 col-md-6">

<div class="card shadow-sm h-100">

<div class="card-body">

<h5 class="card-title">

${p.name}

</h5>

<p class="text-muted">

${p.description ?? ""}

</p>

<hr>

<div class="d-grid gap-2">

<a
href="/portfolio/${p.id}"
class="btn btn-primary">

Open Portfolio

</a>

<button
class="btn btn-outline-secondary"
onclick="editPortfolio('${p.id}')">

Edit

</button>

<button
class="btn btn-outline-danger"
onclick="deletePortfolio('${p.id}')">

Delete

</button>

</div>

</div>

</div>

</div>

`;

        });

    }

    catch(err) {

        showAlert(err.message,"danger");

    }

}

document
.getElementById("portfolioForm")
.addEventListener("submit", async function(e){

    e.preventDefault();

    const response = await fetch("/api/portfolios",{

        method: editingPortfolio ? "PUT" : "POST",

        headers:{
            "Content-Type":"application/json",
            Authorization:`Bearer ${token}`
        },

        body:JSON.stringify({

            name:portfolioName.value,

            description:portfolioDescription.value

        })

    });

    if(!response.ok){

        alert("Unable to create portfolio.");

        return;

    }

    bootstrap.Modal
        .getInstance(document.getElementById("portfolioModal"))
        .hide();

    document.getElementById("portfolioForm").reset();

    editingPortfolio = null;

    document.getElementById(
    "portfolioFormTitle"
       ).innerText = "Create Portfolio";

    document.getElementById(
         "savePortfolioButton"
       ).innerText = "Save Portfolio";

    loadPortfolios();

});

loadPortfolios();
let editingPortfolio = null;

// ---------------------------
// Edit Portfolio
// ---------------------------

async function editPortfolio(id) {

    try {

        const response = await fetch(
            editingPortfolio ?
            `/api/portfolios/${editingPortfolio}`
            : "/api/portfolios", {
            headers: {
                Authorization: `Bearer ${token}`
            }
        });

        if (!response.ok)
            throw new Error("Unable to load portfolio.");

        const portfolio = await response.json();

        editingPortfolio = id;

        portfolioName.value = portfolio.name;
        portfolioDescription.value =
            portfolio.description ?? "";

        document.getElementById(
            "portfolioFormTitle"
        ).innerText = "Edit Portfolio";

        document.getElementById(
            "savePortfolioButton"
        ).innerText = "Update Portfolio";

        bootstrap.Modal
            .getOrCreateInstance(
                document.getElementById("portfolioModal")
            )
            .show();

    }
    catch(err){

        showAlert(err.message,"danger");

    }

}

// ---------------------------
// Delete Portfolio
// ---------------------------

async function deletePortfolio(id){

    if(!confirm("Delete this portfolio?"))
        return;

    try{

        const response = await fetch(
            `/api/portfolios/${id}`,
            {
                method:"DELETE",
                headers:{
                    Authorization:`Bearer ${token}`
                }
            }
        );

        if(!response.ok)
            throw new Error(
                "Unable to delete portfolio."
            );

        loadPortfolios();

        showAlert(
            "Portfolio deleted.",
            "success"
        );

    }
    catch(err){

        showAlert(err.message,"danger");

    }

}

// ---------------------------
// Alert
// ---------------------------

function showAlert(message,type){

    const container =
        document.getElementById("alertContainer");

    if(!container){

        alert(message);

        return;

    }

    container.innerHTML =

`

<div class="alert alert-${type} alert-dismissible fade show">

${message}

<button
class="btn-close"
data-bs-dismiss="alert">

</button>

</div>

`;

    setTimeout(()=>{

        container.innerHTML="";

    },4000);

}