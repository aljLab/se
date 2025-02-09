*Minimal Search engines searching documents in* `./documents`

#### Basics

- `cargo run` to start a search session
- enter query to search documents for query term 
> only 1-gram-queries possible
- Docs with matches are presented and search engine is ready for new query

#### Index

- the index is built before starting the session and kept in memory
- documents in `./documents` are indexed
- the index only stores _tf(t,d)_ in its postings

##### Index Structure:    
```
{
    t1: [(1, tf(t1, d1)), (2, tf(t1, d2)), (2, tf(t1, d3)), ....],
    t2: [(2, tf(t2, d2))],
    ...
}
```

