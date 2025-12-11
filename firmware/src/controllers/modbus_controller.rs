use alloc::vec::Vec;

use crate::errors::modbus_controller_err::ModbusControllerErr;

pub trait ModbusControllerTrait {
    fn read_holding(&mut self, unit: u8, addr: u16, qty: u16) -> Result<(), ModbusControllerErr>;
    fn read_discrete_input(
        &mut self,
        unit: u8,
        addr: u16,
        qty: u16,
    ) -> Result<(), ModbusControllerErr>;

    fn write_holding_single(
        &mut self,
        unit: u8,
        addr: u16,
        value: u16,
    ) -> Result<(), ModbusControllerErr>;

    fn read_input_registers(
        &mut self,
        unit: u8,
        addr: u16,
        qty: u16,
    ) -> Result<Vec<u16>, ModbusControllerErr>;

    fn read_coils(
        &mut self,
        unit: u8,
        addr: u16,
        qty: u16,
    ) -> Result<Vec<bool>, ModbusControllerErr>;

    fn read_discrete_inputs(
        &mut self,
        unit: u8,
        addr: u16,
        qty: u16,
    ) -> Result<Vec<bool>, ModbusControllerErr>;

    fn write_single_coil(
        &mut self,
        unit: u8,
        addr: u16,
        on: bool,
    ) -> Result<(), ModbusControllerErr>;

    fn write_multiple_coils(
        &mut self,
        unit: u8,
        addr: u16,
        coil_qty: u16,
    ) -> Result<(), ModbusControllerErr>;
}
