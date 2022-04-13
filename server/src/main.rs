mod metrics_collector_controllers;

use metrics_collector_controllers::collector;

pub fn main() {
    collector::collect_all_metrics();
}