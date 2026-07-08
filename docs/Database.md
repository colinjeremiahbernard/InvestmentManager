# Database Design

## Users

```sql
id
username
email
password_hash
created_at
updated_at
```

---

## Assets

```sql
id
symbol
company_name
category
currency
current_price
```

---

## Portfolio

```sql
id
user_id
asset_id
quantity
purchase_price
purchase_date
```

---

## Transactions

```sql
id
portfolio_id
transaction_type
quantity
transaction_price
transaction_date
```

---

## Relationships

```text
Users
  │
  ├────────────┐
               │
           Portfolio
               │
               │
         Transactions
               │
               │
            Assets
```
