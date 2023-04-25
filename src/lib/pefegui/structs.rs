use druid::{Data, Lens};
use crate::dos_header::structs::DosHeader;

#[derive(Clone, Data, Lens)]
pub struct PefeApp {
    pub name: String,
    pub dos_head: DosHeader,

    pub current_tab: usize,
}

/*impl PefeApp {
    fn new(name: String, dos_head: DosHeader) -> Self {

        PefeApp { name, dos_head, current_tab: 0}
    }
}*/