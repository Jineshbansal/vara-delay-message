#![no_std]
#![allow(static_mut_refs)]

use sails_rs::{
    prelude::*,
};
use gstd::{msg,exec,ReservationId};
use gstd::ReservationIdExt;

static mut STORAGE: Option<Storage> = None;

pub struct Storage {
    count: u32,
}

impl Storage {
    pub fn get_mut() -> &'static mut u32 {
        unsafe { &mut STORAGE.as_mut().unwrap().count }
    }
}

struct DelayTestService(());

impl DelayTestService {
    pub fn init() -> Self {
        unsafe { STORAGE = Some(Storage { count: 20 }) }
        Self(())
    }

    pub fn get_mut(&mut self) -> &'static mut Storage {
        unsafe { STORAGE.as_mut().expect("Not yet initilised") }
    }

    pub fn get(&self) -> &'static Storage {
        unsafe { STORAGE.as_ref().expect("Not yet initialised") }
    }
}

#[sails_rs::service]
impl DelayTestService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn send(&mut self, count: u32) -> bool {
        let payload = [
            "DelayTest".encode(),
            "Increment".to_string().encode(),
            (count).encode(),
        ]
        .concat();
        let reservation = ReservationId::reserve(500_000_000_000, 100).expect("Unable to reserve");

        msg::send_from_reservation(reservation, exec::program_id(), payload, 0).expect("Unable to send");
        return false;

    }

    pub fn increment(&mut self, count: u32) -> bool {
        self.get_mut().count += count;
        true
    }

    pub fn get_counter(&self) -> u32 {
        self.get().count.clone()
    }
}

pub struct VaraDelayCounterProgram(());

#[sails_rs::program]
impl VaraDelayCounterProgram {
    // Program's constructor
    pub fn new() -> Self {
        DelayTestService::init();
        Self(())
    }

    // Exposed service
    pub fn delay_test(&self) -> DelayTestService {
        DelayTestService::new()
    }
}