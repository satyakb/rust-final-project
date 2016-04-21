pub const VERSION: &'static str = "1.0";

pub const USAGE: &'static str = "
Swarm.

Usage:
  swarm unleash [-n <num>] <host>
  swarm master <cfg>
  swarm slave [-i <interface>] [-p <port>]
  swarm (-h | --help)
  swarm (-v | --version)

Options:
  -n, --num <num>       Number of requests [default: 10].
  -i, --iface <iface>   Interface for slave to listen on.
  -p, --port <port>     Port to listen on [default: 3000].
  -h, --help            Show this screen.
  -V, --version         Show version.
";
