// SPDX-License-Identifier: Apache-2.0

//! Module for dealing with Substrait's type system.
//!
//! See [`DataType`].

use crate::output::diagnostic;
use crate::output::extension;
use crate::string_util;
use crate::string_util::Describe;
use std::collections::HashSet;
use std::fmt::Write;
use std::sync::Arc;
use strum_macros::{Display, EnumString};

/// Typedef for type variations.
pub type Variation = Option<Arc<extension::Reference<extension::TypeVariation>>>;

/// A Substrait data type. Includes facilities for storing unresolved or
/// partially-resolved types.
#[derive(Clone, Debug, PartialEq)]
pub struct DataType {
    /// Type class (simple, compound, or user-defined).
    class: Class,

    /// Nullability.
    nullable: bool,

    /// Type variation, if any.
    variation: Variation,

    /// Type parameters for non-simple types.
    parameters: Vec<Parameter>,
}

impl Describe for DataType {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: string_util::Limit,
    ) -> std::fmt::Result {
        let mut name = String::new();
        write!(&mut name, "{}", self.class)?;
        if self.nullable {
            write!(&mut name, "?")?;
        }
        if let Some(variation) = &self.variation {
            write!(&mut name, "[{variation}]")?;
        }
        write!(f, "{}", name)?;
        let (_, limit) = limit.split(name.len());
        if !self.parameters.is_empty() {
            write!(f, "<")?;
            string_util::describe_sequence(
                f,
                &self.parameters,
                limit,
                20,
                |f, param, _, limit| param.describe(f, limit),
            )?;
            write!(f, ">")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl DataType {
    /// Creates a new type.
    pub fn new(
        class: Class,
        nullable: bool,
        variation: Variation,
        parameters: Vec<Parameter>,
    ) -> diagnostic::Result<Arc<DataType>> {
        // Check whether class and parameters work together.
        class.check_parameters(&parameters)?;

        // Check whether the specified type variation is applicable to this
        // type.
        if let Some(variation) = &variation {
            if let Some(definition) = &variation.definition {
                let base = definition.get_base_class();
                if !base.weak_equals(&class) {
                    return Err(cause!(
                        TypeMismatchedVariation,
                        "variation {} is derived from {}, not {}",
                        variation.common.name,
                        base,
                        class
                    ));
                }
            }
        }

        Ok(Arc::new(DataType {
            class,
            nullable,
            variation,
            parameters,
        }))
    }

    /// Creates a new unresolved type with the given description.
    pub fn new_unresolved() -> Arc<DataType> {
        Arc::new(DataType {
            class: Class::Unresolved,
            nullable: false,
            variation: None,
            parameters: vec![],
        })
    }

    /// Creates a new struct type.
    pub fn new_struct<T: IntoIterator<Item = Arc<DataType>>>(
        fields: T,
        nullable: bool,
    ) -> Arc<DataType> {
        Arc::new(DataType {
            class: Class::Compound(Compound::Struct),
            nullable,
            variation: None,
            parameters: fields.into_iter().map(Parameter::Type).collect(),
        })
    }

    /// Creates a new list type.
    pub fn new_list(element: Arc<DataType>, nullable: bool) -> Arc<DataType> {
        Arc::new(DataType {
            class: Class::Compound(Compound::List),
            nullable,
            variation: None,
            parameters: vec![Parameter::Type(element)],
        })
    }

    /// Creates a new map type.
    pub fn new_map(key: Arc<DataType>, value: Arc<DataType>, nullable: bool) -> Arc<DataType> {
        Arc::new(DataType {
            class: Class::Compound(Compound::List),
            nullable,
            variation: None,
            parameters: vec![Parameter::Type(key), Parameter::Type(value)],
        })
    }

    /// Creates the type of a predicate, i.e. a boolean.
    pub fn new_predicate(nullable: bool) -> Arc<DataType> {
        Arc::new(DataType {
            class: Class::Simple(Simple::Boolean),
            nullable,
            variation: None,
            parameters: vec![],
        })
    }

    /// Creates the type of a (default) integer, i.e. i32.
    pub fn new_integer(nullable: bool) -> Arc<DataType> {
        Arc::new(DataType {
            class: Class::Simple(Simple::I32),
            nullable,
            variation: None,
            parameters: vec![],
        })
    }

    /// Returns a nullable variant of this type.
    pub fn make_nullable(&self) -> Arc<DataType> {
        Arc::new(DataType {
            class: self.class.clone(),
            nullable: true,
            variation: self.variation.clone(),
            parameters: self.parameters.clone(),
        })
    }

    /// Returns the type class.
    pub fn class(&self) -> &Class {
        &self.class
    }

    /// Returns whether the type is nullable.
    pub fn nullable(&self) -> bool {
        self.nullable
    }

    /// Returns the type variation.
    pub fn variation(&self) -> &Variation {
        &self.variation
    }

    /// Returns the type parameters.
    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    /// Returns the value of the given integer parameter.
    pub fn int_parameter(&self, index: usize) -> Option<u64> {
        if let Some(Parameter::Unsigned(value)) = self.parameters.get(index) {
            Some(*value)
        } else {
            None
        }
    }

    /// Returns the value of the given type parameter.
    pub fn type_parameter(&self, index: usize) -> Option<Arc<DataType>> {
        match self.parameters.get(index) {
            Some(Parameter::Type(t)) => Some(t.clone()),
            Some(Parameter::NamedType(_, t)) => Some(t.clone()),
            _ => None,
        }
    }

    /// Returns whether this is an unresolved type.
    pub fn is_unresolved(&self) -> bool {
        matches!(self.class, Class::Unresolved)
    }

    /// Returns whether any part of this type tree is an unresolved type.
    pub fn is_unresolved_deep(&self) -> bool {
        self.is_unresolved()
            || self.parameters.iter().any(|p| match p {
                Parameter::Type(t) => t.is_unresolved_deep(),
                Parameter::NamedType(_, t) => t.is_unresolved_deep(),
                _ => false,
            })
    }

    /// Returns whether this is a STRUCT or NSTRUCT type.
    pub fn is_struct(&self) -> bool {
        matches!(
            self.class,
            Class::Compound(Compound::Struct) | Class::Compound(Compound::NamedStruct)
        )
    }

    /// Returns Some(Vec<T>)) when this is a STRUCT or NSTRUCT type, where the
    /// vector contains the field types. Returns None otherwise.
    pub fn unwrap_struct(&self) -> Option<Vec<Arc<DataType>>> {
        if self.is_struct() {
            Some(
                self.parameters
                    .iter()
                    .map(|x| x.get_type().cloned().unwrap_or_default())
                    .collect(),
            )
        } else {
            None
        }
    }

    /// Returns Some(T) when this is a STRUCT or NSTRUCT type with only a
    /// single element of type T, or None otherwise.
    pub fn unwrap_singular_struct(&self) -> Option<Arc<DataType>> {
        if self.is_struct() && self.parameters.len() == 1 {
            self.type_parameter(0)
        } else {
            None
        }
    }

    /// Returns whether this is a LIST type.
    pub fn is_list(&self) -> bool {
        matches!(self.class, Class::Compound(Compound::List))
    }

    /// Returns Some(T) when this is a LIST type with element type T, or None
    /// otherwise.
    pub fn unwrap_list(&self) -> Option<Arc<DataType>> {
        if self.is_list() {
            self.type_parameter(0)
        } else {
            None
        }
    }

    /// Returns whether this is a MAP type.
    pub fn is_map(&self) -> bool {
        matches!(self.class, Class::Compound(Compound::Map))
    }

    /// Returns Some(T) when this is a MAP type with value type T, or None
    /// otherwise.
    pub fn unwrap_map(&self) -> Option<Arc<DataType>> {
        if self.is_map() {
            self.type_parameter(1)
        } else {
            None
        }
    }

    /// Returns Some(T) when this is a MAP type with key type T, or None
    /// otherwise.
    pub fn unwrap_map_key(&self) -> Option<Arc<DataType>> {
        if self.is_map() {
            self.type_parameter(0)
        } else {
            None
        }
    }

    /// Returns whether this is the base type for this type, i.e. it does
    /// not have a variation.
    pub fn is_base_type(&self) -> bool {
        self.variation.is_none()
    }

    /// Returns the type of the nth field of this struct. Returns None if
    /// out of range or if this is known to not be a struct.
    pub fn index_struct(&self, index: usize) -> Option<Arc<DataType>> {
        if self.is_unresolved() {
            Some(DataType::new_unresolved())
        } else if self.is_struct() {
            match self.parameters.get(index) {
                Some(Parameter::Type(t)) => Some(t.clone()),
                Some(Parameter::NamedType(_, t)) => Some(t.clone()),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Internal helper for split_field_names() and strip_field_names().
    fn split_field_names_internal<F: FnMut(String)>(&self, namer: &mut F) -> Arc<DataType> {
        let is_struct = self.is_struct();
        let parameters = self
            .parameters
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, p)| {
                let p = if is_struct {
                    let (p, name) = p.split_name();
                    namer(name.unwrap_or_else(|| i.to_string()));
                    p
                } else {
                    p
                };
                p.map_type(|t| t.split_field_names_internal(namer))
            })
            .collect();
        Arc::new(DataType {
            class: self.class.clone(),
            nullable: self.nullable,
            variation: self.variation.clone(),
            parameters,
        })
    }

    /// Converts all NSTRUCT types in the tree to STRUCT, and returns the
    /// flattened list of field names encountered. The fields of STRUCT types
    /// are also returned, to ensure that the returned Vec is applicable to
    /// apply_field_names(); their names are simply their zero-based index
    /// converted to a string.
    pub fn split_field_names(&self) -> (Arc<DataType>, Vec<String>) {
        let mut names = vec![];
        let data_type = self.split_field_names_internal(&mut |s| names.push(s));
        (data_type, names)
    }

    /// Like split_field_names(), but drops the name strings.
    pub fn strip_field_names(&self) -> Arc<DataType> {
        self.split_field_names_internal(&mut |_| ())
    }

    /// Internal helper function for apply_field_names().
    fn apply_field_names_internal<F: FnMut() -> diagnostic::Result<String>>(
        &self,
        mut namer: &mut F,
    ) -> diagnostic::Result<Arc<DataType>> {
        if self.is_struct() {
            let parameters: Result<Vec<_>, _> = self
                .parameters
                .iter()
                .cloned()
                .map(|p| {
                    p.with_name(&mut namer)?
                        .map_type_result(|t| t.apply_field_names_internal(namer))
                })
                .collect();

            // The data type may be invalid after renaming, so we need to
            // call new() to perform check validity.
            DataType::new(
                Class::Compound(Compound::NamedStruct),
                self.nullable,
                self.variation.clone(),
                parameters?,
            )
        } else {
            let parameters: Result<Vec<_>, _> = self
                .parameters
                .iter()
                .cloned()
                .map(|p| p.map_type_result(|t| t.apply_field_names_internal(namer)))
                .collect();

            // Data types generated this way can never become invalid, so we
            // can construct directly.
            Ok(Arc::new(DataType {
                class: self.class.clone(),
                nullable: self.nullable,
                variation: self.variation.clone(),
                parameters: parameters?,
            }))
        }
    }

    /// Applies names to STRUCTs, or renames the names in NSTRUCTs, based on a
    /// flattened vector of names.
    pub fn apply_field_names<S: ToString>(&self, names: &[S]) -> diagnostic::Result<Arc<DataType>> {
        let mut names = names.iter();
        let mut num_too_few = 0;
        let mut namer = || {
            Ok(names.next().map(|s| s.to_string()).unwrap_or_else(|| {
                num_too_few += 1;
                format!("unnamed{num_too_few}")
            }))
        };
        let new_type = self.apply_field_names_internal(&mut namer)?;
        let remainder = names.count();
        if self.is_unresolved_deep() {
            Ok(new_type)
        } else if remainder > 0 {
            Err(cause!(
                TypeMismatchedFieldNameAssociations,
                "received {remainder} too many field name(s)"
            ))
        } else if num_too_few > 0 {
            Err(cause!(
                TypeMismatchedFieldNameAssociations,
                "received {num_too_few} too few field name(s)"
            ))
        } else {
            Ok(new_type)
        }
    }
}

impl Default for DataType {
    fn default() -> Self {
        DataType {
            class: Class::Unresolved,
            nullable: false,
            variation: None,
            parameters: vec![],
        }
    }
}

/// Trait for things that can resolve user-defined types and type variations.
pub trait TypeResolver {
    /// Resolves a user-defined type from its name.
    fn resolve_type<S: AsRef<str>>(&self, s: S) -> diagnostic::Result<Arc<extension::DataType>>;

    /// Resolves a type variation from its name and base type.
    fn resolve_type_variation<S: AsRef<str>>(
        &self,
        s: S,
        base_type: Class,
    ) -> diagnostic::Result<Arc<extension::TypeVariation>>;
}

/// Trait for checking the type parameters for a base type.
pub trait ParameterInfo {
    /// Checks whether the given parameter set is valid for this base type.
    fn check_parameters(&self, params: &[Parameter]) -> diagnostic::Result<()>;

    /// Returns the logical name of the given parameter.
    fn parameter_name(&self, index: usize) -> Option<String>;
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
    UserDefined(Arc<extension::Reference<extension::DataType>>),

    /// Unresolved type. Used for error recovery.
    Unresolved,
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Simple(simple) => write!(f, "{simple}"),
            Class::Compound(compound) => write!(f, "{compound}"),
            Class::UserDefined(user_defined) => write!(f, "{user_defined}"),
            Class::Unresolved => write!(f, "!"),
        }
    }
}

impl ParameterInfo for Class {
    fn check_parameters(&self, params: &[Parameter]) -> diagnostic::Result<()> {
        match self {
            Class::Simple(_) => {
                if params.is_empty() {
                    Ok(())
                } else {
                    Err(cause!(
                        TypeMismatchedParameters,
                        "simple types cannot be parameterized"
                    ))
                }
            }
            Class::Compound(compound) => compound.check_parameters(params),
            Class::UserDefined(_) => {
                if params.is_empty() {
                    Ok(())
                } else {
                    Err(cause!(
                        TypeMismatchedParameters,
                        "user-defined types cannot currently be parameterized"
                    ))
                }
            }
            Class::Unresolved => Ok(()),
        }
    }

    fn parameter_name(&self, index: usize) -> Option<String> {
        if let Class::Compound(compound) = self {
            compound.parameter_name(index)
        } else {
            None
        }
    }
}

impl Class {
    /// Checks whether two classes are equal, also returning true if either or
    /// both are unresolved.
    pub fn weak_equals(&self, rhs: &Class) -> bool {
        match (self, rhs) {
            (_, Class::Unresolved) | (Class::Unresolved, _) => true,
            (a, b) => a == b,
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

impl ParameterInfo for Compound {
    fn check_parameters(&self, params: &[Parameter]) -> diagnostic::Result<()> {
        match self {
            Compound::FixedChar | Compound::VarChar | Compound::FixedBinary => {
                if params.len() != 1 {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} expects a single parameter (length)"
                    ));
                }
                if let Parameter::Unsigned(length) = params[0] {
                    // Note: 2147483647 = 2^31-1 = maximum value for signed
                    // 32-bit integer. However, the significance of the number
                    // is just that the Substrait specification says this is
                    // the limit.
                    const MIN_LENGTH: u64 = 1;
                    const MAX_LENGTH: u64 = 2147483647;
                    if length < MIN_LENGTH && length > MAX_LENGTH {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} length {length} is out of range 1..{MAX_LENGTH}"
                        ));
                    }
                } else {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} length parameter must be a positive integer"
                    ));
                }
            }
            Compound::Decimal => {
                if params.len() != 2 {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} expects two parameters (precision and scale)"
                    ));
                }
                if let Parameter::Unsigned(precision) = params[0] {
                    const MAX_PRECISION: u64 = 38;
                    if precision > MAX_PRECISION {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} precision {precision} is out of range 0..{MAX_PRECISION}"
                        ));
                    }
                    if let Parameter::Unsigned(scale) = params[1] {
                        if scale > precision {
                            return Err(cause!(
                                TypeMismatchedParameters,
                                "{self} scale {scale} is out of range 0..{precision}"
                            ));
                        }
                    } else {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} scale parameter must be a positive integer"
                        ));
                    }
                } else {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} precision parameter must be a positive integer"
                    ));
                }
            }
            Compound::Struct => {
                for param in params.iter() {
                    if !matches!(param, Parameter::Type(_)) {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} parameters must be types"
                        ));
                    }
                }
            }
            Compound::NamedStruct => {
                let mut names = HashSet::with_capacity(params.len());
                for param in params.iter() {
                    if let Parameter::NamedType(name, _) = &param {
                        if !names.insert(name) {
                            return Err(cause!(
                                TypeMismatchedParameters,
                                "duplicate field name in {self}: {name}"
                            ));
                        }
                    } else {
                        return Err(cause!(
                            TypeMismatchedParameters,
                            "{self} parameters must be name-types pairs"
                        ));
                    }
                }
            }
            Compound::List => {
                if params.len() != 1 {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} expects a single parameter (element type)"
                    ));
                }
                if !matches!(params[0], Parameter::Type(_)) {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} element type parameter must be a type"
                    ));
                }
            }
            Compound::Map => {
                if params.len() != 2 {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} expects two parameters (key type and value type)"
                    ));
                }
                if !matches!(params[0], Parameter::Type(_)) {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} key type parameter must be a type"
                    ));
                }
                if !matches!(params[1], Parameter::Type(_)) {
                    return Err(cause!(
                        TypeMismatchedParameters,
                        "{self} value type parameter must be a type"
                    ));
                }
            }
        }
        Ok(())
    }

    fn parameter_name(&self, index: usize) -> Option<String> {
        match (self, index) {
            (Compound::FixedChar, 0) => Some(String::from("length")),
            (Compound::VarChar, 0) => Some(String::from("length")),
            (Compound::FixedBinary, 0) => Some(String::from("length")),
            (Compound::Decimal, 0) => Some(String::from("precision")),
            (Compound::Decimal, 1) => Some(String::from("scale")),
            (Compound::Struct, i) => Some(format!("{}", i)),
            (Compound::NamedStruct, i) => Some(format!("{}", i)),
            (Compound::List, 0) => Some(String::from("element")),
            (Compound::Map, 0) => Some(String::from("key")),
            (Compound::Map, 1) => Some(String::from("value")),
            (_, _) => None,
        }
    }
}

