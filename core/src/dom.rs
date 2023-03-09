use std::sync::{Arc, Mutex};

use freya_layout::DioxusDOM;

use std::sync::MutexGuard;

pub struct MaybeDOM {
    #[cfg(not(feature = "devtools"))]
    pub dom: DioxusDOM,

    #[cfg(feature = "devtools")]
    pub dom: Arc<Mutex<DioxusDOM>>,
}

#[cfg(feature = "devtools")]
impl Clone for MaybeDOM {
    fn clone(&self) -> Self {
        Self {
            dom: self.dom.clone(),
        }
    }
}

impl MaybeDOM {
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
