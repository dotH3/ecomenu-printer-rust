# ecomenu-printer

 Native application written in Rust to manage printers through HTTP requests. It’s designed specifically to be integrated with the software Ecomenu, allowing us to print in an efficient and scalable way.

## Features

- 🖨️ Management of local and network printers.
- 🌐 HTTP API to manage print requests.
- ⚡️ Enhanced performance thanks to Rust.
- 🔄 Multiplatform: Windows, iOS, and GNU.
- 🍔 Optimized for integration with **Ecomenu**.

## Endpoints

- **GET /printer-list**: List of printers configured in the system.
- **POST /print**: Print HTML (requires parameters: printerName, height, width, zoom).

