**task list:**  
- [x] PEST parser
- [ ] add evaluate to evaluate the parsed expression using AST
- [ ] add tests

**build:**    
```cargo build```

**run:**  
```cargo run```

Example output

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target\debug\pest-hello-world.exe`
Pair {
    rule: program,
    span: Span {
        str: "1 + 2 * 3",
        start: 0,
        end: 9,
    },
    inner: [
        Pair {
            rule: expression,
            span: Span {
                str: "1 + 2 * 3",
                start: 0,
                end: 9,
            },
            inner: [
                ...
            ],
        },
    ],
}
```
