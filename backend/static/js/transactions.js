const token = localStorage.getItem("token");

if (!token) {
    window.location = "/login";
}

const headers = {
    Authorization: `Bearer ${token}`,
    "Content-Type": "application/json"
};

let editingTransaction = null;

async function loadTransactions() {

    try {

        const response = await fetch(
            "/api/transactions",
            { headers }
        );

        if (!response.ok)
            throw new Error("Unable to load transactions.");

        const transactions = await response.json();

        const table =
            document.getElementById("transactionsTable");

        table.innerHTML = "";

        if (transactions.length === 0) {

            table.innerHTML = `

<tr>

<td colspan="8" class="text-center text-muted">

No transactions found.

</td>

</tr>

`;

            return;

        }

        transactions.forEach(transaction => {

            table.innerHTML += `

<tr>

<td>${new Date(
transaction.created_at
).toLocaleDateString()}</td>

<td>${transaction.portfolio_name ?? "-"}</td>

<td>${transaction.asset_symbol ?? "-"}</td>

<td>

<span class="badge ${
transaction.transaction_type === "Buy"
? "bg-success"
: "bg-danger"
}">

${transaction.transaction_type}

</span>

</td>

<td>${transaction.quantity}</td>

<td>$${Number(
transaction.price
).toFixed(2)}</td>

<td>$${Number(
transaction.amount
).toFixed(2)}</td>

<td>

<button
class="btn btn-sm btn-outline-secondary me-1"
onclick="editTransaction('${transaction.id}')">

Edit

</button>

<button
class="btn btn-sm btn-outline-danger"
onclick="deleteTransaction('${transaction.id}')">

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

async function loadPortfolios() {

    const response = await fetch(
        "/api/portfolios",
        { headers }
    );

    if (!response.ok)
        return;

    const portfolios = await response.json();

    const select =
        document.getElementById("portfolioSelect");

    select.innerHTML = "";

    portfolios.forEach(portfolio => {

        select.innerHTML += `

<option value="${portfolio.id}">

${portfolio.name}

</option>

`;

    });

}

async function loadAssets() {

    const response = await fetch(
        "/api/assets",
        { headers }
    );

    if (!response.ok)
        return;

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

document
.getElementById("transactionForm")
.addEventListener(
    "submit",
    saveTransaction
);
async function saveTransaction(e) {

    e.preventDefault();

    const quantity = parseFloat(
        document.getElementById("quantity").value
    );

    const price = parseFloat(
        document.getElementById("price").value
    );

    const body = {
        portfolio_id: document.getElementById("portfolioSelect").value,
        asset_id: document.getElementById("assetSelect").value,
        transaction_type: document.getElementById("transactionType").value,
        quantity,
        price,
        amount: quantity * price,
        notes: document.getElementById("notes").value
    };

    const url = editingTransaction
        ? `/api/transactions/${editingTransaction}`
        : "/api/transactions";

    const method = editingTransaction ? "PUT" : "POST";

    try {

        const response = await fetch(url, {
            method,
            headers,
            body: JSON.stringify(body)
        });

        if (!response.ok)
            throw new Error("Unable to save transaction.");

        bootstrap.Modal
            .getOrCreateInstance(
                document.getElementById("transactionModal")
            )
            .hide();

        resetForm();

        await loadTransactions();

        showAlert(
            editingTransaction
                ? "Transaction updated."
                : "Transaction created.",
            "success"
        );

    } catch (err) {

        showAlert(err.message, "danger");

    }

}

async function editTransaction(id) {

    try {

        const response = await fetch(
            `/api/transactions/${id}`,
            { headers }
        );

        if (!response.ok)
            throw new Error("Unable to load transaction.");

        const transaction = await response.json();

        editingTransaction = id;

        document.getElementById("portfolioSelect").value =
            transaction.portfolio_id;

        document.getElementById("assetSelect").value =
            transaction.asset_id;

        document.getElementById("transactionType").value =
            transaction.transaction_type;

        document.getElementById("quantity").value =
            transaction.quantity;

        document.getElementById("price").value =
            transaction.price;

        document.getElementById("notes").value =
            transaction.notes ?? "";

        document.getElementById(
            "transactionModalTitle"
        ).textContent = "Edit Transaction";

        bootstrap.Modal
            .getOrCreateInstance(
                document.getElementById("transactionModal")
            )
            .show();

    } catch (err) {

        showAlert(err.message, "danger");

    }

}

async function deleteTransaction(id) {

    if (!confirm("Delete this transaction?"))
        return;

    try {

        const response = await fetch(
            `/api/transactions/${id}`,
            {
                method: "DELETE",
                headers
            }
        );

        if (!response.ok)
            throw new Error("Unable to delete transaction.");

        await loadTransactions();

        showAlert(
            "Transaction deleted.",
            "success"
        );

    } catch (err) {

        showAlert(err.message, "danger");

    }

}

function resetForm() {

    editingTransaction = null;

    document
        .getElementById("transactionForm")
        .reset();

    document.getElementById(
        "transactionModalTitle"
    ).textContent = "New Transaction";

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

document
    .getElementById("quantity")
    .addEventListener("input", updateAmount);

document
    .getElementById("price")
    .addEventListener("input", updateAmount);

function updateAmount() {

    const quantity =
        parseFloat(document.getElementById("quantity").value) || 0;

    const price =
        parseFloat(document.getElementById("price").value) || 0;

    // Reserved for future UI display of the calculated amount.
    quantity * price;

}

loadPortfolios();
loadAssets();
loadTransactions();