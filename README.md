# Linux Metrics Collector
A metrics collection application for Linux machines. Created for MSCS 710 Software Project at Marist College.

## Development Environment
This section discusses the development tools and the steps required to set them up to make changes to the project.
### Required
* Device running Linux or a Linux virtual machine
  * Options for VMs include [Oracle VirtualBox](https://www.virtualbox.org/), [VMware](https://www.vmware.com/), and others
* [Rust](https://www.rust-lang.org/tools/install) programming language
### Recommended
* [IntelliJ IDEA Community Edition](https://www.jetbrains.com/idea/download/#section=windows) with Rust and TOML plugins
  * File -> Settings -> Plugins
  * Search for "Rust" and click "Install"
  * When prompted to install "TOML", click Yes

## Run Application From IntelliJ
### Web Browser Run Configuration
* Open "~/.../Metrics-Collector/server/src" directory
* Select "main.rs" file in project hierarchy
* In the toolbar, click Run -> Edit -> Configurations
* Click "Add a new configuration" -> Cargo
* Name the configuration
* Set the working directory to "~/.../Metrics-Collector/server"
* Check the box that says "Run with root privileges"
* Select Cargo.toml file and click "Attach" in the top left of IntelliJ window
* Hit "Ok" and run the application
* View the application at http://127.0.0.1:8080

### Command Line Run Configuration
* Open "~/.../Metrics-Collector/server/src" directory
* Select "main.rs" file in project hierarchy
* In the toolbar, click Run -> Edit -> Configurations
* Click "Add a new configuration" -> Cargo
* Name the configuration
* Set the working directory to "~/.../Metrics-Collector/server"
* Check the box that says "Run with root privileges"
* For "Command" enter `run cli`
* Select Cargo.toml file and click "Attach" in the top left of IntelliJ window
* Hit "Ok" and run the application

## Run Application From Command Line
* From root directory: `cd server`
* `cargo build`
* To use command line interface version of app:
  * `sudo ./target/debug/server cli`
* To use gui version of app:
  * `sudo ./target/debug/server server`

## Interacting with the Database
* Running the server will automatically create a database if it is not there already.
  * It will appear in the 'metrics_collector_controllers' directory
* To query the database
  * Enter your terminal and cd into the metrics_collector_controllers directory
  * Enter the `sqlite3` command
  * Enter `.open data.db`
  * Query with "SELECT * FROM process;"
* Editing or building upon the database
  * Whenever the database must be changed or restarted, the 'data.db' file has to be deleted from the project folder
  * Rebuild it by re-running the server
  
## Docker
* Build Images 
  * `cd` into the 'server' directory
  * Server-Side:
    * Enter command: `sudo docker build . -t server:01`
* Create Containers
  * CLI Container:
    * From 'server', enter command: `sudo docker run -it --pid="host" --privileged --name myCLI server:01 cli`
  * HTTP Server Container:
    * From 'server', enter command: `sudo docker run -it --pid="host" --network="host" --privileged --name myServer \
                                          server:01 server`
