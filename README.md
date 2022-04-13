# Linux-Metrics-Collector
A metrics collection application for Linux machines. Created for MSCS 710 Project at Marist College.

## Development Environment Setup
The technologies used to develop this application require development tools themselves to make changes to the project as a whole. This section discusses how to access and make changes to the various modules which make up the frontend and backend of the project.

### First Steps
* Download [Rust](https://www.rust-lang.org/tools/install)
* Download [IntelliJ IDEA Community Edition](https://www.jetbrains.com/idea/download/#section=windows)
* Download Rust and TOML plugins for IntelliJ IDEA
  * In IntelliJ IDEA go to File > Settings > Plugins
  * Type "Rust" in the search bar and click Install
  * When asked to install TOML, click Yes
* Clone this repository and open it in IntelliJ
  * In the toolbar, click File > Open > ~/.../Metrics-Collector

### Rust Server
* Setup run configuration for server
  * From the project root, open the 'server/src' directories
  * Select the main.rs file in the project hierarchy
  * In the toolbar, click Run > Edit > Configurations
  * Click "Add a new configuration" > cargo
  * Name the configuration "server"
  * Set the working directory to
    * ~/.../Metrics-Collector/server
  * Check the box that says "Run with root privileges"
* Click on the Cargo.toml file and click Attach in the top left of IntelliJ
* Hit Ok and try running the application

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
