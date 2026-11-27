# Triad Core IPC (early protocol)

The daemon exposes a local Unix socket at `/run/triados/triad-core.sock`. Requests are one JSON object per connection; response is one JSON object followed by newline. The socket is local, not network-bound. This first iteration is read-only.

## Commands
- `{"command":"ping"}` -> liveness
- `{"command":"system_status"}` -> uptime, load, memory and host OS metadata
- `{"command":"applications"}` -> desktop application names

Requests are capped at 16 KiB and connections time out after five seconds. This protocol is experimental and not stable. The socket is currently created with mode 0660; deployment must configure an appropriate group and systemd socket/service ownership before multi-user production use. Avoid exposing the daemon socket outside the host.

## Security limitations
The daemon does not perform privileged actions. Do not add package installation, arbitrary command execution, user switching, or service mutations to this IPC surface without a reviewed authorization design. The systemd unit applies basic hardening, but this is not a complete sandbox.
