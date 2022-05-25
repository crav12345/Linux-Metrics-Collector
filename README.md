# Linux Metrics Collector
A metrics collection application for Linux machines. Created for MSCS 710 Software Project at Marist College.

## Development Environment
This section discusses the development tools and the steps required to set them up to make changes to the project.

### Linux or Virtual Machine
This project requires a device running Linux to operate. If your preferred device doesn't run on a Linux OS, you may prefer to utilize a virtual machine to run the application. Some options for running Linux on a VM include:
* [Oracle VirtualBox](https://www.virtualbox.org/)
* [VMware](https://www.vmware.com/)

### Development Tools
#### Required
* [Rust](https://www.rust-lang.org/tools/install)
* Download [IntelliJ IDEA Community Edition](https://www.jetbrains.com/idea/download/#section=windows)
* Download Rust and TOML plugins for IntelliJ IDEA
  * In IntelliJ IDEA go to File > Settings > Plugins
  * Type "Rust" in the search bar and click Install
  * When asked to install TOML, click Yes
* Clone this repository and open it in IntelliJ
  * In the toolbar, click File > Open > ~/.../Metrics-Collector

### Rust HTTP Server (From IntelliJ)
* Setup run configuration for server
  * From the project root, open the 'server/src' directories
  * Select the main.rs file in the project hierarchy
  * In the toolbar, click Run > Edit > Configurations
  * Click "Add a new configuration" > cargo
  * Name the configuration "Server"
  * Set the working directory to
    * ~/.../Metrics-Collector/server
  * Check the box that says "Run with root privileges"
* Click on the Cargo.toml file and click Attach in the top left of IntelliJ
* Hit Ok and try running the application

### RUST Command Line Server (From IntelliJ)
* Setup a run configuration for the CLI just as you did for the server, except:
  * Name it "CLI"
  * For "Command" enter `run cli`

### Run Program From Command Line
* From root directory: `cd server`
* `cargo build`
* To use command line interface version of app:
  * `sudo ./target/debug/server cli`
* To use gui version of app:
  * `sudo ./target/debug/server server`

### Interacting with the Database
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
  
### Docker Stuff
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


## Docker Cheatsheet (Will be removed later)
* Remove Container:
  * `sudo docker rm <containerName>`
* Remove Image: 
  * `sudo docker image rm <imageName>:<tag>`
* See Containers:
  * `sudo docker ps -a`
* See Images:
  * `sudo docker images`
* Start Container:
  * `sudo docker <containerName> start`
* Pause Container:
  * `sudo docker <containerName> pause`
