# Metrics-Collector

# How to set up Development Environment

First, download rust using this link: https://www.rust-lang.org/tools/install

Once that is finished, go into IntelliJ and download the Rust and TOML plugins.

You will have to set up run configurations. To do this, click on the main.rs file. Next click on 'Run -> Edit 
Configurations' in the top menu bar. From there click "Add a new configuration," and then cargo. 

Give the configuration a name, such as "collector" or "server." You will then need to set the working directory. For the
collector for example, I set it to "~/.../Metrics-Collector/client/collector." Hit Ok and try running the app.