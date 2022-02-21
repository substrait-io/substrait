use crate::diagnostic;
use crate::diagnostic::Cause::MismatchedTypeParameters;
use crate::extension;
use std::collections::HashSet;
use std::rc::Rc;
use strum_macros::{Display, EnumString};

/// A Substrait data type.
#[derive(Clone, Debug, PartialEq)]
pub struct DataType {
    /// Type class (simple, compound, or user-defined).
    pub class: Class,

    /// Nullability.
    pub nullable: bool,

    /// Type variation, if any.
    pub variation: Option<Rc<extension::Reference<extension::TypeVariation>>>,

    /// Type parameters for non-simple types.
    pub parameters: Vec<Parameter>,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.class)?;
        if self.nullable {
            write!(f, "?")?;
        }
        if let Some(variation) = &self.variation {
            write!(f, "[{}]", variation)?;
        }
        if self.parameters.is_empty() {
            write!(f, "<")?;
            let mut first = true;
            for parameter in self.parameters.iter() {
                if first {
                    first = false;
                } else {
                    write!(f, ", ")?;
                }
                write!(f, "{}", parameter)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

/// Trait for things that can resolve user-defined types and type variations.
pub trait TypeResolver {
    /// Resolves a user-defined type from its name.
    fn resolve_type<S: AsRef<str>>(&self, s: S) -> diagnostic::Result<Rc<extension::DataType>>;

    /// Resolves a type variation from its name and base type.
    fn resolve_type_variation<S: AsRef<str>>(
        &self,
        s: S,
        base_type: Class,
    ) -> diagnostic::Result<Rc<extension::TypeVariation>>;
}

/// Trait for checking the type parameters for a base type.
trait ParameterChecker {
    /// Checks whether the given parameter set is valid for this base type.
    fn check_parameters(&self, params: &[Parameter]) -> diagnostic::Result<()>;
}

impl DataType {
    /// Parse a string as a type.
    pub fn parse<S: AsRef<str>, R: TypeResolver>(
        _s: S,
        _type_resolver: R,
    ) -> diagnostic::Result<DataType> {
        todo!(
            "use nom or some other parser to implement this;
            also run ParameterChecker. then make round-trip tests.
            this should probably also not return Result; the
            prototype should allow returning some best-effort/error
            type for recovery in addition to any number of diagnostics"
        )
    }
}

/// Type class.
#[derive(Clone, Debug, PartialEq)]
pub enum Class {
    /// Well-known simple type.
    Simple(Simple),

    /// Well-known compound type.
    Compound(Compound),

    /// User-defined type.
    UserDefined(Rc<extension::Reference<extension::DataType>>),

    /// Unresolved type. Used for error recovery.
    Unresolved(String),
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Simple(simple) => write!(f, "{}", simple),
            Class::Compound(compound) => write!(f, "{}", compound),
            Class::UserDefined(user_defined) => write!(f, "{}", user_defined),
            Class::Unresolved(name) => write!(f, "{}!", name),
        }
    }
}

impl ParameterChecker for Class {
    fn check_parameters(&self, params: &[Parameter]) -> diagnostic::Result<()> {
        match self {
            Class::Simple(_) => {
                if params.is_empty() {
                    Ok(())
                } else {
                    Err(MismatchedTypeParameters(
                        "simple types cannot be parameterized".to_string(),
                    ))
                }
            }
            Class::Compound(compound) => compound.check_parameters(params),
            Class::UserDefined(_) => {
                if params.is_empty() {
                    Ok(())
                } else {
                    Err(MismatchedTypeParameters(
                        "user-defined types cannot currently be parameterized".to_string(),
                    ))
                }
            }
            Class::Unresolved(_) => Ok(()),
        }
    }
}

/// Enumeration of simple types defined by Substrait.
#[derive(Clone, Debug, PartialEq, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "snake_case")]
pub enum Simple {
    Boolean,
    I8,
    I16,
    I32,
    I64,
    Fp32,
    Fp64,
    String,
    Binary,
    Timestamp,
    TimestampTz,
    Date,
    Time,
    IntervalYear,
    IntervalDay,
    Uuid,
}

/// Enumeration of compound types defined by Substrait.
#[derive(Clone, Debug, PartialEq, Display, EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "UPPERCASE")]
pub enum Compound {
    FixedChar,
    VarChar,
    FixedBinary,
    Decimal,
    Struct,
    #[strum(serialize = "NSTRUCT")]
    NamedStruct,
    List,
    Map,
}

