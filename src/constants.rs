pub const SLAVE_ADDR: &'static str = "127.0.0.1:3001";

pub const VERSION: &'static str = "1.0";

pub const USAGE: &'static str = "
Swarm.

Usage:
  swarm unleash [-n <num>] <host>
  swarm master <cfg>
  swarm slave [-p <port>]
  swarm (-h | --help)
  swarm (-v | --version)

Options:
  -n, --num <num>       Number of requests [default: 10].
  -p, --port <port>     Number of requests [default: 3000].
  -h, --help            Show this screen.
  -V, --version         Show version.
";
