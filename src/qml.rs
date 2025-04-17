use crate::WindowManager;
use crate::WindowPosition;
use qmetaobject::*;

#[derive(QObject, Default)]
pub struct WindowManagerQObject {
    base: qt_base_class!(trait QObject),
    window_manager: std::cell::RefCell<WindowManager>,
    
    position_window: qt_method!(fn(&mut self, window_id: String, position: i32)),
    calculate_geometry: qt_method!(fn(&self, position: i32, screen_width: i32, screen_height: i32) 
                                  -> QVariantList),
}

impl WindowManagerQObject {
    pub fn new() -> Self {
        let mut obj = Self::default();
        obj.window_manager = std::cell::RefCell::new(WindowManager::new());
        obj
    }
    
    fn position_window(&mut self, window_id: String, position: i32) {
        let position = match position {
            0 => WindowPosition::Left,
            1 => WindowPosition::Right,
            _ => WindowPosition::Center,
        };
        self.window_manager.borrow_mut().position_window(&window_id, position);
    }
    
    fn calculate_geometry(&self, position: i32, screen_width: i32, screen_height: i32) -> QVariantList {
        let position = match position {
            0 => WindowPosition::Left,
            1 => WindowPosition::Right,
            _ => WindowPosition::Center,
        };
        
        let (x, y, width, height) = self.window_manager
            .borrow()
            .calculate_geometry(position, screen_width, screen_height);
        
        QVariantList::from_iter(vec![
            QVariant::from(x),
            QVariant::from(y),
            QVariant::from(width),
            QVariant::from(height),
        ])
    }
}