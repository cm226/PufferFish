
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "language.pest"]
pub struct PuffParser;

fn span_into_str<'a>(span: pest::Span<'a>) -> &str {
    span.as_str()
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::number))]
pub struct Number {
    #[pest_ast(outer(with(span_into_str), with(String::from)))]
    pub value: String
}
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::varname))]
pub struct Varname {
    #[pest_ast(outer(with(span_into_str), with(String::from)))]
    pub value: String,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::assignment))]
pub struct Assignment {
    pub varname: Varname,
    pub expression : Expression
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::string))]
pub struct StringVal { 
    #[pest_ast(outer(with(span_into_str), with(String::from)))]
    pub value: String
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::value))]
pub enum Value {
    Number(Number),
    Varname(Varname),
    String(StringVal)
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::operator))]
pub struct Operator {
    #[pest_ast(outer(with(span_into_str), with(String::from)))]
    pub value: String
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::complex_expression))]
pub struct ComplexExpression {
    pub value : Value,
    pub opperator : Operator, 
    pub expression : Vec<Expression>
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::expression))]
pub enum Expression {
    Value(Value),
    Complex(ComplexExpression),
    Function(Function)
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::var_declaration))]
pub struct VarDeclaration {
    pub name : Varname,
    pub value : Expression
}


#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::fn_declaration))]
pub struct FnDeclaration {
    pub name : Varname,
    pub args : Vec<Varname>,
    pub lines : Vec<Line>
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::function))]
pub struct  Function {
    pub name : Varname,
    pub args : Vec<Expression>
}

#[allow(dead_code)]
#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::line))]
pub enum Line {
    Assignment(Assignment),
    Decleration(VarDeclaration),
    FnDeclaration(FnDeclaration),
    Expression(Expression)
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::file))]
pub struct File {
    pub lines: Vec<Line>,
    _eoi: Eoi,
}

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::EOI))]
struct Eoi;