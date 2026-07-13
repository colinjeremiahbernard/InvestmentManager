const token = localStorage.getItem("token");

if (!token) {
    window.location = "/login";
}

const headers = {
    Authorization: `Bearer ${token}`
};

let chart = null;

async function loadDashboard() {

    try {

        const response = await fetch(
            "/api/dashboard",
            {
                headers
            }
        );

        if (!response.ok)
            throw new Error("Unable to load dashboard.");

        const dashboard = await response.json();

        document.getElementById("portfolioValue").textContent =
            "$" + Number(dashboard.total_value).toFixed(2);

        document.getElementById("totalInvested").textContent =
            "$" + Number(dashboard.total_invested).toFixed(2);

        document.getElementById("gainLoss").textContent =
            "$" + Number(dashboard.total_gain_loss).toFixed(2);

        document.getElementById("returnPct").textContent =
            Number(dashboard.total_gain_loss_pct).toFixed(2) + "%";

        renderChart(dashboard);

        renderRecentTransactions(
            dashboard.recent_transactions ?? []
        );

    }
    catch (err) {

        console.error(err);

    }

}

function renderChart(dashboard) {

    const ctx =
        document
            .getElementById("portfolioChart")
            .getContext("2d");

    if (chart)
        chart.destroy();

    chart = new Chart(ctx, {

        type: "doughnut",

        data: {

            labels: [
                "Invested",
                "Gain"
            ],

            datasets: [{

                data: [
                    dashboard.total_invested,
                    Math.max(
                        dashboard.total_gain_loss,
                        0
                    )
                ]

            }]

        },

        options: {

            responsive: true,

            plugins: {

                legend: {

                    position: "bottom"

                }

            }

        }

    });

}

function renderRecentTransactions(items) {

    const container =
        document.getElementById(
            "recentTransactions"
        );

    container.innerHTML = "";

    if (items.length === 0) {

        container.innerHTML = `

<div class="list-group-item text-muted">

No recent transactions.

</div>

`;

        return;

    }

    items.forEach(item => {

        container.innerHTML += `

<div class="list-group-item">

<div class="fw-bold">

${item.asset_symbol}

<span class="badge bg-${
item.transaction_type === "Buy"
? "success"
: "danger"
}">

${item.transaction_type}

</span>

</div>

<div>

${item.quantity}
@
$${Number(item.price).toFixed(2)}

</div>

<small class="text-muted">

${new Date(
item.created_at
).toLocaleDateString()}

</small>

</div>

`;

    });

}

loadDashboard();