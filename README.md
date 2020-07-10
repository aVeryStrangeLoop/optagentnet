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

### Configuration file
`grid_size` : Size of the 2-Dimensional grids of agents, a grid of size N contains NxN agents.

`num_tasks` : Number of symbolic tasks rewarded by the environment. This number includes a null task (index 0) that does not fetch any resources for the agents.

`comp_cap` : Computational capacity of agents. This is the maximum number of tasks that the agent can perform in one network cycle. Both normal resource fetching and resource donation utilise this capacity. 

`exp_eff` : The fraction of resource donated by an agent on executing a donation task (denoted by a negative integer in its computational space/genome)

`uf_number`: The environment can reward any one out of the 6 utility functions encoded in the `src/uf.rs` file. This number denotes the function used for a particular run (you can even add your own!).

`uf_param1` : The utility function sometimes requires an extra parameter. This field determines the value of this parameter.

`gpop_size` : Size of the network population used for the genetic algorithm. A larger population optimizes in lesser number of generations but might make individual steps slower.

`gmut_prob` : Mutation probability of a task instruction for the genetic algorithm.

`gcrs_prob` : Crossover probability of two agent's genomes for the genetic algorithm.

`tti_alpha` : Task-to-resource parameter alpha.

`tti_beta` : Task-to-resource parameter beta. Given a task index T, the resource payoff for this task is given by `tti_alpha * T + tti_beta`

`max_gen` : Number of generations for which the genetic algorithm is run in order to optimize the network.

`sv_every` : The number of generations after which the state of the network is saved during optimization(in the results folder).

`threads` : Number of OS threads to use for running the program (Recommended = 8-16)

`donate_allowed` : Inter-agent donations can be turned off by setting this parameter to `false`.
