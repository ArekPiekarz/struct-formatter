## Struct Formatter

Script for expanding one-line structs into multiple lines with proper indentation.

### Build:
`cargo build --release`  
Executable will be placed in:  
`target/release/struct-formatter`

### Example usage:  
`./struct-formatter "{param1:=7,param2=9,param3={param4=11,param5={param6=13}}}"`  
Output:
```
{
    param1:=7,
    param2=9,
    param3=
    {
        param4=11,
        param5=
        {
            param6=13
        }
    }
}
```

Change indentation with `-i` or `--indent`:  
`./struct-formatter -i 2 "{param1:=7,param2={param3=11}}"`  
Output:
```
{
  param1:=7,
  param2=
  {
    param3=11
  }
}
```

### Run tests:  
`cargo test`
