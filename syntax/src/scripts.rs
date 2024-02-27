use super::lexicon::{Keyword, Reserved, StringConstant, Symbol};
use super::sexprs::Sexpr;
use super::sorts::Sort;
use super::terms::{SortedVar, Term};

#[derive(Clone, Debug)]
pub struct SortDec(pub Symbol, pub i64);

impl From<SortDec> for Sexpr {
    fn from(value: SortDec) -> Self {
        let SortDec(symbol, numeral) = value;
        Sexpr::Sequence(vec![symbol.into(), numeral.into()])
    }
}

#[derive(Clone, Debug)]
pub struct SelectorDec {
    pub name: Symbol,
    pub sort: Sort,
}

impl From<SelectorDec> for Sexpr {
    fn from(value: SelectorDec) -> Self {
        let SelectorDec { name, sort } = value;
        Sexpr::Sequence(vec![name.into(), sort.into()])
    }
}

#[derive(Clone, Debug)]
pub struct ConstructorDec {
    pub name: Symbol,
    pub selectors: Vec<SelectorDec>,
}

impl From<ConstructorDec> for Sexpr {
    fn from(value: ConstructorDec) -> Self {
        let ConstructorDec { name, selectors } = value;
        Sexpr::Sequence(
            vec![name.into()]
                .into_iter()
                .chain(selectors.into_iter().map(|sel_dec| sel_dec.into()))
                .collect(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct DatatypeDec {
    par: Vec<Symbol>,
    constructors: Vec<ConstructorDec>,
}

impl From<DatatypeDec> for Sexpr {
    fn from(value: DatatypeDec) -> Self {
        let DatatypeDec { par, constructors } = value;
        if par.is_empty() {
            Sexpr::Sequence(
                constructors
                    .into_iter()
                    .map(|sel_dec| sel_dec.into())
                    .collect(),
            )
        } else {
            todo!()
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDec {
    name: Symbol,
    args: Vec<SortedVar>,
    sort: Sort,
}

impl From<FunctionDec> for Sexpr {
    fn from(value: FunctionDec) -> Self {
        let FunctionDec { name, args, sort } = value;

        Sexpr::Sequence(vec![
            name.into(),
            Sexpr::Sequence(args.into_iter().map(|arg| arg.into()).collect()),
            sort.into(),
        ])
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDef {
    name: Symbol,
    args: Vec<SortedVar>,
    sort: Sort,
    body: Term,
}

impl From<FunctionDef> for Vec<Sexpr> {
    fn from(value: FunctionDef) -> Self {
        let FunctionDef {
            name,
            args,
            sort,
            body,
        } = value;

        vec![
            name.into(),
            Sexpr::Sequence(args.into_iter().map(|arg| arg.into()).collect()),
            sort.into(),
            body.into(),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct PropLiteral {
    negate: bool,
    symbol: Symbol,
}

impl From<PropLiteral> for Sexpr {
    fn from(value: PropLiteral) -> Self {
        let PropLiteral { negate, symbol } = value;
        if negate {
            Sexpr::Sequence(vec![
                Symbol::new("not".to_string()).unwrap().into(),
                symbol.into(),
            ])
        } else {
            symbol.into()
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommandType {
    Assert,
    CheckSat,
    CheckSatAssuming,
    DeclareConst,
    DeclareDatatype,
    DeclareDatatypes,
    DeclareFun,
    DeclareSort,
    DefineFun,
    DefineFunRec,
    DefineFunsRec,
    DefineSort,
    Echo,
    Exit,
    GetAssertions,
    GetAssignment,
    //GetInfo
    GetModel,
    GetOption,
    GetProof,
    GetUnsatAssumptions,
    GetUnsatCore,
    GetValue,
    Pop,
    Push,
    Reset,
    ResetAssertions,
    //SetInfo
    SetLogic,
    //SetOption /* not implemented */
}

impl From<CommandType> for Reserved {
    fn from(value: CommandType) -> Self {
        Reserved::Command(value)
    }
}

impl From<CommandType> for Sexpr {
    fn from(value: CommandType) -> Self {
        let r: Reserved = value.into();
        r.into()
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Assert(Term),
    CheckSat,
    CheckSatAssuming(Vec<PropLiteral>),
    DeclareConst(Symbol, Sort),
    DeclareDatatype(Symbol, DatatypeDec),
    DeclareDatatypes(Vec<(SortDec, DatatypeDec)>),
    DeclareFun(Symbol, Vec<Sort>, Sort),
    DeclareSort(Symbol, i64),
    DefineFun(FunctionDef),
    DefineFunRec(FunctionDef),
    DefineFunsRec(Vec<(FunctionDec, Term)>),
    DefineSort(Symbol, Vec<Symbol>, Sort),
    Echo(StringConstant),
    Exit,
    GetAssertions,
    GetAssignment,
    //GetInfo(InfoFlag /* not implementd */),
    GetModel,
    GetOption(Keyword),
    GetProof,
    GetUnsatAssumptions,
    GetUnsatCore,
    GetValue(Vec<Term>),
    Pop(i64),
    Push(i64),
    Reset,
    ResetAssertions,
    //SetInfo( /* not implemented */)
    SetLogic(Symbol),
    //SetOption(Option) /* not implemented */
}

fn to_seq<S: Into<Sexpr>>(items: Vec<S>) -> Sexpr {
    Sexpr::Sequence(items.into_iter().map(|prop| prop.into()).collect())
}

impl From<Command> for Sexpr {
    fn from(value: Command) -> Self {
        let command_type = value.command_type();
        let inner = match value {
            Command::Exit
            | Command::GetAssertions
            | Command::GetAssignment
            | Command::GetModel
            | Command::GetProof
            | Command::GetUnsatAssumptions
            | Command::GetUnsatCore
            | Command::Reset
            | Command::ResetAssertions
            | Command::CheckSat => vec![command_type.into()],
            Command::Assert(term) => vec![command_type.into(), term.into()],
            Command::CheckSatAssuming(props) => {
                let props = to_seq(props);
                vec![command_type.into(), props]
            }
            Command::DeclareConst(name, sort) => {
                vec![command_type.into(), name.into(), sort.into()]
            }
            Command::DeclareDatatype(name, datatype_dec) => {
                vec![command_type.into(), name.into(), datatype_dec.into()]
            }
            Command::DeclareDatatypes(entries) => {
                let count = entries.len();
                let (sort_decs, datatype_decs) = entries.into_iter().fold(
                    (Vec::with_capacity(count), Vec::with_capacity(count)),
                    |mut acc, (sort_dec, datatype_dec)| {
                        acc.0.push(sort_dec.into());
                        acc.1.push(datatype_dec.into());
                        acc
                    },
                );

                let sort_decs = Sexpr::Sequence(sort_decs);
                let datatype_decs = Sexpr::Sequence(datatype_decs);

                vec![command_type.into(), sort_decs, datatype_decs]
            }
            Command::DeclareFun(name, arg_sorts, ret_sort) => {
                let arg_sorts = to_seq(arg_sorts);
                vec![command_type.into(), name.into(), arg_sorts, ret_sort.into()]
            }
            Command::DeclareSort(name, num) => {
                vec![command_type.into(), name.into(), num.into()]
            }
            Command::DefineFun(fun_def) | Command::DefineFunRec(fun_def) => {
                let fun_def_sexprs: Vec<Sexpr> = fun_def.into();
                vec![command_type.into()]
                    .into_iter()
                    .chain(fun_def_sexprs.into_iter())
                    .collect()
            }
            Command::DefineFunsRec(entries) => {
                let count = entries.len();
                let (fun_decs, terms) = entries.into_iter().fold(
                    (Vec::with_capacity(count), Vec::with_capacity(count)),
                    |mut acc, (fun_dec, term)| {
                        acc.0.push(fun_dec.into());
                        acc.1.push(term.into());
                        acc
                    },
                );

                let fun_decs = Sexpr::Sequence(fun_decs);
                let terms = Sexpr::Sequence(terms);

                vec![command_type.into(), fun_decs, terms]
            }
            Command::DefineSort(name, syms, sort) => {
                vec![command_type.into(), name.into(), to_seq(syms), sort.into()]
            }
            Command::Echo(string) => vec![command_type.into(), string.into()],
            Command::GetOption(kw) => vec![command_type.into(), kw.into()],
            Command::GetValue(terms) => vec![command_type.into(), to_seq(terms)],
            Command::Pop(num) | Command::Push(num) => vec![command_type.into(), num.into()],
            Command::SetLogic(sym) => vec![command_type.into(), sym.into()],
        };

        Sexpr::Sequence(inner)
    }
}

impl Command {
    fn command_type(&self) -> CommandType {
        match self {
            Command::Assert(_) => CommandType::Assert,
            Command::CheckSat => CommandType::CheckSat,
            Command::CheckSatAssuming(_) => CommandType::CheckSatAssuming,
            Command::DeclareConst(_, _) => CommandType::DeclareConst,
            Command::DeclareDatatype(_, _) => CommandType::DeclareDatatype,
            Command::DeclareDatatypes(_) => CommandType::DeclareDatatypes,
            Command::DeclareFun(_, _, _) => CommandType::DeclareFun,
            Command::DeclareSort(_, _) => CommandType::DeclareSort,
            Command::DefineFun(_) => CommandType::DefineFun,
            Command::DefineFunRec(_) => CommandType::DefineFunRec,
            Command::DefineFunsRec(_) => CommandType::DefineFunsRec,
            Command::DefineSort(_, _, _) => CommandType::DefineSort,
            Command::Echo(_) => CommandType::Echo,
            Command::Exit => CommandType::Exit,
            Command::GetAssertions => CommandType::GetAssertions,
            Command::GetAssignment => CommandType::GetAssignment,
            Command::GetModel => CommandType::GetModel,
            Command::GetOption(_) => CommandType::GetOption,
            Command::GetProof => CommandType::GetProof,
            Command::GetUnsatAssumptions => CommandType::GetUnsatAssumptions,
            Command::GetUnsatCore => CommandType::GetUnsatCore,
            Command::GetValue(_) => CommandType::GetValue,
            Command::Pop(_) => CommandType::Pop,
            Command::Push(_) => CommandType::Push,
            Command::Reset => CommandType::Reset,
            Command::ResetAssertions => CommandType::ResetAssertions,
            Command::SetLogic(_) => CommandType::SetLogic,
        }
    }
}

pub struct Script(pub Vec<Command>);
