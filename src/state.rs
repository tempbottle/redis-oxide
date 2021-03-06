// use rand::Rng;
use crate::data_structures::receipt_map::{KeyTypes, Receipt};
use crate::types::{Index, ReturnValue, State, StateRef, StateStore};
use std::fmt;

const DEFAULT_DB: Index = 0;

impl fmt::Display for ReturnValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReturnValue::Ok => write!(f, "OK"),
            ReturnValue::StringRes(s) => write!(f, "{:?}", s),
            ReturnValue::IntRes(i) => write!(f, "{:?}", i),
            ReturnValue::MultiStringRes(ss) => write!(f, "{:?}", ss),
            ReturnValue::Nil => write!(f, "(nil)"),
            ReturnValue::Error(e) => write!(f, "ERR {:?}", e),
            ReturnValue::Array(a) => write!(f, "{:?}", a),
        }
    }
}

impl State {
    pub fn get_receipt(&self) -> Receipt {
        let mut rm = self.reciept_map.lock();
        rm.get_receipt()
    }

    pub fn receipt_timed_out(&self, receipt: Receipt) -> bool {
        let rm = self.reciept_map.lock();
        rm.receipt_timed_out(receipt)
    }

    pub fn wake_list(&self, list_key: &[u8]) {
        let mut rm = self.reciept_map.lock();
        rm.wake_with_key(KeyTypes::list(list_key));
    }
}

impl StateStore {
    pub fn get_or_create(&self, index: Index) -> StateRef {
        let mut guard = self.states.lock();
        guard.entry(index).or_default().clone()
    }

    pub fn get_default(&self) -> StateRef {
        self.get_or_create(DEFAULT_DB)
    }
}
