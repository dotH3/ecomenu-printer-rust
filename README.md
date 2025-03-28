# ecomenu-printer

**ecomenu-printer** es una aplicación nativa escrita en Rust para manejar impresoras a través de solicitudes HTTP. Esta versión alternativa es multiplataforma, soportando Windows, iOS y GNU, y ofrece un mayor rendimiento. Está diseñada específicamente para integrarse con el SaaS **Ecomenu**, permitiendo realizar impresiones de manera eficiente y escalable.

## Características

- 🖨️ Manejo de impresoras locales y en red.
- 🌐 API HTTP para gestionar solicitudes de impresión.
- 🍔 Optimizado para integrarse con **Ecomenu** y operar en sistemas multiplataforma.
- ⚡️ Mayor rendimiento gracias a la implementación en Rust.
- 🔄 Multiplataforma: soporta Windows, iOS y GNU.
  
## Endpoints

- **GET /printer-list**: lista de impresoras configuradas en el sistema.
- **POST /print**: imprimir HTML (requiere parámetros: printerName, height, width, zoom).
