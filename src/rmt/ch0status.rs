#[doc = "Register `CH0STATUS` reader"]
pub struct R(crate::R<CH0STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEM_RADDR_EX_CH0` reader - reg_mem_raddr_ex_ch0."]
pub struct MEM_RADDR_EX_CH0_R(crate::FieldReader<u16, u16>);
impl MEM_RADDR_EX_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MEM_RADDR_EX_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_RADDR_EX_CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE_CH0` reader - reg_state_ch0."]
pub struct STATE_CH0_R(crate::FieldReader<u8, u8>);
impl STATE_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STATE_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_CH0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_WADDR_CH0` reader - reg_apb_mem_waddr_ch0."]
pub struct APB_MEM_WADDR_CH0_R(crate::FieldReader<u16, u16>);
impl APB_MEM_WADDR_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        APB_MEM_WADDR_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_WADDR_CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_RD_ERR_CH0` reader - reg_apb_mem_rd_err_ch0."]
pub struct APB_MEM_RD_ERR_CH0_R(crate::FieldReader<bool, bool>);
impl APB_MEM_RD_ERR_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_MEM_RD_ERR_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_RD_ERR_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_EMPTY_CH0` reader - reg_mem_empty_ch0."]
pub struct MEM_EMPTY_CH0_R(crate::FieldReader<bool, bool>);
impl MEM_EMPTY_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEM_EMPTY_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_EMPTY_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_WR_ERR_CH0` reader - reg_apb_mem_wr_err_ch0."]
pub struct APB_MEM_WR_ERR_CH0_R(crate::FieldReader<bool, bool>);
impl APB_MEM_WR_ERR_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        APB_MEM_WR_ERR_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_WR_ERR_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB_MEM_RADDR_CH0` reader - reg_apb_mem_raddr_ch0."]
pub struct APB_MEM_RADDR_CH0_R(crate::FieldReader<u8, u8>);
impl APB_MEM_RADDR_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        APB_MEM_RADDR_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_MEM_RADDR_CH0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8 - reg_mem_raddr_ex_ch0."]
    #[inline(always)]
    pub fn mem_raddr_ex_ch0(&self) -> MEM_RADDR_EX_CH0_R {
        MEM_RADDR_EX_CH0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - reg_state_ch0."]
    #[inline(always)]
    pub fn state_ch0(&self) -> STATE_CH0_R {
        STATE_CH0_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:20 - reg_apb_mem_waddr_ch0."]
    #[inline(always)]
    pub fn apb_mem_waddr_ch0(&self) -> APB_MEM_WADDR_CH0_R {
        APB_MEM_WADDR_CH0_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bit 21 - reg_apb_mem_rd_err_ch0."]
    #[inline(always)]
    pub fn apb_mem_rd_err_ch0(&self) -> APB_MEM_RD_ERR_CH0_R {
        APB_MEM_RD_ERR_CH0_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - reg_mem_empty_ch0."]
    #[inline(always)]
    pub fn mem_empty_ch0(&self) -> MEM_EMPTY_CH0_R {
        MEM_EMPTY_CH0_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - reg_apb_mem_wr_err_ch0."]
    #[inline(always)]
    pub fn apb_mem_wr_err_ch0(&self) -> APB_MEM_WR_ERR_CH0_R {
        APB_MEM_WR_ERR_CH0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - reg_apb_mem_raddr_ch0."]
    #[inline(always)]
    pub fn apb_mem_raddr_ch0(&self) -> APB_MEM_RADDR_CH0_R {
        APB_MEM_RADDR_CH0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "RMT_CH0STATUS_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0status](index.html) module"]
pub struct CH0STATUS_SPEC;
impl crate::RegisterSpec for CH0STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch0status::R](R) reader structure"]
impl crate::Readable for CH0STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH0STATUS to value 0"]
impl crate::Resettable for CH0STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
