pub(crate) type Tokens = Vec<Token>;

#[derive(Debug)]
pub(crate) enum Token
{
    CurlyBraceOpen,
    CurlyBraceClose,
    Comma,
    Text(String)
}
