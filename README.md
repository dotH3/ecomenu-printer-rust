# ecomenu-printer

 Native application written in Rust to manage printers through HTTP requests. It’s designed specifically to be integrated with the software Ecomenu, allowing us to print in an efficient and scalable way.

## Features

- 🖨️ Management of local and network printers.
- 🌐 HTTP API to manage print requests.
- ⚡️ Enhanced performance thanks to Rust.
- 🔄 Multiplatform: Windows, iOS, and GNU.
- 🍔 Optimized for integration with **Ecomenu**.
- 📜 Requires Ghostscript for PDF processing.

## Endpoints

| Method | Endpoint         | Description                             | Required Form-Data                   |
|--------|------------------|-----------------------------------------|---------------------------------------|
| GET    | `/printer-list`  | Lists printers configured in the system.| None                                  |
| POST   | `/print`         | Prints HTML                             | `printer_name`, `pdf` |
