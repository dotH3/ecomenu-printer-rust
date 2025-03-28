# ecomenu-printer

Aplicación nativa escrita en Rust para manejar impresoras a través de solicitudes HTTP. Está diseñada específicamente para integrarse con el SaaS **Ecomenu**, permitiendo realizar impresiones de manera eficiente y escalable.

## Características

- 🖨️ Manejo de impresoras locales y en red.
- 🌐 API HTTP para gestionar solicitudes de impresión.
- ⚡️ Mayor rendimiento gracias a Rust.
- 🔄 Multiplataforma: Windows, iOS y GNU.
- 🍔 Optimizado para integrarse con **Ecomenu**
  
## Endpoints

- **GET /printer-list**: lista de impresoras configuradas en el sistema.
- **POST /print**: imprimir HTML (requiere parámetros: printerName, height, width, zoom).
