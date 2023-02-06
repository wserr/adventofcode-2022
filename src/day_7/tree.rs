pub enum TreeItem
{
    Folder(FolderInfo),
    File(FileInfo)
}

pub struct FolderInfo
{
    info: Info,
    content: Vec<Box<TreeItem>>
}

pub struct FileInfo
{
    info: Info,
    size: usize
}

pub struct Info
{
    name: String,
    parent: Box<TreeItem>,
}

pub enum Command<'a>
{
    MoveIn(&'a str),
    MoveOut,
    MoveHome,
    List
}

// Executes commands and returns resulting TreeItem
pub fn execute_command<'a>(current_node: Box<TreeItem>, command: &Command) -> Result<Box<TreeItem>, &'static str>
{
    match command {
        &Command::MoveIn(name) => move_in(name, current_node),
        &Command::MoveOut => Err("Not Implemented"),
        &Command::MoveHome => Err("Not Implemented"),
        &Command::List => Err("Not Implemented")
    }
}

fn move_in(name: &str, current_node: Box<TreeItem>) -> Result<Box<TreeItem>, &'static str>
{
    match get_child_folder(name, current_node)
    {
        Ok(tree_item) =>  { 
            Ok(tree_item)
        },
        Err(err) => {
            match err
            {
                GetChildFolderError::ChildFolderNotFound => create_child_folder(name, current_node),
                GetChildFolderError::ChildFoldersNotUnique => Err("Error creating child folder: Child folders not unique"),
                GetChildFolderError::Other(err) => Err(err)
            }
        }
    }
}

fn create_child_folder(name: &str, current_node: Box<TreeItem>) -> Result<TreeItem, &'static str>
{
    match current_node.get()
    {
        TreeItem::File(_) => Err("You cannot create a child folder in a file node."),
        TreeItem::Folder(info) => {
            let content : Vec<Box<TreeItem>> = Vec::new();
            let new_folder_info = FolderInfo {

                info: Info {
                    name: name.to_string(),
                    parent: current_node
                },
                content
            };
            Ok(Box::new(TreeItem::Folder(new_folder_info)))
        }
    }
}

fn get_child_folder(name: &str, current_node: &TreeItem) -> Result<Box<TreeItem>, GetChildFolderError>
{
    match current_node
    {
        TreeItem::File(_) => Err(GetChildFolderError::Other("A File cannot contain child folders")),
        TreeItem::Folder(mut folderInfo) => {
            Ok(folderInfo.content.pop().unwrap())
        }
    }
}

enum GetChildFolderError
{
    Other(&'static str),
    ChildFolderNotFound,
    ChildFoldersNotUnique
}

#[test]
fn create_directory_in_root_node() {

}

