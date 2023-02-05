# Rust-Port-Scanner

This Rust code is a simple port scanner. It takes in two optional command-line arguments: the minimum and maximum ports to scan. The program will scan all the ports from min_port to max_port and determine if they are open or closed. If a port is open, it will be displayed as "Port X is open". If a port is closed, it will be counted as a closed port. At the end of the scan, the number of open and closed ports will be displayed.


Had some help and guidance from Anthony Panarello, @atpan. Thanks Anthony!

# Steps to run the code:

1. Install the Rust programming language, if you haven't already, from the official website (https://www.rust-lang.org/tools/install).

2. Clone or download the source code for the Rust port scanner from this repository.

3. Open a terminal or command prompt and navigate to the directory where the source code is located.

4. Run the following command to build the code:
    
    $ cargo build

5. After the code has been built, run the following command to run the port scanner:

    $ cargo run [arguments]

Note the arguments are optional and consist of the minimum and maximum ports to scan. To specify the ports, use the -p or --ports option followed by the minimum port and maximum port. For example:

    $ cargo run -p 0 1000

This will scan all the ports from 0 to 1000 and display the results on the terminal or command prompt.

Note that the port scanner needs to run as an administrator or root user in order to have the permissions to scan certain ports.
