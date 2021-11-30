#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub struct CLK_MODE_R(crate::FieldReader<u8, u8>);
impl CLK_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub struct CLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CS_HOLD_DLY_RES` reader - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
* 512) SPI_CLK cycles."]
pub struct CS_HOLD_DLY_RES_R(crate::FieldReader<u16, u16>);
impl CS_HOLD_DLY_RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CS_HOLD_DLY_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_HOLD_DLY_RES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_HOLD_DLY_RES` writer - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
* 512) SPI_CLK cycles."]
pub struct CS_HOLD_DLY_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_HOLD_DLY_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 2)) | ((value as u32 & 0x03ff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn clk_mode(&self) -> CLK_MODE_R {
        CLK_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
* 512) SPI_CLK cycles."]
    #[inline(always)]
    pub fn cs_hold_dly_res(&self) -> CS_HOLD_DLY_RES_R {
        CS_HOLD_DLY_RES_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn clk_mode(&mut self) -> CLK_MODE_W {
        CLK_MODE_W { w: self }
    }
    #[doc = "Bits 2:11 - After RES/DP/HPM command is sent, SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\\[9:0\\]
* 512) SPI_CLK cycles."]
    #[inline(always)]
    pub fn cs_hold_dly_res(&mut self) -> CS_HOLD_DLY_RES_W {
        CS_HOLD_DLY_RES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 control1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL1 to value 0x0ffc"]
impl crate::Resettable for CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0ffc
    }
}
