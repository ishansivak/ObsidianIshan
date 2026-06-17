# Bhejo: UPI-Saito Settlement Serialization Schema

## UPI Payment Request (Input)
```json
{
  "upi_transaction_id": "string",
  "amount_inr": "decimal",
  "payer_vpa": "string",
  "payee_vpa": "string",
  "timestamp": "ISO8601"
}
```

## Saito UTXO Transaction Payload (Output)
```json
{
  "saito_public_key": "string",
  "amount_saito": "decimal",
  "upi_ref": "string",
  "signature": "string",
  "timestamp": "ISO8601"
}
```

## Mapping Logic
- `amount_saito` = `amount_inr` * `exchange_rate`
- `upi_ref` = `upi_transaction_id`
- `signature` = `sign(upi_transaction_id + amount_saito + payer_vpa)`
