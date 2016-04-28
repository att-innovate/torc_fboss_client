# torc_fboss_client
# 
# Underlying thrift layer is a simplified version of https://github.com/terminalcloud/thrift/tree/master/lib/rs
#

Simple FBOSS client written in Rust.

This library includes sample code in the example folder.
To run follow steps below and replace `127.0.0.1:5909` with the connection arguments for your FBOSS agent

Build library:

	cargo build

Build and run example list_port_stats:
	
	cargo build --example list_port_stats
	cargo run --example list_port_stats 127.0.0.1:5909

Build and run example list_routes:
	
	cargo build --example list_routes
	cargo run --example list_routes 127.0.0.1:5909

Build and run example modify_routes. Adjust IP addresses of the routes according your FBOSS config:
	
	cargo build --example modify_routes
	cargo run --example modify_routes 127.0.0.1:5909