/// Parameter for parameterized types.
#[derive(Clone, Debug, PartialEq)]
pub enum Parameter {
    /// Type parameter (list element type, struct element types, etc).
    Type(Arc<DataType>),

    /// Named type parameter (named struct/schema pseudotype elements).
    NamedType(String, Arc<DataType>),

    /// Integral type parameter (varchar length, etc.).
    Unsigned(u64),
}

impl Describe for Parameter {
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: string_util::Limit,
    ) -> std::fmt::Result {
        match self {
            Parameter::Type(data_type) => data_type.describe(f, limit),
            Parameter::NamedType(name, data_type) => {
                let (name_limit, type_limit) = limit.split(name.len());
                string_util::describe_identifier(f, name, name_limit)?;
                write!(f, ": ")?;
                data_type.describe(f, type_limit)
            }
            Parameter::Unsigned(value) => write!(f, "{value}"),
        }
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

impl Parameter {
    /// Splits the name annotation off from a named type parameter.
    pub fn split_name(self) -> (Parameter, Option<String>) {
        match self {
            Parameter::NamedType(n, t) => (Parameter::Type(t), Some(n)),
            p => (p, None),
        }
    }

    /// Returns the name of a named type parameter.
    pub fn get_name(&self) -> Option<&str> {
        match self {
            Parameter::NamedType(n, _) => Some(n),
            _ => None,
        }
    }

    /// Returns the type of a type parameter.
    pub fn get_type(&self) -> Option<&Arc<DataType>> {
        match self {
            Parameter::Type(t) => Some(t),
            Parameter::NamedType(_, t) => Some(t),
            _ => None,
        }
    }

    /// Annotates the parameter with a name, if applicable. If the parameter
    /// was already named, the name is replaced. The function is only called
    /// for Types and NamedTypes. None is returned only if the function was
    /// called and returned None.
    pub fn with_name<E, F: FnOnce() -> Result<String, E>>(self, f: F) -> Result<Parameter, E> {
        Ok(match self {
            Parameter::Type(t) => Parameter::NamedType(f()?, t),
            Parameter::NamedType(_, t) => Parameter::NamedType(f()?, t),
            p => p,
        })
    }

    /// Modifies the contained type using the given function, if applicable. If
    /// this is not a type parameter, the function is not called.
    pub fn map_type_result<E, F: FnOnce(Arc<DataType>) -> Result<Arc<DataType>, E>>(
        self,
        f: F,
    ) -> Result<Parameter, E> {
        Ok(match self {
            Parameter::Type(t) => Parameter::Type(f(t)?),
            Parameter::NamedType(n, t) => Parameter::NamedType(n, f(t)?),
            p => p,
        })
    }

    /// Modifies the contained type using the given function, if applicable. If
    /// this is not a type parameter, the function is not called.
    pub fn map_type<F: FnOnce(Arc<DataType>) -> Arc<DataType>>(self, f: F) -> Parameter {
        match self {
            Parameter::Type(t) => Parameter::Type(f(t)),
            Parameter::NamedType(n, t) => Parameter::NamedType(n, f(t)),
            p => p,
        }
    }
}

impl From<DataType> for Parameter {
    fn from(t: DataType) -> Self {
        Parameter::Type(Arc::new(t))
    }
}

impl From<Arc<DataType>> for Parameter {
    fn from(t: Arc<DataType>) -> Self {
        Parameter::Type(t)
    }
}

impl From<u64> for Parameter {
    fn from(x: u64) -> Self {
        Parameter::Unsigned(x)
    }
}
