//! Hardware module for A-lang
//! Provides simulated GPIO, I2C, SPI, and UART interfaces for IoT development

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// GPIO pin modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PinMode {
    Input,
    Output,
    InputPullUp,
    InputPullDown,
}

impl PinMode {
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "input" | "in" => Ok(PinMode::Input),
            "output" | "out" => Ok(PinMode::Output),
            "input_pullup" | "pullup" => Ok(PinMode::InputPullUp),
            "input_pulldown" | "pulldown" => Ok(PinMode::InputPullDown),
            _ => Err(format!("Unknown pin mode: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PinMode::Input => "input",
            PinMode::Output => "output",
            PinMode::InputPullUp => "input_pullup",
            PinMode::InputPullDown => "input_pulldown",
        }
    }
}

/// GPIO pin state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PinState {
    Low,
    High,
}

impl PinState {
    pub fn as_bool(&self) -> bool {
        matches!(self, PinState::High)
    }

    pub fn from_bool(value: bool) -> Self {
        if value {
            PinState::High
        } else {
            PinState::Low
        }
    }

    pub fn as_int(&self) -> i64 {
        match self {
            PinState::Low => 0,
            PinState::High => 1,
        }
    }
}

/// GPIO pin configuration
#[derive(Debug, Clone)]
pub struct GpioPin {
    pub number: u32,
    pub mode: PinMode,
    pub state: PinState,
    pub pwm_duty: Option<f32>, // 0.0 to 1.0 for PWM
}

impl GpioPin {
    pub fn new(number: u32) -> Self {
        Self {
            number,
            mode: PinMode::Input,
            state: PinState::Low,
            pwm_duty: None,
        }
    }

    pub fn set_mode(&mut self, mode: PinMode) {
        self.mode = mode;
    }

    pub fn write(&mut self, state: PinState) -> Result<(), String> {
        if self.mode != PinMode::Output {
            return Err(format!(
                "Pin {} is in {} mode, cannot write",
                self.number,
                self.mode.as_str()
            ));
        }
        self.state = state;
        Ok(())
    }

    pub fn read(&self) -> PinState {
        match self.mode {
            PinMode::InputPullUp => PinState::High,
            PinMode::InputPullDown => PinState::Low,
            _ => self.state,
        }
    }

    pub fn toggle(&mut self) -> Result<(), String> {
        if self.mode != PinMode::Output {
            return Err(format!(
                "Pin {} is in {} mode, cannot toggle",
                self.number,
                self.mode.as_str()
            ));
        }
        self.state = match self.state {
            PinState::Low => PinState::High,
            PinState::High => PinState::Low,
        };
        Ok(())
    }

    pub fn set_pwm(&mut self, duty: f32) -> Result<(), String> {
        if self.mode != PinMode::Output {
            return Err("Pin must be in output mode for PWM".to_string());
        }
        if !(0.0..=1.0).contains(&duty) {
            return Err("PWM duty cycle must be between 0.0 and 1.0".to_string());
        }
        self.pwm_duty = Some(duty);
        Ok(())
    }

    pub fn clear_pwm(&mut self) {
        self.pwm_duty = None;
    }
}

/// GPIO controller
#[derive(Debug, Clone)]
pub struct GpioController {
    pins: HashMap<u32, GpioPin>,
}

impl GpioController {
    pub fn new() -> Self {
        Self {
            pins: HashMap::new(),
        }
    }

    pub fn init_pin(&mut self, pin_number: u32, mode: PinMode) -> Result<(), String> {
        let mut pin = GpioPin::new(pin_number);
        pin.set_mode(mode);
        self.pins.insert(pin_number, pin);
        Ok(())
    }

    pub fn set_mode(&mut self, pin_number: u32, mode: PinMode) -> Result<(), String> {
        let pin = self
            .pins
            .get_mut(&pin_number)
            .ok_or_else(|| format!("Pin {} not initialized", pin_number))?;
        pin.set_mode(mode);
        Ok(())
    }

    pub fn write(&mut self, pin_number: u32, state: PinState) -> Result<(), String> {
        let pin = self
            .pins
            .get_mut(&pin_number)
            .ok_or_else(|| format!("Pin {} not initialized", pin_number))?;
        pin.write(state)
    }

