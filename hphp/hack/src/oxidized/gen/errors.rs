// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.
//
// @generated SignedSource<<b9b27a298f8d8a9cd714fafc5c3ba054>>
//
// To regenerate this file, run:
//   hphp/hack/src/oxidized/regen.sh

use ocamlrep_derive::OcamlRep;
use serde::Deserialize;
use serde::Serialize;

use crate::error_codes;
use crate::phase_map;
use crate::pos;
use crate::relative_path;

pub use error_codes::Naming;
pub use error_codes::NastCheck;
pub use error_codes::Parsing;
pub use error_codes::Typing;

pub type ErrorCode = isize;

/// We use `Pos.t message` on the server and convert to `Pos.absolute message`
/// before sending it to the client
pub type Message<A> = (A, String);

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    OcamlRep,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum Phase {
    Init,
    Parsing,
    Naming,
    Decl,
    Typing,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, OcamlRep, PartialEq, Serialize)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, OcamlRep, PartialEq, Serialize)]
pub enum Format {
    Context,
    Raw,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, OcamlRep, PartialEq, Serialize)]
pub enum NameContext {
    FunctionNamespace,
    ConstantNamespace,
    /// Classes, interfaces, traits, records and type aliases.
    TypeNamespace,
    TraitContext,
    ClassContext,
    RecordContext,
}

/// Results of single file analysis.
pub type FileT<A> = phase_map::PhaseMap<Vec<A>>;

/// Results of multi-file analysis.
pub type FilesT<A> = relative_path::map::Map<FileT<A>>;

#[derive(Clone, Debug, Deserialize, OcamlRep, Serialize)]
pub struct Error_<A>(pub ErrorCode, pub Vec<Message<A>>);

pub type Error = Error_<pos::Pos>;

#[derive(Clone, Debug, Deserialize, OcamlRep, Serialize)]
pub struct AppliedFixme(pub pos::Pos, pub isize);

#[derive(Clone, Debug, Deserialize, OcamlRep, Serialize)]
pub struct Errors(pub FilesT<Error>, pub FilesT<AppliedFixme>);
