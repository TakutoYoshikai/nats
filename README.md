# nats-io
A program to hide file into executable binary

## How to use
hide secret file
```bash
nats-io in <binary> <secret file>
```

extract secret file

```bash
nats-io out <binary that has secret data> <size of original binary> <key>
```
