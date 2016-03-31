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

use std::rc::*;

use super::super::tree::*;
use super::super::util::clonecell::*;

use super::component::*;

///
/// An OutputTreePublisher is a publisher used to collect the output from a component in the form of a tree.
/// It does not pass this output on, but makes it possible to retrieve the tree content at any time.
///
/// The `get_tree_reader()` function can be used to get a function that can read the value of the output
/// tree at any time. This function is generated because publishers are owned by the components that use them,
/// so in order to check the tree, it's necessary to use a separate object.
///
/// Example:
/// ```
/// let consumer  = get_consumer();
/// let publisher = OutputTreePublisher::new();
/// let reader    = publisher.get_tree_reader();
///
/// some_component.into_component(consumer, publisher);
/// let tree_value = reader();
/// ```
///
pub struct OutputTreePublisher {
    tree: Rc<CloneCell<TreeRef>>
}

impl Publisher for OutputTreePublisher {
    ///
    /// Publishes a change to the consumers of this component
    ///
    fn publish(&mut self, change: TreeChange) {
        self.tree.set(change.apply(&self.tree.get()));
    }
}

impl OutputTreePublisher {
    ///
    /// Creates a new OutputTreePublisher
    ///
    pub fn new() -> Box<OutputTreePublisher> {
        Box::new(OutputTreePublisher { tree: Rc::new(CloneCell::new("empty".to_tree_node())) })
    }

    ///
    /// Retrieves a function that can be used to read the published tree at any time
    ///
    pub fn get_tree_reader(&self) -> Box<Fn() -> TreeRef> {
        let tree_reference = self.tree.clone();

        Box::new(move || {
            tree_reference.get().clone()
        })
    }
}