use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::fs::File;
use std::io;
use std::mem;
use std::path::{Path, PathBuf};

use xi_rope::Rope;
use xi_core_lib::editor::Editor;
use xi_core_lib::view::View;
use xi_core_lib::tabs::Counter;
use xi_core_lib::file::{FileManager, FileError};
use xi_core_lib::{ ViewId, BufferId };

pub struct Session {
    editors: BTreeMap<BufferId, RefCell<Editor>>,
    views: BTreeMap<ViewId, RefCell<View>>,
    id_counter: Counter,
    file_manager: FileManager,
}

impl Session {

    pub fn new() -> Self {
        Session {
            views: BTreeMap::new(),
            editors: BTreeMap::new(),
            id_counter: Counter::default(),
            file_manager: FileManager::new(),
        }
    }

    fn next_view_id(&self) -> ViewId {
        ViewId(self.id_counter.next())
    }

    fn next_buffer_id(&self) -> BufferId {
        BufferId(self.id_counter.next())
    }
}

// view-related
impl Session {
    pub fn add_new_view(&mut self, path: Option<PathBuf>) -> Result<ViewId, FileError> {
        let view_id = self.next_view_id();
        let buffer_id = self.next_buffer_id();

        let rope = match path.as_ref() {
            Some(p) => self.file_manager.open(p, buffer_id)?,
            None => Rope::from(""),
        };

        let editor = RefCell::new(Editor::with_text(rope));
        let view = RefCell::new(View::new(view_id, buffer_id));

        self.editors.insert(buffer_id, editor);
        self.views.insert(view_id, view);

        Ok(view_id)
    }
}