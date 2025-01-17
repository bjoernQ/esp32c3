#[doc = "Register `TX_PCM2PDM_CONF1` reader"]
pub struct R(crate::R<TX_PCM2PDM_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_PCM2PDM_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_PCM2PDM_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_PCM2PDM_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_PCM2PDM_CONF1` writer"]
pub struct W(crate::W<TX_PCM2PDM_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_PCM2PDM_CONF1_SPEC>;
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
impl From<crate::W<TX_PCM2PDM_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_PCM2PDM_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PDM_FP` reader - I2S TX PDM Fp"]
pub struct TX_PDM_FP_R(crate::FieldReader<u16, u16>);
impl TX_PDM_FP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TX_PDM_FP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PDM_FP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PDM_FP` writer - I2S TX PDM Fp"]
pub struct TX_PDM_FP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_FP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `TX_PDM_FS` reader - I2S TX PDM Fs"]
pub struct TX_PDM_FS_R(crate::FieldReader<u16, u16>);
impl TX_PDM_FS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TX_PDM_FS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_PDM_FS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_PDM_FS` writer - I2S TX PDM Fs"]
pub struct TX_PDM_FS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PDM_FS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `TX_IIR_HP_MULT12_5` reader - The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5\\[2:0\\])"]
pub struct TX_IIR_HP_MULT12_5_R(crate::FieldReader<u8, u8>);
impl TX_IIR_HP_MULT12_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_IIR_HP_MULT12_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_IIR_HP_MULT12_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_IIR_HP_MULT12_5` writer - The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5\\[2:0\\])"]
pub struct TX_IIR_HP_MULT12_5_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IIR_HP_MULT12_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `TX_IIR_HP_MULT12_0` reader - The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0\\[2:0\\])"]
pub struct TX_IIR_HP_MULT12_0_R(crate::FieldReader<u8, u8>);
impl TX_IIR_HP_MULT12_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_IIR_HP_MULT12_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_IIR_HP_MULT12_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_IIR_HP_MULT12_0` writer - The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0\\[2:0\\])"]
pub struct TX_IIR_HP_MULT12_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_IIR_HP_MULT12_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | ((value as u32 & 0x07) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - I2S TX PDM Fp"]
    #[inline(always)]
    pub fn tx_pdm_fp(&self) -> TX_PDM_FP_R {
        TX_PDM_FP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - I2S TX PDM Fs"]
    #[inline(always)]
    pub fn tx_pdm_fs(&self) -> TX_PDM_FS_R {
        TX_PDM_FS_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:22 - The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5\\[2:0\\])"]
    #[inline(always)]
    pub fn tx_iir_hp_mult12_5(&self) -> TX_IIR_HP_MULT12_5_R {
        TX_IIR_HP_MULT12_5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0\\[2:0\\])"]
    #[inline(always)]
    pub fn tx_iir_hp_mult12_0(&self) -> TX_IIR_HP_MULT12_0_R {
        TX_IIR_HP_MULT12_0_R::new(((self.bits >> 23) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2S TX PDM Fp"]
    #[inline(always)]
    pub fn tx_pdm_fp(&mut self) -> TX_PDM_FP_W {
        TX_PDM_FP_W { w: self }
    }
    #[doc = "Bits 10:19 - I2S TX PDM Fs"]
    #[inline(always)]
    pub fn tx_pdm_fs(&mut self) -> TX_PDM_FS_W {
        TX_PDM_FS_W { w: self }
    }
    #[doc = "Bits 20:22 - The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5\\[2:0\\])"]
    #[inline(always)]
    pub fn tx_iir_hp_mult12_5(&mut self) -> TX_IIR_HP_MULT12_5_W {
        TX_IIR_HP_MULT12_5_W { w: self }
    }
    #[doc = "Bits 23:25 - The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0\\[2:0\\])"]
    #[inline(always)]
    pub fn tx_iir_hp_mult12_0(&mut self) -> TX_IIR_HP_MULT12_0_W {
        TX_IIR_HP_MULT12_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S TX PCM2PDM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_pcm2pdm_conf1](index.html) module"]
pub struct TX_PCM2PDM_CONF1_SPEC;
impl crate::RegisterSpec for TX_PCM2PDM_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_pcm2pdm_conf1::R](R) reader structure"]
impl crate::Readable for TX_PCM2PDM_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_pcm2pdm_conf1::W](W) writer structure"]
impl crate::Writable for TX_PCM2PDM_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_PCM2PDM_CONF1 to value 0x03f7_83c0"]
impl crate::Resettable for TX_PCM2PDM_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03f7_83c0
    }
}
