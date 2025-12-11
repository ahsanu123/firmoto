use crate::{
    controllers::controller_result::CR,
    errors::spi_controller_err::SpiControllerErr,
    request_mappers::spi_controller_request::{
        SpiReadU8Req, SpiReadU16Req, SpiWriteThenReadReq, SpiWriteU8Req,
    },
    response_mappers::spi_controller_response::{
        SpiReadU8Res, SpiReadU16Res, SpiWriteThenReadRes, SpiWriteU8Res,
    },
    service_wrappers::spi_service_wrapper::SpiServiceWrapperTrait,
};
use flatbuffers::FlatBufferBuilder;

type CRes<T> = CR<T, SpiControllerErr>;

pub trait SpiControllerTrait {
    fn write_u8(&mut self, req: SpiWriteU8Req) -> CRes<SpiWriteU8Res>;
    fn read_u16(&mut self, req: SpiReadU16Req) -> CRes<SpiReadU16Res>;
    fn read_u8(&mut self, req: SpiReadU8Req) -> CRes<SpiReadU8Res>;
    fn write_then_read(&mut self, req: SpiWriteThenReadReq) -> CRes<SpiWriteThenReadRes>;
}

pub struct SpiController<T>
where
    T: SpiServiceWrapperTrait,
{
    wrapper: T,
}

impl<T> SpiController<T>
where
    T: SpiServiceWrapperTrait,
{
    pub fn new(spi_wrapper: T) -> Self {
        Self {
            wrapper: spi_wrapper,
        }
    }
}

impl<T> SpiControllerTrait for SpiController<T>
where
    T: SpiServiceWrapperTrait,
{
    fn write_u8(&mut self, req: SpiWriteU8Req) -> CRes<SpiWriteU8Res> {
        let mut _builder = FlatBufferBuilder::with_capacity(2048);

        let success = self.wrapper.write_u8(8u8, 8u8).is_ok();

        CR(Ok(SpiWriteU8Res { success }))
    }

    fn read_u16(&mut self, req: SpiReadU16Req) -> CRes<SpiReadU16Res> {
        todo!()
    }

    fn read_u8(&mut self, req: SpiReadU8Req) -> CRes<SpiReadU8Res> {
        todo!()
    }

    fn write_then_read(&mut self, req: SpiWriteThenReadReq) -> CRes<SpiWriteThenReadRes> {
        todo!()
    }
}