    pub fn read(&self, pin_number: u32) -> Result<PinState, String> {
        let pin = self
            .pins
            .get(&pin_number)
            .ok_or_else(|| format!("Pin {} not initialized", pin_number))?;
        Ok(pin.read())
    }

    pub fn toggle(&mut self, pin_number: u32) -> Result<(), String> {
        let pin = self
            .pins
            .get_mut(&pin_number)
            .ok_or_else(|| format!("Pin {} not initialized", pin_number))?;
        pin.toggle()
    }

    pub fn set_pwm(&mut self, pin_number: u32, duty: f32) -> Result<(), String> {
        let pin = self
            .pins
            .get_mut(&pin_number)
            .ok_or_else(|| format!("Pin {} not initialized", pin_number))?;
        pin.set_pwm(duty)
    }

    pub fn get_pin_info(&self, pin_number: u32) -> Result<String, String> {
        let pin = self
            .pins
            .get(&pin_number)
            .ok_or_else(|| format!("Pin {} not initialized", pin_number))?;
        Ok(format!(
            "Pin {}: mode={}, state={:?}, pwm={:?}",
            pin.number,
            pin.mode.as_str(),
            pin.state,
            pin.pwm_duty
        ))
    }

    pub fn list_pins(&self) -> Vec<u32> {
        let mut pins: Vec<u32> = self.pins.keys().copied().collect();
        pins.sort();
        pins
    }
}

impl Default for GpioController {
    fn default() -> Self {
        Self::new()
    }
}

/// I2C device address
pub type I2cAddress = u8;

/// I2C register
pub type I2cRegister = u8;

/// Simulated I2C device
#[derive(Debug, Clone)]
pub struct I2cDevice {
    pub address: I2cAddress,
    pub registers: HashMap<I2cRegister, u8>,
}

impl I2cDevice {
    pub fn new(address: I2cAddress) -> Self {
        Self {
            address,
            registers: HashMap::new(),
        }
    }

    pub fn write_register(&mut self, register: I2cRegister, value: u8) {
        self.registers.insert(register, value);
    }

    pub fn read_register(&self, register: I2cRegister) -> Option<u8> {
        self.registers.get(&register).copied()
    }

    pub fn write_registers(&mut self, start_register: I2cRegister, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.registers
                .insert(start_register.wrapping_add(i as u8), byte);
        }
    }

    pub fn read_registers(&self, start_register: I2cRegister, count: usize) -> Vec<u8> {
        (0..count)
            .map(|i| {
                self.registers
                    .get(&start_register.wrapping_add(i as u8))
                    .copied()
                    .unwrap_or(0)
            })
            .collect()
    }
}

/// I2C bus controller
#[derive(Debug, Clone)]
pub struct I2cController {
    devices: HashMap<I2cAddress, I2cDevice>,
    bus_speed: u32, // Hz
}

impl I2cController {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            bus_speed: 100_000, // Default: 100 kHz
        }
    }

    pub fn with_speed(mut self, speed: u32) -> Self {
        self.bus_speed = speed;
        self
    }

    pub fn add_device(&mut self, address: I2cAddress) -> Result<(), String> {
        if self.devices.contains_key(&address) {
            return Err(format!(
                "Device with address 0x{:02X} already exists",
                address
            ));
        }
        self.devices.insert(address, I2cDevice::new(address));
        Ok(())
    }

    pub fn scan(&self) -> Vec<I2cAddress> {
        let mut addresses: Vec<I2cAddress> = self.devices.keys().copied().collect();
        addresses.sort();
        addresses
    }

    pub fn write(&mut self, address: I2cAddress, data: &[u8]) -> Result<(), String> {
        let device = self
            .devices
            .get_mut(&address)
            .ok_or_else(|| format!("No device found at address 0x{:02X}", address))?;

        if data.is_empty() {
            return Err("Cannot write empty data".to_string());
        }

        // First byte is register address, rest is data
        let register = data[0];
        if data.len() > 1 {
            device.write_registers(register, &data[1..]);
        }

        Ok(())
    }

    pub fn read(
        &self,
        address: I2cAddress,
        register: I2cRegister,
        count: usize,
    ) -> Result<Vec<u8>, String> {
        let device = self
            .devices
            .get(&address)
            .ok_or_else(|| format!("No device found at address 0x{:02X}", address))?;

        Ok(device.read_registers(register, count))
    }

    pub fn write_read(
        &mut self,
        address: I2cAddress,
        write_data: &[u8],
        read_count: usize,
    ) -> Result<Vec<u8>, String> {
        // Write operation
        if !write_data.is_empty() {
            self.write(address, write_data)?;
        }

        // Read operation (from first byte of write_data as register)
        let register = write_data.first().copied().unwrap_or(0);
        self.read(address, register, read_count)
    }
}

