# Linux-Metrics-Collector
A metrics collection application for Linux machines. Created for MSCS 710 Project at Marist College.

## Development Environment Setup
The technologies used to develop this application require development tools themselves to make changes to the project as a whole. This section discusses how to access and make changes to the various modules which make up the frontend and backend of the project.

### Rust Metrics Collector Module
* Download [Rust](https://www.rust-lang.org/tools/install)
* Download [IntelliJ IDEA Community Edition](https://www.jetbrains.com/idea/download/#section=windows)
* Download Rust and TOML plugins for IntelliJ IDEA
  * In IntelliJ IDEA go to File > Settings > Plugins
  * Type "Rust" in the search bar and click Install
  * When asked to install TOML, click Yes
* Clone this repository and open it in IntelliJ
  * In the toolbar, click File > Open > ~/.../Metrics-Collector/Client/Collector
* Setup run configurations (Must be done for client and server)
  * Enter into either the 'server' or 'client' directories
  * Select the main.rs file in the project hierarchy
  * In the toolbar, click Run > Edit > Configurations
  * Click "Add a new configuration" > cargo
  * Name the configuration "collector" or "server"
  * Set the working directory to 
    * ~/.../Metrics-Collector/client/collector (Client)
    * ~/.../Metrics-Collector/server/server (Server)
* Click on the Cargo.toml file and click Attach in the top left of IntelliJ
* Hit Ok and try running the application
