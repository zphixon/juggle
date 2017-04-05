
pub struct Frame {
    pub is_exec: bool,
    pub name: Option<i64>,
}

impl Frame {
    pub fn new(is_exec: bool) -> Frame {
        Frame {
            is_exec: is_exec,
            name: None,
        }
    }

    pub fn new_routine(is_exec: bool, name: i64) -> Frame {
        Frame {
            is_exec: is_exec,
            name: Some(name)
        }
    }
}

