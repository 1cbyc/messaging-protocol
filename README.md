# my encrypted messaging protocol

to run this, just run 

```rust
     cargo run --bin client sugar
```

or 

```rust
     cargo run --bin client isaac
````

You should see an interactive prompt like:

```text
   sugar > 
```

```text   
isaac > 
```

then, in each client, copy the x25519 key from the other as a contact (the excahnge x25519 keys):
```text:
     add isaac <isaac's_x25519_key>
     add sugar <sugar's_x25519_key>
```

then, send a message:
- in sugar client:

```text
     send isaac Hello Isaac!
```

- in isaac client:

```text
     receive
```