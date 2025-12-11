// NOTE:
// from esp32 bus pirate
// void configure(uint8_t csPin, uint8_t sck, uint8_t miso, uint8_t mosi, uint32_t bitrateKbps = 125);
// void end();
// void reset();
// void flush();
//
// bool sendFrame(uint32_t id, const std::vector<uint8_t>& data);
// bool readFrame(struct can_frame& outFrame);
// std::string readFrameAsString();  // pour affichage
//
// void setBitrate(uint32_t bitrateKbps);
// uint32_t closestSupportedBitrate(uint32_t kbps);
// void setFilter(uint32_t id);
// void setMask(uint32_t mask);
//
// std::string getStatus();
// bool probe();
//

use crate::errors::can_controller_err::CanControllerErr;

pub trait CanControllerTrait {
    fn send_frame(id: u32, data: &[u8]) -> Result<(), CanControllerErr>;
    fn read_frame() -> Result<(), CanControllerErr>;
    fn set_bitrate(bit_kbps: u32) -> Result<(), CanControllerErr>;
}
