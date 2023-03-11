# questions_api-base

## Init a repo

### Extender plantilla

### Adaptar proyecto

#### Raíz del proyecto

- [ ] .env
  - Dirección de la base de datos
- [ ] Cargo.toml
  - Nombre del paquete
  - Revisar dependencias
- [ ] compose.yaml
  - Variables de entorno
  - Servicios extra
- [ ] Containerfile
  - Nombre del binario, viene desde Cargo.toml
- [ ] Rocket.toml
  - Parámetros de configuración del proyecto

#### Directorio src

- [ ] Tests

#### Migraciones

Cada api tiene sus propias migraciones localizadas en el directorio `src/database/migrations`

#### Modules

Directorio principal de trabajo de cada api. Contendrá un módulo por cada entidad con la que trabaje la api y administrará sus rustas.

#### Module

Cada módulo deberá contener, `model.rs` y `controller.rs`. En caso de ser necesario el controlador puede ayudarse de un directorio `handlers` y el modelo puede tener un repositorio dentro del directorio `services`.

El directorio de servicios del módulo también puede contener por ejemplo, `helpers` para el controlador o implementación de `claims` para entidad user.

## Update a repo


## TODO:

Implementar origin_url from config
