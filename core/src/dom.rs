#[cfg(feature = "devtools")]
use std::sync::{Arc, Mutex};

use freya_layout::DioxusDOM;

#[cfg(feature = "devtools")]
use std::sync::MutexGuard;

pub struct DioxusSafeDOM {
    #[cfg(not(feature = "devtools"))]
    pub dom: DioxusDOM,

    #[cfg(feature = "devtools")]
    pub dom: Arc<Mutex<DioxusDOM>>,
}

#[cfg(feature = "devtools")]
impl Clone for DioxusSafeDOM {
    fn clone(&self) -> Self {
        Self {
            dom: self.dom.clone(),
        }
    }
}

impl DioxusSafeDOM {
    #[cfg(not(feature = "devtools"))]
    pub fn new(dom: DioxusDOM) -> Self {
        Self { dom }
    }

    #[cfg(feature = "devtools")]
    pub fn new(dom: DioxusDOM) -> Self {
        Self {
            dom: Arc::new(Mutex::new(dom)),
        }
    }

    #[cfg(not(feature = "devtools"))]
    pub fn dom(&self) -> &DioxusDOM {
        return &self.dom;
    }

    #[cfg(not(feature = "devtools"))]
    pub fn dom_mut(&mut self) -> &mut DioxusDOM {
        return &mut self.dom;
    }

    #[cfg(feature = "devtools")]
    pub fn dom(&self) -> MutexGuard<DioxusDOM> {
        return self.dom.lock().unwrap();
    }

    #[cfg(feature = "devtools")]
    pub fn dom_mut(&self) -> MutexGuard<DioxusDOM> {
        return self.dom.lock().unwrap();
    }
}
