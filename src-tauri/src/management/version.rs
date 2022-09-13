use crate::management::app::WebApp;

// web 应用管理
#[derive(Debug)]
pub struct WebAppManagement {
    pub lists: Vec<WebApp>,
}

impl WebAppManagement {
    pub fn new() -> Self {
        WebAppManagement {
            lists: vec![]
        }
    }

    pub fn load(self) -> Self {
        self
    }
}


#[test]
fn web_app_management(){
   let web_app_management = self::WebAppManagement::new();
    assert_eq!(web_app_management.lists.len(), 0);
}