# A SURF (Standard Unique Resource Finder) parser

## Features

- Transforming a SURF into a structured representation
- Resolving the domain name of the SURF host into an ip address

## Structure

```
grid!domain.com -> {
  host: "grid.domain.com" # Notice that `www` is not the default
  path: None,
  port: 2023, # idk, we need to choose a default port
  query: None,
  fragment: None
}
```

```
grid!domain.com/path/to/something?param=sth#this-section -> {
  host: "grid.domain.com",
  path: ["path", "to", "something"],
  port: 2023, # idk, we need to choose a default port
  query: {
    param: "sth"
  },
  fragment: "the-section"
}
```

```
grid! -> {
  host: "localhost",
  path: None,
  port: 2023,
  query: None,
  fragment: None
}
```
