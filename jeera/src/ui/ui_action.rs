#[derive(Debug)]
pub enum UIAction {
    ListItemClick(usize),
    DropdownItemSelected(usize),
}