impl Default for I2cController {
    fn default() -> Self {
        Self::new()
    }
}

/// SPI mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpiMode {
    Mode0, // CPOL=0, CPHA=0
    Mode1, // CPOL=0, CPHA=1
    Mode2, // CPOL=1, CPHA=0
    Mode3, // CPOL=1, CPHA=1
}

impl SpiMode {
    pub fn from_int(mode: u8) -> Result<Self, String> {
        match mode {
            0 => Ok(SpiMode::Mode0),
            1 => Ok(SpiMode::Mode1),
            2 => Ok(SpiMode::Mode2),
            3 => Ok(SpiMode::Mode3),
            _ => Err(format!("Invalid SPI mode: {}", mode)),
        }
    }
}

/// SPI device
#[derive(Debug, Clone)]
pub struct SpiDevice {
    pub id: String,
    pub data_buffer: Vec<u8>,
    pub mode: SpiMode,
}

impl SpiDevice {
    pub fn new(id: String, mode: SpiMode) -> Self {
        Self {
            id,
            data_buffer: Vec::new(),
            mode,
        }
    }

    pub fn transfer(&mut self, tx_data: &[u8]) -> Vec<u8> {
        // Simulate SPI transfer: send tx_data and return simulated rx_data
        // In real hardware, SPI is full-duplex
        self.data_buffer.extend_from_slice(tx_data);

        // Return dummy data (or echo in simulation)
        tx_data.iter().map(|&b| b.wrapping_add(1)).collect()
    }

    pub fn write(&mut self, data: &[u8]) {
        self.data_buffer.extend_from_slice(data);
    }

    pub fn read(&mut self, count: usize) -> Vec<u8> {
        // Return dummy data
        (0..count).map(|i| i as u8).collect()
    }
}

/// SPI controller
#[derive(Debug, Clone)]
pub struct SpiController {
    devices: HashMap<String, SpiDevice>,
    clock_speed: u32, // Hz
    bits_per_word: u8,
}

impl SpiController {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            clock_speed: 1_000_000, // Default: 1 MHz
            bits_per_word: 8,
        }
    }

    pub fn with_config(mut self, clock_speed: u32, bits_per_word: u8) -> Self {
        self.clock_speed = clock_speed;
        self.bits_per_word = bits_per_word;
        self
    }

    pub fn add_device(&mut self, id: String, mode: SpiMode) -> Result<(), String> {
        if self.devices.contains_key(&id) {
            return Err(format!("SPI device '{}' already exists", id));
        }
        self.devices.insert(id.clone(), SpiDevice::new(id, mode));
        Ok(())
    }

    pub fn transfer(&mut self, device_id: &str, tx_data: &[u8]) -> Result<Vec<u8>, String> {
        let device = self
            .devices
            .get_mut(device_id)
            .ok_or_else(|| format!("SPI device '{}' not found", device_id))?;

        Ok(device.transfer(tx_data))
    }

    pub fn write(&mut self, device_id: &str, data: &[u8]) -> Result<(), String> {
        let device = self
            .devices
            .get_mut(device_id)
            .ok_or_else(|| format!("SPI device '{}' not found", device_id))?;

        device.write(data);
        Ok(())
    }

    pub fn read(&mut self, device_id: &str, count: usize) -> Result<Vec<u8>, String> {
        let device = self
            .devices
            .get_mut(device_id)
            .ok_or_else(|| format!("SPI device '{}' not found", device_id))?;

        Ok(device.read(count))
    }

    pub fn list_devices(&self) -> Vec<String> {
        let mut devices: Vec<String> = self.devices.keys().cloned().collect();
        devices.sort();
        devices
    }
}

impl Default for SpiController {
    fn default() -> Self {
        Self::new()
    }
}

/// UART configuration
#[derive(Debug, Clone)]
pub struct UartConfig {
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,
    pub parity: UartParity,
}

