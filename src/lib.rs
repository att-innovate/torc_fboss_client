// The MIT License (MIT)
//
// Copyright (c) 2015 AT&T
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

extern crate podio;

#[macro_use]
extern crate log;

use std::{io, fmt};
use std::error::Error as StdError;

pub use protocol::Protocol;
pub use transport::Transport;

pub mod protocol;
pub mod transport;
pub mod api;


#[derive(Debug)]
pub enum Error {
	/// An error occurred when reading from/writing to the underlying transport
	TransportError(io::Error),

	/// An error occurred when encoding/decoding the data
	/// (this usually indicates a bug in the library)
	ProtocolError(protocol::Error),

	/// The server code threw a user-defined exception
	UserException,
}

impl From<protocol::Error> for Error {
	fn from(err: protocol::Error) -> Error {
			Error::ProtocolError(err)
	}
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Error {
			Error::TransportError(err)
	}
}

impl StdError for Error {
	fn description(&self) -> &str {
		"Thrift Error"
	}

	fn cause(&self) -> Option<&StdError> {
		match *self {
				Error::TransportError(ref err) => Some(err),
				Error::ProtocolError(ref err) => Some(err),
				_ => None
			}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			fmt::Debug::fmt(self, f)
	}
}

pub type Result<T> = std::result::Result<T, Error>;


