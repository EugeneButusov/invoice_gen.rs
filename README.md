# invoice_gen.rs
A simple tool to generate PDF invoices for your lovely customers.

## Prerequisites
To use this tool, please make sure you have [wkhtmltopdf](https://github.com/wkhtmltopdf/packaging/releases) installed on your machine.

## Installation
```
cargo install --git https://github.com/EugeneButusov/invoice_gen.rs.git
```

## Usage
```
invoice_gen --blueprint-path <BLUEPRINT_PATH> --invoiced-at <INVOICED_AT> --output <OUTPUT>
```

### Options:
```
-b, --blueprint-path <BLUEPRINT_PATH>    Blueprint file path
-h, --help                               Print help information
-i, --invoiced-at <INVOICED_AT>          Date invoice generated at
-o, --output <OUTPUT>                    Output invoice file
-V, --version                            Print version information
```

## Blueprint file format
Blueprint file is [TOML](https://toml.io/), following the format:
```toml
[contract]
date = 2022-01-01
currency = "$"
salary = 1234.56

[recipient]
data = """\
John Doe
1485 Riverside Drive, Augusta, Georgia
USA
TaxID 1234567890
Company ID 123456789012345
"""
payment_instructions = """\
Beneficiary institution: COOLBANK
SWIFT: XXXXXXXX
Beneficiary account: 123456789456123456 (USD)
Beneficiary: John Doe
"""

[payer]
data = """\
My Lovely Customer (Tax ID 123456789456123)
"""

[invoice]
signature = "John Doe"

```