impl UartConfig {
    pub fn new(baud_rate: u32) -> Self {
        Self {
            baud_rate,
            data_bits: 8,
            stop_bits: 1,
            parity: UartParity::None,
        }
    }
}

impl Default for UartConfig {
    fn default() -> Self {
        Self::new(9600)
    }
}

/// UART parity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UartParity {
    None,
    Even,
    Odd,
}

impl UartParity {
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" | "n" => Ok(UartParity::None),
            "even" | "e" => Ok(UartParity::Even),
            "odd" | "o" => Ok(UartParity::Odd),
            _ => Err(format!("Unknown parity: {}", s)),
        }
    }
}

/// UART port
#[derive(Debug, Clone)]
pub struct UartPort {
    pub port_name: String,
    pub config: UartConfig,
    pub rx_buffer: Vec<u8>,
    pub tx_buffer: Vec<u8>,
}

impl UartPort {
    pub fn new(port_name: String, config: UartConfig) -> Self {
        Self {
            port_name,
            config,
            rx_buffer: Vec::new(),
            tx_buffer: Vec::new(),
        }
    }

    pub fn write(&mut self, data: &[u8]) -> usize {
        self.tx_buffer.extend_from_slice(data);
        data.len()
    }

    pub fn write_string(&mut self, s: &str) -> usize {
        self.write(s.as_bytes())
    }

    pub fn read(&mut self, count: usize) -> Vec<u8> {
        let available = self.rx_buffer.len().min(count);
        self.rx_buffer.drain(..available).collect()
    }

    pub fn read_string(&mut self, count: usize) -> Result<String, String> {
        let bytes = self.read(count);
        String::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8: {}", e))
    }

    pub fn available(&self) -> usize {
        self.rx_buffer.len()
    }

    pub fn flush(&mut self) {
        self.tx_buffer.clear();
    }

    pub fn clear_rx(&mut self) {
        self.rx_buffer.clear();
    }

    // Simulate receiving data (for testing)
    pub fn simulate_receive(&mut self, data: &[u8]) {
        self.rx_buffer.extend_from_slice(data);
    }

    pub fn get_transmitted(&self) -> &[u8] {
        &self.tx_buffer
    }
}

/// UART controller
#[derive(Debug, Clone)]
pub struct UartController {
    ports: HashMap<String, UartPort>,
}

impl UartController {
    pub fn new() -> Self {
        Self {
            ports: HashMap::new(),
        }
    }

    pub fn open(&mut self, port_name: String, config: UartConfig) -> Result<(), String> {
        if self.ports.contains_key(&port_name) {
            return Err(format!("Port '{}' already open", port_name));
        }
        self.ports
            .insert(port_name.clone(), UartPort::new(port_name, config));
        Ok(())
    }

    pub fn close(&mut self, port_name: &str) -> Result<(), String> {
        self.ports
            .remove(port_name)
            .ok_or_else(|| format!("Port '{}' not found", port_name))?;
        Ok(())
    }

    pub fn write(&mut self, port_name: &str, data: &[u8]) -> Result<usize, String> {
        let port = self
            .ports
            .get_mut(port_name)
            .ok_or_else(|| format!("Port '{}' not found", port_name))?;
        Ok(port.write(data))
    }

    pub fn write_string(&mut self, port_name: &str, s: &str) -> Result<usize, String> {
        self.write(port_name, s.as_bytes())
    }

    pub fn read(&mut self, port_name: &str, count: usize) -> Result<Vec<u8>, String> {
        let port = self
            .ports
            .get_mut(port_name)
            .ok_or_else(|| format!("Port '{}' not found", port_name))?;
        Ok(port.read(count))
    }

    pub fn read_string(&mut self, port_name: &str, count: usize) -> Result<String, String> {
        let port = self
            .ports
            .get_mut(port_name)
            .ok_or_else(|| format!("Port '{}' not found", port_name))?;
        port.read_string(count)
    }

    pub fn available(&self, port_name: &str) -> Result<usize, String> {
        let port = self
            .ports
            .get(port_name)
            .ok_or_else(|| format!("Port '{}' not found", port_name))?;
        Ok(port.available())
    }

    pub fn list_ports(&self) -> Vec<String> {
        let mut ports: Vec<String> = self.ports.keys().cloned().collect();
        ports.sort();
        ports
    }

