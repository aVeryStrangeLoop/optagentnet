# OptAgentNet
(Agent Network Optimizer)

OptAgentNet is a program that simulates a decentralised network of computational agents on a 2-Dimensional grid. These agents can perform tasks in context of an environmental "utility function" to generate fitness for the entire network. A genetic algorithm optimizes this network to find the optimal state given a utility function. These agents are also capable of communicating with each other and sharing the resources they obtain by performing symbolic "tasks".

### Building OptAgentNet
Building the program requires the rust compiler and the cargo package manager for rust.

For windows systems these can be downloaded from [here](https://www.rust-lang.org/tools/install)
For linux systems these can be installed with:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

