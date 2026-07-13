const token = localStorage.getItem("token");

if (!token) {
    window.location = "/login";
}

let chart = null;

async function loadReport() {

    const response = await fetch("/api/reports", {
        headers: {
            Authorization: `Bearer ${token}`
        }
    });

    if (!response.ok) {
        alert("Unable to load reports.");
        return;
    }

    const report = await response.json();

    document.getElementById("reportInvested").innerHTML =
        "$" + report.total_invested.toFixed(2);

    document.getElementById("reportValue").innerHTML =
        "$" + report.total_value.toFixed(2);

    document.getElementById("reportGain").innerHTML =
        "$" + report.total_gain_loss.toFixed(2);

    document.getElementById("reportReturn").innerHTML =
        report.total_gain_loss_pct.toFixed(2) + "%";

    drawChart(report.portfolios);
}

function drawChart(portfolios) {

    const ctx = document.getElementById("reportChart");

    if (chart) {
        chart.destroy();
    }

    chart = new Chart(ctx, {

        type: "bar",

        data: {

            labels: portfolios.map(p => p.name),

            datasets: [{

                label: "Portfolio Value",

                data: portfolios.map(p => p.value)

            }]

        },

        options: {

            responsive: true,

            plugins: {

                legend: {

                    display: false

                }

            }

        }

    });

}

loadReport();