    // Simulation helper
    pub fn simulate_receive(&mut self, port_name: &str, data: &[u8]) -> Result<(), String> {
        let port = self
            .ports
            .get_mut(port_name)
            .ok_or_else(|| format!("Port '{}' not found", port_name))?;
        port.simulate_receive(data);
        Ok(())
    }
}

impl Default for UartController {
    fn default() -> Self {
        Self::new()
    }
}

/// Hardware manager - centralizes all hardware interfaces
#[derive(Debug, Clone)]
pub struct HardwareManager {
    pub gpio: Arc<Mutex<GpioController>>,
    pub i2c: Arc<Mutex<I2cController>>,
    pub spi: Arc<Mutex<SpiController>>,
    pub uart: Arc<Mutex<UartController>>,
}

impl HardwareManager {
    pub fn new() -> Self {
        Self {
            gpio: Arc::new(Mutex::new(GpioController::new())),
            i2c: Arc::new(Mutex::new(I2cController::new())),
            spi: Arc::new(Mutex::new(SpiController::new())),
            uart: Arc::new(Mutex::new(UartController::new())),
        }
    }

    pub fn reset(&self) {
        *self.gpio.lock().unwrap() = GpioController::new();
        *self.i2c.lock().unwrap() = I2cController::new();
        *self.spi.lock().unwrap() = SpiController::new();
        *self.uart.lock().unwrap() = UartController::new();
    }
}

impl Default for HardwareManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpio_pin_operations() {
        let mut controller = GpioController::new();
        controller.init_pin(13, PinMode::Output).unwrap();

        controller.write(13, PinState::High).unwrap();
        assert_eq!(controller.read(13).unwrap(), PinState::High);

        controller.toggle(13).unwrap();
        assert_eq!(controller.read(13).unwrap(), PinState::Low);
    }

    #[test]
    fn test_gpio_pwm() {
        let mut controller = GpioController::new();
        controller.init_pin(9, PinMode::Output).unwrap();

        assert!(controller.set_pwm(9, 0.5).is_ok());
        assert!(controller.set_pwm(9, 1.5).is_err()); // Out of range
    }

    #[test]
    fn test_i2c_device() {
        let mut controller = I2cController::new();
        controller.add_device(0x48).unwrap();

        // Write to register 0x01
        controller.write(0x48, &[0x01, 0xFF]).unwrap();

        // Read from register 0x01
        let data = controller.read(0x48, 0x01, 1).unwrap();
        assert_eq!(data[0], 0xFF);
    }

    #[test]
    fn test_i2c_scan() {
        let mut controller = I2cController::new();
        controller.add_device(0x48).unwrap();
        controller.add_device(0x50).unwrap();

        let devices = controller.scan();
        assert_eq!(devices.len(), 2);
        assert!(devices.contains(&0x48));
        assert!(devices.contains(&0x50));
    }

    #[test]
    fn test_spi_transfer() {
        let mut controller = SpiController::new();
        controller
            .add_device("device0".to_string(), SpiMode::Mode0)
            .unwrap();

        let tx_data = vec![0x01, 0x02, 0x03];
        let rx_data = controller.transfer("device0", &tx_data).unwrap();
        assert_eq!(rx_data.len(), tx_data.len());
    }

    #[test]
    fn test_uart_communication() {
        let mut controller = UartController::new();
        let config = UartConfig::new(9600);
        controller.open("serial0".to_string(), config).unwrap();

        // Write data
        let written = controller.write_string("serial0", "Hello UART").unwrap();
        assert_eq!(written, 10);

        // Simulate receiving data
        controller.simulate_receive("serial0", b"Response").unwrap();

        // Read data
        let read_data = controller.read_string("serial0", 8).unwrap();
        assert_eq!(read_data, "Response");
    }

    #[test]
    fn test_hardware_manager() {
        let manager = HardwareManager::new();

        // Test GPIO access
        {
            let mut gpio = manager.gpio.lock().unwrap();
            gpio.init_pin(1, PinMode::Output).unwrap();
        }

        // Test I2C access
        {
            let mut i2c = manager.i2c.lock().unwrap();
            i2c.add_device(0x20).unwrap();
        }

        // Reset everything
        manager.reset();

        // Verify reset worked
        {
            let gpio = manager.gpio.lock().unwrap();
            assert_eq!(gpio.list_pins().len(), 0);
        }
    }
}
