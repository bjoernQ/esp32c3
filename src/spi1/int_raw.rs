#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PER_END_INT_RAW` reader - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed. 0: Others."]
pub struct PER_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl PER_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PER_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES_END_INT_RAW` reader - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended. 0: Others."]
pub struct PES_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl PES_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PES_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PES_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPE_END_INT_RAW` reader - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
pub struct WPE_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl WPE_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WPE_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPE_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub struct SLV_ST_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl SLV_ST_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_ST_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_ST_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MST_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
pub struct MST_ST_END_INT_RAW_R(crate::FieldReader<bool, bool>);
impl MST_ST_END_INT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MST_ST_END_INT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MST_ST_END_INT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The raw bit for SPI_MEM_PER_END_INT interrupt. 1: Triggered when Auto Resume command (0x7A) is sent and flash is resumed. 0: Others."]
    #[inline(always)]
    pub fn per_end_int_raw(&self) -> PER_END_INT_RAW_R {
        PER_END_INT_RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The raw bit for SPI_MEM_PES_END_INT interrupt.1: Triggered when Auto Suspend command (0x75) is sent and flash is suspended. 0: Others."]
    #[inline(always)]
    pub fn pes_end_int_raw(&self) -> PES_END_INT_RAW_R {
        PES_END_INT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The raw bit for SPI_MEM_WPE_END_INT interrupt. 1: Triggered when WRSR/PP/SE/BE/CE is sent and flash is already idle. 0: Others."]
    #[inline(always)]
    pub fn wpe_end_int_raw(&self) -> WPE_END_INT_RAW_R {
        WPE_END_INT_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi1_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn slv_st_end_int_raw(&self) -> SLV_ST_END_INT_RAW_R {
        SLV_ST_END_INT_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi1_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn mst_st_end_int_raw(&self) -> MST_ST_END_INT_RAW_R {
        MST_ST_END_INT_RAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "SPI1 interrupt raw register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}