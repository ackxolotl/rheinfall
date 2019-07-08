# rheinfall

rheinfall is a packet forwarder / generator application for [Redox](https://www.redox-os.org/).
It bypasses most of Redox's network stack by writing directly to the `network:` scheme, i.e. the network driver currently in charge.
rheinfall's goal is to give a rough performance assessment for Redox's drivers, especially [its new 10 Gbit/s ixgbe driver](https://github.com/ackxolotl/ixgbed).

## Build instructions

To run rheinfall on Redox, create the directory `cookbook/recipes/rheinfall`.
Add a file `recipe.sh` to this directory with the following content:

```
GIT=https://github.com/ackxolotl/rheinfall.git
```

Open `filesystem.toml` and add a new entry for rheinfall under `userutils = {}`:

```
rheinfall = {}
```

Run one of Redox's build commands, e.g. `make qemu`.

## Usage

To mirror all received packets back on the link, run

```
sudo rheinfall
```

To generate 1,500 Byte packets, run

```
sudo rheinfall --generate
```

The packet size can be modified with the `--size` flag:

```
sudo rheinfall --generate --size 60
```

rheinfall currently supports a size of 60 or 1,500 Byte packets.

