/* anevicon: An UDP-based server stress-testing tool, written in Rust.
 * Copyright (C) 2019  Temirkhan Myrzamadi <gymmasssorla@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * For more information see <https://github.com/Gymmasssorla/anevicon>.
 */

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::net::{AddrParseError, SocketAddr};
use std::num::ParseIntError;
use std::time::Duration;

use clap::ArgMatches;
use humantime::{parse_duration, DurationError};

pub const MIN_PACKET_LENGTH: usize = 1;
pub const MAX_PACKET_LENGTH: usize = 65000;

#[derive(Debug, Clone)]
pub struct ArgsConfig {
    receiver: SocketAddr,
    sender: SocketAddr,
    duration: Duration,
    length: usize,
    waiting: Duration,
    periodicity: Duration,
}

impl ArgsConfig {
    pub fn from_matches(matches: &ArgMatches) -> Result<ArgsConfig, ArgsConfigError> {
        // Check that the specified packet length is bettween [1; 65000]
        let length: usize = matches
            .value_of("length")
            .unwrap()
            .parse()
            .map_err(|error| ArgsConfigError::Length(PacketLengthError::InvalidFormat(error)))?;

        if length < MIN_PACKET_LENGTH {
            return Err(ArgsConfigError::Length(PacketLengthError::Underflow));
        } else if length > MAX_PACKET_LENGTH {
            return Err(ArgsConfigError::Length(PacketLengthError::Overflow));
        }

        // We use unwrappers because we have the defaut options specified
        Ok(ArgsConfig {
            receiver: matches
                .value_of("receiver")
                .unwrap()
                .parse()
                .map_err(|error| ArgsConfigError::Receiver(error))?,
            sender: matches
                .value_of("sender")
                .unwrap()
                .parse()
                .map_err(|error| ArgsConfigError::Sender(error))?,
            duration: parse_duration(matches.value_of("duration").unwrap())
                .map_err(|error| ArgsConfigError::Duration(error))?,
            length,
            waiting: parse_duration(matches.value_of("waiting").unwrap())
                .map_err(|error| ArgsConfigError::Waiting(error))?,
            periodicity: parse_duration(matches.value_of("periodicity").unwrap())
                .map_err(|error| ArgsConfigError::Periodicity(error))?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ArgsConfigError {
    Receiver(AddrParseError),
    Sender(AddrParseError),
    Duration(DurationError),
    Length(PacketLengthError),
    Waiting(DurationError),
    Periodicity(DurationError),
}

impl Display for ArgsConfigError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            ArgsConfigError::Receiver(error) => {
                write!(fmt, "An invalid receiver address was specified: {}!", error)
            }
            ArgsConfigError::Sender(error) => {
                write!(fmt, "An invalid sender address was specified: {}!", error)
            }
            ArgsConfigError::Duration(error) => {
                write!(fmt, "An invalid duration format was specified: {}!", error)
            }
            ArgsConfigError::Length(error) => write!(
                fmt,
                "An invalid packet length was specified: {}. \
                 A packet length must be in the range of [1; 65000]!",
                error
            ),
            ArgsConfigError::Waiting(error) => {
                write!(fmt, "An invalid waiting duration was specified: {}!", error)
            }
            ArgsConfigError::Periodicity(error) => {
                write!(fmt, "An invalid periodicity was specified: {}!", error)
            }
        }
    }
}

impl Error for ArgsConfigError {}

#[derive(Debug, Clone)]
pub enum PacketLengthError {
    InvalidFormat(ParseIntError),
    Overflow,
    Underflow,
}

impl Display for PacketLengthError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            PacketLengthError::InvalidFormat(error) => write!(fmt, "{}", error),
            PacketLengthError::Overflow => write!(fmt, "Overflow occurred"),
            PacketLengthError::Underflow => write!(fmt, "Underflow occurred"),
        }
    }
}

impl Error for PacketLengthError {}
