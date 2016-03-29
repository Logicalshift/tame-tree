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

use super::super::tree::*;

///
/// A processor is a type that converts a tree into another tree
///
/// This is the simplest type of component. It doesn't react to changes to the input tree using any
/// kind of persistent state, and it's not capable of dealing with partial changes.
///
pub trait Processor {
    ///
    /// Transforms an input tree into an output tree
    ///
    fn process(&self, input_tree: TreeRef) -> TreeRef;
}
