use std::{collections::HashMap, sync::Mutex};

use thrift::OrderedFloat;

use crate::firefly::{FireflyServiceSyncHandler, Position};

pub struct FireflyServiceHandler {
    firefly_map: Mutex<HashMap<Position, OrderedFloat<f64>>>,
}

impl FireflyServiceHandler {
    pub fn new() -> Self {
        FireflyServiceHandler {
            firefly_map: Mutex::new(HashMap::new()),
        }
    }
}

impl FireflyServiceSyncHandler for FireflyServiceHandler {
    fn handle_get_phase_by_firefly_position(
        &self,
        position: crate::firefly::Position,
    ) -> thrift::Result<thrift::OrderedFloat<f64>> {
        let firefly_map = self.firefly_map.lock().unwrap();
        if let Some(&phase) = firefly_map.get(&position) {
            //println!("Got phase {} for position {:?}", phase, position);
            Ok(thrift::OrderedFloat(phase.into_inner()))
        } else {
            Err(thrift::Error::Application(thrift::ApplicationError {
                kind: thrift::ApplicationErrorKind::Unknown,
                message: format!("No firefly found at position {:?}", position),
            }))
        }
    }

    fn handle_send_phase_update(&self, firefly: crate::firefly::Firefly) -> thrift::Result<()> {
        let mut firefly_map = self.firefly_map.lock().unwrap();
        firefly_map.insert(firefly.position, firefly.phase);
        Ok(())
    }

    fn handle_get_fireflies(&self) -> thrift::Result<Vec<crate::firefly::Firefly>> {
        let map = self.firefly_map.lock().unwrap();
        Ok(map
            .iter()
            .map(|(position, phase)| crate::firefly::Firefly {
                position: position.clone(),
                phase: phase.clone(),
            })
            .collect())
    }
}
