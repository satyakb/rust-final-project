pub const SLAVE_ADDR: &'static str = "127.0.0.1:3000";

pub const VERSION: &'static str = "1.0";

pub const USAGE: &'static str = "
Swarm.

Usage:
  swarm (unleash | master) [--num=<n>] HOST
  swarm slave
  swarm (-h | --help)
  swarm (-v | --version)

Options:
  --num=<n>      Number of requests [default: 10].
  -h, --help     Show this screen.
  -V, --version  Show version.
";