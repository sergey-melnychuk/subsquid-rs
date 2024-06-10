subsquid-rs
===========

Rust wrapper over [Subsquid EVM API](https://docs.subsquid.io/subsquid-network/reference/evm-api/).

```
$ cargo run --example doc
...
ETH block: 20040963
{
  "header": {
    "number": 20039963,
    "hash": "0xeffd1c7b7793c070c75d7b52a25b1fd2153758fe7682c097eb6f63b4f6a77a49",
    "parentHash": "0x96df296389232b9c96f269d6ec616a807e112a00fa0b6e8c827be73d1e6cdba6"
  },
  "transactions": [
    {
      "hash": "0xbb202706456711bf9a9abea41c180da8f323968c89ee1d1abc56559ab360b725",
      "transactionIndex": 16
    },
    {
      "hash": "0xc612fdb1f18c2722919fe71e2abb31bd1ff8c7c4c8821295831f4d5c5c931bcc",
      "transactionIndex": 18
    },
    {
      "hash": "0x942bc00683cc2b37ac1caf34a28efb71ee8b1926171cf04200f3b76354781c30",
      "transactionIndex": 23
    },
    {
      "hash": "0x8ed8e5bc5bd2f031f249211cc72c3548cae24b6f4d31273b9c37b01d42de32eb",
      "transactionIndex": 25
    },
    {
      "hash": "0xaea00247213f6ee7b75f66f7ee4a79dba4d0ffd1a6fd1c274039e76988c87f4e",
      "transactionIndex": 29
    },
    {
      "hash": "0x814f208c949f3e9cf42f60d3895ab4295e2e3dc62b1e0e025ad7d1544ba5285e",
      "transactionIndex": 135
    },
    {
      "hash": "0xc5c90453214865f4edf4aedaef4f9b0e8500e087e39db06871308287df423039",
      "transactionIndex": 254
    },
    {
      "hash": "0x42e96e8e1c5560ab42ea63854a3fa5e98f16e5ff520216a3554b6b52bba306f8",
      "transactionIndex": 255
    },
    {
      "hash": "0xec1f0631339106197cdc03ea2ab994ce0e8315eabc644abf215d78a665bc87d8",
      "transactionIndex": 257
    },
    {
      "hash": "0x63e8b815e8614169d6423380140d9cb14fb14486ddbb5e81b0da73ab199bd421",
      "transactionIndex": 258
    },
    {
      "hash": "0x7cbd343805e5ed34c03c3aa7e522450df04182742112eec3dca669c9f1400c09",
      "transactionIndex": 260
    },
    {
      "hash": "0x8b75b17e268b0d9cd7a67a58cc341b384d9b6ab842bac34b5ebdb7b3db30c770",
      "transactionIndex": 263
    },
    {
      "hash": "0x646bea97c84654274cd16775f3727f0b6deded4a72bde6e6bc95b1d6d532af2b",
      "transactionIndex": 308
    },
    {
      "hash": "0x10489d86efee08acc6ebfb795ae8d2dae242f16e50b61f15c2e89d12d779fb1e",
      "transactionIndex": 361
    }
  ]
}
(Total 973 entries received)
Stream items count: 1898
```
