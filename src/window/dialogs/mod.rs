mod pick_proj;

pub enum DialogPopups {
    PICKPROJ(pick_proj::PickProjDialog),
}

pub fn mk_dialogs() -> Vec<DialogPopups> {
    vec![DialogPopups::PICKPROJ(pick_proj::PickProjDialog)]
}
