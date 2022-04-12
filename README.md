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

### Rust Metrics Collector Module
* Setup run configuration for collector
  * From the project root, open the 'client/collector/src' directories
  * Select the main.rs file
  * In the toolbar, click Run > Edit > Configurations
  * Click "Add a new configuration" > cargo
  * Name the configuration "collector"
  * Set the working directory to 
    * ~/.../Metrics-Collector/client/collector
* Click on the Cargo.toml file and click Attach in the top left of IntelliJ
* Hit Ok and try running the application

### Rust Server Module
* Setup run configuration for server
  * From the project root, open the 'server/src' directories
  * Select the main.rs file in the project hierarchy
  * In the toolbar, click Run > Edit > Configurations
  * Click "Add a new configuration" > cargo
  * Name the configuration "server"
  * Set the working directory to
    * ~/.../Metrics-Collector/server
* Click on the Cargo.toml file and click Attach in the top left of IntelliJ
* Hit Ok and try running the application

### Database Module
* Create Database in SQLite
  * From the project root, cd into the 'database' directory
  * Create the database by running and start SQLite with the command: "sqlite3 mmc.db"
  * To create tables (or reset the tables), run the command: ".read create_tables.sql"
  * To insert sample data into the tables, run the command: ".read sample_data.sql"
  * Test that everything works with "SELECT * FROM <TABLE_NAME>"