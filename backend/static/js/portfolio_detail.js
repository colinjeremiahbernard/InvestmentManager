const token = localStorage.getItem("token");

if (!token) {
    window.location = "/login";
}

const portfolioId = window.location.pathname.split("/").pop();

let editingItem = null;

const headers = {
    Authorization: `Bearer ${token}`,
    "Content-Type": "application/json"
};

async function loadPortfolio() {

    try {

        const response = await fetch(
            `/api/portfolios/${portfolioId}`,
            {
                headers
            }
        );

        if (!response.ok)
            throw new Error("Unable to load portfolio.");

        const portfolio = await response.json();

        document.getElementById("portfolioName").textContent =
            portfolio.name;

        document.getElementById("portfolioDescription").textContent =
            portfolio.description ?? "";

        document.getElementById("totalInvested").textContent =
            "$" + portfolio.total_invested.toFixed(2);

        document.getElementById("currentValue").textContent =
            "$" + portfolio.total_value.toFixed(2);

        document.getElementById("gainLoss").textContent =
            "$" + portfolio.total_gain_loss.toFixed(2);

        document.getElementById("gainLossPct").textContent =
            portfolio.total_gain_loss_pct.toFixed(2) + "%";

        const tbody =
            document.getElementById("holdingsTable");

        tbody.innerHTML = "";

        portfolio.items.forEach(item => {

            tbody.innerHTML += `

<tr>

<td>${item.asset_symbol}</td>

<td>${item.asset_name}</td>

<td>${item.quantity}</td>

<td>$${item.purchase_price.toFixed(2)}</td>

<td>$${item.current_price.toFixed(2)}</td>

<td>$${item.total_invested.toFixed(2)}</td>

<td>$${item.current_value.toFixed(2)}</td>

<td class="${
item.gain_loss >= 0
? "text-success"
: "text-danger"
}">

$${item.gain_loss.toFixed(2)}

</td>

<td>

<button
class="btn btn-sm btn-outline-secondary me-1"
onclick="editHolding('${item.id}')">

Edit

</button>

<button
class="btn btn-sm btn-outline-danger"
onclick="deleteHolding('${item.id}')">

Delete

</button>

</td>

</tr>

`;

        });

    }
    catch (err) {

        showAlert(err.message, "danger");

    }

}

async function loadAssets() {

    try {

        const response = await fetch(
            "/api/assets",
            {
                headers
            }
        );

        if (!response.ok)
            throw new Error("Unable to load assets.");

        const assets = await response.json();

        const select =
            document.getElementById("assetSelect");

        select.innerHTML = "";

        assets.forEach(asset => {

            select.innerHTML += `

<option value="${asset.id}">

${asset.symbol} - ${asset.name}

</option>

`;

        });

    }
    catch (err) {

        showAlert(err.message, "danger");

    }

}

document
.getElementById("holdingForm")
.addEventListener("submit", saveHolding);
async function saveHolding(e) {

    e.preventDefault();

    const body = {
        asset_id: document.getElementById("assetSelect").value,
        quantity: parseFloat(document.getElementById("quantity").value),
        purchase_price: parseFloat(document.getElementById("purchasePrice").value),
        notes: ""
    };

    const url = editingItem
        ? `/api/portfolios/${portfolioId}/items/${editingItem}`
        : `/api/portfolios/${portfolioId}/items`;

    const method = editingItem ? "PUT" : "POST";

    try {

        const response = await fetch(url, {
            method,
            headers,
            body: JSON.stringify(body)
        });

        if (!response.ok)
            throw new Error("Unable to save holding.");

        bootstrap.Modal
            .getOrCreateInstance(
                document.getElementById("holdingModal")
            )
            .hide();

        resetForm();

        await loadPortfolio();

        showAlert(
            editingItem
                ? "Holding updated."
                : "Holding added.",
            "success"
        );

    }
    catch (err) {

        showAlert(err.message, "danger");

    }

}

async function editHolding(id) {

    try {

        const response = await fetch(
            `/api/portfolios/${portfolioId}`,
            {
                headers
            }
        );

        if (!response.ok)
            throw new Error("Unable to load holding.");

        const portfolio = await response.json();

        const item = portfolio.items.find(i => i.id === id);

        if (!item)
            throw new Error("Holding not found.");

        editingItem = id;

        document.getElementById("assetSelect").value =
            item.asset_id;

        document.getElementById("quantity").value =
            item.quantity;

        document.getElementById("purchasePrice").value =
            item.purchase_price;

        document.getElementById("holdingModalTitle").textContent =
            "Edit Holding";

        bootstrap.Modal
            .getOrCreateInstance(
                document.getElementById("holdingModal")
            )
            .show();

    }
    catch (err) {

        showAlert(err.message, "danger");

    }

}

async function deleteHolding(id) {

    if (!confirm("Delete this holding?"))
        return;

    try {

        const response = await fetch(
            `/api/portfolios/${portfolioId}/items/${id}`,
            {
                method: "DELETE",
                headers
            }
        );

        if (!response.ok)
            throw new Error("Unable to delete holding.");

        await loadPortfolio();

        showAlert(
            "Holding deleted.",
            "success"
        );

    }
    catch (err) {

        showAlert(err.message, "danger");

    }

}

function resetForm() {

    editingItem = null;

    document.getElementById("holdingForm").reset();

    document.getElementById("holdingModalTitle").textContent =
        "Add Holding";

}

function showAlert(message, type = "success") {

    const container =
        document.getElementById("alertContainer");

    if (!container) {

        alert(message);

        return;

    }

    container.innerHTML = `

<div class="alert alert-${type} alert-dismissible fade show">

${message}

<button
class="btn-close"
data-bs-dismiss="alert">
</button>

</div>

`;

    setTimeout(() => {

        container.innerHTML = "";

    }, 4000);

}

loadAssets();
loadPortfolio();