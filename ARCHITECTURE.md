# Architecture

Each game server is a docker container.
Logs are extracted from stdout/stderr, and console input is sent through docker's stdin.

## Networking

Each container will, by default, not be connected to any docker networks (except the default).
When a port is forwarded, a custom proxy server will dynamically listen to the host port and
proxy packets and requests into the container. This proxy will support TCP and UDP.

To communicate with other containers, a network will be created, and all the containers that need
to communicate (as specified by the user) will be connected dynamically to that network. Then, a
container can access another by using its hostname and the correct port, which does not need to be
proxied. An example use case would be multiple Minecraft servers and a BungeeCord/Waterfall proxy.

To expose ports, as said above, Dragon will use a custom proxy to expose ports, instead of using the
built-in Docker proxy. The reason for this is to play nice with firewalls (like UFW or FirewallD),
but also because it allows more control over potential IP-banning, requiring auth, or other small
things. There will be only one of these proxies per node, which will be run on the host machine in
order to actually listen on the host ports dynamically.

## Networking - TL;DR

Instead of what Pterodactyl does, which is connecting all containers to the same network
(`pterodactyl_nw`), Dragon will create a unique network for each container group, and it
will connect the containers inside the group to that network.

To expose ports, Dragon will use a custom proxy (only one per node) to expose ports, instead of using the built-in Docker proxy.