impl ParameterChecker for Compound {
    fn check_parameters(&self, params: &[Parameter]) -> diagnostic::Result<()> {
        match self {
            Compound::FixedChar | Compound::VarChar | Compound::FixedBinary => {
                if params.len() != 1 {
                    return Err(MismatchedTypeParameters(format!(
                        "{} expects a single parameter (length)",
                        self
                    )));
                }
                if let Parameter::Unsigned(length) = params[0] {
                    if length < 1 && length > 2147483647 {
                        return Err(MismatchedTypeParameters(format!(
                            "{} length {} is out of range 1..2147483647",
                            self, length
                        )));
                    }
                } else {
                    return Err(MismatchedTypeParameters(format!(
                        "{} length parameter must be a positive integer",
                        self
                    )));
                }
            }
            Compound::Decimal => {
                if params.len() != 2 {
                    return Err(MismatchedTypeParameters(format!(
                        "{} expects two parameters (precision and scale)",
                        self
                    )));
                }
                if let Parameter::Unsigned(precision) = params[0] {
                    if precision > 38 {
                        return Err(MismatchedTypeParameters(format!(
                            "{} precision {} is out of range 0..38",
                            self, precision
                        )));
                    }
                    if let Parameter::Unsigned(scale) = params[1] {
                        if scale > precision {
                            return Err(MismatchedTypeParameters(format!(
                                "{} scale {} is out of range 0..{}",
                                self, scale, precision
                            )));
                        }
                    } else {
                        return Err(MismatchedTypeParameters(format!(
                            "{} scale parameter must be a positive integer",
                            self
                        )));
                    }
                } else {
                    return Err(MismatchedTypeParameters(format!(
                        "{} precision parameter must be a positive integer",
                        self
                    )));
                }
            }
            Compound::Struct => {
                for param in params.iter() {
                    if !matches!(param, Parameter::Type(_)) {
                        return Err(MismatchedTypeParameters(format!(
                            "{} parameters must be types",
                            self
                        )));
                    }
                }
            }
            Compound::NamedStruct => {
                let mut names = HashSet::with_capacity(params.len());
                for param in params.iter() {
                    if let Parameter::NamedType(name, _) = &param {
                        if !names.insert(name) {
                            return Err(MismatchedTypeParameters(format!(
                                "duplicate field name in {}: {}",
                                self, name
                            )));
                        }
                    } else {
                        return Err(MismatchedTypeParameters(format!(
                            "{} parameters must be name-types pairs",
                            self
                        )));
                    }
                }
            }
            Compound::List => {
                if params.len() != 1 {
                    return Err(MismatchedTypeParameters(format!(
                        "{} expects a single parameter (element type)",
                        self
                    )));
                }
                if !matches!(params[0], Parameter::Type(_)) {
                    return Err(MismatchedTypeParameters(format!(
                        "{} element type parameter must be a type",
                        self
                    )));
                }
            }
            Compound::Map => {
                if params.len() != 2 {
                    return Err(MismatchedTypeParameters(format!(
                        "{} expects two parameters (key type and value type)",
                        self
                    )));
                }
                if !matches!(params[0], Parameter::Type(_)) {
                    return Err(MismatchedTypeParameters(format!(
                        "{} key type parameter must be a type",
                        self
                    )));
                }
                if !matches!(params[1], Parameter::Type(_)) {
                    return Err(MismatchedTypeParameters(format!(
                        "{} value type parameter must be a type",
                        self
                    )));
                }
            }
        }
        Ok(())
    }
}

/// Parameter for parameterized types.
#[derive(Clone, Debug, PartialEq)]
pub enum Parameter {
    /// Type parameter (list element type, struct element types, etc).
    Type(DataType),

    /// Named type parameter (named struct/schema pseudotype elements).
    NamedType(String, DataType),

    /// Integral type parameter (varchar length, etc.).
    Unsigned(u64),
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        static IDENTIFIER_RE: once_cell::sync::Lazy<regex::Regex> =
            once_cell::sync::Lazy::new(|| regex::Regex::new("[a-zA-Z_][a-zA-Z0-9_]*").unwrap());

        match self {
            Parameter::Type(data_type) => write!(f, "{}", data_type),
            Parameter::NamedType(name, data_type) => {
                if IDENTIFIER_RE.is_match(name) {
                    write!(f, "{}: {}", name, data_type)
                } else {
                    write!(f, "\"")?;
                    for c in name.chars() {
                        match c {
                            '\\' => write!(f, "\\\\")?,
                            '"' => write!(f, "\"")?,
                            x => write!(f, "{}", x)?,
                        }
                    }
                    write!(f, "\": {}", data_type)
                }
            }
            Parameter::Unsigned(value) => write!(f, "{}", value),
        }
    }
}
