//
//   Copyright 2016 Andrew Hunter
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.
//

//! # TameTree
//! 
//! TameTree is a library for synchronising trees and reacting to change.
//!
//! ## The Why
//!
//! While 'synchronising trees' is what TameTree does, that doesn't really describe why one might choose to use
//! it. Here's why: TameTree is a method for building large programs and systems from small programs. Small 
//! programs are easier to implement and maintain than larger programs, but composing programs is difficult.
//!
//! Modern languages have a lot of composition functionality built in, so it's much more common to do the
//! composition there. The problem is that this often means components aren't particular isolated from one
//! another and it's easy to end up with a web of dependencies that can make it very hard to maintain or change
//! the individual parts.
//!
//! A TameTree component deals with data in the form of a tree. It monitors an input tree for changes, and it
//! modifies an output tree in response.
//!
//! These trees are abstract: this means it doesn't matter much to a component if its input comes from the same
//! application or if it is from another process or from a server running over the internet.
//!
//! This abstraction has one other important advantage: components can be implemented on their own terms without 
//! needing to take dependencies on other components for APIs or data type definitions. Components can easily
//! maintain a high level of isolation with respect to each other, while still being able to communicate.
//!
//! ## An observation
//!
//! Take a typically modern web app. Its gross structure is typically a database, a web server, and a front-end.
//! The server will talk to a web browser and supply the front-end as HTML, JavaScript and CSS so the browser
//! can run it. In order to interact with the user, the front-end will then talk back to the server using JSON.
//! In turn, the server will talk to the database using whatever query language it uses and format the responses
//! in JSON once again so the front-end can understand them.
//!
//! If the app is particularly modern, it will use microservices: multiple webservers that talk amongst themselves
//! and with the front-end using JSON.
//!
//! What's interesting is that all this communication is done with various forms of tree structure: JSON, HTML,
//! CSS and even the database queries and responses are all types of tree structure. However, code must be written
//! in order to decode these trees and move data around so that the various parts of the application can communicate.
//! Complicating things further is that the data being sent is often updates telling the various parts how to
//! change their state, which can make it hard to determine information about the state of the system as a whole.
//!
//! Why is this? The full reason is complicated: the technology stack has aggregated over time to solve a series
//! of small problems and has come up with different solutions to them, which have been brought together to create
//! what's recognisable as a modern web app. What's interesting is that there's a common thread: when two isolated
//! systems - such as the server and the web browser - want to communicate, they do it by determining how they want
//! the state of the other side to change, encode that change into a tree structure, further encode that structure
//! into a stream of bytes, send the bytes, decode them back into a tree and then go through and apply the changes.
//!
//! The root of all this work is a concept that is now old enough that many have forgotten it had to be invented.
//! When UNIX was being designed, it was necessary to find a way to connect two processes so they could communicate.
//! The solution to this was to use streams: the I/O ports of the computers UNIX was running on worked using 
//! streams of bytes so this would enable processes to talk to each other just like they would talk with the outside
//! world. The data would be in a textual form so that humans could read them by hooking up a teletype printer and
//! telling the processes to talk to that instead of each other, which was handy for troubleshooting.
//!
//! This decision - use streams for communication - has shaped how the internet works: the socket protocols in
//! general and HTTP in particular all present themselves as streams of bytes. However, we don't use teletypes
//! any more, and it turns out programs really want to send more structured data. The end result is that
//! developers spend a lot of time turning things from streams into trees and back again, and are impressed when
//! it turns out that JSON makes that process a lot simpler than XML.
//!
//! TameTree is a 'what if' project: what if the underlying abstraction processes used to communicate was trees of 
//! data instead of streams. It adds in a few modern concepts: most notably the idea of being 'reactive' (essentially
//! watching for changes instead of requesting them)
//!
//! The most interesting thing it allows is feedback loops: certainly in the case of our web app. If the HTML is a
//! tree and we have feedback, then updating the HTML on the server will cause it to update on the client, and events
//! relating to the HTML displayed on the client will go straight back to the server. That would seem to make quite
//! lot of that client-side javascript with all of its JSON encoding and decoding obsolete...

extern crate rustc_serialize;

#[macro_use]
pub mod tree;
pub mod component;
mod util;
