<p align="center">
  <img src="https://saas.ecomenuapp.com/public/ecomenu-logo.jpg" width="200" alt="Ecomenu Logo" style="box-shadow: 0 4px 12px rgba(0,0,0,0.2); opacity: 0.9; border-radius: 12px;" />
</p>

<h3 align="center">Ecomenu Printer</h3>
<p align="center">Native application written in Rust to manage printers through HTTP requests. It’s designed specifically to be integrated with the software Ecomenu, allowing us to print in an efficient and scalable way.</p>

## Features

- 🖨️ Management of local and network printers.
- 🌐 HTTP API to manage print requests.
- ⚡️ Enhanced performance thanks to Rust.
- 🔄 Multiplatform: Windows, iOS, and GNU.
- 🍔 Optimized for integration with **Ecomenu**.

## Endpoints

| Method |    Endpoint     |               Description                |  Required Form-Data   |
| :----: | :-------------: | :--------------------------------------: | :-------------------: |
|  GET   | `/printer-list` | Lists printers configured in the system. |         None          |
|  POST  |    `/print`     |                Prints PDF                | `printer_name`, `pdf` |
