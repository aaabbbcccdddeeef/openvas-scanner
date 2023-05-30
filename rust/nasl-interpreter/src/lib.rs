// SPDX-FileCopyrightText: 2023 Greenbone AG
//
// SPDX-License-Identifier: GPL-2.0-or-later

#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
mod built_in_functions;

mod error;

mod assign;
mod call;
mod declare;
mod helper;
mod include;
mod interpreter;
mod loop_extension;
mod operator;

pub use error::FunctionError;
pub use error::InterpretError;
pub use error::InterpretErrorKind;
pub use nasl_builtin_utils::context::{Context, ContextType, DefaultContext, Register};
pub use nasl_builtin_utils::error::FunctionErrorKind;

pub use interpreter::Interpreter;
pub use nasl_builtin_utils::sessions::Sessions;
pub use nasl_syntax::logger::{DefaultLogger, Mode, NaslLogger};
pub use nasl_syntax::NaslValue;
pub use nasl_syntax::{FSPluginLoader, Loader, LoadError, AsBufReader, NoOpLoader, load_non_utf8_path};

// Is a type definition for built-in functions
pub(crate) type NaslFunction<'a, K> =
    fn(&Register, &Context<K>) -> Result<NaslValue, FunctionErrorKind>;

pub(crate) fn lookup<K>(function_name: &str) -> Option<NaslFunction<K>>
where
    K: AsRef<str>,
{
    built_in_functions::lookup(function_name)
}
