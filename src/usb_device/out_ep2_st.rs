#[doc = "Register `OUT_EP2_ST` reader"]
pub struct R(crate::R<OUT_EP2_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_EP2_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_EP2_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_EP2_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT_EP2_STATE` reader - State of OUT Endpoint 2."]
pub struct OUT_EP2_STATE_R(crate::FieldReader<u8, u8>);
impl OUT_EP2_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_EP2_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EP2_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EP2_WR_ADDR` reader - Write data address of OUT endpoint 2. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2."]
pub struct OUT_EP2_WR_ADDR_R(crate::FieldReader<u8, u8>);
impl OUT_EP2_WR_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_EP2_WR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EP2_WR_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_EP2_RD_ADDR` reader - Read data address of OUT endpoint 2."]
pub struct OUT_EP2_RD_ADDR_R(crate::FieldReader<u8, u8>);
impl OUT_EP2_RD_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OUT_EP2_RD_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_EP2_RD_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - State of OUT Endpoint 2."]
    #[inline(always)]
    pub fn out_ep2_state(&self) -> OUT_EP2_STATE_R {
        OUT_EP2_STATE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:8 - Write data address of OUT endpoint 2. When USB_DEVICE_SERIAL_OUT_RECV_PKT_INT is detected, there are USB_DEVICE_OUT_EP2_WR_ADDR-2 bytes data in OUT EP2."]
    #[inline(always)]
    pub fn out_ep2_wr_addr(&self) -> OUT_EP2_WR_ADDR_R {
        OUT_EP2_WR_ADDR_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:15 - Read data address of OUT endpoint 2."]
    #[inline(always)]
    pub fn out_ep2_rd_addr(&self) -> OUT_EP2_RD_ADDR_R {
        OUT_EP2_RD_ADDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[doc = "USB_DEVICE_OUT_EP2_ST_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_ep2_st](index.html) module"]
pub struct OUT_EP2_ST_SPEC;
impl crate::RegisterSpec for OUT_EP2_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_ep2_st::R](R) reader structure"]
impl crate::Readable for OUT_EP2_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_EP2_ST to value 0"]
impl crate::Resettable for OUT_EP2_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
