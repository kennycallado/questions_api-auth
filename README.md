# questions_api-base

## Init a repo

### Extender plantilla

Acceder al repositorio en github y pulsar **Use this template**, **Create a new repository**. Una vez creado el repositorio puedes clonarlo localmente y continuar con el siguiente punto.

### Adaptar proyecto

Algunos ficheros deben ser revisados y actualizados para cada proyecto derivado de base. En general están listados en la siguiente lista.

#### Raíz del proyecto

- [ ] .env
  - Dirección de la base de datos
- [ ] Cargo.toml
  - Nombre del paquete
  - Revisar dependencias
- [ ] compose.yaml
  - Variables de entorno
  - Servicios extra
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

- [ ] Improve the README
- [X] Implementar origin_url from config
- [X] Implementar parámetro de configuración, migrations_run
- [ ] Nuevo sistema para check claims
- [X] Auto compile and build
