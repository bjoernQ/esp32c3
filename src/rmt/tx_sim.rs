#[doc = "Register `TX_SIM` reader"]
pub struct R(crate::R<TX_SIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_SIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_SIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_SIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_SIM` writer"]
pub struct W(crate::W<TX_SIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_SIM_SPEC>;
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
impl From<crate::W<TX_SIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_SIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMT_TX_SIM_CH0` reader - reg_rmt_tx_sim_ch0."]
pub struct RMT_TX_SIM_CH0_R(crate::FieldReader<bool, bool>);
impl RMT_TX_SIM_CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMT_TX_SIM_CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMT_TX_SIM_CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMT_TX_SIM_CH0` writer - reg_rmt_tx_sim_ch0."]
pub struct RMT_TX_SIM_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_TX_SIM_CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RMT_TX_SIM_CH1` reader - reg_rmt_tx_sim_ch1."]
pub struct RMT_TX_SIM_CH1_R(crate::FieldReader<bool, bool>);
impl RMT_TX_SIM_CH1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMT_TX_SIM_CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMT_TX_SIM_CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMT_TX_SIM_CH1` writer - reg_rmt_tx_sim_ch1."]
pub struct RMT_TX_SIM_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_TX_SIM_CH1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RMT_TX_SIM_EN` reader - reg_rmt_tx_sim_en."]
pub struct RMT_TX_SIM_EN_R(crate::FieldReader<bool, bool>);
impl RMT_TX_SIM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMT_TX_SIM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMT_TX_SIM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMT_TX_SIM_EN` writer - reg_rmt_tx_sim_en."]
pub struct RMT_TX_SIM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_TX_SIM_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - reg_rmt_tx_sim_ch0."]
    #[inline(always)]
    pub fn rmt_tx_sim_ch0(&self) -> RMT_TX_SIM_CH0_R {
        RMT_TX_SIM_CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - reg_rmt_tx_sim_ch1."]
    #[inline(always)]
    pub fn rmt_tx_sim_ch1(&self) -> RMT_TX_SIM_CH1_R {
        RMT_TX_SIM_CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - reg_rmt_tx_sim_en."]
    #[inline(always)]
    pub fn rmt_tx_sim_en(&self) -> RMT_TX_SIM_EN_R {
        RMT_TX_SIM_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reg_rmt_tx_sim_ch0."]
    #[inline(always)]
    pub fn rmt_tx_sim_ch0(&mut self) -> RMT_TX_SIM_CH0_W {
        RMT_TX_SIM_CH0_W { w: self }
    }
    #[doc = "Bit 1 - reg_rmt_tx_sim_ch1."]
    #[inline(always)]
    pub fn rmt_tx_sim_ch1(&mut self) -> RMT_TX_SIM_CH1_W {
        RMT_TX_SIM_CH1_W { w: self }
    }
    #[doc = "Bit 2 - reg_rmt_tx_sim_en."]
    #[inline(always)]
    pub fn rmt_tx_sim_en(&mut self) -> RMT_TX_SIM_EN_W {
        RMT_TX_SIM_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_TX_SIM_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_sim](index.html) module"]
pub struct TX_SIM_SPEC;
impl crate::RegisterSpec for TX_SIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_sim::R](R) reader structure"]
impl crate::Readable for TX_SIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_sim::W](W) writer structure"]
impl crate::Writable for TX_SIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_SIM to value 0"]
impl crate::Resettable for TX_SIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
