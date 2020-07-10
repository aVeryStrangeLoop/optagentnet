# OptAgentNet
(Agent Network Optimizer)

OptAgentNet simulates a decentralised network of agents on a 2-Dimensional grid. These agents can perform tasks in context of an environmental "utility function" to generate fitness for the entire network. A genetic algorithm optimizes this network to find the optimal state given a utility function. These agents are also capable of communicating with each other and sharing the resources they obtain by performing tasks.

### Building OptAgentNet
Building the program requires the rust compiler and the `cargo` package manager for rust.

For windows systems these can be downloaded from [here](https://www.rust-lang.org/tools/install)

For linux systems these can be installed with,
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Once cargo is installed on the system, either download the project as a zip archive or clone it using,
```bash
git clone https://github.com/aVeryStrangeLoop/optagentnet
```
Move to the root directory of the project and run the following command,
```bash
cargo build --release
```
The executable for optagentnet will be created in the `target/release/` subdirectory. The executable requires an `oan.cfg` file in the same directory. A default configuration file is provided in the root directory of the project.



