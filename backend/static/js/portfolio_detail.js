const token = localStorage.getItem("token");

const id =
window.location.pathname.split("/").pop();

async function loadPortfolio(){

const response=await fetch(`/api/portfolios/${id}`,{

headers:{
Authorization:`Bearer ${token}`
}

});

const portfolio=await response.json();

portfolioName.innerHTML=portfolio.name;

portfolioDescription.innerHTML=
portfolio.description ?? "";

totalInvested.innerHTML=
"$"+portfolio.total_invested.toFixed(2);

currentValue.innerHTML=
"$"+portfolio.total_value.toFixed(2);

gainLoss.innerHTML=
"$"+portfolio.total_gain_loss.toFixed(2);

gainPct.innerHTML=
portfolio.total_gain_loss_pct.toFixed(2)+"%";

portfolioItems.innerHTML="";

portfolio.items.forEach(item=>{

portfolioItems.innerHTML+=`

<tr>

<td>${item.asset_symbol}</td>

<td>${item.asset_name}</td>

<td>${item.quantity}</td>

<td>$${item.purchase_price.toFixed(2)}</td>

<td>$${item.current_price.toFixed(2)}</td>

<td>$${item.gain_loss.toFixed(2)}</td>

</tr>

`;

});

}

loadPortfolio();