#![allow(non_snake_case)]

const APP_NAME: &str = env!("CARGO_PKG_NAME");


#[test]
fn shouldFail_andPrintShortHelp_whenNoInputTextIsProvided()
{
    let error =
"error: The following required arguments were not provided:
    <TEXT>

USAGE:
    struct-formatter [OPTIONS] <TEXT>

For more information try --help
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().assert().failure().stderr(error);
}

#[test]
fn shouldPass_andExpandStruct_whenNotExpandedStructIsProvided()
{
    let inputText = "{param1:=7,param2=9,param3={param4=11,param5={param6=13}}}";
    let expectedOutput =
"
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
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().arg(inputText).assert().success().stdout(expectedOutput);
}

#[test]
fn shouldPass_andExpandStructWithChosenIndent_whenNotExpandedStructAndIndentAreProvided()
{
    let inputText = "{param1:=7,param2=9,param3={param4=11,param5={param6=13}}}";
    let indent = "2";
    let expectedOutput =
"
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
";
    assert_cmd::Command::cargo_bin(APP_NAME).unwrap().args(["--indent", indent, inputText]).assert().success()
        .stdout(expectedOutput);
}
