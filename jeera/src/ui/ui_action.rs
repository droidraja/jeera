#[derive(Debug, PartialEq)]
pub enum UIAction {
    ListItemClick(usize),
    DropdownItemSelected(usize),
    CloseCreateIssue,
}
