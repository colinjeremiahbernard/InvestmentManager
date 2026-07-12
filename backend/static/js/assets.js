// ===============================
// Investment Manager - Assets
// ===============================

const API = "/api/assets";

const token = localStorage.getItem("token");

if (!token) {
    window.location = "/login";
}

const headers = {
    "Authorization": `Bearer ${token}`,
    "Content-Type": "application/json"
};

let assets = [];
let editingAsset = null;

// -------------------------------
// Initialization
// -------------------------------

document.addEventListener("DOMContentLoaded", () => {

    loadAssets();

    const createForm = document.getElementById("assetForm");

    if (createForm) {
        createForm.addEventListener("submit", saveAsset);
    }

    const search = document.getElementById("assetSearch");

    if (search) {
        search.addEventListener("keyup", filterAssets);
    }

});

// -------------------------------
// Load Assets
// -------------------------------

async function loadAssets() {

    try {

        const response = await fetch(API, {
            headers
        });

        if (!response.ok)
            throw new Error("Unable to load assets.");

        assets = await response.json();

        renderAssets(assets);

    }

    catch (err) {

        showAlert(err.message, "danger");

    }

}

// -------------------------------
// Render Table
// -------------------------------

function renderAssets(data) {

    const tbody = document.getElementById("assetsTableBody");

    if (!tbody) return;

    tbody.innerHTML = "";

    if (data.length === 0) {

        tbody.innerHTML =
        `
        <tr>

            <td colspan="8" class="text-center text-muted">

                No assets found.

            </td>

        </tr>
        `;

        return;

    }

    data.forEach(asset => {

        tbody.innerHTML += `

<tr>

<td>${asset.symbol}</td>

<td>${asset.name}</td>

<td>${asset.asset_type}</td>

<td>${asset.exchange}</td>

<td>${asset.currency}</td>

<td>$${Number(asset.current_price).toFixed(2)}</td>

<td>

<span class="badge ${asset.is_active ? "bg-success" : "bg-secondary"}">

${asset.is_active ? "Active" : "Inactive"}

</span>

</td>

<td>

<button
class="btn btn-sm btn-primary me-1"
onclick="editAsset('${asset.id}')">

Edit

</button>

<button
class="btn btn-sm btn-danger"
onclick="deleteAsset('${asset.id}')">

Delete

</button>

</td>

</tr>

`;

    });

}

// -------------------------------
// Search
// -------------------------------

function filterAssets() {

    const text = document
        .getElementById("assetSearch")
        .value
        .toLowerCase();

    const filtered = assets.filter(a =>

        a.symbol.toLowerCase().includes(text) ||

        a.name.toLowerCase().includes(text) ||

        a.asset_type.toLowerCase().includes(text) ||

        a.exchange.toLowerCase().includes(text)

    );

    renderAssets(filtered);

}

// -------------------------------
// Create / Update
// -------------------------------

async function saveAsset(e) {

    e.preventDefault();

    const payload = {

        symbol:
            document.getElementById("symbol").value,

        name:
            document.getElementById("name").value,

        asset_type:
            document.getElementById("assetType").value,

        exchange:
            document.getElementById("exchange").value,

        currency:
            document.getElementById("currency").value,

        current_price:
            parseFloat(
                document.getElementById("currentPrice").value
            )

    };

    try {

        let response;

        if (editingAsset) {

            response = await fetch(

                `${API}/${editingAsset}`,

                {
                    method: "PUT",
                    headers,
                    body: JSON.stringify(payload)
                }

            );

        }

        else {

            response = await fetch(

                API,

                {
                    method: "POST",
                    headers,
                    body: JSON.stringify(payload)
                }

            );

        }

        if (!response.ok) {

            const message = await response.text();

            throw new Error(message);

        }

        resetForm();

        await loadAssets();

        showAlert(
            editingAsset
                ? "Asset updated."
                : "Asset created.",
            "success"
        );

    }

    catch (err) {

        showAlert(err.message, "danger");

    }

}
// -------------------------------
// Edit Asset
// -------------------------------

async function editAsset(id) {

    try {

        const response = await fetch(`${API}/${id}`, {
            headers
        });

        if (!response.ok)
            throw new Error("Unable to load asset.");

        const asset = await response.json();

        editingAsset = id;

        document.getElementById("symbol").value = asset.symbol;
        document.getElementById("name").value = asset.name;
        document.getElementById("assetType").value = asset.asset_type;
        document.getElementById("exchange").value = asset.exchange;
        document.getElementById("currency").value = asset.currency;
        document.getElementById("currentPrice").value = asset.current_price;

        const title = document.getElementById("assetFormTitle");

        if (title)
            title.textContent = "Edit Asset";

        const button = document.getElementById("saveAssetButton");

        if (button)
            button.textContent = "Update Asset";

        if (typeof bootstrap !== "undefined") {

            const modalElement = document.getElementById("assetModal");

            if (modalElement) {
                bootstrap.Modal
                    .getOrCreateInstance(modalElement)
                    .show();
            }
        }

    }
    catch (err) {

        showAlert(err.message, "danger");

    }

}

// -------------------------------
// Delete Asset
// -------------------------------

async function deleteAsset(id) {

    if (!confirm("Delete this asset?"))
        return;

    try {

        const response = await fetch(`${API}/${id}`, {
            method: "DELETE",
            headers
        });

        if (!response.ok) {

            const message = await response.text();

            throw new Error(message);

        }

        await loadAssets();

        showAlert("Asset deleted.", "success");

    }
    catch (err) {

        showAlert(err.message, "danger");

    }

}

// -------------------------------
// Reset Form
// -------------------------------

function resetForm() {

    editingAsset = null;

    const form = document.getElementById("assetForm");

    if (form)
        form.reset();

    const title = document.getElementById("assetFormTitle");

    if (title)
        title.textContent = "Create Asset";

    const button = document.getElementById("saveAssetButton");

    if (button)
        button.textContent = "Save Asset";

    if (typeof bootstrap !== "undefined") {

        const modalElement = document.getElementById("assetModal");

        if (modalElement) {

            bootstrap.Modal
                .getOrCreateInstance(modalElement)
                .hide();

        }

    }

}

// -------------------------------
// Alerts
// -------------------------------

function showAlert(message, type = "success") {

    const container = document.getElementById("alertContainer");

    if (!container) {

        alert(message);

        return;

    }

    container.innerHTML = `

<div class="alert alert-${type} alert-dismissible fade show">

${message}

<button
type="button"
class="btn-close"
data-bs-dismiss="alert">
</button>

</div>

`;

    setTimeout(() => {

        container.innerHTML = "";

    }, 4000);

}