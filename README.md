# MultiChat

![GitHub last commit](https://img.shields.io/github/last-commit/umoho/multichat?style=for-the-badge)

* It's a simple chat application that allows you send messages to multiple users when you are in the same network, service port and room.

* You can receive messages from "broadcast" in the same room. (But you must be in the same network, service port and room.) Because it will try to decrypt the protocol data with your room id, if decrypted succeeded, it will be shown, else it will be ignored.

* As like receiving messages, you can send messages to other users in the same room. (But same network, service port and room.) Because it will send (broadcast) the encrypted protocol data to other users.

## Usage

### Linux

#### Simple usage

You can run the application with the following command:

```bash
./multichat -u <USERNAME>
```

then you will "join" the `global` room, which is default.

You **must** specify the username.

#### With room id

You can also join other rooms by typing:

```bash
./multichat -u <USERNAME> -r <ROOM>
```

#### One more thing

Use `-p` option to specify the port number, default is `23380`.

Remember set your firewall to **allow** the port.

#### Example

```bash
./multichat -u "Multi Cat" -r "some secret" -p 23381
```

And you will join the `some secret` room, with the username `Multi Cat`, and the service is on port `23381`.

Use Ctrl+C to exit.

## Build

Simply use the following command to build the application:

```bash
cargo build --release
```

Simply use the following command to run the application:

```bash
cargo run -- -u <USERNAME> -p <PORT> -r <ROOM>
```

Please check out the example for more information about the command line arguments.
