// NOTE:
// from esp32 bus pirate
// struct Reply {
//     bool ready = false;
//     bool ok = false;
//     uint8_t fc = 0;
//     uint8_t exception = 0;
//     uint8_t  byteCount = 0;         // FC01/FC02
//     std::string error;
//     std::vector<uint16_t> regs;     // FC03/FC04
//     std::vector<uint8_t> coilBytes; // FC01/FC02
//     std::vector<uint8_t> raw;
//   };
//
//   // Configure target host
//   bool setTarget(const std::string& hostOrIp, uint16_t port);
//
//   // Apply config
//   void begin(uint32_t reqTimeoutMs = 10000,
//              uint32_t idleCloseMs  = 60000,
//              uint32_t maxInflight  = 4);
//
//   // Basic requests
//   Error readHolding(uint8_t unit, uint16_t addr0, uint16_t qty);
//   Error writeHoldingMultiple(uint8_t unit, uint16_t addr0, const std::vector<uint16_t>& values);
//   Error writeHoldingSingle(uint8_t unit, uint16_t addr0, uint16_t value);
//   Error readInputRegisters(uint8_t unit, uint16_t addr0, uint16_t qty);
//   Error readCoils(uint8_t unit, uint16_t addr0, uint16_t qty);
//   Error readDiscreteInputs(uint8_t unit, uint16_t addr0, uint16_t qty);
//   Error writeSingleCoil(uint8_t unit, uint16_t addr0, bool on);
//   Error writeMultipleCoils(uint8_t unit, uint16_t addr0,
//                             const std::vector<uint8_t>& packedBytes,
//                             uint16_t coilQty);
//
//   // Callbacks
//   using ReplyHandler = std::function<void(const Reply&, uint32_t token)>;
//   void setOnReply(ReplyHandler h) { _onReply = std::move(h); }
//   void setOnData(std::function<void(const ModbusMessage&, uint32_t)> cb) { _onData = std::move(cb); }
//   void setOnError(std::function<void(Error, uint32_t)> cb)               { _onError = std::move(cb); }
//   void clearCallbacks() { _onData = {}; _onError = {}; _onReply = {}; }
//
//   // Helpers
//   static bool parseFC03or04(ModbusMessage& msg, std::vector<uint16_t>& outRegs);
//   static bool parseFC01or02(ModbusMessage& msg,
//                           std::vector<uint8_t>& outBytes,
//                           uint8_t& outByteCount);
